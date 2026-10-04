use super::{WriterMainWindowTransient, WriterMainWindowTransientMutation};
use crate::WriterEditorSelection;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
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
    fn inverse(&self, base: &WriterMainWindowTransient) -> Result<Vec<WriterMainWindowTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { selection: base.editor_selection.clone() }.into()]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Writer Window Editor Selection", "Editorauswahl des Schreibfensters setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["editor_selection".into()]
    }
}
