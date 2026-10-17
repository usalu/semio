//! ↔ WFC 2D mutation — `MoveSlot`: the settled end of a node drag in the `wfc-graph` window. One
//! mutation per GESTURE, never per pointer tick — mid-drag frames ride the window transient.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️MoveSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MoveSlot {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_slot<Op>(id: String, x: f64, y: f64) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::move_slot::MoveSlot>,
{
    Op::from(MoveSlot { id, x, y })
}

impl<Op> MutationKind<Wfc2dSnapshot, Op> for MoveSlot
where
    Op: protocol::Mutation<Wfc2dSnapshot, Diff = Wfc2dDiff> + From<MoveSlot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "slot", kind: "move-slot", record: "MovedSlot" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Move Slot", "Slot verschieben")
    }
}
//#endregion 🔖️MoveSlot
