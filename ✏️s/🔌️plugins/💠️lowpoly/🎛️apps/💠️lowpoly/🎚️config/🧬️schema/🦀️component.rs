//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.lowpoly.lowpoly.config")]
pub struct LowpolyConfig {
    #[state(local_ui)] pub active_object_id: String,
    #[state(local_ui)] pub selection_mode: String,
    #[state(local_ui)] pub selection_ids: Vec<u32>,
    #[state(local_ui)] pub selection_targets_mesh: bool,
    #[state(local_ui)] pub selection_targets_vertex: bool,
    #[state(local_ui)] pub selection_targets_edge: bool,
    #[state(local_ui)] pub selection_targets_face: bool,
    #[state(local_ui)] pub selection_keys: Vec<String>,
    #[state(local_ui)] pub paint_utility: String,
    #[state(local_ui)] pub active_paint_layer: u32,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub selection_mode_default: String,
    #[state(local_ui)] pub selected_object_ids: Vec<String>,
    #[state(local_ui)] pub hovered_object_id: Option<String>,
    #[state(local_ui)] pub hovered_target_object_id: Option<String>,
    #[state(local_ui)] pub hovered_target_mode: Option<String>,
    #[state(local_ui)] pub hovered_target_id: Option<u32>,
    #[state(local_ui)] pub utility_params_json: String,
    #[state(local_ui)] pub paint_color_r: u8,
    #[state(local_ui)] pub paint_color_g: u8,
    #[state(local_ui)] pub paint_color_b: u8,
    #[state(local_ui)] pub paint_color_a: u8,
    #[state(local_ui)] pub world_camera_position: [f64; 3],
    #[state(local_ui)] pub world_camera_target: [f64; 3],
    #[state(local_ui)] pub world_camera_fov: f64,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub show_edges: bool,
    #[state(local_ui)] pub sun_enabled: bool,
    #[state(local_ui)] pub sun_azimuth: f64,
    #[state(local_ui)] pub sun_elevation: f64,
    #[state(local_ui)] pub sun_intensity: f64,
    #[state(local_ui)] pub sun_color: String,
    #[state(local_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 `s.lowpoly.lowpoly`'s config+presence schema descriptor — returned, not self-registered;
/// `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to
/// `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.lowpoly.lowpoly",
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

