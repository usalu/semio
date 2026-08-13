//! 🧬️ schema leaf
use crate::artifacts::program::registers::AdjacencyKind;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.architect.architect.config")]
pub struct ArchitectConfig {
    #[state(config)] pub selected_ids: Vec<String>,
    #[state(config)] pub active_register: String,
    #[state(config)] pub search_query: String,
    #[state(config)] pub search_history_json: String,
    #[state(config)] pub active_report_json: String,
    #[state(config)] pub last_result_json: String,
    #[state(config)] pub last_analysis_json: String,
    #[state(config)] pub adjacency_kind_filter: Option<AdjacencyKind>,
    #[state(config)] pub graph_camera_x: f64,
    #[state(config)] pub graph_camera_y: f64,
    #[state(config)] pub graph_camera_zoom: f64,
}

//region 📎 App-schema self-registration
/// 📎 Self-registers this app's schema descriptor into the open `AppSchemaRegistry`, mirroring the
/// same construction the framework's closed catalog previously hardcoded for `s.architect.architect`.
pub fn register_app_schema() {
    ::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor {
        id: "s.architect.architect",
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

