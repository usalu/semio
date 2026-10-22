//! 🔏️ `set-classification-system` payload. Sets any of a classification system's name, edition, source and entry table (an assigned null removes the source); absent fields stay untouched and the entry table replaces the whole table.

use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Assigned;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::ClassificationSystemPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::ClassificationItem;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetClassificationSystem {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Assigned<Option<String>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<ClassificationItem>>,
}

impl SetClassificationSystem {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ClassificationSystemPatch {
        ClassificationSystemPatch { name: self.name.clone(), edition: self.edition.clone(), source: self.source.clone(), entries: self.entries.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: ClassificationSystemPatch) -> Self {
        Self { id, name: patch.name, edition: patch.edition, source: patch.source, entries: patch.entries }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetClassificationSystem
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetClassificationSystem>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "classification-system", kind: "set-classification-system", record: "SetClassificationSystem" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit classification system \"{}\"", self.id), &format!("Klassifikationssystem \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetClassificationSystem
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetClassificationSystem>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_classification_system::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_classification_system::inverse::inverse(self, base))
    }
}
