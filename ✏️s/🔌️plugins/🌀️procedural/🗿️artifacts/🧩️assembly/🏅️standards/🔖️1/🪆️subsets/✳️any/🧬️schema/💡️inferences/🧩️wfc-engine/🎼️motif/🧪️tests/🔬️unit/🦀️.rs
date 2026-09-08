
use super::*;
use crate::wfc_engine::ids::RelationId;
use crate::wfc_engine::topology::GraphTopologyBuilder;

#[test]
fn isomorphic_rooted_neighborhoods_get_identical_signatures() {
    // Two disjoint triangles: every node in each triangle has the exact same rooted
    // 1-hop neighborhood shape (two same-labeled neighbors), so 1 round of refinement must
    // make all six nodes' signatures identical.
    let mut b = GraphTopologyBuilder::new(6);
    let r = RelationId(0);
    for &(a, c) in &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)] {
        b.arc(NodeId(a), NodeId(c), r);
        b.arc(NodeId(c), NodeId(a), r);
    }
    let topo = b.build().unwrap();
    let labels = vec![7u64; 6]; // uniform base label
    let colors = refine_colors(&topo, &labels, 1);
    assert!(colors.iter().all(|&c| c == colors[0]));
}

#[test]
fn structurally_different_neighborhoods_get_different_signatures() {
    // A 4-node "star" (node0 connects to 1,2,3; they don't connect to each other): the semio_hub
    // has 3 neighbors, the leaves have 1 each — must not collide after refinement.
    let mut b = GraphTopologyBuilder::new(4);
    let r = RelationId(0);
    for leaf in [1, 2, 3] {
        b.arc(NodeId(0), NodeId(leaf), r);
        b.arc(NodeId(leaf), NodeId(0), r);
    }
    let topo = b.build().unwrap();
    let labels = vec![1u64; 4];
    let colors = refine_colors(&topo, &labels, 1);
    assert_ne!(colors[0], colors[1], "semio_hub (degree 3) must differ from a leaf (degree 1)");
    assert_eq!(colors[1], colors[2]);
    assert_eq!(colors[2], colors[3], "the three leaves are structurally identical");
}

#[test]
fn different_base_labels_propagate_into_different_signatures() {
    let mut b = GraphTopologyBuilder::new(2);
    let r = RelationId(0);
    b.arc(NodeId(0), NodeId(1), r);
    b.arc(NodeId(1), NodeId(0), r);
    let topo = b.build().unwrap();
    let same_labels = refine_colors(&topo, &[1, 1], 1);
    let different_labels = refine_colors(&topo, &[1, 2], 1);
    assert_eq!(same_labels[0], same_labels[1]);
    assert_ne!(different_labels[0], different_labels[1]);
}

#[test]
fn zero_rounds_is_the_identity_on_labels_modulo_hashing() {
    // With 0 rounds, colors are exactly `initial_labels` unchanged (no hashing applied at all).
    let b = GraphTopologyBuilder::new(3);
    let topo = b.build().unwrap();
    let labels = vec![10u64, 20, 10];
    assert_eq!(refine_colors(&topo, &labels, 0), labels);
}

#[test]
fn refine_colors_is_deterministic() {
    let mut b = GraphTopologyBuilder::new(5);
    let r = RelationId(0);
    for &(a, c) in &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)] {
        b.arc(NodeId(a), NodeId(c), r);
        b.arc(NodeId(c), NodeId(a), r);
    }
    let topo = b.build().unwrap();
    let labels = vec![1u64, 2, 1, 2, 3];
    assert_eq!(refine_colors(&topo, &labels, 3), refine_colors(&topo, &labels, 3));
}

#[test]
fn canonicalize_produces_dense_first_seen_ids() {
    let (ids, k) = canonicalize(&[100, 200, 100, 300, 200]);
    assert_eq!(ids, vec![0, 1, 0, 2, 1]);
    assert_eq!(k, 3);
}

#[test]
fn canonicalize_of_empty_input_is_empty() {
    let (ids, k) = canonicalize(&[]);
    assert!(ids.is_empty());
    assert_eq!(k, 0);
}
