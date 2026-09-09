//! 🧬️ Set Active Page in the layout.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-active-page")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetActivePage {
    pub page_id: String,
}

impl protocol::MutationKind<LayoutConfig, LayoutConfigMutation> for SetActivePage {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "active-page", kind: "set-active-page", record: "SetActivePage" };
    fn diff(&self, base: &LayoutConfig) -> protocol::MutationOutcome<LayoutConfig> {
        let mut next = base.clone();
        next.active_page_id = self.page_id.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &LayoutConfig) -> Vec<LayoutConfigMutation> {
        vec![LayoutConfigMutation::SetActivePage(Self { page_id: base.active_page_id.clone() })]
    }
    fn label(&self) -> String {
        "Set Active Page".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["active_page_id".into()]
    }
}
