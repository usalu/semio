//! 📚️ `create-classification-system` payload. Brings a new classification system into the model library: its name, edition and the authored table of its entries (code, title, parent). Elements are classified through `set-element-classification`.

use crate::{ClassificationSystem, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateClassificationSystem {
    pub id: String,
    pub system: ClassificationSystem,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateClassificationSystem {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "classification-system", kind: "create-classification-system", record: "CreateClassificationSystem" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create classification system \"{}\"", self.system.name), &format!("Klassifikationssystem \"{}\" anlegen", self.system.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
