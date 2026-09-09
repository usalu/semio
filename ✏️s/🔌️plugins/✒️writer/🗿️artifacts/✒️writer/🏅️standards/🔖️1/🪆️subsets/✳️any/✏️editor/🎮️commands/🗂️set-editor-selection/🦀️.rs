//! 🗂️ 🗂️ Writer play app commands command — `set-editor-selection`.
//!
//! 🧬️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: this used to also resolve the
//! covering jack AST node (`jack_ast_node_for_selection`) and stage `SetSelectedAstIds` — that logic
//! DISSOLVES now that the framework owns the `ast` interaction domain (`HierarchyProvider::Topology`,
//! `selection.transitive = true`): the editor surface dispatches the framework's own `interactionSelect`
//! with the deepest AST node at the caret directly, and transitivity produces the covering behavior.
//! This command keeps only the raw, editor-intrinsic caret/range (`editor_selection` stays app-side,
//! never part of the `ast` domain).

use crate::op::WriterMutation;
use crate::WriterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "editor-selection")]
pub struct SetEditorSelection {
    pub start: usize,
    pub end: usize,
}

pub fn handle(_payload: &SetEditorSelection, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("writer editor selection requires the retained exact-window reducer"))
}
