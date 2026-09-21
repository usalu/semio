//! 🚫 `s.wfc.grid3d` mutation — `MaskCell`: carves one cell out of the topology entirely, which is how
//! a non-box shape is cut from the regular grid. A pinned cell is refused rather than silently
//! unpinned, so the mutation stays atomic and point-invertible.

use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️MaskCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MaskCell {
    pub cell: Grid3dCell,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn mask_cell(cell: Grid3dCell) -> Grid3dMutation {
    Grid3dMutation::MaskCell(MaskCell { cell })
}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for MaskCell {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "cell", kind: "mask-cell", record: "Removed" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Mask cell {}", cell_key(self.cell.x, self.cell.y, self.cell.z)), &format!("Zelle {} maskieren", cell_key(self.cell.x, self.cell.y, self.cell.z)))
    }
    fn target(&self) -> Vec<String> {
        vec![cell_key(self.cell.x, self.cell.y, self.cell.z)]
    }
}
//#endregion 🔖️MaskCell
