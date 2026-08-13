//! 🧬️ schema leaf
use flow::CameraJson;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.flow.flow.config")]
pub struct FlowConfig {
    #[state(config)] pub selected_node_ids: Vec<String>,
    #[state(config)] pub selected_edge_ids: Vec<String>,
    #[state(config)] pub selected_handle_ids: Vec<String>,
    #[state(config)] pub preview_off_node_ids: Vec<String>,
    #[state(config)] pub camera: CameraJson,
    #[state(config)] pub lod_mode: String,
    #[state(config)] pub proximity_distance: f64,
    #[state(config)] pub grid_visible: bool,
    #[state(config)] pub grid_snap_enabled: bool,
    #[state(config)] pub grid_factor: f64,
    #[state(config)] pub catalogue_sections_json: String,
    #[state(config)] pub automation_enabled_json: String,
    #[state(config)] pub contributions_json: String,
    #[state(config)] pub generation_json: String,
    #[state(config)] pub locale: String,
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

