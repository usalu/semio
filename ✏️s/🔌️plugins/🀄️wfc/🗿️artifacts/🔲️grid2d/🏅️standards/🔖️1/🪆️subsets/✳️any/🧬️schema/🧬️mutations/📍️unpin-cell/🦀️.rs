//! 📍 Releases one pre-assigned cell back to the solver.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️UnpinCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UnpinCell {
    pub x: u32,
    pub y: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unpin_cell<Op>(x: u32, y: u32) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::unpin_cell::UnpinCell>,
{
    Op::from(UnpinCell { x, y })
}

impl<Op> MutationKind<Grid2dSnapshot, Op> for UnpinCell
where
    Op: protocol::Mutation<Grid2dSnapshot, Diff = Grid2dDiff> + From<UnpinCell> + From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "cell-pin", kind: "unpin-cell", record: "ClearedCellPin" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Unpin cell ({}, {})", self.x, self.y), &format!("Fixierung von Zelle ({}, {}) aufheben", self.x, self.y))
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{},{}", self.x, self.y)]
    }
}

impl<Op> protocol::ColdMutationKind<Grid2dSnapshot, Op> for UnpinCell
where
    Op: protocol::ColdMutation<Grid2dSnapshot, ReferenceDiff = Grid2dDiff> + From<UnpinCell> + From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
{
    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️UnpinCell
