//! 🗂️ 🗂️ Writer play app commands command — `set-editor-selection`.
//!
//! 🧬️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: this used to also resolve the
//! covering jack AST node (`jack_ast_node_for_selection`) and stage `SetSelectedAstIds` — that logic
//! DISSOLVES now that the framework owns the `ast` interaction domain (`HierarchyProvider::Topology`,
//! `selection.transitive = true`): the editor surface dispatches the framework's own `interactionSelect`
//! with the deepest AST node at the caret directly, and transitivity produces the covering behavior.
//! This command keeps only the raw, editor-intrinsic caret/range (`editor_selection` stays app-side,
//! never part of the `ast` domain).

use crate::editor::writer::config::{WriterConfig, WriterConfigMutation, WriterEditorSelection};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "editor-selection")]
pub struct SetEditorSelection {
    pub start: usize,
    pub end: usize,
}

pub async fn handle(payload: &SetEditorSelection, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let config = cfg.snapshot;
    Ok(Emit::config(vec![
        WriterConfigMutation::SetEditorSelection { selection: Some(WriterEditorSelection { start: payload.start, end: payload.end }) },
        WriterConfigMutation::SetRevision { value: config.revision + 1 },
    ]))
}
