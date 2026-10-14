//! 📍️ CAD mutation — `MoveReference` payload + `MutationKind` impl.

use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
/// 📍️ Move one reference overlay's `origin` field.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "move-reference")]
pub struct MoveReference {
    pub model_definition_id: String,
    pub reference_id: String,
    pub new_origin: [f64; 3],
}

impl<Op> MutationKind<CadSnapshot, Op> for MoveReference
where
    Op: protocol::Mutation<CadSnapshot, Diff = semio_s_artifact_cad_cad_model::diff::CadDiff> + From<crate::move_reference::MoveReference>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "reference", kind: "move-reference", record: "MovedReference" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<semio_s_artifact_cad_cad_model::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move reference \"{}\"", self.reference_id), &format!("Referenz \"{}\" verschieben", self.reference_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.model_definition_id.clone(), self.reference_id.clone()]
    }
}
//#endregion 🔖️Mutation
