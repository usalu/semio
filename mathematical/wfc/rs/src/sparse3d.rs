//! 🌌 Sparse-volume adapter: converts a set of *occupied* 3D integer coordinates (not a dense
//! `width × height × depth` bounding box, unlike `crate::grid3d::Grid3dTopology`) into a
//! [`crate::topology::GraphTopology`], emitting an arc between two occupied voxels only where
//! *both* are present in the set. This is the uniform-resolution building block a true
//! variable-resolution octree (multiple cell sizes, level-of-detail transition arcs between a
//! coarse cell and several finer neighbors) would be built on top of — that hierarchical
//! structure is a substantially bigger, genuinely separate feature and is deferred (see this
//! module's scope note in the phase-12 ticket log); what's implemented here is the sparse-to-graph
//! conversion any such structure still needs at its leaves.

use crate::error::TopologyError;
use crate::ids::{NodeId, RelationId};
use crate::topology::{GraphTopology, GraphTopologyBuilder};
use std::collections::HashMap;

// #region 🔖Volume
/// 🌌 A voxel coordinate, `(x, y, z)`, signed so a sparse region can extend in any direction from
/// an arbitrary origin (unlike `Grid3dTopology`'s `0..width` bounded axes).
pub type VoxelCoord = (i32, i32, i32);

/// 🌌 A sparse set of occupied voxels, each assigned a stable `NodeId` in first-seen order.
pub struct SparseVolume {
    occupied: Vec<VoxelCoord>,
    index: HashMap<VoxelCoord, usize>,
}

impl SparseVolume {
    /// 🌌 Builds from an iterator of coordinates, deduplicating while preserving first-seen order
    /// (so `NodeId` assignment is deterministic given a deterministic input order).
    pub fn from_coords(coords: impl IntoIterator<Item = VoxelCoord>) -> Self {
        let mut occupied = Vec::new();
        let mut index = HashMap::new();
        for c in coords {
            index.entry(c).or_insert_with(|| {
                occupied.push(c);
                occupied.len() - 1
            });
        }
        Self { occupied, index }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.occupied.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.occupied.is_empty()
    }

    #[inline]
    pub fn contains(&self, c: VoxelCoord) -> bool {
        self.index.contains_key(&c)
    }

    /// 🌌 The coordinate `n` was assigned (panics if `n` is out of range — every `NodeId` this
    /// crate hands back for this volume is always in range by construction).
    pub fn coord_of(&self, n: NodeId) -> VoxelCoord {
        self.occupied[n.index()]
    }

    pub fn node_of(&self, c: VoxelCoord) -> Option<NodeId> {
        self.index.get(&c).map(|&i| NodeId::from_index(i))
    }

    /// 🌌 Builds a [`GraphTopology`] over exactly this volume's occupied voxels. `face_relations`
    /// pairs each of the (typically 6, for face adjacency) neighbor offsets this volume should
    /// connect through with the already-compiled model [`RelationId`] that offset corresponds to
    /// (e.g. from [`crate::grid3d::declare_stencil_relations_3d`]) — an arc is emitted from a
    /// voxel to its offset-neighbor only when that neighbor is *also* occupied, which is exactly
    /// what makes this "sparse" rather than a dense masked grid.
    pub fn to_graph_topology(&self, face_relations: &[(VoxelCoord, RelationId)]) -> Result<GraphTopology, TopologyError> {
        let mut b = GraphTopologyBuilder::new(self.occupied.len());
        for (i, &c) in self.occupied.iter().enumerate() {
            for &(offset, relation) in face_relations {
                let neighbor = (c.0 + offset.0, c.1 + offset.1, c.2 + offset.2);
                if let Some(j) = self.node_of(neighbor) {
                    b.arc(NodeId::from_index(i), j, relation);
                }
            }
        }
        b.build()
    }
}
// #endregion 🔖Volume

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid3d::{Stencil3d, declare_stencil_relations_3d};
    use crate::model::ModelBuilder;
    use crate::topology::Topology;

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
        use crate::outcome::SolveOutcome;
        use crate::search::{self, SearchConfig};
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
}
// #endregion 🔖Tests
