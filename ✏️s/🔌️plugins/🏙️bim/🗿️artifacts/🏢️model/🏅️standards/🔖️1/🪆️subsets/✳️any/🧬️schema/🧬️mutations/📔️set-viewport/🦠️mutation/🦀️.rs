//! 📔️ `set-viewport` payload. Sparsely changes a viewport: its sheet, its view, the position of its window on the paper, its scale (an assigned null never clears it), its crop (an assigned null clears it) and its label (an assigned null returns to the name of the view).

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Assigned;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::documentation::ViewportPatch;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Point2, ViewCrop};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetViewport {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sheet: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub crop: Option<Assigned<Option<ViewCrop>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Assigned<Option<String>>>,
}

impl SetViewport {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ViewportPatch {
        ViewportPatch { sheet: self.sheet.clone(), view: self.view.clone(), position: self.position, scale: self.scale, crop: self.crop.clone(), label: self.label.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: ViewportPatch) -> Self {
        Self { id, sheet: patch.sheet, view: patch.view, position: patch.position, scale: patch.scale, crop: patch.crop, label: patch.label }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetViewport
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetViewport>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "viewport", kind: "set-viewport", record: "SetViewport" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_viewport::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_viewport::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change viewport \"{}\"", self.id), &format!("Ansichtsfenster \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
