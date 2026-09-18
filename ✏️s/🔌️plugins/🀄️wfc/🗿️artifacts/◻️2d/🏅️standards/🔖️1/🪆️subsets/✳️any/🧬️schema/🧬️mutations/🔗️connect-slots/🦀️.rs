//! 🔗 WFC 2D mutation — `ConnectSlots`: the settled end of a connect gesture in the `wfc-graph`
//! window. The edge's `relation` names the adjacency class its rules are scoped to.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::schema::snapshot::Wfc2dSlotEdge;

//#region 🔖️ConnectSlots
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ConnectSlots {
    pub edge: Wfc2dSlotEdge,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_slots(edge: Wfc2dSlotEdge) -> Wfc2dMutation {
    Wfc2dMutation::ConnectSlots(ConnectSlots { edge })
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for ConnectSlots {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "slots", kind: "connect-slots", record: "ConnectedSlots" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Connect Slots".into()
    }
}
//#endregion 🔖️ConnectSlots
