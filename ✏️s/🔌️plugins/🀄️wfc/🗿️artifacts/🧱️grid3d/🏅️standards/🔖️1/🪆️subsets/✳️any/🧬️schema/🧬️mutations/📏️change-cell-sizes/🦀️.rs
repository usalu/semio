//! 📏 `s.wfc.grid3d` mutation — `ChangeCellSizes`: rewrites ONE axis' per-cell size array, the only
//! authoring channel for the grid's non-uniformity. The array length must equal that axis' extent,
//! so a size array can never drift out of step with `width`/`height`/`depth`.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeCellSizes
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeCellSizes {
    pub axis: Grid3dAxis,
    pub sizes: Vec<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_cell_sizes<Op>(axis: Grid3dAxis, sizes: Vec<f64>) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_cell_sizes::ChangeCellSizes>,
{
    Op::from(ChangeCellSizes { axis, sizes })
}

impl<Op> MutationKind<Grid3dSnapshot, Op> for ChangeCellSizes
where
    Op: protocol::Mutation<Grid3dSnapshot, Diff = Grid3dDiff> + From<ChangeCellSizes>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "cell-sizes", kind: "change-cell-sizes", record: "ChangedCellSizes" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change {} cell sizes", self.axis.label()), &format!("{} Zellengrößen ändern", self.axis.label()))
    }
}

impl<Op> protocol::ColdMutationKind<Grid3dSnapshot, Op> for ChangeCellSizes
where
    Op: protocol::ColdMutation<Grid3dSnapshot, ReferenceDiff = Grid3dDiff> + From<ChangeCellSizes>,
{
    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️ChangeCellSizes
