//! 🔓 WFC 2D mutation — `UnpinSlot`: releases a slot's hard pre-assignment back to the solver.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️UnpinSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UnpinSlot {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unpin_slot<Op>(id: String) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::unpin_slot::UnpinSlot>,
{
    Op::from(UnpinSlot { id })
}

impl<Op> MutationKind<Wfc2dSnapshot, Op> for UnpinSlot
where
    Op: protocol::Mutation<Wfc2dSnapshot, Diff = Wfc2dDiff> + From<UnpinSlot> + From<crate::standards::v1::subsets::any::schema::mutations::pin_slot::PinSlot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "slot", kind: "unpin-slot", record: "ClearedSlot" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Unpin Slot", "Fixierung des Slots aufheben")
    }
}

impl<Op> protocol::ColdMutationKind<Wfc2dSnapshot, Op> for UnpinSlot
where
    Op: protocol::ColdMutation<Wfc2dSnapshot, ReferenceDiff = Wfc2dDiff> + From<UnpinSlot> + From<crate::standards::v1::subsets::any::schema::mutations::pin_slot::PinSlot>,
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
//#endregion 🔖️UnpinSlot
