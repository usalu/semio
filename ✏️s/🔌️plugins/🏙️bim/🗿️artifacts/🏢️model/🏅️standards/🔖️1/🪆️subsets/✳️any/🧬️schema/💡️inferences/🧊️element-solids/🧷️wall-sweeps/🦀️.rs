//! 🧷️ Wall sweeps: the solid of every sweep, its profile swept along the join-trimmed face of its host wall (a baseboard, a cornice, a drip rail).
//!
//! The path is the face curve of the wall on the swept side (left or right looking along the axis, arcs included), from one end of the trimmed face to the other, interrupted wherever a valid opening of the host reaches into the
//! height range of the sweep (a door cuts a baseboard, a window above it does not). The profile outline is centred on its own origin with the first coordinate running out of the wall and the second running up: its lowest point lies `height`
//! above the base of the wall (the base profile of an attached base included) and its innermost point `inset` metres inside the face plane, so an inset of zero stands the profile on the face. Every run of the path is a closed body with flat
//! end caps (a sweep is cut square at the face ends and at openings; it is not mitered around corners). The material of the sweep is the only material of its solid.

use super::super::super::wall_layout::{face_ends, WallLayout};
use super::super::plan_kit::seg;
use super::super::walls::Cut;
use crate::standards::v1::subsets::any::schema::authored::profile::profile_polygon;
use super::super::{dep_object, dep_value, parts, Anonymous, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::{ModelSnapshot, Wall, WallSide, WallSweep};
use semio_framework_geometry::mesh::TriMesh;
use semio_framework_geometry::triangulation::triangulate;
use semio_framework_geometry::vector::perp;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;

const EPS: f64 = 1e-9;

//#region 🔖️Profile
/// ▭️ The profile of a sweep as a counter-clockwise polygon `(out of the wall, up)` with its lowest point at the origin height and its innermost point at the origin depth: `(a - a_min, b - b_min)`.
pub fn section_of(sweep: &WallSweep) -> Vec<Point> {
    let mut polygon = profile_polygon(&sweep.profile);
    let area: f64 = (0..polygon.len()).map(|k| polygon[k].x * polygon[(k + 1) % polygon.len()].y - polygon[(k + 1) % polygon.len()].x * polygon[k].y).sum();
    if area < 0.0 {
        polygon.reverse();
    }
    let (a_min, b_min) = (polygon.iter().map(|p| p.x).fold(f64::INFINITY, f64::min), polygon.iter().map(|p| p.y).fold(f64::INFINITY, f64::min));
    polygon.iter().map(|p| Point::new(p.x - a_min, p.y - b_min)).collect()
}

/// 📐️ The extents `(out of the wall, up)` of the profile of a sweep.
pub fn extents_of(sweep: &WallSweep) -> (f64, f64) {
    let section = section_of(sweep);
    (section.iter().map(|p| p.x).fold(0.0, f64::max), section.iter().map(|p| p.y).fold(0.0, f64::max))
}

/// 📐️ The area of the cross-section of a sweep.
pub fn section_area(sweep: &WallSweep) -> f64 {
    let polygon = section_of(sweep);
    (0..polygon.len()).map(|k| polygon[k].x * polygon[(k + 1) % polygon.len()].y - polygon[(k + 1) % polygon.len()].x * polygon[k].y).sum::<f64>() / 2.0
}

/// 📐️ The length of the outline of the profile that shows: the parts of its edges farther out than `inset`, without the edges that lie on the face plane.
pub fn visible_perimeter(sweep: &WallSweep) -> f64 {
    let polygon = section_of(sweep);
    let shown = |p: &Point| p.x - sweep.inset;
    (0..polygon.len())
        .map(|k| {
            let (a, b) = (polygon[k], polygon[(k + 1) % polygon.len()]);
            let (sa, sb) = (shown(&a), shown(&b));
            if sa.abs() < EPS && sb.abs() < EPS {
                return 0.0;
            }
            let length = (b - a).hypot();
            match (sa >= 0.0, sb >= 0.0) {
                (true, true) => length,
                (false, false) => 0.0,
                _ => length * (sa.max(sb) / (sa - sb).abs()),
            }
        })
        .sum()
}
//#endregion 🔖️Profile

//#region 🔖️Path
/// 📏️ The face curve of the swept side and the axis arc-length coordinates `(from, to)` of its trimmed ends, `None` when the wall has no footprint.
fn face_of(sweep: &WallSweep, wall: &Wall, layout: &WallLayout) -> Option<(semio_framework_geometry::bulge::BulgeSeg, (f64, f64), f64)> {
    let ends = face_ends(layout, &wall.axis)?;
    let (offset, range, outward) = match sweep.side {
        WallSide::Left => (layout.offset_left, ends.left, 1.0),
        WallSide::Right => (-layout.offset_right, ends.right, -1.0),
    };
    Some((seg(&wall.axis).offset(offset)?, range, outward))
}

/// ✂️ The runs `(from, to)` of the sweep along the axis (arc-length coordinates): the trimmed face less the cut rectangles of the valid openings that overlap the height range of the sweep.
pub fn runs(sweep: &WallSweep, wall: &Wall, layout: &WallLayout, cuts: &[Cut]) -> Vec<(f64, f64)> {
    let Some((_, (from, to), _)) = face_of(sweep, wall, layout) else { return Vec::new() };
    let rise = extents_of(sweep).1;
    let mut blocked: Vec<(f64, f64)> = cuts
        .iter()
        .filter(|cut| {
            let base = layout.base_at((cut.s_min + cut.s_max) / 2.0);
            let (bottom, top) = (base + sweep.height, base + sweep.height + rise);
            base + cut.z_min < top - EPS && base + cut.z_max > bottom + EPS
        })
        .map(|cut| (cut.s_min, cut.s_max))
        .collect();
    blocked.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut out = Vec::new();
    let mut at = from;
    for (low, high) in blocked {
        if low > at + EPS {
            out.push((at, low.min(to)));
        }
        at = at.max(high);
    }
    if to > at + EPS {
        out.push((at, to));
    }
    out.into_iter().filter(|(a, b)| b - a > EPS).collect()
}

/// 📏️ The length of the path of the sweep on its face (the face curve of the swept side, not the axis).
pub fn path_length(sweep: &WallSweep, wall: &Wall, layout: &WallLayout, cuts: &[Cut]) -> f64 {
    let Some((curve, (from, to), _)) = face_of(sweep, wall, layout) else { return 0.0 };
    let scale = if to - from > EPS { curve.length() / layout.length.max(EPS) } else { 0.0 };
    runs(sweep, wall, layout, cuts).iter().map(|(a, b)| (b - a) * scale).sum()
}

fn positions(curve: &semio_framework_geometry::bulge::BulgeSeg, layout: &WallLayout, from: f64, to: f64) -> Vec<f64> {
    let mut out = vec![from, to];
    out.extend(layout.base_profile.iter().map(|point| point.s).filter(|s| *s > from + EPS && *s < to - EPS));
    if !curve.is_line() && layout.length > EPS {
        let radius = curve.radius();
        let step = 2.0 * (1.0 - (CHORD_TOLERANCE / radius).clamp(1e-12, 1.0)).acos();
        let cells = (curve.sweep().abs() * (to - from) / layout.length / step.max(1e-6)).ceil().clamp(1.0, 4096.0) as usize;
        out.extend((1..cells).map(|k| from + (to - from) * k as f64 / cells as f64));
    }
    out.sort_by(f64::total_cmp);
    out.dedup_by(|a, b| (*a - *b).abs() < EPS);
    out
}

fn flipped(mesh: &TriMesh) -> TriMesh {
    let mut out = TriMesh::new();
    for k in 0..mesh.triangle_count() {
        let [a, b, c] = mesh.triangle(k);
        out.push_triangle(a, c, b);
    }
    out
}

fn run_mesh(sweep: &WallSweep, wall: &Wall, layout: &WallLayout, section: &[Point], from: f64, to: f64) -> TriMesh {
    let Some((curve, _, outward)) = face_of(sweep, wall, layout) else { return TriMesh::new() };
    let stations = positions(&curve, layout, from, to);
    let rings: Vec<Vec<[f64; 3]>> = stations
        .iter()
        .map(|&s| {
            let t = s / layout.length;
            let (at, tangent) = (curve.point_at(t), curve.tangent_at(t));
            let normal = perp(tangent) * outward;
            let floor = layout.base_at(s) + sweep.height;
            section.iter().map(|p| [at.x + normal.x * (p.x - sweep.inset), at.y + normal.y * (p.x - sweep.inset), floor + p.y]).collect()
        })
        .collect();
    let mut mesh = TriMesh::new();
    let count = section.len();
    for pair in rings.windows(2) {
        for j in 0..count {
            let next = (j + 1) % count;
            mesh.push_triangle(pair[0][j], pair[0][next], pair[1][next]);
            mesh.push_triangle(pair[0][j], pair[1][next], pair[1][j]);
        }
    }
    let cap = triangulate(section, &[]).triangles;
    for triangle in &cap {
        let [a, b, c] = triangle.map(|i| i as usize);
        mesh.push_triangle(rings[0][a], rings[0][c], rings[0][b]);
        let last = &rings[rings.len() - 1];
        mesh.push_triangle(last[a], last[b], last[c]);
    }
    if mesh.signed_volume() < 0.0 {
        flipped(&mesh)
    } else {
        mesh
    }
}
//#endregion 🔖️Path

//#region 🔖️Solid
/// 🧷️ The solid of a sweep from the layout of its host wall and the valid openings of that wall; empty when the profile has no area, the wall has no footprint or the material is unknown to the caller (the material is only a name here).
pub fn sweep_solid(sweep: &WallSweep, wall: &Wall, layout: &WallLayout, cuts: &[Cut]) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::WallSweep);
    let section = section_of(sweep);
    if section.len() < 3 || section_area(sweep) <= EPS || layout.length <= EPS {
        return builder.build();
    }
    let mut mesh = TriMesh::new();
    for (from, to) in runs(sweep, wall, layout, cuts) {
        mesh.append(&run_mesh(sweep, wall, layout, &section, from, to));
    }
    builder.add(parts::BODY, &sweep.material, 0, &mesh);
    builder.build()
}

/// 🔑️ What `sweep_solid` reads of a sweep besides the layout of its host and the cuts: the sweep record.
pub fn dependency(_snapshot: &ModelSnapshot, sweep: &WallSweep) -> DslValue {
    dep_object([("sweep", dep_value(&sweep.anonymous()))])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
