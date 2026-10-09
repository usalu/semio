//! 🧱️ Wall solids: one closed body per layer, mitered and butted at the joins of `wall-layout`, with every valid opening cut through all layers (reveals through the
//! thickness, door-like openings notch the bottom edge). Straight walls are prisms, arc walls are concentric slices tessellated within [`CHORD_TOLERANCE`].
//! The faces of the layers are offset curves of the axis at the interfaces of `WallLayout::layer_offsets`; the end `s` of an interface interpolates linearly between the
//! trimmed ends of the two outer faces, so a mitered join stays a straight cut across the whole build-up.
//!
//! The wall cuts **exactly the valid frames** of its hosted openings: `opening-frames` judges whether a hole fits strictly inside the full-thickness extent and below the top
//! (one rule, one tolerance), so there is no second predicate here that could drop a hole the frames call valid.
//!
//! The development of every face is the polygon between the base edge and the top edge of the wall: a straight line for a free wall, the elevation edges of
//! `WallLayout` (`base_profile`, `top_profile`) for a wall whose base or top is attached, so a wall under a pitched roof ends in the slope of the roof. Both faces of a layer carry the
//! same breakpoints (the profile points strictly inside the full-thickness extent): the profile is continued with the slope of its end segments over the mitered ends, so no breakpoint is lost there.
//! An opening with an authored reveal wears its reveal material on the jambs, the head and the sill between the front face and the reveal depth; the layers are split at that depth,
//! and the faces of the cut inside the reveal become their own group (`reveal`), so the volume of the wall does not change.

use super::super::super::opening_frames::OpeningFrame;
use super::super::super::wall_layout::attach::ElevationPoint;
use super::super::super::wall_layout::{face_ends, fraction, WallLayout};
use super::super::plan_kit::{point, seg};
use super::super::{dep_object, dep_value, parts, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::{ModelSnapshot, Wall};
use semio_framework_geometry::bulge::{nearest_intersection, BulgeSeg, Extent};
use semio_framework_geometry::mesh::{extrude_between_faces, ElevationFace, TriMesh};
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;

//#region 🔖️Cuts
/// ✂️ A rectangle removed from the development of a wall: `s` along the axis, `z` up from the local wall base (the base at the centre of the cut). `reveal` is the depth in metres of the reveal measured from the front face
/// (zero without a reveal material), `material` the material of its surfaces and `facing_right` whether the front face is the right one.
#[derive(Clone, Debug, PartialEq)]
pub struct Cut {
    pub s_min: f64,
    pub s_max: f64,
    pub z_min: f64,
    pub z_max: f64,
    pub reveal: f64,
    pub material: String,
    pub facing_right: bool,
}

/// ✂️ The cuts of the valid frames among `frames`, ordered along the wall.
pub fn cuts_of<'a>(frames: impl IntoIterator<Item = &'a OpeningFrame>) -> Vec<Cut> {
    let mut cuts: Vec<Cut> = frames
        .into_iter()
        .filter(|frame| frame.valid)
        .map(|frame| Cut {
            s_min: frame.cut.s_min,
            s_max: frame.cut.s_max,
            z_min: frame.cut.z_min,
            z_max: frame.cut.z_max,
            reveal: if frame.reveal_material.is_empty() { 0.0 } else { frame.reveal_depth },
            material: frame.reveal_material.clone(),
            facing_right: frame.facing_right,
        })
        .collect();
    cuts.sort_by(|a, b| a.s_min.total_cmp(&b.s_min));
    cuts
}
//#endregion 🔖️Cuts

//#region 🔖️Faces
const EPS: f64 = 1e-9;

/// 📈️ The marks strictly inside the full-thickness extent `lo..hi` among the arc-length coordinates of an elevation edge.
fn marks(points: &[ElevationPoint], lo: f64, hi: f64) -> Vec<f64> {
    points.iter().map(|point| point.s).filter(|s| *s > lo + EPS && *s < hi - EPS).collect()
}

/// ▭️ The development polygon of one face between the base edge and the top edge, with the bottom notches and the floating holes of the cuts. Heights are relative to the lowest base of the wall.
fn face(layout: &WallLayout, extent: (f64, f64), s_start: f64, s_end: f64, notches: &[Cut], holes: &[Cut]) -> ElevationFace {
    let rel = |z: f64| z - layout.base_z;
    let (base, top) = (|s: f64| rel(layout.base_at(s)), |s: f64| rel(layout.top_at(s)));
    let local = |cut: &Cut| layout.base_at((cut.s_min + cut.s_max) / 2.0);
    let (bottom_marks, top_marks) = (marks(&layout.base_profile, extent.0, extent.1), marks(&layout.top_profile, extent.0, extent.1));
    let mut outer = vec![Point::new(s_start, base(s_start))];
    let mut at = s_start;
    for notch in notches {
        outer.extend(bottom_marks.iter().filter(|mark| **mark > at && **mark < notch.s_min).map(|mark| Point::new(*mark, base(*mark))));
        let head = rel(local(notch) + notch.z_max);
        outer.extend([Point::new(notch.s_min, base(notch.s_min)), Point::new(notch.s_min, head), Point::new(notch.s_max, head), Point::new(notch.s_max, base(notch.s_max))]);
        at = notch.s_max;
    }
    outer.extend(bottom_marks.iter().filter(|mark| **mark > at).map(|mark| Point::new(*mark, base(*mark))));
    outer.extend([Point::new(s_end, base(s_end)), Point::new(s_end, top(s_end))]);
    outer.extend(top_marks.iter().rev().map(|mark| Point::new(*mark, top(*mark))));
    outer.push(Point::new(s_start, top(s_start)));
    let holes = holes
        .iter()
        .map(|hole| {
            let (floor, head) = (rel(local(hole) + hole.z_min), rel(local(hole) + hole.z_max));
            vec![Point::new(hole.s_min, floor), Point::new(hole.s_min, head), Point::new(hole.s_max, head), Point::new(hole.s_max, floor)]
        })
        .collect();
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
    (cuts.iter().filter(|cut| cut.z_min <= EPS).map(|cut| Cut { z_min: 0.0, ..cut.clone() }).collect(), cuts.iter().filter(|cut| cut.z_min > EPS).cloned().collect())
}
//#endregion 🔖️Faces

//#region 🔖️Reveals
/// 🪟️ The lateral coordinate (left positive) where the reveal of `cut` ends inside the wall.
fn reveal_end(cut: &Cut, layout: &WallLayout) -> f64 {
    if cut.facing_right {
        -layout.offset_right + cut.reveal
    } else {
        layout.offset_left - cut.reveal
    }
}

/// 🪟️ Whether the slice of a layer between the lateral coordinates `upper` and `lower` lies inside the reveal of `cut`.
fn inside_reveal(cut: &Cut, layout: &WallLayout, upper: f64, lower: f64) -> bool {
    if cut.facing_right {
        upper <= reveal_end(cut, layout) + EPS
    } else {
        lower >= reveal_end(cut, layout) - EPS
    }
}

/// 🪟️ Whether a triangle with the given corners (building coordinates) is a face of the hole of `cut`: all three corners on the same edge of its rectangle.
fn on_hole(cut: &Cut, layout: &WallLayout, axis: &BulgeSeg, corners: [[f64; 3]; 3]) -> bool {
    let base = layout.base_at((cut.s_min + cut.s_max) / 2.0);
    let tolerance = 1e-7;
    let local: Vec<(f64, f64)> = corners.iter().map(|corner| (fraction(axis, Point::new(corner[0], corner[1])) * layout.length, corner[2] - base)).collect();
    let near = |value: f64, edge: f64| (value - edge).abs() <= tolerance;
    local.iter().all(|(s, z)| *s >= cut.s_min - tolerance && *s <= cut.s_max + tolerance && *z >= cut.z_min - tolerance && *z <= cut.z_max + tolerance)
        && (local.iter().all(|(s, _)| near(*s, cut.s_min)) || local.iter().all(|(s, _)| near(*s, cut.s_max)) || local.iter().all(|(_, z)| near(*z, cut.z_min)) || local.iter().all(|(_, z)| near(*z, cut.z_max)))
}

/// ✂️ The triangles of `mesh` for which `chosen` holds, and the others; vertex data travels with the triangles.
fn split_by(mesh: &TriMesh, chosen: impl Fn([[f64; 3]; 3]) -> bool) -> (TriMesh, TriMesh) {
    let (mut yes, mut no) = (TriMesh::new(), TriMesh::new());
    for (k, triangle) in mesh.indices.iter().enumerate() {
        let target = if chosen(mesh.triangle(k)) { &mut yes } else { &mut no };
        let base = target.positions.len() as u32;
        for index in triangle {
            target.positions.push(mesh.positions[*index as usize]);
            target.normals.push(mesh.normals[*index as usize]);
        }
        target.indices.push([base, base + 1, base + 2]);
    }
    (yes, no)
}
//#endregion 🔖️Reveals

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
    let extent = ends.full_thickness();
    let ((left_start, left_end), (right_start, right_end)) = (ends.left, ends.right);
    let (notches, holes) = split_cuts(cuts);
    let revealing: Vec<&Cut> = cuts.iter().filter(|cut| cut.reveal > EPS && !cut.material.is_empty()).collect();
    let (start_cut, end_cut) = (BulgeSeg::line(point(&layout.left_face.start), point(&layout.right_face.start)), BulgeSeg::line(point(&layout.left_face.end), point(&layout.right_face.end)));
    let interfaces = |y: f64, curve: &BulgeSeg| {
        let t = (layout.offset_left - y) / total;
        (interface_s(curve, &start_cut, t, lerp(left_start, right_start, t), length), interface_s(curve, &end_cut, t, lerp(left_end, right_end, t), length))
    };
    for (index, layer) in layers.iter().enumerate().filter(|(_, layer)| layer.thickness > EPS) {
        let (left_y, right_y) = (layout.layer_offsets[index], layout.layer_offsets[index + 1]);
        let mut edges: Vec<f64> = revealing.iter().map(|cut| reveal_end(cut, layout)).filter(|y| *y < left_y - EPS && *y > right_y + EPS).collect();
        edges.sort_by(|a, b| b.total_cmp(a));
        edges.dedup_by(|a, b| (*a - *b).abs() <= EPS);
        edges.insert(0, left_y);
        edges.push(right_y);
        for slice in edges.windows(2) {
            let (upper, lower) = (slice[0], slice[1]);
            let (Some(left_curve), Some(right_curve)) = (axis.offset(upper), axis.offset(lower)) else { continue };
            let ((ls, le), (rs, re)) = (interfaces(upper, &left_curve), interfaces(lower, &right_curve));
            let mesh = extrude_between_faces(&face(layout, extent, ls, le, &notches, &holes), &face(layout, extent, rs, re, &notches, &holes), &left_curve, &right_curve, length, layout.base_z, CHORD_TOLERANCE);
            let owners: Vec<&Cut> = revealing.iter().copied().filter(|cut| inside_reveal(cut, layout, upper, lower)).collect();
            if owners.is_empty() {
                builder.add(parts::LAYER, &layer.material, index as u32, &mesh);
                continue;
            }
            let mut rest = mesh;
            for cut in owners {
                let (jambs, others) = split_by(&rest, |corners| on_hole(cut, layout, &axis, corners));
                builder.add(parts::REVEAL, &cut.material, index as u32, &jambs);
                rest = others;
            }
            builder.add(parts::LAYER, &layer.material, index as u32, &rest);
        }
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

#[cfg(test)]
#[path = "🧪️tests/🏔️attached/🦀️.rs"]
mod attached;
