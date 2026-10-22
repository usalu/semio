//! 📐️ `create-drawing` — appends a new owned `s.stdio.semio.drawing` CHILD handle to the cad
//! document's `drawings` collection (the "engineering assembly" `model, drawing` composition row —
//! design-full-plan.md §4). Empty today; this triad is the real, conforming lifecycle for the
//! forward-declared slot rather than leaving the facet's collection un-authorable.

use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-drawing")]
pub struct CreateDrawing {
    pub child_id: String,
    pub target: semio_framework_artifact_reference::ArtifactRef,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
}

impl<Op> MutationKind<CadSnapshot, Op> for CreateDrawing
where
    Op: protocol::Mutation<CadSnapshot, Diff = semio_s_artifact_cad_cad_model_diff::CadDiff> + From<crate::delete_drawing::DeleteDrawing>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "drawing", kind: "create-drawing", record: "CreatedDrawing" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create drawing child {}", self.child_id), &format!("Zeichnungs-Kind {} erstellen", self.child_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.child_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<CadSnapshot, Op> for CreateDrawing
where
    Op: protocol::ColdMutation<CadSnapshot, ReferenceDiff = semio_s_artifact_cad_cad_model_diff::CadDiff> + From<crate::delete_drawing::DeleteDrawing>,
{
    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<semio_s_artifact_cad_cad_model_diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
//#endregion 🔖️Mutation
