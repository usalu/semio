use super::{WriterMainWindowConfig, WriterMainWindowConfigMutation};
use crate::WriterEditorSettings;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-editor-settings")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEditorSettings {
    #[dsl(block)]
    pub settings: WriterEditorSettings,
}

impl protocol::MutationKind<WriterMainWindowConfig, WriterMainWindowConfigMutation> for SetEditorSettings {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "writer-window-editor-settings", kind: "set-editor-settings", record: "SetEditorSettings" };

    fn diff(&self, base: &WriterMainWindowConfig) -> protocol::MutationOutcome<WriterMainWindowConfig> {
        let mut next = base.clone();
        next.editor_settings.clone_from(&self.settings);
        protocol::MutationOutcome::new(next)
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
