//! 🔓 `s.wfc.grid3d` mutation — `UnmaskCell`: puts one carved-out cell back into the topology.

use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️UnmaskCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UnmaskCell {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unmask_cell(x: u32, y: u32, z: u32) -> Grid3dMutation {
    Grid3dMutation::UnmaskCell(UnmaskCell { x, y, z })
}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for UnmaskCell {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "restore", entity: "cell", kind: "unmask-cell", record: "Restored" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Unmask cell {}", cell_key(self.x, self.y, self.z)), &format!("Zelle {} demaskieren", cell_key(self.x, self.y, self.z)))
    }
    fn target(&self) -> Vec<String> {
        vec![cell_key(self.x, self.y, self.z)]
    }
}
//#endregion 🔖️UnmaskCell
