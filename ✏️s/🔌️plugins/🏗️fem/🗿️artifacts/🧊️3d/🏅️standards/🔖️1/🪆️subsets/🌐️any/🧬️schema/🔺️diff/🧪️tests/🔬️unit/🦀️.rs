//! 🧪️ Unit laws of the fem3d diff algebra: `absorb` coalesces same-key rows, `inverse` is the negative delta, and the absorbed diff applies exactly like its parts in sequence.

use super::*;
use crate::{FemDof, FemLoad, FemLoadCase};

fn node(id: &str, x: f64) -> FemNode {
    FemNode { id: id.into(), x, y: 0.0, z: 0.0 }
}

fn load(id: &str, value: f64) -> FemLoad {
    FemLoad::Nodal { id: id.into(), node_id: "n1".into(), dof: FemDof::Ty, value }
}

fn base() -> Fem3dSnapshot {
    Fem3dSnapshot {
        nodes: vec![node("n1", 0.0), node("n2", 1.0), node("n3", 2.0)],
        load_cases: vec![FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![load("l1", -1.0), load("l2", -2.0)], self_weight: false }],
        ..Default::default()
    }
}

fn nodes(delta: Fem3dNodesDelta) -> Fem3dDiff {
    Fem3dDiff { nodes: Some(delta), ..Default::default() }
}

fn patch_node(id: &str, x: f64) -> Fem3dDiff {
    nodes(Fem3dNodesDelta { patched: vec![Fem3dNodesPatchEntry { id: id.into(), item: node(id, x) }], ..Default::default() })
}

fn apply(diff: &Fem3dDiff, base: &Fem3dSnapshot) -> Fem3dSnapshot {
    protocol::apply_diff(diff, base).expect("the diff applies")
}

fn absorbed(first: &Fem3dDiff, second: &Fem3dDiff) -> Fem3dDiff {
    let mut sum = first.clone();
    protocol::MutationDiff::absorb(&mut sum, second.clone());
    sum
}

/// 🩹 Two patches of one id fold into one patch whose application equals the sequence.
#[test]
fn absorb_folds_two_patches_of_one_id_into_one() {
    let (first, second) = (patch_node("n1", 5.0), patch_node("n1", 7.0));
    let sum = absorbed(&first, &second);
    assert_eq!(sum.nodes.as_ref().expect("nodes").patched.len(), 1);
    assert_eq!(apply(&sum, &base()), apply(&second, &apply(&first, &base())));
}

/// ⚖️ A row added and then removed is no row at all.
#[test]
fn absorb_cancels_an_added_then_removed_row() {
    let first = nodes(Fem3dNodesDelta { added: vec![node("n9", 9.0)], ..Default::default() });
    let second = nodes(Fem3dNodesDelta { removed: vec!["n9".into()], ..Default::default() });
    let sum = absorbed(&first, &second);
    assert_eq!(sum.nodes, None);
    assert_eq!(apply(&sum, &base()), base());
}

/// 🔁️ A row removed and then added again is a replace: removed and added together, applied in sequence order.
#[test]
fn absorb_turns_removed_then_added_into_a_replace() {
    let first = nodes(Fem3dNodesDelta { removed: vec!["n2".into()], ..Default::default() });
    let second = nodes(Fem3dNodesDelta { added: vec![node("n2", 8.0)], ..Default::default() });
    let sum = absorbed(&first, &second);
    let delta = sum.nodes.as_ref().expect("nodes");
    assert_eq!((delta.removed.clone(), delta.added.len()), (vec!["n2".to_string()], 1));
    assert_eq!(apply(&sum, &base()), apply(&second, &apply(&first, &base())));
}

/// ➕️ A patch on a row the same diff added folds into the added row.
#[test]
fn absorb_folds_a_patch_into_the_added_row() {
    let first = nodes(Fem3dNodesDelta { added: vec![node("n9", 9.0)], ..Default::default() });
    let second = patch_node("n9", 4.0);
    let sum = absorbed(&first, &second);
    let delta = sum.nodes.as_ref().expect("nodes");
    assert_eq!((delta.added.clone(), delta.patched.len()), (vec![node("n9", 4.0)], 0));
    assert_eq!(apply(&sum, &base()), apply(&second, &apply(&first, &base())));
}

/// 🏋️ Owned-field load-case patches compose field by field and row by row.
#[test]
fn absorb_composes_load_case_patches() {
    let rename = Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { patched: vec![Fem3dLoadCasesPatchEntry { id: "dead".into(), patch: Fem3dLoadCasePatch { name: Some("Self".into()), ..Default::default() } }], ..Default::default() }), ..Default::default() };
    let add = Fem3dDiff {
        load_cases: Some(Fem3dLoadCasesDelta {
            patched: vec![Fem3dLoadCasesPatchEntry { id: "dead".into(), patch: Fem3dLoadCasePatch { loads: Some(Fem3dLoadsDelta { added: vec![load("l3", -3.0)], ..Default::default() }), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let sum = absorbed(&rename, &add);
    let delta = sum.load_cases.as_ref().expect("load cases");
    assert_eq!(delta.patched.len(), 1);
    assert_eq!(delta.patched[0].patch.name.as_deref(), Some("Self"));
    assert_eq!(apply(&sum, &base()), apply(&add, &apply(&rename, &base())));
}

/// 🎛️ Analysis patches keep the latest value of every field they set.
#[test]
fn absorb_composes_analysis_patches() {
    let first = Fem3dDiff { analysis: Some(Fem3dAnalysisPatch { modal_count: Some(5), ..Default::default() }), ..Default::default() };
    let second = Fem3dDiff { analysis: Some(Fem3dAnalysisPatch { modal_count: Some(6), deformation_scale: Some(10.0), ..Default::default() }), ..Default::default() };
    let sum = absorbed(&first, &second);
    assert_eq!(sum.analysis, Some(Fem3dAnalysisPatch { modal_count: Some(6), buckling_count: None, deformation_scale: Some(10.0) }));
}

/// ↩️ The inverse applied after the diff restores the base, for patches, adds, removes and load rows alike.
#[test]
fn inverse_restores_the_base() {
    let cases = [
        patch_node("n2", 3.0),
        nodes(Fem3dNodesDelta { added: vec![node("n4", 4.0)], ..Default::default() }),
        nodes(Fem3dNodesDelta { removed: vec!["n2".into()], ..Default::default() }),
        Fem3dDiff { analysis: Some(Fem3dAnalysisPatch { modal_count: Some(9), ..Default::default() }), ..Default::default() },
        Fem3dDiff {
            load_cases: Some(Fem3dLoadCasesDelta {
                patched: vec![Fem3dLoadCasesPatchEntry { id: "dead".into(), patch: Fem3dLoadCasePatch { self_weight: Some(true), loads: Some(Fem3dLoadsDelta { removed: vec!["l1".into()], ..Default::default() }), ..Default::default() } }],
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

/// 🧭️ `between` carries one snapshot to the other and is empty between equal snapshots.
#[test]
fn between_carries_one_snapshot_to_the_other() {
    let mut other = base();
    other.nodes.remove(0);
    other.nodes.push(node("n7", 7.0));
    other.load_cases[0].loads.remove(1);
    other.analysis.modal_count = 8;
    let diff = <Fem3dDiff as protocol::DiffAlgebra<Fem3dSnapshot>>::between(&base(), &other);
    assert_eq!(apply(&diff, &base()), other);
    assert!(<Fem3dDiff as protocol::DiffAlgebra<Fem3dSnapshot>>::is_empty(&<Fem3dDiff as protocol::DiffAlgebra<Fem3dSnapshot>>::between(&base(), &base())));
}
