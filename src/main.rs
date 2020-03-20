use futures::future::join_all;
use std::error::*;
use tokio::fs::*;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	let mut parallel = vec![];
	for i in 0..20 {
		parallel.push(get_image(format!(
			"2020-01-{}T11%3A45",
			match i {
				i if i < 10 => format!("0{}", i),
				i => format!("{}", i),
			}
		)));
	}

	join_all(parallel).await;

	Ok(())
}

async fn get_image(time: String) -> Result<(), Box<Error>> {
	let url =format!("https://eumetview.eumetsat.int/geoserv/wms?SERVICE=WMS&REQUEST=GetMap&TRANSPARENT=TRUE&EXCEPTIONS=INIMAGE&VERSION=1.3.0&LAYERS=meteosat%3Amsg_naturalenhncd&STYLES=raster&SRS=EPSG%3A4326&WIDTH=3712&HEIGHT=3712&BBOX=-76.9170259,-77,77.0829741,77&FORMAT=image%2Fpng&TIME={}%3A00.000Z&", time);
	println!("starting download of image {}", time);
	let img = reqwest::get(&url).await?.bytes().await?;
	println!("saving image {}", time);
	let mut file = File::create(format!("image{}.png", time)).await?;
	let p = file.write_all(img.as_ref()).await?;
	Ok(())
}
