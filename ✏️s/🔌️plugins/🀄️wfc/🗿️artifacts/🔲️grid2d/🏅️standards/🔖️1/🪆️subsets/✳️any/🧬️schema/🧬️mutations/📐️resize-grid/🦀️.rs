//! 📐 Changes the cell extent of the grid. Cells that fall outside the new extent are cascaded away — a pinned or masked cell may never address a cell the grid does not have.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;
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
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_grid<Op>(width: u32, height: u32) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::resize_grid::ResizeGrid>,
{
    Op::from(ResizeGrid { width, height })
}

impl<Op> MutationKind<Grid2dSnapshot, Op> for ResizeGrid
where
    Op: protocol::Mutation<Grid2dSnapshot, Diff = Grid2dDiff> + From<ResizeGrid> + From<crate::standards::v1::subsets::any::schema::mutations::mask_cell::MaskCell> + From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "grid", kind: "resize-grid", record: "ResizedGrid" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Resize grid to {}×{}", self.width, self.height), &format!("Größe des Rasters auf {}×{} ändern", self.width, self.height))
    }
}
//#endregion 🔖️ResizeGrid
