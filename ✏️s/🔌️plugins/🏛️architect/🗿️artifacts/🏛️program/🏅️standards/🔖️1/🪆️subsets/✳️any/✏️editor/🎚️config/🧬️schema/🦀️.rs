//! 🧬️ schema leaf
use crate::artifacts::program::registers::AdjacencyKind;
use schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.architect.architect.config")]
pub struct ArchitectConfig {
    #[state(config)]
    pub active_register: String,
    #[state(config)]
    pub search_query: String,
    #[state(config)]
    pub search_history_json: String,
    #[state(config)]
    pub active_report_json: String,
    #[state(config)]
    pub last_result_json: String,
    #[state(config)]
    pub last_analysis_json: String,
    #[state(config)]
    pub adjacency_kind_filter: Option<AdjacencyKind>,
    #[state(config)]
    pub graph_camera_x: f64,
    #[state(config)]
    pub graph_camera_y: f64,
    #[state(config)]
    pub graph_camera_zoom: f64,
}

//region 📎 App-schema descriptor
/// 📎 `s.architect.architect`'s config and presence schema, owned by this leaf.
pub async fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.architect.architect",
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
