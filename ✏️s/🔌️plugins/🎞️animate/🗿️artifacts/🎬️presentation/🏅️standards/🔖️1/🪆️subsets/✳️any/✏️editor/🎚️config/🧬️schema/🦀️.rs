//! 🧬️ Presentation app config schema — every local-ui field of PresentationConfig.

use schema::ArtifactSchema;

//#region 🔖️Config
/// 🎚️ Animate presentation app config — unshared local app state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.animate.presentation.config")]
pub struct PresentationConfig {
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Config

//region 📎 App-schema descriptor
/// 📎 `s.animate.presentation`'s config and presence schema, owned by this leaf.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.animate.presentation",
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
