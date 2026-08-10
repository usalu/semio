//! 🧬️ Process3d artifact schema — every field of the artifact with its state class.

use crate::artifacts::process3d::{ProcessStep, Stock, Workshop};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full process3d artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.process.process3d")]
pub struct Process3dArtifact {
    #[state(persistent)] pub workshop: Workshop,
    #[state(persistent)] pub stock: Stock,
    #[state(persistent)] pub steps: Vec<ProcessStep>,
    #[state(persistent)] pub resolved_up_to: Option<usize>,
    #[state(shared_ui)] pub selected_id: Option<String>,
    #[state(shared_ui)] pub selected_face_id: Option<usize>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub camera_position_x: f64,
    #[state(local_ui)] pub camera_position_y: f64,
    #[state(local_ui)] pub camera_position_z: f64,
    #[state(local_ui)] pub camera_target_x: f64,
    #[state(local_ui)] pub camera_target_y: f64,
    #[state(local_ui)] pub camera_target_z: f64,
    #[state(local_ui)] pub camera_fov: f64,
    #[state(local_ui)] pub sun_enabled: bool,
    #[state(local_ui)] pub sun_azimuth: f64,
    #[state(local_ui)] pub sun_elevation: f64,
    #[state(local_ui)] pub sun_intensity: f64,
    #[state(local_ui)] pub sun_color: String,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub contributions_json: String,
    #[state(preview)] pub hovered_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Process3dArtifact {
    fn default() -> Self {
        Self {
            workshop: Workshop::default(),
            stock: Stock::default(),
            steps: Vec::new(),
            resolved_up_to: None,
            selected_id: None,
            selected_face_id: None,
            active_utility_id: "select".into(),
            selection_method: "rectangle".into(),
            engagement_input: String::new(),
            camera_position_x: 3.0,
            camera_position_y: -3.0,
            camera_position_z: 2.0,
            camera_target_x: 0.0,
            camera_target_y: 0.0,
            camera_target_z: 0.0,
            camera_fov: 45.0,
            sun_enabled: false,
            sun_azimuth: 45.0,
            sun_elevation: 35.0,
            sun_intensity: 0.85,
            sun_color: "#ffffff".into(),
            locale: "en-US".into(),
            contributions_json: "[]".into(),
            hovered_id: None,
        }
    }
}

impl Process3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::process3d::Process3dSnapshot {
        crate::artifacts::process3d::Process3dSnapshot {
            workshop: self.workshop.clone(),
            stock: self.stock.clone(),
            steps: self.steps.clone(),
            resolved_up_to: self.resolved_up_to,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::process3d::Process3dSnapshot) -> Self {
        Self {
            workshop: snapshot.workshop,
            stock: snapshot.stock,
            steps: snapshot.steps,
            resolved_up_to: snapshot.resolved_up_to,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::process3d::Process3dSnapshot) {
        self.workshop = snapshot.workshop;
        self.stock = snapshot.stock;
        self.steps = snapshot.steps;
        self.resolved_up_to = snapshot.resolved_up_to;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.process.process3d` — twenty handcrafted schema leaves.
pub fn process3d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.process.process3d",
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
