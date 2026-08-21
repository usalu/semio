//! 🧬️ schema leaf
use crate::artifacts::shooting::ShootingCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.shooting.shooting.config")]
pub struct ShootingConfig {
    #[state(config)]
    pub default_shot_format: String,
    #[state(config)]
    pub default_shot_shape: String,
    #[state(config)]
    pub default_asset_format: String,
    #[state(config)]
    pub selected_shot_ids: Vec<String>,
    #[state(config)]
    pub center_model: bool,
    #[state(config)]
    pub fit_revision: u32,
    #[state(config)]
    pub camera_draft_label: String,
    #[state(config)]
    pub camera: ShootingCamera,
    #[state(config)]
    pub active_utility_id: String,
    #[state(config)]
    pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 The `s.shooting.shooting` app-schema descriptor (config + presence facets) — returned, not
/// self-registered; `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1c) hands it to `register_document_app` for registration.
pub async fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.shooting.shooting",
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
