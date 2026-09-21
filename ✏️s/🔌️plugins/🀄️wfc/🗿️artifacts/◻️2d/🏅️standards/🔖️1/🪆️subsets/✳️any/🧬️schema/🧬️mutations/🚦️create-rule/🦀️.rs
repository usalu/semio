//! 🚦 WFC 2D mutation — `CreateRule`: declares one tile pair allowed or forbidden, optionally only
//! across one relation class. A deny always beats an allow of the same pair at compile time.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::schema::snapshot::Wfc2dRule;

//#region 🔖️CreateRule
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateRule {
    pub rule: Wfc2dRule,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_rule(rule: Wfc2dRule) -> Wfc2dMutation {
    Wfc2dMutation::CreateRule(CreateRule { rule })
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for CreateRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "rule", kind: "create-rule", record: "CreatedRule" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Create Rule", "Regel erstellen")
    }
}
//#endregion 🔖️CreateRule
