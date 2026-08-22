//! 🦠️ `🔄replace-synapse` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::SynapseSpec;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceSynapse {
    pub synapse: SynapseSpec,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_synapse(synapse: SynapseSpec) -> Procedural2dMutation {
    Procedural2dMutation::ReplaceSynapse(ReplaceSynapse { synapse })
}

impl MutationKind<Procedural2dSnapshot, Procedural2dMutation> for ReplaceSynapse {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "synapse", kind: "replace-synapse", record: "ReplacedSynapse" };

    fn diff(&self, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
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
