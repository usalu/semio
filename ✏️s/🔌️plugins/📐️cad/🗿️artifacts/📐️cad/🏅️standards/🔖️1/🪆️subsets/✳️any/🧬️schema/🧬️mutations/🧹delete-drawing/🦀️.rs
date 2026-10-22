//! 🧹️ `delete-drawing` — removes the entry matching `child_id` from `drawings`. Idempotent no-op
//! if absent; the inverse escrows the removed handle from BASE.

use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-drawing")]
pub struct DeleteDrawing {
    pub child_id: String,
}

impl<Op> MutationKind<CadSnapshot, Op> for DeleteDrawing
where
    Op: protocol::Mutation<CadSnapshot, Diff = semio_s_artifact_cad_cad_model_diff::CadDiff> + From<crate::create_drawing::CreateDrawing>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "drawing", kind: "delete-drawing", record: "DeletedDrawing" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete drawing child {}", self.child_id), &format!("Zeichnungs-Kind {} löschen", self.child_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.child_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<CadSnapshot, Op> for DeleteDrawing
where
    Op: protocol::ColdMutation<CadSnapshot, ReferenceDiff = semio_s_artifact_cad_cad_model_diff::CadDiff> + From<crate::create_drawing::CreateDrawing>,
{
    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<semio_s_artifact_cad_cad_model_diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
//#endregion 🔖️Mutation
