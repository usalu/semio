//! 🧬️ schema leaf
use crate::artifacts::writer::{WriterCamera, WriterEditorSelection, WriterEditorSettings};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.writer.writer.config")]
pub struct WriterConfig {
    #[state(config)] pub selected_ast_ids: Vec<String>,
    #[state(config)] pub editor_selection: Option<WriterEditorSelection>,
    #[state(config)] pub format_signal: u32,
    #[state(config)] pub lint_signal: u32,
    #[state(config)] pub revision: u32,
    #[state(config)] pub editor_settings: WriterEditorSettings,
    #[state(config)] pub tree_hovered_ast_id: Option<String>,
    #[state(config)] pub editor_hover_offset: Option<usize>,
    #[state(config)] pub engagement_input: String,
    #[state(config)] pub camera: WriterCamera,
    #[state(config)] pub locale: String,
}

//region 📎 App-schema self-registration
/// 📎 Registers `s.writer.writer`'s config+presence schema descriptor into the process-local registry.
pub fn register_app_schema() {
    ::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor {
        id: "s.writer.writer",
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

