//! ✂️ `wfc3d` mutation — `DisconnectSlots`: removes an id-addressed adjacency edge.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
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
pub fn disconnect_slots(id: String) -> Wfc3dMutation {
    Wfc3dMutation::DisconnectSlots(DisconnectSlots { id })
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for DisconnectSlots {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "disconnect", entity: "slots", kind: "disconnect-slots", record: "DisconnectedSlots" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Result<Vec<Wfc3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Disconnect slots edge \"{}\"", self.id), &format!("Slot-Kante \"{}\" trennen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DisconnectSlots
