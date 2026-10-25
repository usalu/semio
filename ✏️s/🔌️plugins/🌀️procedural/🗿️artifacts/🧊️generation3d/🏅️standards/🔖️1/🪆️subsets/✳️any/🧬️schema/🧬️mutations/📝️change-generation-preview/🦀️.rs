//! 📝️ `change-generation-preview` payload — the document-level preview text (`GenerationPlayState.preview_text`,
//! restored when a saved document is loaded), set or cleared.

use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

//#region 🔖️ChangeGenerationPreview
/// 📝️ Sets the document preview text to `text`, or clears it when it is `None`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeGenerationPreview {
    pub text: Option<String>,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for ChangeGenerationPreview
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<ChangeGenerationPreview>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "preview", kind: "change-generation-preview", record: "ChangedGenerationPreview" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match &self.text {
            Some(_) => semio_framework_ui_locale::LocalizedLabel::native("Change the generation preview", "Generierungsvorschau ändern"),
            None => semio_framework_ui_locale::LocalizedLabel::native("Clear the generation preview", "Generierungsvorschau löschen"),
        }
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for ChangeGenerationPreview
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<ChangeGenerationPreview>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::change_generation_preview::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_generation_preview::inverse::inverse(self, base)
    }
}
//#endregion 🔖️ChangeGenerationPreview
