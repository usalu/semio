//! 🧬️ schema leaf
use flow::CameraJson;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.flow.flow.config")]
pub struct FlowConfig {
    #[state(config)]
    pub preview_off_node_ids: Vec<String>,
    #[state(config)]
    pub camera: CameraJson,
    #[state(config)]
    pub lod_mode: String,
    #[state(config)]
    pub proximity_distance: f64,
    #[state(config)]
    pub grid_visible: bool,
    #[state(config)]
    pub grid_snap_enabled: bool,
    #[state(config)]
    pub grid_factor: f64,
    #[state(config)]
    pub catalogue_sections_json: String,
    #[state(config)]
    pub automation_enabled_json: String,
    #[state(config)]
    pub contributions_json: String,
    #[state(config)]
    pub generation_json: String,
    #[state(config)]
    pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 `s.flow.flow`'s config and presence schema, owned by this leaf.
pub async fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
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
    }
}
//endregion 📎 App-schema descriptor
