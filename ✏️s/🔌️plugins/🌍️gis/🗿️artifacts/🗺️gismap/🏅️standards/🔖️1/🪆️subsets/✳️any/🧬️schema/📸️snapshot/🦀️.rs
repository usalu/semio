//! 🧬️ GIS map snapshot schema — artifact-lane fields only.
//!
//! Typed native records preserve intrinsic feature values and durable `drawing`/`image`/`value`
//! child identities through the shared Text, Pack and SQLite projection in
//! [`crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack`].

use crate::{gis_map_drawing_child_handle, gis_map_value_child_handle, GisMapDrawingChild, GisMapImageChild, GisMapValueChild, MapFeature};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;


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


