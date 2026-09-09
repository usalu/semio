//! 🧬️ Set Engagement Input in the presentation.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-engagement-input")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEngagementInput {
    pub value: String,
}

impl protocol::MutationKind<PresentationConfig, PresentationConfigMutation> for SetEngagementInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "engagement-input", kind: "set-engagement-input", record: "SetEngagementInput" };
    fn diff(&self, base: &PresentationConfig) -> protocol::MutationOutcome<PresentationConfig> {
        let mut next = base.clone();
        next.engagement_input = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &PresentationConfig) -> Vec<PresentationConfigMutation> {
        vec![PresentationConfigMutation::SetEngagementInput(Self { value: base.engagement_input.clone() })]
    }
    fn label(&self) -> String {
        "Set Engagement Input".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["engagementInput".into()]
    }
}
