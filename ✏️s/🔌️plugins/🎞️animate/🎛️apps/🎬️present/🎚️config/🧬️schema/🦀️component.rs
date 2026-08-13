//! 🧬️ Present app config schema — every local-ui field of PresentConfig.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🎚️ Animate present app config — unshared local app state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.animate.present.config")]
pub struct PresentConfig {
    #[state(config)] pub selected_ids: Vec<String>,
    #[state(config)] pub engagement_input: String,
    #[state(config)] pub locale: String,
}
//#endregion 🔖️Config

//region 📎 App-schema self-registration
/// 📎 Self-registers this app's schema descriptor into the open `AppSchemaRegistry`, mirroring the
/// same construction the framework's closed catalog previously hardcoded for `s.animate.present`.
pub fn register_app_schema() {
    ::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor {
        id: "s.animate.present",
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

