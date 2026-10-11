//! 🗾️ `create-zone` payload. Brings a new zone into the model: a named group of spaces with a purpose and an occupancy density; the spaces join it through `set-space`.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Zone};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateZone {
    pub id: String,
    pub zone: Zone,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateZone {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "zone", kind: "create-zone", record: "CreateZone" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create zone \"{}\"", self.zone.name), &format!("Zone \"{}\" anlegen", self.zone.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
