//! 🧱️ Wall solids: one closed body per layer, mitered and butted at the joins of `wall-layout`, with every valid opening cut through all layers (reveals through the
//! thickness, door-like openings notch the bottom edge). Straight walls are prisms, arc walls are concentric slices tessellated within [`CHORD_TOLERANCE`].
//! The faces of the layers are offset curves of the axis at the interfaces of `WallLayout::layer_offsets`; the end `s` of an interface interpolates linearly between the
//! trimmed ends of the two outer faces, so a mitered join stays a straight cut across the whole build-up.
//!
//! The wall cuts **exactly the valid frames** of its hosted openings: `opening-frames` judges whether a hole fits strictly inside the full-thickness extent and below the top
//! (one rule, one tolerance), so there is no second predicate here that could drop a hole the frames call valid.

use super::super::super::opening_frames::OpeningFrame;
use super::super::super::wall_layout::{face_ends, fraction, WallLayout};
use super::super::plan_kit::{point, seg};
use super::super::{dep_object, dep_value, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::{ModelSnapshot, Wall};
use semio_framework_geometry::bulge::{nearest_intersection, BulgeSeg, Extent};
use semio_framework_geometry::mesh::{extrude_between_faces, ElevationFace};
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;

//#region 🔖️Cuts
/// ✂️ A rectangle removed from the development of a wall: `s` along the axis, `z` up from the wall base.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cut {
    pub s_min: f64,
    pub s_max: f64,
    pub z_min: f64,
    pub z_max: f64,
}

/// ✂️ The cuts of the valid frames among `frames`, ordered along the wall.
pub fn cuts_of<'a>(frames: impl IntoIterator<Item = &'a OpeningFrame>) -> Vec<Cut> {
    let mut cuts: Vec<Cut> = frames.into_iter().filter(|frame| frame.valid).map(|frame| Cut { s_min: frame.cut.s_min, s_max: frame.cut.s_max, z_min: frame.cut.z_min, z_max: frame.cut.z_max }).collect();
    cuts.sort_by(|a, b| a.s_min.total_cmp(&b.s_min));
    cuts
}
//#endregion 🔖️Cuts

//#region 🔖️Faces
const EPS: f64 = 1e-9;

fn face(s_start: f64, s_end: f64, height: f64, notches: &[Cut], holes: &[Cut]) -> ElevationFace {
    let mut outer = vec![Point::new(s_start, 0.0)];
    for notch in notches {
        outer.extend([Point::new(notch.s_min, 0.0), Point::new(notch.s_min, notch.z_max), Point::new(notch.s_max, notch.z_max), Point::new(notch.s_max, 0.0)]);
    }
    outer.extend([Point::new(s_end, 0.0), Point::new(s_end, height), Point::new(s_start, height)]);
    let holes = holes.iter().map(|hole| vec![Point::new(hole.s_min, hole.z_min), Point::new(hole.s_min, hole.z_max), Point::new(hole.s_max, hole.z_max), Point::new(hole.s_max, hole.z_min)]).collect();
    ElevationFace { outer, holes }
}

/// 📐️ Where the straight end cut of the build-up meets the interface curve at lateral fraction `t` (left face 0, right face 1), as an arc-length coordinate; `fallback` is the interpolated value used on the outer faces and when the cut is degenerate.
fn interface_s(curve: &BulgeSeg, cut: &BulgeSeg, t: f64, fallback: f64, length: f64) -> f64 {
    if t <= EPS || t >= 1.0 - EPS || cut.chord() <= EPS {
        return fallback;
    }
    nearest_intersection(cut, curve, Extent::Unbounded, cut.point_at(t)).map_or(fallback, |hit| fraction(curve, hit) * length)
}

fn lerp(from: f64, to: f64, t: f64) -> f64 {
    from + (to - from) * t
}

/// ✂️ Splits the cuts into bottom notches (they start on the wall base) and holes (they float above it). Every cut is a valid frame cut and therefore lies strictly inside the wall.
pub fn split_cuts(cuts: &[Cut]) -> (Vec<Cut>, Vec<Cut>) {
    (cuts.iter().filter(|cut| cut.z_min <= EPS).map(|cut| Cut { z_min: 0.0, ..*cut }).collect(), cuts.iter().filter(|cut| cut.z_min > EPS).copied().collect())
}
//#endregion 🔖️Faces

//#region 🔖️Solid
/// 🧊️ The layered solid of one wall from its layout and the cuts of its valid openings.
pub fn wall_solid(snapshot: &ModelSnapshot, wall: &Wall, layout: &WallLayout, cuts: &[Cut]) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Wall);
    let layers = snapshot.wall_types.get(&wall.wall_type).map_or(&[][..], |kind| &kind.layers[..]);
    let (axis, total) = (seg(&wall.axis), layout.offset_left + layout.offset_right);
    let Some(ends) = face_ends(layout, &wall.axis) else { return builder.build() };
    if layout.height <= EPS || layout.length <= EPS || total <= EPS || layout.layer_offsets.len() != layers.len() + 1 {
        return builder.build();
    }
    let length = layout.length;
    let ((left_start, left_end), (right_start, right_end)) = (ends.left, ends.right);
    let (notches, holes) = split_cuts(cuts);
    let (start_cut, end_cut) = (BulgeSeg::line(point(&layout.left_face.start), point(&layout.right_face.start)), BulgeSeg::line(point(&layout.left_face.end), point(&layout.right_face.end)));
    let interfaces = |y: f64, curve: &BulgeSeg| {
        let t = (layout.offset_left - y) / total;
        (interface_s(curve, &start_cut, t, lerp(left_start, right_start, t), length), interface_s(curve, &end_cut, t, lerp(left_end, right_end, t), length))
    };
    for (index, layer) in layers.iter().enumerate().filter(|(_, layer)| layer.thickness > EPS) {
        let (left_y, right_y) = (layout.layer_offsets[index], layout.layer_offsets[index + 1]);
        let (Some(left_curve), Some(right_curve)) = (axis.offset(left_y), axis.offset(right_y)) else { continue };
        let ((ls, le), (rs, re)) = (interfaces(left_y, &left_curve), interfaces(right_y, &right_curve));
        let mesh = extrude_between_faces(&face(ls, le, layout.height, &notches, &holes), &face(rs, re, layout.height, &notches, &holes), &left_curve, &right_curve, length, layout.base_z, CHORD_TOLERANCE);
        builder.add(super::super::parts::LAYER, &layer.material, index as u32, &mesh);
    }
    builder.build()
}

/// 🔑️ What `wall_solid` reads of a wall besides its layout and cuts: the layers of its type (materials and thicknesses).
pub fn dependency(snapshot: &ModelSnapshot, wall: &Wall) -> DslValue {
    dep_object([("layers", dep_value(&snapshot.wall_types.get(&wall.wall_type).map(|kind| kind.layers.clone())))])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
