//! 🗺️ Dense 2D grid topology: arithmetic neighbor lookup (zero adjacency storage — a `for_each_out_arc`
//! call is a handful of integer additions and a boundary-wrap branch, not a CSR slice walk).
//! Relations are supplied by the caller (via [`declare_stencil_relations`]) rather than assumed
//! from stencil-offset order, so a grid model can freely mix stencil relations with others.

use crate::wfc_engine::error::{ModelError, TopologyError};
use crate::wfc_engine::ids::{NodeId, PatternId, RegionId, RelationId};
use crate::wfc_engine::model::ModelBuilder;
use crate::wfc_engine::tiled::TiledModelBuilder;
use crate::wfc_engine::topology::Topology;

// #region 🔖️Stencil
/// 🗺️ Which offsets count as "neighbors" of a 2D cell. Every built-in stencil is symmetric (each
/// offset's negation is also present) so a single relation-per-offset naturally gets a matching
/// inverse; [`Stencil2d::Custom`] must uphold that itself or [`declare_stencil_relations`] rejects it.
#[derive(Clone, PartialEq, Debug)]
pub enum Stencil2d {
    /// 🗺️ 4-neighbor: N, S, E, W.
    VonNeumann,
    /// 🗺️ 8-neighbor: von Neumann plus the four diagonals.
    Moore,
    /// 🗺️ 6-neighbor axial hex grid (cells addressed by `(x, y)` axial coordinates directly).
    Hex,
    /// 🗺️ An arbitrary offset list, each entry's negation required to also be present.
    Custom(Vec<(i32, i32)>),
}

impl Stencil2d {
    pub async fn offsets(&self) -> Vec<(i32, i32)> {
        match self {
            Stencil2d::VonNeumann => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
            Stencil2d::Moore => vec![(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)],
            Stencil2d::Hex => vec![(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)],
            Stencil2d::Custom(v) => v.clone(),
        }
    }

    async fn validate(&self) -> Result<(), TopologyError> {
        let offsets = self.offsets();
        if offsets.is_empty() {
            return Err(TopologyError::InvalidStencil { reason: "stencil has zero offsets" });
        }
        for (i, &a) in offsets.iter().enumerate() {
            if a == (0, 0) {
                return Err(TopologyError::InvalidStencil { reason: "self-offset (0,0) is not supported" });
            }
            for &b in &offsets[i + 1..] {
                if a == b {
                    return Err(TopologyError::InvalidStencil { reason: "duplicate offset" });
                }
            }
            if !offsets.contains(&(-a.0, -a.1)) {
                return Err(TopologyError::InvalidStencil { reason: "offset's negation is not present in the stencil" });
            }
        }
        Ok(())
    }
}

/// 🗺️ Registers one directed relation per stencil offset (paired with its negation as inverse)
/// and returns them in `stencil.offsets()` order, ready to pass to [`Grid2dTopology::new`].
pub async fn declare_stencil_relations(builder: &mut ModelBuilder, stencil: &Stencil2d) -> Result<Vec<RelationId>, ModelError> {
    stencil.validate().map_err(|_| ModelError::InvalidSymmetryGroup { reason: "invalid stencil passed to declare_stencil_relations" })?;
    let offsets = stencil.offsets();
    let mut relations = Vec::with_capacity(offsets.len());
    for &(dx, dy) in &offsets {
        relations.push(builder.add_relation(&format!("offset({dx},{dy})")));
    }
    for (i, &(dx, dy)) in offsets.iter().enumerate() {
        if let Some(j) = offsets.iter().position(|&o| o == (-dx, -dy)) {
            builder.set_relation_inverse(relations[i], relations[j]);
        }
    }
    Ok(relations)
}

/// 🗺️ [`declare_stencil_relations`] for a [`TiledModelBuilder`].
pub async fn declare_stencil_relations_tiled(builder: &mut TiledModelBuilder, stencil: &Stencil2d) -> Result<Vec<RelationId>, ModelError> {
    stencil.validate().map_err(|_| ModelError::InvalidSymmetryGroup { reason: "invalid stencil passed to declare_stencil_relations_tiled" })?;
    let offsets = stencil.offsets();
    let mut relations = Vec::with_capacity(offsets.len());
    for &(dx, dy) in &offsets {
        relations.push(builder.relation(&format!("offset({dx},{dy})")));
    }
    for (i, &(dx, dy)) in offsets.iter().enumerate() {
        if let Some(j) = offsets.iter().position(|&o| o == (-dx, -dy)) {
            builder.set_relation_inverse(relations[i], relations[j]);
        }
    }
    Ok(relations)
}
// #endregion 🔖️Stencil

// #region 🔖️Boundary
/// 🗺️ Per-axis behavior when a stencil offset points outside `0..size`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Boundary {
    /// 🗺️ No arc is drawn — the edge cell simply has fewer neighbors.
    Open,
    /// 🗺️ No arc is drawn, but the edge cell's initial domain is restricted as if a permanently-
    /// resolved neighbor of the given pattern were there (an init-time unary restriction, not a
    /// propagation-participating virtual node).
    FixedOutside(PatternId),
    /// 🗺️ The axis is periodic: `size - 1` wraps to `0` and vice versa.
    Wrap,
    /// 🗺️ Out-of-range coordinates reflect back into range (`-1` mirrors to `1`, matching the
    /// common "symmetric" image-padding convention). `for_each_in_arc`'s per-offset reverse lookup
    /// can under-count sources at a mirrored boundary on very small grids (it recovers the
    /// *nearest* predecessor per offset, not every predecessor a mirror fold may have created) —
    /// the AC-4 engine's support counts can therefore under-count support (sound but incomplete
    /// pruning) on `Mirror` boundaries; `Open`/`Wrap`/`FixedOutside` are unaffected. Avoid `Mirror`
    /// with AC-4 on grids smaller than roughly `2 * max stencil radius` until this is revisited.
    Mirror,
}

pub(crate) async fn resolve_coord(coord: i32, size: usize, boundary: Boundary) -> Option<usize> {
    if coord >= 0 && (coord as usize) < size {
        return Some(coord as usize);
    }
    match boundary {
        Boundary::Open | Boundary::FixedOutside(_) => None,
        Boundary::Wrap => {
            let n = size as i32;
            Some((((coord % n) + n) % n) as usize)
        }
        Boundary::Mirror => {
            if size <= 1 {
                return Some(0);
            }
            let period = 2 * (size as i32 - 1);
            let mut m = coord % period;
            if m < 0 {
                m += period;
            }
            if m >= size as i32 {
                m = period - m;
            }
            Some(m as usize)
        }
    }
}
// #endregion 🔖️Boundary

// #region 🔖️Topology
/// 🗺️ A dense, row-major 2D grid topology. `NodeId(y * width + x)`.
#[derive(Clone, Debug)]
pub struct Grid2dTopology {
    width: usize,
    height: usize,
    offsets: Vec<(i32, i32)>,
    relations: Vec<RelationId>,
    boundary_x: Boundary,
    boundary_y: Boundary,
    mask: Option<Vec<bool>>,
}

impl Grid2dTopology {
    #[allow(clippy::too_many_arguments)]
    pub async fn new(width: usize, height: usize, stencil: &Stencil2d, relations: Vec<RelationId>, boundary_x: Boundary, boundary_y: Boundary, mask: Option<Vec<bool>>) -> Result<Self, TopologyError> {
        if width == 0 {
            return Err(TopologyError::ZeroDimension { axis: "width" });
        }
        if height == 0 {
            return Err(TopologyError::ZeroDimension { axis: "height" });
        }
        width.checked_mul(height).ok_or(TopologyError::SizeOverflow)?;
        stencil.validate()?;
        let offsets = stencil.offsets();
        if offsets.len() != relations.len() {
            return Err(TopologyError::InvalidStencil { reason: "relations length does not match stencil offset count" });
        }
        if let Some(m) = &mask {
            if m.len() != width * height {
                return Err(TopologyError::MaskShapeMismatch { expected: width * height, actual: m.len() });
            }
        }
        Ok(Self { width, height, offsets, relations, boundary_x, boundary_y, mask })
    }

    #[inline]
    pub async fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub async fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub async fn node_at(&self, x: usize, y: usize) -> Option<NodeId> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(NodeId::from_index(y * self.width + x))
    }

    #[inline]
    pub async fn coords(&self, n: NodeId) -> (usize, usize) {
        let idx = n.index();
        (idx % self.width, idx / self.width)
    }

    #[inline]
    pub async fn is_active(&self, x: usize, y: usize) -> bool {
        self.mask.as_ref().is_none_or(|m| m[y * self.width + x])
    }

    /// 🗺️ Cells masked out (inactive) — these must be pinned to a placeholder pattern by the
    /// solver builder so they never participate in the search (see [`crate::wfc_engine::solver_grid2d`]).
    pub async fn inactive_cells(&self) -> Vec<NodeId> {
        let Some(mask) = &self.mask else { return Vec::new() };
        (0..mask.len()).filter(|&i| !mask[i]).map(NodeId::from_index).collect()
    }

    /// 🗺️ Every `(node, relation, outside_pattern)` an edge cell must be restricted by at init
    /// time, derived from [`Boundary::FixedOutside`] axes.
    pub async fn fixed_outside_restrictions(&self) -> Vec<(NodeId, RelationId, PatternId)> {
        let mut out = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if !self.is_active(x, y) {
                    continue;
                }
                for (i, &(dx, dy)) in self.offsets.iter().enumerate() {
                    let tx = x as i32 + dx;
                    let ty = y as i32 + dy;
                    let x_out = tx < 0 || tx as usize >= self.width;
                    let y_out = ty < 0 || ty as usize >= self.height;
                    if x_out {
                        if let Boundary::FixedOutside(p) = self.boundary_x {
                            out.push((NodeId::from_index(y * self.width + x), self.relations[i], p));
                            continue;
                        }
                    }
                    if y_out {
                        if let Boundary::FixedOutside(p) = self.boundary_y {
                            out.push((NodeId::from_index(y * self.width + x), self.relations[i], p));
                        }
                    }
                }
            }
        }
        out
    }
}

impl Topology for Grid2dTopology {
    #[inline]
    async fn node_count(&self) -> usize {
        self.width * self.height
    }

    async fn arc_count(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if !self.is_active(x, y) {
                    continue;
                }
                for &(dx, dy) in &self.offsets {
                    let rx = resolve_coord(x as i32 + dx, self.width, self.boundary_x);
                    let ry = resolve_coord(y as i32 + dy, self.height, self.boundary_y);
                    if let (Some(nx), Some(ny)) = (rx, ry) {
                        if self.is_active(nx, ny) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    #[inline]
    async fn region_of(&self, _n: NodeId) -> RegionId {
        RegionId(0)
    }

    async fn for_each_out_arc(&self, n: NodeId, mut f: impl FnMut(NodeId, RelationId)) {
        let (x, y) = self.coords(n);
        if !self.is_active(x, y) {
            return;
        }
        for (i, &(dx, dy)) in self.offsets.iter().enumerate() {
            let rx = resolve_coord(x as i32 + dx, self.width, self.boundary_x);
            let ry = resolve_coord(y as i32 + dy, self.height, self.boundary_y);
            if let (Some(nx), Some(ny)) = (rx, ry) {
                if self.is_active(nx, ny) {
                    f(NodeId::from_index(ny * self.width + nx), self.relations[i]);
                }
            }
        }
    }

    async fn for_each_in_arc(&self, n: NodeId, mut f: impl FnMut(NodeId, RelationId, usize)) {
        let (x, y) = self.coords(n);
        if !self.is_active(x, y) {
            return;
        }
        for (i, &(dx, dy)) in self.offsets.iter().enumerate() {
            let rx = resolve_coord(x as i32 - dx, self.width, self.boundary_x);
            let ry = resolve_coord(y as i32 - dy, self.height, self.boundary_y);
            if let (Some(sx), Some(sy)) = (rx, ry) {
                if self.is_active(sx, sy) {
                    // Slot = target's node index * offset count + offset index; a fixed dense id
                    // per (target, offset) pair regardless of which candidate source resolved it.
                    f(NodeId::from_index(sy * self.width + sx), self.relations[i], n.index() * self.offsets.len() + i);
                }
            }
        }
    }

    async fn max_in_degree(&self) -> usize {
        self.offsets.len()
    }
}
// #endregion 🔖️Topology

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn von_neumann_offsets_are_symmetric() {
        Stencil2d::VonNeumann.validate().unwrap();
        Stencil2d::Moore.validate().unwrap();
        Stencil2d::Hex.validate().unwrap();
    }

    #[semio_framework_async_macros::async_test]
    async fn custom_stencil_rejects_unpaired_offset() {
        let s = Stencil2d::Custom(vec![(1, 0)]);
        assert!(s.validate().is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn custom_stencil_rejects_duplicate_and_self_offset() {
        assert!(Stencil2d::Custom(vec![(1, 0), (1, 0), (-1, 0)]).validate().is_err());
        assert!(Stencil2d::Custom(vec![(0, 0)]).validate().is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn node_at_and_coords_roundtrip() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let topo = Grid2dTopology::new(4, 3, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, None).unwrap();
        let n = topo.node_at(2, 1).unwrap();
        assert_eq!(topo.coords(n), (2, 1));
        assert_eq!(topo.node_at(4, 0), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn open_boundary_drops_out_of_range_arcs() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let topo = Grid2dTopology::new(3, 3, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, None).unwrap();
        let corner = topo.node_at(0, 0).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(corner, |m, _| out.push(m));
        assert_eq!(out.len(), 2); // only east and south exist from the top-left corner
    }

    #[semio_framework_async_macros::async_test]
    async fn wrap_boundary_connects_opposite_edges() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let topo = Grid2dTopology::new(3, 3, &Stencil2d::VonNeumann, rels, Boundary::Wrap, Boundary::Wrap, None).unwrap();
        let corner = topo.node_at(0, 0).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(corner, |m, _| out.push(m));
        assert_eq!(out.len(), 4); // wraps to all four neighbors
        assert!(out.contains(&topo.node_at(2, 0).unwrap())); // west wraps to x=2
        assert!(out.contains(&topo.node_at(0, 2).unwrap())); // north wraps to y=2
    }

    #[semio_framework_async_macros::async_test]
    async fn size_one_axis_wrap_self_loops() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let topo = Grid2dTopology::new(1, 3, &Stencil2d::VonNeumann, rels, Boundary::Wrap, Boundary::Open, None).unwrap();
        let n = topo.node_at(0, 1).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(n, |m, _| out.push(m));
        // east/west both wrap to the same single column -> self-loop arcs, plus north/south.
        assert_eq!(out.len(), 4);
    }

    #[semio_framework_async_macros::async_test]
    async fn mirror_boundary_reflects_at_edges() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let topo = Grid2dTopology::new(5, 5, &Stencil2d::VonNeumann, rels, Boundary::Mirror, Boundary::Mirror, None).unwrap();
        let corner = topo.node_at(0, 0).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(corner, |m, _| out.push(m));
        assert_eq!(out.len(), 4);
        assert!(out.contains(&topo.node_at(1, 0).unwrap())); // west mirrors back to x=1
        assert!(out.contains(&topo.node_at(0, 1).unwrap())); // north mirrors back to y=1
    }

    #[semio_framework_async_macros::async_test]
    async fn mask_excludes_inactive_cells_from_arcs() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let mut mask = vec![true; 9];
        mask[4] = false; // center of 3x3 inactive
        let topo = Grid2dTopology::new(3, 3, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, Some(mask)).unwrap();
        let north_of_center = topo.node_at(1, 0).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(north_of_center, |m, _| out.push(m));
        assert!(!out.contains(&topo.node_at(1, 1).unwrap())); // no arc into the masked-out center
        assert_eq!(topo.inactive_cells(), vec![NodeId(4)]);
    }

    #[semio_framework_async_macros::async_test]
    async fn fixed_outside_restrictions_only_on_boundary_facing_axis() {
        let mut b = ModelBuilder::new();
        let solid = b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let topo = Grid2dTopology::new(2, 2, &Stencil2d::VonNeumann, rels, Boundary::FixedOutside(solid), Boundary::Open, None).unwrap();
        let restrictions = topo.fixed_outside_restrictions();
        // Only x-axis boundary is FixedOutside; every cell touches an x-edge in a 2x2 grid.
        assert_eq!(restrictions.len(), 4);
        assert!(restrictions.iter().all(|&(_, _, p)| p == solid));
    }

    #[semio_framework_async_macros::async_test]
    async fn in_arc_matches_out_arc_on_open_boundary() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::Moore).unwrap();
        let topo = Grid2dTopology::new(4, 4, &Stencil2d::Moore, rels, Boundary::Open, Boundary::Open, None).unwrap();
        for y in 0..4 {
            for x in 0..4 {
                let n = topo.node_at(x, y).unwrap();
                let mut outgoing = Vec::new();
                topo.for_each_out_arc(n, |m, r| outgoing.push((m, r)));
                let mut incoming_as_source: Vec<(NodeId, RelationId)> = Vec::new();
                for oy in 0..4 {
                    for ox in 0..4 {
                        let other = topo.node_at(ox, oy).unwrap();
                        let mut theirs = Vec::new();
                        topo.for_each_in_arc(other, |src, r, _slot| theirs.push((src, r)));
                        if theirs.contains(&(n, RelationId(0))) || theirs.iter().any(|&(s, _)| s == n) {
                            for &(s, r) in &theirs {
                                if s == n {
                                    incoming_as_source.push((other, r));
                                }
                            }
                        }
                    }
                }
                let mut a = outgoing;
                let mut b2 = incoming_as_source;
                a.sort_by_key(|&(m, r)| (m.get(), r.get()));
                b2.sort_by_key(|&(m, r)| (m.get(), r.get()));
                assert_eq!(a, b2, "node ({x},{y}) out-arcs must match in-arc reconstruction on an open boundary");
            }
        }
    }
}
// #endregion 🔖️Tests
