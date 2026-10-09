//! 🛤️ `railings`: the solid of every railing, posts along the path, a top rail swept over it, optional balusters and an infill, standing on the storey floor plus `base_offset`.
//!
//! Every section is authored. The top rail sweeps `profile` with its highest point on the top of the railing (`height` above the base), like a beam profile; a post extrudes `post_profile` (its `x`
//! along the segment it belongs to, like a column) from the base to the underside of the rail. Posts stand on every vertex of the path and divide each straight segment into the fewest equal parts
//! no longer than `post_spacing` (zero or less leaves only the vertex posts); the parts between two neighbouring posts are the bays. In every bay `ceil(length / spacing)` balusters stand at equal
//! intervals strictly between the two posts, never on one. The infill is one slab per bay, as thick as authored and centred on the path, from `INFILL_FOOT` above the base to the underside of the
//! rail, inset by half the width of the post at both ends; glass takes the first material of the library of the glass category, a panel and a missing glass material the material of the railing.
//! A path with fewer than two distinct points is absent.
//!
//! Related: <https://en.wikipedia.org/wiki/Baluster>.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::beams::section_of;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::columns::profile_loop;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{extents_of, placed, point, rectangle};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_object, dep_value, parts, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::{Infill, MaterialCategory, ModelSnapshot, Railing};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::mesh::{extrude_loops, sweep_profile, TriMesh};
use semio_framework_geometry::placement::ZPlane;
use semio_framework_geometry::vector::lerp;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;

//#region 🔖️Geometry
/// 📏️ Height of the underside of an infill above the base of the railing, in metres.
pub const INFILL_FOOT: f64 = 0.05;

/// 🛤️ The posts, the balusters, the rail and the infill of one railing, with the resolved heights.
#[derive(Clone, Debug, PartialEq)]
pub struct RailingGeometry {
    pub base_z: f64,
    pub top_z: f64,
    pub length: f64,
    pub posts: Vec<Point>,
    pub balusters: Vec<Point>,
    pub post_mesh: TriMesh,
    pub baluster_mesh: TriMesh,
    pub rail_mesh: TriMesh,
    pub infill_mesh: TriMesh,
}

/// 🧱️ The part of a straight segment between two neighbouring posts: its end points and the plan angle of the segment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bay {
    pub start: Point,
    pub end: Point,
    pub angle: f64,
}

fn segments_of(railing: &Railing) -> Vec<BulgeSeg> {
    let mut points: Vec<Point> = Vec::new();
    for p in railing.path.iter().map(point) {
        if points.last().is_none_or(|last| last.distance(p) > 1e-9) {
            points.push(p);
        }
    }
    points.windows(2).map(|pair| BulgeSeg::line(pair[0], pair[1])).collect()
}

fn divisions(length: f64, spacing: f64) -> usize {
    if spacing > 1e-9 { ((length / spacing - 1e-9).ceil() as usize).max(1) } else { 1 }
}

/// 🛤️ The post positions of a path: every vertex and equal subdivisions of at most `spacing` along each segment, with the orientation of the segment they belong to.
pub fn post_positions(segments: &[BulgeSeg], spacing: f64) -> Vec<(Point, f64)> {
    let mut posts = Vec::new();
    for (index, segment) in segments.iter().enumerate() {
        let (length, angle) = (segment.length(), (segment.end.y - segment.start.y).atan2(segment.end.x - segment.start.x));
        let count = divisions(length, spacing);
        for step in (if index == 0 { 0 } else { 1 })..=count {
            posts.push((segment.point_at(step as f64 / count as f64), angle));
        }
    }
    posts
}

/// 🧱️ The bays of a path: the parts between two neighbouring posts, in path order.
pub fn bays_of(segments: &[BulgeSeg], spacing: f64) -> Vec<Bay> {
    segments
        .iter()
        .flat_map(|segment| {
            let (length, angle) = (segment.length(), (segment.end.y - segment.start.y).atan2(segment.end.x - segment.start.x));
            let count = divisions(length, spacing);
            (0..count).map(move |step| Bay { start: segment.point_at(step as f64 / count as f64), end: segment.point_at((step + 1) as f64 / count as f64), angle })
        })
        .collect()
}

/// 🏛️ The baluster positions of a bay: `ceil(length / spacing)` stations at equal intervals strictly between its posts.
pub fn baluster_stations(bay: &Bay, spacing: f64) -> Vec<Point> {
    let count = divisions(bay.start.distance(bay.end), spacing);
    (1..count).map(|step| lerp(bay.start, bay.end, step as f64 / count as f64)).collect()
}

/// 🏛️ The number of balusters of a railing: the stations of every bay of its path (none without a baluster row).
pub fn baluster_count(railing: &Railing) -> u32 {
    railing.baluster.as_ref().map_or(0, |row| bays_of(&segments_of(railing), railing.post_spacing).iter().map(|bay| baluster_stations(bay, row.spacing).len() as u32).sum())
}

/// 🪟️ The glass of a library: the first material of the glass category by id.
pub fn glass_material(snapshot: &ModelSnapshot) -> Option<&str> {
    snapshot.materials.iter().find(|(_, material)| material.category == MaterialCategory::Glass).map(|(id, _)| id.as_str())
}

/// 🛤️ The geometry of a railing from the level of its storey.
pub fn railing_geometry(railing: &Railing, own: &StoreyLevel) -> RailingGeometry {
    let base_z = own.elevation + railing.base_offset;
    let top_z = base_z + railing.height;
    let segments = segments_of(railing);
    let length: f64 = segments.iter().map(BulgeSeg::length).sum();
    let empty = |posts: Vec<Point>| RailingGeometry { base_z, top_z, length, posts, balusters: Vec::new(), post_mesh: TriMesh::new(), baluster_mesh: TriMesh::new(), rail_mesh: TriMesh::new(), infill_mesh: TriMesh::new() };
    if segments.is_empty() || railing.height < 1e-9 {
        return empty(Vec::new());
    }
    let section = section_of(&railing.profile);
    let rail_mesh = if section.len() >= 3 { sweep_profile(&section, &[], &segments, top_z, CHORD_TOLERANCE) } else { TriMesh::new() };
    let underside = top_z - extents_of(&railing.profile).1;
    let positions = post_positions(&segments, railing.post_spacing);
    let posts: Vec<Point> = positions.iter().map(|row| row.0).collect();
    let column = |profile: &crate::Profile, at: Point, angle: f64| extrude_loops(&placed(&profile_loop(profile), at, angle), &[], CHORD_TOLERANCE, ZPlane::flat(base_z), ZPlane::flat(underside));
    if underside - base_z <= 1e-9 {
        return RailingGeometry { rail_mesh, ..empty(posts) };
    }
    let mut post_mesh = TriMesh::new();
    for (position, angle) in &positions {
        post_mesh.append(&column(&railing.post_profile, *position, *angle));
    }
    let bays = bays_of(&segments, railing.post_spacing);
    let (mut balusters, mut baluster_mesh) = (Vec::new(), TriMesh::new());
    if let Some(row) = &railing.baluster {
        for bay in &bays {
            for station in baluster_stations(bay, row.spacing) {
                baluster_mesh.append(&column(&row.profile, station, bay.angle));
                balusters.push(station);
            }
        }
    }
    let mut infill_mesh = TriMesh::new();
    if let Infill::Glass { thickness } | Infill::Panel { thickness } = railing.infill {
        let (inset, foot) = (extents_of(&railing.post_profile).0 / 2.0, base_z + INFILL_FOOT);
        for bay in &bays {
            let reach = bay.start.distance(bay.end) - 2.0 * inset;
            if reach > 1e-9 && underside - foot > 1e-9 {
                infill_mesh.append(&extrude_loops(&placed(&rectangle(reach, thickness), lerp(bay.start, bay.end, 0.5), bay.angle), &[], CHORD_TOLERANCE, ZPlane::flat(foot), ZPlane::flat(underside)));
            }
        }
    }
    RailingGeometry { base_z, top_z, length, posts, balusters, post_mesh, baluster_mesh, rail_mesh, infill_mesh }
}
//#endregion 🔖️Geometry

//#region 🔖️Solid
/// 🛤️ The solid of a railing from the level of its storey and the glass of the library.
pub fn railing_solid(snapshot: &ModelSnapshot, railing: &Railing, own: &StoreyLevel) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Railing);
    let geometry = railing_geometry(railing, own);
    let infill = match railing.infill {
        Infill::Glass { .. } => glass_material(snapshot).unwrap_or(&railing.material),
        _ => &railing.material,
    };
    builder.add(parts::POST, &railing.material, 0, &geometry.post_mesh).add(parts::RAIL, &railing.material, 0, &geometry.rail_mesh).add(parts::BALUSTER, &railing.material, 0, &geometry.baluster_mesh).add(parts::INFILL, infill, 0, &geometry.infill_mesh);
    builder.build()
}

/// 🔑️ What `railing_solid` reads besides the level: the railing record and, for a glass infill, the glass material it picks.
pub fn dependency(snapshot: &ModelSnapshot, railing: &Railing) -> DslValue {
    let glass = if matches!(railing.infill, Infill::Glass { .. }) { glass_material(snapshot).map(str::to_string) } else { None };
    dep_object([("railing", dep_value(railing)), ("glass", dep_value(&glass))])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
