//! 🧺️ `delete-annotation-style` payload. Removes an annotation style that no dimension, tag, text note or leader uses.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteAnnotationStyle {
    pub id: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for DeleteAnnotationStyle
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeleteAnnotationStyle> + From<crate::standards::v1::subsets::any::schema::mutations::create_annotation_style::CreateAnnotationStyle>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "annotation-style", kind: "delete-annotation-style", record: "DeleteAnnotationStyle" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete annotation style \"{}\"", self.id), &format!("Beschriftungsstil \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for DeleteAnnotationStyle
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<DeleteAnnotationStyle> + From<crate::standards::v1::subsets::any::schema::mutations::create_annotation_style::CreateAnnotationStyle>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_annotation_style::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::delete_annotation_style::inverse::inverse(self, base))
    }
}
