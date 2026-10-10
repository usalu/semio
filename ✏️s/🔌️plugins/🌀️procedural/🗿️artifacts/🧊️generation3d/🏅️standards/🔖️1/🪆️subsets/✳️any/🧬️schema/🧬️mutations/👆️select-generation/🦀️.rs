//! 👆️ `select-generation` payload — the document-level selection: which generation the DOCUMENT names as selected
//! (`GenerationPlayState.selected_generation_id`, restored when a saved document is loaded), or none. The selected row of
//! a view is config (`SetSelectedGeneration`), never this leaf.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;

//#region 🔖️SelectGeneration
/// 👆️ Selects the generation `generation_id` names, or clears the selection when it is `None`.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SelectGeneration {
    pub generation_id: Option<String>,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for SelectGeneration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "selection", kind: "select-generation", record: "SelectedGeneration" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }

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
//#endregion 🔖️SelectGeneration
