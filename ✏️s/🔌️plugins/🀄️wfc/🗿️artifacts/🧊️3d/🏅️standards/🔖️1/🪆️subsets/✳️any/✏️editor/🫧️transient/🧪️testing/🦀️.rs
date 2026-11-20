//! 🧪️ Test-only inferred preview fixtures use the owned headless oracle explicitly.
use super::{Wfc3dTransient, Wfc3dAssignment};

/// 🔬️ The preview a solve of `document` leaves behind; an unsatisfiable document is the contradiction verdict, not a fixture failure.
pub fn solved_preview(document: &crate::Wfc3dSnapshot) -> Wfc3dTransient {
    match crate::inferences::solve_with_job(document) {
        Ok(commit) => Wfc3dTransient { source_revision: Vec::new(), contradiction: false, assignments: commit.assignments.into_iter().map(|(slot_id, tile_id)| Wfc3dAssignment { slot_id, tile_id }).collect() },
        Err(failure) if failure.message() == "wfc-unsatisfiable" => Wfc3dTransient { contradiction: true, ..Wfc3dTransient::default() },
        Err(failure) => panic!("test preview fixture must solve through its original caller: {failure:?}"),
    }
}
