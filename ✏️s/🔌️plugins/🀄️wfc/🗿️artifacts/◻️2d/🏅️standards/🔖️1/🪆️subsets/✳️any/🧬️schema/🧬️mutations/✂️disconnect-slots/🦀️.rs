//! ✂ WFC 2D mutation — `DisconnectSlots`: severs one adjacency edge, leaving both slots in place.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️DisconnectSlots
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
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

impl<Op> MutationKind<Wfc2dSnapshot, Op> for DisconnectSlots
where
    Op: protocol::Mutation<Wfc2dSnapshot, Diff = Wfc2dDiff> + From<DisconnectSlots> + From<crate::standards::v1::subsets::any::schema::mutations::connect_slots::ConnectSlots>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "disconnect", entity: "slots", kind: "disconnect-slots", record: "DisconnectedSlots" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Disconnect Slots", "Slots trennen")
    }
}

impl<Op> protocol::ColdMutationKind<Wfc2dSnapshot, Op> for DisconnectSlots
where
    Op: protocol::ColdMutation<Wfc2dSnapshot, ReferenceDiff = Wfc2dDiff> + From<DisconnectSlots> + From<crate::standards::v1::subsets::any::schema::mutations::connect_slots::ConnectSlots>,
{
    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️DisconnectSlots
