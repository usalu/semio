//! 📏 Changes the world size of one cell — the box every tile's media is scaled into, and the `grid_factor` the grid window snaps to.

use crate::diff::Grid2dDiff;
use crate::mutations::Grid2dMutation;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeCellSize
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeCellSize {
    pub cell_width: f64,
    pub cell_height: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_cell_size(cell_width: f64, cell_height: f64) -> Grid2dMutation {
    Grid2dMutation::ChangeCellSize(ChangeCellSize { cell_width, cell_height })
}

impl MutationKind<Grid2dSnapshot, Grid2dMutation> for ChangeCellSize {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "cell-size", kind: "change-cell-size", record: "ChangedCellSize" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change cell size to {} × {}", self.cell_width, self.cell_height), &format!("Zellengröße auf {} × {} ändern", self.cell_width, self.cell_height))
    }
}
//#endregion 🔖️ChangeCellSize
