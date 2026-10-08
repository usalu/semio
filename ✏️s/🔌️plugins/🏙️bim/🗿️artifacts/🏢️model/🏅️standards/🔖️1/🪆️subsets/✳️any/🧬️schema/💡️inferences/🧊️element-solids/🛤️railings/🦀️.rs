//! 🛤️ `railings`: the solid of every railing, square posts along the path and a top rail swept over it, standing on the storey floor plus `base_offset`.
//!
//! The railing record has no profile parameters, so the post and rail sections are the named constants [`POST_SIZE`], [`RAIL_WIDTH`] and [`RAIL_DEPTH`]. The top of the rail
//! lies at `height` above the base; posts rise from the base to the underside of the rail. Posts stand on every vertex of the path and divide each straight segment into the
//! fewest equal parts no longer than `post_spacing` (a spacing of zero or less leaves only the vertex posts). The rail is a mitered sweep along the whole path.
//! A path with fewer than two distinct points is absent.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{placed, point, rectangle};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_object, dep_value, parts, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::Railing;
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::mesh::{extrude_loops, sweep_profile, TriMesh};
use semio_framework_geometry::placement::ZPlane;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;

//#region 🔖️Geometry
/// 📏️ Side of the square posts, in metres.
pub const POST_SIZE: f64 = 0.05;
/// 📏️ Width of the top rail, in metres.
pub const RAIL_WIDTH: f64 = 0.06;
/// 📏️ Depth of the top rail, in metres.
pub const RAIL_DEPTH: f64 = 0.04;

/// 🛤️ The posts, the rail and the resolved heights of one railing.
#[derive(Clone, Debug, PartialEq)]
pub struct RailingGeometry {
    pub base_z: f64,
    pub top_z: f64,
    pub length: f64,
    pub posts: Vec<Point>,
    pub post_mesh: TriMesh,
    pub rail_mesh: TriMesh,
}

fn segments_of(railing: &Railing) -> Vec<BulgeSeg> {
    let mut points: Vec<Point> = Vec::new();
    for p in railing.path.iter().map(point) {
        if points.last().map_or(true, |last| last.distance(p) > 1e-9) {
            points.push(p);
        }
    }
    points.windows(2).map(|pair| BulgeSeg::line(pair[0], pair[1])).collect()
}

/// 🛤️ The post positions of a path: every vertex and equal subdivisions of at most `spacing` along each segment, with the orientation of the segment they belong to.
pub fn post_positions(segments: &[BulgeSeg], spacing: f64) -> Vec<(Point, f64)> {
    let mut posts = Vec::new();
    for (index, segment) in segments.iter().enumerate() {
        let (length, angle) = (segment.length(), (segment.end.y - segment.start.y).atan2(segment.end.x - segment.start.x));
        let divisions = if spacing > 1e-9 { ((length / spacing - 1e-9).ceil() as usize).max(1) } else { 1 };
        for step in (if index == 0 { 0 } else { 1 })..=divisions {
            posts.push((segment.point_at(step as f64 / divisions as f64), angle));
        }
    }
    posts
}

/// 🛤️ The geometry of a railing from the level of its storey.
pub fn railing_geometry(railing: &Railing, own: &StoreyLevel) -> RailingGeometry {
    let base_z = own.elevation + railing.base_offset;
    let top_z = base_z + railing.height;
    let segments = segments_of(railing);
    let length: f64 = segments.iter().map(BulgeSeg::length).sum();
    if segments.is_empty() || railing.height < 1e-9 {
        return RailingGeometry { base_z, top_z, length, posts: Vec::new(), post_mesh: TriMesh::new(), rail_mesh: TriMesh::new() };
    }
    let positions = post_positions(&segments, railing.post_spacing);
    let post_top = top_z - RAIL_DEPTH;
    let mut post_mesh = TriMesh::new();
    if post_top - base_z > 1e-9 {
        for (position, angle) in &positions {
            post_mesh.append(&extrude_loops(&placed(&rectangle(POST_SIZE, POST_SIZE), *position, *angle), &[], CHORD_TOLERANCE, ZPlane::flat(base_z), ZPlane::flat(post_top)));
        }
    }
    let section = [Point::new(-RAIL_WIDTH / 2.0, -RAIL_DEPTH), Point::new(RAIL_WIDTH / 2.0, -RAIL_DEPTH), Point::new(RAIL_WIDTH / 2.0, 0.0), Point::new(-RAIL_WIDTH / 2.0, 0.0)];
    let rail_mesh = sweep_profile(&section, &[], &segments, top_z, CHORD_TOLERANCE);
    RailingGeometry { base_z, top_z, length, posts: positions.into_iter().map(|row| row.0).collect(), post_mesh, rail_mesh }
}
//#endregion 🔖️Geometry

//#region 🔖️Solid
/// 🛤️ The solid of a railing from the level of its storey.
pub fn railing_solid(railing: &Railing, own: &StoreyLevel) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Railing);
    let geometry = railing_geometry(railing, own);
    builder.add(parts::POST, &railing.material, 0, &geometry.post_mesh).add(parts::RAIL, &railing.material, 0, &geometry.rail_mesh);
    builder.build()
}

/// 🔑️ What `railing_solid` reads besides the level: the railing record.
pub fn dependency(railing: &Railing) -> DslValue {
    dep_object([("railing", dep_value(railing))])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
