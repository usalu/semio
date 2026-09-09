//! 🧬️ schema leaf
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.generation.2d.config")]
pub struct Generation2dConfig {
    #[state(config)]
    pub camera: CameraJson,
    #[state(config)]
    pub show_mode: String,
    #[state(config)]
    pub selected_generation_id: Option<String>,
}

//region 📎 App-schema descriptor
/// 📎 Returns the `s.generation.2d` app-schema descriptor for `ArtifactApp::app_schema`.
pub fn app_schema_descriptor() -> ::semio_framework_schema::AppSchemaDescriptor {
    ::semio_framework_schema::AppSchemaDescriptor {
        id: "s.generation.2d",
        config: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        presence: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor
