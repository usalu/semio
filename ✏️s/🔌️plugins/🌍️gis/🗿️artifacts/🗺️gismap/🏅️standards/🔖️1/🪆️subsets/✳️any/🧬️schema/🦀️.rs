//! 🧬️ GIS map artifact schema — every field of the artifact with its state class.

pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::feature;

pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::drawing_bridge;
pub use drawing_bridge::*;


use crate::{GisMapSnapshot, MapFeature};
use crate::standards::v1::subsets::any::map_composition::{gis_map_snapshot_with_derived_children, GisMapDrawingChild, GisMapImageChild, GisMapValueChild};
use ::semio_framework_schema::ArtifactSchema;
#[cfg(test)] use semio_framework_value::ToValue;

use semio_framework_value::{DslValue,Number};

//#region 🔹Artifact
/// 🧬️ GIS map document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gismap")]
pub struct GisMapArtifact {
    #[state(artifact)]
    pub positions: Vec<MapFeature>,
    #[state(artifact)]
    pub routes: Vec<MapFeature>,
    #[state(artifact)]
    pub regions: Vec<MapFeature>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub drawing: GisMapDrawingChild,
    /// 🕸️ Mirrors `GisMapSnapshot.image` — see that field's own doc comment and
    /// `crate::🦀️.rs`'s `🔖️Composition` region. Carried verbatim (never
    /// derived) since, unlike `drawing`/`value`, nothing in this plugin can rebuild it from
    /// `positions`/`routes`/`regions` — dropping it silently on `from_snapshot`/`to_snapshot` would
    /// be a real, undocumented data loss the moment a future basemap-capture path populates it.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<GisMapImageChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub value: GisMapValueChild,
}
//#endregion 🔹Artifact

//#region 🔹Conversions


impl Default for GisMapArtifact {
    fn default() -> Self {
        Self::from_snapshot(GisMapSnapshot::default())
    }
}

impl GisMapArtifact {
    /// 📸️ Persisted subset with stable drawing/value member coordinates; their content is emitted
    /// as typed child work while `image` carries straight through.
    pub fn to_snapshot(&self) -> GisMapSnapshot {
        GisMapSnapshot { positions: self.positions.clone(), routes: self.routes.clone(), regions: self.regions.clone(), drawing: self.drawing.clone(), image: self.image.clone(), value: self.value.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: GisMapSnapshot) -> Self {
        Self { positions: snapshot.positions, routes: snapshot.routes, regions: snapshot.regions, drawing: snapshot.drawing, image: snapshot.image, value: snapshot.value }
    }

    /// 📤️ Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: GisMapSnapshot) {
        self.positions = snapshot.positions;
        self.routes = snapshot.routes;
        self.regions = snapshot.regions;
        self.drawing = snapshot.drawing;
        self.image = snapshot.image;
        self.value = snapshot.value;
    }
}
//#endregion 🔹Conversions

//#region 🔹Descriptor
/// 🧬️ Descriptor for `s.gis.gismap` — twenty handcrafted schema leaves.
pub fn gismap_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.gis.gismap",
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
//#endregion 🔹Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers


//#endregion 🔖️DocumentHelpers




//#region 🔖️MediaExport
//#endregion 🔖️MediaExport

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️relocated-engine/🦀️.rs"]
mod relocated_engine_tests;
//#endregion 🧪️Tests
