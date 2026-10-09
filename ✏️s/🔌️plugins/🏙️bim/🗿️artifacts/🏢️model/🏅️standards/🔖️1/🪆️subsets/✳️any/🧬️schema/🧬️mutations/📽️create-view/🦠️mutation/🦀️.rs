//! 📽️ `create-view` payload. Brings a new view into a building: a plan or ceiling plan of one of its storeys, a section or elevation through a vertical plane, or a camera. Each kind owns exactly its fields; the linework it draws is inferred.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, View};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateView {
    pub id: String,
    pub view: View,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateView {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "view", kind: "create-view", record: "CreateView" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create view \"{}\"", self.view.name), &format!("Ansicht \"{}\" anlegen", self.view.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
