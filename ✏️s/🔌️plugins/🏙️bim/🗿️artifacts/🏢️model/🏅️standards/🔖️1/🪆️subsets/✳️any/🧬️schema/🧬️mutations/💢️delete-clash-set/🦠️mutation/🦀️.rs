//! 💢️ `delete-clash-set` payload. Removes a clash set together with its properties and classifications. Issues raised from one of its clashes keep naming the set as history.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteClashSet {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteClashSet {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "clash-set", kind: "delete-clash-set", record: "DeleteClashSet" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete clash set \"{}\"", self.id), &format!("Kollisionssatz \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
