//! Serialize layout to stdio.svg.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::{parse_svg_xml, SvgSnapshot};
use semio_s_plugin_stdio::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA;

pub async fn register() {}

pub async fn serialize(from: &LayoutSnapshot) -> Result<SvgSnapshot, store::PackError> {
    let text = <LayoutSnapshot as store::ArtifactDsl>::print_dsl(from);
    let doc = parse_svg_xml(&text).map_err(|e| store::PackError::Schema(e))?;
    Ok(SvgSnapshot { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc })
}

pub async fn serialize_text(from: &LayoutSnapshot) -> Result<String, store::PackError> {
    Ok(<LayoutSnapshot as store::ArtifactDsl>::print_dsl(from))
}
