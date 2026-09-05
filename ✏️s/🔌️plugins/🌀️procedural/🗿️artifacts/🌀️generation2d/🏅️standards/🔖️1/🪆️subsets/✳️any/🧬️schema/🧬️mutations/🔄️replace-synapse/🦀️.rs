//! 🦠️ `🔄️replace-synapse` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use crate::artifacts::generation2d::diff::Generation2dDiff;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use flow::SynapseSpec;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceSynapse {
    pub synapse: SynapseSpec,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_synapse(synapse: SynapseSpec) -> Generation2dMutation {
    Generation2dMutation::ReplaceSynapse(ReplaceSynapse { synapse })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for ReplaceSynapse {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "synapse", kind: "replace-synapse", record: "ReplacedSynapse" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace synapse \"{}\"", self.synapse.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.synapse.id.clone()]
    }
}
//#endregion 🔖️Mutation
