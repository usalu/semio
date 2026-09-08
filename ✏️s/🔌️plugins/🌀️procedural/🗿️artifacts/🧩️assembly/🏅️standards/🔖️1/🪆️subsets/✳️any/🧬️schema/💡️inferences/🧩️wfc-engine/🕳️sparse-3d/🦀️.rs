//! 🌌️ Sparse-volume adapter: converts a set of *occupied* 3D integer coordinates (not a dense
//! `width × height × depth` bounding box, unlike `crate::wfc_engine::grid3d::Grid3dTopology`) into a
//! [`crate::wfc_engine::topology::GraphTopology`], emitting an arc between two occupied voxels only where
//! *both* are present in the set. This is the uniform-resolution building block a true
//! variable-resolution octree (multiple cell sizes, level-of-detail transition arcs between a
//! coarse cell and several finer neighbors) would be built on top of — that hierarchical
//! structure is a substantially bigger, genuinely separate feature and is deferred (see this
//! module's scope note in the phase-12 ticket log); what's implemented here is the sparse-to-graph
//! conversion any such structure still needs at its leaves.

use crate::wfc_engine::error::TopologyError;
use crate::wfc_engine::ids::{NodeId, RelationId};
use crate::wfc_engine::topology::{GraphTopology, GraphTopologyBuilder};
use std::collections::HashMap;

// #region 🔖️Volume
/// 🌌️ A voxel coordinate, `(x, y, z)`, signed so a sparse region can extend in any direction from
/// an arbitrary origin (unlike `Grid3dTopology`'s `0..width` bounded axes).
pub type VoxelCoord = (i32, i32, i32);

/// 🌌️ A sparse set of occupied voxels, each assigned a stable `NodeId` in first-seen order.
pub struct SparseVolume {
    occupied: Vec<VoxelCoord>,
    index: HashMap<VoxelCoord, usize>,
}

impl SparseVolume {
    /// 🌌️ Builds from an iterator of coordinates, deduplicating while preserving first-seen order
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

    /// 🌌️ The coordinate `n` was assigned (panics if `n` is out of range — every `NodeId` this
    /// crate hands back for this volume is always in range by construction).
    pub fn coord_of(&self, n: NodeId) -> VoxelCoord {
        self.occupied[n.index()]
    }

    pub fn node_of(&self, c: VoxelCoord) -> Option<NodeId> {
        self.index.get(&c).map(|&i| NodeId::from_index(i))
    }

    /// 🌌️ Builds a [`GraphTopology`] over exactly this volume's occupied voxels. `face_relations`
    /// pairs each of the (typically 6, for face adjacency) neighbor offsets this volume should
    /// connect through with the already-compiled model [`RelationId`] that offset corresponds to
    /// (e.g. from [`crate::wfc_engine::grid3d::declare_stencil_relations_3d`]) — an arc is emitted from a
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
// #endregion 🔖️Volume

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
