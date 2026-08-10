//! 🧬️ Shooting artifact schema — every field of the artifact with its state class.

use crate::artifacts::shooting::{
    ShootingAsset, ShootingCamera, ShootingSavedCamera, ShootingSceneLighting, ShootingShot,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full shooting artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.shooting.shooting")]
pub struct ShootingArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub assets: Vec<ShootingAsset>,
    #[state(persistent)]
    pub saved_cameras: Vec<ShootingSavedCamera>,
    #[state(persistent)]
    pub scene: ShootingSceneLighting,
    #[state(persistent)]
    pub shots: Vec<ShootingShot>,
    #[state(persistent)]
    pub active_shot_id: String,
    #[state(persistent)]
    pub active_asset_id: String,
    #[state(shared_ui)]
    pub selected_shot_ids: Vec<String>,
    #[state(shared_ui)]
    pub selected_asset_ids: Vec<String>,
    #[state(shared_ui)]
    pub active_utility_id: String,
    #[state(local_ui)]
    pub default_shot_format: String,
    #[state(local_ui)]
    pub default_shot_shape: String,
    #[state(local_ui)]
    pub default_asset_format: String,
    #[state(local_ui)]
    pub selection_method: String,
    #[state(local_ui)]
    pub center_model: bool,
    #[state(local_ui)]
    pub fit_revision: u32,
    #[state(local_ui)]
    pub camera_draft_label: String,
    #[state(local_ui)]
    pub camera: ShootingCamera,
    #[state(local_ui)]
    pub locale: String,
    #[state(preview)]
    pub hovered_asset_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for ShootingArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::shooting::SHOOTING_DOCUMENT_SCHEMA.into(),
            assets: Vec::new(),
            saved_cameras: Vec::new(),
            scene: ShootingSceneLighting::default(),
            shots: Vec::new(),
            active_shot_id: String::new(),
            active_asset_id: String::new(),
            selected_shot_ids: Vec::new(),
            selected_asset_ids: Vec::new(),
            active_utility_id: "move".into(),
            default_shot_format: "png".into(),
            default_shot_shape: "rectangle".into(),
            default_asset_format: "glb".into(),
            selection_method: "rectangle".into(),
            center_model: true,
            fit_revision: 0,
            camera_draft_label: String::new(),
            camera: ShootingCamera::default(),
            locale: "en-US".into(),
            hovered_asset_id: None,
        }
    }
}

impl ShootingArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::shooting::ShootingSnapshot {
        crate::artifacts::shooting::ShootingSnapshot {
            schema: self.schema.clone(),
            assets: self.assets.clone(),
            saved_cameras: self.saved_cameras.clone(),
            scene: self.scene.clone(),
            shots: self.shots.clone(),
            active_shot_id: self.active_shot_id.clone(),
            active_asset_id: self.active_asset_id.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::shooting::ShootingSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            assets: snapshot.assets,
            saved_cameras: snapshot.saved_cameras,
            scene: snapshot.scene,
            shots: snapshot.shots,
            active_shot_id: snapshot.active_shot_id,
            active_asset_id: snapshot.active_asset_id,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::shooting::ShootingSnapshot) {
        self.schema = snapshot.schema;
        self.assets = snapshot.assets;
        self.saved_cameras = snapshot.saved_cameras;
        self.scene = snapshot.scene;
        self.shots = snapshot.shots;
        self.active_shot_id = snapshot.active_shot_id;
        self.active_asset_id = snapshot.active_asset_id;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.shooting.shooting` — twenty handcrafted schema leaves.
pub fn shooting_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.shooting.shooting",
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
