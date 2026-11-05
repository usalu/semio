//! 📱️ The app transient — the solve lane, and the law that abort does not invent a solve.

use crate::editor::wfc3d::transient::testing::solved_preview;

use crate::editor::wfc3d::transient::{assigned_tile, SetSolve, Wfc3dAssignment, Wfc3dTransient, Wfc3dTransientMutation};

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
    let base = Wfc3dTransient { source_revision: Vec::new(), assignments: vec![Wfc3dAssignment { slot_id: "room-a".into(), tile_id: "room".into() }], contradiction: false };
    let next = SetSolve { source_revision: Vec::new(), assignments: Vec::new(), contradiction: true };
    let produced = <SetSolve as MutationKind<Wfc3dTransient, Wfc3dTransientMutation>>::diff(&next, &base).diff().clone();
    assert!(produced.contradiction && produced.assignments.is_empty());
    let inverse = <SetSolve as MutationKind<Wfc3dTransient, Wfc3dTransientMutation>>::inverse(&next, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse, vec![Wfc3dTransientMutation::SetSolve(SetSolve { source_revision: Vec::new(), assignments: base.assignments, contradiction: false })]);
}

#[test]
fn assigned_tile_finds_the_solved_row() {
    let transient = Wfc3dTransient { source_revision: Vec::new(), assignments: vec![Wfc3dAssignment { slot_id: "corridor".into(), tile_id: "corridor".into() }], contradiction: false };
    assert_eq!(assigned_tile(&transient, "corridor"), Some("corridor"));
    assert_eq!(assigned_tile(&transient, "room-a"), None);
}

#[test]
fn solved_transient_folds_the_inference_without_touching_the_document() {
    let document = crate::examples::tower_stack::snapshot();
    let transient = solved_preview(&document);
    assert!(!transient.contradiction);
    assert_eq!(transient.assignments.len(), document.slots.len());
    assert_eq!(document.seed, crate::examples::tower_stack::SEED, "the document seed is untouched by the solve lane");
}

/// 🪪️ Portable revision cases fence cached solves even when authored slot and tile identities survive.
#[test]
fn cached_solve_requires_the_exact_render_revision() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧪️tests/🧫️preview-revisions/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let source: Vec<u8> = serde_json::from_value(case["sourceRevision"].clone()).unwrap();
        let revision: Option<[u8;32]> = serde_json::from_value(case["currentRevision"].clone()).unwrap();
        assert_eq!(crate::editor::wfc3d::transient::source_matches(&source, revision.as_ref()), case["visible"].as_bool().unwrap(), "{}", case["id"]);
    }
}
