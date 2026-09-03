//! ✂️ Generation2d mutation — `DisconnectSynapse`: removes an id-keyed edge (captures the removed
//! synapse for its inverse). Wired module name (`remove_synapse`) is a leftover of the pre-semantic
//! generic slot this triad was repurposed from — see `sharedFileRequests` in this ticket's wave2
//! report for the glue.rs rename that would align the directory/module with the verb.

use crate::artifacts::generation2d::diff::Generation2dDiff;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️DisconnectSynapse
/// ✂️ `disconnect-synapse` payload — removes the edge with `id`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DisconnectSynapse {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_synapse(id: String) -> Generation2dMutation {
    Generation2dMutation::DisconnectSynapse(DisconnectSynapse { id })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for DisconnectSynapse {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "disconnect", entity: "synapse", kind: "disconnect-synapse", record: "DisconnectedSynapse" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Disconnect synapse \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DisconnectSynapse
