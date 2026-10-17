//! 🔳 Puts a masked cell back into the problem.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️UnmaskCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UnmaskCell {
    pub x: u32,
    pub y: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unmask_cell<Op>(x: u32, y: u32) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::unmask_cell::UnmaskCell>,
{
    Op::from(UnmaskCell { x, y })
}

impl<Op> MutationKind<Grid2dSnapshot, Op> for UnmaskCell
where
    Op: protocol::Mutation<Grid2dSnapshot, Diff = Grid2dDiff> + From<UnmaskCell> + From<crate::standards::v1::subsets::any::schema::mutations::mask_cell::MaskCell>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "restore", entity: "cell", kind: "unmask-cell", record: "RestoredCell" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Unmask cell ({}, {})", self.x, self.y), &format!("Zelle ({}, {}) demaskieren", self.x, self.y))
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{},{}", self.x, self.y)]
    }
}
//#endregion 🔖️UnmaskCell
