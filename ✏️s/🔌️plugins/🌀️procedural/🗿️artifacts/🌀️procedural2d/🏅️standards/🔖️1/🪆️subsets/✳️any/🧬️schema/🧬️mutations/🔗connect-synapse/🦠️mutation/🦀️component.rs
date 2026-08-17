//! 🔗 Procedural2d mutation — `ConnectSynapse`: creates a new edge between two widget ports at a
//! FINAL-state insertion index. Wired module name (`set_synapse`) is a leftover of the pre-semantic
//! generic slot this triad was repurposed from — see `sharedFileRequests` in this ticket's wave2
//! report for the glue.rs rename that would align the directory/module with the verb.

use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::SynapseSpec;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️ConnectSynapse
/// 🔗 `connect-synapse` payload — the full new edge plus a FINAL-state insertion index.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectSynapse {
    pub index: usize,
    pub synapse: SynapseSpec,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_synapse(index: usize, synapse: SynapseSpec) -> Procedural2dMutation {
    Procedural2dMutation::ConnectSynapse(ConnectSynapse { index, synapse })
}

impl MutationKind<Procedural2dSnapshot, Procedural2dMutation> for ConnectSynapse {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "synapse", kind: "connect-synapse", record: "ConnectedSynapse" };

    fn diff(&self, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
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
