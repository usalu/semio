//! 🛁️ `set-component` payload. Sets exactly the provided fields of a component: storey, family, position, elevation, rotation, mirror flag, host wall (set or cleared), system (set or cleared) and name; absent fields stay untouched.

use crate::{Assigned, ComponentPatch, MepSystem, ModelDiff, ModelMutation, ModelSnapshot, Point2};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetComponent {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mirrored: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<Assigned<Option<String>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<Assigned<Option<MepSystem>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetComponent {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ComponentPatch {
        ComponentPatch { storey: self.storey.clone(), family: self.family.clone(), position: self.position, elevation: self.elevation, rotation: self.rotation, mirrored: self.mirrored, host: self.host.clone(), system: self.system.clone(), name: self.name.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: ComponentPatch) -> Self {
        Self { id, storey: patch.storey, family: patch.family, position: patch.position, elevation: patch.elevation, rotation: patch.rotation, mirrored: patch.mirrored, host: patch.host, system: patch.system, name: patch.name }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetComponent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "component", kind: "set-component", record: "SetComponent" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change component \"{}\"", self.id), &format!("Komponente \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
