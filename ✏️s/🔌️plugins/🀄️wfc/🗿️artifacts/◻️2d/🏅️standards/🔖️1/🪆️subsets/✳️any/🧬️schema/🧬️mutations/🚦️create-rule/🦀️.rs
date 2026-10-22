//! 🚦 WFC 2D mutation — `CreateRule`: declares one tile pair allowed or forbidden, optionally only
//! across one relation class. A deny always beats an allow of the same pair at compile time.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::schema::snapshot::Wfc2dRule;

//#region 🔖️CreateRule
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateRule {
    pub rule: Wfc2dRule,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_rule<Op>(rule: Wfc2dRule) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule>,
{
    Op::from(CreateRule { rule })
}

impl<Op> MutationKind<Wfc2dSnapshot, Op> for CreateRule
where
    Op: protocol::Mutation<Wfc2dSnapshot, Diff = Wfc2dDiff> + From<CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "rule", kind: "create-rule", record: "CreatedRule" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Create Rule", "Regel erstellen")
    }
}

impl<Op> protocol::ColdMutationKind<Wfc2dSnapshot, Op> for CreateRule
where
    Op: protocol::ColdMutation<Wfc2dSnapshot, ReferenceDiff = Wfc2dDiff> + From<CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule>,
{
    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️CreateRule
