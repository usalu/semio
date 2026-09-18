//! 📍 Releases one pre-assigned cell back to the solver.

use crate::diff::Grid2dDiff;
use crate::mutations::Grid2dMutation;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️UnpinCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UnpinCell {
    pub x: u32,
    pub y: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unpin_cell(x: u32, y: u32) -> Grid2dMutation {
    Grid2dMutation::UnpinCell(UnpinCell { x, y })
}

impl MutationKind<Grid2dSnapshot, Grid2dMutation> for UnpinCell {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "cell-pin", kind: "unpin-cell", record: "ClearedCellPin" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Unpin cell ({}, {})", self.x, self.y)
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{},{}", self.x, self.y)]
    }
}
//#endregion 🔖️UnpinCell
