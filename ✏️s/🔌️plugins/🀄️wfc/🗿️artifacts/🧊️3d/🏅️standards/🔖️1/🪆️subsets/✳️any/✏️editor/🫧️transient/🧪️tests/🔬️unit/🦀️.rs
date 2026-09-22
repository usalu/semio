//! ️ The app transient — the solve lane, and the law that abort does not invent a solve.

use crate::editor::wfc3d::transient::{assigned_tile, solved_transient, SetSolve, Wfc3dAssignment, Wfc3dTransient, Wfc3dTransientMutation};

#[test]
fn a_fresh_transient_holds_no_solve() {
    let transient = Wfc3dTransient::default();
    assert!(transient.assignments.is_empty());
    assert!(!transient.contradiction);
    assert_eq!(assigned_tile(&transient, "slot-a"), None);
}

#[test]
fn set_solve_replaces_and_inverts() {
    use protocol::MutationKind;
    let base = Wfc3dTransient { assignments: vec![Wfc3dAssignment { slot_id: "room-a".into(), tile_id: "room".into() }], contradiction: false };
    let next = SetSolve { assignments: Vec::new(), contradiction: true };
    let produced = <SetSolve as MutationKind<Wfc3dTransient, Wfc3dTransientMutation>>::diff(&next, &base).diff().clone();
    assert!(produced.contradiction && produced.assignments.is_empty());
    let inverse = <SetSolve as MutationKind<Wfc3dTransient, Wfc3dTransientMutation>>::inverse(&next, &base);
    assert_eq!(inverse, vec![Wfc3dTransientMutation::SetSolve(SetSolve { assignments: base.assignments, contradiction: false })]);
}

#[test]
fn assigned_tile_finds_the_solved_row() {
    let transient = Wfc3dTransient { assignments: vec![Wfc3dAssignment { slot_id: "corridor".into(), tile_id: "corridor".into() }], contradiction: false };
    assert_eq!(assigned_tile(&transient, "corridor"), Some("corridor"));
    assert_eq!(assigned_tile(&transient, "room-a"), None);
}

#[test]
fn solved_transient_folds_the_inference_without_touching_the_document() {
    let document = crate::examples::tower_stack::snapshot();
    let transient = solved_transient(&document);
    assert!(!transient.contradiction);
    assert_eq!(transient.assignments.len(), document.slots.len());
    assert_eq!(document.seed, crate::examples::tower_stack::SEED, "the document seed is untouched by the solve lane");
}
