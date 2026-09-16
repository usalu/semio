//! 🖱️ Ephemeral viewport marquee state and hit-testing for the Canvas2d model/results windows.

use super::{fem2d_hit_test, interaction_select_effect, Fem2dPick, FEM2D_GRANULARITY_ELEMENT, FEM2D_GRANULARITY_LOAD, FEM2D_GRANULARITY_NODE, FEM2D_GRANULARITY_REGION, FEM2D_GRANULARITY_SUPPORT};
use crate::editor::fem2d::modes::edit::windows::model::{fem2d_element_endpoints, find_node_2d, screen_2d};
use super::fem2d_load_glyph;
use crate::{element_id, Fem2dSnapshot, Viewport2d};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation, ViewModel};
use std::cell::RefCell;
use std::collections::HashMap;

type Fem2dMutation = crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;

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

pub fn clear_gesture(window_id: &str) {
    GESTURES.with(|store| {
        store.borrow_mut().remove(window_id);
    });
}

pub fn fem2d_active_utility(view: &ViewModel) -> &str {
    view.active_utility_id
        .as_deref()
        .or_else(|| view.window_id.as_ref().and_then(|id| view.active_utility_by_window_id.get(id).map(String::as_str)))
        .unwrap_or(FEM2D_UTILITY_SELECT_DIRECT)
}

pub fn fem2d_addressed_window_id(view: &ViewModel, fault: &str) -> Result<String, Fault> {
    view.window_id.clone().ok_or_else(|| Fault::from(format!("{fault}.window-required")))
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

pub fn fem2d_canvas_meta_utility_layer(utility: &str) -> dsl::json::Value {
    dsl::json!({
        "id": "meta:utility",
        "role": "meta",
        "utility": utility,
    })
}

pub fn fem2d_gesture_window_id_for_render(view: &ViewModel, body_key: &str) -> Option<String> {
    if let Some(id) = view.window_id.clone() {
        return Some(id);
    }
    let kind = match body_key {
        crate::editor::fem2d::modes::edit::windows::model::BODY_KEY => crate::editor::fem2d::modes::edit::windows::model::WINDOW_KIND_ID,
        crate::editor::fem2d::modes::edit::windows::results::BODY_KEY => crate::editor::fem2d::modes::edit::windows::results::WINDOW_KIND_ID,
        _ => return None,
    };
    view.window_instances.iter().find(|window| window.window_kind_id == kind).map(|window| window.id.clone())
}

pub fn fem2d_finish_canvas_layers_json(mut layers: Vec<dsl::json::Value>, window_instance_id: Option<&str>, active_utility: &str, gumball_meta: Option<dsl::json::Value>) -> String {
    layers.insert(0, fem2d_canvas_meta_utility_layer(active_utility));
    if let Some(meta) = gumball_meta {
        layers.insert(1, meta);
    }
    if let Some(window_id) = window_instance_id {
        layers.extend(fem2d_marquee_overlay_layers(&gesture_for_window(window_id)));
    }
    dsl::json::to_string(&dsl::json::Value::Array(layers))
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

pub fn fem2d_addressed_body_key(view: &ViewModel, fault: &str) -> Result<&'static str, Fault> {
    match super::fem2d_addressed_window_kind(view, fault)? {
        crate::editor::fem2d::modes::edit::windows::model::WINDOW_KIND_ID => Ok(crate::editor::fem2d::modes::edit::windows::model::BODY_KEY),
        crate::editor::fem2d::modes::edit::windows::results::WINDOW_KIND_ID => Ok(crate::editor::fem2d::modes::edit::windows::results::BODY_KEY),
        _ => Err(Fault::from(format!("{fault}.window-kind"))),
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

fn rect_from_points(points: &[(f64, f64)]) -> Option<(f64, f64, f64, f64)> {
    if points.len() < 2 {
        return None;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for (x, y) in points {
        min_x = min_x.min(*x);
        min_y = min_y.min(*y);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }
    Some((min_x, min_y, max_x, max_y))
}

fn point_in_rect(p: (f64, f64), min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> bool {
    p.0 >= min_x && p.0 <= max_x && p.1 >= min_y && p.1 <= max_y
}

fn rect_contains_point(min_x: f64, min_y: f64, max_x: f64, max_y: f64, p: (f64, f64)) -> bool {
    point_in_rect(p, min_x, min_y, max_x, max_y)
}

fn segment_intersects_rect(a: (f64, f64), b: (f64, f64), rect: (f64, f64, f64, f64)) -> bool {
    if point_in_rect(a, rect.0, rect.1, rect.2, rect.3) || point_in_rect(b, rect.0, rect.1, rect.2, rect.3) {
        return true;
    }
    let (min_x, min_y, max_x, max_y) = rect;
    let edges = [((min_x, min_y), (max_x, min_y)), ((max_x, min_y), (max_x, max_y)), ((max_x, max_y), (min_x, max_y)), ((min_x, max_y), (min_x, min_y))];
    edges.iter().any(|&(c, d)| segments_intersect(a, b, c, d))
}

fn segments_intersect(a: (f64, f64), b: (f64, f64), c: (f64, f64), d: (f64, f64)) -> bool {
    fn orient(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
        (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
    }
    fn on_segment(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
        c.0 <= a.0.max(b.0) && c.0 >= a.0.min(b.0) && c.1 <= a.1.max(b.1) && c.1 >= a.1.min(b.1)
    }
    let o1 = orient(a, b, c);
    let o2 = orient(a, b, d);
    let o3 = orient(c, d, a);
    let o4 = orient(c, d, b);
    if o1 == 0.0 && on_segment(a, b, c) {
        return true;
    }
    if o2 == 0.0 && on_segment(a, b, d) {
        return true;
    }
    if o3 == 0.0 && on_segment(c, d, a) {
        return true;
    }
    if o4 == 0.0 && on_segment(c, d, b) {
        return true;
    }
    o1.signum() != o2.signum() && o3.signum() != o4.signum()
}

fn even_odd_in_polygon(point: (f64, f64), polygon: &[(f64, f64)]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];
        let intersects = (yi > point.1) != (yj > point.1) && point.0 < (xj - xi) * (point.1 - yi) / (yj - yi + 1e-12) + xi;
        if intersects {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn entity_in_marquee(method: &str, canvas_points: &[(f64, f64)], enclosing: bool, probe: (f64, f64), segment: Option<((f64, f64), (f64, f64))>) -> bool {
    if method == "lasso" {
        if canvas_points.len() < 3 {
            return false;
        }
        if enclosing {
            return even_odd_in_polygon(probe, canvas_points);
        }
        if even_odd_in_polygon(probe, canvas_points) {
            return true;
        }
        return segment.is_some_and(|(a, b)| polygon_intersects_segment(canvas_points, a, b));
    }
    let Some(rect) = rect_from_points(canvas_points) else { return false };
    let rect_tuple = (rect.0, rect.1, rect.2, rect.3);
    if enclosing {
        if let Some((a, b)) = segment {
            return rect_contains_point(rect.0, rect.1, rect.2, rect.3, a) && rect_contains_point(rect.0, rect.1, rect.2, rect.3, b);
        }
        return rect_contains_point(rect.0, rect.1, rect.2, rect.3, probe);
    }
    if let Some((a, b)) = segment {
        return segment_intersects_rect(a, b, rect_tuple);
    }
    point_in_rect(probe, rect.0, rect.1, rect.2, rect.3)
}

fn polygon_intersects_segment(polygon: &[(f64, f64)], a: (f64, f64), b: (f64, f64)) -> bool {
    if polygon.len() < 2 {
        return false;
    }
    for window in polygon.windows(2) {
        if segments_intersect(a, b, window[0], window[1]) {
            return true;
        }
    }
    if polygon.len() >= 3 {
        let first = polygon[0];
        let last = *polygon.last().unwrap_or(&first);
        if segments_intersect(a, b, last, first) {
            return true;
        }
    }
    false
}

pub fn fem2d_marquee_hits(doc: &Fem2dSnapshot, camera: &Viewport2d, method: &str, canvas_points: &[(f64, f64)], width: f64, height: f64) -> Vec<Fem2dPick> {
    if canvas_points.len() < 2 {
        return Vec::new();
    }
    let start = canvas_points[0];
    let enclosing = selection_drag_enclosing(method, start, canvas_points);
    let layer_at = |model: (f64, f64)| super::fem2d_layer_to_canvas(camera, screen_2d(model.0, model.1), width, height);
    let mut hits: Vec<Fem2dPick> = Vec::new();
    let mut push = |pick: Fem2dPick| {
        if !hits.iter().any(|(_, id)| id == &pick.1) {
            hits.push(pick);
        }
    };

    for node in &doc.nodes {
        let probe = layer_at((node.x, node.y));
        if entity_in_marquee(method, canvas_points, enclosing, probe, None) {
            push((FEM2D_GRANULARITY_NODE, node.id.clone()));
        }
    }
    for support in &doc.supports {
        let Some(node) = find_node_2d(&doc.nodes, &support.node_id) else { continue };
        let probe = layer_at((node.x, node.y));
        if entity_in_marquee(method, canvas_points, enclosing, probe, None) {
            push((FEM2D_GRANULARITY_SUPPORT, support.id.clone()));
        }
    }
    for case in &doc.load_cases {
        for load in &case.loads {
            let Some((start_layer, end_layer)) = fem2d_load_glyph(doc, load) else { continue };
            let start_px = super::fem2d_layer_to_canvas(camera, start_layer, width, height);
            let end_px = super::fem2d_layer_to_canvas(camera, end_layer, width, height);
            if entity_in_marquee(method, canvas_points, enclosing, start_px, Some((start_px, end_px))) {
                push((FEM2D_GRANULARITY_LOAD, crate::load_id(load).to_string()));
            }
        }
    }
    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(a), Some(b)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let start_px = layer_at((a.x, a.y));
        let end_px = layer_at((b.x, b.y));
        if entity_in_marquee(method, canvas_points, enclosing, start_px, Some((start_px, end_px))) {
            push((FEM2D_GRANULARITY_ELEMENT, element_id(element).to_string()));
        }
    }
    for region in &doc.regions {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for [x, y] in &region.outline {
            let px = layer_at((*x, *y));
            min_x = min_x.min(px.0);
            min_y = min_y.min(px.1);
            max_x = max_x.max(px.0);
            max_y = max_y.max(px.1);
        }
        if min_x.is_finite() {
            let corners = [(min_x, min_y), (max_x, min_y), (max_x, max_y), (min_x, max_y)];
            let selected = if method == "lasso" {
                corners.iter().all(|corner| entity_in_marquee(method, canvas_points, enclosing, *corner, None))
                    || corners.iter().any(|corner| entity_in_marquee(method, canvas_points, false, *corner, None))
            } else {
                entity_in_marquee(method, canvas_points, enclosing, ((min_x + max_x) * 0.5, (min_y + max_y) * 0.5), Some(((min_x, min_y), (max_x, max_y))))
            };
            if selected {
                push((FEM2D_GRANULARITY_REGION, region.id.clone()));
            }
        }
    }
    hits.sort_by(|left, right| left.1.cmp(&right.1));
    hits
}

fn marquee_overlay_layer(id: &str, segments: Vec<dsl::json::Value>) -> dsl::json::Value {
    dsl::json!({
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

pub fn fem2d_marquee_overlay_layers(gesture: &Fem2dCanvasGesture) -> Vec<dsl::json::Value> {
    if !gesture.active || gesture.layer_points.len() < 2 {
        return Vec::new();
    }
    if gesture.method == "lasso" {
        let mut segments = Vec::with_capacity(gesture.layer_points.len() + 1);
        for (index, (x, y)) in gesture.layer_points.iter().enumerate() {
            segments.push(if index == 0 {
                dsl::json!({ "kind": "move", "to": [x, y] })
            } else {
                dsl::json!({ "kind": "line", "to": [x, y] })
            });
        }
        segments.push(dsl::json!({ "kind": "close" }));
        return vec![marquee_overlay_layer("overlay:marquee-lasso", segments)];
    }
    let (x0, y0) = gesture.layer_points[0];
    let (x1, y1) = *gesture.layer_points.last().unwrap_or(&(x0, y0));
    let x = x0.min(x1);
    let y = y0.min(y1);
    let width = (x1 - x0).abs();
    let height = (y1 - y0).abs();
    let segments = vec![
        dsl::json!({ "kind": "move", "to": [x, y] }),
        dsl::json!({ "kind": "line", "to": [x + width, y] }),
        dsl::json!({ "kind": "line", "to": [x + width, y + height] }),
        dsl::json!({ "kind": "line", "to": [x, y + height] }),
        dsl::json!({ "kind": "close" }),
    ];
    vec![marquee_overlay_layer("overlay:marquee-rect", segments)]
}
//#endregion 🔖️MarqueeGeometry

//#region 🔖️PointerHandlers
pub fn pointer_down(
    doc: &Fem2dSnapshot,
    camera: &Viewport2d,
    view: &ViewModel,
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
        let body = fem2d_addressed_body_key(view, "fem2d.canvas-pointer-down")?;
        return Ok(Emit { ui_scope: partial_window_refresh(body), ..Default::default() });
    }
    let hit = fem2d_hit_test(doc, camera, x, y, width, height);
    let merge = if hit.is_some() { merge } else { "replace" };
    let targets: Vec<Fem2dPick> = hit.into_iter().collect();
    Ok(Emit { effects: vec![interaction_select_effect(&targets, merge, "pick")], ..Default::default() })
}

/// 🧵️ One batched move (design L4): `samples` are canvas pixels, oldest first, never empty.
/// While a marquee/lasso is tracking EVERY sample joins the polyline (the lasso needs the path;
/// the rectangle only reads its last point) and the activation threshold is judged against the
/// LAST sample; the partial window refresh is emitted once per batch, not once per sample. When
/// nothing is tracking only the last sample is hover hit-tested — intermediate hovers are moot.
pub fn pointer_move(doc: &Fem2dSnapshot, camera: &Viewport2d, view: &ViewModel, samples: &[(f64, f64)], width: f64, height: f64) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let window_id = fem2d_addressed_window_id(view, "fem2d.canvas-pointer-move")?;
    let Some(&(x, y)) = samples.last() else { return Ok(Emit::default()) };
    let mut gesture = gesture_for_window(&window_id);
    if gesture.tracking {
        for &(sx, sy) in samples {
            let layer = canvas_to_layer(camera, sx, sy, width, height);
            gesture.canvas_points.push((sx, sy));
            gesture.layer_points.push(layer);
        }
        if !gesture.active {
            let start = gesture.canvas_points[0];
            let distance = ((x - start.0).powi(2) + (y - start.1).powi(2)).sqrt();
            if distance >= FEM2D_MARQUEE_THRESHOLD_PX {
                gesture.active = true;
            }
        }
        set_gesture(&window_id, gesture.clone());
        let body = fem2d_addressed_body_key(view, "fem2d.canvas-pointer-move")?;
        return Ok(Emit { ui_scope: partial_window_refresh(body), ..Default::default() });
    }
    let targets: Vec<Fem2dPick> = fem2d_hit_test(doc, camera, x, y, width, height).into_iter().collect();
    Ok(Emit { effects: vec![super::interaction_hover_effect(&targets)], ..Default::default() })
}

/// 🖱️ Closes the gesture. `cancelled` (pointer left the canvas / capture lost — design §2 D) drops
/// any in-flight marquee/lasso and emits ONLY the window refresh: no `interactionSelect`, no pick.
pub fn pointer_up(
    doc: &Fem2dSnapshot,
    camera: &Viewport2d,
    view: &ViewModel,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    merge: &str,
    cancelled: bool,
) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let window_id = fem2d_addressed_window_id(view, "fem2d.canvas-pointer-up")?;
    let gesture = gesture_for_window(&window_id);
    clear_gesture(&window_id);
    let body = fem2d_addressed_body_key(view, "fem2d.canvas-pointer-up")?;
    if cancelled || !gesture.tracking {
        return Ok(Emit { ui_scope: partial_window_refresh(body), ..Default::default() });
    }
    if gesture.active {
        let targets = fem2d_marquee_hits(doc, camera, &gesture.method, &gesture.canvas_points, width, height);
        let merge = if targets.is_empty() { "replace" } else { merge };
        return Ok(Emit {
            effects: vec![interaction_select_effect(&targets, merge, &gesture.method)],
            ui_scope: partial_window_refresh(body),
            ..Default::default()
        });
    }
    let hit = fem2d_hit_test(doc, camera, x, y, width, height);
    let merge = if hit.is_some() { merge } else { "replace" };
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
