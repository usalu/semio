//! 🧬️ schema leaf
use flow::CameraJson;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.2d.config")]
pub struct Procedural2dConfig {
    #[state(config)]
    pub camera: CameraJson,
    #[state(config)]
    pub show_mode: String,
    #[state(config)]
    pub selected_generation_id: Option<String>,
    #[state(config)]
    pub generation_preview_text: Option<String>,
    #[state(config)]
    pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 Returns the `s.procedural.2d` app-schema descriptor for `ArtifactApp::app_schema`.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.procedural.2d",
        config: ::schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        presence: ::schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor
