//! 📦️ `bodies`: the prismatic bodies of the placed elements of a storey, shared by `plan-linework` and `diagnostics`.
//!
//! A body is a footprint (regions of flattened rings in building coordinates) between two heights (building datum): the vocabulary of
//! the plan cut and of the clash tests. Walls, curtain walls, columns, beams, slabs and stairs have one; they are derived from authored
//! parameters and the already inferred storey levels, wall layouts and stair runs (the nodes the callers are children of). Sloped or curved parts are bounded by their extent.

use super::super::element_solids::plan_kit::{bulged, depth_of, extents_of, placed, point, seg};
use super::super::stair_runs::StairRun;
use super::super::storey_levels::{target_of, vertical_of, StoreyLevel};
use super::super::wall_layout::WallLayout;
use crate::{Beam, Column, CurtainWall, ModelSnapshot, Profile, Slab, Stair, TopConstraint, Wall};
use semio_framework_2d::regions::Region;
use semio_framework_geometry::loops::{self, Vertex as Corner};
use semio_framework_geometry::Point;
use std::collections::BTreeMap;

//#region 🔖️Vocabulary
/// 📏️ Sagitta (metres) allowed when an arc becomes a polygon ring for booleans and overlap tests.
pub const CHORD_TOLERANCE: f64 = 1e-4;

/// 🧱️ What a body is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BodyKind {
    Wall,
    CurtainWall,
    Column,
    Beam,
    Slab,
    Stair,
}

/// 📦️ One element as a prism: its footprint between `z_min` and `z_max` (building datum), with the rectangle `bounds = [min_x, min_y, max_x, max_y]` of the footprint.
#[derive(Clone, Debug, PartialEq)]
pub struct Body {
    pub id: String,
    pub kind: BodyKind,
    pub storey: String,
    pub z_min: f64,
    pub z_max: f64,
    pub regions: Vec<Region>,
    pub bounds: [f64; 4],
}

impl Body {
    /// 🧱️ A body over `regions`; empty regions give `None`.
    pub fn new(id: &str, kind: BodyKind, storey: &str, z_min: f64, z_max: f64, regions: Vec<Region>) -> Option<Self> {
        let points = || regions.iter().flat_map(|region| region.outer.iter());
        let first = *points().next()?;
        let bounds = points().fold([first[0], first[1], first[0], first[1]], |b, p| [b[0].min(p[0]), b[1].min(p[1]), b[2].max(p[0]), b[3].max(p[1])]);
        if !(points().all(|p| p[0].is_finite() && p[1].is_finite()) && z_min.is_finite() && z_max.is_finite()) {
            return None;
        }
        Some(Self { id: id.to_string(), kind, storey: storey.to_string(), z_min: z_min.min(z_max), z_max: z_max.max(z_min), regions, bounds })
    }

    /// 📐️ Footprint area.
    pub fn area(&self) -> f64 {
        self.regions.iter().map(Region::area).sum()
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Conversions
/// 🔷️ A polygon ring of a geometry loop within [`CHORD_TOLERANCE`].
pub fn ring(vertices: &[Corner]) -> Vec<[f64; 2]> {
    loops::flatten(vertices, CHORD_TOLERANCE).into_iter().map(|p| [p.x, p.y]).collect()
}

/// 🧱️ The region of an outer loop and its hole loops.
pub fn region(outer: &[Corner], holes: &[Vec<Corner>]) -> Region {
    Region::new(&ring(outer), &holes.iter().map(|hole| ring(hole)).collect::<Vec<_>>())
}
//#endregion 🔖️Conversions

//#region 🔖️Profiles
/// ▭️ The counter-clockwise outline of a profile around its origin (`x` across, `y` deep).
pub fn profile_outline(profile: &Profile) -> Vec<Corner> {
    let corner = |x: f64, y: f64| Corner::corner(x, y);
    match profile {
        Profile::Rectangle { width, depth } => {
            let (w, d) = (width / 2.0, depth / 2.0);
            vec![corner(-w, -d), corner(w, -d), corner(w, d), corner(-w, d)]
        }
        Profile::Circle { diameter } => {
            let r = diameter / 2.0;
            vec![Corner::new(Point::new(r, 0.0), 1.0), Corner::new(Point::new(-r, 0.0), 1.0)]
        }
        Profile::IShape { width, depth, web, flange } => {
            let (w, d, t, f) = (width / 2.0, depth / 2.0, web / 2.0, *flange);
            vec![corner(-w, -d), corner(w, -d), corner(w, -d + f), corner(t, -d + f), corner(t, d - f), corner(w, d - f), corner(w, d), corner(-w, d), corner(-w, d - f), corner(-t, d - f), corner(-t, -d + f), corner(-w, -d + f)]
        }
        Profile::Custom { outline } => loops::ccw(&bulged(outline)),
    }
}
//#endregion 🔖️Profiles

//#region 🔖️Vertical
/// 🍰️ Total thickness of a slab's layers.
pub fn slab_thickness(snapshot: &ModelSnapshot, slab: &Slab) -> f64 {
    snapshot.slab_types.get(&slab.slab_type).map_or(0.0, |kind| kind.layers.iter().map(|layer| layer.thickness).sum())
}

/// 📐️ The vertical span a slab occupies: its top is the storey elevation plus its offset, its thickness hangs below, a slope widens it by the rise across its footprint.
pub fn slab_span(snapshot: &ModelSnapshot, slab: &Slab, own: &StoreyLevel) -> (f64, f64) {
    let top = own.elevation + slab.offset;
    let rise = slab.slope.map_or(0.0, |slope| {
        let points = slab.boundary.iter().map(|v| v.point.x * slope.direction.cos() + v.point.y * slope.direction.sin());
        let (low, high) = points.fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), p| (low.min(p), high.max(p)));
        if low.is_finite() {
            (high - low) * slope.angle.tan().abs()
        } else {
            0.0
        }
    });
    (top - slab_thickness(snapshot, slab), top + rise)
}
//#endregion 🔖️Vertical

//#region 🔖️Bodies
/// ➖️ The footprint of a straight band of `width` along `start..end`.
pub fn band(start: Point, end: Point, width: f64) -> Option<Vec<Corner>> {
    let along = end - start;
    let length = along.hypot();
    (length > 1e-9 && width > 1e-9).then(|| {
        let n = Point::new(-along.y / length * width / 2.0, along.x / length * width / 2.0);
        vec![Corner::corner(start.x - n.x, start.y - n.y), Corner::corner(end.x - n.x, end.y - n.y), Corner::corner(end.x + n.x, end.y + n.y), Corner::corner(start.x + n.x, start.y + n.y)]
    })
}

/// 🏛️ The placed outline of a column in building coordinates.
pub fn column_outline(snapshot: &ModelSnapshot, column: &Column) -> Option<Vec<Corner>> {
    let kind = snapshot.column_types.get(&column.column_type)?;
    let (width, depth) = extents_of(&kind.profile);
    (width > 1e-9 && depth > 1e-9 && column.position.x.is_finite() && column.position.y.is_finite() && column.rotation.is_finite()).then(|| placed(&profile_outline(&kind.profile), point(&column.position), column.rotation))
}

/// ➖️ The plan footprint of a beam: its profile width along its axis.
pub fn beam_outline(snapshot: &ModelSnapshot, beam: &Beam) -> Option<Vec<Corner>> {
    let kind = snapshot.beam_types.get(&beam.beam_type)?;
    band(point(&beam.start), point(&beam.end), extents_of(&kind.profile).0)
}

/// 🧱️ The vertical extent of a beam: its top hangs `top_offset` from the storey top, its depth below.
pub fn beam_span(snapshot: &ModelSnapshot, beam: &Beam, own: &StoreyLevel) -> Option<(f64, f64)> {
    let kind = snapshot.beam_types.get(&beam.beam_type)?;
    let top = own.top_elevation + beam.top_offset;
    Some((top - extents_of(&kind.profile).1, top))
}

/// 🧱️ The body of a wall: its join-trimmed footprint between its base and top.
pub fn wall_body(id: &str, wall: &Wall, layout: &WallLayout) -> Option<Body> {
    let outline = bulged(&layout.footprint);
    if outline.len() < 3 {
        return None;
    }
    Body::new(id, BodyKind::Wall, &wall.storey, layout.base_z, layout.top_z, vec![region(&outline, &[])])
}

/// 🪟️ The body of a curtain wall: its axis thickened by the mullion depth.
pub fn curtain_body(id: &str, curtain: &CurtainWall, own: &StoreyLevel, target: Option<&StoreyLevel>) -> Option<Body> {
    let depth = depth_of(&curtain.mullion);
    let (base, top) = vertical_of(curtain.base_offset, &curtain.top, own, target);
    let rows = semio_framework_geometry::bulge::band_loop(&seg(&curtain.axis), depth / 2.0, depth / 2.0, None, None)?;
    let outline: Vec<Corner> = rows.iter().map(|(p, b)| Corner::new(*p, *b)).collect();
    Body::new(id, BodyKind::CurtainWall, &curtain.storey, base, top, vec![region(&outline, &[])])
}

/// 🏛️ The body of a column.
pub fn column_body(snapshot: &ModelSnapshot, id: &str, column: &Column, own: &StoreyLevel, target: Option<&StoreyLevel>) -> Option<Body> {
    let (base, top) = vertical_of(column.base_offset, &column.top, own, target);
    Body::new(id, BodyKind::Column, &column.storey, base, top, vec![region(&column_outline(snapshot, column)?, &[])])
}

/// ➖️ The body of a beam.
pub fn beam_body(snapshot: &ModelSnapshot, id: &str, beam: &Beam, own: &StoreyLevel) -> Option<Body> {
    let (low, high) = beam_span(snapshot, beam, own)?;
    Body::new(id, BodyKind::Beam, &beam.storey, low, high, vec![region(&beam_outline(snapshot, beam)?, &[])])
}

/// ⬜️ The body of a slab.
pub fn slab_body(snapshot: &ModelSnapshot, id: &str, slab: &Slab, own: &StoreyLevel) -> Option<Body> {
    let (low, high) = slab_span(snapshot, slab, own);
    let holes: Vec<Vec<Corner>> = slab.holes.iter().map(|hole| bulged(hole)).collect();
    if slab.boundary.len() < 3 {
        return None;
    }
    Body::new(id, BodyKind::Slab, &slab.storey, low, high, vec![region(&bulged(&slab.boundary), &holes)])
}

/// 🪜️ The footprint of a rectangle `back` behind and `forward` ahead of `centre` along `direction`, `width` wide.
pub fn rectangle(centre: Point, direction: f64, back: f64, forward: f64, width: f64) -> Vec<Corner> {
    let (sin, cos) = direction.sin_cos();
    let at = |along: f64, aside: f64| Corner::corner(centre.x + along * cos - aside * sin, centre.y + along * sin + aside * cos);
    vec![at(-back, -width / 2.0), at(forward, -width / 2.0), at(forward, width / 2.0), at(-back, width / 2.0)]
}

/// 🪜️ The footprints of the flights and landings of a stair run, in building coordinates; a winding flight is its annular sector.
pub fn stair_outlines(run: &StairRun) -> Vec<Vec<Corner>> {
    let mut outlines = Vec::new();
    for flight in &run.flights {
        if let Some(winder) = flight.winder {
            let steps = ((winder.sweep.abs() / 0.1).ceil() as usize).max(2);
            let arc = |radius: f64| (0..=steps).map(|k| winder.start_angle + winder.sweep * k as f64 / steps as f64).map(move |a| Point::new(winder.centre.x + radius * a.cos(), winder.centre.y + radius * a.sin()));
            let mut sector: Vec<Point> = arc(winder.outer_radius).collect();
            let mut inner: Vec<Point> = arc(winder.inner_radius.max(1e-3)).collect();
            inner.reverse();
            sector.extend(inner);
            outlines.push(loops::ccw(&loops::from_polygon(&sector)));
        } else {
            let start = point(&flight.start);
            let (sin, cos) = flight.direction.sin_cos();
            let length = flight.length.max(flight.tread);
            let centre = Point::new(start.x + length / 2.0 * cos, start.y + length / 2.0 * sin);
            outlines.push(rectangle(centre, flight.direction, length / 2.0, length / 2.0, run.width));
        }
    }
    for landing in &run.landings {
        outlines.push(rectangle(point(&landing.centre), landing.direction, landing.depth / 2.0, landing.depth / 2.0, landing.width));
    }
    outlines
}

/// 🪜️ The body of a stair: its flights and landings between its base and top height.
pub fn stair_body(id: &str, stair: &Stair, run: &StairRun) -> Option<Body> {
    let regions: Vec<Region> = stair_outlines(run).iter().map(|outline| region(outline, &[])).collect();
    Body::new(id, BodyKind::Stair, &stair.storey, run.base_z, run.top_z, regions)
}

/// 📦️ Every body of one storey in id order within kind order: walls, curtain walls, columns, beams, slabs, stairs. Elements whose type or geometry does not resolve are absent. The wall layouts and stair runs are the ones the model graph inferred.
pub fn storey_bodies(snapshot: &ModelSnapshot, storey: &str, levels: &BTreeMap<String, StoreyLevel>, layouts: &BTreeMap<&str, &WallLayout>, runs: &BTreeMap<&str, &StairRun>) -> Vec<Body> {
    let Some(own) = levels.get(storey) else { return Vec::new() };
    let mut bodies = Vec::new();
    for (id, wall) in snapshot.walls.iter().filter(|(_, wall)| wall.storey == storey) {
        if let Some(layout) = layouts.get(id.as_str()) {
            bodies.extend(wall_body(id, wall, layout));
        }
    }
    for (id, curtain) in snapshot.curtain_walls.iter().filter(|(_, curtain)| curtain.storey == storey) {
        bodies.extend(curtain_body(id, curtain, own, target_of(&curtain.top, levels)));
    }
    for (id, column) in snapshot.columns.iter().filter(|(_, column)| column.storey == storey) {
        bodies.extend(column_body(snapshot, id, column, own, target_of(&column.top, levels)));
    }
    for (id, beam) in snapshot.beams.iter().filter(|(_, beam)| beam.storey == storey) {
        bodies.extend(beam_body(snapshot, id, beam, own));
    }
    for (id, slab) in snapshot.slabs.iter().filter(|(_, slab)| slab.storey == storey) {
        bodies.extend(slab_body(snapshot, id, slab, own));
    }
    for (id, stair) in snapshot.stairs.iter().filter(|(_, stair)| stair.storey == storey) {
        if let Some(run) = runs.get(id.as_str()) {
            bodies.extend(stair_body(id, stair, run));
        }
    }
    bodies
}
//#endregion 🔖️Bodies

//#region 🔖️Scope
/// 🧭️ The storeys whose levels an element set of `storey` depends on: its own, then every storey a top constraint targets, sorted and unique.
pub fn level_storeys(snapshot: &ModelSnapshot, storey: &str) -> Vec<String> {
    let targets = snapshot
        .walls
        .values()
        .filter(|row| row.storey == storey)
        .map(|row| &row.top)
        .chain(snapshot.curtain_walls.values().filter(|row| row.storey == storey).map(|row| &row.top))
        .chain(snapshot.columns.values().filter(|row| row.storey == storey).map(|row| &row.top))
        .chain(snapshot.stairs.values().filter(|row| row.storey == storey).map(|row| &row.top))
        .filter_map(|top| if let TopConstraint::Storey { storey, .. } = top { Some(storey.clone()) } else { None });
    let mut ids: Vec<String> = std::iter::once(storey.to_string()).chain(targets).filter(|id| snapshot.storeys.contains_key(id)).collect();
    let sorted_from = 1.min(ids.len());
    ids[sorted_from..].sort();
    ids.dedup();
    ids
}
//#endregion 🔖️Scope

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
