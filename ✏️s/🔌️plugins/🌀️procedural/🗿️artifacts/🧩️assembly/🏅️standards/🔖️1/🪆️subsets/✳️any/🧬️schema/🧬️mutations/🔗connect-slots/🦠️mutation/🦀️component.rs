//! 🔗 Assembly mutation — `ConnectSlots`: brings a new id-keyed adjacency edge between two slots
//! into existence at a FINAL-state insertion index — the generic graph topology `wfc_engine`
//! propagates constraints over.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::mutations::AssemblyMutation;
use crate::artifacts::assembly::schema::snapshot::{AssemblySlotEdge, AssemblySnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️ConnectSlots
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectSlots {
    pub index: usize,
    pub edge: AssemblySlotEdge,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_slots(index: usize, edge: AssemblySlotEdge) -> AssemblyMutation {
    AssemblyMutation::ConnectSlots(ConnectSlots { index, edge })
}

impl MutationKind<AssemblySnapshot, AssemblyMutation> for ConnectSlots {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "slots", kind: "connect-slots", record: "ConnectedSlots" };

    fn diff(&self, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Connect slots \"{}\" ↔ \"{}\"", self.edge.from_slot_id, self.edge.to_slot_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.edge.id.clone()]
    }
}
//#endregion 🔖️ConnectSlots
