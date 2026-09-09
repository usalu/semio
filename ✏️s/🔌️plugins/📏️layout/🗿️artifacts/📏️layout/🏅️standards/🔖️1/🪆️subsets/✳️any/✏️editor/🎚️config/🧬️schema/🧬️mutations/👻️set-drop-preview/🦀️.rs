//! 🧬️ Set Drop Preview in the layout.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-drop-preview")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDropPreview {
    #[dsl(block)]
    pub preview: LayoutDropPreviewState,
}

impl protocol::MutationKind<LayoutConfig, LayoutConfigMutation> for SetDropPreview {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "drop-preview", kind: "set-drop-preview", record: "SetDropPreview" };
    fn diff(&self, base: &LayoutConfig) -> protocol::MutationOutcome<LayoutConfig> {
        let mut next = base.clone();
        next.drop_preview = self.preview.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &LayoutConfig) -> Vec<LayoutConfigMutation> {
        vec![LayoutConfigMutation::SetDropPreview(Self { preview: base.drop_preview.clone() })]
    }
    fn label(&self) -> String {
        "Set Drop Preview".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["drop_preview".into()]
    }
}
