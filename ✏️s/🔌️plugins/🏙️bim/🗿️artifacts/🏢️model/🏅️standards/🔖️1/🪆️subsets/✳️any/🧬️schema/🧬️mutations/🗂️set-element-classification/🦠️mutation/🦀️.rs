//! 🗂️ `set-element-classification` payload. Sets the code of an element or type in one classification system; an element without classifications gets its first, another system adds a classification and the same system replaces the code. The code need not be an entry of the system (a diagnostic reports it).

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetElementClassification {
    pub id: String,
    pub system: String,
    pub code: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetElementClassification
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetElementClassification> + From<crate::standards::v1::subsets::any::schema::mutations::remove_element_classification::RemoveElementClassification>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "element-classification", kind: "set-element-classification", record: "SetElementClassification" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_element_classification::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_element_classification::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Classify \"{}\" as {} in {}", self.id, self.code, self.system), &format!("\"{}\" als {} in {} klassifizieren", self.id, self.code, self.system))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
