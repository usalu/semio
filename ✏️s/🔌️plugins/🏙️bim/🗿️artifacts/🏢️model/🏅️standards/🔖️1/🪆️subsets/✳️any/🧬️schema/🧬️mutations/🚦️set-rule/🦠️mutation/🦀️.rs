//! 🚦️ `set-rule` payload. Sparsely changes a rule: its name, kind, limit, severity and scope (the scope is replaced as a whole). The findings are inferred and follow.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use crate::{RuleKind, RulePatch, RuleScope, RuleSeverity};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRule {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<RuleKind>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<RuleSeverity>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<RuleScope>,
}

impl SetRule {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> RulePatch {
        RulePatch { name: self.name.clone(), kind: self.kind, limit: self.limit, severity: self.severity, scope: self.scope.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: RulePatch) -> Self {
        Self { id, name: patch.name, kind: patch.kind, limit: patch.limit, severity: patch.severity, scope: patch.scope }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "rule", kind: "set-rule", record: "SetRule" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change rule \"{}\"", self.id), &format!("Regel \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
