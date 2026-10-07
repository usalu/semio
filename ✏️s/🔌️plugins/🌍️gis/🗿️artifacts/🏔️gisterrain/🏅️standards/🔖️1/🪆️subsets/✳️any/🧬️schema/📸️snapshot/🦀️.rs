//! 🧬️ GIS terrain snapshot schema — artifact-lane fields only.
//!
//! P6 handcrafted `ArtifactDsl`/`ArtifactPack` (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`):
//! `GisTerrainSnapshot` now carries a real `store::ArtifactChild<SemioMeshSnapshot>` handle for its
//! `mesh` slot, which `dsl::DslRecord`'s derive cannot represent (no `DslField` impl for
//! `ArtifactChild<S>`) — the same reason `✳️object`/`✳️kit` (stdio) and `💠️lowpoly`/`📐️cad` hand-roll
//! their own codecs rather than deriving. This file follows their exact hex/bracket convention, never
//! a hand-written slot list — `#[derive(ArtifactSchema)]` still emits `field_states()`/the `#[child(…)]`
//! slot table for the top-level facets.

use crate::{gis_terrain_mesh_child_handle, gis_terrain_mesh_content_key};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;

use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔹Snapshot
/// 📸️ Persisted GIS terrain document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gisterrain")]
pub struct GisTerrainSnapshot {
    #[state(artifact)]
    pub exaggeration: f64,
    /// 🔌️ `map:in`'s insertion point — last-imported `2d.map` descriptor JSON.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub imported_map: Option<crate::schema::ImportedMap>,
    /// 🪆️ Exact independently owned mesh handle, preserved by parent scalar edits.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>,
}

impl Default for GisTerrainSnapshot {
    fn default() -> Self {
        let mesh = Some(gis_terrain_mesh_child_handle(&gis_terrain_mesh_content_key(0.0, None)));
        Self { exaggeration: 0.0, imported_map: None, mesh }
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

#[path="🧮️row-admission/🦀️.rs"]
mod row_admission;
#[path="🧮️value-admission/🦀️.rs"]
mod value_admission;

