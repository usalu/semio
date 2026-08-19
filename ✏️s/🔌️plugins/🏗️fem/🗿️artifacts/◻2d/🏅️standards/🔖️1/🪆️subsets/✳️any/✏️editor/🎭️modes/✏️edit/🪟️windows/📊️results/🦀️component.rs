//! 📊️ Fem2d play app — the results window: static/modal/buckling analysis views, nodal-averaged
//! von-Mises stress contours, reaction labels and moment diagrams.

use crate::editor::fem2d::modes::edit::windows::model::{
    fem2d_deformed_shape_layers, fem2d_element_endpoints, fem2d_model_extent, fem2d_region_mesh_triangles, fem2d_structure_layers, find_node_2d, screen_2d, MOMENT_SCALE_2D,
};
use crate::artifacts::fem2d::{element_id, Fem2dSnapshot, FemCamera};
use crate::app_surface::{hex_to_rgb01, normalize_mode_shape, DisplayMode, ResultDisplay, MODE_SHAPE_AMPLITUDE_RATIO, VON_MISES_BANDS};
use crate::model::ElementResult;
use semio_framework_plugin::{build_canvas_2d_scene, ui_text, Canvas2dScene, Label, UiNode};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "fem2d-results";
pub const BODY_KEY: &str = "fem2d.play.results";
//#endregion 🔖️Constants

//#region 🔖️StressContourHelpers
/// 🌡️ A filled-triangle Canvas2d path layer (`segments` + `fill`, evenodd) for a contour cell —
/// see `framework/renderer/react/components/canvas-2d-host.tsx`'s `buildScenePath`/`drawSceneNode`
/// for the exact JSON shape this mirrors.
async fn filled_triangle_layer(id: String, p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), color: &str, alpha: f64) -> Value {
    let (r, g, b) = hex_to_rgb01(color);
    json!({
        "id": id,
        "transform": [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        "segments": [
            { "kind": "move", "to": [p0.0, p0.1] },
            { "kind": "line", "to": [p1.0, p1.1] },
            { "kind": "line", "to": [p2.0, p2.1] },
            { "kind": "close" },
        ],
        "fill": { "color": [r, g, b, alpha] },
    })
}

/// 🌡️ A filled polygon Canvas2d path layer (arbitrary vertex count) — the marching-triangle contour
/// bands need this (a clipped triangle can come out as a quad), unlike `filled_triangle_layer`'s
/// fixed 3-point shape.
async fn filled_polygon_layer(id: String, points: &[(f64, f64)], color: &str, alpha: f64) -> Value {
    let (r, g, b) = hex_to_rgb01(color);
    let mut segments = Vec::with_capacity(points.len() + 1);
    for (i, &(x, y)) in points.iter().enumerate() {
        segments.push(if i == 0 { json!({ "kind": "move", "to": [x, y] }) } else { json!({ "kind": "line", "to": [x, y] }) });
    }
    segments.push(json!({ "kind": "close" }));
    json!({
        "id": id,
        "transform": [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        "segments": segments,
        "fill": { "color": [r, g, b, alpha] },
    })
}

/// ✂️ One point of a polygon being clipped for contour banding: screen position plus the (linearly
/// interpolated across the source triangle) scalar value driving the clip.
type ValuedPoint = ((f64, f64), f64);

/// ✂️ Interpolates the crossing point where the segment `a->b`'s value equals `threshold`.
async fn interpolate_at_value(a: ValuedPoint, b: ValuedPoint, threshold: f64) -> ValuedPoint {
    let t = if (b.1 - a.1).abs() < 1e-12 { 0.5 } else { (threshold - a.1) / (b.1 - a.1) };
    ((a.0 .0 + (b.0 .0 - a.0 .0) * t, a.0 .1 + (b.0 .1 - a.0 .1) * t), threshold)
}

/// ✂️ Sutherland-Hodgman clip of a (convex, value-carrying) polygon against a scalar half-plane —
/// keeps the portion where `value >= threshold` (`keep_above`) or `value <= threshold` (else),
/// inserting an interpolated vertex at every edge crossing. The core of marching-triangle contour
/// banding: clipping a triangle's linear value field against 2 thresholds bands it into one polygon.
async fn clip_by_value(poly: &[ValuedPoint], threshold: f64, keep_above: bool) -> Vec<ValuedPoint> {
    if poly.is_empty() {
        return Vec::new();
    }
    let n = poly.len();
    let mut out = Vec::with_capacity(n + 1);
    for i in 0..n {
        let cur = poly[i];
        let prev = poly[(i + n - 1) % n];
        let cur_in = if keep_above { cur.1 >= threshold } else { cur.1 <= threshold };
        let prev_in = if keep_above { prev.1 >= threshold } else { prev.1 <= threshold };
        if cur_in {
            if !prev_in {
                out.push(interpolate_at_value(prev, cur, threshold));
            }
            out.push(cur);
        } else if prev_in {
            out.push(interpolate_at_value(prev, cur, threshold));
        }
    }
    out
}

/// 🌡️ A stress-contour legend: a small vertical stack of `VON_MISES_BANDS` swatches plus min/max text
/// labels, anchored near the canvas origin.
async fn von_mises_legend_layers(min: f64, max: f64) -> Vec<Value> {
    let mut layers = Vec::with_capacity(VON_MISES_BANDS.len() + 2);
    for (i, color) in VON_MISES_BANDS.iter().enumerate() {
        let y = 20.0 + i as f64 * 14.0;
        layers.push(filled_triangle_layer(format!("legend-swatch-{i}-a"), (10.0, y), (26.0, y), (26.0, y + 14.0), color, 1.0));
        layers.push(filled_triangle_layer(format!("legend-swatch-{i}-b"), (10.0, y), (26.0, y + 14.0), (10.0, y + 14.0), color, 1.0));
    }
    layers.push(json!({
        "id": "legend-label-min",
        "transform": [1.0, 0.0, 0.0, 1.0, 30.0, 20.0 + VON_MISES_BANDS.len() as f64 * 14.0],
        "text": { "content": format!("{min:.1} Pa"), "size": 11.0 },
    }));
    layers.push(json!({
        "id": "legend-label-max",
        "transform": [1.0, 0.0, 0.0, 1.0, 30.0, 28.0],
        "text": { "content": format!("{max:.1} Pa"), "size": 11.0 },
    }));
    layers
}
//#endregion 🔖️StressContourHelpers

//#region 🔖️Render
/// 📊️ Results window dispatcher — picks the static/modal/buckling render based on `display`.
pub async fn render(doc: &Fem2dSnapshot, display: &ResultDisplay, camera: &FemCamera) -> UiNode {
    match display.mode {
        DisplayMode::Static => render_static(doc, display.source_id.as_deref(), camera),
        DisplayMode::Modal(mode_index) => render_modal(doc, mode_index, camera),
        DisplayMode::Buckling(mode_index) => render_buckling(doc, display.source_id.as_deref(), mode_index, camera),
    }
}

/// 📊️ Static results: undeformed structure faintly, plus a deformed-shape polyline, text labels at
/// every support reaction, (for beams) a moment-diagram polyline, and (for meshed regions) a
/// nodal-averaged, marching-triangle-banded von-Mises stress contour with a color-swatch legend.
/// `source_id` selects a `fem2d_solve_all` case/combination id, falling back to the first load case
/// when `None`/unknown (preserves v0's default behavior).
async fn render_static(doc: &Fem2dSnapshot, source_id: Option<&str>, camera: &FemCamera) -> UiNode {
    let results = match crate::fem2d_engine::fem2d_solve_all(doc) {
        Ok(results) => results,
        Err(e) => return ui_text(Label::data(format!("Analysis error: {e}"))),
    };
    let case_id = source_id.filter(|id| results.contains_key(*id)).map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone()));
    let Some(case_id) = case_id else {
        return ui_text(Label::data("No load case defined"));
    };
    let Some(result) = results.get(&case_id) else {
        return ui_text(Label::data(format!("Result not found: {case_id}")));
    };

    let mut layers = fem2d_structure_layers(doc, "#334155", "#334155", "#334155");
    let mut disp_map: HashMap<String, [f64; 6]> = HashMap::new();
    for d in &result.displacements {
        disp_map.insert(d.node_id.clone(), d.values);
    }
    layers.extend(fem2d_deformed_shape_layers(doc, &disp_map, doc.analysis.deformation_scale));

    //#region 🔖️ReactionLabels
    for reaction in &result.reactions {
        let Some(node) = find_node_2d(&doc.nodes, &reaction.node_id) else { continue };
        let (sx, sy) = screen_2d(node.x, node.y);
        layers.push(json!({
            "id": format!("reaction-{}-{:?}", reaction.node_id, reaction.dof),
            "transform": [1.0, 0.0, 0.0, 1.0, sx + 8.0, sy + 14.0],
            "text": { "content": format!("{:?}: {:.0} N", reaction.dof, reaction.value), "size": 10.0 },
        }));
    }
    //#endregion 🔖️ReactionLabels

    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(n1), Some(n2)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let (x0, y0) = screen_2d(n1.x, n1.y);
        let (x1, y1) = screen_2d(n2.x, n2.y);
        if let Some((_, ElementResult::Beam { stations })) = result.elements.iter().find(|(id, _)| id.as_str() == element_id(element)) {
            let model_length = ((n2.x - n1.x).powi(2) + (n2.y - n1.y).powi(2)).sqrt().max(1e-9);
            let dx = x1 - x0;
            let dy = y1 - y0;
            let len = (dx * dx + dy * dy).sqrt().max(1e-9);
            let (px, py) = (-dy / len, dx / len);
            let points: Vec<[f64; 2]> = stations
                .iter()
                .map(|s| {
                    let t = s.x / model_length;
                    let bx = x0 + dx * t;
                    let by = y0 + dy * t;
                    [bx + px * s.m * MOMENT_SCALE_2D, by + py * s.m * MOMENT_SCALE_2D]
                })
                .collect();
            layers.push(json!({
                "kind": "polyline",
                "id": format!("moment-{}", element_id(element)),
                "points": points,
                "color": "#fbbf24",
            }));
        }
    }

    //#region 🔖️StressContour
    let nodal_von_mises = crate::fem2d_engine::mesh_preview::fem2d_nodal_von_mises(doc, &case_id).unwrap_or_default();
    let mesh_triangles = fem2d_region_mesh_triangles(doc);
    let mut valued_triangles: Vec<([(f64, f64); 3], [f64; 3])> = Vec::new();
    for (_, tri_points, node_ids) in &mesh_triangles {
        if let (Some(&v0), Some(&v1), Some(&v2)) = (nodal_von_mises.get(&node_ids[0]), nodal_von_mises.get(&node_ids[1]), nodal_von_mises.get(&node_ids[2])) {
            valued_triangles.push((*tri_points, [v0, v1, v2]));
        }
    }
    if !valued_triangles.is_empty() {
        let min = valued_triangles.iter().flat_map(|(_, v)| v.iter().copied()).fold(f64::INFINITY, f64::min);
        let max = valued_triangles.iter().flat_map(|(_, v)| v.iter().copied()).fold(f64::NEG_INFINITY, f64::max);
        let n_bands = VON_MISES_BANDS.len();
        let span = (max - min).max(1e-9);
        let boundaries: Vec<f64> = (0..=n_bands).map(|k| min + span * k as f64 / n_bands as f64).collect();
        for (tri_index, (points, values)) in valued_triangles.iter().enumerate() {
            let base: Vec<ValuedPoint> = points.iter().zip(values.iter()).map(|(&p, &v)| (p, v)).collect();
            for band in 0..n_bands {
                let mut poly = base.clone();
                if band > 0 {
                    poly = clip_by_value(&poly, boundaries[band], true);
                }
                if band < n_bands - 1 {
                    poly = clip_by_value(&poly, boundaries[band + 1], false);
                }
                if poly.len() >= 3 {
                    let screen_points: Vec<(f64, f64)> = poly.iter().map(|(p, _)| *p).collect();
                    layers.push(filled_polygon_layer(format!("contour-{tri_index}-{band}"), &screen_points, VON_MISES_BANDS[band], 0.85));
                }
            }
        }
        layers.extend(von_mises_legend_layers(min, max));
    }
    //#endregion 🔖️StressContour

    let layers_json = serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into());
    build_canvas_2d_scene(BODY_KEY, crate::editor::fem2d::FEM2D_APP_ID, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json })
}

/// 📊️ Modal mode-shape overlay: undeformed structure faintly plus the selected mode's deformed-shape
/// polyline (normalized to unit peak, then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own
/// extent — see `normalize_mode_shape`) and a frequency caption.
async fn render_modal(doc: &Fem2dSnapshot, mode_index: usize, camera: &FemCamera) -> UiNode {
    let (freq_hz, mut disp_map) = match crate::fem2d_engine::modal_buckling::fem2d_modal_mode_values(doc, mode_index) {
        Ok(values) => values,
        Err(e) => return ui_text(Label::data(format!("Modal analysis error: {e}"))),
    };
    normalize_mode_shape(&mut disp_map);
    let mut layers = fem2d_structure_layers(doc, "#334155", "#334155", "#334155");
    layers.extend(fem2d_deformed_shape_layers(doc, &disp_map, fem2d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO));
    layers.push(json!({
        "id": "modal-caption",
        "transform": [1.0, 0.0, 0.0, 1.0, 10.0, 20.0],
        "text": { "content": format!("Mode {}: {freq_hz:.3} Hz", mode_index + 1), "size": 12.0 },
    }));
    let layers_json = serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into());
    build_canvas_2d_scene(BODY_KEY, crate::editor::fem2d::FEM2D_APP_ID, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json })
}

/// 📊️ Buckling mode-shape overlay: undeformed structure faintly plus the selected mode's deformed-shape
/// polyline (normalized to unit peak, then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own
/// extent — see `normalize_mode_shape`) and a load-factor caption. `source_id` selects the reference
/// load case, falling back to the first load case when `None`.
async fn render_buckling(doc: &Fem2dSnapshot, source_id: Option<&str>, mode_index: usize, camera: &FemCamera) -> UiNode {
    let Some(case_id) = source_id.map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone())) else {
        return ui_text(Label::data("No load case defined"));
    };
    let (factor, mut disp_map) = match crate::fem2d_engine::modal_buckling::fem2d_buckling_mode_values(doc, &case_id, mode_index) {
        Ok(values) => values,
        Err(e) => return ui_text(Label::data(format!("Buckling analysis error: {e}"))),
    };
    normalize_mode_shape(&mut disp_map);
    let mut layers = fem2d_structure_layers(doc, "#334155", "#334155", "#334155");
    layers.extend(fem2d_deformed_shape_layers(doc, &disp_map, fem2d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO));
    layers.push(json!({
        "id": "buckling-caption",
        "transform": [1.0, 0.0, 0.0, 1.0, 10.0, 20.0],
        "text": { "content": format!("Buckling mode {}: factor {factor:.3}", mode_index + 1), "size": 12.0 },
    }));
    let layers_json = serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into());
    build_canvas_2d_scene(BODY_KEY, crate::editor::fem2d::FEM2D_APP_ID, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem2d::testkit::{dispatch, fem2d_app, render as render_body};
    use crate::editor::fem2d::Fem2dCommand;

    async fn load_default_example(app: &mut crate::editor::fem2d::testkit::Fem2dApp) {
        dispatch(app, Fem2dCommand::SetActiveExample(crate::editor::fem2d::commands::set_active_example::SetActiveExample { example_id: "default".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_fem2d_results_scene() {
        let mut app = fem2d_app();
        load_default_example(&mut app);
        assert!(render_body(&mut app, BODY_KEY).contains("canvas-2d"));
    }

    #[semio_framework_async_macros::async_test]
    async fn results_window_surfaces_solver_error_without_panicking_2d() {
        let mut app = fem2d_app();
        let _ = render_body(&mut app, BODY_KEY);
    }

    #[semio_framework_async_macros::async_test]
    async fn results_window_buckling_with_no_load_case_shows_placeholder_2d() {
        let doc = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let display = ResultDisplay { source_id: None, mode: DisplayMode::Buckling(0) };
        let camera = FemCamera::default();
        let json = serde_json::to_string(&render(&doc, &display, &camera)).unwrap();
        assert!(json.contains("No load case defined"), "{json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn results_window_renders_contour_for_region() {
        let mut app = fem2d_app();
        load_default_example(&mut app);
        dispatch(&mut app, Fem2dCommand::SetResultDisplay(crate::editor::fem2d::commands::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "static".into(), mode_index: 0 }));
        let json = render_body(&mut app, BODY_KEY);
        // `layers_json` is itself a JSON string embedded inside `UiNode`'s own serialization, so its
        // quotes come out backslash-escaped in `json` — match on the unescaped substrings instead.
        assert!(json.contains("fill"), "expected filled-path contour layers for the region's Tri3Cst elements: {json}");
        assert!(json.contains("contour-"), "expected contour-prefixed layer ids: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn results_window_renders_reaction_labels_2d() {
        let mut app = fem2d_app();
        load_default_example(&mut app);
        dispatch(&mut app, Fem2dCommand::SetResultDisplay(crate::editor::fem2d::commands::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "static".into(), mode_index: 0 }));
        let json = render_body(&mut app, BODY_KEY);
        assert!(json.contains("reaction-"), "expected reaction-prefixed text label layers: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn results_window_renders_modal_mode_shape_2d() {
        let mut app = fem2d_app();
        load_default_example(&mut app);
        dispatch(&mut app, Fem2dCommand::SetResultDisplay(crate::editor::fem2d::commands::set_result_display::SetResultDisplay { source_id: None, mode: "modal".into(), mode_index: 0 }));
        let json = render_body(&mut app, BODY_KEY);
        assert!(json.contains("canvas-2d"), "expected a valid canvas-2d scene, got: {json}");
        assert!(!json.contains("Modal analysis error"), "unexpected modal error: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn results_window_renders_buckling_mode_shape_2d() {
        let mut app = fem2d_app();
        load_default_example(&mut app);
        dispatch(&mut app, Fem2dCommand::SetResultDisplay(crate::editor::fem2d::commands::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "buckling".into(), mode_index: 0 }));
        let json = render_body(&mut app, BODY_KEY);
        assert!(json.contains("canvas-2d"), "expected a valid canvas-2d scene, got: {json}");
        assert!(!json.contains("Buckling analysis error"), "unexpected buckling error: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn interpolate_at_value_falls_back_to_midpoint_when_values_equal() {
        let (point, value) = interpolate_at_value(((0.0, 0.0), 5.0), ((10.0, 20.0), 5.0), 5.0);
        assert_eq!(point, (5.0, 10.0));
        assert_eq!(value, 5.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn clip_by_value_empty_polygon_returns_empty() {
        assert!(clip_by_value(&[], 0.0, true).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn clip_by_value_keeps_only_the_requested_half_plane() {
        let poly: Vec<ValuedPoint> = vec![((0.0, 0.0), 0.0), ((10.0, 0.0), 10.0), ((0.0, 10.0), 0.0)];
        let above = clip_by_value(&poly, 5.0, true);
        assert!(above.len() >= 3 && above.iter().all(|(_, v)| *v >= 5.0 - 1e-9));
        let below = clip_by_value(&poly, 5.0, false);
        assert!(below.len() >= 3 && below.iter().all(|(_, v)| *v <= 5.0 + 1e-9));
    }
}
//#endregion 🧪️Tests
