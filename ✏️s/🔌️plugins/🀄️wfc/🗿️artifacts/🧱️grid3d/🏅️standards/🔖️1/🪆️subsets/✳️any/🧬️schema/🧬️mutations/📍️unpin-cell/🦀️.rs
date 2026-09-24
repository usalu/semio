//! 📍 `s.wfc.grid3d` mutation — `UnpinCell`: releases one cell back to the full tile domain.

use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️UnpinCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UnpinCell {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unpin_cell(x: u32, y: u32, z: u32) -> Grid3dMutation {
    Grid3dMutation::UnpinCell(UnpinCell { x, y, z })
}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for UnpinCell {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "cell", kind: "unpin-cell", record: "Cleared" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Unpin cell {}", cell_key(self.x, self.y, self.z)), &format!("Fixierung von Zelle {} aufheben", cell_key(self.x, self.y, self.z)))
    }
    fn target(&self) -> Vec<String> {
        vec![cell_key(self.x, self.y, self.z)]
    }
}
//#endregion 🔖️UnpinCell
