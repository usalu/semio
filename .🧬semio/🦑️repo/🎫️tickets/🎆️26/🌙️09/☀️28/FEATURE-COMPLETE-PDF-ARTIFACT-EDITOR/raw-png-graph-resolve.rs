//! 🧪️ Opens the bounded native PNG source as one straight color or grayscale alpha plane.

use super::{PngGraphSource, PngPlane, PNG_GRAPH_MAXIMUM_PIXELS};
use crate::standards::v1_7::subsets::base::{
    io::error::{PResult, PdfEngineError},
    schema::snapshot::{PdfDictEntry, PdfObject, PdfStreamFilter},
};
use semio_framework_pixels::PngScanlineDecoder;
use semio_framework_value::paged::ByteReadSource;

pub(crate) fn lower_source_plane(plane: PngPlane, source: PngGraphSource<'_>) -> PResult<PdfObject> {
    let malformed = || PdfEngineError::Malformed("PDF PNG source is incomplete or outside its admitted raster extent".into());
    let mut data = Vec::new();
    data.try_reserve_exact(source.byte_len()).map_err(|_| malformed())?;
    data.resize(source.byte_len(), 0);
    if source.read_bytes(0, &mut data) != data.len() { return Err(malformed()); }
    let geometry = data.get(16..24).ok_or_else(malformed)?;
    let width = u32::from_be_bytes(geometry[..4].try_into().unwrap());
    let height = u32::from_be_bytes(geometry[4..].try_into().unwrap());
    let count = (width as usize).checked_mul(height as usize).filter(|pixels| (1..=PNG_GRAPH_MAXIMUM_PIXELS).contains(pixels))
        .ok_or_else(malformed)? * plane.channels();
    let mut decoder = PngScanlineDecoder::from_vec(data).map_err(|_| malformed())?;
    if (decoder.width(), decoder.height()) != (width, height) { return Err(malformed()); }
    let mut samples = Vec::new();
    samples.try_reserve_exact(count).map_err(|_| malformed())?;
    while let Some(row) = decoder.next_row().map_err(|_| malformed())? {
        if row.len() != width as usize * 4 || samples.len() + width as usize * plane.channels() > count {
            return Err(malformed());
        }
        for pixel in row.chunks_exact(4) {
            match plane { PngPlane::Rgb => samples.extend_from_slice(&pixel[..3]), PngPlane::Alpha => samples.push(pixel[3]) }
        }
    }
    if samples.len() != count { return Err(malformed()); }
    let dict = vec![
        PdfDictEntry::new("Width", PdfObject::Int(i64::from(width))),
        PdfDictEntry::new("Height", PdfObject::Int(i64::from(height))),
        PdfDictEntry::new("SemioChannels", PdfObject::Int(plane.channels() as i64)),
    ];
    Ok(PdfObject::Stream { dict, data: samples, filters: vec![PdfStreamFilter::Flate { predictor: None }] })
}
