//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.norm.config")]
pub struct NormConfig {
    #[state(config)]
    pub selected_check_index: Option<u32>,
}

//region 📎 App-schema descriptor
/// 📎 `s.norm.norm`'s config+presence schema descriptor — returned, not self-registered;
/// `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d) hands it to
/// `register_document_app` for registration. ONE descriptor, identical for all fifteen norm apps —
/// `NormConfig` (see `🎚️config/🦀️component.rs`'s doc) is the single shared `ArtifactApp::Config`
/// every standard's `PlayApp` uses, so every one of the fifteen `app_schema()` overrides returns this
/// same struct literal; `register_document_app` inserting the identical `id`/content fifteen times
/// into the OS-wide `HashMap<&'static str, _>` catalog is an idempotent overwrite, not a conflict.
pub async fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.norm.norm",
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
