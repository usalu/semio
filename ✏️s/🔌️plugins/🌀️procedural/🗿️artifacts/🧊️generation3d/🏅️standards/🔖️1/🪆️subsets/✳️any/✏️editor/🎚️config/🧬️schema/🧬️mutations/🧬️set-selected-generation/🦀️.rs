//! 🧬️ Sets which generation the generate mode's form and preview are bound to.

use super::{Generation3dConfig, Generation3dConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "selected-generation")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSelectedGeneration {
    pub selected_generation_id: Option<String>,
}

impl protocol::MutationKind<Generation3dConfig, Generation3dConfigMutation> for SetSelectedGeneration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "selected-generation", kind: "set-selected-generation", record: "SetSelectedGeneration" };

    fn diff(&self, base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfig> {
        let mut next = base.clone();
        next.selected_generation_id.clone_from(&self.selected_generation_id);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dConfig) -> Vec<Generation3dConfigMutation> {
        vec![Self { selected_generation_id: base.selected_generation_id.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Selected Generation".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["selectedGenerationId".into()]
    }
}
