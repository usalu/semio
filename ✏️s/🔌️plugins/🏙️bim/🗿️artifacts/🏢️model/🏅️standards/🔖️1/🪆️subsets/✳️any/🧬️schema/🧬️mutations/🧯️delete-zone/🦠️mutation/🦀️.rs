//! 🧯️ `delete-zone` payload. Removes a zone with its properties and classifications and clears the zone of every space that belonged to it; refuses while an area scheme still counts it.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteZone {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteZone {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "zone", kind: "delete-zone", record: "DeleteZone" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete zone \"{}\"", self.id), &format!("Zone \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
