//! 📐️ `wfc3d` mutation — `ResizeSlot`: sets one slot's box extent. The preview scales the assigned
//! tile's unit-box media into exactly this extent, so a zero or negative dimension is refused.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ResizeSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ResizeSlot {
    pub id: String,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_slot<Op>(id: String, width: f64, height: f64, depth: f64) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::resize_slot::ResizeSlot>,
{
    Op::from(ResizeSlot { id, width, height, depth })
}

impl<Op> MutationKind<Wfc3dSnapshot, Op> for ResizeSlot
where
    Op: protocol::Mutation<Wfc3dSnapshot, Diff = Wfc3dDiff> + From<ResizeSlot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "slot", kind: "resize-slot", record: "ResizedSlot" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Resize slot \"{}\"", self.id), &format!("Größe von Slot \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ResizeSlot
