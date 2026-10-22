//! 📎️ CAD mutation — `ReplaceReferences` payload + `MutationKind` impl.

use semio_s_artifact_cad_cad_model::{snapshot::CadReference, snapshot::CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
/// 📎️ Whole-value swap of one model definition's entire reference-overlay list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-references")]
pub struct ReplaceReferences {
    pub model_definition_id: String,
    #[dsl(table)]
    pub references: Vec<CadReference>,
}

impl<Op> MutationKind<CadSnapshot, Op> for ReplaceReferences
where
    Op: protocol::Mutation<CadSnapshot, Diff = semio_s_artifact_cad_cad_model_diff::CadDiff> + From<crate::replace_references::ReplaceReferences>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "references", kind: "replace-references", record: "ReplacedReferences" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace references for \"{}\"", self.model_definition_id), &format!("Referenzen für \"{}\" ersetzen", self.model_definition_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.model_definition_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<CadSnapshot, Op> for ReplaceReferences
where
    Op: protocol::ColdMutation<CadSnapshot, ReferenceDiff = semio_s_artifact_cad_cad_model_diff::CadDiff> + From<crate::replace_references::ReplaceReferences>,
{
    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<semio_s_artifact_cad_cad_model_diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
//#endregion 🔖️Mutation
