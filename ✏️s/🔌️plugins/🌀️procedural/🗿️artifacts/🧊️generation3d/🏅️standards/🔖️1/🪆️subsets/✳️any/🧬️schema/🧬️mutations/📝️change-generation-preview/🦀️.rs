//! 📝️ `change-generation-preview` payload — the document-level preview text (`GenerationPlayState.preview_text`,
//! restored when a saved document is loaded), set or cleared.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;

//#region 🔖️ChangeGenerationPreview
/// 📝️ Sets the document preview text to `text`, or clears it when it is `None`.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeGenerationPreview {
    pub text: Option<String>,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for ChangeGenerationPreview {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "preview", kind: "change-generation-preview", record: "ChangedGenerationPreview" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match &self.text {
            Some(_) => semio_framework_ui_locale::LocalizedLabel::native("Change the generation preview", "Generierungsvorschau ändern"),
            None => semio_framework_ui_locale::LocalizedLabel::native("Clear the generation preview", "Generierungsvorschau löschen"),
        }
    }
}
//#endregion 🔖️ChangeGenerationPreview
