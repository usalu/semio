use super::{WriterMainWindowTransient, WriterMainWindowTransientDiff, WriterMainWindowTransientMutation, WriterOptionalSelection};
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
    fn diff(&self, base: &WriterMainWindowTransient) -> protocol::MutationOutcome<WriterMainWindowTransientDiff> {
        protocol::MutationOutcome::new(WriterMainWindowTransientDiff { editor_selection: (base.editor_selection != self.selection).then(|| WriterOptionalSelection { value: self.selection.clone() }), ..Default::default() })
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

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = WriterMainWindowTransient::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&WriterMainWindowTransientMutation::from(SetEditorSelection { selection: Some(crate::WriterEditorSelection { start: 1, end: 4, splice: 0 }) }), &base).await;
    }
}
