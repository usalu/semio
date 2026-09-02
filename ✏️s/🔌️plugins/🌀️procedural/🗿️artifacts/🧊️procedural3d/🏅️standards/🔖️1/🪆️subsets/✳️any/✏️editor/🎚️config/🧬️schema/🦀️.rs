//! 🧬️ schema leaf
use super::Procedural3dPreviewCamera;
use flow::CameraJson;
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.3d.config")]
pub struct Procedural3dConfig {
    #[state(config)]
    pub lod_mode: String,
    #[state(config)]
    pub show_mode: String,
    #[state(config)]
    pub camera: CameraJson,
    #[state(config)]
    pub preview_camera: Procedural3dPreviewCamera,
    #[state(config)]
    pub sun_json: String,
    #[state(config)]
    pub selected_generation_id: Option<String>,
    #[state(config)]
    pub generation_preview_text: Option<String>,
    #[state(config)]
    pub active_utility_id: String,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub preview_eval_text: Option<String>,
}

//region 📎 App-schema descriptor
/// 📎 Returns the `s.procedural.3d` app-schema descriptor for `ArtifactApp::app_schema`.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.procedural.3d",
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
