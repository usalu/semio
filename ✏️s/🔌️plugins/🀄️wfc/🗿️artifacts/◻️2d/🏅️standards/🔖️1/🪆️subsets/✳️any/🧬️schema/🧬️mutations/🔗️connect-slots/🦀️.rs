//! 🔗 WFC 2D mutation — `ConnectSlots`: the settled end of a connect gesture in the `wfc-graph`
//! window. The edge's `relation` names the adjacency class its rules are scoped to.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::schema::snapshot::Wfc2dSlotEdge;

//#region 🔖️ConnectSlots
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ConnectSlots {
    pub edge: Wfc2dSlotEdge,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_slots<Op>(edge: Wfc2dSlotEdge) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_slots::ConnectSlots>,
{
    Op::from(ConnectSlots { edge })
}

impl<Op> MutationKind<Wfc2dSnapshot, Op> for ConnectSlots
where
    Op: protocol::Mutation<Wfc2dSnapshot, Diff = Wfc2dDiff> + From<ConnectSlots> + From<crate::standards::v1::subsets::any::schema::mutations::disconnect_slots::DisconnectSlots>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "slots", kind: "connect-slots", record: "ConnectedSlots" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Connect Slots", "Slots verbinden")
    }
}
//#endregion 🔖️ConnectSlots
