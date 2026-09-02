//! 🧬️ schema leaf
use flow::CameraJson;
use schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
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
    pub duplicate_widget_progress_json: String,
    #[state(config)]
    pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 `s.flow.flow`'s config and presence schema, owned by this leaf.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.flow.flow",
        config: ::schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        presence: ::schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor
