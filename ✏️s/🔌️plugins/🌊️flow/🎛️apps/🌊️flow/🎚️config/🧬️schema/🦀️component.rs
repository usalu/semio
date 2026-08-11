//! 🧬️ schema leaf
use flow::CameraJson;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.flow.flow.config")]
pub struct FlowConfig {
    #[state(local_ui)] pub selected_node_ids: Vec<String>,
    #[state(local_ui)] pub selected_edge_ids: Vec<String>,
    #[state(local_ui)] pub selected_handle_ids: Vec<String>,
    #[state(local_ui)] pub preview_off_node_ids: Vec<String>,
    #[state(local_ui)] pub camera: CameraJson,
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

//region 📎 App-schema self-registration
/// 📎 Registers the `s.flow.flow` app-schema descriptor (config + presence facets) into the
/// open [`::schema::AppSchemaRegistry`], mirroring the transplanted-from-framework closed-catalog
/// entry — see `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs::register_all_app_schema_descriptors()`.
pub fn register_app_schema() {
    ::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor {
        id: "s.flow.flow",
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
    });
}
//endregion 📎 App-schema self-registration

