//! 🧬️ schema leaf
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.cad.cad.config")]
pub struct CadConfig {
    #[state(config)]
    pub selected_node_ids: Vec<String>,
    #[state(config)]
    pub hovered_reference_id: Option<String>,
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub engagement_step: String,
    #[state(config)]
    pub active_example_id: Option<String>,
    #[state(config)]
    pub selected_reference_model_definition_id: Option<String>,
    #[state(config)]
    pub selected_reference_id: Option<String>,
    #[state(config)]
    pub engagement_pane: Option<String>,
    #[state(config)]
    pub engagement_session_json: Option<String>,
    #[state(config)]
    pub engagement_preview_operation_json: Option<String>,
    /// 🔢️ Lossless cross-surface generation domain: `0..=2_147_483_647`.
    #[state(config)]
    pub engagement_preview_generation: i32,
    #[state(config)]
    pub last_finalized_interaction_id: Option<String>,
    #[state(config)]
    pub contributions_json: String,
}

//region 📎 App-schema descriptor
/// 📎 The `s.cad.cad` app-schema descriptor (config + presence facets) — returned, not
/// self-registered; `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1c) hands it to `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::framework_schema::AppSchemaDescriptor {
    ::framework_schema::AppSchemaDescriptor {
        id: "s.cad.cad",
        config: ::framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        presence: ::framework_schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor
