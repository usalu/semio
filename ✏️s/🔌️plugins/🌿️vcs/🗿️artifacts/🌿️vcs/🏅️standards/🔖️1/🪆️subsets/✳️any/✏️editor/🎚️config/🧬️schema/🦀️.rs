//! 🧬️ schema leaf
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.vcs.vcs.config")]
pub struct VcsDemoConfig {}

//region 📎 App-schema descriptor
/// 📎 `s.vcs.vcs`'s config and presence schema, owned by this leaf.
pub fn app_schema_descriptor() -> ::semio_framework_schema_registry::AppSchemaDescriptor {
    ::semio_framework_schema_registry::AppSchemaDescriptor {
        id: "s.vcs.vcs",
        config: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        presence: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor
