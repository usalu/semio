//! 🧩️ `create-family` payload. Brings a new, empty parametric family into the project: a name and a category; parameters and solids are added afterwards.

use crate::{Family, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateFamily {
    pub id: String,
    pub family: Family,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateFamily {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "family", kind: "create-family", record: "CreateFamily" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create family \"{}\"", self.family.name), &format!("Familie \"{}\" anlegen", self.family.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
