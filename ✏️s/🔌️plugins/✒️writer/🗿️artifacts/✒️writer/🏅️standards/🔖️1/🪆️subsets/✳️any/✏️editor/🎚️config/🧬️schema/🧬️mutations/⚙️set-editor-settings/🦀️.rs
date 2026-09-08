//! 🧬️ Set Editor Settings in the Writer config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-editor-settings")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEditorSettings {
    #[dsl(block)]
    pub settings: WriterEditorSettings,
}

impl protocol::MutationKind<WriterConfig, WriterConfigMutation> for SetEditorSettings {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "editor-settings", kind: "set-editor-settings", record: "SetEditorSettings" };
    fn diff(&self, base: &WriterConfig) -> protocol::MutationOutcome<WriterConfig> {
        let mut next = base.clone();
        next.editor_settings = self.settings.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &WriterConfig) -> Vec<WriterConfigMutation> { vec![WriterConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Editor Settings".into() }
    fn target(&self) -> Vec<String> { vec!["editor_settings".into()] }
}
