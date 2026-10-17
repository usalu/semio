//! 🚦 Declares whether tile B may sit in one direction of tile A. Unspecified pairs default to FORBIDDEN, so the rule set is the complete whitelist.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::{Grid2dSnapshot, WfcAdjacencyRule2d};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️CreateRule
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateRule {
    pub rule: WfcAdjacencyRule2d,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_rule<Op>(rule: WfcAdjacencyRule2d) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule>,
{
    Op::from(CreateRule { rule })
}

impl<Op> MutationKind<Grid2dSnapshot, Op> for CreateRule
where
    Op: protocol::Mutation<Grid2dSnapshot, Diff = Grid2dDiff> + From<CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "rule", kind: "create-rule", record: "CreatedRule" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create rule \"{}\"", self.rule.id), &format!("Regel \"{}\" erstellen", self.rule.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.rule.id.clone()]
    }
}
//#endregion 🔖️CreateRule
