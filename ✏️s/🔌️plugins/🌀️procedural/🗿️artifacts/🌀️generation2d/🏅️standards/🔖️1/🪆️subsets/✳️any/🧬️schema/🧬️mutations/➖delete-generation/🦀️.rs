//! 🦠️ `➖delete-generation` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use crate::artifacts::generation2d::diff::Generation2dDiff;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteGeneration {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_generation(id: String) -> Generation2dMutation {
    Generation2dMutation::DeleteGeneration(DeleteGeneration { id })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for DeleteGeneration {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "generation", kind: "delete-generation", record: "DeletedGeneration" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete generation \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
