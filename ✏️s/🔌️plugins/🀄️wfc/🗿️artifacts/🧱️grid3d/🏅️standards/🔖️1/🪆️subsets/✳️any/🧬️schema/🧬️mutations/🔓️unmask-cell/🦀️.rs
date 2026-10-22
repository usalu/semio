//! 🔓 `s.wfc.grid3d` mutation — `UnmaskCell`: puts one carved-out cell back into the topology.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;
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
    pub z: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unmask_cell<Op>(x: u32, y: u32, z: u32) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::unmask_cell::UnmaskCell>,
{
    Op::from(UnmaskCell { x, y, z })
}

impl<Op> MutationKind<Grid3dSnapshot, Op> for UnmaskCell
where
    Op: protocol::Mutation<Grid3dSnapshot, Diff = Grid3dDiff> + From<UnmaskCell> + From<crate::standards::v1::subsets::any::schema::mutations::mask_cell::MaskCell>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "restore", entity: "cell", kind: "unmask-cell", record: "Restored" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Unmask cell {}", cell_key(self.x, self.y, self.z)), &format!("Zelle {} demaskieren", cell_key(self.x, self.y, self.z)))
    }
    fn target(&self) -> Vec<String> {
        vec![cell_key(self.x, self.y, self.z)]
    }
}

impl<Op> protocol::ColdMutationKind<Grid3dSnapshot, Op> for UnmaskCell
where
    Op: protocol::ColdMutation<Grid3dSnapshot, ReferenceDiff = Grid3dDiff> + From<UnmaskCell> + From<crate::standards::v1::subsets::any::schema::mutations::mask_cell::MaskCell>,
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
//#endregion 🔖️UnmaskCell
