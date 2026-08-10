//! 🧬️ Flow artifact schema — every field of the artifact with its state class.

use crate::artifacts::flow::FlowSnapshot;
use flow::{CameraJson, SynapseSpec, Widget, WidgetLayout, FLOW_LOD_MODE_AUTOMATIC};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔹Artifact
/// 🧬️ Full flow artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.flow.flow")]
pub struct FlowArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub camera: CameraJson,
    #[state(persistent)] pub widgets: Vec<Widget>,
    #[state(persistent)] pub synapses: Vec<SynapseSpec>,
    #[state(persistent)] pub layout: BTreeMap<String, WidgetLayout>,
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(shared_ui)] pub selected_edge_ids: Vec<String>,
    #[state(shared_ui)] pub selected_handle_ids: Vec<String>,
    #[state(shared_ui)] pub preview_off_node_ids: Vec<String>,
    #[state(local_ui)] pub lod_mode: String,
    #[state(local_ui)] pub proximity_distance: f64,
    #[state(local_ui)] pub grid_visible: bool,
    #[state(local_ui)] pub grid_snap_enabled: bool,
    #[state(local_ui)] pub grid_factor: f64,
    #[state(local_ui)] pub catalogue_sections_json: String,
    #[state(local_ui)] pub automation_enabled_json: String,
    #[state(local_ui)] pub contributions_json: String,
    #[state(local_ui)] pub generation_json: String,
    #[state(local_ui)] pub locale: String,
}
//#endregion 🔹Artifact

//#region 🔹Conversions
impl Default for FlowArtifact {
    fn default() -> Self {
        Self::from_snapshot(FlowSnapshot::default())
    }
}

impl FlowArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> FlowSnapshot {
        FlowSnapshot {
            schema: self.schema.clone(),
            camera: self.camera.clone(),
            widgets: self.widgets.clone(),
            synapses: self.synapses.clone(),
            layout: self.layout.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: FlowSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            camera: snapshot.camera,
            widgets: snapshot.widgets,
            synapses: snapshot.synapses,
            layout: snapshot.layout,
            selected_node_ids: Vec::new(),
            selected_edge_ids: Vec::new(),
            selected_handle_ids: Vec::new(),
            preview_off_node_ids: Vec::new(),
            lod_mode: FLOW_LOD_MODE_AUTOMATIC.into(),
            proximity_distance: crate::artifacts::flow::engine::FLOW_DEFAULT_PROXIMITY_DISTANCE,
            grid_visible: true,
            grid_snap_enabled: false,
            grid_factor: crate::artifacts::flow::engine::FLOW_DEFAULT_GRID_FACTOR,
            catalogue_sections_json: "[]".into(),
            automation_enabled_json: String::new(),
            contributions_json: "[]".into(),
            generation_json: String::new(),
            locale: "en-US".into(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: FlowSnapshot) {
        self.schema = snapshot.schema;
        self.camera = snapshot.camera;
        self.widgets = snapshot.widgets;
        self.synapses = snapshot.synapses;
        self.layout = snapshot.layout;
    }
}
//#endregion 🔹Conversions

//#region 🔹Descriptor
/// 🧬️ Descriptor for `s.flow.flow` — twenty handcrafted schema leaves.
pub fn flow_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.flow.flow",
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
