//! 🧬️ GIS terrain artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full GIS terrain artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gisterrain")]
pub struct GisTerrainArtifact {
    #[state(persistent)] pub exaggeration: f64,
    #[state(persistent)] pub imported_features_json: String,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub camera_json: String,
    #[state(local_ui)] pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for GisTerrainArtifact {
    fn default() -> Self {
        Self {
            exaggeration: 0.0,
            imported_features_json: String::new(),
            selected_ids: Vec::new(),
            camera_json: serde_json::json!({ "position": [800.0, -800.0, 600.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "fov": 45.0 }).to_string(),
            locale: "en-US".into(),
        }
    }
}

impl GisTerrainArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::gisterrain::GisTerrainSnapshot {
        crate::artifacts::gisterrain::GisTerrainSnapshot {
            exaggeration: self.exaggeration,
            imported_features_json: self.imported_features_json.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::gisterrain::GisTerrainSnapshot) -> Self {
        Self {
            exaggeration: snapshot.exaggeration,
            imported_features_json: snapshot.imported_features_json,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::gisterrain::GisTerrainSnapshot) {
        self.exaggeration = snapshot.exaggeration;
        self.imported_features_json = snapshot.imported_features_json;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.gis.gisterrain` — twenty handcrafted schema leaves.
pub fn gisterrain_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.gis.gisterrain",
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
//#endregion 🔖️Descriptor
