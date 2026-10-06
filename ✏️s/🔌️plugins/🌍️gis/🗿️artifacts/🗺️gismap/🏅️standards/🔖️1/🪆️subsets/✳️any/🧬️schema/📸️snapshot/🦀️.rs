//! 🧬️ GIS map snapshot schema — artifact-lane fields only.
//!
//! P6 handcrafted `ArtifactDsl`/`ArtifactPack` (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`):
//! `GisMapSnapshot` now carries real `store::ArtifactChild<…>` handles for its composed
//! `drawing`/`image`/`value` slots, which `dsl::DslRecord`'s derive cannot represent (no `DslField`
//! impl for `ArtifactChild<S>`) — same reason `🏔️gisterrain`/`💠️lowpoly`/`📐️cad` hand-roll their own
//! codecs. Follows their exact hex/bracket convention; `positions`/`routes`/`regions` (still real
//! `Vec<MapFeature>`, gis's own domain data, see `crate::🦀️.rs`'s
//! `🔖️Composition` region) round-trip via JSON-then-hex, matching `📐️cad`'s `enc_json`/`dec_json`
//! convention for its own structured (non-child) fields.

use crate::{gis_map_drawing_child_handle, gis_map_value_child_handle, GisMapDrawingChild, GisMapImageChild, GisMapValueChild, MapFeature};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};

//#region 🔹Snapshot
/// 📸️ Persisted GIS map document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gismap")]
pub struct GisMapSnapshot {
    #[state(artifact)]
    pub positions: Vec<MapFeature>,
    #[state(artifact)]
    pub routes: Vec<MapFeature>,
    #[state(artifact)]
    pub regions: Vec<MapFeature>,
    /// 🕸️ Composed `s.stdio.semio.drawing` child — see `crate::🦀️.rs`'s
    /// `🔖️Composition` region for the full stable-member design.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub drawing: GisMapDrawingChild,
    /// 🕸️ Composed `s.stdio.semio.image` child — always absent today (see `🔖️Composition`'s doc).
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<GisMapImageChild>,
    /// 🕸️ Composed `s.stdio.semio.value` child — the lossless `{positions,routes,regions}` mirror.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub value: GisMapValueChild,
}

impl Default for GisMapSnapshot {
    fn default() -> Self {
        Self { positions: Vec::new(), routes: Vec::new(), regions: Vec::new(), drawing: gis_map_drawing_child_handle(), image: None, value: gis_map_value_child_handle() }
    }
}
//#endregion 🔹Snapshot

//#region 🔖️CodecPrimitives















//#endregion 🔖️CodecPrimitives

//#region 🔖️TextPrimitives


//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives














//#endregion 🔖️BinaryPrimitives



//#region 🌉️IdentityBridge

//#endregion 🌉️IdentityBridge



