//! 🚦️ `wfc3d` mutation — `CreateRule`: brings a new id-keyed adjacency rule into existence at a
//! FINAL-state insertion index.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::{GraphRule, Wfc3dSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️CreateRule
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateRule {
    pub index: usize,
    pub rule: GraphRule,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_rule(index: usize, rule: GraphRule) -> Wfc3dMutation {
    Wfc3dMutation::CreateRule(CreateRule { index, rule })
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for CreateRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "rule", kind: "create-rule", record: "CreatedRule" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Result<Vec<Wfc3dMutation>, semio_framework_value::ValueError> {
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
