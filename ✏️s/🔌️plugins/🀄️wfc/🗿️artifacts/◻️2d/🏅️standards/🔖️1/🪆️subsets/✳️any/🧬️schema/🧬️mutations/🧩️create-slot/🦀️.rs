//! 🧩 WFC 2D mutation — `CreateSlot`: brings a new id-keyed slot into existence at its CANONICAL
//! ascending-id position, never at the end, so `delete-slot`'s inverse restores the same index.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::schema::snapshot::Wfc2dSlot;

//#region 🔖️CreateSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateSlot {
    pub slot: Wfc2dSlot,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_slot<Op>(slot: Wfc2dSlot) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_slot::CreateSlot>,
{
    Op::from(CreateSlot { slot })
}

impl<Op> MutationKind<Wfc2dSnapshot, Op> for CreateSlot
where
    Op: protocol::Mutation<Wfc2dSnapshot, Diff = Wfc2dDiff> + From<CreateSlot> + From<crate::standards::v1::subsets::any::schema::mutations::delete_slot::DeleteSlot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "slot", kind: "create-slot", record: "CreatedSlot" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Create Slot", "Slot erstellen")
    }
}

impl<Op> protocol::ColdMutationKind<Wfc2dSnapshot, Op> for CreateSlot
where
    Op: protocol::ColdMutation<Wfc2dSnapshot, ReferenceDiff = Wfc2dDiff> + From<CreateSlot> + From<crate::standards::v1::subsets::any::schema::mutations::delete_slot::DeleteSlot>,
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
//#endregion 🔖️CreateSlot
