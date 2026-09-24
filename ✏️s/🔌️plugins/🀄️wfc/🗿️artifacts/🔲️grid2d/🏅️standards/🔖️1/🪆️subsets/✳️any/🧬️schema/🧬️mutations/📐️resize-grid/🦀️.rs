//! 📐 Changes the cell extent of the grid. Cells that fall outside the new extent are cascaded away — a pinned or masked cell may never address a cell the grid does not have.

use crate::diff::Grid2dDiff;
use crate::mutations::Grid2dMutation;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ResizeGrid
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ResizeGrid {
    pub width: u32,
    pub height: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_grid(width: u32, height: u32) -> Grid2dMutation {
    Grid2dMutation::ResizeGrid(ResizeGrid { width, height })
}

impl MutationKind<Grid2dSnapshot, Grid2dMutation> for ResizeGrid {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "grid", kind: "resize-grid", record: "ResizedGrid" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Resize grid to {}×{}", self.width, self.height), &format!("Größe des Rasters auf {}×{} ändern", self.width, self.height))
    }
}
//#endregion 🔖️ResizeGrid
