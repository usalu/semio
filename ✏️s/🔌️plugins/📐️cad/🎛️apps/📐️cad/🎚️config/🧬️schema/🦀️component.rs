//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadHoverTarget {
    pub object_id: Option<String>,
    pub mode: Option<String>,
    pub id: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadSelectionTargets {
    pub mesh: bool,
    pub vertex: bool,
    pub edge: bool,
    pub face: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadComponentSelection {
    pub targets: CadSelectionTargets,
    pub mode: String,
    pub ids: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadSunConfig {
    pub enabled: bool,
    pub azimuth: f64,
    pub elevation: f64,
    pub intensity: f64,
    pub color: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadProjectionDsl {
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadCamera {
    pub position: [f64; 3],
    pub target: [f64; 3],
    pub zoom: f64,
    pub fov: f64,
    pub projection: CadProjectionDsl,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadDislocateOptions {
    pub move_enabled: bool,
    pub rotate_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.cad.cad.config")]
pub struct CadConfig {
    #[state(local_ui)] pub selected_object_ids: Vec<String>,
    #[state(local_ui)] pub selected_node_ids: Vec<String>,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub hovered_object_id: Option<String>,
    #[state(local_ui)] pub hovered_target: Option<CadHoverTarget>,
    #[state(local_ui)] pub active_object_id: Option<String>,
    #[state(local_ui)] pub component_selection: CadComponentSelection,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub engagement_step: String,
    #[state(local_ui)] pub active_example_id: Option<String>,
    #[state(local_ui)] pub selected_reference_model_definition_id: Option<String>,
    #[state(local_ui)] pub selected_reference_id: Option<String>,
    #[state(local_ui)] pub selected_primitive_id: Option<String>,
    #[state(local_ui)] pub selected_primitive_kind: Option<String>,
    #[state(local_ui)] pub engagement_pane: Option<String>,
    #[state(local_ui)] pub engagement_session_json: Option<String>,
    #[state(local_ui)] pub last_finalized_interaction_id: Option<String>,
    #[state(local_ui)] pub sun: CadSunConfig,
    #[state(local_ui)] pub camera: CadCamera,
    #[state(local_ui)] pub camera_building: CadCamera,
    #[state(local_ui)] pub camera_energy: CadCamera,
    #[state(local_ui)] pub camera_structure_classic: CadCamera,
    #[state(local_ui)] pub dislocate_shape: CadDislocateOptions,
    #[state(local_ui)] pub dislocate_building: CadDislocateOptions,
    #[state(local_ui)] pub dislocate_energy: CadDislocateOptions,
    #[state(local_ui)] pub dislocate_structure_classic: CadDislocateOptions,
    #[state(local_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub terminology: String,
    #[state(local_ui)] pub contributions_json: String,
}

//region 📎 App-schema descriptor
/// 📎 The `s.cad.cad` app-schema descriptor (config + presence facets) — returned, not
/// self-registered; `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1c) hands it to `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.cad.cad",
        config: ::schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        presence: ::schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️component.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️component.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor

