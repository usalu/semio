//! 🛗️ `create-door-type` payload. Brings a new door type into the library; it needs a free id, an existing material and valid dimensions.

use crate::{DoorType, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateDoorType {
    pub id: String,
    pub door_type: DoorType,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateDoorType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "door-type", kind: "create-door-type", record: "CreateDoorType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create door type \"{}\"", self.door_type.name), &format!("Türtyp \"{}\" anlegen", self.door_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
