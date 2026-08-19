//! ✂️ Assembly mutation — `DisconnectSlots`: removes an id-addressed adjacency edge.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::mutations::AssemblyMutation;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️DisconnectSlots
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisconnectSlots {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn disconnect_slots(id: String) -> AssemblyMutation {
    AssemblyMutation::DisconnectSlots(DisconnectSlots { id })
}

impl MutationKind<AssemblySnapshot, AssemblyMutation> for DisconnectSlots {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "disconnect", entity: "slots", kind: "disconnect-slots", record: "DisconnectedSlots" };

    async fn diff(&self, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Disconnect slots edge \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DisconnectSlots
