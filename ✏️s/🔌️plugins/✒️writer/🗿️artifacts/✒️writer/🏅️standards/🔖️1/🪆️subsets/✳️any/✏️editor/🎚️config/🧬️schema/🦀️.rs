//! 🧬️ schema leaf
use crate::artifacts::writer::{WriterCamera, WriterEditorSelection, WriterEditorSettings};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.writer.writer.config")]
pub struct WriterConfig {
    #[state(config)]
    pub editor_selection: Option<WriterEditorSelection>,
    #[state(config)]
    pub format_signal: u32,
    #[state(config)]
    pub lint_signal: u32,
    #[state(config)]
    pub revision: u32,
    #[state(config)]
    pub editor_settings: WriterEditorSettings,
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub camera: WriterCamera,
    #[state(config)]
    pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 `s.writer.writer`'s config and presence schema, owned by this leaf.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.writer.writer",
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
