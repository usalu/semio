//! ✂️ `wfc3d` mutation — `DisconnectSlots`: removes an id-addressed adjacency edge.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️DisconnectSlots
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DisconnectSlots {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_slots<Op>(id: String) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::disconnect_slots::DisconnectSlots>,
{
    Op::from(DisconnectSlots { id })
}

impl<Op> MutationKind<Wfc3dSnapshot, Op> for DisconnectSlots
where
    Op: protocol::Mutation<Wfc3dSnapshot, Diff = Wfc3dDiff> + From<DisconnectSlots> + From<crate::standards::v1::subsets::any::schema::mutations::connect_slots::ConnectSlots>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "disconnect", entity: "slots", kind: "disconnect-slots", record: "DisconnectedSlots" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Disconnect slots edge \"{}\"", self.id), &format!("Slot-Kante \"{}\" trennen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Wfc3dSnapshot, Op> for DisconnectSlots
where
    Op: protocol::ColdMutation<Wfc3dSnapshot, ReferenceDiff = Wfc3dDiff> + From<DisconnectSlots> + From<crate::standards::v1::subsets::any::schema::mutations::connect_slots::ConnectSlots>,
{
    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️DisconnectSlots
