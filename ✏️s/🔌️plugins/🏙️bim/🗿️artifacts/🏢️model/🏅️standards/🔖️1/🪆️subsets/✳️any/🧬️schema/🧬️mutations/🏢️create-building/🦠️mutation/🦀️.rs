//! 🏢️ `create-building` payload. Brings a new building onto an existing site.

use crate::{Building, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateBuilding {
    pub id: String,
    pub building: Building,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateBuilding {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "building", kind: "create-building", record: "CreateBuilding" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create building \"{}\"", self.building.name), &format!("Gebäude \"{}\" anlegen", self.building.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
