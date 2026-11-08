//! 📍️ Layout: where every geometry object stands. Persisted placements win; otherwise the documented default rule applies (golden-tested): objects run along `+X` in id order with a 0.5 m gap, their footprint starting at the origin.

use super::kit::v3;
use super::model::part_2::{BoundingBox, GeometryNode, GeometryObject};
use super::model::Iso16757Snapshot;
use super::scene::Vec3;
use std::collections::BTreeMap;

pub use super::scene::projection_kit::Placement;

pub const OBJECT_GAP: f64 = 0.5;
pub const DEPTH_LIMIT: usize = 8;

/// 📍️ One resolved object pose: the world position of the object's local origin.
#[derive(Clone, Debug, PartialEq)]
pub struct Pose {
    pub origin: Vec3,
}

impl Pose {
    pub fn at(&self, x: f64, y: f64, z: f64) -> Vec3 {
        v3(self.origin.x + x, self.origin.y + y, self.origin.z + z)
    }
}

/// 🧱️ One drawn primitive of a flattened shape tree, in object coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct Leaf {
    pub kind: String,
    pub parameters: BTreeMap<String, f64>,
    pub offset: [f64; 3],
}

/// 🌳️ The primitives of `node` with translations accumulated and references resolved up to [`DEPTH_LIMIT`]; rotations are not drawn, the leaf stands axis-aligned.
pub fn leaves(document: &Iso16757Snapshot, node: &GeometryNode, offset: [f64; 3], depth: usize, out: &mut Vec<Leaf>) {
    if depth > DEPTH_LIMIT {
        return;
    }
    match node {
        GeometryNode::Primitive { kind, parameters } => out.push(Leaf { kind: kind.clone(), parameters: parameters.clone(), offset }),
        GeometryNode::Transform { translation, child, .. } => leaves(document, child, [offset[0] + translation[0], offset[1] + translation[1], offset[2] + translation[2]], depth + 1, out),
        GeometryNode::Boolean { children, .. } => children.iter().for_each(|c| leaves(document, c, offset, depth + 1, out)),
        GeometryNode::Reference { geometry_id } => {
            if let Some(shape) = document.geometry.objects.get(geometry_id).and_then(|o| o.shape.as_ref()) {
                leaves(document, shape, offset, depth + 1, out);
            }
        }
    }
}

/// 📦️ The axis-aligned extent `(min, max)` of one leaf: a box spans width × depth × height, a cylinder its radius about the axis, a sphere its radius about the offset.
pub fn leaf_extent(leaf: &Leaf) -> ([f64; 3], [f64; 3]) {
    let p = |name: &str| leaf.parameters.get(name).copied().unwrap_or(0.0).max(0.0);
    let o = leaf.offset;
    match leaf.kind.as_str() {
        "cylinder" => ([o[0] - p("radius"), o[1] - p("radius"), o[2]], [o[0] + p("radius"), o[1] + p("radius"), o[2] + p("height")]),
        "sphere" => ([o[0] - p("radius"), o[1] - p("radius"), o[2] - p("radius")], [o[0] + p("radius"), o[1] + p("radius"), o[2] + p("radius")]),
        _ => (o, [o[0] + p("width"), o[1] + p("depth"), o[2] + p("height")]),
    }
}

/// 📦️ The extent of an object: its shape leaves together with its space envelopes.
pub fn extent(document: &Iso16757Snapshot, object: &GeometryObject) -> BoundingBox {
    let mut found = Vec::new();
    if let Some(shape) = &object.shape {
        leaves(document, shape, [0.0; 3], 0, &mut found);
    }
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
                None => Pose { origin: v3(cursor - bounds.min[0], -bounds.min[1], -bounds.min[2].min(0.0)), },
            };
            cursor += (bounds.max[0] - bounds.min[0]).max(0.1) + OBJECT_GAP;
            layout.objects.insert(id.clone(), pose);
        }
        layout
    }

    pub fn pose(&self, object: &str) -> Option<&Pose> {
        self.objects.get(object)
    }
}
