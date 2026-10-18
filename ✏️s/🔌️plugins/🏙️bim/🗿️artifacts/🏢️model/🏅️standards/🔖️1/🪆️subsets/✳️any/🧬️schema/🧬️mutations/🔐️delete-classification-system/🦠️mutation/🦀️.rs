//! 🔐️ `delete-classification-system` payload. Removes a classification system with every element and type classification that names it; the inverse restores the system and each classification.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteClassificationSystem {
    pub id: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for DeleteClassificationSystem
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeleteClassificationSystem> + From<crate::standards::v1::subsets::any::schema::mutations::create_classification_system::CreateClassificationSystem> + From<crate::standards::v1::subsets::any::schema::mutations::set_element_classification::SetElementClassification>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "classification-system", kind: "delete-classification-system", record: "DeleteClassificationSystem" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_classification_system::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::delete_classification_system::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete classification system \"{}\"", self.id), &format!("Klassifikationssystem \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
