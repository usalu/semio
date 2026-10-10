//! 🩺️ The findings about the components of one storey: instances of a missing or profile family, hosts that are no wall of the storey, overrides whose formulas have no usable value, components outside their storey,
//! non-hosted components that stand inside a wall and terminals without a run of their system. They are computed from the `Component` values (the parents of the `Diagnostics` node of the storey), the wall layouts and the
//! MEP values of the storey; nothing is read from the snapshot besides the identity of the storey.

use super::{ComponentIssueCode, ComponentValue};
use crate::standards::v1::subsets::any::schema::inferences::bodies::{region, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::{Diagnostic, DiagnosticCode, Inputs};
use crate::standards::v1::subsets::any::schema::inferences::mep::{MepValue, CONNECT_DISTANCE};
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::WallLayout;
use semio_framework_2d::booleans::BooleanOperation;
use semio_framework_2d::regions::{region_boolean, Region};
use semio_framework_geometry::loops::{self, Vertex as Corner};
use semio_framework_geometry::Point;
use std::collections::BTreeSet;

/// 📏️ A component farther than this (metres) beyond the walls of its storey in plan is outside it.
pub const FAR: f64 = 25.0;
/// 📏️ Vertical overlaps and heights below this (metres) are none.
pub const HEIGHT_EPS: f64 = 1e-6;
/// 📏️ Overlaps with a wall below this (square metres, 1 cm2) are none.
pub const OVERLAP_EPS: f64 = 1e-4;
/// 📏️ A hosted component reaching deeper than this (metres) behind the face of its wall stands in the wall.
pub const DEPTH_EPS: f64 = 1e-3;

fn wall_region(layout: &WallLayout) -> Option<(Region, [f64; 4])> {
    let ring: Vec<Corner> = layout.footprint.iter().map(|vertex| Corner::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect();
    if ring.len() < 3 {
        return None;
    }
    let points = loops::flatten(&ring, CHORD_TOLERANCE);
    let first = points.first()?;
    let bounds = points.iter().fold([first.x, first.y, first.x, first.y], |b, p| [b[0].min(p.x), b[1].min(p.y), b[2].max(p.x), b[3].max(p.y)]);
    Some((region(&ring, &[]), bounds))
}

fn footprint_region(value: &ComponentValue) -> Region {
    let ring: Vec<Corner> = value.footprint.iter().map(|point| Corner::corner(point.x, point.y)).collect();
    region(&ring, &[])
}

fn overlap_area(a: &Region, b: &Region) -> f64 {
    region_boolean(BooleanOperation::Intersection, std::slice::from_ref(a), std::slice::from_ref(b), &mut |_| true).map_or(0.0, |rows| rows.iter().map(Region::area).sum())
}

fn depth_behind_face(value: &ComponentValue) -> f64 {
    let Some(fit) = &value.placement.host else { return 0.0 };
    value.footprint.iter().map(|corner| (corner.x - fit.face.x) * fit.normal.x + (corner.y - fit.face.y) * fit.normal.y).fold(f64::INFINITY, f64::min).min(0.0).abs()
}

fn beyond_walls(value: &ComponentValue, walls: &[(&str, Region, [f64; 4])]) -> f64 {
    let Some(first) = walls.first() else { return 0.0 };
    let all = walls.iter().fold(first.2, |b, (_, _, w)| [b[0].min(w[0]), b[1].min(w[1]), b[2].max(w[2]), b[3].max(w[3])]);
    let (x, y) = (value.placement.x, value.placement.y);
    (all[0] - x).max(x - all[2]).max(0.0).hypot((all[1] - y).max(y - all[3]).max(0.0))
}

fn connected(value: &ComponentValue, meps: &[(&str, &MepValue)]) -> bool {
    let Some(connector) = &value.connector else { return true };
    let at = connector.position;
    meps.iter().filter(|(_, mep)| mep.system == connector.system && mep.buildable()).any(|(_, mep)| {
        mep.ends().is_some_and(|(start, end)| [start, end].iter().any(|point| (point.x - at.x).hypot(point.y - at.y).hypot(point.z - at.z) <= CONNECT_DISTANCE))
    })
}

/// 🩺️ The findings about the components of `storey`.
pub fn storey(storey: &str, inputs: &Inputs<'_>) -> Vec<Diagnostic> {
    let mut found = Vec::new();
    let level = inputs.levels.get(storey).copied().unwrap_or_default();
    let height = level.top_elevation - level.elevation;
    let walls: Vec<(&str, Region, [f64; 4])> = inputs.layouts.iter().filter_map(|(id, layout)| wall_region(layout).map(|(region, bounds)| (*id, region, bounds))).collect();
    let meps: Vec<(&str, &MepValue)> = inputs.meps.iter().map(|(id, value)| (*id, *value)).collect();
    for (id, value) in inputs.components.iter().filter(|(_, value)| value.storey == storey) {
        let at = |code: DiagnosticCode| Diagnostic::new(code, &[id]).on(storey);
        let mut parameters: BTreeSet<&str> = BTreeSet::new();
        for issue in &value.issues {
            match issue.code {
                ComponentIssueCode::FamilyMissing | ComponentIssueCode::FamilyProfile => found.push(at(DiagnosticCode::RefComponentFamily).lacking(&issue.subject)),
                ComponentIssueCode::HostMissing | ComponentIssueCode::HostOtherStorey => found.push(at(DiagnosticCode::RefComponentHost).lacking(&issue.subject)),
                ComponentIssueCode::Override => {
                    parameters.insert(issue.subject.as_str());
                }
                ComponentIssueCode::NonFinite => found.push(at(DiagnosticCode::NonFinite)),
                ComponentIssueCode::HostDegenerate => {}
            }
        }
        if !parameters.is_empty() {
            found.push(parameters.into_iter().fold(at(DiagnosticCode::ComponentOverride), |finding, name| finding.lacking(name)));
        }
        if value.has(ComponentIssueCode::NonFinite) {
            continue;
        }
        let placed = value.placement.z - level.elevation;
        let distance = beyond_walls(value, &walls);
        if placed < -HEIGHT_EPS || placed > height + HEIGHT_EPS || distance > FAR {
            found.push(at(DiagnosticCode::ComponentOutsideStorey).with("height", placed).with("storey_height", height).with("distance", distance));
        }
        if value.solid() {
            let shape = footprint_region(value);
            let test = |wall: &str, region: &Region| -> Option<Diagnostic> {
                let layout = inputs.layouts.get(wall)?;
                let (low, high) = (value.bounds.min.z.max(layout.base_z), value.bounds.max.z.min(layout.top_z));
                let area = if high - low > HEIGHT_EPS { overlap_area(&shape, region) } else { 0.0 };
                (area > OVERLAP_EPS).then(|| Diagnostic::new(DiagnosticCode::ComponentInWall, &[id, wall]).on(storey).with("overlap_area", area).with("overlap_height", high - low))
            };
            match &value.placement.host {
                None => {
                    let (x0, y0, x1, y1) = (value.bounds.min.x, value.bounds.min.y, value.bounds.max.x, value.bounds.max.y);
                    found.extend(walls.iter().filter(|(_, _, b)| b[0] <= x1 && b[2] >= x0 && b[1] <= y1 && b[3] >= y0).filter_map(|(wall, region, _)| test(wall, region)));
                }
                Some(fit) if depth_behind_face(value) > DEPTH_EPS => found.extend(walls.iter().filter(|(wall, _, _)| *wall == fit.wall).filter_map(|(wall, region, _)| test(wall, region))),
                Some(_) => {}
            }
        }
        if value.terminal() && !connected(value, &meps) {
            found.push(at(DiagnosticCode::TerminalUnconnected).with("distance", CONNECT_DISTANCE));
        }
    }
    found
}
