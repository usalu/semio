//! 🔡️ `set-family-parameter` payload. Sets exactly the provided kind and formula of the parameter `name` of a family, or adds it (a new parameter needs both). Formulas are canonical text of the expression language, use only existing parameters and never close a circle.

use crate::{FamilyParameterPatch, ModelDiff, ModelMutation, ModelSnapshot, ParameterKind};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFamilyParameter {
    pub family: String,
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ParameterKind>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl SetFamilyParameter {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> FamilyParameterPatch {
        FamilyParameterPatch { kind: self.kind, value: self.value.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(family: String, name: String, patch: FamilyParameterPatch) -> Self {
        Self { family, name, kind: patch.kind, value: patch.value }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetFamilyParameter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "family-parameter", kind: "set-family-parameter", record: "SetFamilyParameter" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set parameter \"{}\" of family \"{}\"", self.name, self.family), &format!("Parameter \"{}\" der Familie \"{}\" setzen", self.name, self.family))
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{}.{}", self.family, self.name)]
    }
}
