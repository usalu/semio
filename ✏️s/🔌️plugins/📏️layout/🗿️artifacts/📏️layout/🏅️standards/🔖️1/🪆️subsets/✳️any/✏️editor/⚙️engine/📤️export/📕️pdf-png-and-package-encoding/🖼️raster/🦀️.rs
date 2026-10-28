//! 🖼️ Bounded original RGB and alpha planes for Layout PDF image XObjects.
#[derive(Clone, Copy)]
pub(crate) enum RasterPlane { Rgb, Alpha }

pub(crate) fn raster_length(width: u32, height: u32, rgba: &[u8], plane: RasterPlane) -> Result<usize, String> {
    if width == 0 || height == 0 { return Err("layout-export-pdf-raster-dimensions".into()); }
    let pixels = usize::try_from(width).ok().and_then(|width| usize::try_from(height).ok().and_then(|height| width.checked_mul(height))).ok_or("layout-export-pdf-raster-dimensions")?;
    if pixels.checked_mul(4) != Some(rgba.len()) { return Err("layout-export-pdf-raster-bytes".into()); }
    pixels.checked_mul(match plane { RasterPlane::Rgb => 3, RasterPlane::Alpha => 1 }).ok_or_else(|| "layout-export-pdf-raster-bytes".into())
}

pub(super) fn raster_chunk(width:u32,height:u32,rgba:&[u8],plane:RasterPlane,cursor:usize,target:&mut[u8],work:&mut crate::output::OutputWork)->Result<usize,String>{let length=raster_length(width,height,rgba,plane)?;if cursor>length||target.is_empty()||target.len()>4096{return Err("layout-export-pdf-raster-turn".into());}let take=target.len().min(length-cursor);if take==0{return Ok(0);}work.copy(take+std::mem::size_of::<usize>()*3,1)?;for(index,byte)in target[..take].iter_mut().enumerate(){let at=cursor+index;*byte=rgba[match plane{RasterPlane::Rgb=>at/3*4+at%3,RasterPlane::Alpha=>at*4+3}];}Ok(take)}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
