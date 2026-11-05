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
    let base = Wfc2dTransient { source_revision: Vec::new(), assignments: vec![Wfc2dAssignment { slot_id: "room-a".into(), tile_id: "room".into() }], contradiction: false };
    let next = SetSolve { source_revision: Vec::new(), assignments: Vec::new(), contradiction: true };
    let produced = <SetSolve as MutationKind<Wfc2dTransient, Wfc2dTransientMutation>>::diff(&next, &base).diff().clone();
    assert!(produced.contradiction && produced.assignments.is_empty());
    let inverse = <SetSolve as MutationKind<Wfc2dTransient, Wfc2dTransientMutation>>::inverse(&next, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse, vec![Wfc2dTransientMutation::SetSolve(SetSolve { source_revision: Vec::new(), assignments: base.assignments, contradiction: false })]);
}

/// 🔎️ The preview's only lookup finds the solved tile by slot id.
#[test]
fn assigned_tile_finds_the_solved_row() {
    let transient = Wfc2dTransient { source_revision: Vec::new(), assignments: vec![Wfc2dAssignment { slot_id: "corridor".into(), tile_id: "corridor".into() }], contradiction: false };
    assert_eq!(assigned_tile(&transient, "corridor"), Some("corridor"));
    assert_eq!(assigned_tile(&transient, "room-a"), None);
}

/// 🪪️ Portable revision cases fence cached solves even when authored slot and tile identities survive.
#[test]
fn cached_solve_requires_the_exact_render_revision() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧪️tests/🧫️preview-revisions/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let source: Vec<u8> = serde_json::from_value(case["sourceRevision"].clone()).unwrap();
        let revision: Option<[u8;32]> = serde_json::from_value(case["currentRevision"].clone()).unwrap();
        assert_eq!(crate::editor::wfc2d::transient::source_matches(&source, revision.as_ref()), case["visible"].as_bool().unwrap(), "{}", case["id"]);
    }
}
