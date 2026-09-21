//! 🔗 `wfc3d` mutation — `ConnectSlots`: brings a new id-keyed adjacency edge between two slots into
//! existence at a FINAL-state insertion index — the graph topology the engine propagates over.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::{SlotEdge, Wfc3dSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ConnectSlots
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ConnectSlots {
    pub index: usize,
    pub edge: SlotEdge,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_slots(index: usize, edge: SlotEdge) -> Wfc3dMutation {
    Wfc3dMutation::ConnectSlots(ConnectSlots { index, edge })
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for ConnectSlots {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "slots", kind: "connect-slots", record: "ConnectedSlots" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Connect slots \"{}\" ↔ \"{}\"", self.edge.from_slot_id, self.edge.to_slot_id), &format!("Plätze \"{}\" ↔ \"{}\" verbinden", self.edge.from_slot_id, self.edge.to_slot_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.edge.id.clone()]
    }
}
//#endregion 🔖️ConnectSlots
