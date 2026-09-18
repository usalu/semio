//! 📌 Pre-assigns one cell to a tile — a hard fix the solver seeds its domains with, not a hint.

use crate::diff::Grid2dDiff;
use crate::mutations::Grid2dMutation;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️PinCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct PinCell {
    pub x: u32,
    pub y: u32,
    pub tile_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn pin_cell(x: u32, y: u32, tile_id: String) -> Grid2dMutation {
    Grid2dMutation::PinCell(PinCell { x, y, tile_id })
}

impl MutationKind<Grid2dSnapshot, Grid2dMutation> for PinCell {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "fix", entity: "cell", kind: "pin-cell", record: "FixedCell" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Pin cell ({}, {}) to \"{}\"", self.x, self.y, self.tile_id)
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{},{}", self.x, self.y)]
    }
}
//#endregion 🔖️PinCell
