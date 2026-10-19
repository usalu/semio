//! 🎨️ `create-annotation-style` payload. Brings a new annotation style into the library: text height, line end mark, printed unit and precision shared by dimensions, tags, notes and leaders.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{AnnotationStyle, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateAnnotationStyle {
    pub id: String,
    pub annotation_style: AnnotationStyle,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateAnnotationStyle
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateAnnotationStyle> + From<crate::standards::v1::subsets::any::schema::mutations::delete_annotation_style::DeleteAnnotationStyle>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "annotation-style", kind: "create-annotation-style", record: "CreateAnnotationStyle" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_annotation_style::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_annotation_style::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create annotation style \"{}\"", self.annotation_style.name), &format!("Beschriftungsstil \"{}\" anlegen", self.annotation_style.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
