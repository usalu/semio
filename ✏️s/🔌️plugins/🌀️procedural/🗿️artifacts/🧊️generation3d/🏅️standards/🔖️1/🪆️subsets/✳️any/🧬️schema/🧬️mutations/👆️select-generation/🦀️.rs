//! 👆️ `select-generation` payload — the document-level selection: which generation the DOCUMENT names as selected
//! (`GenerationPlayState.selected_generation_id`, restored when a saved document is loaded), or none. The selected row of
//! a view is config (`SetSelectedGeneration`), never this leaf.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

//#region 🔖️SelectGeneration
/// 👆️ Selects the generation `generation_id` names, or clears the selection when it is `None`.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct SelectGeneration {
    pub generation_id: Option<String>,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for SelectGeneration
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<SelectGeneration>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "selection", kind: "select-generation", record: "SelectedGeneration" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match &self.generation_id {
            Some(id) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Select generation \"{id}\""), &format!("Generierung \"{id}\" auswählen")),
            None => semio_framework_ui_locale::LocalizedLabel::native("Clear the generation selection", "Generierungsauswahl aufheben"),
        }
    }

    fn target(&self) -> Vec<String> {
        self.generation_id.iter().cloned().collect()
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for SelectGeneration
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<SelectGeneration>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::select_generation::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::select_generation::inverse::inverse(self, base)
    }
}
//#endregion 🔖️SelectGeneration
