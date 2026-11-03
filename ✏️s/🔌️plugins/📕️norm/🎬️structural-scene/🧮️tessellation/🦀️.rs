//! 🧮️ Tessellation: pure, deterministic, LOD-bounded meshes (positions, normals, indices) for every scene geometry; no framework types.

#[path = "📐️vector/🦀️.rs"]
pub mod vector;
#[path = "🔺️triangulate/🦀️.rs"]
pub mod triangulate;
#[path = "🧱️section-outline/🦀️.rs"]
pub mod section_outline;
#[path = "📏️extrude/🦀️.rs"]
pub mod extrude;
#[path = "🔩️fasteners/🦀️.rs"]
pub mod fasteners;
#[path = "🪨️solids/🦀️.rs"]
pub mod solids;
#[path = "🏹️glyphs/🦀️.rs"]
pub mod glyphs;
#[path = "🌐️shells/🦀️.rs"]
pub mod shells;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;

use crate::{canonical_json, Aabb, Geometry, Vec3};
use std::collections::HashMap;
use vector::*;

/// 🪜️ Level of detail ladder: box impostor, coarse, moderate, filleted; each level caps the segments of a full turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum Lod {
    Lod0,
    Lod1,
    Lod2,
    Lod3,
}

/// 📏️ Maximum chord deviation (sagitta, metres) of a curve from its tessellation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tolerance {
    pub chord: f64,
}

/// 🔑️ Stable cache key of a mesh source; append the level with [`MeshKey::at`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeshKey(pub String);

/// 🧊️ Indexed triangle mesh: counter-clockwise outward winding, unit normals, optional per-vertex scalar (relative position along a member).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct MeshData {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub scalar_slot: Option<Vec<f32>>,
}

/// 🧵️ Anything that can be turned into a mesh at a tolerance and level; invalid input yields an empty mesh, use the module functions for diagnostics.
pub trait Tessellate {
    fn mesh(&self, tol: Tolerance, lod: Lod) -> MeshData;
    fn key(&self) -> MeshKey;
}

/// 🧱️ Triangle-soup accumulator that welds and smooths below a crease angle when built.
#[derive(Clone, Debug, Default)]
pub struct MeshBuilder {
    tris: Vec<[P; 3]>,
    params: Vec<[f32; 3]>,
    scalar: bool,
}

impl Default for Tolerance {
    fn default() -> Self {
        Self { chord: 5e-4 }
    }
}

impl Tolerance {
    pub fn new(chord: f64) -> Self {
        Self { chord }
    }
}

impl MeshKey {
    pub fn at(&self, lod: Lod) -> Self {
        Self(format!("{}@{:?}", self.0, lod))
    }

    pub fn of(kind: &str, canonical: &str) -> Self {
        let hash = canonical.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3));
        Self(format!("{kind}:{hash:016x}"))
    }
}

impl Lod {
    pub fn turn_segments(self) -> (usize, usize) {
        match self {
            Self::Lod0 => (4, 4),
            Self::Lod1 => (8, 12),
            Self::Lod2 => (16, 32),
            Self::Lod3 => (32, 96),
        }
    }

    pub fn max_instances(self) -> usize {
        match self {
            Self::Lod0 => 1,
            Self::Lod1 => 16,
            Self::Lod2 => 128,
            Self::Lod3 => 1024,
        }
    }
}

/// 🎚️ Segments for an arc of `sweep` radians at `radius`: chord tolerance bounded by the level's per-turn range, at least one.
pub fn segments(radius: f64, sweep: f64, tol: Tolerance, lod: Lod) -> usize {
    let (lo, hi) = lod.turn_segments();
    let step = 2.0 * (1.0 - tol.chord / radius.max(1e-12)).clamp(-1.0, 1.0).acos();
    let turn = if step > 0.0 { (std::f64::consts::TAU / step).ceil() as usize } else { hi };
    let full = turn.clamp(lo, hi);
    ((full as f64 * sweep.abs() / std::f64::consts::TAU).ceil() as usize).max(1)
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_scalar() -> Self {
        Self { scalar: true, ..Self::default() }
    }

    pub fn is_empty(&self) -> bool {
        self.tris.is_empty()
    }

    pub fn tri_s(&mut self, a: P, b: P, c: P, s: [f32; 3]) {
        if len(cross(sub(b, a), sub(c, a))) > 1e-18 {
            self.tris.push([a, b, c]);
            self.params.push(s);
        }
    }

    pub fn tri(&mut self, a: P, b: P, c: P) {
        self.tri_s(a, b, c, [0.0; 3]);
    }

    pub fn tri_toward(&mut self, a: P, b: P, c: P, hint: P) {
        self.tri_toward_s(a, b, c, hint, [0.0; 3]);
    }

    pub fn tri_toward_s(&mut self, a: P, b: P, c: P, hint: P, s: [f32; 3]) {
        if dot(cross(sub(b, a), sub(c, a)), hint) >= 0.0 {
            self.tri_s(a, b, c, s);
        } else {
            self.tri_s(a, c, b, [s[0], s[2], s[1]]);
        }
    }

    pub fn quad_toward(&mut self, q: [P; 4], hint: P) {
        self.quad_toward_s(q, hint, [0.0; 4]);
    }

    pub fn quad_toward_s(&mut self, q: [P; 4], hint: P, s: [f32; 4]) {
        self.tri_toward_s(q[0], q[1], q[2], hint, [s[0], s[1], s[2]]);
        self.tri_toward_s(q[0], q[2], q[3], hint, [s[0], s[2], s[3]]);
    }

    pub fn fan_toward(&mut self, ring: &[P], hint: P) {
        (1..ring.len().saturating_sub(1)).for_each(|i| self.tri_toward(ring[0], ring[i], ring[i + 1], hint));
    }

    /// 🧊️ Welds identical positions, shades smooth below `crease_deg` degrees and splits above; vertices equal in position, normal and scalar are shared.
    pub fn build(self, crease_deg: f64) -> MeshData {
        let cos = crease_deg.to_radians().cos();
        let cr: Vec<P> = self.tris.iter().map(|t| cross(sub(t[1], t[0]), sub(t[2], t[0]))).collect();
        let nn: Vec<P> = cr.iter().map(|c| unit(*c)).collect();
        let mut at: HashMap<[i64; 3], Vec<usize>> = HashMap::new();
        for (i, t) in self.tris.iter().enumerate() {
            for c in t {
                at.entry(quantise(*c)).or_default().push(i);
            }
        }
        let mut seen: HashMap<([i64; 3], [i64; 3], u32), u32> = HashMap::new();
        let mut mesh = MeshData { scalar_slot: self.scalar.then(Vec::new), ..MeshData::default() };
        for (i, t) in self.tris.iter().enumerate() {
            for (k, c) in t.iter().enumerate() {
                let sum = at[&quantise(*c)].iter().filter(|&&j| dot(nn[i], nn[j]) >= cos).fold([0.0; 3], |acc, &j| add(acc, cr[j]));
                let n = unit(sum);
                let s = self.params[i][k];
                let key = (quantise(*c), [(n[0] * 1e4).round() as i64, (n[1] * 1e4).round() as i64, (n[2] * 1e4).round() as i64], s.to_bits());
                let index = *seen.entry(key).or_insert_with(|| {
                    mesh.positions.push([c[0] as f32, c[1] as f32, c[2] as f32]);
                    mesh.normals.push([n[0] as f32, n[1] as f32, n[2] as f32]);
                    if let Some(slot) = &mut mesh.scalar_slot {
                        slot.push(s);
                    }
                    (mesh.positions.len() - 1) as u32
                });
                mesh.indices.push(index);
            }
        }
        mesh
    }
}

impl MeshData {
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    fn corner(&self, i: u32) -> P {
        let c = self.positions[i as usize];
        [f64::from(c[0]), f64::from(c[1]), f64::from(c[2])]
    }

    pub fn aabb(&self) -> Aabb {
        let pts: Vec<Vec3> = self.positions.iter().map(|c| Vec3::new(f64::from(c[0]), f64::from(c[1]), f64::from(c[2]))).collect();
        Aabb::of_points(&pts).unwrap_or(Aabb::new(Vec3::ZERO, Vec3::ZERO))
    }

    /// 📦️ Enclosed volume by the divergence theorem; positive for outward winding.
    pub fn volume(&self) -> f64 {
        self.indices.chunks(3).map(|t| dot(self.corner(t[0]), cross(self.corner(t[1]), self.corner(t[2]))) / 6.0).sum()
    }

    pub fn surface_area(&self) -> f64 {
        self.indices.chunks(3).map(|t| len(cross(sub(self.corner(t[1]), self.corner(t[0])), sub(self.corner(t[2]), self.corner(t[0])))) / 2.0).sum()
    }

    /// 💧️ Every directed edge between welded positions has exactly one opposite twin.
    pub fn is_watertight(&self) -> bool {
        let key = |i: u32| {
            let c = self.corner(i);
            [(c[0] * 1e5).round() as i64, (c[1] * 1e5).round() as i64, (c[2] * 1e5).round() as i64]
        };
        let mut edges: HashMap<([i64; 3], [i64; 3]), i32> = HashMap::new();
        for t in self.indices.chunks(3) {
            for k in 0..3 {
                let (a, b) = (key(t[k]), key(t[(k + 1) % 3]));
                if a != b {
                    *edges.entry((a, b)).or_default() += 1;
                }
            }
        }
        edges.iter().all(|(&(a, b), &n)| n == 1 && edges.get(&(b, a)) == Some(&1))
    }

    pub fn merge(&mut self, other: &MeshData) {
        let base = self.positions.len() as u32;
        self.positions.extend_from_slice(&other.positions);
        self.normals.extend_from_slice(&other.normals);
        self.indices.extend(other.indices.iter().map(|i| i + base));
        if other.scalar_slot.is_some() && self.scalar_slot.is_none() {
            self.scalar_slot = Some(vec![0.0; base as usize]);
        }
        if let Some(slot) = &mut self.scalar_slot {
            match &other.scalar_slot {
                Some(b) => slot.extend_from_slice(b),
                None => slot.extend(std::iter::repeat_n(0.0, other.positions.len())),
            }
        }
    }

    pub fn merged(parts: impl IntoIterator<Item = MeshData>) -> Self {
        parts.into_iter().fold(Self::default(), |mut acc, m| {
            acc.merge(&m);
            acc
        })
    }

    /// 🧭️ Maps local `(x, y, z)` to `origin + x*ex + y*ey + z*ez`; the basis must be orthonormal and right-handed.
    pub fn placed(&self, origin: P, ex: P, ey: P, ez: P) -> Self {
        let map = |c: [f32; 3], w: f64| {
            let r = add(add(mul(ex, f64::from(c[0])), mul(ey, f64::from(c[1]))), mul(ez, f64::from(c[2])));
            let r = if w > 0.0 { add(r, origin) } else { r };
            [r[0] as f32, r[1] as f32, r[2] as f32]
        };
        Self {
            positions: self.positions.iter().map(|c| map(*c, 1.0)).collect(),
            normals: self.normals.iter().map(|c| map(*c, 0.0)).collect(),
            indices: self.indices.clone(),
            scalar_slot: self.scalar_slot.clone(),
        }
    }

    pub fn translated(&self, by: P) -> Self {
        self.placed(by, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
    }

    /// 📦️ Level-0 box impostor of this mesh.
    pub fn impostor(&self) -> Self {
        if self.is_empty() {
            return Self::default();
        }
        let b = self.aabb();
        let (lo, hi) = (p(&b.min), p(&b.max));
        let mut m = MeshBuilder::new();
        solids::cuboid(&mut m, lo, hi);
        m.build(30.0)
    }
}

/// 📦️ Bounds of a geometry from its coarse mesh; `None` for groups and invalid geometry.
pub fn geometry_bounds(geometry: &Geometry) -> Option<Aabb> {
    let m = geometry.mesh(Tolerance::default(), Lod::Lod1);
    (!m.is_empty()).then(|| m.aabb())
}

impl Tessellate for Geometry {
    fn mesh(&self, tol: Tolerance, lod: Lod) -> MeshData {
        if lod == Lod::Lod0 {
            return self.mesh(tol, Lod::Lod1).impostor();
        }
        match self {
            Geometry::Member(g) => extrude::member_mesh(g, tol, lod).unwrap_or_default(),
            Geometry::Plate(g) => shells::plate_mesh(g, tol, lod).unwrap_or_default(),
            Geometry::Shell(g) => shells::shell_mesh(g, tol, lod).unwrap_or_default(),
            Geometry::Solid(g) => solids::solid_mesh(&g.shape, tol, lod).unwrap_or_default(),
            Geometry::Fastener(g) => fasteners::fastener_mesh(g, tol, lod).unwrap_or_default(),
            Geometry::Support(g) => glyphs::support_mesh(g, tol, lod),
            Geometry::Load(g) => glyphs::load_mesh(g, tol, lod),
            Geometry::Surface(g) => shells::surface_mesh(g, tol, lod),
            Geometry::Group { .. } => MeshData::default(),
        }
    }

    fn key(&self) -> MeshKey {
        MeshKey::of(&format!("{:?}", self.kind()), &canonical_json(self))
    }
}
