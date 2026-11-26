//! 🔗️ One surface for the two sides of a partition. The envelope of a space lists its partitions on its own, so a wall between two rooms, or the floor of one room that is the ceiling of the room below, shows up in both; gbXML describes that surface
//! once with two `AdjacentSpaceId`s. Two sides pair when they are the same kind of partition between the same two spaces and overlap: walls of one wall element facing each other (azimuths 180 degrees apart) overlap in at least half of the smaller
//! side measured in the plane of the wall; a floor and the ceiling it lies on have the same area and the same centre in plan. The surface of the pair is the wall of the space with the lower id, or the floor of the upper space; a side without
//! a partner stays a surface of its own with one `AdjacentSpaceId`. Windows and doors follow their host, merged by element.
//! 📎 https://www.gbxml.org/schema_doc/7.03/GreenBuildingXML_Ver7.03.html

use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::{Boundary, EnvelopeSpace, EnvelopeSurface, SurfaceKind};
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

/// 🪟️ Whether an envelope side can have a reciprocal partner.
pub fn shared(surface: &EnvelopeSurface) -> bool {
    matches!(surface.boundary, Boundary::Adjacent | Boundary::Adiabatic) && !surface.adjacent.is_empty() && !surface.polygon.is_empty()
}

fn horizontal(surface: &EnvelopeSurface) -> bool {
    matches!(surface.kind, SurfaceKind::Floor | SurfaceKind::Ceiling)
}

/// 🪪️ Selects the first side of a paired surface in canonical output.
pub fn prefer_side(space: &str, surface: &EnvelopeSurface, other: &str) -> bool {
    if horizontal(surface) { surface.kind == SurfaceKind::Floor } else { space <= other }
}

fn overlap(a: [f64; 2], b: [f64; 2]) -> f64 {
    (a[1].min(b[1]) - a[0].max(b[0])).max(0.0)
}

#[derive(Clone, Copy)]
struct Extent {
    centre_sum: [f64; 2],
    along: [f64; 2],
    height: [f64; 2],
}

impl Default for Extent {
    fn default() -> Self {
        Self { centre_sum: [0.0; 2], along: [f64::INFINITY, f64::NEG_INFINITY], height: [f64::INFINITY, f64::NEG_INFINITY] }
    }
}

#[derive(Clone, Copy)]
enum Metric { Horizontal, Wall([f64; 2]) }

/// 📏️ Bounded score for two reciprocal envelope surfaces.
#[derive(Clone, Copy)]
pub struct PairingScore {
    metric: Metric,
    sides: [Extent; 2],
    side: usize,
    point: usize,
}

impl PairingScore {
    /// 🎛️ Selects the geometric comparison from the authored surface kinds and wall directions.
    pub fn new(mine: &EnvelopeSurface, theirs: &EnvelopeSurface) -> Option<Self> {
        if horizontal(mine) != horizontal(theirs) { return None; }
        let wanted = if mine.kind == SurfaceKind::Floor { SurfaceKind::Ceiling } else { SurfaceKind::Floor };
        let metric = if horizontal(mine) {
            if theirs.kind != wanted { return None; }
            Metric::Horizontal
        } else {
            if mine.kind != theirs.kind || mine.element != theirs.element || ((mine.azimuth - theirs.azimuth).rem_euclid(360.0) - 180.0).abs() > 20.0 {
                return None;
            }
            let radians = mine.azimuth.to_radians();
            Metric::Wall([radians.cos(), -radians.sin()])
        };
        Some(Self { metric, sides: [Extent::default(); 2], side: 0, point: 0 })
    }

    /// 🧭️ Inspects at most `points` vertices from the same two original sides.
    pub fn advance(&mut self, mine: &EnvelopeSurface, theirs: &EnvelopeSurface, points: usize) -> Option<Option<f64>> {
        let surfaces = [mine, theirs];
        for _ in 0..points {
            if self.side == 2 { break; }
            let Some(point) = surfaces[self.side].polygon.get(self.point) else {
                self.side += 1;
                self.point = 0;
                continue;
            };
            let extent = &mut self.sides[self.side];
            match self.metric {
                Metric::Horizontal => {
                    extent.centre_sum[0] += point.x;
                    extent.centre_sum[1] += point.y;
                }
                Metric::Wall(direction) => {
                    let at = point.x * direction[0] + point.y * direction[1];
                    extent.along = [extent.along[0].min(at), extent.along[1].max(at)];
                    extent.height = [extent.height[0].min(point.z), extent.height[1].max(point.z)];
                }
            }
            self.point += 1;
        }
        (self.side == 2).then(|| self.result(mine, theirs))
    }

    fn result(&self, mine: &EnvelopeSurface, theirs: &EnvelopeSurface) -> Option<f64> {
        let [a, b] = self.sides;
        match self.metric {
            Metric::Horizontal => {
                let a = a.centre_sum.map(|coordinate| coordinate / mine.polygon.len() as f64);
                let b = b.centre_sum.map(|coordinate| coordinate / theirs.polygon.len() as f64);
                let distance = (a[0] - b[0]).hypot(a[1] - b[1]);
                let close = distance <= 1e-3 && (mine.area - theirs.area).abs() <= 1e-4 * (1.0 + mine.area.max(theirs.area));
                close.then(|| 1.0 / (1.0 + distance))
            }
            Metric::Wall(_) => {
                let common = overlap(a.along, b.along) * overlap(a.height, b.height);
                let size = |extent: Extent| (extent.along[1] - extent.along[0]) * (extent.height[1] - extent.height[0]);
                (common > 1e-9 && common >= 0.5 * size(a).min(size(b))).then_some(common)
            }
        }
    }
}

/// 🔗️ The surfaces of the file from the envelopes of the spaces, in space order then envelope order; the second side of a pair is not listed again.
pub fn merge<'a>(envelopes: &'a BTreeMap<String, EnvelopeSpace>) -> Vec<Merged<'a>> {
    merge_checked(envelopes, &mut || Ok::<_, std::convert::Infallible>(()))
        .unwrap_or_else(|never| match never {})
}

/// 🧭️ Pairs the same hosts while honoring a caller checkpoint during indexing and reciprocal searches.
pub fn merge_checked<'a, E>(envelopes: &'a BTreeMap<String, EnvelopeSpace>, checkpoint: &mut impl FnMut() -> Result<(), E>) -> Result<Vec<Merged<'a>>, E> {
    let mut hosts = Vec::new();
    let mut partitions = BTreeMap::<_, Vec<_>>::new();
    let mut openings = BTreeMap::<_, Vec<_>>::new();
    for (space, envelope) in envelopes {
        for surface in &envelope.surfaces {
            checkpoint()?;
            let row = (space.as_str(), surface);
            if !surface.parent.is_empty() {
                openings.entry((row.0, surface.parent.as_str())).or_default().push(row);
                continue;
            }
            hosts.push(row);
            if shared(surface) {
                partitions.entry((row.0, surface.adjacent.as_str(), partition_family(surface))).or_default().push(row);
            }
        }
    }
    let children = |space: &'a str, host: &'a EnvelopeSurface| openings.get(&(space, host.id.as_str())).into_iter().flatten().copied();
    let mut taken: BTreeSet<(&str, &str)> = BTreeSet::new();
    let mut merged = Vec::new();
    for (space, host) in hosts {
        checkpoint()?;
        if !taken.insert((space, host.id.as_str())) {
            continue;
        }
        let candidates = shared(host).then(|| partitions.get(&(host.adjacent.as_str(), space, partition_family(host)))).flatten();
        let mut partner: Option<(f64, &str, &EnvelopeSurface)> = None;
        for &(other, candidate) in candidates.into_iter().flatten() {
            checkpoint()?;
            if taken.contains(&(other, candidate.id.as_str())) { continue; }
            let Some(mut score) = PairingScore::new(host, candidate) else { continue };
            let scored = loop {
                checkpoint()?;
                if let Some(scored) = score.advance(host, candidate, 64) { break scored; }
            };
            let Some(value) = scored else { continue };
            if partner.is_none_or(|(best, _, previous)| value.total_cmp(&best).then_with(|| previous.id.cmp(&candidate.id)).is_ge()) {
                partner = Some((value, other, candidate));
            }
        }
        let Some((_, other, candidate)) = partner else {
            merged.push(Merged { space, surface: host, other: None, children: children(space, host).collect() });
            continue;
        };
        taken.insert((other, candidate.id.as_str()));
        let prefer_host = prefer_side(space, host, other);
        let (first, second) = if prefer_host {
            ((space, host), (other, candidate))
        } else {
            ((other, candidate), (space, host))
        };
        let mut kids: Vec<_> = children(first.0, first.1).collect();
        let known: BTreeSet<&str> = kids.iter().map(|(_, kid)| kid.element.as_str()).collect();
        kids.extend(children(second.0, second.1).filter(|(_, kid)| !known.contains(kid.element.as_str())));
        merged.push(Merged { space: first.0, surface: first.1, other: Some(second), children: kids });
    }
    Ok(merged)
}

fn partition_family(surface: &EnvelopeSurface) -> Option<(&str, &'static str)> {
    (!horizontal(surface)).then(|| (surface.element.as_str(), surface.kind.name()))
}

