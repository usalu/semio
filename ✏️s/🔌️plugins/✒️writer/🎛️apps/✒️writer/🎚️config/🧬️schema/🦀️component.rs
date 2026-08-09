//! 🧬️ schema leaf
use crate::artifacts::writer::{WriterCamera, WriterEditorSelection, WriterEditorSettings};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.writer.writer.config")]
pub struct WriterConfig {
    #[state(local_ui)] pub selected_ast_ids: Vec<String>,
    #[state(local_ui)] pub editor_selection: Option<WriterEditorSelection>,
    #[state(local_ui)] pub format_signal: u32,
    #[state(local_ui)] pub lint_signal: u32,
    #[state(local_ui)] pub revision: u32,
    #[state(local_ui)] pub editor_settings: WriterEditorSettings,
    #[state(local_ui)] pub tree_hovered_ast_id: Option<String>,
    #[state(local_ui)] pub editor_hover_offset: Option<usize>,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub camera: WriterCamera,
    #[state(local_ui)] pub locale: String,
}

//#region 🔖️AppSchemaDescriptor
/// 📚 Handcrafted app schema descriptor for this owner (config + presence facets).
pub fn app_schema_descriptor() -> schema::AppSchemaDescriptor {
    schema::AppSchemaDescriptor {
        id: "s.writer.writer",
        config: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        presence: schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️component.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️component.proto"),
        },
    }
}

/// 📎 Registers this owner's app schema into the OS-wide catalog.
pub fn register_app_schema() {
    schema::register_app_schema_descriptor(app_schema_descriptor());
}
//#endregion 🔖️AppSchemaDescriptor
