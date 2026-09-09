use super::{WriterMainWindowTransient, WriterMainWindowTransientMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-lint-generation")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLintGeneration {
    pub value: u32,
}

impl protocol::MutationKind<WriterMainWindowTransient, WriterMainWindowTransientMutation> for SetLintGeneration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "writer-window-lint-generation", kind: "set-lint-generation", record: "SetLintGeneration" };
    fn diff(&self, base: &WriterMainWindowTransient) -> protocol::MutationOutcome<WriterMainWindowTransient> {
        let mut next = base.clone();
        next.lint_generation = self.value;
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &WriterMainWindowTransient) -> Vec<WriterMainWindowTransientMutation> {
        vec![Self { value: base.lint_generation }.into()]
    }
    fn label(&self) -> String {
        "Set Writer Window Lint Generation".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["lint_generation".into()]
    }
}
