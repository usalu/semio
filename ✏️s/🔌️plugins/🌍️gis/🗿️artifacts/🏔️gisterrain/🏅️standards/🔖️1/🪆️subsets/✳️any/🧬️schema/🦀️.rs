//! 🧬️ GIS terrain artifact schema — every field of the artifact with its state class.



use crate::{gis_terrain_mesh_child_handle, gis_terrain_mesh_content_key, GisTerrainSnapshot};
use ::semio_framework_schema::ArtifactSchema;
#[cfg(test)] use semio_framework_surface::terrain::tiles;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Artifact
/// 🧬️ GIS terrain document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
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
    pub mesh: Option<store::os_store::artifact_child::ArtifactChild<SemioMeshSnapshot>>,
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



//#region 🧪️Tests

//#endregion 🧪️Tests

pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::standards::v1::subsets::any::schema::terrain_descriptor;
pub use terrain_descriptor::*;

pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::imported_map;
pub use imported_map::{ImportedMap,ImportedProperty};
