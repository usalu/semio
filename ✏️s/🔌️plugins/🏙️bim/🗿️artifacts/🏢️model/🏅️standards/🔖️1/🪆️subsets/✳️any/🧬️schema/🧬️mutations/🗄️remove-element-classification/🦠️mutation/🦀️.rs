//! 🗄️ `remove-element-classification` payload. Removes the classification of an element or type in one classification system; its other classifications stay.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveElementClassification {
    pub id: String,
    pub system: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for RemoveElementClassification
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<RemoveElementClassification> + From<crate::standards::v1::subsets::any::schema::mutations::set_element_classification::SetElementClassification>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "element-classification", kind: "remove-element-classification", record: "RemoveElementClassification" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::remove_element_classification::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::remove_element_classification::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove classification {} of \"{}\"", self.system, self.id), &format!("Klassifizierung {} von \"{}\" entfernen", self.system, self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
