//! 🧬️ schema leaf
use crate::artifacts::layout::{LayoutCamera, LayoutDropPreviewState};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.layout.layout.config")]
pub struct LayoutConfig {
    #[state(config)]
    pub active_page_id: String,
    #[state(config)]
    pub drop_preview: LayoutDropPreviewState,
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub camera: LayoutCamera,
    #[state(config)]
    pub preview_camera: LayoutCamera,
    #[state(config)]
    pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 The `s.layout.layout` app-schema descriptor (config + presence facets) — returned, not
/// self-registered; `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1c) hands it to `register_document_app` for registration.
pub async fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.layout.layout",
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
