//! 🚨️ `create-clash-set` payload. Brings a new clash set into the model: side A and side B as selectors (classes, storeys, phases, element ids), the tolerance below which an overlap is no hard clash and the clearance below which two elements are a soft clash. The clashes themselves are inferred.

use crate::{ClashSet, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateClashSet {
    pub id: String,
    pub clash_set: ClashSet,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateClashSet {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "clash-set", kind: "create-clash-set", record: "CreateClashSet" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create clash set \"{}\"", self.clash_set.name), &format!("Kollisionssatz \"{}\" anlegen", self.clash_set.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
