//! 📌️ `wfc3d` mutation — `PinSlot`: binds one slot to a tile the solve must keep. A pin is a domain
//! restriction the solver reads, never an assignment the solver writes.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️PinSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[mutation_leaf(contract = ::protocol)]
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

impl<Op> MutationKind<Wfc3dSnapshot, Op> for PinSlot
where
    Op: protocol::Mutation<Wfc3dSnapshot, Diff = Wfc3dDiff> + From<PinSlot> + From<crate::standards::v1::subsets::any::schema::mutations::unpin_slot::UnpinSlot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "fix", entity: "slot", kind: "pin-slot", record: "Fixed" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Pin slot \"{}\" to tile \"{}\"", self.id, self.tile_id), &format!("Slot \"{}\" auf Kachel \"{}\" fixieren", self.id, self.tile_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️PinSlot
