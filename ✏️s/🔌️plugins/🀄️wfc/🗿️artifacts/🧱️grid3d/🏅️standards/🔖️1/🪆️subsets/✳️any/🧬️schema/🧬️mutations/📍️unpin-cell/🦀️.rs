//! 📍 `s.wfc.grid3d` mutation — `UnpinCell`: releases one cell back to the full tile domain.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️UnpinCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UnpinCell {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unpin_cell<Op>(x: u32, y: u32, z: u32) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::unpin_cell::UnpinCell>,
{
    Op::from(UnpinCell { x, y, z })
}

impl<Op> MutationKind<Grid3dSnapshot, Op> for UnpinCell
where
    Op: protocol::Mutation<Grid3dSnapshot, Diff = Grid3dDiff> + From<UnpinCell> + From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "cell", kind: "unpin-cell", record: "Cleared" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Unpin cell {}", cell_key(self.x, self.y, self.z)), &format!("Fixierung von Zelle {} aufheben", cell_key(self.x, self.y, self.z)))
    }
    fn target(&self) -> Vec<String> {
        vec![cell_key(self.x, self.y, self.z)]
    }
}
//#endregion 🔖️UnpinCell
