//! 🧬️ Mathematical app config schema — every local-ui field of MathematicalConfig.

use crate::artifacts::mathematical::MathematicalCamera;
use schema::ArtifactSchema;
// 🌱️ Additive `ToValue`/`FromValue` — see `🦀️.rs`'s own docstring note on this crate's
// interim (not-yet-serde-free) state.
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Config
/// 🎚️ Mathematical app config — unshared local app state.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.mathematical.mathematical.config")]
pub struct MathematicalConfig {
    #[state(config)]
    pub camera: MathematicalCamera,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Config

//#region 🔖️Registration
/// 📎 `s.mathematical.mathematical`'s config+presence schema descriptor — returned, not
/// self-registered; `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1c) hands it to `register_document_app` for registration.
pub async fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.mathematical.mathematical",
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
//#endregion 🔖️Registration
