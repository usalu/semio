//! 🦠️ `➕create-generation` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use crate::artifacts::generation2d::diff::Generation2dDiff;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use flow::playbook::FormGeneration;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateGeneration {
    pub generation: FormGeneration,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_generation(generation: FormGeneration) -> Generation2dMutation {
    Generation2dMutation::CreateGeneration(CreateGeneration { generation })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for CreateGeneration {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "generation", kind: "create-generation", record: "CreatedGeneration" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create generation \"{}\"", self.generation.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.generation.id.clone()]
    }
}
//#endregion 🔖️Mutation
