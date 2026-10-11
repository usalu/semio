//! ✂ WFC 2D mutation — `DisconnectSlots`: severs one adjacency edge, leaving both slots in place.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
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
pub fn disconnect_slots(id: String) -> Wfc2dMutation {
    Wfc2dMutation::DisconnectSlots(DisconnectSlots { id })
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for DisconnectSlots {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "disconnect", entity: "slots", kind: "disconnect-slots", record: "DisconnectedSlots" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Result<Vec<Wfc2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Disconnect Slots", "Slots trennen")
    }
}
//#endregion 🔖️DisconnectSlots
