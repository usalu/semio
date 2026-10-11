use super::{WriterMainWindowConfig, WriterMainWindowConfigDiff, WriterMainWindowConfigMutation};
use crate::WriterEditorSettings;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "set-editor-settings")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEditorSettings {
    #[dsl(block)]
    pub settings: WriterEditorSettings,
}

impl protocol::MutationKind<WriterMainWindowConfig, WriterMainWindowConfigMutation> for SetEditorSettings {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "writer-window-editor-settings", kind: "set-editor-settings", record: "SetEditorSettings" };

    fn diff(&self, base: &WriterMainWindowConfig) -> protocol::MutationOutcome<WriterMainWindowConfigDiff> {
        protocol::MutationOutcome::new(WriterMainWindowConfigDiff { editor_settings: (base.editor_settings != self.settings).then(|| self.settings.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &WriterMainWindowConfig) -> Result<Vec<WriterMainWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { settings: base.editor_settings.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Writer Window Editor Settings", "Editoreinstellungen des Schreibfensters setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["editor_settings".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = WriterMainWindowConfig::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&WriterMainWindowConfigMutation::from(SetEditorSettings { settings: crate::WriterEditorSettings { show_line_numbers: false, font_px: 16, line_height: 24, tab_size: 4 } }), &base).await;
    }
}
