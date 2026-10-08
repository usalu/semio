//! 🖌️ `set-material` payload. Patches exactly the provided fields of a material; layouts, solids and quantities that use it follow by inference.

use crate::{MaterialCategory, MaterialPatch, ModelDiff, ModelMutation, ModelSnapshot, Rgb};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetMaterial {
    pub id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub category: Option<MaterialCategory>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub color: Option<Rgb>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub conductivity: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub specific_heat: Option<f64>,
}

impl SetMaterial {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> MaterialPatch {
        MaterialPatch { name: self.name.clone(), category: self.category.clone(), color: self.color.clone(), density: self.density.clone(), conductivity: self.conductivity.clone(), specific_heat: self.specific_heat.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: MaterialPatch) -> Self {
        Self { id, name: patch.name, category: patch.category, color: patch.color, density: patch.density, conductivity: patch.conductivity, specific_heat: patch.specific_heat }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetMaterial {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "material", kind: "set-material", record: "SetMaterial" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit material \"{}\"", self.id), &format!("Material \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
