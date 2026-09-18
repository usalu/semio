//! 🔓 WFC 2D mutation — `UnpinSlot`: releases a slot's hard pre-assignment back to the solver.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️UnpinSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UnpinSlot {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unpin_slot(id: String) -> Wfc2dMutation {
    Wfc2dMutation::UnpinSlot(UnpinSlot { id })
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for UnpinSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "slot", kind: "unpin-slot", record: "ClearedSlot" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Unpin Slot".into()
    }
}
//#endregion 🔖️UnpinSlot
