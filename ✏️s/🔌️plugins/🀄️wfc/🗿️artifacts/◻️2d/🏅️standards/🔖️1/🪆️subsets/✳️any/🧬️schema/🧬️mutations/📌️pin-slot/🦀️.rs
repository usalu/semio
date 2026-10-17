//! 📌 WFC 2D mutation — `PinSlot`: hard-assigns one tile to one slot. A pin is a DOMAIN RESTRICTION
//! the solve must respect, never a solved assignment written back into the document.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️PinSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct PinSlot {
    pub id: String,
    pub tile_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn pin_slot<Op>(id: String, tile_id: String) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_slot::PinSlot>,
{
    Op::from(PinSlot { id, tile_id })
}

impl<Op> MutationKind<Wfc2dSnapshot, Op> for PinSlot
where
    Op: protocol::Mutation<Wfc2dSnapshot, Diff = Wfc2dDiff> + From<PinSlot> + From<crate::standards::v1::subsets::any::schema::mutations::unpin_slot::UnpinSlot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "fix", entity: "slot", kind: "pin-slot", record: "FixedSlot" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Pin Slot", "Slot fixieren")
    }
}
//#endregion 🔖️PinSlot
