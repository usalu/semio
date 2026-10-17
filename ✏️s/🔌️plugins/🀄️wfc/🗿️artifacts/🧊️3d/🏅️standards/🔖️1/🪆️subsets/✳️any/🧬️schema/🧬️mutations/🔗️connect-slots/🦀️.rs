//! 🔗 `wfc3d` mutation — `ConnectSlots`: brings a new id-keyed adjacency edge between two slots into
//! existence at a FINAL-state insertion index — the graph topology the engine propagates over.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::{SlotEdge, Wfc3dSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ConnectSlots
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ConnectSlots {
    pub index: usize,
    pub edge: SlotEdge,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_slots<Op>(index: usize, edge: SlotEdge) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_slots::ConnectSlots>,
{
    Op::from(ConnectSlots { index, edge })
}

impl<Op> MutationKind<Wfc3dSnapshot, Op> for ConnectSlots
where
    Op: protocol::Mutation<Wfc3dSnapshot, Diff = Wfc3dDiff> + From<ConnectSlots> + From<crate::standards::v1::subsets::any::schema::mutations::disconnect_slots::DisconnectSlots>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "slots", kind: "connect-slots", record: "ConnectedSlots" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect slots \"{}\" ↔ \"{}\"", self.edge.from_slot_id, self.edge.to_slot_id), &format!("Slots \"{}\" ↔ \"{}\" verbinden", self.edge.from_slot_id, self.edge.to_slot_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.edge.id.clone()]
    }
}
//#endregion 🔖️ConnectSlots
