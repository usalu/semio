//! 🕳 Cuts one cell out of the problem entirely. A masked cell carries no domain, draws no tile and cascades away any pin it held.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️MaskCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MaskCell {
    pub x: u32,
    pub y: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn mask_cell<Op>(x: u32, y: u32) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::mask_cell::MaskCell>,
{
    Op::from(MaskCell { x, y })
}

impl<Op> MutationKind<Grid2dSnapshot, Op> for MaskCell
where
    Op: protocol::Mutation<Grid2dSnapshot, Diff = Grid2dDiff> + From<MaskCell> + From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell> + From<crate::standards::v1::subsets::any::schema::mutations::unmask_cell::UnmaskCell>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "cell", kind: "mask-cell", record: "RemovedCell" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Mask cell ({}, {})", self.x, self.y), &format!("Zelle ({}, {}) maskieren", self.x, self.y))
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{},{}", self.x, self.y)]
    }
}

impl<Op> protocol::ColdMutationKind<Grid2dSnapshot, Op> for MaskCell
where
    Op: protocol::ColdMutation<Grid2dSnapshot, ReferenceDiff = Grid2dDiff> + From<MaskCell> + From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell> + From<crate::standards::v1::subsets::any::schema::mutations::unmask_cell::UnmaskCell>,
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
//#endregion 🔖️MaskCell
