//! 🗺️ BIM plan window: the architectural floor plan of one authored plan view on a `Canvas2d` surface. It paints the `view-linework` inference of the configured view (the plan of its storey cut at the height the view resolves) (cut walls as
//! poché, projections, hidden lines, openings, stairs, grid, space tags) with the selection and hover highlighted by element id, and answers a pointer press by picking the
//! topmost element under it. Nothing here computes geometry: a view without linework paints nothing.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

use self::config::BimPlanWindowConfig;
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::kit::{canvas_surface, meta_record, path, path_record, text_record, Paint, Rgba};
use crate::editor::bim::terminology::BimLabels;
use crate::render::plan::{canvas_point, plan_point};
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanLinework, PlanVertex};
use crate::{ModelInference, ModelSnapshot, ViewKind};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::Point;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "bim-edit-plan";
pub const BODY_KEY: &str = "bim.edit.plan";
const SURFACE_ID: &str = "bim.edit.plan2d/plan";
const HOVERED: Rgba = [0.36, 0.65, 0.98, 1.0];
/// 📏️ The chord tolerance, in metres, arcs are flattened to for the hover overlay and the pick.
const FLATTEN_TOLERANCE: f64 = crate::render::plan::FLATTEN_TOLERANCE;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: crate::editor::bim::utilities::initial(),
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native(BimLabels::NATIVE_EN.window_plan.as_str(), BimLabels::NATIVE_DE.window_plan.as_str()),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "layout-panel-top".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: crate::editor::bim::utilities::for_window(WINDOW_KIND_ID),
        interactions: vec![InteractionRef::new(BIM_ELEMENT_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️View
/// 🗺️ Every plan and ceiling plan view of the model in browser order: by building, then the level of its storey, plans before ceiling plans, then name.
pub fn plan_views(snapshot: &ModelSnapshot) -> Vec<String> {
    let mut rows: Vec<(&String, i32, bool, &String, &String)> = snapshot
        .views
        .iter()
        .filter(|(_, view)| view.kind.is_plan())
        .filter_map(|(id, view)| view.storey.as_ref().and_then(|storey| snapshot.storeys.get(storey)).map(|storey| (&view.building, storey.level, view.kind == ViewKind::CeilingPlan, &view.name, id)))
        .collect();
    rows.sort();
    rows.into_iter().map(|row| row.4.clone()).collect()
}

/// 🗺️ The plan view a plan window shows: its configured one while it still is a plan or ceiling plan view, else the first one in browser order; none when the model has no plan view.
pub fn active_view(snapshot: &ModelSnapshot, config: &BimPlanWindowConfig) -> Option<String> {
    if snapshot.views.get(&config.view).is_some_and(|view| view.kind.is_plan() && view.storey.as_ref().is_some_and(|storey| snapshot.storeys.contains_key(storey))) {
        return Some(config.view.clone());
    }
    plan_views(snapshot).into_iter().next()
}

/// 🪜️ The storey a plan window cuts: the storey of its view, so every drawing tool places on it.
pub fn active_storey(snapshot: &ModelSnapshot, config: &BimPlanWindowConfig) -> Option<String> {
    active_view(snapshot, config).and_then(|id| snapshot.views.get(&id)).and_then(|view| view.storey.clone())
}
//#endregion 🔖️View

//#region 🔖️Geometry
/// ➰️ The flattened points of a bulged vertex list, mirrored into the window's space; a closed list repeats no closing point.
fn flatten(vertices: &[PlanVertex], closed: bool) -> Vec<[f64; 2]> {
    let point = |vertex: &PlanVertex| Point { x: vertex.x, y: vertex.y };
    let Some(first) = vertices.first() else { return Vec::new() };
    let mut points = vec![point(first)];
    let edges = if closed { vertices.len() } else { vertices.len().saturating_sub(1) };
    for index in 0..edges {
        let (from, to) = (&vertices[index], &vertices[(index + 1) % vertices.len()]);
        BulgeSeg::new(point(from), point(to), from.bulge).flatten_into(FLATTEN_TOLERANCE, &mut points);
    }
    if closed && points.len() > 1 {
        points.pop();
    }
    points.iter().map(|p| canvas_point(p.x, p.y)).collect()
}

fn tinted(colour: Rgba, alpha: f64) -> Rgba {
    [colour[0], colour[1], colour[2], alpha]
}

/// 🔦️ Whether a hover mark outlines `element`: a selected element is already drawn in the accent colour.
fn hovered(element: &str, selection: &[String], hover: &[String]) -> bool {
    hover.iter().any(|id| id == element) && !selection.iter().any(|id| id == element)
}
//#endregion 🔖️Geometry

//#region 🔖️Render
/// 🎨️ The records of one plan in paint order: the shared layers (selection in the accent colour), the hover outlines, the utility meta record.
pub fn records(plan: &PlanLinework, selection: &[String], hover: &[String], utility: &str) -> Vec<DslValue> {
    let mut records = vec![meta_record(utility)];
    records.extend(crate::render::plan::layers(plan, selection));
    for region in plan.regions.iter().filter(|region| hovered(&region.element, selection, hover)) {
        let mut segments = path(&flatten(&region.outer, true), true);
        for hole in &region.holes {
            segments.extend(path(&flatten(hole, true), true));
        }
        records.push(path_record(&format!("hover:{}", region.id), "overlay", segments, Paint { fill: Some(tinted(HOVERED, 0.35)), stroke: Some((HOVERED, 0.05)), dash: None }));
    }
    for line in plan.polylines.iter().filter(|line| hovered(&line.element, selection, hover)) {
        records.push(path_record(&format!("hover:{}", line.id), "overlay", path(&flatten(&line.vertices, line.closed), line.closed), Paint { fill: None, stroke: Some((HOVERED, 0.05)), dash: None }));
    }
    records
}

/// 🗺️ Renders the plan window: the configured storey's linework, or an empty surface when the model has no storey or no linework yet.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimPlanWindowConfig, selection: &[String], hover: &[String], utility: &str, revision: u32, labels: &BimLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    render_over(snapshot, inference, config, selection, hover, utility, revision, labels, &[])
}

/// 🗺️ [`render`] with the records of the authoring overlay (handles, snap markers, the gesture preview) painted last, over everything.
#[allow(clippy::too_many_arguments)]
pub fn render_over(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimPlanWindowConfig, selection: &[String], hover: &[String], utility: &str, revision: u32, labels: &BimLabels, overlay: &[DslValue]) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let view = active_view(snapshot, config);
    let plan = view.as_ref().and_then(|view| inference.view_linework.get(view)).map(|drawing| &drawing.lines);
    let mut records = plan.map_or_else(|| vec![meta_record(utility)], |plan| records(plan, selection, hover, utility));
    if view.is_none() {
        records.push(text_record("empty", (0.0, 0.0), labels.empty_plan.as_str(), crate::render::plan::TEXT_SIZE * 2.0, [0.22, 0.24, 0.28, 1.0]));
    }
    records.extend(overlay.iter().cloned());
    let framing = plan.filter(|_| !config.framed).and_then(|plan| crate::render::plan::canvas_bounds(plan).map(|bounds| semio_framework_plugin::Canvas2dFraming { revision, bounds, padding: 48.0 }));
    canvas_surface(SURFACE_ID, config.viewport, framing, &records)
}
//#endregion 🔖️Render

//#region 🔖️Pick
/// 🖱️ Converts a canvas pixel to the plan's model coordinates (the exact inverse of the host's `screenToWorldLogical`, then the mirror).
pub fn pixel_to_model(viewport: &store::Viewport2d, x: f64, y: f64, width: f64, height: f64) -> (f64, f64) {
    let zoom = viewport.zoom.max(0.01);
    ((x - width * 0.5) / zoom + viewport.x, -((y - height * 0.5) / zoom + viewport.y))
}

fn distance_to_segment(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let length_squared = dx * dx + dy * dy;
    let t = if length_squared == 0.0 { 0.0 } else { (((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / length_squared).clamp(0.0, 1.0) };
    ((p[0] - (a[0] + t * dx)).powi(2) + (p[1] - (a[1] + t * dy)).powi(2)).sqrt()
}

fn inside(ring: &[[f64; 2]], p: [f64; 2]) -> bool {
    let mut crossings = false;
    for index in 0..ring.len() {
        let (a, b) = (ring[index], ring[(index + 1) % ring.len()]);
        if (a[1] > p[1]) != (b[1] > p[1]) && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0] {
            crossings = !crossings;
        }
    }
    crossings
}

/// 🎯️ The element of `plan` under the model point `at` within `tolerance` metres: a cut region containing it wins, else the nearest stroked line.
pub fn pick(plan: &PlanLinework, at: (f64, f64), tolerance: f64) -> Option<String> {
    let p = [at.0, at.1];
    let model = |vertices: &[PlanVertex], closed: bool| flatten(vertices, closed).into_iter().map(|[x, y]| plan_point(x, y)).collect::<Vec<_>>();
    let hit = plan.regions.iter().find(|region| {
        let ring = model(&region.outer, true);
        inside(&ring, p) && !region.holes.iter().any(|hole| inside(&model(hole, true), p))
    });
    if let Some(region) = hit {
        return Some(region.element.clone());
    }
    plan.polylines
        .iter()
        .filter(|line| !line.element.is_empty())
        .filter_map(|line| {
            let points = model(&line.vertices, line.closed);
            let edges = if line.closed { points.len() } else { points.len().saturating_sub(1) };
            let nearest = (0..edges).map(|index| distance_to_segment(p, points[index], points[(index + 1) % points.len()])).fold(f64::INFINITY, f64::min);
            (nearest <= tolerance).then(|| (nearest, line.element.clone()))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, element)| element)
}
//#endregion 🔖️Pick

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
