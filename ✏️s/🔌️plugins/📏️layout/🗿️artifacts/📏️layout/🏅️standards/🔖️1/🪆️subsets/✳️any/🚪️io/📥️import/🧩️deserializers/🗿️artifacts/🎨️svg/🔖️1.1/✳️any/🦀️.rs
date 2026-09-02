//! Deserialize layout via stdio.svg.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::{write_svg_xml, SvgSnapshot};
use semio_s_plugin_stdio::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA;

pub async fn register() {}

pub async fn deserialize(from: &SvgSnapshot) -> Result<LayoutSnapshot, store::TextError> {
    let _ = STDIO_SVG_DOCUMENT_SCHEMA;
    let text = write_svg_xml(&from.doc);
    <LayoutSnapshot as store::ArtifactDsl>::parse_dsl(&text)
}

pub async fn deserialize_text(text: &str) -> Result<LayoutSnapshot, store::TextError> {
    <LayoutSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
