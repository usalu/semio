//! 🔗️ Wall joins: the plan-view contacts between the axes of the walls of one storey and the face trims they imply.
//!
//! A wall is a [`Band`]: its axis plus the distances from the axis to its left and right face. Contacts are found per axis end:
//! an end that coincides with other ends (within [`JOIN_TOLERANCE`]) forms a node of 2 or more walls, mitered pairwise around it; an
//! end that lies on the axis interior of another wall butts against that wall's near face (T join); two axes that cross in both
//! interiors form an X join, which trims nothing. The functions are pure geometry over bands and never read the snapshot.

use super::{JoinEnd, JoinKind, WallJoin};
use crate::Point2;
use semio_framework_geometry::bulge::{bulge_from_sweep, intersect, nearest_intersection, BulgeSeg, Extent};
use semio_framework_geometry::triangulation::triangulate;
use semio_framework_geometry::vector::{angle_between, cross};
use semio_framework_geometry::{Point, Vec2};
use std::collections::{BTreeMap, BTreeSet};

//#region 🔖️Vocabulary
/// 📏️ Distance (metres) within which two axis ends coincide, or an axis end lies on another axis.
pub const JOIN_TOLERANCE: f64 = 1e-6;

/// 📐️ A corner whose miter point lies farther than this many thicknesses from the node is cut square instead.
pub const MITER_LIMIT: f64 = 4.0;

/// 📍️ One end of a wall axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tip {
    Start,
    End,
}

impl Tip {
    pub const BOTH: [Tip; 2] = [Tip::Start, Tip::End];

    fn join_end(self) -> JoinEnd {
        match self {
            Tip::Start => JoinEnd::Start,
            Tip::End => JoinEnd::End,
        }
    }
}

/// ↔️ One face of a band, seen along the axis direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    fn other(self) -> Side {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }

    fn counter_clockwise_of(tip: Tip) -> Side {
        match tip {
            Tip::Start => Side::Left,
            Tip::End => Side::Right,
        }
    }
}

/// 🧱️ A wall in plan: its axis and the distances from the axis to its left and right face.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Band {
    pub axis: BulgeSeg,
    pub left: f64,
    pub right: f64,
}

/// ✂️ Where the left and right face of a band end after the joins at one of its tips. `cut` is the face of the wall it butts against: the end edge of the footprint follows it (an arc when that face is curved), otherwise the end edge is straight.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Trim {
    pub left: Point,
    pub right: Point,
    pub cut: Option<BulgeSeg>,
}

impl Trim {
    fn cut_bulge(&self, from: Point, to: Point) -> f64 {
        self.cut.and_then(|face| face.center()).map_or(0.0, |center| bulge_from_sweep(angle_between(from - center, to - center)))
    }
}

/// 🔗️ Everything the neighbours decide about one wall: its joins, its two end trims and the ids it depends on.
#[derive(Clone, Debug, PartialEq)]
pub struct Joined {
    pub start: Trim,
    pub end: Trim,
    pub joins: Vec<WallJoin>,
    pub neighbours: BTreeSet<String>,
}

/// 🧱️ The trimmed faces and the counter-clockwise footprint `[right.start, right.end, left.end, left.start]` of a band.
#[derive(Clone, Debug, PartialEq)]
pub struct Footprint {
    pub left: BulgeSeg,
    pub right: BulgeSeg,
    pub outline: [(Point, f64); 4],
}

enum Contact {
    Free,
    Node(Vec<(String, Tip)>),
    Butt { through: String, hit: Point, tangent: Vec2 },
}
//#endregion 🔖️Vocabulary

//#region 🔖️Band
impl Band {
    /// 🍰️ Distance between the two faces.
    pub fn thickness(&self) -> f64 {
        self.left + self.right
    }

    /// ↔️ The face curve on one side, parallel to the axis; `None` when an arc face collapses.
    pub fn face(&self, side: Side) -> Option<BulgeSeg> {
        match side {
            Side::Left => self.axis.offset(self.left),
            Side::Right => self.axis.offset(-self.right),
        }
    }

    /// 📍️ The axis point at a tip.
    pub fn tip(&self, tip: Tip) -> Point {
        match tip {
            Tip::Start => self.axis.start,
            Tip::End => self.axis.end,
        }
    }

    fn outward(&self, tip: Tip) -> Vec2 {
        match tip {
            Tip::Start => self.axis.tangent_at(0.0),
            Tip::End => -self.axis.tangent_at(1.0),
        }
    }

    fn square(&self, tip: Tip) -> Trim {
        let pick = |side: Side| self.face(side).map_or(self.tip(tip), |face| if tip == Tip::Start { face.start } else { face.end });
        Trim { left: pick(Side::Left), right: pick(Side::Right), cut: None }
    }

    fn interior(&self, t: f64) -> bool {
        let length = self.axis.length();
        t * length > JOIN_TOLERANCE && (1.0 - t) * length > JOIN_TOLERANCE
    }
}
//#endregion 🔖️Band

//#region 🔖️Contacts
fn contact(bands: &BTreeMap<String, Band>, id: &str, tip: Tip) -> Contact {
    let point = bands[id].tip(tip);
    let node: Vec<(String, Tip)> = bands.iter().filter(|(other, _)| other.as_str() != id).flat_map(|(other, band)| Tip::BOTH.into_iter().filter(move |end| (band.tip(*end) - point).hypot() <= JOIN_TOLERANCE).map(move |end| (other.clone(), end))).collect();
    if !node.is_empty() {
        return Contact::Node(node);
    }
    bands
        .iter()
        .filter(|(other, _)| other.as_str() != id)
        .find_map(|(other, band)| {
            let closest = band.axis.closest(point);
            (closest.distance <= JOIN_TOLERANCE).then(|| Contact::Butt { through: other.clone(), hit: closest.point, tangent: band.axis.tangent_at(closest.t) })
        })
        .unwrap_or(Contact::Free)
}

fn miter(ours: &Band, ours_side: Side, theirs: &Band, theirs_side: Side, node: Point) -> Option<Point> {
    let (a, b) = (ours.face(ours_side)?, theirs.face(theirs_side)?);
    let point = nearest_intersection(&a, &b, Extent::Unbounded, node)?;
    ((point - node).hypot() <= MITER_LIMIT * ours.thickness().max(theirs.thickness()) + JOIN_TOLERANCE).then_some(point)
}

fn node_trim(bands: &BTreeMap<String, Band>, id: &str, tip: Tip, members: &[(String, Tip)]) -> Trim {
    let (me, node) = (&bands[id], bands[id].tip(tip));
    let mut ring: Vec<(f64, &str, Tip)> = members.iter().map(|(other, end)| (other.as_str(), *end)).chain(std::iter::once((id, tip))).map(|(other, end)| (bands[other].outward(end).y.atan2(bands[other].outward(end).x), other, end)).collect();
    ring.sort_by(|a, b| a.0.total_cmp(&b.0).then_with(|| a.1.cmp(b.1)).then_with(|| a.2.cmp(&b.2)));
    let (count, index) = (ring.len(), ring.iter().position(|(_, other, end)| *other == id && *end == tip).unwrap_or(0));
    let (ccw, cw) = (&ring[(index + 1) % count], &ring[(index + count - 1) % count]);
    let ours_ccw = Side::counter_clockwise_of(tip);
    let at_ccw = miter(me, ours_ccw, &bands[ccw.1], Side::counter_clockwise_of(ccw.2).other(), node);
    let at_cw = miter(me, ours_ccw.other(), &bands[cw.1], Side::counter_clockwise_of(cw.2), node);
    let (left, right) = if tip == Tip::Start { (at_ccw, at_cw) } else { (at_cw, at_ccw) };
    let square = me.square(tip);
    Trim { left: left.unwrap_or(square.left), right: right.unwrap_or(square.right), cut: None }
}

fn butt_trim(bands: &BTreeMap<String, Band>, id: &str, tip: Tip, through: &str, hit: Point, tangent: Vec2) -> Trim {
    let (me, wall) = (&bands[id], &bands[through]);
    let square = me.square(tip);
    let orientation = cross(tangent, me.outward(tip));
    if orientation.abs() <= 1e-9 {
        return square;
    }
    let near = wall.face(if orientation > 0.0 { Side::Left } else { Side::Right });
    let cut = |side: Side| {
        let (face, near) = (me.face(side)?, near?);
        let point = nearest_intersection(&face, &near, Extent::Unbounded, hit)?;
        ((point - hit).hypot() <= MITER_LIMIT * me.thickness().max(wall.thickness()) + JOIN_TOLERANCE).then_some(point)
    };
    match (cut(Side::Left), cut(Side::Right)) {
        (Some(left), Some(right)) => Trim { left, right, cut: near },
        (left, right) => Trim { left: left.unwrap_or(square.left), right: right.unwrap_or(square.right), cut: None },
    }
}

fn crossings(a: &Band, b: &Band) -> Vec<Point> {
    intersect(&a.axis, &b.axis, Extent::Bounded).into_iter().filter(|hit| a.interior(hit.ta) && b.interior(hit.tb)).map(|hit| hit.point).collect()
}

fn triangles(band: &Band) -> Vec<[Point; 3]> {
    let (Some(left), Some(right)) = (band.face(Side::Left), band.face(Side::Right)) else { return Vec::new() };
    let mut ring = Vec::new();
    for segment in [right, BulgeSeg::line(right.end, left.end), left.reversed(), BulgeSeg::line(left.start, right.start)] {
        let mut points = segment.flatten(1e-6);
        points.pop();
        ring.extend(points);
    }
    let mesh = triangulate(&ring, &[]);
    mesh.triangles.iter().map(|corner| [mesh.vertices[corner[0] as usize], mesh.vertices[corner[1] as usize], mesh.vertices[corner[2] as usize]]).collect()
}

fn clip(subject: Vec<Point>, triangle: &[Point; 3]) -> Vec<Point> {
    let mut polygon = subject;
    for index in 0..3 {
        let (from, to) = (triangle[index], triangle[(index + 1) % 3]);
        let side = |point: Point| cross(to - from, point - from);
        let input = std::mem::take(&mut polygon);
        for (position, current) in input.iter().enumerate() {
            let previous = input[(position + input.len() - 1) % input.len()];
            let (inside_now, inside_before) = (side(*current) >= 0.0, side(previous) >= 0.0);
            if inside_now != inside_before {
                let t = side(previous) / (side(previous) - side(*current));
                polygon.push(Point::new(previous.x + (current.x - previous.x) * t, previous.y + (current.y - previous.y) * t));
            }
            if inside_now {
                polygon.push(*current);
            }
        }
        if polygon.is_empty() {
            break;
        }
    }
    polygon
}

fn overlap_area(a: &Band, b: &Band) -> f64 {
    let (first, second) = (triangles(a), triangles(b));
    first.iter().flat_map(|subject| second.iter().map(move |clipper| (subject, clipper))).map(|(subject, clipper)| area_of(&clip(subject.to_vec(), clipper))).sum()
}

fn area_of(polygon: &[Point]) -> f64 {
    (0..polygon.len()).map(|index| cross(polygon[index] - Point::new(0.0, 0.0), polygon[(index + 1) % polygon.len()] - Point::new(0.0, 0.0))).sum::<f64>() / 2.0
}

fn mark(point: Point) -> Point2 {
    Point2 { x: point.x, y: point.y }
}
//#endregion 🔖️Contacts

//#region 🔖️Joins
/// 🔗️ The joins, end trims and neighbour ids of wall `id` among the `bands` of its storey; `None` when `id` is not a band.
pub fn join(bands: &BTreeMap<String, Band>, id: &str) -> Option<Joined> {
    let me = bands.get(id)?;
    let (mut joins, mut neighbours) = (Vec::new(), BTreeSet::new());
    let mut trim_at = |tip: Tip| {
        let point = me.tip(tip);
        match contact(bands, id, tip) {
            Contact::Free => me.square(tip),
            Contact::Node(members) => {
                for (other, end) in &members {
                    joins.push(WallJoin { kind: JoinKind::Miter, end: tip.join_end(), other: other.clone(), other_end: end.join_end(), point: mark(point), overlap_area: 0.0 });
                    neighbours.insert(other.clone());
                }
                node_trim(bands, id, tip, &members)
            }
            Contact::Butt { through, hit, tangent } => {
                joins.push(WallJoin { kind: JoinKind::Butt, end: tip.join_end(), other: through.clone(), other_end: JoinEnd::Along, point: mark(hit), overlap_area: 0.0 });
                neighbours.insert(through.clone());
                butt_trim(bands, id, tip, &through, hit, tangent)
            }
        }
    };
    let (start, end) = (trim_at(Tip::Start), trim_at(Tip::End));
    for (other, band) in bands.iter().filter(|(other, _)| other.as_str() != id) {
        for tip in Tip::BOTH {
            let point = band.tip(tip);
            if me.axis.closest(point).distance <= JOIN_TOLERANCE {
                if let Contact::Butt { through, hit, .. } = contact(bands, other, tip) {
                    if through == id {
                        joins.push(WallJoin { kind: JoinKind::Through, end: JoinEnd::Along, other: other.clone(), other_end: tip.join_end(), point: mark(hit), overlap_area: 0.0 });
                        neighbours.insert(other.clone());
                    }
                }
            }
        }
        for point in crossings(me, band) {
            joins.push(WallJoin { kind: JoinKind::Cross, end: JoinEnd::Along, other: other.clone(), other_end: JoinEnd::Along, point: mark(point), overlap_area: overlap_area(me, band) });
            neighbours.insert(other.clone());
        }
    }
    Some(Joined { start, end, joins, neighbours })
}

/// 🧱️ The faces of `band` retargeted to the end trims and the footprint loop; `None` when a face collapses.
pub fn footprint(band: &Band, start: Trim, end: Trim) -> Option<Footprint> {
    let left = band.face(Side::Left)?.retarget(start.left, end.left);
    let right = band.face(Side::Right)?.retarget(start.right, end.right);
    Some(Footprint { left, right, outline: [(right.start, right.bulge), (right.end, end.cut_bulge(right.end, left.end)), (left.end, -left.bulge), (left.start, start.cut_bulge(left.start, right.start))] })
}
//#endregion 🔖️Joins

//#region 🔖️Neighbourhood
/// 🤝️ Whether two bands touch: an end of one lies on the axis of the other (within [`JOIN_TOLERANCE`]), or both axes cross in their interiors. The relation is symmetric; every contact [`join`] looks for is a touch.
pub fn touches(a: &Band, b: &Band) -> bool {
    Tip::BOTH.into_iter().any(|tip| b.axis.closest(a.tip(tip)).distance <= JOIN_TOLERANCE) || Tip::BOTH.into_iter().any(|tip| a.axis.closest(b.tip(tip)).distance <= JOIN_TOLERANCE) || !crossings(a, b).is_empty()
}

/// 🤝️ The walls each band touches, by one sweep over the inflated axis bounding boxes along the longer side of the storey and an exact [`touches`] test per candidate pair; `O(n log n + candidates)`, never a scan of the storey per wall.
pub fn touching(bands: &BTreeMap<String, Band>) -> BTreeMap<String, BTreeSet<String>> {
    let boxes: Vec<(&String, &Band, [f64; 4])> = bands
        .iter()
        .map(|(id, band)| {
            let rect = band.axis.bounds();
            (id, band, [rect.x0() - JOIN_TOLERANCE, rect.y0() - JOIN_TOLERANCE, rect.x1() + JOIN_TOLERANCE, rect.y1() + JOIN_TOLERANCE])
        })
        .collect();
    let span = |low: usize, high: usize| boxes.iter().map(|row| row.2[high]).fold(f64::NEG_INFINITY, f64::max) - boxes.iter().map(|row| row.2[low]).fold(f64::INFINITY, f64::min);
    let (lead, cross) = if span(0, 2) >= span(1, 3) { (0, 1) } else { (1, 0) };
    let mut order: Vec<usize> = (0..boxes.len()).collect();
    order.sort_by(|&a, &b| boxes[a].2[lead].total_cmp(&boxes[b].2[lead]));
    let mut found: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (rank, &first) in order.iter().enumerate() {
        for &second in &order[rank + 1..] {
            if boxes[second].2[lead] > boxes[first].2[lead + 2] {
                break;
            }
            if boxes[second].2[cross] > boxes[first].2[cross + 2] || boxes[first].2[cross] > boxes[second].2[cross + 2] {
                continue;
            }
            if touches(boxes[first].1, boxes[second].1) {
                found.entry(boxes[first].0.clone()).or_default().insert(boxes[second].0.clone());
                found.entry(boxes[second].0.clone()).or_default().insert(boxes[first].0.clone());
            }
        }
    }
    found
}

/// 🤝️ The walls within two touches of `id` (`id` itself excluded): everything [`join`] of `id` can depend on. The end of a neighbour butts against the lowest-id wall it touches, so the join of `id` with that neighbour is decided by the walls the neighbour touches.
pub fn neighbourhood(touching: &BTreeMap<String, BTreeSet<String>>, id: &str) -> BTreeSet<String> {
    let first = touching.get(id).cloned().unwrap_or_default();
    let mut all = first.clone();
    for neighbour in &first {
        all.extend(touching.get(neighbour).into_iter().flatten().cloned());
    }
    all.remove(id);
    all
}
//#endregion 🔖️Neighbourhood
