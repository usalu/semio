//! 🧬️ Writer artifact schema — every field with its state class.

use crate::artifacts::writer::{WriterEditorSelection, WriterEditorSettings, WRITER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full writer artifact across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.writer.writer")]
pub struct WriterArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub id: String,
    #[state(persistent)] pub language_id: String,
    #[state(persistent)] pub uri: String,
    #[state(persistent)] pub text: String,
    #[state(shared_ui)] pub selected_ast_ids: Vec<String>,
    #[state(shared_ui)] pub editor_selection: Option<WriterEditorSelection>,
    #[state(shared_ui)] pub editor_settings: WriterEditorSettings,
    #[state(local_ui)] pub format_signal: u32,
    #[state(local_ui)] pub lint_signal: u32,
    #[state(local_ui)] pub revision: u32,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub camera_x: f64,
    #[state(local_ui)] pub camera_y: f64,
    #[state(local_ui)] pub camera_zoom: f64,
    #[state(local_ui)] pub locale: String,
    #[state(preview)] pub tree_hovered_ast_id: Option<String>,
    #[state(preview)] pub editor_hover_offset: Option<usize>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for WriterArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::writer::WriterSnapshot::default())
    }
}

impl WriterArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::writer::WriterSnapshot {
        crate::artifacts::writer::WriterSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            language_id: self.language_id.clone(),
            uri: self.uri.clone(),
            text: self.text.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot with UI defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::writer::WriterSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            language_id: snapshot.language_id,
            uri: snapshot.uri,
            text: snapshot.text,
            ..Self::default_ui()
        }
    }

    fn default_ui() -> Self {
        Self {
            schema: WRITER_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            language_id: "plaintext".into(),
            uri: crate::artifacts::writer::default_uri(),
            text: String::new(),
            selected_ast_ids: Vec::new(),
            editor_selection: None,
            editor_settings: WriterEditorSettings::default(),
            format_signal: 0,
            lint_signal: 0,
            revision: 0,
            engagement_input: String::new(),
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            locale: "en-US".into(),
            tree_hovered_ast_id: None,
            editor_hover_offset: None,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::writer::WriterSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.language_id = snapshot.language_id;
        self.uri = snapshot.uri;
        self.text = snapshot.text;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.writer.writer` — fifteen handcrafted schema leaves.
pub fn writer_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.writer.writer",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
