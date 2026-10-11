//! 🪑️ `components`: the inferred placement of every component, an instance of a parametric family (furniture, equipment, casework, a fixture, a terminal). A component stores its storey, family, plan position, height above
//! the storey, rotation, mirror flag, optional host wall and optional terminal system, and its parameter overrides live in `component_overrides`; everything else is derived here and never stored: the family value under the
//! overrides, the position and turn in the building frame including the fit to the host wall, the footprint, the bounds, the volume, the connector and the issues. Its parents in the model graph are the storey node it stands
//! on, the `Family` node of its family (shared as is while the component has no override) and the layout of its host wall.
//!
//! Frame. The family frame is `x` right, `y` depth, `z` up (metres). A component maps it into the building frame by mirroring `x` (when `mirrored`), turning about the vertical axis by `yaw` and moving the origin to
//! `(x, y, z)`. Free-standing: `yaw = rotation`. Hosted: the authored plan position is projected onto the axis of the host wall, the face of the wall on the side where the position lies becomes the plane `y = 0` of the family
//! (the origin is the projection on that face) and the family turns so that its `+y` points away from the wall; `rotation` is added to that turn. The origin of a component lies at the elevation of its storey plus `elevation`.
//!
//! Related: <https://en.wikipedia.org/wiki/Family_(building_information_modeling)>.

use super::super::element_solids::plan_kit::seg;
use super::super::element_solids::{dep_object, dep_value, Anonymous, SolidBounds, SolidPoint};
use super::super::families::{self, issues, FamilyIssue, FamilyValue, ResolvedParameter};
use super::super::mep;
use super::super::storey_levels::StoreyLevel;
use super::super::wall_layout::WallLayout;
use crate::{Component, FamilyCategory, MepSystem, ModelSnapshot, Point2, Point3};
use semio_framework_geometry::vector::{cross, perp};
use semio_framework_geometry::Vec2;
use semio_framework_value::DslValue;
use std::collections::BTreeMap;
use std::sync::Arc;

#[path = "🩺️findings/🦀️.rs"]
pub mod findings;

//#region 🔖️Values
/// 🧱️ How a hosted component clings to its wall: the arc length of the projection of the authored position on the axis, the side of the axis it lies on (`1` left, `-1` right, along the axis direction), the point on the face and the unit normal pointing away from the wall.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct HostFit {
    pub wall: String,
    pub station: f64,
    pub side: f64,
    pub face: Point2,
    pub normal: Point2,
}

/// 🧭️ Where the family origin stands in the building frame (metres, `z` from the building datum) and how the family is turned: `yaw` in radians counter-clockwise about the vertical axis, `mirrored` flips the local `x` first.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct ComponentPlacement {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f64,
    pub mirrored: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<HostFit>,
}

impl ComponentPlacement {
    /// 🧭️ The image of a point of the family frame in the building frame.
    pub fn apply(&self, local: [f64; 3]) -> [f64; 3] {
        let x = if self.mirrored { -local[0] } else { local[0] };
        let (sin, cos) = self.yaw.sin_cos();
        [self.x + cos * x - sin * local[1], self.y + sin * x + cos * local[1], self.z + local[2]]
    }

    /// 🧭️ The image of a direction of the family frame (no translation).
    pub fn rotate(&self, local: [f64; 3]) -> [f64; 3] {
        let x = if self.mirrored { -local[0] } else { local[0] };
        let (sin, cos) = self.yaw.sin_cos();
        [cos * x - sin * local[1], sin * x + cos * local[1], local[2]]
    }
}

/// 🔌️ The connector of a terminal: the service it carries, its colour and where it sits.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct Connector {
    pub system: MepSystem,
    pub colour: String,
    pub position: Point3,
}

/// 🚦️ What is wrong with a component itself (the faults of the placement; the diagnostics of the geometry come from the levels and walls).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub enum ComponentIssueCode {
    FamilyMissing,
    FamilyProfile,
    HostMissing,
    HostOtherStorey,
    HostDegenerate,
    Override,
    NonFinite,
}

impl ComponentIssueCode {
    /// 🏷️ The stable kebab-case slug.
    pub fn slug(self) -> &'static str {
        match self {
            Self::FamilyMissing => "family-missing",
            Self::FamilyProfile => "family-profile",
            Self::HostMissing => "host-missing",
            Self::HostOtherStorey => "host-other-storey",
            Self::HostDegenerate => "host-degenerate",
            Self::Override => "override",
            Self::NonFinite => "non-finite",
        }
    }
}

/// 🚦️ One issue: `subject` is the family, wall or parameter id it is about, `detail` an English fallback text and `family_issue` the issue of the family formulas an override causes (its message is `families::issues::message`).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct ComponentIssue {
    pub code: ComponentIssueCode,
    pub subject: String,
    pub detail: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub family_issue: Option<FamilyIssue>,
}

/// 🪑️ Everything inferred about one component. `footprint` is the oriented rectangle (counter-clockwise, building frame) of the bounds of the visible solids in the family frame, `bounds` the box of that rectangle
/// and the heights of the visible solids, `volume` the sum of the volumes of the visible solids, `parameters` the parameters of the family under the overrides and `overridden` the names of the overrides that name a parameter.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct ComponentValue {
    pub storey: String,
    pub family: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<FamilyCategory>,
    pub placement: ComponentPlacement,
    pub footprint: Vec<Point2>,
    pub footprint_area: f64,
    pub bounds: SolidBounds,
    pub volume: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub connector: Option<Connector>,
    pub overridden: Vec<String>,
    pub parameters: BTreeMap<String, ResolvedParameter>,
    pub issues: Vec<ComponentIssue>,
}

/// 📦️ The value of a `Component` node: the projected value and the family under the overrides (the shared `Family` value while there is no override), which the solid is built from.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ComponentEntry {
    pub value: ComponentValue,
    pub family: Arc<FamilyValue>,
}

impl ComponentValue {
    /// 🧊️ Whether the component has geometry: a footprint of positive area.
    pub fn solid(&self) -> bool {
        self.footprint.len() >= 3 && self.footprint_area > 0.0
    }

    /// 🔌️ Whether the component is a terminal.
    pub fn terminal(&self) -> bool {
        self.connector.is_some()
    }

    /// 🚦️ Whether one of the issues has the code.
    pub fn has(&self, code: ComponentIssueCode) -> bool {
        self.issues.iter().any(|issue| issue.code == code)
    }
}
//#endregion 🔖️Values

//#region 🔖️Overrides
/// 🎚️ The override formulas of component `id` by parameter name, read from the keyed rows `id.name` without scanning the collection.
pub fn overrides_of(snapshot: &ModelSnapshot, id: &str) -> BTreeMap<String, String> {
    let prefix = format!("{id}.");
    snapshot.component_overrides.range(prefix.clone()..).take_while(|(key, _)| key.starts_with(&prefix)).filter(|(_, row)| row.component == id).map(|(_, row)| (row.name.clone(), row.value.clone())).collect()
}
//#endregion 🔖️Overrides

//#region 🔖️Host
fn unit(vector: Vec2) -> Vec2 {
    let length = vector.hypot();
    if length > 0.0 {
        vector / length
    } else {
        vector
    }
}

/// 🧱️ The fit of a position to the wall with `axis`, whose faces lie `left` and `right` metres from the axis: the projection on the axis, the side of the position, the point on the face of that side and the normal pointing away from the wall.
pub fn fit_to_wall(wall: &str, axis: &crate::Axis, left: f64, right: f64, position: Point2) -> Option<HostFit> {
    let segment = seg(axis);
    if segment.length() <= mep::EPS {
        return None;
    }
    let at = semio_framework_geometry::Point::new(position.x, position.y);
    let closest = segment.closest(at);
    let tangent = unit(segment.tangent_at(closest.t.clamp(0.0, 1.0)));
    let normal = perp(tangent);
    let side = if cross(tangent, Vec2::new(at.x - closest.point.x, at.y - closest.point.y)) >= 0.0 { 1.0 } else { -1.0 };
    let offset = if side > 0.0 { left } else { right };
    let away = normal * side;
    Some(HostFit { wall: wall.to_string(), station: closest.t.clamp(0.0, 1.0) * segment.length(), side, face: Point2 { x: closest.point.x + away.x * offset, y: closest.point.y + away.y * offset }, normal: Point2 { x: away.x, y: away.y } })
}

fn host_fit(snapshot: &ModelSnapshot, component: &Component, wall_id: &str, layout: Option<&WallLayout>, issues: &mut Vec<ComponentIssue>) -> Option<HostFit> {
    let Some(wall) = snapshot.walls.get(wall_id) else {
        issues.push(ComponentIssue { code: ComponentIssueCode::HostMissing, subject: wall_id.to_string(), detail: format!("the host wall {wall_id} does not exist"), family_issue: None });
        return None;
    };
    if wall.storey != component.storey {
        issues.push(ComponentIssue { code: ComponentIssueCode::HostOtherStorey, subject: wall_id.to_string(), detail: format!("the host wall {wall_id} stands on another storey"), family_issue: None });
        return None;
    }
    let fit = layout.and_then(|layout| fit_to_wall(wall_id, &wall.axis, layout.offset_left, layout.offset_right, component.position));
    if fit.is_none() {
        issues.push(ComponentIssue { code: ComponentIssueCode::HostDegenerate, subject: wall_id.to_string(), detail: format!("the host wall {wall_id} has no axis of positive length"), family_issue: None });
    }
    fit
}
//#endregion 🔖️Host

//#region 🔖️Value
fn bounds_of(family: &FamilyValue) -> Option<([f64; 3], [f64; 3])> {
    family.visible().filter(|(_, solid)| !solid.indices.is_empty()).map(|(_, solid)| ([solid.bounds.min.x, solid.bounds.min.y, solid.bounds.min.z], [solid.bounds.max.x, solid.bounds.max.y, solid.bounds.max.z])).reduce(|(lo, hi), (a, b)| ([lo[0].min(a[0]), lo[1].min(a[1]), lo[2].min(a[2])], [hi[0].max(b[0]), hi[1].max(b[1]), hi[2].max(b[2])]))
}

fn signed_area(ring: &[Point2]) -> f64 {
    let count = ring.len();
    (0..count).map(|index| ring[index].x * ring[(index + 1) % count].y - ring[(index + 1) % count].x * ring[index].y).sum::<f64>() / 2.0
}

/// 🪑️ The value of component `id` on the storey whose level is `level`. `base` is the `Family` node value of its family (shared while there are no overrides), `layout` the layout of its host wall.
pub fn component_of(snapshot: &ModelSnapshot, id: &str, level: &StoreyLevel, base: Option<&Arc<FamilyValue>>, layout: Option<&WallLayout>) -> ComponentEntry {
    let Some(component) = snapshot.components.get(id) else { return ComponentEntry::default() };
    let mut issues: Vec<ComponentIssue> = Vec::new();
    let overrides = overrides_of(snapshot, id);
    let category = snapshot.families.get(&component.family).map(|row| row.category);
    let family: Arc<FamilyValue> = match category {
        None => {
            issues.push(ComponentIssue { code: ComponentIssueCode::FamilyMissing, subject: component.family.clone(), detail: format!("the family {} does not exist", component.family), family_issue: None });
            Arc::new(FamilyValue::default())
        }
        Some(FamilyCategory::Profile) => {
            issues.push(ComponentIssue { code: ComponentIssueCode::FamilyProfile, subject: component.family.clone(), detail: format!("the family {} is a profile and cannot be placed", component.family), family_issue: None });
            Arc::new(FamilyValue::default())
        }
        Some(_) if overrides.is_empty() => base.cloned().unwrap_or_else(|| Arc::new(families::family_of(snapshot, &component.family, &BTreeMap::new()))),
        Some(_) => {
            let own = Arc::new(families::family_of(snapshot, &component.family, &overrides));
            let plain = base.map(|shared| shared.issues.clone()).unwrap_or_else(|| families::family_of(snapshot, &component.family, &BTreeMap::new()).issues);
            for issue in own.issues.iter().filter(|issue| !plain.contains(issue)) {
                issues.push(ComponentIssue { code: ComponentIssueCode::Override, subject: issue.subject.clone(), detail: issues::message(issue, "en"), family_issue: Some(issue.clone()) });
            }
            own
        }
    };
    let finite = component.position.x.is_finite() && component.position.y.is_finite() && component.elevation.is_finite() && component.rotation.is_finite();
    if !finite {
        issues.push(ComponentIssue { code: ComponentIssueCode::NonFinite, subject: String::new(), detail: "a coordinate is not a finite number".to_string(), family_issue: None });
    }
    let fit = if finite { component.host.as_deref().and_then(|wall| host_fit(snapshot, component, wall, layout, &mut issues)) } else { None };
    let z = level.elevation + component.elevation;
    let placement = match &fit {
        Some(fit) => ComponentPlacement { x: fit.face.x, y: fit.face.y, z, yaw: (-fit.normal.x).atan2(fit.normal.y) + component.rotation, mirrored: component.mirrored, host: Some(fit.clone()) },
        None => ComponentPlacement { x: component.position.x, y: component.position.y, z, yaw: component.rotation, mirrored: component.mirrored, host: None },
    };
    let (mut footprint, mut footprint_area, mut bounds, mut volume) = (Vec::new(), 0.0, SolidBounds::default(), 0.0);
    if let Some((lo, hi)) = bounds_of(&family).filter(|_| finite) {
        let corners = [[lo[0], lo[1]], [hi[0], lo[1]], [hi[0], hi[1]], [lo[0], hi[1]]];
        footprint = corners.iter().map(|corner| placement.apply([corner[0], corner[1], 0.0])).map(|at| Point2 { x: at[0], y: at[1] }).collect();
        if signed_area(&footprint) < 0.0 {
            footprint.reverse();
        }
        footprint_area = (hi[0] - lo[0]) * (hi[1] - lo[1]);
        let span = |value: fn(&Point2) -> f64| footprint.iter().map(value).fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), v| (low.min(v), high.max(v)));
        let ((x0, x1), (y0, y1)) = (span(|point| point.x), span(|point| point.y));
        bounds = SolidBounds { min: SolidPoint { x: x0, y: y0, z: z + lo[2] }, max: SolidPoint { x: x1, y: y1, z: z + hi[2] } };
        volume = family.volume();
    }
    let connector = component.system.filter(|_| finite).map(|system| Connector { system, colour: mep::colour(system).to_string(), position: Point3 { x: placement.x, y: placement.y, z } });
    let overridden = overrides.keys().filter(|name| family.parameters.contains_key(*name)).cloned().collect();
    let value = ComponentValue { storey: component.storey.clone(), family: component.family.clone(), category, placement, footprint, footprint_area, bounds, volume, connector, overridden, parameters: family.parameters.clone(), issues };
    ComponentEntry { value, family }
}

/// 🔑️ Everything `component_of` reads of the snapshot besides the level of the storey, the shared family value and the layout of the host (its parents): the component record without its name, its overrides, the family
/// reads when there are overrides (the instance evaluates the family itself) and the axis and storey of the host wall.
pub fn dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let Some(component) = snapshot.components.get(id) else { return DslValue::Null };
    let overrides = overrides_of(snapshot, id);
    let family = if overrides.is_empty() { DslValue::Null } else { families::dependency(snapshot, &component.family) };
    let host = component.host.as_ref().and_then(|wall| snapshot.walls.get(wall)).map(|wall| dep_object([("storey", dep_value(&wall.storey)), ("axis", dep_value(&wall.axis))]));
    dep_object([("component", dep_value(&component.anonymous())), ("overrides", dep_value(&overrides)), ("family", family), ("host", host.unwrap_or(DslValue::Null))])
}

/// 🗺️ The snapshot collections a component reads.
pub const READS: &[&str] = &["components", "component_overrides", "families", "family_parameters", "family_solids", "materials", "walls", "storeys", "buildings", "sites"];
//#endregion 🔖️Value

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🕸️graph/🦀️.rs"]
mod graph_tests;
