//! 🌡️ `energy-envelope`: the thermal envelope of every space and what it adds up to. A space states its conditions (`space_conditions`), a window or door type its U-value, a layer its material conductivity; everything else here is
//! derived and nothing of it is stored.
//!
//! Definitions. Lengths in metres, areas in square metres, angles in degrees, transmittances in watts per square metre and kelvin.
//! * the room of a space (`spaces`) gives its outline; each edge of the outline is classified by probing across it: the wall (or curtain wall) whose footprint the edge lies on, and what lies on the far side of that wall
//!   after its thickness: another room of the storey ([`Boundary::Adjacent`], or [`Boundary::Adiabatic`] when both rooms have the same set points and no heat flows), the same room (an island, no surface) or nothing
//!   ([`Boundary::Exterior`], or [`Boundary::Ground`] below the datum). Consecutive edge pieces of one wall with one far side and a facet angle below ten degrees are one surface. Its area is its length times the clear height of
//!   the room, minus the windows and doors of the wall that stand on it; the windows and doors are surfaces of their own.
//! * the floor is split by the rooms below it (area of the intersection with each), the ceiling by the rooms above it; a part without a room beyond is exterior, and below the lowest storey it is ground (when the storey
//!   stands on the datum). The construction of a floor is the slab of the storey under the part, of a ceiling the slab of the storey above or else the roof of the storey.
//! * azimuth is the compass bearing of the outward normal clockwise from true north (site `true_north`, building `rotation`); tilt follows gbXML: 0 degrees faces up (ceiling, roof), 90 a wall, 180 faces down (floor).
//! * the transmittance of an opaque surface is ISO 6946 over the layer stack of its type (see [`thermal`]), of a window or door the authored `u_value` of its type. A surface that lacks data has no transmittance and an issue.
//! * the aggregates of a scope (a zone, a building, the project) sum its conditioned spaces (a heating or cooling set point): the envelope area `A` counts every surface whose far side is exterior, ground or a space outside
//!   the scope that is not conditioned in it; `H_T = sum(F_x * U * A)` with `F_x` 1.0 (exterior), 0.6 (ground) and 0.5 (another climate); `H'_T = (H_T + dU_WB * A) / A` with the flat thermal bridge allowance `dU_WB = 0.05`
//!   after DIN V 4108-6 and the GEG; `A/V` relates `A` to the net volume of the rooms.
//!
//! The graph has one `Envelope` node per space (parents: the room of its storey and of the storeys above and below, the wall layouts of its storey, the frames of the openings hosted by them) and one `EnergyTotals`
//! node per zone, building and the project (parents: the `Envelope` nodes of the spaces in the scope). They exist only when the model states conditions for at least one space.
//!
//! Related: ISO 6946 <https://www.iso.org/standard/65708.html>, gbXML <https://gbxml.org/schema_doc/7.03/GreenBuildingXML_Ver7.03.html>, GEG <https://www.gesetze-im-internet.de/geg/>.

use super::super::curtain_layout::DEFAULT_MULLION_DEPTH;
use super::super::element_solids::{dep_object, dep_value};
use super::super::opening_frames::{OpeningFrame, Vec3};
use super::super::spaces::{holds, inside, interior_point, obstacles_of, Obstacle, SpaceRoom, SpaceStatus, StoreyRooms};
use super::super::wall_layout::WallLayout;
use crate::{Layer, ModelSnapshot, OpeningKind, SpaceConditions, Vertex};
use semio_framework_2d::booleans::BooleanOperation;
use semio_framework_2d::regions::{region_boolean, Region};
use semio_framework_2d::Vec2;
use semio_framework_geometry::loops;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;
use std::collections::{BTreeMap, BTreeSet};
use thermal::{FarSide, HeatFlow};

#[path = "🧱️thermal/🦀️.rs"]
pub mod thermal;

/// 🗺️ The snapshot collections the envelope and its aggregates read, directly or through the rooms and frames they stand on.
pub const READS: &[&str] = &["spaces", "space_conditions", "zones", "walls", "wall_types", "curtain_walls", "curtain_wall_types", "openings", "window_types", "door_types", "slabs", "slab_types", "roofs", "roof_types", "columns", "column_types", "ceilings", "ceiling_types", "materials", "storeys", "buildings", "sites"];

/// 🌡️ The flat allowance for thermal bridges added to every transmittance of the envelope in `H'_T`, in watts per square metre and kelvin (DIN V 4108-6, GEG).
pub const BRIDGE_ALLOWANCE: f64 = 0.05;
/// 🌍️ The temperature correction factor `F_x` of a surface against the ground.
pub const GROUND_FACTOR: f64 = 0.6;
/// 🏠️ The temperature correction factor `F_x` of a surface against a neighbour with another climate.
pub const NEIGHBOUR_FACTOR: f64 = 0.5;
/// 🧭️ The eight compass sectors of the aggregates, each 45 degrees wide and centred on its bearing, north first.
pub const COMPASS: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];

const PROBE: f64 = 0.002;
const STEP: f64 = 0.5;
const FLATTEN_TOLERANCE: f64 = 0.01;
const FACET_COSINE: f64 = 0.984_807_753_012_208;
const MIN_PART: f64 = 1e-4;

//#region 🔖️Values
/// 🧱️ What a surface is: a wall, a curtain wall, the floor or the ceiling (a roof when its far side is exterior) of a space, or a window or door in a wall.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum SurfaceKind {
    #[default]
    Wall,
    CurtainWall,
    Floor,
    Ceiling,
    Window,
    Door,
}

impl SurfaceKind {
    /// 🏷️ The stable name of the kind.
    pub fn name(self) -> &'static str {
        match self {
            Self::Wall => "wall",
            Self::CurtainWall => "curtain-wall",
            Self::Floor => "floor",
            Self::Ceiling => "ceiling",
            Self::Window => "window",
            Self::Door => "door",
        }
    }
}

/// 🔭️ What lies on the far side of a surface.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum Boundary {
    #[default]
    Exterior,
    Ground,
    Adjacent,
    Adiabatic,
}

impl Boundary {
    /// 🏷️ The stable name of the boundary condition.
    pub fn name(self) -> &'static str {
        match self {
            Self::Exterior => "exterior",
            Self::Ground => "ground",
            Self::Adjacent => "adjacent",
            Self::Adiabatic => "adiabatic",
        }
    }
}

/// 🧱️ One surface of the envelope of a space. `area` is net of the openings in it; `polygon` is its outline in building coordinates, counter-clockwise seen from outside.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct EnvelopeSurface {
    pub id: String,
    pub kind: SurfaceKind,
    pub element: String,
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub holder: String,
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub construction: String,
    pub boundary: Boundary,
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub adjacent: String,
    pub area: f64,
    pub gross_area: f64,
    pub azimuth: f64,
    pub tilt: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub u_value: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub g_value: Option<f64>,
    pub frame_fraction: f64,
    pub polygon: Vec<Vec3>,
}

impl EnvelopeSurface {
    /// ☀️ The compass sector (index into [`COMPASS`]) the surface faces.
    pub fn sector(&self) -> usize {
        (((self.azimuth + 22.5).rem_euclid(360.0)) / 45.0).floor() as usize % 8
    }

    /// 🧱️ Whether the surface is vertical (a wall, curtain wall, window or door).
    pub fn vertical(&self) -> bool {
        matches!(self.kind, SurfaceKind::Wall | SurfaceKind::CurtainWall | SurfaceKind::Window | SurfaceKind::Door)
    }
}

/// 🩺️ What an envelope issue is about.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
pub enum EnergyCode {
    ThermalDataMissing,
    OpenBoundary,
    ConditionsMissing,
}

/// 🩺️ One issue of an envelope: the code, the element it is about and the detail (the material, type or length that explains it).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct EnergyIssue {
    pub code: EnergyCode,
    pub element: String,
    pub detail: String,
}

/// 🌡️ The envelope of one space. `conditioned` is a heating or cooling set point, `heated` a heating set point; `open_length` is the length of its outline that lies on no wall.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct EnvelopeSpace {
    pub space: String,
    pub storey: String,
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub zone: String,
    pub conditioned: bool,
    pub heated: bool,
    pub floor_area: f64,
    pub volume: f64,
    pub height: f64,
    pub open_length: f64,
    pub surfaces: Vec<EnvelopeSurface>,
    pub issues: Vec<EnergyIssue>,
}

/// 🎯️ The scope an aggregate adds up.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
pub enum EnergyScope {
    Zone(String),
    Building(String),
    Project,
}

impl EnergyScope {
    /// 🔑️ The key of the scope in the inference: `zone:<id>`, `building:<id>` or `project`.
    pub fn key(&self) -> String {
        match self {
            Self::Zone(id) => format!("zone:{id}"),
            Self::Building(id) => format!("building:{id}"),
            Self::Project => "project".to_string(),
        }
    }
}

/// 📊️ What the conditioned spaces of a scope add up to. The vectors have one entry per compass sector ([`COMPASS`]).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct EnergyTotals {
    pub spaces: Vec<String>,
    pub floor_area: f64,
    pub volume: f64,
    pub envelope_area: f64,
    pub a_over_v: f64,
    pub transmission: f64,
    pub h_t_prime: f64,
    pub opaque_by_sector: Vec<f64>,
    pub window_by_sector: Vec<f64>,
    pub solar_by_sector: Vec<f64>,
    pub roof_area: f64,
    pub floor_envelope_area: f64,
    pub glazing_area: f64,
    pub glazing_ratio: f64,
    pub solar_aperture: f64,
    pub mean_u_opaque: f64,
    pub mean_u_window: f64,
    pub missing: u32,
}

/// 🌡️ The climate of every space the aggregates need: whether it is conditioned and its zone and building.
#[derive(Clone, Debug, Default)]
pub struct Climate {
    pub conditioned: BTreeMap<String, bool>,
    pub zone: BTreeMap<String, String>,
    pub building: BTreeMap<String, String>,
}

impl Climate {
    /// 🌡️ The climate of the spaces of a snapshot.
    pub fn of(snapshot: &ModelSnapshot) -> Self {
        let mut climate = Self::default();
        for (id, space) in &snapshot.spaces {
            climate.conditioned.insert(id.clone(), snapshot.space_conditions.get(id).is_some_and(SpaceConditions::conditioned));
            if let Some(zone) = &space.zone {
                climate.zone.insert(id.clone(), zone.clone());
            }
            if let Some(building) = snapshot.storeys.get(&space.storey).map(|storey| storey.building.clone()) {
                climate.building.insert(id.clone(), building);
            }
        }
        climate
    }

    /// 🎯️ Whether the space is in the scope.
    pub fn in_scope(&self, scope: &EnergyScope, space: &str) -> bool {
        match scope {
            EnergyScope::Zone(zone) => self.zone.get(space).is_some_and(|own| own == zone),
            EnergyScope::Building(building) => self.building.get(space).is_some_and(|own| own == building),
            EnergyScope::Project => true,
        }
    }
}
//#endregion 🔖️Values

impl SpaceConditions {
    /// 🌡️ Whether the space is heated or cooled: it states a set point.
    pub fn conditioned(&self) -> bool {
        self.heating_setpoint.is_some() || self.cooling_setpoint.is_some()
    }

    fn same_climate(&self, other: &SpaceConditions) -> bool {
        self.heating_setpoint == other.heating_setpoint && self.cooling_setpoint == other.cooling_setpoint
    }
}

//#region 🔖️Geometry
fn ring_of(vertices: &[Vertex]) -> Vec<Vec2> {
    let geometry: Vec<loops::Vertex> = vertices.iter().map(|vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect();
    loops::flatten(&geometry, FLATTEN_TOLERANCE).iter().map(|point| [point.x, point.y]).collect()
}

fn signed_area(ring: &[Vec2]) -> f64 {
    (0..ring.len()).map(|index| (ring[index][0] * ring[(index + 1) % ring.len()][1] - ring[(index + 1) % ring.len()][0] * ring[index][1]) / 2.0).sum()
}

fn oriented(mut ring: Vec<Vec2>, counter_clockwise: bool) -> Vec<Vec2> {
    if (signed_area(&ring) > 0.0) != counter_clockwise {
        ring.reverse();
    }
    ring
}

/// 🗺️ The region of a resolved room, `None` while the room has no outline.
pub fn region_of(room: &SpaceRoom) -> Option<Region> {
    let outer = ring_of(&room.outline);
    (outer.len() >= 3 && matches!(room.status, SpaceStatus::Inferred | SpaceStatus::Explicit)).then(|| Region::new(&outer, &room.holes.iter().map(|hole| ring_of(hole)).filter(|hole| hole.len() >= 3).collect::<Vec<_>>()))
}

fn polygon_at(ring: &[Vec2], z: f64) -> Vec<Vec3> {
    ring.iter().map(|point| Vec3 { x: point[0], y: point[1], z }).collect()
}

fn quad(a: Vec2, b: Vec2, z0: f64, z1: f64) -> Vec<Vec3> {
    vec![Vec3 { x: a[0], y: a[1], z: z0 }, Vec3 { x: b[0], y: b[1], z: z0 }, Vec3 { x: b[0], y: b[1], z: z1 }, Vec3 { x: a[0], y: a[1], z: z1 }]
}

fn distance(a: Vec2, b: Vec2) -> f64 {
    (b[0] - a[0]).hypot(b[1] - a[1])
}

fn along(a: Vec2, b: Vec2, t: f64) -> Vec2 {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]
}

/// 🧭️ The storey directly above (or below) `storey` in its building: the nearest level above (below).
pub fn neighbour_storey(snapshot: &ModelSnapshot, storey: &str, up: bool) -> Option<String> {
    let own = snapshot.storeys.get(storey)?;
    let others = snapshot.storeys.iter().filter(|(id, other)| other.building == own.building && id.as_str() != storey);
    if up {
        others.filter(|(_, other)| other.level > own.level).min_by(|a, b| a.1.level.cmp(&b.1.level).then_with(|| a.0.cmp(b.0))).map(|(id, _)| id.clone())
    } else {
        others.filter(|(_, other)| other.level < own.level).max_by(|a, b| a.1.level.cmp(&b.1.level).then_with(|| b.0.cmp(a.0))).map(|(id, _)| id.clone())
    }
}
//#endregion 🔖️Geometry

//#region 🔖️Construction
fn stack(snapshot: &ModelSnapshot, layers: &[Layer]) -> Result<Vec<(f64, f64)>, String> {
    layers.iter().map(|layer| snapshot.materials.get(&layer.material).map(|material| (layer.thickness, material.conductivity)).ok_or_else(|| layer.material.clone())).collect()
}

fn construction(snapshot: &ModelSnapshot, layers: Option<&[Layer]>, flow: HeatFlow, far: FarSide, label: &str) -> Result<f64, String> {
    let Some(layers) = layers else { return Err(label.to_string()) };
    let numbers = stack(snapshot, layers)?;
    thermal::transmittance(&numbers, flow, far).ok_or_else(|| layers.iter().zip(&numbers).find(|(_, (thickness, conductivity))| !(*thickness > 0.0 && *conductivity > 0.0)).map_or_else(|| label.to_string(), |(layer, _)| layer.material.clone()))
}

fn far_side(boundary: Boundary) -> FarSide {
    match boundary {
        Boundary::Exterior => FarSide::Outdoor,
        Boundary::Ground => FarSide::Ground,
        Boundary::Adjacent | Boundary::Adiabatic => FarSide::Room,
    }
}

fn contains(boundary: &[crate::Vertex], holes: &[Vec<crate::Vertex>], x: f64, y: f64) -> bool {
    let plan = |vertices: &[crate::Vertex]| -> Vec<loops::Vertex> { vertices.iter().map(|vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect() };
    let probe = Point::new(x, y);
    loops::contains(&plan(boundary), probe) && !holes.iter().any(|hole| loops::contains(&plan(hole), probe))
}

/// 🧱️ The construction that lies under a point: the id of the element, the id of its type and the layers of the type.
type Construction<'a> = (&'a str, &'a str, &'a [Layer]);

fn slab_layers<'a>(snapshot: &'a ModelSnapshot, storey: &str, at: Vec2) -> Option<Construction<'a>> {
    snapshot
        .slabs
        .iter()
        .filter(|(_, slab)| slab.storey == storey && contains(&slab.boundary, &slab.holes, at[0], at[1]))
        .filter_map(|(id, slab)| snapshot.slab_types.get(&slab.slab_type).map(|kind| (id.as_str(), slab.slab_type.as_str(), kind.layers.as_slice())))
        .max_by(|left, right| left.2.iter().map(|layer| layer.thickness).sum::<f64>().total_cmp(&right.2.iter().map(|layer| layer.thickness).sum::<f64>()).then_with(|| right.0.cmp(left.0)))
}

fn roof_layers<'a>(snapshot: &'a ModelSnapshot, storey: &str, at: Vec2) -> Option<Construction<'a>> {
    snapshot
        .roofs
        .iter()
        .filter(|(_, roof)| roof.storey == storey && contains(&roof.footprint, &[], at[0], at[1]))
        .filter_map(|(id, roof)| snapshot.roof_types.get(&roof.roof_type).map(|kind| (id.as_str(), roof.roof_type.as_str(), kind.layers.as_slice())))
        .min_by(|left, right| left.0.cmp(right.0))
}
//#endregion 🔖️Construction

//#region 🔖️Envelope
/// 🧩️ The values of the parents an envelope is computed from.
pub struct Inputs<'a> {
    pub rooms: &'a BTreeMap<&'a str, &'a StoreyRooms>,
    pub layouts: &'a BTreeMap<&'a str, &'a WallLayout>,
    pub frames: &'a BTreeMap<&'a str, &'a OpeningFrame>,
}

#[derive(Clone, Debug, PartialEq)]
enum Side {
    Space(String),
    Same,
    Open,
}

#[derive(Clone, Debug, PartialEq)]
struct Class {
    wall: Option<usize>,
    side: Side,
}

struct Piece {
    start: Vec2,
    end: Vec2,
    length: f64,
    normal: Vec2,
    class: Class,
}

struct Context<'a> {
    snapshot: &'a ModelSnapshot,
    inputs: &'a Inputs<'a>,
    space: &'a str,
    regions: Vec<(&'a str, Region)>,
    obstacles: Vec<Obstacle>,
    conditions: Option<&'a SpaceConditions>,
    bearing: f64,
    floor_z: f64,
    height: f64,
}

impl Context<'_> {
    fn thickness(&self, element: &str) -> f64 {
        self.inputs.layouts.get(element).map_or(DEFAULT_MULLION_DEPTH, |layout| layout.thickness)
    }

    fn classify(&self, point: Vec2, normal: Vec2) -> Class {
        let near = [point[0] + normal[0] * PROBE, point[1] + normal[1] * PROBE];
        let Some(wall) = self.obstacles.iter().position(|obstacle| near[0] >= obstacle.bounds[0] && near[0] <= obstacle.bounds[2] && near[1] >= obstacle.bounds[1] && near[1] <= obstacle.bounds[3] && inside(&obstacle.ring, near)) else {
            return Class { wall: None, side: Side::Open };
        };
        let depth = self.thickness(&self.obstacles[wall].id) + PROBE;
        let far = [point[0] + normal[0] * depth, point[1] + normal[1] * depth];
        let side = self.regions.iter().find(|(_, region)| holds(region, far)).map_or(Side::Open, |(id, _)| if *id == self.space { Side::Same } else { Side::Space((*id).to_string()) });
        Class { wall: Some(wall), side }
    }

    fn transition(&self, a: Vec2, b: Vec2, normal: Vec2, mut low: f64, mut high: f64, expected: &Class) -> f64 {
        for _ in 0..30 {
            let middle = (low + high) / 2.0;
            if &self.classify(along(a, b, middle), normal) == expected {
                low = middle;
            } else {
                high = middle;
            }
        }
        (low + high) / 2.0
    }

    fn pieces(&self, ring: &[Vec2]) -> Vec<Piece> {
        let mut found: Vec<Piece> = Vec::new();
        for index in 0..ring.len() {
            let (a, b) = (ring[index], ring[(index + 1) % ring.len()]);
            let length = distance(a, b);
            if length < 1e-6 {
                continue;
            }
            let normal = [(b[1] - a[1]) / length, -(b[0] - a[0]) / length];
            let count = ((length / STEP).ceil() as usize).max(1);
            let classes: Vec<Class> = (0..count).map(|sample| self.classify(along(a, b, (sample as f64 + 0.5) / count as f64), normal)).collect();
            let mut start = 0.0;
            let mut class = classes[0].clone();
            let mut runs: Vec<(f64, f64, Class)> = Vec::new();
            for sample in 1..count {
                if classes[sample] != classes[sample - 1] {
                    let cut = self.transition(a, b, normal, (sample as f64 - 0.5) / count as f64, (sample as f64 + 0.5) / count as f64, &classes[sample - 1]);
                    runs.push((start, cut, class));
                    start = cut;
                    class = classes[sample].clone();
                }
            }
            runs.push((start, 1.0, class));
            for (from, to, class) in runs {
                let (start, end) = (along(a, b, from), along(a, b, to));
                let piece = Piece { start, end, length: length * (to - from), normal, class };
                match found.last_mut() {
                    Some(last) if last.class == piece.class && distance(last.end, piece.start) < 1e-6 && last.normal[0] * piece.normal[0] + last.normal[1] * piece.normal[1] >= FACET_COSINE => {
                        let total = last.length + piece.length;
                        let sum = [last.normal[0] * last.length + piece.normal[0] * piece.length, last.normal[1] * last.length + piece.normal[1] * piece.length];
                        let norm = sum[0].hypot(sum[1]).max(1e-12);
                        last.end = piece.end;
                        last.normal = [sum[0] / norm, sum[1] / norm];
                        last.length = total;
                    }
                    _ => found.push(piece),
                }
            }
        }
        found
    }

    fn azimuth(&self, normal: Vec2) -> f64 {
        (normal[0].atan2(normal[1]) + self.bearing).to_degrees().rem_euclid(360.0)
    }

    fn boundary(&self, side: &Side, horizontal_ground: bool) -> (Boundary, String) {
        match side {
            Side::Space(other) => {
                let theirs = self.snapshot.space_conditions.get(other);
                let flows = match (self.conditions, theirs) {
                    (Some(own), Some(theirs)) => !own.same_climate(theirs),
                    (None, None) => false,
                    _ => true,
                };
                (if flows { Boundary::Adjacent } else { Boundary::Adiabatic }, other.clone())
            }
            _ => (if horizontal_ground { Boundary::Ground } else { Boundary::Exterior }, String::new()),
        }
    }
}

fn opening_surface(context: &Context<'_>, id: &str, frame: &OpeningFrame, wall: &str, parent: &str, boundary: Boundary, adjacent: &str, piece: &Piece, order: usize, issues: &mut Vec<EnergyIssue>) -> Option<EnvelopeSurface> {
    let opening = context.snapshot.openings.get(id)?;
    let (kind, type_id, u_value, g_value, fraction) = match &opening.kind {
        OpeningKind::Window { window_type } => {
            let record = context.snapshot.window_types.get(window_type);
            (SurfaceKind::Window, window_type.as_str(), record.and_then(|record| record.u_value), record.and_then(|record| record.g_value), record.and_then(|record| record.frame_fraction).unwrap_or(0.0))
        }
        OpeningKind::Door { door_type } => (SurfaceKind::Door, door_type.as_str(), context.snapshot.door_types.get(door_type).and_then(|record| record.u_value), None, 0.0),
        OpeningKind::Void { .. } => return None,
    };
    if u_value.is_none() {
        issues.push(EnergyIssue { code: EnergyCode::ThermalDataMissing, element: id.to_string(), detail: wall.to_string() });
    }
    let direction = [(piece.end[0] - piece.start[0]) / piece.length.max(1e-12), (piece.end[1] - piece.start[1]) / piece.length.max(1e-12)];
    let reach = (frame.point.x - piece.start[0]) * direction[0] + (frame.point.y - piece.start[1]) * direction[1];
    let centre = [piece.start[0] + direction[0] * reach, piece.start[1] + direction[1] * reach];
    let half = frame.width / 2.0;
    let (a, b) = ([centre[0] - direction[0] * half, centre[1] - direction[1] * half], [centre[0] + direction[0] * half, centre[1] + direction[1] * half]);
    Some(EnvelopeSurface {
        id: format!("{}/o{}", context.space, order),
        kind,
        element: id.to_string(),
        parent: parent.to_string(),
        holder: id.to_string(),
        construction: type_id.to_string(),
        boundary,
        adjacent: adjacent.to_string(),
        area: frame.width * frame.height,
        gross_area: frame.width * frame.height,
        azimuth: context.azimuth(piece.normal),
        tilt: 90.0,
        u_value,
        g_value,
        frame_fraction: fraction,
        polygon: quad(a, b, frame.local.origin.z, frame.local.origin.z + frame.height),
    })
}

fn hosted<'a>(context: &Context<'a>, wall: &str, piece: &Piece, thickness: f64) -> Vec<(&'a str, &'a OpeningFrame)> {
    let direction = [(piece.end[0] - piece.start[0]) / piece.length.max(1e-12), (piece.end[1] - piece.start[1]) / piece.length.max(1e-12)];
    context
        .snapshot
        .openings
        .iter()
        .filter(|(_, opening)| opening.host == wall)
        .filter_map(|(id, _)| context.inputs.frames.get(id.as_str()).map(|frame| (id.as_str(), *frame)))
        .filter(|(_, frame)| frame.valid)
        .filter(|(_, frame)| {
            let (dx, dy) = (frame.point.x - piece.start[0], frame.point.y - piece.start[1]);
            let reach = dx * direction[0] + dy * direction[1];
            let off = (dx * direction[1] - dy * direction[0]).abs();
            reach >= -1e-9 && reach <= piece.length + 1e-9 && off <= thickness + 0.01
        })
        .collect()
}

fn wall_surfaces(context: &Context<'_>, room: &SpaceRoom, rings: &[Vec<Vec2>], out: &mut EnvelopeSpace) {
    let (mut walls, mut openings) = (0, 0);
    for ring in rings {
        for piece in context.pieces(ring) {
            let Some(index) = piece.class.wall else {
                out.open_length += piece.length;
                continue;
            };
            if piece.class.side == Side::Same {
                continue;
            }
            let element = context.obstacles[index].id.as_str();
            let is_wall = context.snapshot.walls.contains_key(element);
            let (boundary, adjacent) = context.boundary(&piece.class.side, false);
            let grade = (-room.floor_z).clamp(0.0, context.height);
            let split = boundary == Boundary::Exterior && grade > 1e-9;
            let thickness = context.thickness(element);
            let inside_openings = if is_wall { hosted(context, element, &piece, thickness) } else { Vec::new() };
            let cut: f64 = inside_openings.iter().map(|(_, frame)| frame.width * frame.height).sum();
            let curtain = context.snapshot.curtain_walls.get(element).and_then(|curtain| context.snapshot.curtain_wall_types.get(&curtain.curtain_wall_type).map(|kind| (curtain.curtain_wall_type.as_str(), kind)));
            let wall_type = context.snapshot.walls.get(element).map_or("", |wall| wall.wall_type.as_str());
            let wall_u = |boundary: Boundary| -> Option<f64> {
                if !is_wall {
                    return curtain.and_then(|(_, kind)| kind.u_value);
                }
                let layers = context.snapshot.wall_types.get(wall_type).map(|kind| kind.layers.as_slice());
                construction(context.snapshot, layers, HeatFlow::Horizontal, far_side(boundary), element).ok()
            };
            let azimuth = context.azimuth(piece.normal);
            walls += 1;
            let id = format!("{}/w{}", context.space, walls);
            let parts: Vec<(Boundary, f64, f64, f64, String)> = if split {
                vec![(Boundary::Ground, grade, context.floor_z, context.floor_z + grade, format!("{id}g")), (Boundary::Exterior, context.height - grade, context.floor_z + grade, context.floor_z + context.height, id.clone())]
            } else {
                vec![(boundary, context.height, context.floor_z, context.floor_z + context.height, id.clone())]
            };
            for (part_boundary, part_height, z0, z1, part_id) in parts.into_iter().filter(|part| part.1 > 1e-9) {
                let gross = piece.length * part_height;
                let net = if part_boundary == Boundary::Ground { gross } else { (gross - cut).max(0.0) };
                let u_value = wall_u(part_boundary);
                if u_value.is_none() {
                    let detail = if is_wall { wall_type.to_string() } else { curtain.map_or_else(|| "curtain wall".to_string(), |(kind, _)| kind.to_string()) };
                    out.issues.push(EnergyIssue { code: EnergyCode::ThermalDataMissing, element: element.to_string(), detail });
                }
                out.surfaces.push(EnvelopeSurface {
                    id: part_id.clone(),
                    kind: if is_wall { SurfaceKind::Wall } else { SurfaceKind::CurtainWall },
                    element: element.to_string(),
                    parent: String::new(),
                    holder: element.to_string(),
                    construction: if is_wall { wall_type.to_string() } else { curtain.map_or_else(String::new, |(kind, _)| kind.to_string()) },
                    boundary: part_boundary,
                    adjacent: adjacent.clone(),
                    area: net,
                    gross_area: gross,
                    azimuth,
                    tilt: 90.0,
                    u_value,
                    g_value: curtain.and_then(|(_, kind)| kind.g_value),
                    frame_fraction: curtain.and_then(|(_, kind)| kind.frame_fraction).unwrap_or(0.0),
                    polygon: quad(piece.start, piece.end, z0, z1),
                });
                if part_boundary == Boundary::Exterior || !split {
                    for (opening, frame) in &inside_openings {
                        openings += 1;
                        if let Some(surface) = opening_surface(context, opening, frame, element, &part_id, part_boundary, &adjacent, &piece, openings, &mut out.issues) {
                            out.surfaces.push(surface);
                        }
                    }
                }
            }
        }
    }
}

fn horizontal_surfaces(context: &Context<'_>, room: &SpaceRoom, own: &Region, storey: &str, up: bool, out: &mut EnvelopeSpace) {
    let neighbour = neighbour_storey(context.snapshot, storey, up);
    let others: Vec<(&str, Region)> = neighbour
        .as_deref()
        .and_then(|other| context.inputs.rooms.get(other))
        .map(|rooms| rooms.iter().filter_map(|(id, room)| region_of(room).map(|region| (id.as_str(), region))).collect())
        .unwrap_or_default();
    let control = &mut |_: &semio_framework_2d::booleans::BooleanProgress| true;
    let mut parts: Vec<(Option<&str>, f64, Vec<Vec2>, Vec2)> = Vec::new();
    for (id, region) in &others {
        let overlap = region_boolean(BooleanOperation::Intersection, std::slice::from_ref(own), std::slice::from_ref(region), control).unwrap_or_default();
        let area: f64 = overlap.iter().map(Region::area).sum();
        if area > MIN_PART {
            if let Some(largest) = overlap.iter().max_by(|a, b| a.area().total_cmp(&b.area())) {
                let at = interior_point(largest).map_or([room.point.x, room.point.y], |point| [point.x, point.y]);
                parts.push((Some(id), area, largest.outer.clone(), at));
            }
        }
    }
    let rest = own.area() - parts.iter().map(|part| part.1).sum::<f64>();
    if rest > MIN_PART || parts.is_empty() {
        parts.push((None, rest.max(0.0), own.outer.clone(), [room.point.x, room.point.y]));
    }
    let on_datum = room.floor_z <= 1e-9;
    let z = if up { room.floor_z + room.clear_height } else { room.floor_z };
    for (index, (other, area, outline, at)) in parts.into_iter().enumerate() {
        if area <= 0.0 && other.is_none() {
            continue;
        }
        let side = other.map_or(Side::Open, |id| Side::Space(id.to_string()));
        let (boundary, adjacent) = context.boundary(&side, !up && neighbour.is_none() && on_datum);
        let flow = if up { HeatFlow::Up } else { HeatFlow::Down };
        let found = if up { neighbour.as_deref().and_then(|above| slab_layers(context.snapshot, above, at)).or_else(|| roof_layers(context.snapshot, storey, at)) } else { slab_layers(context.snapshot, storey, at) };
        let label = if up { "ceiling" } else { "floor" };
        let u_value = construction(context.snapshot, found.map(|(_, _, layers)| layers), flow, far_side(boundary), label).ok();
        let (holder, kind) = found.map_or((String::new(), String::new()), |(holder, kind, _)| (holder.to_string(), kind.to_string()));
        let element = if holder.is_empty() { format!("{storey}/{label}") } else { holder.clone() };
        if u_value.is_none() {
            out.issues.push(EnergyIssue { code: EnergyCode::ThermalDataMissing, element: context.space.to_string(), detail: label.to_string() });
        }
        let mut polygon = polygon_at(&oriented(outline, true), z);
        if !up {
            polygon.reverse();
        }
        out.surfaces.push(EnvelopeSurface {
            id: format!("{}/{}{}", context.space, if up { "c" } else { "f" }, index + 1),
            kind: if up { SurfaceKind::Ceiling } else { SurfaceKind::Floor },
            element,
            parent: String::new(),
            holder,
            construction: kind,
            boundary,
            adjacent,
            area,
            gross_area: area,
            azimuth: 0.0,
            tilt: if up { 0.0 } else { 180.0 },
            u_value,
            g_value: None,
            frame_fraction: 0.0,
            polygon,
        });
    }
}

/// 🌡️ The envelope of one space from the values of its parents; a space without a resolved room has no surfaces.
pub fn envelope_of(snapshot: &ModelSnapshot, id: &str, inputs: &Inputs<'_>) -> EnvelopeSpace {
    let Some(space) = snapshot.spaces.get(id) else { return EnvelopeSpace::default() };
    let conditions = snapshot.space_conditions.get(id);
    let mut out = EnvelopeSpace { space: id.to_string(), storey: space.storey.clone(), zone: space.zone.clone().unwrap_or_default(), conditioned: conditions.is_some_and(SpaceConditions::conditioned), heated: conditions.is_some_and(SpaceConditions::heated), ..EnvelopeSpace::default() };
    if conditions.is_none() {
        out.issues.push(EnergyIssue { code: EnergyCode::ConditionsMissing, element: id.to_string(), detail: String::new() });
    }
    let Some(room) = inputs.rooms.get(space.storey.as_str()).and_then(|rooms| rooms.get(id)) else { return out };
    let Some(own) = region_of(room) else { return out };
    out.floor_area = room.area;
    out.volume = room.volume;
    out.height = room.clear_height;
    let building = snapshot.storeys.get(&space.storey).and_then(|storey| snapshot.buildings.get(&storey.building));
    let north = building.and_then(|building| snapshot.sites.get(&building.site)).map_or(0.0, |site| site.true_north);
    let regions: Vec<(&str, Region)> = inputs.rooms.get(space.storey.as_str()).into_iter().flat_map(|rooms| rooms.iter()).filter_map(|(other, room)| region_of(room).map(|region| (other.as_str(), region))).collect();
    let context = Context {
        snapshot,
        inputs,
        space: id,
        regions,
        obstacles: obstacles_of(snapshot, &space.storey, inputs.layouts),
        conditions,
        bearing: north - building.map_or(0.0, |building| building.rotation),
        floor_z: room.floor_z,
        height: room.clear_height,
    };
    let mut rings = vec![oriented(own.outer.clone(), true)];
    rings.extend(own.holes.iter().cloned().map(|hole| oriented(hole, false)));
    wall_surfaces(&context, room, &rings, &mut out);
    horizontal_surfaces(&context, room, &own, &space.storey, false, &mut out);
    horizontal_surfaces(&context, room, &own, &space.storey, true, &mut out);
    if out.open_length > 1e-6 {
        out.issues.push(EnergyIssue { code: EnergyCode::OpenBoundary, element: id.to_string(), detail: format!("{:.3}", out.open_length) });
    }
    out.issues.sort_by(|a, b| (a.code, &a.element, &a.detail).cmp(&(b.code, &b.element, &b.detail)));
    out.issues.dedup();
    out
}
//#endregion 🔖️Envelope

//#region 🔖️Totals
/// 📊️ What the conditioned spaces of `scope` add up to, from their envelopes.
pub fn totals_of(scope: &EnergyScope, envelopes: &[&EnvelopeSpace], climate: &Climate) -> EnergyTotals {
    let mut totals = EnergyTotals { opaque_by_sector: vec![0.0; 8], window_by_sector: vec![0.0; 8], solar_by_sector: vec![0.0; 8], ..EnergyTotals::default() };
    let (mut opaque_u, mut opaque_area, mut window_u, mut window_area) = (0.0, 0.0, 0.0, 0.0);
    let mut members: Vec<&EnvelopeSpace> = envelopes.iter().copied().filter(|envelope| envelope.conditioned && climate.in_scope(scope, &envelope.space)).collect();
    members.sort_by(|a, b| a.space.cmp(&b.space));
    for envelope in &members {
        totals.spaces.push(envelope.space.clone());
        totals.floor_area += envelope.floor_area;
        totals.volume += envelope.volume;
        for surface in &envelope.surfaces {
            let factor = match surface.boundary {
                Boundary::Exterior => 1.0,
                Boundary::Ground => GROUND_FACTOR,
                Boundary::Adiabatic => continue,
                Boundary::Adjacent => {
                    if climate.conditioned.get(&surface.adjacent).copied().unwrap_or(false) && climate.in_scope(scope, &surface.adjacent) {
                        continue;
                    }
                    NEIGHBOUR_FACTOR
                }
            };
            totals.envelope_area += surface.area;
            match surface.u_value {
                Some(u_value) => totals.transmission += factor * u_value * surface.area,
                None => totals.missing += 1,
            }
            if surface.vertical() {
                let sector = surface.sector();
                if matches!(surface.kind, SurfaceKind::Window | SurfaceKind::CurtainWall) {
                    totals.window_by_sector[sector] += surface.area;
                    let aperture = surface.g_value.unwrap_or(0.0) * (1.0 - surface.frame_fraction) * surface.area;
                    totals.solar_by_sector[sector] += aperture;
                    totals.solar_aperture += aperture;
                    totals.glazing_area += surface.area;
                    if let Some(u_value) = surface.u_value {
                        window_u += u_value * surface.area;
                        window_area += surface.area;
                    }
                } else {
                    totals.opaque_by_sector[sector] += surface.area;
                    if let Some(u_value) = surface.u_value {
                        opaque_u += u_value * surface.area;
                        opaque_area += surface.area;
                    }
                }
            } else if matches!(surface.kind, SurfaceKind::Ceiling) {
                totals.roof_area += surface.area;
                if let Some(u_value) = surface.u_value {
                    opaque_u += u_value * surface.area;
                    opaque_area += surface.area;
                }
            } else {
                totals.floor_envelope_area += surface.area;
                if let Some(u_value) = surface.u_value {
                    opaque_u += u_value * surface.area;
                    opaque_area += surface.area;
                }
            }
        }
    }
    let facade = totals.opaque_by_sector.iter().sum::<f64>() + totals.glazing_area;
    totals.glazing_ratio = if facade > 0.0 { totals.glazing_area / facade } else { 0.0 };
    totals.a_over_v = if totals.volume > 0.0 { totals.envelope_area / totals.volume } else { 0.0 };
    totals.h_t_prime = if totals.envelope_area > 0.0 { (totals.transmission + BRIDGE_ALLOWANCE * totals.envelope_area) / totals.envelope_area } else { 0.0 };
    totals.mean_u_opaque = if opaque_area > 0.0 { opaque_u / opaque_area } else { 0.0 };
    totals.mean_u_window = if window_area > 0.0 { window_u / window_area } else { 0.0 };
    totals
}
//#endregion 🔖️Totals

//#region 🔖️Dependency
fn layers_dependency(snapshot: &ModelSnapshot, layers: &[Layer]) -> DslValue {
    DslValue::object(layers.iter().enumerate().map(|(index, layer)| (format!("{index:04}"), dep_object([("material", dep_value(&layer.material)), ("thickness", dep_value(&layer.thickness)), ("conductivity", dep_value(&snapshot.materials.get(&layer.material).map(|material| material.conductivity)))]))))
}

/// 🔑️ Everything `envelope_of` reads of the snapshot for a space apart from its parents: the conditions of the spaces of its storey and of the storeys beside it, the constructions of the walls, slabs and roofs it can lie on,
/// the thermal data of the windows and doors in those walls, and the building placement and the true north.
pub fn dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let Some(space) = snapshot.spaces.get(id) else { return DslValue::Null };
    let storey = space.storey.as_str();
    let above = neighbour_storey(snapshot, storey, true);
    let below = neighbour_storey(snapshot, storey, false);
    let near: BTreeSet<&str> = [Some(storey), above.as_deref(), below.as_deref()].into_iter().flatten().collect();
    let conditions = DslValue::object(snapshot.spaces.iter().filter(|(_, other)| near.contains(other.storey.as_str())).map(|(other, _)| (other.clone(), dep_value(&snapshot.space_conditions.get(other).cloned()))));
    let walls = DslValue::object(snapshot.walls.iter().filter(|(_, wall)| wall.storey == storey).map(|(wall_id, wall)| (wall_id.clone(), dep_object([("type", dep_value(&wall.wall_type)), ("layers", layers_dependency(snapshot, snapshot.wall_types.get(&wall.wall_type).map_or(&[][..], |kind| kind.layers.as_slice())))]))));
    let hosted = DslValue::object(snapshot.openings.iter().filter(|(_, opening)| snapshot.walls.get(&opening.host).is_some_and(|wall| wall.storey == storey)).map(|(opening_id, opening)| {
        let thermal = match &opening.kind {
            OpeningKind::Window { window_type } => dep_value(&snapshot.window_types.get(window_type).map(|kind| (kind.u_value, kind.g_value, kind.frame_fraction))),
            OpeningKind::Door { door_type } => dep_value(&snapshot.door_types.get(door_type).map(|kind| kind.u_value)),
            OpeningKind::Void { .. } => DslValue::Null,
        };
        (opening_id.clone(), dep_object([("host", dep_value(&opening.host)), ("kind", dep_value(&opening.kind)), ("thermal", thermal)]))
    }));
    let slabs = DslValue::object(snapshot.slabs.iter().filter(|(_, slab)| near.contains(slab.storey.as_str())).map(|(slab_id, slab)| {
        (slab_id.clone(), dep_object([("storey", dep_value(&slab.storey)), ("boundary", dep_value(&slab.boundary)), ("holes", dep_value(&slab.holes)), ("layers", layers_dependency(snapshot, snapshot.slab_types.get(&slab.slab_type).map_or(&[][..], |kind| kind.layers.as_slice())))]))
    }));
    let roofs = DslValue::object(snapshot.roofs.iter().filter(|(_, roof)| roof.storey == storey).map(|(roof_id, roof)| {
        (roof_id.clone(), dep_object([("footprint", dep_value(&roof.footprint)), ("layers", layers_dependency(snapshot, snapshot.roof_types.get(&roof.roof_type).map_or(&[][..], |kind| kind.layers.as_slice())))]))
    }));
    let building = snapshot.storeys.get(storey).and_then(|row| snapshot.buildings.get(&row.building));
    let levels = DslValue::object(snapshot.storeys.iter().filter(|(_, other)| building.is_some() && snapshot.buildings.get(&other.building).is_some_and(|row| Some(row) == building)).map(|(other, row)| (other.clone(), dep_value(&row.level))));
    dep_object([
        ("zone", dep_value(&space.zone)),
        ("conditions", conditions),
        ("walls", walls),
        ("hosted", hosted),
        ("slabs", slabs),
        ("roofs", roofs),
        ("levels", levels),
        ("rotation", dep_value(&building.map(|row| row.rotation))),
        ("true_north", dep_value(&building.and_then(|row| snapshot.sites.get(&row.site)).map(|site| site.true_north))),
        ("authored", dep_value(&!snapshot.space_conditions.is_empty())),
    ])
}

/// 🔑️ Everything `totals_of` reads of the snapshot for a scope apart from its parents: which spaces are conditioned and where they belong.
pub fn totals_dependency(snapshot: &ModelSnapshot, scope: &EnergyScope) -> DslValue {
    let climate = Climate::of(snapshot);
    dep_object([
        ("scope", dep_value(scope)),
        ("spaces", DslValue::object(snapshot.spaces.keys().map(|id| (id.clone(), dep_object([("conditioned", dep_value(&climate.conditioned.get(id).copied().unwrap_or(false))), ("zone", dep_value(&climate.zone.get(id).cloned())), ("building", dep_value(&climate.building.get(id).cloned()))]))))),
    ])
}
//#endregion 🔖️Dependency

//#region 🔖️Projection
/// 🧾️ The area and heat loss of the surfaces of one space that share a kind, a boundary, a neighbour and a compass sector; `loss` is `sum(U * A)`, absent when a surface of the group has no transmittance.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
struct Group {
    area: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    loss: Option<f64>,
}

/// 🧾️ One space of the table the third-party oracle reproduces.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
struct SpaceMetrics {
    conditioned: bool,
    heated: bool,
    floor_area: f64,
    volume: f64,
    height: f64,
    open_length: f64,
    groups: BTreeMap<String, Group>,
}

/// 🧾️ The table the third-party oracle reproduces: the surfaces of every space merged by (kind, boundary, neighbour, sector) and the totals of every scope.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
struct EnergyTable {
    spaces: BTreeMap<String, SpaceMetrics>,
    totals: BTreeMap<String, EnergyTotals>,
}

fn group_key(surface: &EnvelopeSurface) -> String {
    let sector = if surface.vertical() { surface.sector().to_string() } else { "-".to_string() };
    format!("{}|{}|{}|{}", surface.kind.name(), surface.boundary.name(), surface.adjacent, sector)
}

fn metrics_of(envelope: &EnvelopeSpace) -> SpaceMetrics {
    let mut groups: BTreeMap<String, Group> = BTreeMap::new();
    for surface in &envelope.surfaces {
        let group = groups.entry(group_key(surface)).or_insert(Group { area: 0.0, loss: Some(0.0) });
        group.area += surface.area;
        group.loss = group.loss.zip(surface.u_value).map(|(loss, u_value)| loss + u_value * surface.area);
    }
    SpaceMetrics { conditioned: envelope.conditioned, heated: envelope.heated, floor_area: envelope.floor_area, volume: envelope.volume, height: envelope.height, open_length: envelope.open_length, groups }
}

/// 🧾️ The table as canonical JSON text.
pub fn table_json(envelopes: &BTreeMap<String, EnvelopeSpace>, totals: &BTreeMap<String, EnergyTotals>) -> String {
    semio_framework_pack_json::to_json_string(&EnergyTable { spaces: envelopes.iter().map(|(id, envelope)| (id.clone(), metrics_of(envelope))).collect(), totals: totals.clone() })
}
//#endregion 🔖️Projection

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
