//! 🖱️ Ephemeral viewport marquee state and hit-testing for the Canvas2d model/results windows.

use super::fem2d_hit_test;

use super::interaction_select_effect;

use super::Fem2dPick;

use super::FEM2D_GRANULARITY_ELEMENT;

use super::FEM2D_GRANULARITY_LOAD;

use super::FEM2D_GRANULARITY_NODE;

use super::FEM2D_GRANULARITY_REGION;

use super::FEM2D_GRANULARITY_SUPPORT;
use super::fem2d_element_endpoints;
use super::find_node_2d;
use super::screen_2d;
use super::fem2d_load_glyph;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_framework_os_kernel::Viewport2d;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::ViewModel;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use semio_framework_geometry::geom_sel;
use semio_framework_geometry::Point;

type Fem2dMutation = semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;

//#region 🔖️Constants
pub const FEM2D_UTILITY_SELECT_DIRECT: &str = "selectDirect";
pub const FEM2D_UTILITY_SELECT_MARQUEE: &str = "selectMarquee";
pub const FEM2D_UTILITY_SELECT_LASSO: &str = "selectLasso";
pub const FEM2D_UTILITY_PAN: &str = "transformMove";
pub const FEM2D_UTILITY_TRANSFORM: &str = "transform";

pub const FEM2D_MARQUEE_THRESHOLD_PX: f64 = 4.0;
const SELECTION_DRAG_DIRECTION_THRESHOLD_PX: f64 = 4.0;
//#endregion 🔖️Constants

//#region 🔖️GestureStore
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fem2dCanvasGesture {
    pub tracking: bool,
    pub active: bool,
    pub method: String,
    pub canvas_points: Vec<(f64, f64)>,
    pub layer_points: Vec<(f64, f64)>,
}

thread_local! {
    static GESTURES: RefCell<HashMap<String, Fem2dCanvasGesture>> = RefCell::new(HashMap::new());
}

pub fn gesture_for_window(window_id: &str) -> Fem2dCanvasGesture {
    GESTURES.with(|store| store.borrow().get(window_id).cloned().unwrap_or_default())
}

fn set_gesture(window_id: &str, gesture: Fem2dCanvasGesture) {
    GESTURES.with(|store| {
        store.borrow_mut().insert(window_id.to_string(), gesture);
    });
}

fn take_gesture(window_id: &str) -> Fem2dCanvasGesture {
    GESTURES.with(|store| store.borrow_mut().remove(window_id).unwrap_or_default())
}

fn append_gesture_samples(gesture: &mut Fem2dCanvasGesture, samples: &[(f64, f64)], camera: &Viewport2d, width: f64, height: f64) {
    let lasso = gesture.method == "lasso";
    let retained = gesture.canvas_points.len();
    semio_framework::interaction::gesture::append_selection_drag_points(&mut gesture.canvas_points, &mut gesture.active, lasso, samples, FEM2D_MARQUEE_THRESHOLD_PX);
    let begin = if lasso { retained } else { gesture.layer_points.clear(); 0 };
    for &(x, y) in gesture.canvas_points.iter().skip(begin) {
        gesture.layer_points.push(canvas_to_layer(camera, x, y, width, height));
    }
}

pub fn clear_gesture(window_id: &str) {
    GESTURES.with(|store| {
        store.borrow_mut().remove(window_id);
    });
}

pub fn fem2d_active_utility(view: &ViewModel) -> &str {
    view.window_utility(None).unwrap_or(FEM2D_UTILITY_SELECT_DIRECT)
}

pub fn fem2d_addressed_window_id(view: &ViewModel, fault: &str) -> Result<String, Fault> {
    view.window_id.clone().ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("fem.canvas.window-required"), format!("{fault}: the gesture names no window")))
}

fn fem2d_selection_method_for_utility(utility: &str) -> Option<&'static str> {
    match utility {
        FEM2D_UTILITY_SELECT_MARQUEE => Some("rectangle"),
        FEM2D_UTILITY_SELECT_LASSO => Some("lasso"),
        _ => None,
    }
}

pub fn fem2d_is_marquee_utility(utility: &str) -> bool {
    fem2d_selection_method_for_utility(utility).is_some()
}

const FEM2D_MARQUEE_STROKE: [f64; 4] = [0.22, 0.74, 0.97, 0.9];
const FEM2D_MARQUEE_FILL: [f64; 4] = [0.22, 0.74, 0.97, 0.12];

pub fn fem2d_canvas_meta_utility_layer(utility: &str) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::json!({
        "id": "meta:utility",
        "role": "meta",
        "utility": utility,
    })
}

pub fn fem2d_gesture_window_id_for_render(view: &ViewModel, window_kind_id: &str) -> Option<String> {
    if let Some(id) = view.window_id.clone() {
        return Some(id);
    }
    view.window_instances.iter().find(|window| window.window_kind_id == window_kind_id).map(|window| window.id.clone())
}

pub fn fem2d_finish_canvas_layers_json(mut layers: Vec<semio_framework_pack_json::Value>, window_instance_id: Option<&str>, active_utility: &str, gumball_meta: Option<semio_framework_pack_json::Value>) -> String {
    layers.insert(0, fem2d_canvas_meta_utility_layer(active_utility));
    if let Some(meta) = gumball_meta {
        layers.insert(1, meta);
    }
    if let Some(window_id) = window_instance_id {
        layers.extend(fem2d_marquee_overlay_layers(&gesture_for_window(window_id)));
    }
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(layers))
}

fn canvas_to_layer(camera: &Viewport2d, x: f64, y: f64, width: f64, height: f64) -> (f64, f64) {
    let zoom = if camera.zoom.abs() < 1e-9 { 1.0 } else { camera.zoom };
    ((x - width * 0.5) / zoom + camera.x, (y - height * 0.5) / zoom + camera.y)
}

fn partial_window_refresh(body_key: &str) -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: vec![body_key.to_owned()],
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    }
}

//#endregion 🔖️GestureStore

//#region 🔖️MarqueeGeometry
fn selection_drag_enclosing(method: &str, start: (f64, f64), points: &[(f64, f64)]) -> bool {
    if method == "lasso" {
        for point in points.iter().skip(1) {
            let dx = point.0 - start.0;
            if dx.abs() < SELECTION_DRAG_DIRECTION_THRESHOLD_PX {
                continue;
            }
            return dx > 0.0;
        }
        return false;
    }
    points.last().copied().unwrap_or(start).0 >= start.0
}

fn selection_polygon(method: &str, points: &[(f64, f64)]) -> Vec<Point> {
    if method == "lasso" {
        return points.iter().map(|&(x, y)| Point::new(x, y)).collect();
    }
    let (Some(&start), Some(&end)) = (points.first(), points.last()) else { return Vec::new() };
    vec![Point::new(start.0, start.1), Point::new(end.0, start.1), Point::new(end.0, end.1), Point::new(start.0, end.1)]
}

fn entity_in_marquee(polygon: &[Point], enclosing: bool, probe: (f64, f64), segment: Option<((f64, f64), (f64, f64))>) -> bool {
    if let Some((a, b)) = segment {
        let (a, b) = (Point::new(a.0, a.1), Point::new(b.0, b.1));
        return if enclosing { geom_sel::polygon_contains_segment(polygon, a, b) } else { geom_sel::segment_intersects_polygon(a, b, polygon) };
    }
    geom_sel::polygon_covers_point(polygon, Point::new(probe.0, probe.1))
}
pub fn fem2d_marquee_hits(doc: &Fem2dSnapshot, camera: &Viewport2d, method: &str, canvas_points: &[(f64, f64)], width: f64, height: f64) -> Result<Vec<Fem2dPick>, protocol::wire::ScopedIdentityError> {
    if canvas_points.len() < 2 {
        return Ok(Vec::new());
    }
    let polygon = selection_polygon(method, canvas_points);
    let start = canvas_points[0];
    let enclosing = selection_drag_enclosing(method, start, canvas_points);
    let layer_at = |model: (f64, f64)| super::fem2d_layer_to_canvas(camera, screen_2d(model.0, model.1), width, height);
    let mut hits: Vec<Fem2dPick> = Vec::new();
    let mut seen = HashSet::new();
    let mut push = |pick: Fem2dPick| {
        if seen.insert(pick.clone()) {
            hits.push(pick);
        }
    };

    for node in &doc.nodes {
        let probe = layer_at((node.x, node.y));
        if entity_in_marquee(&polygon, enclosing, probe, None) {
            push((FEM2D_GRANULARITY_NODE, super::fem2d_entity_target_id(FEM2D_GRANULARITY_NODE, &node.id)?));
        }
    }
    for support in &doc.supports {
        let Some(node) = find_node_2d(&doc.nodes, &support.node_id) else { continue };
        let probe = layer_at((node.x, node.y));
        if entity_in_marquee(&polygon, enclosing, probe, None) {
            push((FEM2D_GRANULARITY_SUPPORT, super::fem2d_entity_target_id(FEM2D_GRANULARITY_SUPPORT, &support.id)?));
        }
    }
    for case in &doc.load_cases {
        for load in &case.loads {
            let Some((start_layer, end_layer)) = fem2d_load_glyph(doc, load) else { continue };
            let start_px = super::fem2d_layer_to_canvas(camera, start_layer, width, height);
            let end_px = super::fem2d_layer_to_canvas(camera, end_layer, width, height);
            if entity_in_marquee(&polygon, enclosing, start_px, Some((start_px, end_px))) {
                push((FEM2D_GRANULARITY_LOAD, super::fem2d_load_target_id(&case.id, semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::load_id(load))?));
            }
        }
    }
    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(a), Some(b)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let start_px = layer_at((a.x, a.y));
        let end_px = layer_at((b.x, b.y));
        if entity_in_marquee(&polygon, enclosing, start_px, Some((start_px, end_px))) {
            push((FEM2D_GRANULARITY_ELEMENT, super::fem2d_entity_target_id(FEM2D_GRANULARITY_ELEMENT, element_id(element))?));
        }
    }
    for region in &doc.regions {
        let project = |points: &[[f64; 2]]| points.iter().map(|&[x, y]| { let p = layer_at((x, y)); Point::new(p.0, p.1) }).collect::<Vec<_>>();
        let outline = project(&region.outline);
        let selected = if enclosing {
            geom_sel::polygon_contains_polygon(&polygon, &outline)
        } else {
            let holes = region.holes.iter().map(|hole| project(hole)).collect::<Vec<_>>();
            geom_sel::polygon_region_intersects_polygon(&outline, &holes, &polygon)
        };
        if selected {
            push((FEM2D_GRANULARITY_REGION, super::fem2d_entity_target_id(FEM2D_GRANULARITY_REGION, &region.id)?));
        }
    }
    hits.sort_by(|left, right| left.1.cmp(&right.1));
    Ok(hits)
}

fn marquee_overlay_layer(id: &str, segments: Vec<semio_framework_pack_json::Value>) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::json!({
        "id": id,
        "role": "overlay",
        "transform": [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        "segments": segments,
        "fill": { "color": FEM2D_MARQUEE_FILL },
        "stroke": { "color": FEM2D_MARQUEE_STROKE, "width": 1.5, "cap": "round", "join": "round" },
        "fillRule": "evenodd",
        "opacity": 1.0,
        "blendMode": "normal",
        "visible": true,
    })
}

pub fn fem2d_marquee_overlay_layers(gesture: &Fem2dCanvasGesture) -> Vec<semio_framework_pack_json::Value> {
    if !gesture.active || gesture.layer_points.len() < 2 {
        return Vec::new();
    }
    if gesture.method == "lasso" {
        let mut segments = Vec::with_capacity(gesture.layer_points.len() + 1);
        for (index, (x, y)) in gesture.layer_points.iter().enumerate() {
            segments.push(if index == 0 {
                semio_framework_pack_json::json!({ "kind": "move", "to": [x, y] })
            } else {
                semio_framework_pack_json::json!({ "kind": "line", "to": [x, y] })
            });
        }
        segments.push(semio_framework_pack_json::json!({ "kind": "close" }));
        return vec![marquee_overlay_layer("overlay:marquee-lasso", segments)];
    }
    let (x0, y0) = gesture.layer_points[0];
    let (x1, y1) = *gesture.layer_points.last().unwrap_or(&(x0, y0));
    let x = x0.min(x1);
    let y = y0.min(y1);
    let width = (x1 - x0).abs();
    let height = (y1 - y0).abs();
    let segments = vec![
        semio_framework_pack_json::json!({ "kind": "move", "to": [x, y] }),
        semio_framework_pack_json::json!({ "kind": "line", "to": [x + width, y] }),
        semio_framework_pack_json::json!({ "kind": "line", "to": [x + width, y + height] }),
        semio_framework_pack_json::json!({ "kind": "line", "to": [x, y + height] }),
        semio_framework_pack_json::json!({ "kind": "close" }),
    ];
    vec![marquee_overlay_layer("overlay:marquee-rect", segments)]
}
//#endregion 🔖️MarqueeGeometry

//#region 🔖️PointerHandlers
pub fn pointer_down(
    doc: &Fem2dSnapshot,
    camera: &Viewport2d,
    view: &ViewModel,
    body: &str,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    button: u32,
    merge: &str,
) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    if button != 0 {
        return Ok(Emit::default());
    }
    let window_id = fem2d_addressed_window_id(view, "fem2d.canvas-pointer-down")?;
    let utility = fem2d_active_utility(view);
    if utility == FEM2D_UTILITY_PAN || utility == FEM2D_UTILITY_TRANSFORM {
        return Ok(Emit::default());
    }
    if let Some(method) = fem2d_selection_method_for_utility(utility) {
        let layer = canvas_to_layer(camera, x, y, width, height);
        set_gesture(
            &window_id,
            Fem2dCanvasGesture { tracking: true, active: false, method: method.to_string(), canvas_points: vec![(x, y)], layer_points: vec![layer] },
        );
                return Ok(Emit { ui_scope: partial_window_refresh(body), ..Default::default() });
    }
    let hit = fem2d_hit_test(doc, camera, x, y, width, height).map_err(|_| Fault::from("fem2d.interaction.identity"))?;
    let targets: Vec<Fem2dPick> = hit.into_iter().collect();
    Ok(Emit { effects: vec![interaction_select_effect(&targets, merge, "pick")], ..Default::default() })
}

/// 🧵️ One batched move (design L4): `samples` are canvas pixels, oldest first, never empty.
/// Lasso retains distinct samples; rectangle retains only the start and latest endpoint.
/// Activation latches when any sample crosses the threshold. The partial refresh is emitted once per batch. When
/// nothing is tracking only the last sample is hover hit-tested — intermediate hovers are moot.
pub fn pointer_move(doc: &Fem2dSnapshot, camera: &Viewport2d, view: &ViewModel, body: &str, samples: &[(f64, f64)], width: f64, height: f64) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let window_id = fem2d_addressed_window_id(view, "fem2d.canvas-pointer-move")?;
    let Some(&(x, y)) = samples.last() else { return Ok(Emit::default()) };
    let tracking = GESTURES.with(|store| {
        let mut store = store.borrow_mut();
        let Some(gesture) = store.get_mut(&window_id).filter(|gesture| gesture.tracking) else { return false };
        append_gesture_samples(gesture, samples, camera, width, height);
        true
    });
    if tracking {
        return Ok(Emit { ui_scope: partial_window_refresh(body), ..Default::default() });
    }
    let targets: Vec<Fem2dPick> = fem2d_hit_test(doc, camera, x, y, width, height).map_err(|_| Fault::from("fem2d.interaction.identity"))?.into_iter().collect();
    Ok(Emit { effects: vec![super::interaction_hover_effect(&targets)], ..Default::default() })
}

/// 🖱️ Closes the gesture. `cancelled` (pointer left the canvas / capture lost — design §2 D) drops
/// any in-flight marquee/lasso and emits ONLY the window refresh: no `interactionSelect`, no pick.
pub fn pointer_up(
    doc: &Fem2dSnapshot,
    camera: &Viewport2d,
    view: &ViewModel,
    body: &str,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    merge: &str,
    cancelled: bool,
) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let window_id = fem2d_addressed_window_id(view, "fem2d.canvas-pointer-up")?;
    let mut gesture = take_gesture(&window_id);
    if cancelled || !gesture.tracking {
        return Ok(Emit { ui_scope: partial_window_refresh(body), ..Default::default() });
    }
    append_gesture_samples(&mut gesture, &[(x, y)], camera, width, height);
    if gesture.active {
        let targets = fem2d_marquee_hits(doc, camera, &gesture.method, &gesture.canvas_points, width, height).map_err(|_| Fault::from("fem2d.interaction.identity"))?;
        return Ok(Emit {
            effects: vec![interaction_select_effect(&targets, merge, &gesture.method)],
            ui_scope: partial_window_refresh(body),
            ..Default::default()
        });
    }
    let hit = fem2d_hit_test(doc, camera, x, y, width, height).map_err(|_| Fault::from("fem2d.interaction.identity"))?;
    let targets: Vec<Fem2dPick> = hit.into_iter().collect();
    Ok(Emit {
        effects: vec![interaction_select_effect(&targets, merge, "pick")],
        ui_scope: partial_window_refresh(body),
        ..Default::default()
    })
}
//#endregion 🔖️PointerHandlers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
