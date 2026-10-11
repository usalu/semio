//! 📍️ `wfc3d` mutation — `UnpinSlot`: releases one slot's tile pin, handing the whole tile domain
//! back to the solver.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️UnpinSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct UnpinSlot {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unpin_slot(id: String) -> Wfc3dMutation {
    Wfc3dMutation::UnpinSlot(UnpinSlot { id })
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for UnpinSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "slot", kind: "unpin-slot", record: "Cleared" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Result<Vec<Wfc3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Unpin slot \"{}\"", self.id), &format!("Fixierung von Slot \"{}\" aufheben", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️UnpinSlot
