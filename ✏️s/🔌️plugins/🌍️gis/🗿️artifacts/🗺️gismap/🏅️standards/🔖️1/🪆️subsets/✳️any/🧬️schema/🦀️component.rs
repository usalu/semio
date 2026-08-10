//! 🧬️ GIS map artifact schema — every field of the artifact with its state class.

use crate::artifacts::gismap::MapFeature;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔹Artifact
/// 🧬️ Full GIS map artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gismap")]
pub struct GisMapArtifact {
    #[state(persistent)] pub positions: Vec<MapFeature>,
    #[state(persistent)] pub routes: Vec<MapFeature>,
    #[state(persistent)] pub regions: Vec<MapFeature>,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub feature_selection_json: String,
    #[state(shared_ui)] pub layer_visibility: BTreeMap<String, bool>,
    #[state(shared_ui)] pub layer_stroke_scale: BTreeMap<String, f64>,
    #[state(local_ui)] pub camera_json: String,
    #[state(local_ui)] pub render_mode: String,
    #[state(local_ui)] pub vector_style: String,
    #[state(local_ui)] pub lod_mode: String,
    #[state(local_ui)] pub hover_json: String,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub selection_mode: String,
    #[state(local_ui)] pub locale: String,
}
//#endregion 🔹Artifact

//#region 🔹Conversions
impl Default for GisMapArtifact {
    fn default() -> Self {
        Self {
            positions: Vec::new(),
            routes: Vec::new(),
            regions: Vec::new(),
            selected_ids: Vec::new(),
            feature_selection_json: r#"{"positions":[],"routes":[]}"#.into(),
            layer_visibility: BTreeMap::new(),
            layer_stroke_scale: BTreeMap::new(),
            camera_json: r#"{"x":0,"y":0,"zoom":1}"#.into(),
            render_mode: "combined".into(),
            vector_style: "colored".into(),
            lod_mode: "automatic".into(),
            hover_json: "null".into(),
            selection_method: "rectangle".into(),
            selection_mode: "default".into(),
            locale: "en-US".into(),
        }
    }
}

impl GisMapArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::gismap::GisMapSnapshot {
        crate::artifacts::gismap::GisMapSnapshot {
            positions: self.positions.clone(),
            routes: self.routes.clone(),
            regions: self.regions.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::gismap::GisMapSnapshot) -> Self {
        Self {
            positions: snapshot.positions,
            routes: snapshot.routes,
            regions: snapshot.regions,
            ..Self::default()
        }
    }

    /// Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::gismap::GisMapSnapshot) {
        self.positions = snapshot.positions;
        self.routes = snapshot.routes;
        self.regions = snapshot.regions;
    }
}
//#endregion 🔹Conversions

//#region 🔹Descriptor
/// 🧬️ Descriptor for `s.gis.gismap` — twenty handcrafted schema leaves.
pub fn gismap_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.gis.gismap",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔹Descriptor
