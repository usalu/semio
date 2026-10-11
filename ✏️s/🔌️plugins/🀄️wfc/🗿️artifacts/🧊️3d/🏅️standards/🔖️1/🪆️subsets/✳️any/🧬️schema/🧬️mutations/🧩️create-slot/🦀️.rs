//! 🌱 `wfc3d` mutation — `CreateSlot`: brings a new id-keyed slot into existence at a FINAL-state
//! insertion index (the caller passes the collection's canonical sorted position, so the row comes
//! back exactly where it was after an undo).

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::{Slot3d, Wfc3dSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️CreateSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSlot {
    pub index: usize,
    pub slot: Slot3d,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_slot(index: usize, slot: Slot3d) -> Wfc3dMutation {
    Wfc3dMutation::CreateSlot(CreateSlot { index, slot })
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for CreateSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "slot", kind: "create-slot", record: "CreatedSlot" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Result<Vec<Wfc3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create slot \"{}\"", self.slot.id), &format!("Slot \"{}\" erstellen", self.slot.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.slot.id.clone()]
    }
}
//#endregion 🔖️CreateSlot
