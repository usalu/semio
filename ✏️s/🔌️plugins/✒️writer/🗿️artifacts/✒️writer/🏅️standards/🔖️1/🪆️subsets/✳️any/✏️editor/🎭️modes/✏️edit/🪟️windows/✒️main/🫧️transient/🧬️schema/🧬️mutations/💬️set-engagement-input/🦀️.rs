use super::{WriterMainWindowTransient, WriterMainWindowTransientMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-engagement-input")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEngagementInput {
    pub value: String,
}

impl protocol::MutationKind<WriterMainWindowTransient, WriterMainWindowTransientMutation> for SetEngagementInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "writer-window-engagement-input", kind: "set-engagement-input", record: "SetEngagementInput" };
    fn diff(&self, base: &WriterMainWindowTransient) -> protocol::MutationOutcome<WriterMainWindowTransient> {
        let mut next = base.clone();
        next.engagement_input.clone_from(&self.value);
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &WriterMainWindowTransient) -> Vec<WriterMainWindowTransientMutation> {
        vec![Self { value: base.engagement_input.clone() }.into()]
    }
    fn label(&self) -> String {
        "Set Writer Window Engagement Input".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["engagement_input".into()]
    }
}
