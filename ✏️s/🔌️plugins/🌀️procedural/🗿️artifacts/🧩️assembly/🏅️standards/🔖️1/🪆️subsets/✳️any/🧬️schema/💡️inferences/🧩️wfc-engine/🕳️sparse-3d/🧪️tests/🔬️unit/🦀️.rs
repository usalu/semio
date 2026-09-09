use super::*;
use crate::wfc_engine::grid3d::{declare_stencil_relations_3d, Stencil3d};
use crate::wfc_engine::model::ModelBuilder;
use crate::wfc_engine::topology::Topology;

fn face6_relations(b: &mut ModelBuilder) -> Vec<(VoxelCoord, RelationId)> {
    let rels = declare_stencil_relations_3d(b, &Stencil3d::Face6).unwrap();
    Stencil3d::Face6.offsets().into_iter().zip(rels).collect()
}

#[test]
fn from_coords_dedups_and_assigns_stable_first_seen_ids() {
    let volume = SparseVolume::from_coords([(0, 0, 0), (1, 0, 0), (0, 0, 0), (0, 1, 0)]);
    assert_eq!(volume.len(), 3);
    assert_eq!(volume.node_of((0, 0, 0)), Some(NodeId(0)));
    assert_eq!(volume.node_of((1, 0, 0)), Some(NodeId(1)));
    assert_eq!(volume.node_of((0, 1, 0)), Some(NodeId(2)));
    assert_eq!(volume.node_of((5, 5, 5)), None);
    assert!(volume.contains((0, 0, 0)));
    assert!(volume.contains((1, 0, 0)));
    assert!(volume.contains((0, 1, 0)));
    assert!(!volume.contains((5, 5, 5)));
    assert_eq!(volume.coord_of(NodeId(1)), (1, 0, 0));
}

#[test]
fn to_graph_topology_only_connects_occupied_neighbors() {
    let mut b = ModelBuilder::new();
    let rels = face6_relations(&mut b);
    // An L-shape: (0,0,0), (1,0,0), (1,1,0) — (0,0,0) and (1,1,0) are NOT face-adjacent, and
    // the "missing" cell (0,1,0) must not silently create a phantom connection either.
    let volume = SparseVolume::from_coords([(0, 0, 0), (1, 0, 0), (1, 1, 0)]);
    let topo = volume.to_graph_topology(&rels).unwrap();

    assert_eq!(topo.node_count(), 3);
    let origin = volume.node_of((0, 0, 0)).unwrap();
    let mid = volume.node_of((1, 0, 0)).unwrap();
    let corner = volume.node_of((1, 1, 0)).unwrap();

    let mut origin_neighbors = Vec::new();
    topo.for_each_out_arc(origin, |m, _r| origin_neighbors.push(m));
    assert_eq!(origin_neighbors, vec![mid], "origin should only reach mid, not the non-adjacent corner");

    let mut mid_neighbors = Vec::new();
    topo.for_each_out_arc(mid, |m, _r| mid_neighbors.push(m));
    mid_neighbors.sort_by_key(|n| n.get());
    let mut expected = vec![origin, corner];
    expected.sort_by_key(|n| n.get());
    assert_eq!(mid_neighbors, expected);
}

#[test]
fn empty_volume_builds_a_zero_node_topology() {
    let mut b = ModelBuilder::new();
    let rels = face6_relations(&mut b);
    let volume = SparseVolume::from_coords(std::iter::empty());
    assert!(volume.is_empty());
    let topo = volume.to_graph_topology(&rels).unwrap();
    assert_eq!(topo.node_count(), 0);
}

#[test]
fn a_single_isolated_voxel_has_no_arcs() {
    let mut b = ModelBuilder::new();
    let rels = face6_relations(&mut b);
    let volume = SparseVolume::from_coords([(3, -2, 7)]);
    let topo = volume.to_graph_topology(&rels).unwrap();
    assert_eq!(topo.node_count(), 1);
    let mut neighbors = Vec::new();
    topo.for_each_out_arc(NodeId(0), |m, _r| neighbors.push(m));
    assert!(neighbors.is_empty());
}

#[test]
fn negative_coordinates_work_like_any_other_origin() {
    let mut b = ModelBuilder::new();
    let rels = face6_relations(&mut b);
    let volume = SparseVolume::from_coords([(-1, -1, -1), (0, -1, -1)]);
    let topo = volume.to_graph_topology(&rels).unwrap();
    let a = volume.node_of((-1, -1, -1)).unwrap();
    let b_node = volume.node_of((0, -1, -1)).unwrap();
    let mut neighbors = Vec::new();
    topo.for_each_out_arc(a, |m, _r| neighbors.push(m));
    assert_eq!(neighbors, vec![b_node]);
}

#[test]
fn sparse_graph_solves_through_the_ordinary_kernel() {
    use crate::wfc_engine::outcome::SolveOutcome;
    use crate::wfc_engine::search::{self, SearchConfig};
    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(1.0);
    let rels = face6_relations(&mut b);
    for &(_, r) in &rels {
        b.allow_mirrored(r, black, white);
    }
    let model = b.compile().unwrap();
    let volume = SparseVolume::from_coords([(0, 0, 0), (1, 0, 0), (2, 0, 0)]);
    let topo = volume.to_graph_topology(&rels).unwrap();
    let config = SearchConfig::default();
    assert!(matches!(search::solve(&model, &topo, &config, 1, None, &[]), SolveOutcome::Solved(_)));
}
