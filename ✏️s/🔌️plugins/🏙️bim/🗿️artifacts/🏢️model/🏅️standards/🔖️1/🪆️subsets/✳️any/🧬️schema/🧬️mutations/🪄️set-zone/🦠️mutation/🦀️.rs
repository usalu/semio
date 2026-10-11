//! 🪄️ `set-zone` payload. Sets any of a zone's name, category and occupancy density; absent fields stay untouched.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, ZonePatch};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetZone {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub occupancy_density: Option<f64>,
}

impl SetZone {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ZonePatch {
        ZonePatch { name: self.name.clone(), category: self.category.clone(), occupancy_density: self.occupancy_density }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: ZonePatch) -> Self {
        Self { id, name: patch.name, category: patch.category, occupancy_density: patch.occupancy_density }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetZone {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "zone", kind: "set-zone", record: "SetZone" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit zone \"{}\"", self.id), &format!("Zone \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
