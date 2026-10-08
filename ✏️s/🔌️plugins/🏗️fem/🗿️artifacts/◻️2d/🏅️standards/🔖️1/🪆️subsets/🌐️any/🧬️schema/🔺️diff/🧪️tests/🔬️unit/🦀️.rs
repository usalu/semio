//! 🧪️ Unit laws of the fem2d diff algebra: `absorb` coalesces same-key rows, `inverse` is the negative delta, and the absorbed diff applies exactly like its parts in sequence.

use super::*;
use crate::{FemDof, FemLoad, FemLoadCase};

fn node(id: &str, x: f64) -> FemNode {
    FemNode { id: id.into(), x, y: 0.0 }
}

fn load(id: &str, value: f64) -> FemLoad {
    FemLoad::Nodal { id: id.into(), node_id: "n1".into(), dof: FemDof::Ty, value }
}

fn base() -> Fem2dSnapshot {
    Fem2dSnapshot {
        nodes: vec![node("n1", 0.0), node("n2", 1.0), node("n3", 2.0)],
        load_cases: vec![FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![load("l1", -1.0), load("l2", -2.0)], self_weight: false }],
        ..Default::default()
    }
}

fn nodes(delta: Fem2dNodesDelta) -> Fem2dDiff {
    Fem2dDiff { nodes: Some(delta), ..Default::default() }
}

fn patch_node(id: &str, x: f64) -> Fem2dDiff {
    nodes(Fem2dNodesDelta { modified: vec![Fem2dNodesModification { id: id.into(), patch: node(id, x) }], ..Default::default() })
}

fn apply(diff: &Fem2dDiff, base: &Fem2dSnapshot) -> Fem2dSnapshot {
    protocol::apply_diff(diff, base).expect("the diff applies")
}

fn absorbed(first: &Fem2dDiff, second: &Fem2dDiff) -> Fem2dDiff {
    let mut sum = first.clone();
    protocol::MutationDiff::absorb(&mut sum, second.clone());
    sum
}

/// 🩹 Two patches of one id fold into one patch whose application equals the sequence.
#[test]
fn absorb_folds_two_patches_of_one_id_into_one() {
    let (first, second) = (patch_node("n1", 5.0), patch_node("n1", 7.0));
    let sum = absorbed(&first, &second);
    assert_eq!(sum.nodes.as_ref().expect("nodes").modified.len(), 1);
    assert_eq!(apply(&sum, &base()), apply(&second, &apply(&first, &base())));
}

fn insert(index: usize, row: FemNode) -> Fem2dNodesDelta {
    Fem2dNodesDelta { inserted: vec![Fem2dNodeInsertion { index, row }], ..Default::default() }
}

fn remove(id: &str, index: usize) -> Fem2dNodesDelta {
    Fem2dNodesDelta { removed: vec![Fem2dNodeRemoval { id: id.into(), index }], ..Default::default() }
}

fn relocate(id: &str, from: usize, to: usize) -> Fem2dNodesDelta {
    Fem2dNodesDelta { moved: vec![Fem2dNodeRelocation { id: id.into(), from, to }], ..Default::default() }
}

fn node_ids(snapshot: &Fem2dSnapshot) -> Vec<&str> {
    snapshot.nodes.iter().map(|node| node.id.as_str()).collect()
}

/// ⚖️ A row inserted and then removed is no row at all.
#[test]
fn absorb_cancels_an_inserted_then_removed_row() {
    let sum = absorbed(&nodes(insert(3, node("n9", 9.0))), &nodes(remove("n9", 3)));
    assert_eq!(sum.nodes, None);
    assert_eq!(apply(&sum, &base()), base());
}

/// 🔁️ A row removed and then inserted again is a replacement: one removal at the base index and one insertion at the final index.
#[test]
fn absorb_turns_removed_then_inserted_into_a_replacement() {
    let first = nodes(remove("n2", 1));
    let second = nodes(insert(1, node("n2", 8.0)));
    let sum = absorbed(&first, &second);
    let delta = sum.nodes.as_ref().expect("nodes");
    assert_eq!((delta.removed.len(), delta.inserted.len(), delta.moved.len()), (1, 1, 0));
    assert_eq!(apply(&sum, &base()), apply(&second, &apply(&first, &base())));
}

/// ➕️ A patch on a row the same diff inserted stays its own modified entry; the inserted row is not rewritten outside the central applier.
#[test]
fn absorb_keeps_a_patch_of_an_inserted_row_as_its_own_entry() {
    let first = nodes(insert(1, node("n9", 9.0)));
    let second = patch_node("n9", 4.0);
    let sum = absorbed(&first, &second);
    let delta = sum.nodes.as_ref().expect("nodes");
    assert_eq!((delta.inserted.clone(), delta.modified.len()), (vec![Fem2dNodeInsertion { index: 1, row: node("n9", 9.0) }], 1));
    assert_eq!(apply(&sum, &base()), apply(&second, &apply(&first, &base())));
}

/// 📍️ An insertion lands at its index and a middle removal restores at the same index through the inverse.
#[test]
fn positional_rows_place_and_restore_rows_in_the_middle() {
    let inserted = nodes(insert(1, node("n9", 9.0)));
    let after = apply(&inserted, &base());
    assert_eq!(node_ids(&after), ["n1", "n9", "n2", "n3"]);
    let removed = nodes(remove("n2", 1));
    let after = apply(&removed, &base());
    assert_eq!(node_ids(&after), ["n1", "n3"]);
    let inverse = protocol::DiffAlgebra::inverse(&removed, &base());
    assert_eq!(inverse.nodes.as_ref().expect("nodes").inserted, vec![Fem2dNodeInsertion { index: 1, row: node("n2", 1.0) }]);
    assert_eq!(node_ids(&apply(&inverse, &after)), ["n1", "n2", "n3"]);
}

/// ↕️ Moves compose into one move, and a move back to the start applies as the identity.
#[test]
fn absorb_composes_moves() {
    let first = nodes(relocate("n1", 0, 2));
    let second = nodes(relocate("n1", 2, 1));
    let sum = absorbed(&first, &second);
    assert_eq!(sum.nodes.as_ref().expect("nodes").moved, vec![Fem2dNodeRelocation { id: "n1".into(), from: 0, to: 1 }]);
    assert_eq!(apply(&sum, &base()), apply(&second, &apply(&first, &base())));
    let back = nodes(relocate("n1", 2, 0));
    assert_eq!(apply(&absorbed(&first, &back), &base()), base());
}

/// 🔀️ A move followed by a removal of the same row removes it at its base index.
#[test]
fn absorb_turns_moved_then_removed_into_a_base_removal() {
    let first = nodes(relocate("n1", 0, 2));
    let second = nodes(remove("n1", 2));
    let sum = absorbed(&first, &second);
    assert_eq!(sum.nodes.as_ref().expect("nodes").removed, vec![Fem2dNodeRemoval { id: "n1".into(), index: 0 }]);
    assert_eq!(apply(&sum, &base()), apply(&second, &apply(&first, &base())));
}

/// ⚖️ The inverse steps sum to the inverse of the sum.
#[test]
fn the_sum_of_inverses_is_the_inverse_of_the_sum() {
    let first = nodes(insert(1, node("n9", 9.0)));
    let after_first = apply(&first, &base());
    let second = nodes(remove("n3", 3));
    let sum = absorbed(&first, &second);
    let undo_second = protocol::DiffAlgebra::inverse(&second, &after_first);
    let undo_first = protocol::DiffAlgebra::inverse(&first, &base());
    assert_eq!(absorbed(&undo_second, &undo_first), protocol::DiffAlgebra::inverse(&sum, &base()));
}

/// 🏋️ Owned-field load-case patches compose field by field and row by row.
#[test]
fn absorb_composes_load_case_patches() {
    let rename = Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { modified: vec![Fem2dLoadCasesModification { id: "dead".into(), patch: Fem2dLoadCasePatch { name: Some("Self".into()), ..Default::default() } }], ..Default::default() }), ..Default::default() };
    let add = Fem2dDiff {
        load_cases: Some(Fem2dLoadCasesDelta {
            modified: vec![Fem2dLoadCasesModification { id: "dead".into(), patch: Fem2dLoadCasePatch { loads: Some(Fem2dLoadsDelta { inserted: vec![Fem2dLoadInsertion { index: 2, row: load("l3", -3.0) }], ..Default::default() }), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let sum = absorbed(&rename, &add);
    let delta = sum.load_cases.as_ref().expect("load cases");
    assert_eq!(delta.modified.len(), 1);
    assert_eq!(delta.modified[0].patch.name.as_deref(), Some("Self"));
    assert_eq!(apply(&sum, &base()), apply(&add, &apply(&rename, &base())));
}

/// 🎛️ Analysis patches keep the latest value of every field they set.
#[test]
fn absorb_composes_analysis_patches() {
    let first = Fem2dDiff { analysis: Some(Fem2dAnalysisPatch { modal_count: Some(5), ..Default::default() }), ..Default::default() };
    let second = Fem2dDiff { analysis: Some(Fem2dAnalysisPatch { modal_count: Some(6), deformation_scale: Some(10.0), ..Default::default() }), ..Default::default() };
    let sum = absorbed(&first, &second);
    assert_eq!(sum.analysis, Some(Fem2dAnalysisPatch { modal_count: Some(6), buckling_count: None, deformation_scale: Some(10.0) }));
}

/// ↩️ The inverse applied after the diff restores the base, for patches, adds, removes and load rows alike.
#[test]
fn inverse_restores_the_base() {
    let cases = [
        patch_node("n2", 3.0),
        nodes(insert(3, node("n4", 4.0))),
        nodes(remove("n2", 1)),
        nodes(relocate("n3", 2, 0)),
        Fem2dDiff { analysis: Some(Fem2dAnalysisPatch { modal_count: Some(9), ..Default::default() }), ..Default::default() },
        Fem2dDiff {
            load_cases: Some(Fem2dLoadCasesDelta {
                modified: vec![Fem2dLoadCasesModification { id: "dead".into(), patch: Fem2dLoadCasePatch { self_weight: Some(true), loads: Some(Fem2dLoadsDelta { removed: vec![Fem2dLoadRemoval { id: "l1".into(), index: 0 }], ..Default::default() }), ..Default::default() } }],
                ..Default::default()
            }),
            ..Default::default()
        },
    ];
    for diff in cases {
        let after = apply(&diff, &base());
        assert_ne!(after, base());
        let inverse = protocol::DiffAlgebra::inverse(&diff, &base());
        assert_eq!(apply(&inverse, &after), base(), "inverse of {diff:?}");
    }
}

