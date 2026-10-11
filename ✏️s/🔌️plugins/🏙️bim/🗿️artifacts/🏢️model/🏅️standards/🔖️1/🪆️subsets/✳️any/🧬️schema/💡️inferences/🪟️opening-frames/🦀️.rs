//! 🪟️ `opening-frames`: where every window, door and void sits in its host and what it cuts out of it.
//!
//! An opening stores its host id, its type or void size, its centre `offset` along the host axis, an optional size override, an optional sill
//! override and two flips. Everything else is resolved here: size and sill (override, else type; a window sill is its sill override, else the type sill),
//! the point and tangent on the host axis at arc length `offset` (line or arc), the local frame (origin at the opening centre at sill
//! level on the host location line, `z` up, `y` the host's left normal, both `x` and `y` turned by `flip_facing`), the world frame
//! (building origin, rotation and datum), the cut rectangle in the host's `(s, z)` development, the reveal depth (the authored reveal, else the host thickness), the setback of the frame and the reveal material,
//! the door swing and window glazing as plan strokes, and the validity of the placement.
//!
//! The graph is storey → wall or curtain layout → host → opening, so one storey height edit, one `set-wall-axis` or one wall top edit
//! re-derives exactly the frames that depend on it. Siblings on the same host are parents of an opening through their `Cut` nodes (the
//! resolved size and cut rectangle), because overlap is judged against them.
//!
//! One validity rule ([`host_issues`]) decides whether an opening is cut out of its host: the wall solid cuts exactly the frames that are `valid`, fillers
//! are built for exactly those, quantities subtract exactly those cuts and diagnostics report every issue. A cut must lie strictly inside the part of the
//! host that has its full thickness (the join-trimmed extent) and strictly below the top, with the one tolerance [`LENGTH_EPS`]; a hole that touches the
//! border of the face is [`OpeningIssue::OutsideTrimmedExtent`].

use super::super::curtain_layout::{CurtainLayout, DEFAULT_MULLION_DEPTH};
use super::super::element_solids::plan_kit::{depth_of, mark, seg};
use crate::standards::v1::subsets::any::schema::authored::sizes::{resolve_size, Resolved};
use super::super::storey_levels::StoreyLevel;
use super::super::wall_layout::attach::{elevation_at, ElevationPoint};
use super::super::wall_layout::{face_ends, WallLayout};
use crate::{Axis, CurtainWall, DoorLeaves, DoorType, ModelSnapshot, Opening, OpeningKind, Point2, Profile, Swing, Wall, WindowType};
use semio_framework_geometry::placement::Affine3;
use semio_framework_geometry::vector::{perp, LENGTH_EPS};
use semio_framework_geometry::{Point, Vec2};
use semio_framework_value::DslValue;
use std::collections::BTreeMap;
use std::f64::consts::FRAC_PI_2;

//#region 🔖️Values
/// 🧭️ A vector or point in metres.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🧭️ A right-handed placement: an origin and three unit axes, as the columns of a rigid transform.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct Frame {
    pub origin: Vec3,
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
}

impl Frame {
    /// 🧭️ The transform mapping opening-local coordinates into the frame's space.
    pub fn affine(&self) -> Affine3 {
        let array = |v: &Vec3| [v.x, v.y, v.z];
        Affine3::from_frame(array(&self.origin), array(&self.x_axis), array(&self.y_axis), array(&self.z_axis))
    }
}

/// ✂️ The rectangle an opening removes from its host, in the host's development: `s` is the arc length along the axis, `z` runs up from the host base.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct OpeningCut {
    pub s_min: f64,
    pub s_max: f64,
    pub z_min: f64,
    pub z_max: f64,
}

/// 🩺️ Why a placement is not valid; an empty list means valid.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub enum OpeningIssue {
    HostMissing,
    TypeMissing,
    NonPositiveSize,
    HostDegenerate,
    OutsideHostExtent,
    BelowHostBase,
    AboveHostTop,
    OutsideTrimmedExtent,
    OverlapsSibling,
}

/// 🖊️ What a plan stroke depicts.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub enum PlanRole {
    Leaf,
    Swing,
    Glazing,
}

/// ✏️ A plan primitive in building plan coordinates: a segment, or a circular arc from `start_angle` through the signed `sweep` (radians, counter-clockwise positive).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub enum PlanShape {
    Line { from: Point2, to: Point2 },
    Arc { centre: Point2, radius: f64, start_angle: f64, sweep: f64 },
}

/// 🖊️ One plan stroke of an opening symbol.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct PlanStroke {
    pub role: PlanRole,
    pub shape: PlanShape,
}

fn is_false(value: &bool) -> bool {
    !*value
}

/// 🪟️ The resolved placement of one opening. Lengths in metres, `local` in building coordinates (`z` up from the building datum), `world` after the building origin, rotation and datum.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct OpeningFrame {
    pub width: f64,
    pub height: f64,
    pub sill: f64,
    pub offset: f64,
    pub cut: OpeningCut,
    pub reveal_depth: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub setback: Option<f64>,
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub reveal_material: String,
    #[value(default, skip_serializing_if = "is_false")]
    pub facing_right: bool,
    pub face_front: f64,
    pub face_back: f64,
    pub host_length: f64,
    pub host_height: f64,
    pub point: Point2,
    pub local: Frame,
    pub world: Frame,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hand: Option<Swing>,
    pub plan: Vec<PlanStroke>,
    pub issues: Vec<OpeningIssue>,
    pub overlaps: Vec<String>,
    pub valid: bool,
}

impl Default for OpeningFrame {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            sill: 0.0,
            offset: 0.0,
            cut: OpeningCut::default(),
            reveal_depth: 0.0,
            setback: None,
            reveal_material: String::new(),
            facing_right: false,
            face_front: 0.0,
            face_back: 0.0,
            host_length: 0.0,
            host_height: 0.0,
            point: Point2 { x: 0.0, y: 0.0 },
            local: Frame::default(),
            world: Frame::default(),
            hand: None,
            plan: Vec::new(),
            issues: Vec::new(),
            overlaps: Vec::new(),
            valid: false,
        }
    }
}

/// 🏗️ The resolved host of openings (the `Host` node of the model graph): its axis, vertical extent and thickness, the building placement it lives in and `trim`, the arc-length
/// extent `lo..hi` over which it has its full thickness (the join-trimmed extent of a wall, the whole axis of a curtain wall).
#[derive(Clone, Debug, PartialEq)]
pub struct HostExtent {
    pub axis: Axis,
    pub base_z: f64,
    pub height: f64,
    pub thickness: f64,
    pub length: f64,
    pub face_left: f64,
    pub face_right: f64,
    pub datum: f64,
    pub origin: Point2,
    pub rotation: f64,
    pub trim: (f64, f64),
    pub base_profile: Vec<ElevationPoint>,
    pub top_profile: Vec<ElevationPoint>,
}

impl HostExtent {
    /// 🧱️ The host a wall is, from its layout: the vertical extent, thickness and faces are the layout's, the trimmed extent follows the join-trimmed faces.
    pub fn of_wall(wall: &Wall, layout: &WallLayout, own: &StoreyLevel, placement: (Point2, f64)) -> Self {
        let trim = face_ends(layout, &wall.axis).map_or((0.0, layout.length), |ends| ends.full_thickness());
        Self { axis: wall.axis.clone(), base_z: layout.base_z, height: layout.height, thickness: layout.thickness, length: layout.length, face_left: layout.offset_left, face_right: layout.offset_right, datum: own.absolute_elevation - own.elevation, origin: placement.0, rotation: placement.1, trim, base_profile: layout.base_profile.clone(), top_profile: layout.top_profile.clone() }
    }

    /// 🪞️ The host a curtain wall is, from its layout and the interior mullion of its type: the mullion depth (a default while the type is missing) is the thickness, centred on the axis, and it has its full thickness along the whole axis.
    pub fn of_curtain(curtain: &CurtainWall, mullion: Option<&Profile>, layout: &CurtainLayout, own: &StoreyLevel, placement: (Point2, f64)) -> Self {
        let thickness = mullion.map_or(DEFAULT_MULLION_DEPTH, depth_of);
        Self { axis: curtain.axis.clone(), base_z: layout.base_z, height: layout.height, thickness, length: layout.length, face_left: thickness / 2.0, face_right: thickness / 2.0, datum: own.absolute_elevation - own.elevation, origin: placement.0, rotation: placement.1, trim: (0.0, layout.length), base_profile: Vec::new(), top_profile: Vec::new() }
    }

    /// 📈️ The absolute height of the base of the host at arc length `s`: the base profile of an attached base, the flat base otherwise.
    pub fn base_at(&self, s: f64) -> f64 {
        if self.base_profile.is_empty() {
            self.base_z
        } else {
            elevation_at(&self.base_profile, s)
        }
    }

    /// 📈️ The absolute height of the top of the host at arc length `s`: the top profile of an attached top, the flat top otherwise.
    pub fn top_at(&self, s: f64) -> f64 {
        if self.top_profile.is_empty() {
            self.base_z + self.height
        } else {
            elevation_at(&self.top_profile, s)
        }
    }

    /// 📏️ The height above the local base (the base at the centre of the cut) up to which a cut may reach: the flat height, or the lowest top over the arc length range of the cut less that base.
    pub fn limit(&self, cut: &OpeningCut) -> f64 {
        if self.top_profile.is_empty() && self.base_profile.is_empty() {
            return self.height;
        }
        let base = self.base_at((cut.s_min + cut.s_max) / 2.0);
        let inner = self.top_profile.iter().map(|point| point.s).filter(|s| *s > cut.s_min && *s < cut.s_max);
        [cut.s_min, cut.s_max].into_iter().chain(inner).map(|s| self.top_at(s)).fold(f64::INFINITY, f64::min) - base
    }

    /// ↔️ The lateral centre of the host thickness (left positive): the offset of the middle of the host from its location line.
    pub fn centre(&self) -> f64 {
        (self.face_left - self.face_right) / 2.0
    }
}

/// 📐️ The resolved size and cut rectangle of one opening (the `Cut` node of the model graph).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CutRect {
    pub size: Resolved,
    pub cut: OpeningCut,
}
//#endregion 🔖️Values

//#region 🔖️Resolution
/// ✂️ The cut rectangle of an opening in its host's `(s, z)` development.
pub fn cut_of(snapshot: &ModelSnapshot, opening: &Opening) -> OpeningCut {
    let size = resolve_size(snapshot, opening);
    OpeningCut { s_min: opening.offset - size.width / 2.0, s_max: opening.offset + size.width / 2.0, z_min: size.sill, z_max: size.sill + size.height }
}

/// ✂️ The resolved size and cut rectangle of an opening (the `Cut` node).
pub fn cut_rect(snapshot: &ModelSnapshot, opening: &Opening) -> CutRect {
    CutRect { size: resolve_size(snapshot, opening), cut: cut_of(snapshot, opening) }
}

/// 🔑️ What `cut_rect` reads: the opening record and the window or door type it names.
pub fn cut_dependency(snapshot: &ModelSnapshot, opening: &Opening) -> DslValue {
    let window: Option<WindowType> = if let OpeningKind::Window { window_type } = &opening.kind { snapshot.window_types.get(window_type).cloned() } else { None };
    let door: Option<DoorType> = if let OpeningKind::Door { door_type } = &opening.kind { snapshot.door_types.get(door_type).cloned() } else { None };
    DslValue::object([("opening".to_string(), semio_framework_value::ToValue::to_value(opening)), ("window_type".to_string(), semio_framework_value::ToValue::to_value(&window)), ("door_type".to_string(), semio_framework_value::ToValue::to_value(&door))])
}

fn intersects(a: &OpeningCut, b: &OpeningCut) -> bool {
    a.s_max.min(b.s_max) - a.s_min.max(b.s_min) > LENGTH_EPS && a.z_max.min(b.z_max) - a.z_min.max(b.z_min) > LENGTH_EPS
}

/// 🏙️ The origin and rotation of the building a storey belongs to (the identity placement when the storey or building is missing).
pub fn building_placement(snapshot: &ModelSnapshot, storey: &str) -> (Point2, f64) {
    snapshot.storeys.get(storey).and_then(|row| snapshot.buildings.get(&row.building)).map_or((Point2 { x: 0.0, y: 0.0 }, 0.0), |building| (building.origin, building.rotation))
}

/// 🩺️ The one validity rule of an opening in a host (the issues of the placement; empty means the opening is cut out of the host). All comparisons use [`LENGTH_EPS`].
/// * `OutsideHostExtent`: the cut reaches beyond the axis (`s < 0` or `s > length`); `BelowHostBase`: it starts below the host base; `AboveHostTop`: it ends above the host top.
/// * `OutsideTrimmedExtent`: it is not beyond those limits but it does not lie strictly inside the part of the host that has its full thickness (`host.trim`, the join-trimmed extent) and strictly below the top, so a hole would touch the border of the face and the host solid could not cut it.
pub fn host_issues(cut: &OpeningCut, host: &HostExtent) -> Vec<OpeningIssue> {
    let mut issues = Vec::new();
    let beyond = cut.s_min < -LENGTH_EPS || cut.s_max > host.length + LENGTH_EPS;
    let limit = host.limit(cut);
    let above = cut.z_max > limit + LENGTH_EPS;
    if beyond {
        issues.push(OpeningIssue::OutsideHostExtent);
    }
    if cut.z_min < -LENGTH_EPS {
        issues.push(OpeningIssue::BelowHostBase);
    }
    if above {
        issues.push(OpeningIssue::AboveHostTop);
    }
    let (lo, hi) = host.trim;
    let inside = cut.s_min > lo + LENGTH_EPS && cut.s_max < hi - LENGTH_EPS && cut.z_max < limit - LENGTH_EPS;
    if !beyond && !above && !inside {
        issues.push(OpeningIssue::OutsideTrimmedExtent);
    }
    issues
}
//#endregion 🔖️Resolution

//#region 🔖️Geometry
fn rotate(rotation: f64, x: f64, y: f64) -> (f64, f64) {
    let (sin, cos) = rotation.sin_cos();
    (x * cos - y * sin, x * sin + y * cos)
}

fn world_frame(local: &Frame, origin: Point2, rotation: f64, datum: f64) -> Frame {
    let vector = |v: &Vec3| {
        let (x, y) = rotate(rotation, v.x, v.y);
        Vec3 { x, y, z: v.z }
    };
    let (x, y) = rotate(rotation, local.origin.x, local.origin.y);
    Frame { origin: Vec3 { x: origin.x + x, y: origin.y + y, z: local.origin.z + datum }, x_axis: vector(&local.x_axis), y_axis: vector(&local.y_axis), z_axis: local.z_axis }
}

fn effective_hand(opening: &Opening, door: &DoorType) -> Swing {
    match (door.swing, opening.flip_hand) {
        (Swing::Left, false) | (Swing::Right, true) => Swing::Left,
        (Swing::Right, false) | (Swing::Left, true) => Swing::Right,
    }
}

fn leaf(hinge: Point, side: f64, length: f64, x: Vec2, y: Vec2) -> [PlanStroke; 2] {
    let free = hinge - x * (side * length);
    let open = hinge + y * length;
    let closed = free - hinge;
    let start_angle = closed.y.atan2(closed.x);
    let sweep = if closed.x * y.y - closed.y * y.x > 0.0 { FRAC_PI_2 } else { -FRAC_PI_2 };
    [
        PlanStroke { role: PlanRole::Leaf, shape: PlanShape::Line { from: mark(hinge), to: mark(open) } },
        PlanStroke { role: PlanRole::Swing, shape: PlanShape::Arc { centre: mark(hinge), radius: length, start_angle, sweep } },
    ]
}

/// 🚪️ Plan strokes of an opening symbol in building coordinates. A door leaf swings towards `+y` about its hinge on the jamb at the host face on the `+y` side (`face` from the location line): seen from the `+y` side, a `Left` hand hinges at `+x`, a `Right` hand at `-x`; a double door has two leaves of half the width. A window draws its glazing line on the location line; a void nothing.
pub fn plan_strokes(kind: &OpeningKind, leaves: DoorLeaves, hand: Option<Swing>, centre: Point, x: Vec2, y: Vec2, width: f64, face: f64) -> Vec<PlanStroke> {
    match (kind, hand) {
        (OpeningKind::Window { .. }, _) => vec![PlanStroke { role: PlanRole::Glazing, shape: PlanShape::Line { from: mark(centre - x * (width / 2.0)), to: mark(centre + x * (width / 2.0)) } }],
        (OpeningKind::Door { .. }, Some(hand)) => {
            let (side, centre) = (if hand == Swing::Left { 1.0 } else { -1.0 }, centre + y * face);
            match leaves {
                DoorLeaves::Single => leaf(centre + x * (side * width / 2.0), side, width, x, y).to_vec(),
                DoorLeaves::Double => [1.0, -1.0].into_iter().flat_map(|side| leaf(centre + x * (side * width / 2.0), side, width / 2.0, x, y)).collect(),
            }
        }
        _ => Vec::new(),
    }
}

/// 🪟️ The frame of one opening on a resolved host (`None` when the host does not resolve) among its `siblings` (the id and cut rectangle of every other opening of the host, in id order).
/// A host without length gets the issue `HostDegenerate` and no placement, never an invented direction.
pub fn frame_of(snapshot: &ModelSnapshot, id: &str, own: &CutRect, host: Option<&HostExtent>, siblings: &[(String, OpeningCut)]) -> OpeningFrame {
    let Some(opening) = snapshot.openings.get(id) else { return OpeningFrame::default() };
    let (size, cut) = (own.size, own.cut);
    let mut issues = Vec::new();
    if host.is_none() {
        issues.push(OpeningIssue::HostMissing);
    }
    if !size.type_found {
        issues.push(OpeningIssue::TypeMissing);
    }
    if size.width <= LENGTH_EPS || size.height <= LENGTH_EPS {
        issues.push(OpeningIssue::NonPositiveSize);
    }
    let base = OpeningFrame { width: size.width, height: size.height, sill: size.sill, offset: opening.offset, cut, ..OpeningFrame::default() };
    let Some(host) = host else { return OpeningFrame { issues, ..base } };
    if host.length <= LENGTH_EPS {
        issues.push(OpeningIssue::HostDegenerate);
        return OpeningFrame { issues, ..base };
    }
    issues.extend(host_issues(&cut, host));
    let overlaps: Vec<String> = siblings.iter().filter(|(_, other)| intersects(&cut, other)).map(|(other, _)| other.clone()).collect();
    if !overlaps.is_empty() {
        issues.push(OpeningIssue::OverlapsSibling);
    }
    let axis = seg(&host.axis);
    let (centre, tangent) = (axis.point_at_length(opening.offset), axis.tangent_at_length(opening.offset));
    let facing = if opening.flip_facing { -1.0 } else { 1.0 };
    let (x, y) = (tangent * facing, perp(tangent) * facing);
    let (face_front, face_back) = if opening.flip_facing { (host.face_right, host.face_left) } else { (host.face_left, host.face_right) };
    let z = host.base_at(opening.offset) + size.sill;
    let local = Frame { origin: Vec3 { x: centre.x, y: centre.y, z }, x_axis: Vec3 { x: x.x, y: x.y, z: 0.0 }, y_axis: Vec3 { x: y.x, y: y.y, z: 0.0 }, z_axis: Vec3 { x: 0.0, y: 0.0, z: 1.0 } };
    let (leaves, hand) = match &opening.kind {
        OpeningKind::Door { door_type } => snapshot.door_types.get(door_type).map_or((DoorLeaves::Single, None), |door| (door.leaves, Some(effective_hand(opening, door)))),
        _ => (DoorLeaves::Single, None),
    };
    OpeningFrame {
        reveal_depth: opening.reveal_depth.map_or(host.thickness, |depth| depth.clamp(0.0, host.thickness)),
        setback: opening.reveal_depth.map(|depth| depth.clamp(0.0, host.thickness)),
        reveal_material: opening.reveal_material.clone().unwrap_or_default(),
        facing_right: opening.flip_facing,
        face_front,
        face_back,
        host_length: host.length,
        host_height: host.height,
        point: mark(centre),
        world: world_frame(&local, host.origin, host.rotation, host.datum),
        local,
        hand,
        plan: plan_strokes(&opening.kind, leaves, hand, centre, x, y, size.width, face_front),
        valid: issues.is_empty(),
        issues,
        overlaps,
        ..base
    }
}
//#endregion 🔖️Geometry

//#region 🔖️Projection
/// 🔑️ What `frame_of` reads of an opening besides the host, its own cut and the sibling cuts: the opening record and the window or door type it names.
pub fn dependency(snapshot: &ModelSnapshot, opening: &Opening) -> DslValue {
    cut_dependency(snapshot, opening)
}

/// 🪟️ The frame of every opening (the `OpeningFrame` nodes of the model graph).
#[cfg(test)]
pub fn compute_opening_frames(snapshot: &ModelSnapshot) -> BTreeMap<String, OpeningFrame> {
    std::mem::take(&mut super::super::model_graph::infer_selected::<{ super::super::model_graph::kinds::FRAMES }>(snapshot).opening_frames)
}
//#endregion 🔖️Projection

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
