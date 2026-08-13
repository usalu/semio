//! 🧬️ schema leaf
use crate::artifacts::layout::{LayoutCamera, LayoutDropPreviewState};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.layout.layout.config")]
pub struct LayoutConfig {
    #[state(config)] pub active_page_id: String,
    #[state(config)] pub selected_ids: Vec<String>,
    #[state(config)] pub hovered_id: Option<String>,
    #[state(config)] pub drop_preview: LayoutDropPreviewState,
    #[state(config)] pub engagement_input: String,
    #[state(config)] pub camera: LayoutCamera,
    #[state(config)] pub preview_camera: LayoutCamera,
    #[state(config)] pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 The `s.layout.layout` app-schema descriptor (config + presence facets) — returned, not
/// self-registered; `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1c) hands it to `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.layout.layout",
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

