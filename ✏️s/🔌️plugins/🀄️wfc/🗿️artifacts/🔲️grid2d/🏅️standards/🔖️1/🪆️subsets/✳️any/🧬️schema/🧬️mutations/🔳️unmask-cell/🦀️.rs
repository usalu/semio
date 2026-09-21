//! 🔳 Puts a masked cell back into the problem.

use crate::diff::Grid2dDiff;
use crate::mutations::Grid2dMutation;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️UnmaskCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UnmaskCell {
    pub x: u32,
    pub y: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unmask_cell(x: u32, y: u32) -> Grid2dMutation {
    Grid2dMutation::UnmaskCell(UnmaskCell { x, y })
}

impl MutationKind<Grid2dSnapshot, Grid2dMutation> for UnmaskCell {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "restore", entity: "cell", kind: "unmask-cell", record: "RestoredCell" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Unmask cell ({}, {})", self.x, self.y), &format!("Zelle ({}, {}) demaskieren", self.x, self.y))
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{},{}", self.x, self.y)]
    }
}
//#endregion 🔖️UnmaskCell
