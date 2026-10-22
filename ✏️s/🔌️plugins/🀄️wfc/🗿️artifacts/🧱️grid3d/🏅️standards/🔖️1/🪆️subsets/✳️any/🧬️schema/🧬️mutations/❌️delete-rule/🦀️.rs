//! ❌ `s.wfc.grid3d` mutation — `DeleteRule`: drops one adjacency rule. Because the rule set is a
//! closed allow-list, deleting an `allowed` rule NARROWS the solve rather than widening it.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️DeleteRule
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteRule {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_rule<Op>(id: String) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule>,
{
    Op::from(DeleteRule { id })
}

impl<Op> MutationKind<Grid3dSnapshot, Op> for DeleteRule
where
    Op: protocol::Mutation<Grid3dSnapshot, Diff = Grid3dDiff> + From<DeleteRule> + From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "rule", kind: "delete-rule", record: "DeletedRule" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete rule \"{}\"", self.id), &format!("Regel \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Grid3dSnapshot, Op> for DeleteRule
where
    Op: protocol::ColdMutation<Grid3dSnapshot, ReferenceDiff = Grid3dDiff> + From<DeleteRule> + From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule>,
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
//#endregion 🔖️DeleteRule
