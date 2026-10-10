//! 🚥️ `set-clash-set` payload. Sparsely changes a clash set: its name, either selector (replaced as a whole), the tolerance and the clearance. The clashes are inferred and follow.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use crate::{ClashSetPatch, ElementSelector};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetClashSet {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub a: Option<ElementSelector>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub b: Option<ElementSelector>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub clearance: Option<f64>,
}

impl SetClashSet {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ClashSetPatch {
        ClashSetPatch { name: self.name.clone(), a: self.a.clone(), b: self.b.clone(), tolerance: self.tolerance, clearance: self.clearance }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: ClashSetPatch) -> Self {
        Self { id, name: patch.name, a: patch.a, b: patch.b, tolerance: patch.tolerance, clearance: patch.clearance }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetClashSet {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "clash-set", kind: "set-clash-set", record: "SetClashSet" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change clash set \"{}\"", self.id), &format!("Kollisionssatz \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
