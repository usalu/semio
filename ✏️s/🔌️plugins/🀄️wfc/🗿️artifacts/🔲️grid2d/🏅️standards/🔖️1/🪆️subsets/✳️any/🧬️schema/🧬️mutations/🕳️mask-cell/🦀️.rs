//! 🕳 Cuts one cell out of the problem entirely. A masked cell carries no domain, draws no tile and cascades away any pin it held.

use crate::diff::Grid2dDiff;
use crate::mutations::Grid2dMutation;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️MaskCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MaskCell {
    pub x: u32,
    pub y: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn mask_cell(x: u32, y: u32) -> Grid2dMutation {
    Grid2dMutation::MaskCell(MaskCell { x, y })
}

impl MutationKind<Grid2dSnapshot, Grid2dMutation> for MaskCell {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "cell", kind: "mask-cell", record: "RemovedCell" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Mask cell ({}, {})", self.x, self.y), &format!("Zelle ({}, {}) maskieren", self.x, self.y))
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{},{}", self.x, self.y)]
    }
}
//#endregion 🔖️MaskCell
