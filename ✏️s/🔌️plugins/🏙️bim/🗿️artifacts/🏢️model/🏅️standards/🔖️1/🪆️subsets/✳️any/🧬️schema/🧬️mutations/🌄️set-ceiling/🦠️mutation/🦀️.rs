//! 🌄️ `set-ceiling` payload. Sets exactly the provided fields of a ceiling: type, drop below the storey top, slope (set or cleared) and name; absent fields stay untouched.

use crate::{Assigned, CeilingPatch, ModelDiff, ModelMutation, ModelSnapshot, Slope};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCeiling {
    pub id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub ceiling_type: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub slope: Option<Assigned<Option<Slope>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetCeiling {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> CeilingPatch {
        CeilingPatch { ceiling_type: self.ceiling_type.clone(), offset: self.offset, slope: self.slope.clone(), name: self.name.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: CeilingPatch) -> Self {
        Self { id, ceiling_type: patch.ceiling_type, offset: patch.offset, slope: patch.slope, name: patch.name }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetCeiling {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "ceiling", kind: "set-ceiling", record: "SetCeiling" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change ceiling \"{}\"", self.id), &format!("Unterdecke \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
