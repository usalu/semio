//! 📐 `s.wfc.grid3d` mutation — `ResizeGrid`: changes the grid extent and derives each axis' size
//! array from it (truncated, or extended with the last authored size). Refuses outright when a pin
//! or a mask would be left outside the new extent, so the mutation stays atomic and point-invertible
//! instead of silently cascading a cell away.

use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ResizeGrid
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ResizeGrid {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_grid(width: u32, height: u32, depth: u32) -> Grid3dMutation {
    Grid3dMutation::ResizeGrid(ResizeGrid { width, height, depth })
}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for ResizeGrid {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "grid", kind: "resize-grid", record: "ResizedGrid" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Resize grid to {}×{}×{}", self.width, self.height, self.depth), &format!("Raster auf {}×{}×{} skalieren", self.width, self.height, self.depth))
    }
}
//#endregion 🔖️ResizeGrid
