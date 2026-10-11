//! 🧱️ `wall-layout`: the resolved vertical extent, plan geometry and quantities of every wall. A wall stores its axis, type, location
//! line and base/top constraints and never its height or footprint; the `WallLayout` node of the model graph resolves them from the
//! storey nodes it is resolved by (its own storey and the target of a `TopConstraint::Storey`) and from the `Band` nodes of the walls it
//! touches, so one storey height edit re-infers exactly the walls that depend on it and one moved wall exactly the walls within two touches.
//! This module is the pure part: bands, offsets, the layout of one wall over the bands of its neighbourhood.
//!
//! Conventions. Layers of a wall type are listed from the interior (left) face to the exterior (right) face, left and right being
//! taken along the axis direction. `LocationLine::Interior` names the interior (left) face as the axis (the body extends to the right),
//! `Exterior` the exterior (right) face (the body extends to the left), `Center` the mid plane and `CoreCenter` the centre of the `Core`
//! layers (else the `Structure` layers, else the mid plane). Faces are offset curves of the axis (parallel lines, or concentric arcs with
//! the same bulge); joins trim them as described in [`joins`]. Z values: see `storey-levels`.

pub use super::super::storey_levels::top_of;

use crate::standards::v1::subsets::any::schema::authored::plan::{axis_length, segment_of as seg};
use super::super::element_solids::plan_kit::mark;
use super::super::storey_levels::{vertical_of, StoreyLevel};
use crate::{Axis, LocationLine, ModelSnapshot, Point2, Vertex, Wall};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::loops;
use semio_framework_geometry::vector::angle_between;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

#[path = "🔗️joins/🦀️.rs"]
pub mod joins;
#[path = "🧲️attach/🦀️.rs"]
pub mod attach;

use joins::{Band, JOIN_TOLERANCE};

//#region 🔖️Values
/// 🔗️ Where on a wall a join sits: at its axis start, at its axis end, or along its interior.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub enum JoinEnd {
    Start,
    End,
    Along,
}

/// 🔗️ How two walls meet: `Miter` end to end (2 or more walls around a node), `Butt` this wall's end against the face of the other, `Through` the other wall's end against this wall, `Cross` both axes crossing in their interiors.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub enum JoinKind {
    Miter,
    Butt,
    Through,
    Cross,
}

/// 🔗️ One edge of the join graph, as seen from one wall: `end` is the site on this wall, `other_end` the site on `other`. `overlap_area` is the area both bodies cover at a `Cross` (otherwise 0).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct WallJoin {
    pub kind: JoinKind,
    pub end: JoinEnd,
    pub other: String,
    pub other_end: JoinEnd,
    pub point: Point2,
    pub overlap_area: f64,
}

/// 〰️ A join-trimmed face: a line (`bulge == 0`) or an arc from `start` to `end`, `bulge = tan(sweep / 4)`.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct FaceCurve {
    pub start: Point2,
    pub end: Point2,
    pub bulge: f64,
}

impl Default for FaceCurve {
    fn default() -> Self {
        Self { start: Point2 { x: 0.0, y: 0.0 }, end: Point2 { x: 0.0, y: 0.0 }, bulge: 0.0 }
    }
}

/// 🧱️ Resolved layout of one wall, in metres, square metres and cubic metres.
/// `length` is the centreline length; `offset_left`/`offset_right` the distances from the axis to the faces and `layer_offsets` the signed
/// distances (left positive) of the `layers + 1` layer interfaces from the left face to the right face. `left_face`/`right_face` are
/// join-trimmed; `footprint` is the counter-clockwise loop `[right.start, right.end, left.end, left.start]` (bulge of the edge to the
/// next vertex), empty when a face collapses. Side areas are face length times height before openings; `footprint_area` is the exact
/// loop area, `volume` the footprint area times height. `joins` lists every contact with a neighbour.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct WallLayout {
    pub base_z: f64,
    pub top_z: f64,
    pub height: f64,
    pub thickness: f64,
    pub length: f64,
    pub offset_left: f64,
    pub offset_right: f64,
    pub layer_offsets: Vec<f64>,
    pub left_face: FaceCurve,
    pub right_face: FaceCurve,
    pub left_length: f64,
    pub right_length: f64,
    pub left_area: f64,
    pub right_area: f64,
    pub side_area: f64,
    pub footprint: Vec<Vertex>,
    pub footprint_area: f64,
    pub volume: f64,
    pub joins: Vec<WallJoin>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub base_profile: Vec<attach::ElevationPoint>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub top_profile: Vec<attach::ElevationPoint>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub top_attach: Option<attach::AttachState>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_attach: Option<attach::AttachState>,
}

impl WallLayout {
    /// 📈️ The absolute height of the top of the wall at arc length `s` along its axis: the elevation edge for an attached top, the flat top otherwise.
    pub fn top_at(&self, s: f64) -> f64 {
        if self.top_profile.is_empty() {
            self.top_z
        } else {
            attach::elevation_at(&self.top_profile, s)
        }
    }

    /// 📈️ The absolute height of the base of the wall at arc length `s` along its axis: the elevation edge for an attached base, the flat base otherwise.
    pub fn base_at(&self, s: f64) -> f64 {
        if self.base_profile.is_empty() {
            self.base_z
        } else {
            attach::elevation_at(&self.base_profile, s)
        }
    }
}

/// ↔️ Distances from a wall axis to its faces: `layers` are the signed interface offsets (left positive), `layers + 1` of them.
#[derive(Clone, Debug, PartialEq)]
pub struct Offsets {
    pub left: f64,
    pub right: f64,
    pub layers: Vec<f64>,
}

/// ✂️ Where the four face ends of a trimmed wall fall on the axis, as arc-length coordinates (the axis start is 0, its end the length).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FaceEnds {
    pub left: (f64, f64),
    pub right: (f64, f64),
}

impl FaceEnds {
    /// ✂️ The extent over which the wall has its full thickness: from the later of the two face starts to the earlier of the two face ends.
    pub fn full_thickness(&self) -> (f64, f64) {
        (self.left.0.max(self.right.0), self.left.1.min(self.right.1))
    }
}
//#endregion 🔖️Values

//#region 🔖️Geometry
/// 🍰️ Total thickness of the layers of a wall type.
pub fn thickness_of(snapshot: &ModelSnapshot, wall: &Wall) -> f64 {
    snapshot.wall_types.get(&wall.wall_type).map_or(0.0, |kind| kind.layers.iter().map(|layer| layer.thickness).sum())
}

fn core_centre(layers: &[crate::Layer], thickness: f64) -> f64 {
    let span = |function: crate::LayerFunction| {
        let first = layers.iter().position(|layer| layer.function == function)?;
        let last = layers.iter().rposition(|layer| layer.function == function)?;
        Some((layers[..first].iter().map(|layer| layer.thickness).sum::<f64>(), layers[..=last].iter().map(|layer| layer.thickness).sum::<f64>()))
    };
    span(crate::LayerFunction::Core).or_else(|| span(crate::LayerFunction::Structure)).map_or(thickness / 2.0, |(start, end)| (start + end) / 2.0)
}

/// ↔️ The face distances of a wall from its location line and the layers of its type.
pub fn offsets_of(snapshot: &ModelSnapshot, wall: &Wall) -> Offsets {
    let layers: &[crate::Layer] = snapshot.wall_types.get(&wall.wall_type).map_or(&[][..], |kind| &kind.layers[..]);
    let thickness: f64 = layers.iter().map(|layer| layer.thickness).sum();
    let left = match wall.location {
        LocationLine::Center => thickness / 2.0,
        LocationLine::Interior => 0.0,
        LocationLine::Exterior => thickness,
        LocationLine::CoreCenter => core_centre(layers, thickness),
    };
    let mut cumulative = 0.0;
    let interfaces = std::iter::once(left).chain(layers.iter().map(|layer| {
        cumulative += layer.thickness;
        left - cumulative
    }));
    Offsets { left, right: thickness - left, layers: interfaces.collect() }
}

/// 🧱️ The plan band of one wall: its axis and the distances to its faces; `None` when the axis has no length (such a wall joins nothing).
pub fn band_of(snapshot: &ModelSnapshot, wall: &Wall) -> Option<Band> {
    let axis = seg(&wall.axis);
    let offsets = offsets_of(snapshot, wall);
    (axis.length() > JOIN_TOLERANCE).then(|| Band { axis, left: offsets.left, right: offsets.right, start_join: wall.start_join, end_join: wall.end_join })
}

/// 🧱️ The plan bands of the walls of one storey, keyed by wall id: the whole-storey reference the neighbourhood-based layouts are tested against.
#[cfg(test)]
pub fn storey_bands(snapshot: &ModelSnapshot, storey: &str) -> BTreeMap<String, Band> {
    snapshot.walls.iter().filter(|(_, wall)| wall.storey == storey).filter_map(|(id, wall)| band_of(snapshot, wall).map(|band| (id.clone(), band))).collect()
}

fn face_curve(face: &BulgeSeg) -> FaceCurve {
    FaceCurve { start: mark(face.start), end: mark(face.end), bulge: face.bulge }
}

/// 📍️ The fraction `0..=1` of `point` along a curve: the angle fraction for an arc, the line parameter otherwise.
pub fn fraction(curve: &BulgeSeg, point: Point) -> f64 {
    match curve.center() {
        Some(centre) => angle_between(curve.start - centre, point - centre) / curve.sweep(),
        None => curve.param_of(point),
    }
}

/// ✂️ The arc-length coordinates, along the axis, of the ends of the two trimmed faces of a layout; `None` when the layout has no footprint (a face collapsed or the axis has no length).
pub fn face_ends(layout: &WallLayout, axis: &Axis) -> Option<FaceEnds> {
    if layout.footprint.is_empty() {
        return None;
    }
    let axis = seg(axis);
    let (left, right) = (axis.offset(layout.offset_left)?, axis.offset(-layout.offset_right)?);
    let at = |curve: &BulgeSeg, face: Point2| fraction(curve, Point::new(face.x, face.y)) * layout.length;
    Some(FaceEnds { left: (at(&left, layout.left_face.start), at(&left, layout.left_face.end)), right: (at(&right, layout.right_face.start), at(&right, layout.right_face.end)) })
}

/// 🧮️ The layout of wall `id` from the levels of the storeys it is resolved by and the `bands` of its neighbourhood (its own band and the bands of the walls within two touches; the join of `id` over them equals its join over the whole storey).
pub fn layout_of(snapshot: &ModelSnapshot, id: &str, wall: &Wall, own: &StoreyLevel, target: Option<&StoreyLevel>, bands: &BTreeMap<String, Band>) -> WallLayout {
    let (base_z, top_z) = vertical_of(wall.base_offset, &wall.top, own, target);
    let height = top_z - base_z;
    let (offsets, length) = (offsets_of(snapshot, wall), axis_length(&wall.axis));
    let joined = joins::join(bands, id);
    let placed = joined.as_ref().and_then(|joined| joins::footprint(&bands[id], joined.start, joined.end));
    let mut layout = WallLayout {
        base_z,
        top_z,
        height,
        thickness: thickness_of(snapshot, wall),
        length,
        offset_left: offsets.left,
        offset_right: offsets.right,
        layer_offsets: offsets.layers,
        side_area: length * height,
        joins: joined.map(|joined| joined.joins).unwrap_or_default(),
        ..WallLayout::default()
    };
    if let Some(placed) = placed {
        let outline: Vec<loops::Vertex> = placed.outline.iter().map(|(point, bulge)| loops::Vertex { point: *point, bulge: *bulge }).collect();
        layout.left_face = face_curve(&placed.left);
        layout.right_face = face_curve(&placed.right);
        layout.left_length = placed.left.length();
        layout.right_length = placed.right.length();
        layout.left_area = layout.left_length * height;
        layout.right_area = layout.right_length * height;
        layout.footprint = placed.outline.iter().map(|(point, bulge)| Vertex { point: mark(*point), bulge: *bulge }).collect();
        layout.footprint_area = loops::area(&outline);
        layout.volume = layout.footprint_area * height;
    }
    layout
}

/// 🔑️ What `layout_of` reads of a wall besides the bands: the wall record and the layers of its type.
pub fn dependency(snapshot: &ModelSnapshot, wall: &Wall) -> DslValue {
    DslValue::object([
        ("wall".to_string(), semio_framework_value::ToValue::to_value(wall)),
        ("layers".to_string(), semio_framework_value::ToValue::to_value(&snapshot.wall_types.get(&wall.wall_type).map(|kind| kind.layers.clone()))),
    ])
}

/// 🔑️ What a `Band` reads of a wall: its axis, its location line, the join preference of each end and the layers of its type.
pub fn band_dependency(snapshot: &ModelSnapshot, wall: &Wall) -> DslValue {
    DslValue::object([
        ("axis".to_string(), semio_framework_value::ToValue::to_value(&wall.axis)),
        ("location".to_string(), semio_framework_value::ToValue::to_value(&wall.location)),
        ("start_join".to_string(), semio_framework_value::ToValue::to_value(&wall.start_join)),
        ("end_join".to_string(), semio_framework_value::ToValue::to_value(&wall.end_join)),
        ("layers".to_string(), semio_framework_value::ToValue::to_value(&snapshot.wall_types.get(&wall.wall_type).map(|kind| kind.layers.clone()))),
    ])
}
//#endregion 🔖️Geometry

//#region 🔖️Projection
/// 🧱️ The layout of every wall (the `WallLayout` nodes of the model graph).
#[cfg(test)]
pub fn compute_wall_layout(snapshot: &ModelSnapshot) -> BTreeMap<String, WallLayout> {
    std::mem::take(&mut super::super::model_graph::infer_selected::<{ super::super::model_graph::kinds::LAYOUTS }>(snapshot).wall_layout)
}
//#endregion 🔖️Projection

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
