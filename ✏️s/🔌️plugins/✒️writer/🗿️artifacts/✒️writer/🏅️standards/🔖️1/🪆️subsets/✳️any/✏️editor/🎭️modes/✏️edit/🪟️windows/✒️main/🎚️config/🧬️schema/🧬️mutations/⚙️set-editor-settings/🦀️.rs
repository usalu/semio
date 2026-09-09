use super::{WriterMainWindowConfig, WriterMainWindowConfigMutation};
use crate::WriterEditorSettings;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
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

    fn inverse(&self, base: &WriterMainWindowConfig) -> Vec<WriterMainWindowConfigMutation> {
        vec![Self { settings: base.editor_settings.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Writer Window Editor Settings".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["editor_settings".into()]
    }
}
