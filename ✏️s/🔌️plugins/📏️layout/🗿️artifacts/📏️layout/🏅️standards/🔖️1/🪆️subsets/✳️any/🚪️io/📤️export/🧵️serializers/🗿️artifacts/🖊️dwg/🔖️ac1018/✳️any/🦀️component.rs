//! Serialize layout to stdio.dwg.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::dwg::schema::snapshot::decode_dwg;
use semio_s_plugin_stdio::artifacts::dwg::DwgSnapshot;

pub async fn register() {}

/// 🩹️ 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining Part A: the old body
/// constructed a `DwgSnapshot { bytes, section_names, sections, decode_status: SentinelOnly, .. }`
/// sentinel that no longer exists on the real R2004+ `DwgSnapshot` (see stdio's
/// `📸️snapshot/🦀️component.rs`) -- this is stale drift, the same class E2 already fixed inside
/// `🗒️note`'s sibling serializer. There is no raw-byte field left to stash synthetic JSON in, so
/// route through the same honest path `🗒️note` uses instead of inventing one: this leaf's own DSL
/// text is SVG (see the sibling `🎨️svg` serializer in this directory), so render it to SVG and
/// decode it through the real `svg_to_dwg_bytes` -> `decode_dwg` pipeline -- a genuine (if
/// minimal) decode rather than a fabricated status.
pub async fn serialize(from: &LayoutSnapshot) -> Result<DwgSnapshot, store::PackError> {
    let text = <LayoutSnapshot as store::ArtifactDsl>::print_dsl(from);
    let bytes = semio_framework_os::svg_to_dwg_bytes(&text).map_err(store::PackError::Schema)?;
    decode_dwg(&bytes).map_err(store::PackError::Schema)
}
