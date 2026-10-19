//! 📌️ `set-dimension` payload. Sets exactly the provided fields of a dimension: anchors, measuring direction, offset, style, lock (an assigned null removes it) and name; absent fields stay untouched.

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::AnnotationAnchor;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Assigned;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::documentation::DimensionPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDimension {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub anchors: Option<Vec<AnnotationAnchor>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lock: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetDimension {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> DimensionPatch {
        DimensionPatch { anchors: self.anchors.clone(), angle: self.angle, offset: self.offset, style: self.style.clone(), lock: self.lock.clone(), name: self.name.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: DimensionPatch) -> Self {
        Self { id, anchors: patch.anchors, angle: patch.angle, offset: patch.offset, style: patch.style, lock: patch.lock, name: patch.name }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetDimension
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetDimension>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "dimension", kind: "set-dimension", record: "SetDimension" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_dimension::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_dimension::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change dimension \"{}\"", self.id), &format!("Bemaßung \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
