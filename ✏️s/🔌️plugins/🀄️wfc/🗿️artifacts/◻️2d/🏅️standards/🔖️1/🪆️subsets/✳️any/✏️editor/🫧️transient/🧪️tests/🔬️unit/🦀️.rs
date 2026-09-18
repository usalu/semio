//! 🧪️ The app transient — the solve lane, and the law that it never reaches the document.

use crate::editor::wfc2d::transient::{assigned_tile, SetSolve, Wfc2dAssignment, Wfc2dTransient, Wfc2dTransientMutation};

#[test]
fn a_fresh_transient_holds_no_solve() {
    let transient = Wfc2dTransient::default();
    assert!(transient.assignments.is_empty());
    assert!(!transient.contradiction);
    assert_eq!(assigned_tile(&transient, "slot-a"), None);
}

/// 🏁️ `set-solve` replaces the lane wholesale and inverts back to what was there before.
#[test]
fn set_solve_replaces_and_inverts() {
    use protocol::MutationKind;
    let base = Wfc2dTransient { assignments: vec![Wfc2dAssignment { slot_id: "room-a".into(), tile_id: "room".into() }], contradiction: false };
    let next = SetSolve { assignments: Vec::new(), contradiction: true };
    let produced = <SetSolve as MutationKind<Wfc2dTransient, Wfc2dTransientMutation>>::diff(&next, &base).diff().clone();
    assert!(produced.contradiction && produced.assignments.is_empty());
    let inverse = <SetSolve as MutationKind<Wfc2dTransient, Wfc2dTransientMutation>>::inverse(&next, &base);
    assert_eq!(inverse, vec![Wfc2dTransientMutation::SetSolve(SetSolve { assignments: base.assignments, contradiction: false })]);
}

/// 🔎️ The preview's only lookup finds the solved tile by slot id.
#[test]
fn assigned_tile_finds_the_solved_row() {
    let transient = Wfc2dTransient { assignments: vec![Wfc2dAssignment { slot_id: "corridor".into(), tile_id: "corridor".into() }], contradiction: false };
    assert_eq!(assigned_tile(&transient, "corridor"), Some("corridor"));
    assert_eq!(assigned_tile(&transient, "room-a"), None);
}
