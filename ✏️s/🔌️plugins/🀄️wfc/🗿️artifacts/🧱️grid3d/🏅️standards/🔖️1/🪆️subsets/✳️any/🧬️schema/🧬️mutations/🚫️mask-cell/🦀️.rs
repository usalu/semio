//! 🚫 `s.wfc.grid3d` mutation — `MaskCell`: carves one cell out of the topology entirely, which is how
//! a non-box shape is cut from the regular grid. A pinned cell is refused rather than silently
//! unpinned, so the mutation stays atomic and point-invertible.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️MaskCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MaskCell {
    pub cell: Grid3dCell,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn mask_cell<Op>(cell: Grid3dCell) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::mask_cell::MaskCell>,
{
    Op::from(MaskCell { cell })
}

impl<Op> MutationKind<Grid3dSnapshot, Op> for MaskCell
where
    Op: protocol::Mutation<Grid3dSnapshot, Diff = Grid3dDiff> + From<MaskCell> + From<crate::standards::v1::subsets::any::schema::mutations::unmask_cell::UnmaskCell>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "cell", kind: "mask-cell", record: "Removed" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Mask cell {}", cell_key(self.cell.x, self.cell.y, self.cell.z)), &format!("Zelle {} maskieren", cell_key(self.cell.x, self.cell.y, self.cell.z)))
    }
    fn target(&self) -> Vec<String> {
        vec![cell_key(self.cell.x, self.cell.y, self.cell.z)]
    }
}
//#endregion 🔖️MaskCell
