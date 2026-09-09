use super::{WriterMainWindowTransient, WriterMainWindowTransientMutation};
use crate::WriterEditorSelection;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-editor-selection")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEditorSelection {
    #[dsl(block)]
    pub selection: Option<WriterEditorSelection>,
}

impl protocol::MutationKind<WriterMainWindowTransient, WriterMainWindowTransientMutation> for SetEditorSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "writer-window-editor-selection", kind: "set-editor-selection", record: "SetEditorSelection" };
    fn diff(&self, base: &WriterMainWindowTransient) -> protocol::MutationOutcome<WriterMainWindowTransient> {
        let mut next = base.clone();
        next.editor_selection.clone_from(&self.selection);
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &WriterMainWindowTransient) -> Vec<WriterMainWindowTransientMutation> {
        vec![Self { selection: base.editor_selection.clone() }.into()]
    }
    fn label(&self) -> String {
        "Set Writer Window Editor Selection".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["editor_selection".into()]
    }
}
