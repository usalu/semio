//! 🔭️ `createView`: brings authored views into the model, placed from the authored geometry of the building. A plan or ceiling plan cuts a storey; a section is a plane through the middle of the building;
//! an elevation is a plane outside the building looking at one of its sides, and `elevations` creates the four of them at once, south, east, north and west; a camera orbits the building. Every view is
//! one ordinary `create-view` mutation with concrete values, so the command writes no more than a person could author, and it binds the addressed window to the view it made.

use crate::editor::bim::entities::ordered_storeys;
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::kit::{fault, select_effect, IdMint};
use crate::editor::bim::modes::edit::windows::{plan, section};
use crate::editor::bim::terminology::BimLabels;
use crate::editor::bim::BimDispatchCtx;
use crate::{Axis, Point2, View, ViewCamera, ViewKind, ViewPlane};
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "create-view")]
pub struct CreateView {
    pub kind: String,
    pub parent: String,
    pub name: String,
}

/// 🌍️ The distance in metres between a plane outside the building and the building, and the margin around it.
pub const MARGIN: f64 = 2.0;

fn axis_ends(axis: &Axis) -> [Point2; 2] {
    match axis {
        Axis::Line { start, end } | Axis::Arc { start, end, .. } => [*start, *end],
    }
}

fn half_thickness(snapshot: &ModelSnapshot, wall_type: &str) -> f64 {
    snapshot.wall_types.get(wall_type).map_or(0.0, |row| row.layers.iter().map(|layer| layer.thickness).sum::<f64>() / 2.0)
}

/// 📏️ The plan rectangle `[x0, y0, x1, y1]` of the authored elements of a building (wall, curtain wall, column, beam, slab, roof, railing, stair and ramp path points, component positions and MEP path points, walls grown by half their thickness), none for an empty building.
pub fn extents(snapshot: &ModelSnapshot, building: &str) -> Option<[f64; 4]> {
    let on = |storey: &String| snapshot.storeys.get(storey).is_some_and(|row| row.building == building);
    let mut points: Vec<Point2> = Vec::new();
    let mut grow = 0.0_f64;
    for wall in snapshot.walls.values().filter(|wall| on(&wall.storey)) {
        points.extend(axis_ends(&wall.axis));
        grow = grow.max(half_thickness(snapshot, &wall.wall_type));
    }
    points.extend(snapshot.curtain_walls.values().filter(|row| on(&row.storey)).flat_map(|row| axis_ends(&row.axis)));
    points.extend(snapshot.columns.values().filter(|row| on(&row.storey)).map(|row| row.position));
    points.extend(snapshot.beams.values().filter(|row| on(&row.storey)).flat_map(|row| axis_ends(&row.axis)));
    points.extend(snapshot.slabs.values().filter(|row| on(&row.storey)).flat_map(|row| row.boundary.iter().map(|vertex| vertex.point)));
    points.extend(snapshot.roofs.values().filter(|row| on(&row.storey)).flat_map(|row| row.footprint.iter().map(|vertex| vertex.point)));
    points.extend(snapshot.railings.values().filter(|row| on(&row.storey)).flat_map(|row| row.path.iter().copied()));
    points.extend(snapshot.stairs.values().filter(|row| on(&row.storey)).map(|row| row.start));
    points.extend(snapshot.ramps.values().filter(|row| on(&row.storey)).flat_map(|row| row.path.iter().map(|vertex| vertex.point)));
    points.extend(snapshot.components.values().filter(|row| on(&row.storey)).map(|row| row.position));
    points.extend(snapshot.mep_elements.values().filter(|row| on(&row.storey)).flat_map(|row| row.path.iter().map(|point| Point2 { x: point.x, y: point.y })));
    let first = *points.first()?;
    let [x0, y0, x1, y1] = points.iter().fold([first.x, first.y, first.x, first.y], |[x0, y0, x1, y1], p| [x0.min(p.x), y0.min(p.y), x1.max(p.x), y1.max(p.y)]);
    Some([x0 - grow, y0 - grow, x1 + grow, y1 + grow])
}

/// 🧭️ The side of a building a default elevation looks at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    South,
    East,
    North,
    West,
}

/// 🧭️ The four default sides, in the order they are created.
pub const SIDES: [Side; 4] = [Side::South, Side::East, Side::North, Side::West];

/// 🔭️ The plane and the depth of the elevation that looks at `side` of a building covering `rect`: the plane stands [`MARGIN`] metres outside the side and spans the side plus the margin on both ends, the viewer stands on the
/// right of the line and looks to its left, so the south elevation runs east along the south side and looks north. The depth reaches just behind the opposite side.
pub fn elevation_of(rect: [f64; 4], side: Side) -> (ViewPlane, f64) {
    let [x0, y0, x1, y1] = rect;
    let (west, east, south, north) = (x0 - MARGIN, x1 + MARGIN, y0 - MARGIN, y1 + MARGIN);
    let at = |x: f64, y: f64| Point2 { x, y };
    match side {
        Side::South => (ViewPlane { start: at(west, south), end: at(east, south) }, (y1 - y0) + MARGIN + 1.0),
        Side::East => (ViewPlane { start: at(east, south), end: at(east, north) }, (x1 - x0) + MARGIN + 1.0),
        Side::North => (ViewPlane { start: at(east, north), end: at(west, north) }, (y1 - y0) + MARGIN + 1.0),
        Side::West => (ViewPlane { start: at(west, north), end: at(west, south) }, (x1 - x0) + MARGIN + 1.0),
    }
}

fn word(labels: Option<&'static BimLabels>, pick: fn(&BimLabels) -> semio_framework_ui_locale::LabelText, fallback: &str) -> String {
    labels.map_or_else(|| fallback.to_string(), |labels| pick(labels).as_str().to_string())
}

fn side_name(labels: Option<&'static BimLabels>, side: Side) -> String {
    match side {
        Side::South => word(labels, |labels| labels.name_south, "South"),
        Side::East => word(labels, |labels| labels.name_east, "East"),
        Side::North => word(labels, |labels| labels.name_north, "North"),
        Side::West => word(labels, |labels| labels.name_west, "West"),
    }
}

fn building_of(snapshot: &ModelSnapshot, parent: &str, selected: &[String]) -> Option<String> {
    let of = |id: &str| -> Option<String> {
        snapshot
            .buildings
            .contains_key(id)
            .then(|| id.to_string())
            .or_else(|| snapshot.storeys.get(id).map(|storey| storey.building.clone()))
            .or_else(|| snapshot.views.get(id).map(|view| view.building.clone()))
            .or_else(|| crate::editor::bim::entities::storey_of(snapshot, id).and_then(|storey| snapshot.storeys.get(&storey).map(|row| row.building.clone())))
    };
    of(parent).or_else(|| selected.iter().find_map(|id| of(id))).or_else(|| snapshot.buildings.keys().next().cloned())
}

fn storey_of(snapshot: &ModelSnapshot, building: &str, parent: &str, selected: &[String]) -> Option<String> {
    let of = |id: &str| -> Option<String> { snapshot.storeys.contains_key(id).then(|| id.to_string()).or_else(|| crate::editor::bim::entities::storey_of(snapshot, id)) };
    of(parent).or_else(|| selected.iter().find_map(|id| of(id))).or_else(|| ordered_storeys(snapshot, building).into_iter().next())
}

fn camera_of(snapshot: &ModelSnapshot, building: &str, rect: [f64; 4]) -> ViewCamera {
    let height: f64 = snapshot.storeys.values().filter(|storey| storey.building == building && storey.level >= 0).map(|storey| storey.height).sum();
    let (width, depth) = (rect[2] - rect[0], rect[3] - rect[1]);
    ViewCamera { target: Point2 { x: (rect[0] + rect[2]) / 2.0, y: (rect[1] + rect[3]) / 2.0 }, target_height: height / 2.0, azimuth: std::f64::consts::FRAC_PI_4, pitch: 0.5, distance: 2.0 * width.max(depth).max(height) + 5.0 }
}

/// 🔭️ The views a request creates, as `(kind, name, view)` rows; `names` supplies the display word of a kind and `rect` the building rectangle (a default square for an empty building).
fn views_for(payload: &CreateView, snapshot: &ModelSnapshot, building: &str, storey: Option<&str>, labels: Option<&'static BimLabels>) -> Result<Vec<View>, Fault> {
    let rect = extents(snapshot, building).unwrap_or([0.0, 0.0, 10.0, 10.0]);
    let named = |fallback: String| if payload.name.trim().is_empty() { fallback } else { payload.name.clone() };
    let storey_row = || storey.and_then(|storey| snapshot.storeys.get(storey).map(|row| (storey, row))).ok_or_else(|| fault("bim.create.storey-missing", "a plan needs a storey"));
    match payload.kind.as_str() {
        "plan" => {
            let (storey, row) = storey_row()?;
            Ok(vec![View::of_storey(building, &named(pattern(labels, |labels| labels.view_plan_of, "Plan {name}", &row.name)), ViewKind::Plan, storey)])
        }
        "ceiling-plan" => {
            let (storey, row) = storey_row()?;
            Ok(vec![View::of_storey(building, &named(pattern(labels, |labels| labels.view_ceiling_of, "Ceiling plan {name}", &row.name)), ViewKind::CeilingPlan, storey)])
        }
        "section" => {
            let middle = (rect[1] + rect[3]) / 2.0;
            let plane = ViewPlane { start: Point2 { x: rect[0] - MARGIN, y: middle }, end: Point2 { x: rect[2] + MARGIN, y: middle } };
            Ok(vec![View { depth: (rect[3] - rect[1]) / 2.0 + MARGIN, ..View::through(building, &named(word(labels, |labels| labels.name_section, "Section")), ViewKind::Section, plane) }])
        }
        "elevation" | "elevations" => {
            let sides: &[Side] = if payload.kind == "elevations" { &SIDES } else { &SIDES[..1] };
            Ok(sides
                .iter()
                .map(|side| {
                    let (plane, depth) = elevation_of(rect, *side);
                    let fallback = side_name(labels, *side);
                    let name = if sides.len() == 1 { named(fallback) } else { fallback };
                    View { depth, ..View::through(building, &name, ViewKind::Elevation, plane) }
                })
                .collect())
        }
        "orthographic" | "perspective" => {
            let kind = if payload.kind == "orthographic" { ViewKind::Orthographic } else { ViewKind::Perspective };
            let fallback = word(labels, |labels| labels.name_camera, "3D view");
            Ok(vec![View { camera: Some(camera_of(snapshot, building, rect)), ..View::standard(building, &named(fallback), kind) }])
        }
        other => Err(fault("bim.view.kind-unknown", format!("'{other}' is not a kind of view"))),
    }
}

fn pattern(labels: Option<&'static BimLabels>, pick: fn(&BimLabels) -> semio_framework_ui_locale::LabelText, fallback: &str, name: &str) -> String {
    labels.map_or_else(|| fallback.replace("{name}", name), |labels| BimLabels::named(pick(labels), name))
}

pub fn handle(payload: &CreateView, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let building = building_of(snapshot, &payload.parent, &ctx.selected).ok_or_else(|| fault("bim.view.building-missing", "a view belongs to a building"))?;
    let storey = storey_of(snapshot, &building, &payload.parent, &ctx.selected);
    let views = views_for(payload, snapshot, &building, storey.as_deref(), ctx.labels())?;
    let mut mint = IdMint::new(doc.operation_optional());
    let mut taken = snapshot.clone();
    let mut mutations = Vec::new();
    let mut made = Vec::new();
    for mut view in views {
        view.name = crate::editor::bim::entities::views::unique_name(&taken, &building, &view.name);
        let id = mint.mint("view", |id| crate::editor::bim::entities::id_taken(snapshot, id));
        taken.views.insert(id.clone(), view.clone());
        made.push((view.kind, id.clone()));
        mutations.push(ModelMutation::CreateView(crate::mutations::create_view::CreateView { id, view }));
    }
    let mut emit = Emit::mutations(mutations);
    if let Some((kind, id)) = made.first() {
        emit.effects.push(select_effect(BIM_ELEMENT_DOMAIN, &[("view".to_string(), id.clone())], "replace"));
        if let Some(view) = ctx.view.clone() {
            if kind.is_plan() && ctx.window_kind == plan::WINDOW_KIND_ID {
                emit.window_config_mutations.push(plan::config::addressed(&view, plan::config::BimPlanWindowConfig { view: id.clone(), framed: false, ..ctx.plan.clone() })?);
            } else if kind.is_vertical() && ctx.window_kind == section::WINDOW_KIND_ID {
                emit.window_config_mutations.push(section::config::addressed(&view, section::config::BimSectionWindowConfig { view: id.clone(), framed: false, ..ctx.section.clone() })?);
            }
        }
    }
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
