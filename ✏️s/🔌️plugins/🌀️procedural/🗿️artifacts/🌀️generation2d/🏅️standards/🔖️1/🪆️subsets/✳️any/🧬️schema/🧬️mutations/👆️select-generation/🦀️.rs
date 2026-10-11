//! 👆️ `select-generation` payload — the document-level selection: which generation the DOCUMENT names as selected
//! (`GenerationPlayState.selected_generation_id`, restored when a saved document is loaded), or none. The selected row of
//! a view is config (`SetSelectedGeneration`), never this leaf.

use crate::standards::v1::subsets::any::schema::diff::Generation2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SelectGeneration
/// 👆️ Selects the generation `generation_id` names, or clears the selection when it is `None`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SelectGeneration {
    pub generation_id: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn select_generation(generation_id: Option<String>) -> Generation2dMutation {
    Generation2dMutation::SelectGeneration(SelectGeneration { generation_id })
}

impl protocol::MutationKind<Generation2dSnapshot, Generation2dMutation> for SelectGeneration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "selection", kind: "select-generation", record: "SelectedGeneration" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Generation2dMutation>, semio_framework_value::ValueError> {
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
