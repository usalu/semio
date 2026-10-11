//! 💡️ GIS terrain inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use crate::GisTerrainSnapshot;
use ::semio_framework_schema::ArtifactSchema;

use super::bounds::{imported_lon_lat_positions, lon_lat_bounds};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Inference
/// 💡️ Everything inferable from a gisterrain snapshot. Today: the geographic bounding box and
/// position count of the `map:in` overlay decoded from `imported_map` (see
/// `📦bounds/🦀️.rs`). A simple whole-snapshot scalar — no `InferredField` caching, the
/// overlay is small and re-decoding is O(positions).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gisterrain.inference")]
pub struct GisTerrainInference {
    #[derived]
    pub position_count: usize,
    #[derived]
    pub bounds: Option<GisTerrainBounds>,
}

impl protocol::Inference<GisTerrainSnapshot> for GisTerrainInference {
    fn infer(snapshot: &GisTerrainSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        let positions = imported_lon_lat_positions(snapshot);
        Self { position_count: positions.len(), bounds: lon_lat_bounds(&positions) }
    
        })
    }
}

impl protocol::InferenceSpec<GisTerrainSnapshot> for GisTerrainInference {
    fn inference_schema_id() -> &'static str {
        "s.gis.gisterrain.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.gis.gisterrain.inference.positionCount", reads: &["importedMap"] }, protocol::InferenceFieldSpec { id: "s.gis.gisterrain.inference.bounds", reads: &["importedMap"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️FixtureText
/// 🧭️ Relocated from the artifact's `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): `parse_descriptor` is a pure
/// snapshot → projection function (`&GisTerrainSnapshot` → `TerrainDescriptorJson`), matching the
/// `🧬️schema/💡️inferences/` destination — same family as `GisTerrainInference` above, just not yet
/// wired through the typed `#[derived]` registry.
///
/// ⚠️ `crate::modules::terrain` used to be a pre-existing unresolved import (predating this ticket,
/// flagged by another session — `crate::modules` was never mounted in this crate's `🦀️.rs`).
/// Fixed opportunistically while touching this file for the `mesh` composition (ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`): the real home of these types is
/// `crate::schema`'s `🔖️TerrainDescriptor` region — a one-line path
/// correction, not an engine-dissolution rewrite.
use crate::schema::{TerrainDescriptorJson, TerrainPositionData, TerrainProjectOrigin};
/// 📌️ Pin projection requires an identifier; durable map admission remains independent.
fn map_positions(map:&crate::schema::ImportedMap)->Vec<TerrainPositionData>{
    map.positions.iter().filter_map(|entry|Some(TerrainPositionData{id:entry.get("id")?.as_str()?.to_string(),lon:entry.get("lon")?.as_f64()?,lat:entry.get("lat")?.as_f64()?,label:entry.get("label").and_then(|value|value.as_str()).map(str::to_string),icon:entry.get("icon").and_then(|value|value.as_str()).map(str::to_string)})).collect()
}
/// 🔌️ The optional imported map contributes an independent pin overlay.
fn imported_positions(document: &GisTerrainSnapshot) -> Vec<TerrainPositionData> {
    let Some(map) = &document.imported_map else { return Vec::new(); };
    map_positions(map)
}

/// 🏔️ The full rendering descriptor (project origin + fixture pins + `map:in` overlay pins +
/// exaggeration) for the given document — `exaggeration` always mirrors the LIVE document, and the
/// bundled fixture's own `gisterrain exaggeration=...` header only ever seeds it once via
/// `crate::schema::default_terrain_document`.
pub fn parse_descriptor(document:&GisTerrainSnapshot,map:&crate::schema::ImportedMap)->TerrainDescriptorJson {
    let origin=map.properties.iter().find(|member|member.name=="projectOrigin").map(|member|&member.value);
    let project_origin=TerrainProjectOrigin{lon:origin.and_then(|value|value.get("lon")).and_then(|value|value.as_f64()).unwrap_or(0.0),lat:origin.and_then(|value|value.get("lat")).and_then(|value|value.as_f64()).unwrap_or(0.0)};
    let mut descriptor=TerrainDescriptorJson{schema:crate::GIS_3D_TERRAIN_SCHEMA.into(),project_origin,positions:map_positions(map),exaggeration:document.exaggeration};
    descriptor.positions.extend(imported_positions(document));
    descriptor
}
//#endregion 🔖️FixtureText

//#region 🔖️Descriptor
/// 💡️ Registers `s.gis.gisterrain.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `gisterrain_artifact_schema_descriptor`'s registration.
pub fn gisterrain_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.gis.gisterrain.inference",
        inference: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::bounds::GisTerrainBounds;
//#endregion 🔁️Re-exports
