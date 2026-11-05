//! 🧪️ Test-only inferred preview fixtures use the owned headless oracle explicitly.
use super::{Wfc3dTransient, Wfc3dAssignment};

pub fn solved_preview(document: &crate::Wfc3dSnapshot) -> Wfc3dTransient {
    let commit = crate::inferences::solve_with_job(document).expect("test preview fixture must solve through its original caller");
    Wfc3dTransient { source_revision: Vec::new(), contradiction: false, assignments: commit.assignments.into_iter().map(|(slot_id, tile_id)| Wfc3dAssignment { slot_id, tile_id }).collect() }
}
