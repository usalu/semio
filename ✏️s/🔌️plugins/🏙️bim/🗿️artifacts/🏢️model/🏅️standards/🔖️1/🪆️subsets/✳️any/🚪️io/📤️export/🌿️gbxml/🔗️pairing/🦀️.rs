//! 🔗️ One surface for the two sides of a partition. The envelope of a space lists its partitions on its own, so a wall between two rooms, or the floor of one room that is the ceiling of the room below, shows up in both; gbXML describes that surface
//! once with two `AdjacentSpaceId`s. Two sides pair when they are the same kind of partition between the same two spaces and overlap: walls of one wall element facing each other (azimuths 180 degrees apart) overlap in at least half of the smaller
//! side measured in the plane of the wall; a floor and the ceiling it lies on have the same area and the same centre in plan. The surface of the pair is the wall of the space with the lower id, or the floor of the upper space; a side without
//! a partner stays a surface of its own with one `AdjacentSpaceId`. Windows and doors follow their host, merged by element.
//! 📎 https://www.gbxml.org/schema_doc/7.03/GreenBuildingXML_Ver7.03.html

use crate::standards::v1::subsets::any::schema::inferences::energy_envelope::{Boundary, EnvelopeSpace, EnvelopeSurface, SurfaceKind};
use std::collections::{BTreeMap, BTreeSet};

/// 🧱️ One surface of the file: the side that gives its geometry, the side behind it when the partition is shared, and the windows and doors in it.
#[derive(Clone, Debug)]
pub struct Merged<'a> {
    pub space: &'a str,
    pub surface: &'a EnvelopeSurface,
    pub other: Option<(&'a str, &'a EnvelopeSurface)>,
    pub children: Vec<(&'a str, &'a EnvelopeSurface)>,
}

impl Merged<'_> {
    /// 🏠️ The spaces on the two sides: the space the normal points out of, then the one behind.
    pub fn spaces(&self) -> Vec<&str> {
        let mut spaces = vec![self.space];
        if let Some((other, _)) = self.other {
            spaces.push(other);
        } else if !self.surface.adjacent.is_empty() {
            spaces.push(self.surface.adjacent.as_str());
        }
        spaces.dedup();
        spaces
    }
}

fn shared(surface: &EnvelopeSurface) -> bool {
    matches!(surface.boundary, Boundary::Adjacent | Boundary::Adiabatic) && !surface.adjacent.is_empty() && !surface.polygon.is_empty()
}

fn horizontal(surface: &EnvelopeSurface) -> bool {
    matches!(surface.kind, SurfaceKind::Floor | SurfaceKind::Ceiling)
}

fn plan_centre(surface: &EnvelopeSurface) -> [f64; 2] {
    let count = surface.polygon.len() as f64;
    [surface.polygon.iter().map(|point| point.x).sum::<f64>() / count, surface.polygon.iter().map(|point| point.y).sum::<f64>() / count]
}

fn stretch(surface: &EnvelopeSurface, direction: [f64; 2]) -> ([f64; 2], [f64; 2]) {
    let mut along = [f64::INFINITY, f64::NEG_INFINITY];
    let mut height = [f64::INFINITY, f64::NEG_INFINITY];
    for point in &surface.polygon {
        let at = point.x * direction[0] + point.y * direction[1];
        along = [along[0].min(at), along[1].max(at)];
        height = [height[0].min(point.z), height[1].max(point.z)];
    }
    (along, height)
}

fn overlap(a: [f64; 2], b: [f64; 2]) -> f64 {
    (a[1].min(b[1]) - a[0].max(b[0])).max(0.0)
}

fn score(mine: &EnvelopeSurface, theirs: &EnvelopeSurface) -> Option<f64> {
    if horizontal(mine) != horizontal(theirs) {
        return None;
    }
    if horizontal(mine) {
        let wanted = if mine.kind == SurfaceKind::Floor { SurfaceKind::Ceiling } else { SurfaceKind::Floor };
        let (a, b) = (plan_centre(mine), plan_centre(theirs));
        let close = (a[0] - b[0]).hypot(a[1] - b[1]) <= 1e-3 && (mine.area - theirs.area).abs() <= 1e-4 * (1.0 + mine.area.max(theirs.area));
        return (theirs.kind == wanted && close).then(|| 1.0 / (1.0 + (a[0] - b[0]).hypot(a[1] - b[1])));
    }
    if mine.kind != theirs.kind || mine.element != theirs.element || ((mine.azimuth - theirs.azimuth).rem_euclid(360.0) - 180.0).abs() > 20.0 {
        return None;
    }
    let radians = mine.azimuth.to_radians();
    let direction = [radians.cos(), -radians.sin()];
    let ((a_along, a_height), (b_along, b_height)) = (stretch(mine, direction), stretch(theirs, direction));
    let common = overlap(a_along, b_along) * overlap(a_height, b_height);
    let smaller = ((a_along[1] - a_along[0]) * (a_height[1] - a_height[0])).min((b_along[1] - b_along[0]) * (b_height[1] - b_height[0]));
    (common > 1e-9 && common >= 0.5 * smaller).then_some(common)
}

/// 🔗️ The surfaces of the file from the envelopes of the spaces, in space order then envelope order; the second side of a pair is not listed again.
pub fn merge<'a>(envelopes: &'a BTreeMap<String, EnvelopeSpace>) -> Vec<Merged<'a>> {
    let hosts: Vec<(&str, &EnvelopeSurface)> = envelopes.iter().flat_map(|(space, envelope)| envelope.surfaces.iter().filter(|surface| surface.parent.is_empty()).map(move |surface| (space.as_str(), surface))).collect();
    let children = |space: &str, host: &EnvelopeSurface| -> Vec<(&'a str, &'a EnvelopeSurface)> {
        envelopes.get_key_value(space).into_iter().flat_map(|(key, envelope)| envelope.surfaces.iter().filter(|surface| surface.parent == host.id).map(move |surface| (key.as_str(), surface))).collect()
    };
    let mut taken: BTreeSet<(&str, &str)> = BTreeSet::new();
    let mut merged = Vec::new();
    for (space, host) in &hosts {
        if taken.contains(&(*space, host.id.as_str())) {
            continue;
        }
        taken.insert((*space, host.id.as_str()));
        let partner = shared(host)
            .then(|| {
                hosts
                    .iter()
                    .filter(|(other, candidate)| *other == host.adjacent && candidate.adjacent == *space && shared(candidate) && !taken.contains(&(*other, candidate.id.as_str())))
                    .filter_map(|(other, candidate)| score(host, candidate).map(|value| (value, *other, *candidate)))
                    .max_by(|a, b| a.0.total_cmp(&b.0).then_with(|| b.2.id.cmp(&a.2.id)))
            })
            .flatten();
        let Some((_, other, candidate)) = partner else {
            merged.push(Merged { space, surface: host, other: None, children: children(space, host) });
            continue;
        };
        taken.insert((other, candidate.id.as_str()));
        let (first, second) = if horizontal(host) {
            if host.kind == SurfaceKind::Floor { ((*space, *host), (other, candidate)) } else { ((other, candidate), (*space, *host)) }
        } else if *space <= other {
            ((*space, *host), (other, candidate))
        } else {
            ((other, candidate), (*space, *host))
        };
        let mut kids = children(first.0, first.1);
        let known: BTreeSet<String> = kids.iter().map(|(_, kid)| kid.element.clone()).collect();
        kids.extend(children(second.0, second.1).into_iter().filter(|(_, kid)| !known.contains(&kid.element)));
        merged.push(Merged { space: first.0, surface: first.1, other: Some(second), children: kids });
    }
    merged
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
