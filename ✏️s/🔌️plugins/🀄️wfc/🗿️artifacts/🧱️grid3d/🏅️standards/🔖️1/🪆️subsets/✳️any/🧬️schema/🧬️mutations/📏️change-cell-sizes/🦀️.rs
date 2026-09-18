//! 📏 `s.wfc.grid3d` mutation — `ChangeCellSizes`: rewrites ONE axis' per-cell size array, the only
//! authoring channel for the grid's non-uniformity. The array length must equal that axis' extent,
//! so a size array can never drift out of step with `width`/`height`/`depth`.

use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeCellSizes
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeCellSizes {
    pub axis: Grid3dAxis,
    pub sizes: Vec<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_cell_sizes(axis: Grid3dAxis, sizes: Vec<f64>) -> Grid3dMutation {
    Grid3dMutation::ChangeCellSizes(ChangeCellSizes { axis, sizes })
}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for ChangeCellSizes {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "cell-sizes", kind: "change-cell-sizes", record: "ChangedCellSizes" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change {} cell sizes", self.axis.label())
    }
}
//#endregion 🔖️ChangeCellSizes
