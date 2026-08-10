//! present -> svg
use crate::artifacts::present::PresentSnapshot;
use semio_s_plugin_stdio::artifacts::svg::{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &PresentSnapshot) -> Result<SvgSnapshot, store::TextError> {
    let bytes = <PresentSnapshot as store::DocumentPack>::encode_pack(snapshot)
        .or_else(|_| Ok(<PresentSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes()))?;
    semio_s_plugin_stdio::artifacts::svg::engine::decode_svg(&bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &PresentSnapshot) -> Result<Vec<u8>, store::TextError> {
    semio_s_plugin_stdio::artifacts::svg::engine::encode_svg(&serialize(snapshot)?)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
