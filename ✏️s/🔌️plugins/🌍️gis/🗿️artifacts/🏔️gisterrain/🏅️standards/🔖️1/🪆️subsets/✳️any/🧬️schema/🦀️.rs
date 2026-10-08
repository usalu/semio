//! 🧬️ GIS terrain artifact schema — every field of the artifact with its state class.

#[cfg(test)]
#[path = "🧪️tests/🪪️document/🦀️.rs"]
mod document_contract_tests;


use crate::{gis_terrain_mesh_child_handle, gis_terrain_mesh_content_key, GisTerrainSnapshot};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_surface::terrain::tiles;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Artifact
/// 🧬️ GIS terrain document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gisterrain")]
pub struct GisTerrainArtifact {
    #[state(artifact)]
    pub exaggeration: f64,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub imported_map: Option<ImportedMap>,
    /// 🕸️ Exact owned mesh handle preserved across artifact and snapshot conversions.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for GisTerrainArtifact {
    fn default() -> Self {
        Self { exaggeration: 0.0, imported_map: None, mesh: Some(gis_terrain_mesh_child_handle(&gis_terrain_mesh_content_key(0.0, None))) }
    }
}

impl GisTerrainArtifact {
    /// 📸️ Projects the persisted document without replacing any owned child identity.
    pub fn to_snapshot(&self) -> GisTerrainSnapshot {
        GisTerrainSnapshot { exaggeration: self.exaggeration, imported_map: self.imported_map.clone(), mesh: self.mesh.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: GisTerrainSnapshot) -> Self {
        Self { exaggeration: snapshot.exaggeration, imported_map: snapshot.imported_map, mesh: snapshot.mesh }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: GisTerrainSnapshot) {
        self.exaggeration = snapshot.exaggeration;
        self.imported_map = snapshot.imported_map;
        self.mesh = snapshot.mesh;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.gis.gisterrain` — twenty handcrafted schema leaves.
pub fn gisterrain_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.gis.gisterrain",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers



//#endregion 🔖️DocumentHelpers

//#region 🔖️TerrainDescriptor
/// 🧭️ Relocated from `⚙️engine/terrain/🦀️.rs` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): pure DTOs + a pure formatter, neither
/// snapshot-derived nor stateful — the `.gis.json` shape a `gis/3d` example is authored in, and the
/// `World3dScene.terrain_json` payload built from it for the `gis3d-play` app's terrain window.
///
/// 🧭️ Originally relocated out of the generic `framework_surface_terrain` engine (audit finding A5:
/// the framework must not know ✏️s — these DTOs and `build_terrain_scene_json` name gis-specific
/// concepts). The DEM-tile decode/session/mesh engine itself
/// (`semio_framework_surface::terrain::{tiles, projection, TerrainSessionCore}`) stays in the framework:
/// it is also path-mounted directly into `framework/os/infinite`'s `World3dState` (to dodge a
/// surface↔infinite cargo cycle) to drive the generic `World3d` terrain layer, so it is genuinely
/// shared rendering engine code, not gis-specific — only this descriptor/DTO layer belonged here.
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TerrainProjectOrigin {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TerrainPositionData {
    pub id: String,
    pub lon: f64,
    pub lat: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TerrainDescriptorJson {
    pub schema: String,
    pub project_origin: TerrainProjectOrigin,
    #[value(default)]
    pub positions: Vec<TerrainPositionData>,
    #[value(default = "default_exaggeration")]
    pub exaggeration: f64,
}

fn default_exaggeration() -> f64 {
    1.0
}

pub const GIS_3D_TERRAIN_TILE_URL_TEMPLATE: &str = "/dem/{z}/{x}/{y}.png";




//#endregion 🔖️TerrainDescriptor

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️relocated-engine/🦀️.rs"]
mod relocated_engine_tests;
//#endregion 🧪️Tests

#[path="🗺️imported-map/🦀️.rs"]
pub mod imported_map;
pub use imported_map::{ImportedMap,ImportedProperty};
