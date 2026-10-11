//! 💨️ `set-mep-element` payload. Sets exactly the provided fields of an MEP element: storey, system, cross-section, path and name; absent fields stay untouched. A provided path replaces the path as one field.

use crate::{MepElementPatch, MepShape, MepSystem, ModelDiff, ModelMutation, ModelSnapshot, Point3};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetMepElement {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<MepSystem>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<MepShape>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<Point3>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetMepElement {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> MepElementPatch {
        MepElementPatch { storey: self.storey.clone(), system: self.system, shape: self.shape.clone(), path: self.path.clone(), name: self.name.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: MepElementPatch) -> Self {
        Self { id, storey: patch.storey, system: patch.system, shape: patch.shape, path: patch.path, name: patch.name }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetMepElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "mep-element", kind: "set-mep-element", record: "SetMepElement" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change MEP element \"{}\"", self.id), &format!("TGA-Element \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
