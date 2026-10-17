//! 📐 `s.wfc.grid3d` mutation — `ResizeGrid`: changes the grid extent and derives each axis' size
//! array from it (truncated, or extended with the last authored size). Refuses outright when a pin
//! or a mask would be left outside the new extent, so the mutation stays atomic and point-invertible
//! instead of silently cascading a cell away.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ResizeGrid
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ResizeGrid {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_grid<Op>(width: u32, height: u32, depth: u32) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::resize_grid::ResizeGrid>,
{
    Op::from(ResizeGrid { width, height, depth })
}

impl<Op> MutationKind<Grid3dSnapshot, Op> for ResizeGrid
where
    Op: protocol::Mutation<Grid3dSnapshot, Diff = Grid3dDiff> + From<ResizeGrid> + From<crate::standards::v1::subsets::any::schema::mutations::change_cell_sizes::ChangeCellSizes>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "grid", kind: "resize-grid", record: "ResizedGrid" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Resize grid to {}×{}×{}", self.width, self.height, self.depth), &format!("Größe des Rasters auf {}×{}×{} ändern", self.width, self.height, self.depth))
    }
}
//#endregion 🔖️ResizeGrid
