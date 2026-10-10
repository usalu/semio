//! 🪑️ `set-space` payload. Sets any of a room's number, name, boundary, usage, zone and floor, wall and ceiling finish (an assigned null leaves the zone or removes a finish); absent fields stay untouched and the number stays unique within the storey.

use crate::{Assigned, ModelDiff, ModelMutation, ModelSnapshot, SpaceBoundary, SpacePatch};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSpace {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub boundary: Option<SpaceBoundary>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<Assigned<Option<String>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub floor_finish: Option<Assigned<Option<String>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub wall_finish: Option<Assigned<Option<String>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ceiling_finish: Option<Assigned<Option<String>>>,
}

impl SetSpace {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> SpacePatch {
        SpacePatch {
            number: self.number.clone(),
            name: self.name.clone(),
            boundary: self.boundary.clone(),
            usage: self.usage.clone(),
            zone: self.zone.clone(),
            floor_finish: self.floor_finish.clone(),
            wall_finish: self.wall_finish.clone(),
            ceiling_finish: self.ceiling_finish.clone(),
            ..Default::default()
        }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: SpacePatch) -> Self {
        Self { id, number: patch.number, name: patch.name, boundary: patch.boundary, usage: patch.usage, zone: patch.zone, floor_finish: patch.floor_finish, wall_finish: patch.wall_finish, ceiling_finish: patch.ceiling_finish }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetSpace {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "space", kind: "set-space", record: "SetSpace" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit space \"{}\"", self.id), &format!("Raum \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
