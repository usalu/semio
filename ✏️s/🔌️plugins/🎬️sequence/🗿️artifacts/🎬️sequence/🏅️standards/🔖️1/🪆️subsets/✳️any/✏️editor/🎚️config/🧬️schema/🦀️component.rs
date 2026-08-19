//! 🧬️ schema leaf
use crate::artifacts::sequence::SequenceCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
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
pub async fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.sequence.sequence",
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
