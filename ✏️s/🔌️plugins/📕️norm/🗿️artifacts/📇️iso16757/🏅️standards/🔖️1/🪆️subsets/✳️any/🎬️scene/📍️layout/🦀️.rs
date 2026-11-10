//! 📍️ Layout: where every geometry object stands. Persisted placements win; otherwise the documented default rule applies (golden-tested): objects run along world `+X` (their depth) in id order with a 0.5 m gap, their footprint starting at the origin.

use super::kit::v3;
use super::model::part_2::solid::{self, Primitive};
use super::model::part_2::{BoundingBox, GeometryNode, GeometryObject};
use super::model::Iso16757Snapshot;
use super::scene::Vec3;
use std::collections::BTreeMap;

pub use super::scene::projection_kit::Placement;

pub const OBJECT_GAP: f64 = 0.5;

/// 📍️ One resolved object pose: the world position of the object's local origin. Object space is the schema convention (x width, y height, z depth); the world is Z-up, so the cyclic map `(x, y, z) -> (z, x, y)` stands depth along `+X`, width along `+Y` and height along `+Z` (a rotation, handedness kept).
#[derive(Clone, Debug, PartialEq)]
pub struct Pose {
    pub origin: Vec3,
}

impl Pose {
    pub fn at(&self, x: f64, y: f64, z: f64) -> Vec3 {
        v3(self.origin.x + z, self.origin.y + x, self.origin.z + y)
    }

    /// 🧭️ The world direction of the object axis `axis` (0 width, 1 height, 2 depth).
    pub fn dir(axis: usize) -> Vec3 {
        [Vec3::Y, Vec3::Z, Vec3::X][axis]
    }
}

/// 🧱️ One drawn primitive in the world frame of its object: the shared definition of [`model::part_2::solid`] with rotations and translations applied.
pub type Leaf = Primitive;

/// 🌳️ The primitives of `node`, references resolved; a cyclic or incomplete tree draws nothing.
pub fn leaves(document: &Iso16757Snapshot, node: &GeometryNode) -> Vec<Leaf> {
    solid::placed_primitives(node, &document.geometry).unwrap_or_default()
}

/// 📦️ The exact axis-aligned extent `(min, max)` of one leaf, rotation included.
pub fn leaf_extent(leaf: &Leaf) -> ([f64; 3], [f64; 3]) {
    let b = leaf.bounds();
    (b.min, b.max)
}

/// 📦️ The extent of an object: its shape leaves together with its space envelopes.
pub fn extent(document: &Iso16757Snapshot, object: &GeometryObject) -> BoundingBox {
    let found = object.shape.as_ref().map(|shape| leaves(document, shape)).unwrap_or_default();
    let mut boxes: Vec<([f64; 3], [f64; 3])> = found.iter().map(leaf_extent).collect();
    boxes.extend(object.spaces.iter().map(|s| (s.bounds.min, s.bounds.max)));
    boxes.extend(object.surfaces.iter().map(|s| (s.bounds.min, s.bounds.max)));
    let mut min = [f64::MAX; 3];
    let mut max = [f64::MIN; 3];
    for (lo, hi) in &boxes {
        for axis in 0..3 {
            min[axis] = min[axis].min(lo[axis]);
            max[axis] = max[axis].max(hi[axis]);
        }
    }
    if boxes.is_empty() {
        return BoundingBox { min: [0.0; 3], max: [0.1; 3] };
    }
    BoundingBox { min, max }
}

/// 📍️ All resolved object poses of one projection.
#[derive(Clone, Debug, Default)]
pub struct Layout {
    objects: BTreeMap<String, Pose>,
}

impl Layout {
    pub fn of(document: &Iso16757Snapshot, placements: &[Placement]) -> Self {
        let mut layout = Self::default();
        let mut cursor = 0.0;
        for (id, object) in &document.geometry.objects {
            let bounds = extent(document, object);
            let pose = match placements.iter().find(|p| &p.entity_id == id) {
                Some(p) => Pose { origin: p.origin.clone() },
                None => Pose { origin: v3(cursor - bounds.min[2], -bounds.min[0], -bounds.min[1].min(0.0)) },
            };
            cursor += (bounds.max[2] - bounds.min[2]).max(0.1) + OBJECT_GAP;
            layout.objects.insert(id.clone(), pose);
        }
        layout
    }

    pub fn pose(&self, object: &str) -> Option<&Pose> {
        self.objects.get(object)
    }
}
