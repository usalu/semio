//! 🏞️ Places the native graph's two authenticated original PNG plane references.

use super::{place_named_image_bodies, ImagePlacement, PdfImageBody, PdfMutation, PngGraphWriter};
use semio_plugin::Fault;
use semio_framework_artifact_reference::ArtifactRef;
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts::{
    PdfArtifactResourcePort, png_source::PngPlane,
};

/// 📍 Finishes a checked placement using resource names reserved against the same live snapshot.
pub fn place_graph_png_named(
    placement: ImagePlacement, source: &PngGraphWriter, width: u32, height: u32,
    color_id: String, mask_id: Option<String>,
) -> Result<Vec<PdfMutation>, Fault> {
    let resource = source.resource().filter(|_| source.rows_done())
        .ok_or_else(|| Fault::from("PDF PNG source rows must precede image placement"))?;
    let page = match placement { ImagePlacement::Insert { page, .. } | ImagePlacement::Replace { page, .. } => page };
    let body = |plane: PngPlane| PdfImageBody::Artifact { reference: plane.reference(resource.digest) };
    let mask = mask_id.map(|id| (id, body(PngPlane::Alpha)));
    place_named_image_bodies(placement, page, width, height, color_id, body(PngPlane::Rgb), mask)
}

/// 🖼️ Opens the exact straight native samples for the existing preview object's bounds.
pub fn graph_png_preview(
    port: &impl PdfArtifactResourcePort, reference: &ArtifactRef, size: (u32, u32), channels: usize,
) -> Option<Vec<u8>> {
    let plane = PngPlane::match_reference(reference)?;
    if plane.channels() != channels { return None; }
    let value = port.resolve(reference).ok()?;
    if value.dict_get("Width")?.as_i64()? != i64::from(size.0) || value.dict_get("Height")?.as_i64()? != i64::from(size.1) {
        return None;
    }
    let super::PdfObject::Stream { data, .. } = value else { return None; };
    Some(data)
}
