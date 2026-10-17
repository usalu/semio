//! 📐 WFC 2D mutation — `ResizeSlot`: changes the rectangle a slot occupies. The tile media is drawn
//! SCALED into that rectangle, so this is also how a tile's on-canvas size is authored.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ResizeSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ResizeSlot {
    pub id: String,
    pub width: f64,
    pub height: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_slot<Op>(id: String, width: f64, height: f64) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::resize_slot::ResizeSlot>,
{
    Op::from(ResizeSlot { id, width, height })
}

impl<Op> MutationKind<Wfc2dSnapshot, Op> for ResizeSlot
where
    Op: protocol::Mutation<Wfc2dSnapshot, Diff = Wfc2dDiff> + From<ResizeSlot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "slot", kind: "resize-slot", record: "ResizedSlot" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Resize Slot", "Größe des Slots ändern")
    }
}
//#endregion 🔖️ResizeSlot
