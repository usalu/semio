//! 🫧️ WFC 3D editor — app-local transient state, shared across every window of one app instance and
//! never part of the document. It carries the LAST SOLVE the preview window paints: the solve is an
//! inference, so its result must never reach the mutation/undo machinery. `MutationDiff` is trivial
//! (`apply` replaces wholesale) because this is a cache, not an edit history.

use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Transient
/// 🫧️ `Wfc3dEditor::Transient` — the inferred assignment, as `slot id -> tile id` pairs, plus the
/// contradiction verdict the preview banner reads.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Wfc3dTransient {
    pub assignments: Vec<(String, String)>,
    pub contradiction: bool,
}

impl protocol::MutationDiff<Wfc3dTransient> for Wfc3dTransient {
    fn apply(&self, _base: &Wfc3dTransient) -> protocol::MutationApplyResult<Wfc3dTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

/// 🔎️ The tile the last solve put in `slot_id`, if any — the preview window's only lookup.
pub fn assigned_tile<'a>(transient: &'a Wfc3dTransient, slot_id: &str) -> Option<&'a str> {
    transient.assignments.iter().find(|(slot, _)| slot == slot_id).map(|(_, tile)| tile.as_str())
}

/// 🧮️ Runs the solve inference and folds it into a fresh transient — the editor's only bridge from
/// the derived world into what the preview paints.
pub fn solved_transient(document: &crate::Wfc3dSnapshot) -> Wfc3dTransient {
    let assignments = crate::inferences::solve_assignments(document);
    Wfc3dTransient { contradiction: assignments.is_empty() && !document.slots.is_empty(), assignments: assignments.into_iter().collect() }
}
//#endregion 🔖️Transient
