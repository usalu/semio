//! 🗺️ The geometric maps of the modify kernel (translation, rotation, reflection) and their image of each placed element record. A map
//! moves every authored point and turns every authored direction; a reflection also reverses what is handed: the axis of a wall runs the
//! other way so that its interior face stays on its left, a loop runs the other way so that it stays counter-clockwise, the hand of a stair
//! flight swaps and a door keeps its swing. Points and directions are snapped to a picometre so that quarter turns and reflections about an
//! axis-parallel line stay exact.

use crate::mutations::elements::{self, Refusal};
use super::{AlignAxis, AlignEdge};
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::segment_of;
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::loops;
use semio_framework_geometry::{Point, Rect};
use crate::{Axis, Beam, Ceiling, Column, CurtainWall, EndJoin, GridLine, ModelSnapshot, Opening, Point2, Railing, Ramp, Roof, RoofShape, Slab, Slope, Space, SpaceBoundary, Stair, StairFlight, Turn, Vertex, Wall};
use protocol::OutcomeCode;

fn snap(value: f64) -> f64 {
    (value * 1e12).round() / 1e12 + 0.0
}

/// 🗺️ A rigid map of the plan: a translation, a rotation about a pivot or a reflection about the line through two points.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Map {
    Translate(Point2),
    Rotate { pivot: Point2, angle: f64 },
    Mirror { start: Point2, end: Point2 },
}

impl Map {
    /// 🔢️ Whether every number of the map is finite.
    pub fn finite(&self) -> bool {
        match self {
            Map::Translate(vector) => vector.x.is_finite() && vector.y.is_finite(),
            Map::Rotate { pivot, angle } => pivot.x.is_finite() && pivot.y.is_finite() && angle.is_finite(),
            Map::Mirror { start, end } => start.x.is_finite() && start.y.is_finite() && end.x.is_finite() && end.y.is_finite(),
        }
    }

    /// 🪞️ Whether the map is a reflection.
    pub fn mirrors(&self) -> bool {
        matches!(self, Map::Mirror { .. })
    }

    /// 🕳️ Whether the map has no effect or no meaning: a reflection line without length is meaningless, a null translation or turn moves nothing.
    pub fn degenerate(&self) -> bool {
        match self {
            Map::Translate(vector) => vector.x == 0.0 && vector.y == 0.0,
            Map::Rotate { angle, .. } => *angle == 0.0,
            Map::Mirror { start, end } => start == end,
        }
    }

    /// 📍️ The image of a point.
    pub fn point(&self, point: Point2) -> Point2 {
        match self {
            Map::Translate(vector) => Point2 { x: point.x + vector.x, y: point.y + vector.y },
            Map::Rotate { pivot, angle } => {
                let (sin, cos) = angle.sin_cos();
                let (dx, dy) = (point.x - pivot.x, point.y - pivot.y);
                Point2 { x: snap(pivot.x + dx * cos - dy * sin), y: snap(pivot.y + dx * sin + dy * cos) }
            }
            Map::Mirror { start, end } => {
                let (dx, dy) = (end.x - start.x, end.y - start.y);
                let length = dx.hypot(dy);
                let (ux, uy) = (dx / length, dy / length);
                let (vx, vy) = (point.x - start.x, point.y - start.y);
                let along = vx * ux + vy * uy;
                Point2 { x: snap(start.x + 2.0 * along * ux - vx), y: snap(start.y + 2.0 * along * uy - vy) }
            }
        }
    }

    /// 🧭️ The image of a direction given as an angle in radians: unchanged by a translation, turned by a rotation, reflected by a mirror.
    pub fn turn(&self, direction: f64) -> f64 {
        match self {
            Map::Translate(_) => direction,
            Map::Rotate { angle, .. } if *angle == 0.0 => direction,
            Map::Rotate { angle, .. } => snap(direction + angle),
            Map::Mirror { start, end } => snap(2.0 * (end.y - start.y).atan2(end.x - start.x) - direction),
        }
    }
}

//#region 🔖️Records
fn axis_of(line: bool, start: Point2, end: Point2, bulge: f64) -> Axis {
    if line {
        Axis::Line { start, end }
    } else {
        Axis::Arc { start, end, bulge }
    }
}

/// 〰️ The image of an axis. A reflection of a wall (`reversed`) runs the image the other way, which keeps the bulge, so the left and the
/// right face of the image are the images of the left and the right face; a reflection that keeps the direction negates the bulge.
fn axis_image(axis: &Axis, map: &Map, reversed: bool) -> Axis {
    let (line, start, end, bulge) = match axis {
        Axis::Line { start, end } => (true, *start, *end, 0.0),
        Axis::Arc { start, end, bulge } => (false, *start, *end, *bulge),
    };
    let (first, second) = (map.point(start), map.point(end));
    match (map.mirrors(), reversed) {
        (true, true) => axis_of(line, second, first, bulge),
        (true, false) => axis_of(line, first, second, 0.0 - bulge),
        (false, _) => axis_of(line, first, second, bulge),
    }
}

/// ➰️ The image of a closed loop: a reflection runs the image the other way, which keeps it counter-clockwise and keeps every bulge.
pub fn loop_image(vertices: &[Vertex], map: &Map) -> Vec<Vertex> {
    let count = vertices.len();
    if !map.mirrors() {
        return vertices.iter().map(|vertex| Vertex { point: map.point(vertex.point), bulge: vertex.bulge }).collect();
    }
    (0..count).map(|index| Vertex { point: map.point(vertices[(count - index) % count].point), bulge: vertices[(count - 1 - index) % count].bulge }).collect()
}

/// 〰️ The image of an open polyline with bulges: the order stays, a reflection negates every bulge.
fn path_image(vertices: &[Vertex], map: &Map) -> Vec<Vertex> {
    let sign = if map.mirrors() { -1.0 } else { 1.0 };
    vertices.iter().map(|vertex| Vertex { point: map.point(vertex.point), bulge: if vertex.bulge == 0.0 { 0.0 } else { sign * vertex.bulge } }).collect()
}

fn slope_image(slope: &Option<Slope>, map: &Map) -> Option<Slope> {
    slope.map(|slope| Slope { direction: map.turn(slope.direction), angle: slope.angle })
}

fn flight_image(flight: &StairFlight, map: &Map) -> Option<StairFlight> {
    if !map.mirrors() {
        return Some(flight.clone());
    }
    match flight {
        StairFlight::Straight => Some(StairFlight::Straight),
        StairFlight::LTurn { split, turn } => Some(StairFlight::LTurn { split: *split, turn: if *turn == Turn::Left { Turn::Right } else { Turn::Left } }),
        StairFlight::UTurn { .. } => None,
        StairFlight::Spiral { radius, sweep } => Some(StairFlight::Spiral { radius: *radius, sweep: 0.0 - sweep }),
    }
}

fn swapped(map: &Map, first: Option<EndJoin>, second: Option<EndJoin>) -> (Option<EndJoin>, Option<EndJoin>) {
    if map.mirrors() {
        (second, first)
    } else {
        (first, second)
    }
}

/// 🧱️ The image of a wall: its axis, and with a reflection the two end join preferences trade places because the ends do.
pub fn wall(wall: &Wall, map: &Map) -> Wall {
    let (start_join, end_join) = swapped(map, wall.start_join, wall.end_join);
    Wall { axis: axis_image(&wall.axis, map, true), start_join, end_join, ..wall.clone() }
}

/// 🪟️ The image of a curtain wall: its axis keeps its direction, so the grid keeps its origin.
pub fn curtain_wall(wall: &CurtainWall, map: &Map) -> CurtainWall {
    CurtainWall { axis: axis_image(&wall.axis, map, false), ..wall.clone() }
}

/// 🪜️ The image of a stair, none when its flight has a fixed hand that a reflection cannot give (a U-turn always turns to the left).
pub fn stair(stair: &Stair, map: &Map) -> Option<Stair> {
    Some(Stair { start: map.point(stair.start), direction: map.turn(stair.direction), flight: flight_image(&stair.flight, map)?, ..stair.clone() })
}

/// 🚪️ The image of an opening whose host was mapped to `host`: a reflection of a wall runs the wall the other way, so the opening keeps its
/// side, takes the offset from the other end and swaps its hand; a curtain wall keeps its direction, so the offset stays and both flips swap.
pub fn opening(opening: &Opening, host_length: f64, curtain: bool, map: &Map) -> Opening {
    if !map.mirrors() {
        return opening.clone();
    }
    if curtain {
        Opening { flip_hand: !opening.flip_hand, flip_facing: !opening.flip_facing, ..opening.clone() }
    } else {
        Opening { offset: ((host_length - opening.offset) * 1e9).round() / 1e9 + 0.0, flip_hand: !opening.flip_hand, ..opening.clone() }
    }
}
//#endregion 🔖️Records

//#region 🔖️Image
/// 🗺️ The image of one placed element, whole.
#[derive(Clone, Debug, PartialEq)]
pub enum Image {
    Wall(Wall),
    CurtainWall(CurtainWall),
    Column(Column),
    Beam(Beam),
    Slab(Slab),
    Ceiling(Ceiling),
    Roof(Roof),
    Stair(Stair),
    Railing(Railing),
    Ramp(Ramp),
    Space(Space),
    Grid(GridLine),
}

fn refused(id: &str, message: &str) -> Refusal {
    Refusal::new(OutcomeCode::Invariant, message, [id.to_string()])
}

/// 🗺️ The image of the placed element `id` under `map`: unknown ids are `TargetMissing`, elements without a placement (sites, buildings,
/// storeys) and kinds the modify kernel does not know `Invariant`, a stair with a fixed hand under a reflection `Invariant`.
pub fn image(base: &ModelSnapshot, id: &str, map: &Map) -> Result<Image, Refusal> {
    if let Some(row) = base.walls.get(id) {
        return Ok(Image::Wall(wall(row, map)));
    }
    if let Some(row) = base.curtain_walls.get(id) {
        return Ok(Image::CurtainWall(curtain_wall(row, map)));
    }
    if let Some(row) = base.columns.get(id) {
        return Ok(Image::Column(Column { position: map.point(row.position), rotation: map.turn(row.rotation), tilt: slope_image(&row.tilt, map), ..row.clone() }));
    }
    if let Some(row) = base.beams.get(id) {
        return Ok(Image::Beam(Beam { axis: axis_image(&row.axis, map, false), ..row.clone() }));
    }
    if let Some(row) = base.slabs.get(id) {
        return Ok(Image::Slab(Slab { boundary: loop_image(&row.boundary, map), holes: row.holes.iter().map(|hole| loop_image(hole, map)).collect(), slope: slope_image(&row.slope, map), ..row.clone() }));
    }
    if let Some(row) = base.ceilings.get(id) {
        return Ok(Image::Ceiling(Ceiling { boundary: loop_image(&row.boundary, map), holes: row.holes.iter().map(|hole| loop_image(hole, map)).collect(), slope: slope_image(&row.slope, map), ..row.clone() }));
    }
    if let Some(row) = base.roofs.get(id) {
        let shape = match &row.shape {
            RoofShape::Shed { pitch, direction } => RoofShape::Shed { pitch: *pitch, direction: map.turn(*direction) },
            RoofShape::Gable { pitch, ridge_direction } => RoofShape::Gable { pitch: *pitch, ridge_direction: map.turn(*ridge_direction) },
            other => other.clone(),
        };
        return Ok(Image::Roof(Roof { footprint: loop_image(&row.footprint, map), shape, ..row.clone() }));
    }
    if let Some(row) = base.stairs.get(id) {
        return stair(row, map).map(Image::Stair).ok_or_else(|| refused(id, "A U-turn stair always turns to the left and has no mirror image."));
    }
    if let Some(row) = base.railings.get(id) {
        return Ok(Image::Railing(Railing { path: row.path.iter().map(|point| map.point(*point)).collect(), ..row.clone() }));
    }
    if let Some(row) = base.ramps.get(id) {
        return Ok(Image::Ramp(Ramp { path: path_image(&row.path, map), ..row.clone() }));
    }
    if let Some(row) = base.spaces.get(id) {
        let boundary = match &row.boundary {
            SpaceBoundary::Bounded { seed } => SpaceBoundary::Bounded { seed: map.point(*seed) },
            SpaceBoundary::Explicit { outline } => SpaceBoundary::Explicit { outline: loop_image(outline, map) },
        };
        return Ok(Image::Space(Space { boundary, ..row.clone() }));
    }
    if let Some(row) = base.grids.get(id) {
        return Ok(Image::Grid(GridLine { start: map.point(row.start), end: map.point(row.end), ..row.clone() }));
    }
    if elements::exists(base, id) {
        return Err(refused(id, "The element has no placement of its own."));
    }
    Err(Refusal::new(OutcomeCode::TargetMissing, format!("Element \"{id}\" does not exist."), [id.to_string()]))
}

impl Image {
    /// 📍️ The placement fields of the image, the record a move, a rotation and their exact inverse speak.
    pub fn placement(&self) -> elements::Placement {
        use elements::Placement;
        match self {
            Image::Wall(row) => Placement::Wall { axis: row.axis.clone() },
            Image::CurtainWall(row) => Placement::CurtainWall { axis: row.axis.clone() },
            Image::Column(row) => Placement::Column { position: row.position, rotation: row.rotation, tilt: row.tilt },
            Image::Beam(row) => Placement::Beam { axis: row.axis.clone() },
            Image::Slab(row) => Placement::Slab { boundary: row.boundary.clone(), holes: row.holes.clone(), slope: row.slope },
            Image::Ceiling(row) => Placement::Ceiling { boundary: row.boundary.clone(), holes: row.holes.clone(), slope: row.slope },
            Image::Roof(row) => Placement::Roof { footprint: row.footprint.clone(), shape: row.shape.clone() },
            Image::Stair(row) => Placement::Stair { start: row.start, direction: row.direction, flight: Some(row.flight.clone()) },
            Image::Railing(row) => Placement::Railing { path: row.path.clone() },
            Image::Ramp(row) => Placement::Ramp { path: row.path.clone() },
            Image::Space(row) => Placement::Space { boundary: row.boundary.clone() },
            Image::Grid(row) => Placement::Grid { start: row.start, end: row.end },
        }
    }
}
//#endregion 🔖️Image

//#region 🔖️Bounds
fn spanned(points: impl IntoIterator<Item = Point2>) -> Option<[f64; 4]> {
    points.into_iter().fold(None, |found: Option<[f64; 4]>, point| Some(found.map_or([point.x, point.y, point.x, point.y], |at| [at[0].min(point.x), at[1].min(point.y), at[2].max(point.x), at[3].max(point.y)])))
}

fn united(first: Option<[f64; 4]>, second: Option<[f64; 4]>) -> Option<[f64; 4]> {
    match (first, second) {
        (Some(a), Some(b)) => Some([a[0].min(b[0]), a[1].min(b[1]), a[2].max(b[2]), a[3].max(b[3])]),
        (a, b) => a.or(b),
    }
}

fn rectangle(rect: Rect) -> Option<[f64; 4]> {
    Some([rect.x0(), rect.y0(), rect.x1(), rect.y1()])
}

fn loop_bounds(vertices: &[Vertex]) -> Option<[f64; 4]> {
    let inner: Vec<loops::Vertex> = vertices.iter().map(|vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect();
    loops::bounds(&inner).and_then(rectangle)
}

/// 📏️ The extent `[x0, y0, x1, y1]` of the authored plan geometry of a placement, arcs included: the axis of a wall, a beam, the points of a grid line, a railing or a stair start, the position of a column, the outline of a slab, ceiling, roof or space, the path of a
/// ramp. None for an opening, which has no place of its own.
pub fn bounds(placement: &elements::Placement) -> Option<[f64; 4]> {
    use elements::Placement as P;
    match placement {
        P::Wall { axis } | P::CurtainWall { axis } | P::Beam { axis } => rectangle(segment_of(axis).bounds()),
        P::Column { position, .. } => spanned([*position]),
        P::Grid { start, end } => spanned([*start, *end]),
        P::Slab { boundary, .. } | P::Ceiling { boundary, .. } => loop_bounds(boundary),
        P::Roof { footprint, .. } => loop_bounds(footprint),
        P::Stair { start, .. } => spanned([*start]),
        P::Railing { path } => spanned(path.iter().copied()),
        P::Ramp { path } => {
            let steps = path.windows(2).map(|pair| rectangle(BulgeSeg::new(Point::new(pair[0].point.x, pair[0].point.y), Point::new(pair[1].point.x, pair[1].point.y), pair[0].bulge).bounds()));
            steps.fold(spanned(path.iter().map(|vertex| vertex.point)), united)
        }
        P::Space { boundary: SpaceBoundary::Bounded { seed } } => spanned([*seed]),
        P::Space { boundary: SpaceBoundary::Explicit { outline } } => loop_bounds(outline),
        P::Opening { .. } => None,
    }
}

/// 📐️ How far an element with the authored `extent` has to move along `axis` so that its `edge` lies on `target`, snapped to a picometre.
pub fn align_gap(extent: [f64; 4], axis: AlignAxis, edge: AlignEdge, target: f64) -> f64 {
    let (low, high) = if axis == AlignAxis::X { (extent[0], extent[2]) } else { (extent[1], extent[3]) };
    let at = match edge {
        AlignEdge::Min => low,
        AlignEdge::Center => (low + high) / 2.0,
        AlignEdge::Max => high,
    };
    snap(target - at)
}
//#endregion 🔖️Bounds
