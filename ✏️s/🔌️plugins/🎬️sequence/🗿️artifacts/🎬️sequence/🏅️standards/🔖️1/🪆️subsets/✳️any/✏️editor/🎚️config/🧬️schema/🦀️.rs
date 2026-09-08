//! 🧬️ schema leaf
use crate::SequenceCamera;
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.sequence.sequence.config")]
pub struct SequenceConfig {
    #[state(config)]
    pub last_run_json: String,
    #[state(config)]
    pub orientation: String,
    #[state(config)]
    pub camera: SequenceCamera,
    #[state(config)]
    pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 `s.sequence.sequence`'s config and presence schema, owned by this leaf.
pub fn app_schema_descriptor() -> ::framework_schema::AppSchemaDescriptor {
    ::framework_schema::AppSchemaDescriptor {
        id: "s.sequence.sequence",
        config: ::framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
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
