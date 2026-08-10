//! Serialize cad to stdio.png.

use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::io::cad_to_wire;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

//#region Serialize
pub fn register() {}

pub fn serialize(from: &CadSnapshot) -> Result<PngSnapshot, store::PackError> {
    let mut raw = cad_to_wire(from);
    let width = raw.len() as u32;
    let pad = (4 - (raw.len() % 4)) % 4;
    raw.extend(std::iter::repeat(0u8).take(pad));
    Ok(PngSnapshot {
        schema: STDIO_PNG_DOCUMENT_SCHEMA.into(),
        image: semio_s_plugin_stdio::artifacts::png::schema::snapshot::RasterImage {
            width,
            height: 1,
            rgba: raw,
        },
    })
}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::ArtifactDsl>::print_dsl(from))
}
//#endregion Serialize
