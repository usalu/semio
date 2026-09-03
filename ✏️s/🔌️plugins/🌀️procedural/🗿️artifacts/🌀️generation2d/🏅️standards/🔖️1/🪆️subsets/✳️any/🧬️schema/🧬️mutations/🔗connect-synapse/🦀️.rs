//! 🔗 Generation2d mutation — `ConnectSynapse`: creates a new edge between two widget ports at a
//! FINAL-state insertion index. Wired module name (`set_synapse`) is a leftover of the pre-semantic
//! generic slot this triad was repurposed from — see `sharedFileRequests` in this ticket's wave2
//! report for the glue.rs rename that would align the directory/module with the verb.

use crate::artifacts::generation2d::diff::Generation2dDiff;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use flow::SynapseSpec;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ConnectSynapse
/// 🔗 `connect-synapse` payload — the full new edge plus a FINAL-state insertion index.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ConnectSynapse {
    pub index: usize,
    pub synapse: SynapseSpec,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_synapse(index: usize, synapse: SynapseSpec) -> Generation2dMutation {
    Generation2dMutation::ConnectSynapse(ConnectSynapse { index, synapse })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for ConnectSynapse {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "synapse", kind: "connect-synapse", record: "ConnectedSynapse" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Connect synapse \"{}\" ({} → {})", self.synapse.id, self.synapse.from, self.synapse.to)
    }
    fn target(&self) -> Vec<String> {
        vec![self.synapse.id.clone()]
    }
}
//#endregion 🔖️ConnectSynapse
