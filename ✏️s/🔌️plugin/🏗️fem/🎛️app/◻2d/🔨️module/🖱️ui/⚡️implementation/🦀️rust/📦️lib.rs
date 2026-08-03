//! 🖼️ FEM 2D app — `DocumentApp` impl, render, manifest (constitutional: ui). B1: the pure-trait
//! pilot — `Fem2dPlayApp` is a unit struct; every former `Fem2dPlayApp` `RefCell` field (result
//! display, camera) plus the deleted `ViewState::locale` now live in `fem2d_engine::Fem2dConfig`,
//! written via `fem2d_op::Fem2dConfigOperation`s (real `backwards`, no ad hoc `InverseAction`); every
//! action dispatches through the single typed `fem2d_protocol::Fem2dCommand` channel via
//! `DocumentApp::handle`.

use fem2d::{Fem2dDocument, FemCamera};
use fem2d_engine::Fem2dConfig;
use fem2d_op::{Fem2dConfigOperation, Fem2dOperation};
use fem2d_protocol::Fem2dCommand;
use fem_core::{Dof, ElementResult};
use fem_shared::{hex_to_rgb01, next_id, normalize_mode_shape, result_display_action_args, DisplayMode, ResultDisplay, MODE_SHAPE_AMPLITUDE_RATIO, VON_MISES_BANDS};
use semio_framework_plugin::{
    build_canvas_2d_scene, create_default_layout, ui_text, ActionArgDef, ActionArgOption, App, AppIo, ArtifactKindSpec, Canvas2dScene, ConfigSpec, ConfigView, DocumentApp, DocumentView, Emit, Label, LocalizedLabel, Media, MediaClass, MediaError,
    MediaForm, MediaPayload, MediaType, OsMediaCapability, SurfaceKind, UiNode,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use store::DocumentDsl;

//#region 🔖️Constants
const FEM2D_APP_ID: &str = "fem2d-play";
const FEM2D_WINDOW_MODEL: &str = "fem2d-model";
const FEM2D_WINDOW_RESULTS: &str = "fem2d-results";
const FEM2D_BODY_MODEL: &str = "fem2d.play.model";
const FEM2D_BODY_RESULTS: &str = "fem2d.play.results";

/// 📦️ The `fem2d-play` "default" example — shared by the manifest's `.example(...)` registration, the
/// `setActiveExample` handler, and every test fixture. See `fem2d_dsl`'s `🔖️Dsl` region.
const FEM2D_EXAMPLE_DSL: &str = fem2d_dsl::FEM2D_EXAMPLE_TEXT;

/// 📐️ Model-meters -> screen-pixels scale for the 2D canvas (a 6m span shouldn't render as 6px wide).
const SCALE_2D: f64 = 20.0;
/// 📐️ Screen-space origin offset so a structure anchored at (0,0) isn't drawn at the canvas corner.
const ORIGIN_2D: f64 = 40.0;
/// 📐️ Exaggeration factor for offsetting the moment-diagram polyline perpendicular to a member.
const MOMENT_SCALE_2D: f64 = 0.001;

/// 🎨️ Muted color for the mesh-edge preview overlay drawn under the model window's members.
const MESH_EDGE_COLOR: &str = "#475569";
//#endregion 🔖️Constants

//#region 🔖️StressContourHelpers
/// 🌡️ A filled-triangle Canvas2d path layer (`segments` + `fill`, evenodd) for a contour cell —
/// see `framework/renderer/react/components/canvas-2d-host.tsx`'s `buildScenePath`/`drawSceneNode`
/// for the exact JSON shape this mirrors.
fn filled_triangle_layer(id: String, p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), color: &str, alpha: f64) -> Value {
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
fn filled_polygon_layer(id: String, points: &[(f64, f64)], color: &str, alpha: f64) -> Value {
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
fn interpolate_at_value(a: ValuedPoint, b: ValuedPoint, threshold: f64) -> ValuedPoint {
    let t = if (b.1 - a.1).abs() < 1e-12 { 0.5 } else { (threshold - a.1) / (b.1 - a.1) };
    ((a.0 .0 + (b.0 .0 - a.0 .0) * t, a.0 .1 + (b.0 .1 - a.0 .1) * t), threshold)
}

/// ✂️ Sutherland-Hodgman clip of a (convex, value-carrying) polygon against a scalar half-plane —
/// keeps the portion where `value >= threshold` (`keep_above`) or `value <= threshold` (else),
/// inserting an interpolated vertex at every edge crossing. The core of marching-triangle contour
/// banding: clipping a triangle's linear value field against 2 thresholds bands it into one polygon.
fn clip_by_value(poly: &[ValuedPoint], threshold: f64, keep_above: bool) -> Vec<ValuedPoint> {
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
fn von_mises_legend_layers(min: f64, max: f64) -> Vec<Value> {
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

//#region 🔖️Fem2dRender
fn screen_2d(x: f64, y: f64) -> (f64, f64) {
    (x * SCALE_2D + ORIGIN_2D, -y * SCALE_2D + ORIGIN_2D)
}

fn find_node_2d<'a>(nodes: &'a [fem2d::FemNode], id: &str) -> Option<&'a fem2d::FemNode> {
    nodes.iter().find(|n| n.id == id)
}

fn fem2d_element_endpoints(element: &fem2d::FemElement) -> (&str, &str) {
    match element {
        fem2d::FemElement::Bar { start, end, .. } | fem2d::FemElement::Beam { start, end, .. } => (start.as_str(), end.as_str()),
    }
}

/// 🔎️ Finds the load case an incoming load/self-weight edit should target: the named `case_id` if it
/// exists, else the first case, else a freshly synthesized `"case-1"` — shared by `addNodalLoad`,
/// `addMemberUdl`, `addAreaLoad`, and `setSelfWeight` so every load-mutating action resolves its
/// target case the same way. Returns the case's collection index (`load_cases.len()` for a fresh one)
/// alongside an owned clone ready to be mutated and re-emitted via `SetLoadCase`.
fn fem2d_resolve_load_case(doc: &Fem2dDocument, case_id: Option<&str>) -> (usize, fem2d::FemLoadCase) {
    let named = case_id.and_then(|id| doc.load_cases.iter().find(|lc| lc.id == id).cloned());
    let load_case = named.or_else(|| doc.load_cases.first().cloned()).unwrap_or_else(|| fem2d::FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: Vec::new(), self_weight: false });
    let index = doc.load_cases.iter().position(|lc| lc.id == load_case.id).unwrap_or(doc.load_cases.len());
    (index, load_case)
}

/// 📐️ Bounding-box diagonal (in model meters) over every node plus every region outline vertex — the
/// reference length `MODE_SHAPE_AMPLITUDE_RATIO` scales a normalized mode shape against. Falls back to
/// `1.0` for a degenerate (empty or point-like) model so mode-shape rendering never divides by zero.
fn fem2d_model_extent(doc: &Fem2dDocument) -> f64 {
    let mut min = [f64::INFINITY; 2];
    let mut max = [f64::NEG_INFINITY; 2];
    let mut expand = |x: f64, y: f64| {
        min[0] = min[0].min(x);
        min[1] = min[1].min(y);
        max[0] = max[0].max(x);
        max[1] = max[1].max(y);
    };
    for node in &doc.nodes {
        expand(node.x, node.y);
    }
    for region in &doc.regions {
        for p in &region.outline {
            expand(p[0], p[1]);
        }
    }
    if min[0] > max[0] {
        return 1.0;
    }
    let d = [max[0] - min[0], max[1] - min[1]];
    (d[0] * d[0] + d[1] * d[1]).sqrt().max(1.0)
}

/// 🖼️ Nodes/members/supports as Canvas2d layers — shared by the model window (bright colors) and the
/// results window's faint undeformed backdrop (a single muted color for every layer kind).
fn fem2d_structure_layers(doc: &Fem2dDocument, node_color: &str, line_color: &str, support_color: &str) -> Vec<Value> {
    let mut layers = Vec::new();
    for node in &doc.nodes {
        let (sx, sy) = screen_2d(node.x, node.y);
        layers.push(json!({ "kind": "circle", "id": format!("node-{}", node.id), "x": sx - 4.0, "y": sy - 4.0, "width": 8.0, "height": 8.0, "color": node_color }));
    }
    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        if let (Some(n1), Some(n2)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) {
            let (x0, y0) = screen_2d(n1.x, n1.y);
            let (x1, y1) = screen_2d(n2.x, n2.y);
            layers.push(json!({ "kind": "line", "id": format!("el-{}", fem2d::element_id(element)), "x0": x0, "y0": y0, "x1": x1, "y1": y1, "color": line_color }));
        }
    }
    for support in &doc.supports {
        if let Some(node) = find_node_2d(&doc.nodes, &support.node_id) {
            let (sx, sy) = screen_2d(node.x, node.y);
            layers.push(json!({ "kind": "circle", "id": format!("support-{}", support.id), "x": sx - 5.0, "y": sy - 5.0, "width": 10.0, "height": 10.0, "color": support_color }));
        }
    }
    layers
}

/// 🗺️ Every meshed region's triangles as `(element_id, [screen_p0, screen_p1, screen_p2])` — the
/// element id matches `fem2d_solve`/`fem2d_solve_all`'s `Tri3Cst` ids (`"{region_id}_t{tri_index}"`),
/// so callers can correlate a solved `ElementResult::Plane` back to on-screen triangle geometry. A
/// mesh failure for one region silently yields fewer triangles rather than failing the whole render.
fn fem2d_region_triangles(doc: &Fem2dDocument) -> Vec<(String, [(f64, f64); 3])> {
    let mut out = Vec::new();
    let Ok(meshes) = fem2d_engine::fem2d_mesh_preview(doc) else { return out };
    for mesh in &meshes {
        for (tri_index, tri) in mesh.tris.iter().enumerate() {
            let id = format!("{}_t{}", mesh.region_id, tri_index);
            let p0 = mesh.points[tri[0] as usize];
            let p1 = mesh.points[tri[1] as usize];
            let p2 = mesh.points[tri[2] as usize];
            out.push((id, [screen_2d(p0[0], p0[1]), screen_2d(p1[0], p1[1]), screen_2d(p2[0], p2[1])]));
        }
    }
    out
}

/// 🗺️ Every meshed region's triangles as `(element_id, screen points, node ids)` — like
/// `fem2d_region_triangles` but also carrying each vertex's mesh node id, needed to look values up in
/// `fem2d_nodal_von_mises`'s node-keyed map for banded contour rendering.
fn fem2d_region_mesh_triangles(doc: &Fem2dDocument) -> Vec<(String, [(f64, f64); 3], [String; 3])> {
    let mut out = Vec::new();
    let Ok(meshes) = fem2d_engine::fem2d_mesh_preview(doc) else { return out };
    for mesh in &meshes {
        for (tri_index, tri) in mesh.tris.iter().enumerate() {
            let id = format!("{}_t{}", mesh.region_id, tri_index);
            let p0 = mesh.points[tri[0] as usize];
            let p1 = mesh.points[tri[1] as usize];
            let p2 = mesh.points[tri[2] as usize];
            let node_ids = [mesh.node_ids[tri[0] as usize].clone(), mesh.node_ids[tri[1] as usize].clone(), mesh.node_ids[tri[2] as usize].clone()];
            out.push((id, [screen_2d(p0[0], p0[1]), screen_2d(p1[0], p1[1]), screen_2d(p2[0], p2[1])], node_ids));
        }
    }
    out
}

/// 🖼️ Every element's deformed-shape polyline (pink), given a node-id-keyed displacement map and a
/// display scale — shared by the static, modal, and buckling results renders.
fn fem2d_deformed_shape_layers(doc: &Fem2dDocument, disp_map: &HashMap<String, [f64; 6]>, deform_scale: f64) -> Vec<Value> {
    let mut layers = Vec::new();
    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(n1), Some(n2)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let (x0, y0) = screen_2d(n1.x, n1.y);
        let (x1, y1) = screen_2d(n2.x, n2.y);
        let d1 = disp_map.get(&n1.id).copied().unwrap_or([0.0; 6]);
        let d2 = disp_map.get(&n2.id).copied().unwrap_or([0.0; 6]);
        let dx0 = d1[Dof::Tx.index()] * deform_scale * SCALE_2D;
        let dy0 = -d1[Dof::Ty.index()] * deform_scale * SCALE_2D;
        let dx1 = d2[Dof::Tx.index()] * deform_scale * SCALE_2D;
        let dy1 = -d2[Dof::Ty.index()] * deform_scale * SCALE_2D;
        layers.push(json!({
            "kind": "polyline",
            "id": format!("deformed-{}", fem2d::element_id(element)),
            "points": [[x0 + dx0, y0 + dy0], [x1 + dx1, y1 + dy1]],
            "color": "#f472b6",
        }));
    }
    layers
}

fn render_fem2d_model(doc: &Fem2dDocument, camera: &FemCamera) -> UiNode {
    let mut layers = fem2d_structure_layers(doc, "#38bdf8", "#94a3b8", "#f97316");
    for (tri_index, (_, tri)) in fem2d_region_triangles(doc).iter().enumerate() {
        let [(x0, y0), (x1, y1), (x2, y2)] = *tri;
        layers.push(json!({
            "kind": "polyline",
            "id": format!("mesh-edge-{tri_index}"),
            "points": [[x0, y0], [x1, y1], [x1, y1], [x2, y2], [x2, y2], [x0, y0]],
            "color": MESH_EDGE_COLOR,
        }));
    }
    let layers_json = serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into());
    build_canvas_2d_scene(FEM2D_BODY_MODEL, FEM2D_APP_ID, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json })
}

/// 📊️ Results window dispatcher — picks the static/modal/buckling render based on `display`.
fn render_fem2d_results(doc: &Fem2dDocument, display: &ResultDisplay, camera: &FemCamera) -> UiNode {
    match display.mode {
        DisplayMode::Static => render_fem2d_results_static(doc, display.source_id.as_deref(), camera),
        DisplayMode::Modal(mode_index) => render_fem2d_results_modal(doc, mode_index, camera),
        DisplayMode::Buckling(mode_index) => render_fem2d_results_buckling(doc, display.source_id.as_deref(), mode_index, camera),
    }
}

/// 📊️ Static results: undeformed structure faintly, plus a deformed-shape polyline, text labels at
/// every support reaction, (for beams) a moment-diagram polyline, and (for meshed regions) a
/// nodal-averaged, marching-triangle-banded von-Mises stress contour with a color-swatch legend.
/// `source_id` selects a `fem2d_solve_all` case/combination id, falling back to the first load case
/// when `None`/unknown (preserves v0's default behavior).
fn render_fem2d_results_static(doc: &Fem2dDocument, source_id: Option<&str>, camera: &FemCamera) -> UiNode {
    let results = match fem2d_engine::fem2d_solve_all(doc) {
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
        if let Some((_, ElementResult::Beam { stations })) = result.elements.iter().find(|(id, _)| id.as_str() == fem2d::element_id(element)) {
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
                "id": format!("moment-{}", fem2d::element_id(element)),
                "points": points,
                "color": "#fbbf24",
            }));
        }
    }

    //#region 🔖️StressContour
    let nodal_von_mises = fem2d_engine::fem2d_nodal_von_mises(doc, &case_id).unwrap_or_default();
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
    build_canvas_2d_scene(FEM2D_BODY_RESULTS, FEM2D_APP_ID, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json })
}

/// 📊️ Modal mode-shape overlay: undeformed structure faintly plus the selected mode's deformed-shape
/// polyline (normalized to unit peak, then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own
/// extent — see `normalize_mode_shape`) and a frequency caption.
fn render_fem2d_results_modal(doc: &Fem2dDocument, mode_index: usize, camera: &FemCamera) -> UiNode {
    let (freq_hz, mut disp_map) = match fem2d_engine::fem2d_modal_mode_values(doc, mode_index) {
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
    build_canvas_2d_scene(FEM2D_BODY_RESULTS, FEM2D_APP_ID, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json })
}

/// 📊️ Buckling mode-shape overlay: undeformed structure faintly plus the selected mode's deformed-shape
/// polyline (normalized to unit peak, then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own
/// extent — see `normalize_mode_shape`) and a load-factor caption. `source_id` selects the reference
/// load case, falling back to the first load case when `None`.
fn render_fem2d_results_buckling(doc: &Fem2dDocument, source_id: Option<&str>, mode_index: usize, camera: &FemCamera) -> UiNode {
    let Some(case_id) = source_id.map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone())) else {
        return ui_text(Label::data("No load case defined"));
    };
    let (factor, mut disp_map) = match fem2d_engine::fem2d_buckling_mode_values(doc, &case_id, mode_index) {
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
    build_canvas_2d_scene(FEM2D_BODY_RESULTS, FEM2D_APP_ID, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json })
}
//#endregion 🔖️Fem2dRender

//#region 🔖️Fem2dPlayApp
/// 👁️ B1: `cfg`-driven counterpart of the deleted `ResultDisplay` `RefCell` — converts the flat
/// `Fem2dConfig` result-display fields back into `fem_shared::ResultDisplay`/`DisplayMode` so the
/// existing `render_fem2d_results` pipeline (built around those shared types) needs no changes.
fn config_result_display(cfg: &Fem2dConfig) -> ResultDisplay {
    let mode = match cfg.result_mode.as_str() {
        "modal" => DisplayMode::Modal(cfg.result_mode_index as usize),
        "buckling" => DisplayMode::Buckling(cfg.result_mode_index as usize),
        _ => DisplayMode::Static,
    };
    ResultDisplay { source_id: cfg.result_source_id.clone(), mode }
}

/// 🎨️ Manual `fem_core::StaticResult` -> JSON bridge for `"results:out"` (see
/// `Fem2dPlayApp::export_media`) — `fem_core::StaticResult`/`ElementResult`/`Dof` don't derive
/// `Serialize` (out of this ticket's scope: `🫀️core` is a shared crate), so this hand-rolls the same
/// shape `serde_json::to_string` would have produced, using `Dof`'s existing `{:?}` formatting (already
/// used for the reaction-label layers in `render_fem2d_results_static` above).
fn dof_json(dof: Dof) -> Value {
    json!(format!("{dof:?}"))
}

fn element_result_json(result: &ElementResult) -> Value {
    match result {
        ElementResult::Bar { n } => json!({ "kind": "bar", "n": n }),
        ElementResult::Beam { stations } => {
            json!({ "kind": "beam", "stations": stations.iter().map(|s| json!({ "x": s.x, "n": s.n, "v": s.v, "m": s.m })).collect::<Vec<_>>() })
        }
        ElementResult::Plane { gauss } => {
            json!({ "kind": "plane", "gauss": gauss.iter().map(|g| json!({ "sxx": g.sxx, "syy": g.syy, "sxy": g.sxy, "vonMises": g.von_mises })).collect::<Vec<_>>() })
        }
        ElementResult::Plate { gauss } => {
            json!({ "kind": "plate", "gauss": gauss.iter().map(|g| json!({ "mx": g.mx, "my": g.my, "mxy": g.mxy })).collect::<Vec<_>>() })
        }
        ElementResult::Solid { gauss } => json!({
            "kind": "solid",
            "gauss": gauss.iter().map(|g| json!({ "sxx": g.sxx, "syy": g.syy, "szz": g.szz, "sxy": g.sxy, "syz": g.syz, "sxz": g.sxz, "vonMises": g.von_mises })).collect::<Vec<_>>(),
        }),
        ElementResult::Shell { gauss } => json!({
            "kind": "shell",
            "gauss": gauss.iter().map(|g| json!({ "nxx": g.nxx, "nyy": g.nyy, "nxy": g.nxy, "mxx": g.mxx, "myy": g.myy, "mxy": g.mxy, "vonMisesTop": g.von_mises_top, "vonMisesBottom": g.von_mises_bottom })).collect::<Vec<_>>(),
        }),
    }
}

fn static_result_json(result: &fem_core::StaticResult) -> Value {
    json!({
        "displacements": result.displacements.iter().map(|d| json!({ "nodeId": d.node_id, "values": d.values })).collect::<Vec<_>>(),
        "reactions": result.reactions.iter().map(|r| json!({ "nodeId": r.node_id, "dof": dof_json(r.dof), "value": r.value })).collect::<Vec<_>>(),
        "elements": result.elements.iter().map(|(id, element_result)| json!({ "id": id, "result": element_result_json(element_result) })).collect::<Vec<_>>(),
        "checks": { "residualNorm": result.checks.residual_norm, "reactionSum": result.checks.reaction_sum },
    })
}

fn results_map_json(results: &HashMap<String, fem_core::StaticResult>) -> Value {
    Value::Object(results.iter().map(|(id, result)| (id.clone(), static_result_json(result))).collect())
}

/// 🧪️ B1: unit struct — every former `Fem2dPlayApp` `RefCell` field (`result_display`, `camera`) plus
/// the deleted `ViewState::locale` now live in `fem2d_engine::Fem2dConfig` (see `DocumentApp::Config`),
/// written through `fem2d_op::Fem2dConfigOperation`s. v0 design unchanged: results are never persisted
/// or cached — `fem2d_solve`/`fem2d_solve_all` run fresh inside `render()`/`export_media` whenever the
/// results window is drawn or the `"results:out"` port is read.
#[derive(Default)]
pub struct Fem2dPlayApp;

impl DocumentApp for Fem2dPlayApp {
    type Projection = Fem2dDocument;
    type Operation = Fem2dOperation;
    type Config = Fem2dConfig;
    type ConfigOperation = Fem2dConfigOperation;
    type Command = Fem2dCommand;

    fn app_id(&self) -> &str {
        FEM2D_APP_ID
    }

    fn document_schema(&self) -> &str {
        fem2d::FEM_2D_SCHEMA
    }

    fn initial_projection(&self) -> Fem2dDocument {
        fem2d_engine::empty_fem2d_projection()
    }

    fn io(&self) -> Option<AppIo> {
        Some(fem2d_engine::fem2d_io())
    }

    /// 🎞️ `"document:out"` reproduces the trait's default whole-document pack (overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new
    /// one). `"results:out"` runs every load case/combination's analysis fresh and returns them as plain
    /// JSON text in a `Structured` payload — `MediaPayload::Structured.json` doesn't require a
    /// `pack`-encoded value (see `shooting_engine::shooting_photo_media`'s base64-PNG payload for
    /// another existing non-pack producer). A document with no load cases, or a solve failure, is
    /// reported as `MediaError::Payload` rather than an empty/panicking export.
    fn export_media(&self, port: &str, doc: &DocumentView<'_, Fem2dDocument>) -> Result<Media, MediaError> {
        match port {
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = <Fem2dDocument as store::DocumentPack>::encode_pack(doc.projection);
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "results:out" => {
                if doc.projection.load_cases.is_empty() {
                    return Err(MediaError::Payload("results:out".into(), "no load cases defined".into()));
                }
                let results = fem2d_engine::fem2d_solve_all(doc.projection).map_err(|error| MediaError::Payload("results:out".into(), error.to_string()))?;
                let json = results_map_json(&results).to_string();
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.fem2d".into(), json } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn whole_document_operation(&self, projection: Fem2dDocument) -> Option<Fem2dOperation> {
        Some(Fem2dOperation::SetDocument { document: projection })
    }

    /// 🎞️ `"document:in"` reproduces the trait's default whole-document-pack importer (overriding
    /// `import_media` shadows it for every port). `"geometry:in"` decodes a minimal, app-owned
    /// `{"outline": [[f64;2]...], "holes": [[[f64;2]...]...]}` polygon-with-holes contract — no
    /// canonical cross-app 2D-vector schema exists yet in this codebase, so this app owns choosing its
    /// own decode shape — into a new `FemRegion`, defaulted to the document's first existing material if
    /// any, else an `"unassigned"` placeholder id (the region simply won't solve until a real material
    /// is assigned; there's no generically "sensible" default material to synthesize here).
    fn import_media(&self, port: &str, media: &Media, doc: &DocumentView<'_, Fem2dDocument>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, MediaError> {
        match port {
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let projection = <Fem2dDocument as store::DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                match self.whole_document_operation(projection) {
                    Some(operation) => Ok(Emit::operations(vec![operation])),
                    None => Err(MediaError::NotImplemented),
                }
            }
            "geometry:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "geometry:in only accepts a Structured JSON payload".into()));
                };
                let value: Value = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let outline: Vec<[f64; 2]> = serde_json::from_value(value.get("outline").cloned().unwrap_or(Value::Null)).map_err(|error| MediaError::Payload(port.to_string(), format!("outline: {error}")))?;
                let holes: Vec<Vec<[f64; 2]>> = match value.get("holes").cloned() {
                    Some(holes_value) => serde_json::from_value(holes_value).map_err(|error| MediaError::Payload(port.to_string(), format!("holes: {error}")))?,
                    None => Vec::new(),
                };
                let material_id = doc.projection.materials.first().map(|material| material.id.clone()).unwrap_or_else(|| "unassigned".into());
                let id = next_id(doc.projection.regions.iter().map(|r| r.id.clone()), "r");
                let index = doc.projection.regions.len();
                let region = fem2d::FemRegion { id, name: "Imported Geometry".into(), outline, holes, thickness: 0.02, material_id, mesh_size: 0.25 };
                Ok(Emit::operations(vec![Fem2dOperation::SetRegion { index, region }]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ Maps each `Fem2dCommand` variant back to the action id it was declared under in
    /// `create_fem2d_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &Fem2dCommand) -> &str {
        match command {
            Fem2dCommand::AddNode { .. } => "addNode",
            Fem2dCommand::AddBar { .. } => "addBar",
            Fem2dCommand::AddBeam { .. } => "addBeam",
            Fem2dCommand::AddMaterial { .. } => "addMaterial",
            Fem2dCommand::AddSection { .. } => "addSection",
            Fem2dCommand::AddSupport { .. } => "addSupport",
            Fem2dCommand::AddNodalLoad { .. } => "addNodalLoad",
            Fem2dCommand::AddMemberUdl { .. } => "addMemberUdl",
            Fem2dCommand::AddAreaLoad { .. } => "addAreaLoad",
            Fem2dCommand::AddRegion { .. } => "addRegion",
            Fem2dCommand::AddLoadCase { .. } => "addLoadCase",
            Fem2dCommand::AddCombination { .. } => "addCombination",
            Fem2dCommand::SetSelfWeight { .. } => "setSelfWeight",
            Fem2dCommand::SetAnalysisSettings { .. } => "setAnalysisSettings",
            Fem2dCommand::RemoveSelection { .. } => "removeSelection",
            Fem2dCommand::SetActiveExample { .. } => "setActiveExample",
            Fem2dCommand::SetCamera { .. } => "setCamera",
            Fem2dCommand::SetResultDisplay { .. } => "setResultDisplay",
            Fem2dCommand::SetLocale { .. } => "setLocale",
        }
    }

    /// 🧩️ B1: the pure heart of the app — a total, side-effect-free function from
    /// `(command, document, config)` to an `Emit`. Every former `handle_action` match arm keeps working,
    /// just through this typed channel instead of the `{action, args}` JSON channel.
    fn handle(&self, command: &Fem2dCommand, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Emit<Fem2dOperation, Fem2dConfigOperation> {
        let projection = doc.projection;
        match command {
            Fem2dCommand::AddNode { x, y } => {
                let id = next_id(projection.nodes.iter().map(|n| n.id.clone()), "n");
                let index = projection.nodes.len();
                Emit::operations(vec![Fem2dOperation::SetNode { index, node: fem2d::FemNode { id, x: *x, y: *y } }])
            }
            Fem2dCommand::AddBar { start, end, material_id, section_id } => {
                let id = next_id(projection.elements.iter().map(|e| fem2d::element_id(e).to_string()), "e");
                let index = projection.elements.len();
                let element = fem2d::FemElement::Bar { id, start: start.clone(), end: end.clone(), material_id: material_id.clone(), section_id: section_id.clone() };
                Emit::operations(vec![Fem2dOperation::SetElement { index, element: Box::new(element) }])
            }
            Fem2dCommand::AddBeam { start, end, material_id, section_id } => {
                let id = next_id(projection.elements.iter().map(|e| fem2d::element_id(e).to_string()), "e");
                let index = projection.elements.len();
                let element = fem2d::FemElement::Beam { id, start: start.clone(), end: end.clone(), material_id: material_id.clone(), section_id: section_id.clone() };
                Emit::operations(vec![Fem2dOperation::SetElement { index, element: Box::new(element) }])
            }
            Fem2dCommand::AddMaterial { name, e } => {
                let id = next_id(projection.materials.iter().map(|m| m.id.clone()), "m");
                let index = projection.materials.len();
                Emit::operations(vec![Fem2dOperation::SetMaterial { index, material: fem2d::FemMaterial { id, name: name.clone(), e: *e, nu: 0.3, rho: 7850.0 } }])
            }
            Fem2dCommand::AddSection { name, area, iy } => {
                let id = next_id(projection.sections.iter().map(|s| s.id.clone()), "s");
                let index = projection.sections.len();
                Emit::operations(vec![Fem2dOperation::SetSection { index, section: fem2d::FemSection { id, name: name.clone(), area: *area, iy: *iy } }])
            }
            Fem2dCommand::AddSupport { node_id, fixed } => {
                let id = next_id(projection.supports.iter().map(|s| s.id.clone()), "sup");
                let index = projection.supports.len();
                Emit::operations(vec![Fem2dOperation::SetSupport { index, support: fem2d::FemSupport { id, node_id: node_id.clone(), fixed: fixed.clone() } }])
            }
            Fem2dCommand::AddNodalLoad { node_id, dof, value, case_id } => {
                let (index, mut load_case) = fem2d_resolve_load_case(projection, case_id.as_deref());
                let load_id = next_id(load_case.loads.iter().map(|l| fem2d::load_id(l).to_string()), "l");
                load_case.loads.push(fem2d::FemLoad::Nodal { id: load_id, node_id: node_id.clone(), dof: *dof, value: *value });
                Emit::operations(vec![Fem2dOperation::SetLoadCase { index, load_case }])
            }
            Fem2dCommand::AddMemberUdl { element_id, wx, wy, case_id } => {
                let (index, mut load_case) = fem2d_resolve_load_case(projection, case_id.as_deref());
                let load_id = next_id(load_case.loads.iter().map(|l| fem2d::load_id(l).to_string()), "l");
                load_case.loads.push(fem2d::FemLoad::MemberUdl { id: load_id, element_id: element_id.clone(), wx: *wx, wy: *wy });
                Emit::operations(vec![Fem2dOperation::SetLoadCase { index, load_case }])
            }
            Fem2dCommand::AddAreaLoad { region_id, pressure, case_id } => {
                let (index, mut load_case) = fem2d_resolve_load_case(projection, case_id.as_deref());
                let load_id = next_id(load_case.loads.iter().map(|l| fem2d::load_id(l).to_string()), "l");
                load_case.loads.push(fem2d::FemLoad::Area { id: load_id, region_id: region_id.clone(), pressure: *pressure });
                Emit::operations(vec![Fem2dOperation::SetLoadCase { index, load_case }])
            }
            Fem2dCommand::AddRegion { x, y, width, height, material_id, thickness, mesh_size } => {
                let id = next_id(projection.regions.iter().map(|r| r.id.clone()), "r");
                let index = projection.regions.len();
                let outline = vec![[*x, *y], [x + width, *y], [x + width, y + height], [*x, y + height]];
                let region = fem2d::FemRegion { id, name: "Region".into(), outline, holes: Vec::new(), thickness: thickness.unwrap_or(0.02), material_id: material_id.clone(), mesh_size: mesh_size.unwrap_or(0.25) };
                Emit::operations(vec![Fem2dOperation::SetRegion { index, region }])
            }
            Fem2dCommand::AddLoadCase { name, self_weight } => {
                let id = next_id(projection.load_cases.iter().map(|lc| lc.id.clone()), "case-");
                let index = projection.load_cases.len();
                Emit::operations(vec![Fem2dOperation::SetLoadCase { index, load_case: fem2d::FemLoadCase { id, name: name.clone(), loads: Vec::new(), self_weight: *self_weight } }])
            }
            Fem2dCommand::AddCombination { name, terms } => {
                let id = next_id(projection.combinations.iter().map(|c| c.id.clone()), "c");
                let index = projection.combinations.len();
                Emit::operations(vec![Fem2dOperation::SetCombination { index, combination: fem2d::FemCombination { id, name: name.clone(), terms: terms.clone() } }])
            }
            Fem2dCommand::SetSelfWeight { case_id, enabled } => match projection.load_cases.iter().position(|lc| &lc.id == case_id) {
                Some(index) => {
                    let mut load_case = projection.load_cases[index].clone();
                    load_case.self_weight = *enabled;
                    Emit::operations(vec![Fem2dOperation::SetLoadCase { index, load_case }])
                }
                None => Emit::default(),
            },
            Fem2dCommand::SetAnalysisSettings { modal_count, buckling_count, deformation_scale } => {
                let current = &projection.analysis;
                let settings = fem2d::FemAnalysisSettings {
                    modal_count: modal_count.map(|value| value as usize).unwrap_or(current.modal_count),
                    buckling_count: buckling_count.map(|value| value as usize).unwrap_or(current.buckling_count),
                    deformation_scale: deformation_scale.unwrap_or(current.deformation_scale),
                };
                Emit::operations(vec![Fem2dOperation::SetAnalysisSettings { settings }])
            }
            Fem2dCommand::RemoveSelection { ids } => {
                let mut operations = Vec::new();
                for id in ids {
                    if projection.nodes.iter().any(|n| &n.id == id) {
                        operations.push(Fem2dOperation::RemoveNode { id: id.clone() });
                    } else if projection.elements.iter().any(|e| fem2d::element_id(e) == id) {
                        operations.push(Fem2dOperation::RemoveElement { id: id.clone() });
                    } else if projection.materials.iter().any(|m| &m.id == id) {
                        operations.push(Fem2dOperation::RemoveMaterial { id: id.clone() });
                    } else if projection.sections.iter().any(|s| &s.id == id) {
                        operations.push(Fem2dOperation::RemoveSection { id: id.clone() });
                    } else if projection.supports.iter().any(|s| &s.id == id) {
                        operations.push(Fem2dOperation::RemoveSupport { id: id.clone() });
                    } else if projection.load_cases.iter().any(|l| &l.id == id) {
                        operations.push(Fem2dOperation::RemoveLoadCase { id: id.clone() });
                    } else if projection.regions.iter().any(|r| &r.id == id) {
                        operations.push(Fem2dOperation::RemoveRegion { id: id.clone() });
                    } else if projection.combinations.iter().any(|c| &c.id == id) {
                        operations.push(Fem2dOperation::RemoveCombination { id: id.clone() });
                    }
                }
                if operations.is_empty() {
                    Emit::default()
                } else {
                    Emit::operations(operations)
                }
            }
            Fem2dCommand::SetActiveExample { example_id } => {
                let document = if example_id == "default" { Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap_or_else(|_| fem2d_engine::empty_fem2d_projection()) } else { fem2d_engine::empty_fem2d_projection() };
                Emit { document_operations: vec![Fem2dOperation::SetDocument { document }], config_operations: vec![Fem2dConfigOperation::Snapshot { config: Fem2dConfig::default() }], ..Default::default() }
            }
            // 🎥️ Config-only: the canvas camera never touches the document.
            Fem2dCommand::SetCamera { x, y, zoom } => Emit::config(vec![Fem2dConfigOperation::SetCamera { camera: FemCamera { x: *x, y: *y, zoom: *zoom } }]),
            // 👁️ Config-only: which case/mode the results window shows never touches the document.
            Fem2dCommand::SetResultDisplay { source_id, mode, mode_index } => Emit::config(vec![Fem2dConfigOperation::SetResultDisplay { source_id: source_id.clone(), mode: mode.clone(), mode_index: *mode_index }]),
            Fem2dCommand::SetLocale { value } => Emit::config(vec![Fem2dConfigOperation::SetLocale { value: value.clone() }]),
        }
    }

    /// 🧮️ This app's typed configuration spec — no sticky ActionArgDef defaults are mirrored here (all
    /// of `addRegion`'s `thickness`/`meshSize` defaults are baked directly into `handle`, not
    /// user-configurable settings), so this simply declares no fields, matching
    /// `shooting_ui::ShootingPlayApp::config_spec`'s documented judgment call for camera/selection.
    fn config_spec(&self) -> ConfigSpec {
        ConfigSpec::empty()
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Fem2dDocument>, cfg: &ConfigView<'_, Fem2dConfig>) -> UiNode {
        let camera = &cfg.projection.camera;
        match body_key {
            FEM2D_BODY_MODEL => render_fem2d_model(doc.projection, camera),
            FEM2D_BODY_RESULTS => render_fem2d_results(doc.projection, &config_result_display(cfg.projection), camera),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Fem2dPlayApp

//#region 🔖️Manifest
pub fn create_fem2d_app() -> App {
    App::from_builder(
        App::builder(FEM2D_APP_ID, LocalizedLabel::native("FEM 2D", "FEM 2D"))
            .document(["semio", "fem", "fem2d"])
            // 🔌️ The computed-results output artifact (`results:out`'s `kind_id`, see `fem2d_engine::fem2d_io`)
            // — the OS-catalog-level resource descriptor for `computation.fem2d`; deliberately a
            // different `media_type` (`Computation`×`Value`) than the PORT's wire-level `Data`×`Value`
            // (see WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE's port recipe).
            .artifact_kind(ArtifactKindSpec {
                id: "computation.fem2d".into(),
                name: "FEM 2D Results".into(),
                source_format: "computation.fem2d".into(),
                component_kind: "fem2d-results".into(),
                dimension: "computation".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
                schema: "computation.fem2d".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("fem-app")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind(FEM2D_WINDOW_MODEL, LocalizedLabel::native("Model", "Modell"), FEM2D_BODY_MODEL, SurfaceKind::Canvas2d, "fem-model")
            .window_kind(FEM2D_WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), FEM2D_BODY_RESULTS, SurfaceKind::Canvas2d, "bar-chart-3")
            .default_layout(create_default_layout(
                &[FEM2D_WINDOW_MODEL.into(), FEM2D_WINDOW_RESULTS.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Model".into(), "Results".into()]),
            ))
            .operation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_args("addNode", vec![
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required(),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required(),
            ])
            .operation("addBar", LocalizedLabel::native("Add Bar", "Stab hinzufügen"))
            .operation("addBeam", LocalizedLabel::native("Add Beam", "Balken hinzufügen"))
            .operation("addMaterial", LocalizedLabel::native("Add Material", "Material hinzufügen"))
            .operation("addSection", LocalizedLabel::native("Add Section", "Querschnitt hinzufügen"))
            .operation("addSupport", LocalizedLabel::native("Add Support", "Lager hinzufügen"))
            .operation("addNodalLoad", LocalizedLabel::native("Add Nodal Load", "Knotenlast hinzufügen"))
            .action_args("addNodalLoad", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .operation("addMemberUdl", LocalizedLabel::native("Add Member UDL", "Streckenlast hinzufügen"))
            .action_args("addMemberUdl", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .operation("addAreaLoad", LocalizedLabel::native("Add Area Load", "Flächenlast hinzufügen"))
            .action_args("addAreaLoad", vec![
                ActionArgDef::text("regionId", LocalizedLabel::native("Region", "Bereich")).required(),
                ActionArgDef::number("pressure", LocalizedLabel::native("Pressure", "Druck")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .operation("addRegion", LocalizedLabel::native("Add Region", "Bereich hinzufügen"))
            .action_args("addRegion", vec![
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required(),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required(),
                ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required(),
                ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::native("Material", "Material")).required(),
                ActionArgDef::number("thickness", LocalizedLabel::native("Thickness", "Dicke")).default_value(0.02),
                ActionArgDef::number("meshSize", LocalizedLabel::native("Mesh Size", "Netzgröße")).default_value(0.25),
            ])
            .operation("addLoadCase", LocalizedLabel::native("Add Load Case", "Lastfall hinzufügen"))
            .action_args("addLoadCase", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                ActionArgDef::toggle("selfWeight", LocalizedLabel::native("Self Weight", "Eigengewicht")).default_value(false),
            ])
            // 🎯️ `terms` is now `Fem2dCommand::AddCombination`'s typed `Vec<fem2d::FemCombinationTerm>`
            // (no longer a JSON-string blob) — no single `ActionArgDef` control maps to that shape, so
            // (mirroring `shooting_ui`'s precedent for commands with no matching staged form, e.g.
            // `SetShotCamera`) this action simply has no `.action_args(...)` declaration.
            .operation("addCombination", LocalizedLabel::native("Add Combination", "Kombination hinzufügen"))
            .operation("setSelfWeight", LocalizedLabel::native("Set Self Weight", "Eigengewicht festlegen"))
            .action_args("setSelfWeight", vec![
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")).required(),
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).required(),
            ])
            .operation("setAnalysisSettings", LocalizedLabel::native("Set Analysis Settings", "Analyseeinstellungen festlegen"))
            .action_args("setAnalysisSettings", vec![
                ActionArgDef::number("modalCount", LocalizedLabel::native("Modal Count", "Anzahl Eigenformen")),
                ActionArgDef::number("bucklingCount", LocalizedLabel::native("Buckling Count", "Anzahl Knickformen")),
                ActionArgDef::number("deformationScale", LocalizedLabel::native("Deformation Scale", "Verformungsmaßstab")),
            ])
            .operation("removeSelection", LocalizedLabel::native("Remove Selection", "Auswahl entfernen"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new("default", LocalizedLabel::native("Default", "Standard"))])
                    .default_value("default"),
            ])
            .view_action("setResultDisplay", LocalizedLabel::native("Set Result Display", "Ergebnisanzeige festlegen"))
            .action_args("setResultDisplay", result_display_action_args())
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS Wave 1) —
            // `config_spec()`/`fem2d_io()` are this same information's single source of truth, reused
            // here rather than duplicated.
            .config(Fem2dPlayApp::default().config_spec())
            .io(fem2d_engine::fem2d_io()),
    )
    .example("default", LocalizedLabel::native("Family House", "Einfamilienhaus"), FEM2D_EXAMPLE_DSL, "file")
    .workflow("fem2d", "FEM 2D", "structure")
}
//#endregion 🔖️Manifest

// #region 🔖️WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use fem2d_op::{Fem2dEnvelope, Fem2dStore};
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Fem2dDocumentVcs {
        store: RefCell<Fem2dStore>,
    }

    #[wasm_bindgen]
    impl Fem2dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Fem2dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Fem2dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Fem2dStore::new(envelope)
                }
                None => Fem2dStore::new(create_document_envelope(fem2d::FEM_2D_SCHEMA, "fem2d", fem2d_engine::empty_fem2d_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
// #endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionKind, HistoryView, Locale, Terminology};

    fn history_view() -> HistoryView {
        HistoryView::empty()
    }

    fn default_config() -> Fem2dConfig {
        Fem2dConfig::default()
    }

    //#region 🔖️RendersScenes
    #[test]
    fn renders_fem2d_model_scene() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let node = app.render(FEM2D_BODY_MODEL, &doc, &cfg);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn renders_fem2d_results_scene() {
        let app = Fem2dPlayApp::default();
        let projection: Fem2dDocument = Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let node = app.render(FEM2D_BODY_RESULTS, &doc, &cfg);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }
    //#endregion 🔖️RendersScenes

    //#region 🔖️AddNodeAction
    #[test]
    fn add_node_action_emits_op_2d() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let emit = app.handle(&Fem2dCommand::AddNode { x: 1.0, y: 2.0 }, &doc, &cfg);
        assert_eq!(emit.document_operations.len(), 1);
        assert!(emit.config_operations.is_empty());
        match &emit.document_operations[0] {
            Fem2dOperation::SetNode { node, .. } => {
                assert_eq!(node.x, 1.0);
                assert_eq!(node.y, 2.0);
            }
            _ => panic!("expected SetNode"),
        }
    }
    //#endregion 🔖️AddNodeAction

    //#region 🔖️SolverErrorSurfaced
    #[test]
    fn results_window_surfaces_solver_error_without_panicking_2d() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let _ = app.render(FEM2D_BODY_RESULTS, &doc, &cfg);
    }
    //#endregion 🔖️SolverErrorSurfaced

    //#region 🔖️ExampleFixtureRenders
    #[test]
    fn example_fixture_renders_2d() {
        let app = Fem2dPlayApp::default();
        let projection: Fem2dDocument = Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let _ = app.render(FEM2D_BODY_MODEL, &doc, &cfg);
        let _ = app.render(FEM2D_BODY_RESULTS, &doc, &cfg);
    }
    //#endregion 🔖️ExampleFixtureRenders

    //#region 🔖️MeshPreviewRender
    #[test]
    fn mesh_preview_renders_region_edges() {
        let app = Fem2dPlayApp::default();
        let projection: Fem2dDocument = Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let node = app.render(FEM2D_BODY_MODEL, &doc, &cfg);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("mesh-edge-"), "expected mesh-edge preview layers in the model scene");
    }
    //#endregion 🔖️MeshPreviewRender

    //#region 🔖️ResultDisplayAction
    #[test]
    fn set_result_display_is_config_only() {
        let app = Fem2dPlayApp::default();
        let projection: Fem2dDocument = Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let command = Fem2dCommand::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 };
        let emit = app.handle(&command, &doc, &cfg);
        assert!(emit.document_operations.is_empty(), "setResultDisplay must not emit document operations (it's config-only)");
        assert_eq!(emit.config_operations.len(), 1);
        match &emit.config_operations[0] {
            Fem2dConfigOperation::SetResultDisplay { source_id, mode, mode_index } => {
                assert_eq!(source_id.as_deref(), Some("dead"));
                assert_eq!(mode, "modal");
                assert_eq!(*mode_index, 0);
            }
            other => panic!("expected SetResultDisplay, got {other:?}"),
        }
    }
    //#endregion 🔖️ResultDisplayAction

    //#region 🔖️SetActiveExample
    #[test]
    fn set_active_example_loads_default_fixture_2d() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let emit = app.handle(&Fem2dCommand::SetActiveExample { example_id: "default".into() }, &doc, &cfg);
        assert_eq!(emit.document_operations.len(), 1);
        // 🧮️ Also resets the config back to its default (mirrors the pre-B1 `result_display`/`camera`
        // resets on `setActiveExample`) — a single whole-record `Snapshot`.
        assert_eq!(emit.config_operations, vec![Fem2dConfigOperation::Snapshot { config: Fem2dConfig::default() }]);
        match &emit.document_operations[0] {
            Fem2dOperation::SetDocument { document } => assert!(!document.nodes.is_empty(), "expected the default fixture's nodes"),
            _ => panic!("expected SetDocument"),
        }
    }

    #[test]
    fn set_active_example_unknown_id_yields_empty_document_2d() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let emit = app.handle(&Fem2dCommand::SetActiveExample { example_id: "".into() }, &doc, &cfg);
        match &emit.document_operations[0] {
            Fem2dOperation::SetDocument { document } => assert_eq!(document, &fem2d_engine::empty_fem2d_projection()),
            _ => panic!("expected SetDocument"),
        }
    }

    /// 🧬️ `setActiveExample` replaces document content via `SetDocument` operations, so it MUST be declared as
    /// an Operation, not a View/Shell action — the framework's "View/Shell actions must not emit
    /// operations" guard would otherwise reject it.
    #[test]
    fn set_active_example_is_declared_as_operation_2d() {
        let definition = create_fem2d_app().definition;
        let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, ActionKind::Operation), "loading an example emits SetDocument operations, so it is an Operation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
    }
    //#endregion 🔖️SetActiveExample

    //#region 🔖️ContourRender
    #[test]
    fn results_window_renders_contour_for_region() {
        let app = Fem2dPlayApp::default();
        let projection: Fem2dDocument = Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = Fem2dConfig { result_source_id: Some("dead".into()), result_mode: "static".into(), ..Fem2dConfig::default() };
        let cfg = ConfigView { projection: &config };
        let node = app.render(FEM2D_BODY_RESULTS, &doc, &cfg);
        let json = serde_json::to_string(&node).unwrap();
        // `layers_json` is itself a JSON string embedded inside `UiNode`'s own serialization, so its
        // quotes come out backslash-escaped in `json` — match on the unescaped substrings instead.
        assert!(json.contains("fill"), "expected filled-path contour layers for the region's Tri3Cst elements: {json}");
        assert!(json.contains("contour-"), "expected contour-prefixed layer ids: {json}");
    }
    //#endregion 🔖️ContourRender

    //#region 🔖️ReactionLabels
    #[test]
    fn results_window_renders_reaction_labels_2d() {
        let app = Fem2dPlayApp::default();
        let projection: Fem2dDocument = Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = Fem2dConfig { result_source_id: Some("dead".into()), result_mode: "static".into(), ..Fem2dConfig::default() };
        let cfg = ConfigView { projection: &config };
        let node = app.render(FEM2D_BODY_RESULTS, &doc, &cfg);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("reaction-"), "expected reaction-prefixed text label layers: {json}");
    }
    //#endregion 🔖️ReactionLabels

    //#region 🔖️ModeShapeRender
    #[test]
    fn results_window_renders_modal_mode_shape_2d() {
        let app = Fem2dPlayApp::default();
        let projection: Fem2dDocument = Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = Fem2dConfig { result_mode: "modal".into(), result_mode_index: 0, ..Fem2dConfig::default() };
        let cfg = ConfigView { projection: &config };
        let node = app.render(FEM2D_BODY_RESULTS, &doc, &cfg);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"), "expected a valid canvas-2d scene, got: {json}");
        assert!(!json.contains("Modal analysis error"), "unexpected modal error: {json}");
    }

    #[test]
    fn results_window_renders_buckling_mode_shape_2d() {
        let app = Fem2dPlayApp::default();
        let projection: Fem2dDocument = Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = Fem2dConfig { result_source_id: Some("dead".into()), result_mode: "buckling".into(), result_mode_index: 0, ..Fem2dConfig::default() };
        let cfg = ConfigView { projection: &config };
        let node = app.render(FEM2D_BODY_RESULTS, &doc, &cfg);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"), "expected a valid canvas-2d scene, got: {json}");
        assert!(!json.contains("Buckling analysis error"), "unexpected buckling error: {json}");
    }
    //#endregion 🔖️ModeShapeRender

    //#region 🔖️StructureActions
    #[test]
    fn add_region_action_emits_set_region_2d() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let command = Fem2dCommand::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id: "steel".into(), thickness: None, mesh_size: None };
        let emit = app.handle(&command, &doc, &cfg);
        assert_eq!(emit.document_operations.len(), 1);
        match &emit.document_operations[0] {
            Fem2dOperation::SetRegion { region, .. } => {
                assert_eq!(region.outline, vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]]);
                assert_eq!(region.thickness, 0.02);
                assert_eq!(region.mesh_size, 0.25);
            }
            _ => panic!("expected SetRegion"),
        }
    }

    #[test]
    fn remove_selection_covers_regions_and_combinations_2d() {
        let app = Fem2dPlayApp::default();
        let mut projection = fem2d_engine::empty_fem2d_projection();
        projection.regions.push(fem2d::FemRegion { id: "r1".into(), name: "R".into(), outline: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 0.5 });
        projection.combinations.push(fem2d::FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }] });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let emit = app.handle(&Fem2dCommand::RemoveSelection { ids: vec!["r1".into(), "uls".into()] }, &doc, &cfg);
        assert_eq!(emit.document_operations.len(), 2);
        assert!(matches!(emit.document_operations[0], Fem2dOperation::RemoveRegion { .. }));
        assert!(matches!(emit.document_operations[1], Fem2dOperation::RemoveCombination { .. }));
    }
    //#endregion 🔖️StructureActions

    //#region 🔖️LoadCaseActions
    #[test]
    fn add_load_case_and_combination_emit_ops_2d() {
        let app = Fem2dPlayApp::default();
        let mut projection = fem2d_engine::empty_fem2d_projection();
        projection.load_cases.push(fem2d::FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };

        let emit_case = app.handle(&Fem2dCommand::AddLoadCase { name: "Live".into(), self_weight: false }, &doc, &cfg);
        match &emit_case.document_operations[0] {
            Fem2dOperation::SetLoadCase { load_case, .. } => assert_eq!(load_case.name, "Live"),
            _ => panic!("expected SetLoadCase"),
        }

        let emit_combo = app.handle(&Fem2dCommand::AddCombination { name: "ULS".into(), terms: vec![fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }] }, &doc, &cfg);
        match &emit_combo.document_operations[0] {
            Fem2dOperation::SetCombination { combination, .. } => assert_eq!(combination.terms, vec![fem2d::FemCombinationTerm { case_id: "dead".to_string(), factor: 1.35 }]),
            _ => panic!("expected SetCombination"),
        }
    }

    #[test]
    fn add_area_load_targets_named_case_2d() {
        let app = Fem2dPlayApp::default();
        let mut projection = fem2d_engine::empty_fem2d_projection();
        projection.load_cases.push(fem2d::FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: false });
        projection.load_cases.push(fem2d::FemLoadCase { id: "live".into(), name: "Live".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let command = Fem2dCommand::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some("live".into()) };
        let emit = app.handle(&command, &doc, &cfg);
        match &emit.document_operations[0] {
            Fem2dOperation::SetLoadCase { index, load_case } => {
                assert_eq!(*index, 1);
                assert_eq!(load_case.id, "live");
                assert!(matches!(load_case.loads[0], fem2d::FemLoad::Area { .. }));
            }
            _ => panic!("expected SetLoadCase"),
        }
    }

    #[test]
    fn set_self_weight_toggles_case_2d() {
        let app = Fem2dPlayApp::default();
        let mut projection = fem2d_engine::empty_fem2d_projection();
        projection.load_cases.push(fem2d::FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let emit = app.handle(&Fem2dCommand::SetSelfWeight { case_id: "dead".into(), enabled: true }, &doc, &cfg);
        match &emit.document_operations[0] {
            Fem2dOperation::SetLoadCase { load_case, .. } => assert!(load_case.self_weight),
            _ => panic!("expected SetLoadCase"),
        }
    }

    #[test]
    fn set_analysis_settings_partial_args_keep_current_2d() {
        let app = Fem2dPlayApp::default();
        let mut projection = fem2d_engine::empty_fem2d_projection();
        projection.analysis = fem2d::FemAnalysisSettings { modal_count: 4, buckling_count: 6, deformation_scale: 50.0 };
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let command = Fem2dCommand::SetAnalysisSettings { modal_count: None, buckling_count: None, deformation_scale: Some(300.0) };
        let emit = app.handle(&command, &doc, &cfg);
        match &emit.document_operations[0] {
            Fem2dOperation::SetAnalysisSettings { settings } => {
                assert_eq!(settings.modal_count, 4);
                assert_eq!(settings.buckling_count, 6);
                assert_eq!(settings.deformation_scale, 300.0);
            }
            _ => panic!("expected SetAnalysisSettings"),
        }
    }
    //#endregion 🔖️LoadCaseActions

    //#region 🔖️SharedHelpers
    #[test]
    fn interpolate_at_value_falls_back_to_midpoint_when_values_equal() {
        let (point, value) = interpolate_at_value(((0.0, 0.0), 5.0), ((10.0, 20.0), 5.0), 5.0);
        assert_eq!(point, (5.0, 10.0));
        assert_eq!(value, 5.0);
    }

    #[test]
    fn clip_by_value_empty_polygon_returns_empty() {
        assert!(clip_by_value(&[], 0.0, true).is_empty());
    }

    #[test]
    fn clip_by_value_keeps_only_the_requested_half_plane() {
        let poly: Vec<ValuedPoint> = vec![((0.0, 0.0), 0.0), ((10.0, 0.0), 10.0), ((0.0, 10.0), 0.0)];
        let above = clip_by_value(&poly, 5.0, true);
        assert!(above.len() >= 3 && above.iter().all(|(_, v)| *v >= 5.0 - 1e-9));
        let below = clip_by_value(&poly, 5.0, false);
        assert!(below.len() >= 3 && below.iter().all(|(_, v)| *v <= 5.0 + 1e-9));
    }

    #[test]
    fn fem2d_resolve_load_case_synthesizes_case_when_none_exist() {
        let projection = fem2d_engine::empty_fem2d_projection();
        let (index, load_case) = fem2d_resolve_load_case(&projection, None);
        assert_eq!(index, 0);
        assert_eq!(load_case.id, "case-1");
    }

    #[test]
    fn fem2d_model_extent_degenerate_model_returns_one() {
        assert_eq!(fem2d_model_extent(&fem2d_engine::empty_fem2d_projection()), 1.0);
    }
    //#endregion 🔖️SharedHelpers

    //#region 🔖️UnknownBodyAndGermanLabels
    #[test]
    fn render_unknown_body_key_returns_placeholder_text_2d() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let json = serde_json::to_string(&app.render("nonsense", &doc, &cfg)).unwrap();
        assert!(json.contains("Unknown body: nonsense"));
    }

    /// 🗣️ B1: the manifest itself (not a runtime `cfg.locale`-driven overlay) now carries every
    /// locale's translation via `LocalizedLabel` — see `create_fem2d_app`'s `.window_kind`/`.operation`/
    /// `.view_action` calls. Replaces the deleted `Fem2dPlayApp::app_labels`/`AppLabelsOverlay` test.
    #[test]
    fn manifest_labels_resolve_german_locale_2d() {
        let definition = create_fem2d_app().definition;
        let window_model = definition.window_kinds.iter().find(|window| window.id == FEM2D_WINDOW_MODEL).expect("model window kind declared");
        assert_eq!(window_model.label.resolve(Terminology::Native, Locale::En), "Model");
        assert_eq!(window_model.label.resolve(Terminology::Native, Locale::De), "Modell");
        let add_node = definition.actions.iter().find(|action| action.id == "addNode").expect("addNode action declared");
        assert_eq!(add_node.label.resolve(Terminology::Native, Locale::En), "Add Node");
        assert_eq!(add_node.label.resolve(Terminology::Native, Locale::De), "Knoten hinzufügen");
        let set_locale = definition.actions.iter().find(|action| action.id == "setLocale").expect("setLocale action declared");
        assert_eq!(set_locale.label.resolve(Terminology::Native, Locale::En), "Set Locale");
        assert_eq!(set_locale.label.resolve(Terminology::Native, Locale::De), "Sprache festlegen");
    }

    #[test]
    fn results_window_buckling_with_no_load_case_shows_placeholder_2d() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = Fem2dConfig { result_mode: "buckling".into(), result_mode_index: 0, ..Fem2dConfig::default() };
        let cfg = ConfigView { projection: &config };
        let json = serde_json::to_string(&app.render(FEM2D_BODY_RESULTS, &doc, &cfg)).unwrap();
        assert!(json.contains("No load case defined"), "{json}");
    }
    //#endregion 🔖️UnknownBodyAndGermanLabels

    //#region 🔖️MoreStructureAndLoadActions
    #[test]
    fn add_bar_and_add_beam_actions_emit_ops_2d() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };

        let emit_bar = app.handle(&Fem2dCommand::AddBar { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() }, &doc, &cfg);
        match &emit_bar.document_operations[0] {
            Fem2dOperation::SetElement { element, .. } => assert!(matches!(**element, fem2d::FemElement::Bar { .. })),
            _ => panic!("expected SetElement"),
        }

        let emit_beam = app.handle(&Fem2dCommand::AddBeam { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() }, &doc, &cfg);
        match &emit_beam.document_operations[0] {
            Fem2dOperation::SetElement { element, .. } => assert!(matches!(**element, fem2d::FemElement::Beam { .. })),
            _ => panic!("expected SetElement"),
        }
    }

    #[test]
    fn add_material_action_emits_op_2d() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let emit = app.handle(&Fem2dCommand::AddMaterial { name: "Steel".into(), e: 2.1e11 }, &doc, &cfg);
        match &emit.document_operations[0] {
            Fem2dOperation::SetMaterial { material, .. } => {
                assert_eq!(material.name, "Steel");
                assert_eq!(material.e, 2.1e11);
            }
            _ => panic!("expected SetMaterial"),
        }
    }

    #[test]
    fn add_section_action_emits_op_2d() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let command = Fem2dCommand::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369 };
        let emit = app.handle(&command, &doc, &cfg);
        match &emit.document_operations[0] {
            Fem2dOperation::SetSection { section, .. } => assert_eq!(section.name, "HEA200"),
            _ => panic!("expected SetSection"),
        }
    }

    #[test]
    fn add_support_action_emits_op_with_fixed_dofs_2d() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let command = Fem2dCommand::AddSupport { node_id: "n1".into(), fixed: vec![fem2d::FemDof::Tx, fem2d::FemDof::Ty] };
        let emit = app.handle(&command, &doc, &cfg);
        match &emit.document_operations[0] {
            Fem2dOperation::SetSupport { support, .. } => assert_eq!(support.fixed, vec![fem2d::FemDof::Tx, fem2d::FemDof::Ty]),
            _ => panic!("expected SetSupport"),
        }
    }

    #[test]
    fn add_nodal_load_action_targets_named_case_2d() {
        let app = Fem2dPlayApp::default();
        let mut projection = fem2d_engine::empty_fem2d_projection();
        projection.load_cases.push(fem2d::FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: false });
        projection.load_cases.push(fem2d::FemLoadCase { id: "live".into(), name: "Live".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let command = Fem2dCommand::AddNodalLoad { node_id: "n1".into(), dof: fem2d::FemDof::Ty, value: -5000.0, case_id: Some("live".into()) };
        let emit = app.handle(&command, &doc, &cfg);
        match &emit.document_operations[0] {
            Fem2dOperation::SetLoadCase { index, load_case } => {
                assert_eq!(*index, 1);
                assert!(matches!(load_case.loads[0], fem2d::FemLoad::Nodal { .. }));
            }
            _ => panic!("expected SetLoadCase"),
        }
    }

    #[test]
    fn add_member_udl_action_emits_op_2d() {
        let app = Fem2dPlayApp::default();
        let mut projection = fem2d_engine::empty_fem2d_projection();
        projection.load_cases.push(fem2d::FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let command = Fem2dCommand::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: -500.0, case_id: None };
        let emit = app.handle(&command, &doc, &cfg);
        match &emit.document_operations[0] {
            Fem2dOperation::SetLoadCase { load_case, .. } => assert!(matches!(load_case.loads[0], fem2d::FemLoad::MemberUdl { .. })),
            _ => panic!("expected SetLoadCase"),
        }
    }

    #[test]
    fn set_camera_action_writes_config_not_document_operations() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let emit = app.handle(&Fem2dCommand::SetCamera { x: 1.0, y: 2.0, zoom: 1.5 }, &doc, &cfg);
        assert!(emit.document_operations.is_empty(), "setCamera must not emit a document VCS operation");
        assert_eq!(emit.config_operations, vec![Fem2dConfigOperation::SetCamera { camera: FemCamera { x: 1.0, y: 2.0, zoom: 1.5 } }]);
    }

    #[test]
    fn remove_selection_covers_nodes_elements_materials_sections_supports_load_cases_2d() {
        let app = Fem2dPlayApp::default();
        let mut projection = fem2d_engine::empty_fem2d_projection();
        projection.nodes.push(fem2d::FemNode { id: "n1".into(), x: 0.0, y: 0.0 });
        projection.elements.push(fem2d::FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n1".into(), material_id: "m1".into(), section_id: "s1".into() });
        projection.materials.push(fem2d::FemMaterial { id: "m1".into(), name: "Steel".into(), e: 2.1e11, nu: 0.3, rho: 7850.0 });
        projection.sections.push(fem2d::FemSection { id: "s1".into(), name: "Sec".into(), area: 0.01, iy: 0.0001 });
        projection.supports.push(fem2d::FemSupport { id: "sup1".into(), node_id: "n1".into(), fixed: vec![fem2d::FemDof::Tx] });
        projection.load_cases.push(fem2d::FemLoadCase { id: "case1".into(), name: "Case".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let ids = vec!["n1".into(), "e1".into(), "m1".into(), "s1".into(), "sup1".into(), "case1".into()];
        let emit = app.handle(&Fem2dCommand::RemoveSelection { ids }, &doc, &cfg);
        assert_eq!(emit.document_operations.len(), 6);
        assert!(matches!(emit.document_operations[0], Fem2dOperation::RemoveNode { .. }));
        assert!(matches!(emit.document_operations[1], Fem2dOperation::RemoveElement { .. }));
        assert!(matches!(emit.document_operations[2], Fem2dOperation::RemoveMaterial { .. }));
        assert!(matches!(emit.document_operations[3], Fem2dOperation::RemoveSection { .. }));
        assert!(matches!(emit.document_operations[4], Fem2dOperation::RemoveSupport { .. }));
        assert!(matches!(emit.document_operations[5], Fem2dOperation::RemoveLoadCase { .. }));
    }
    //#endregion 🔖️MoreStructureAndLoadActions

    //#region 🔖️CommandId
    #[test]
    fn command_id_maps_every_variant_to_a_declared_action_id() {
        let app = Fem2dPlayApp::default();
        let definition = create_fem2d_app().definition;
        let samples: Vec<Fem2dCommand> = vec![
            Fem2dCommand::AddNode { x: 0.0, y: 0.0 },
            Fem2dCommand::AddBar { start: "n1".into(), end: "n2".into(), material_id: "m".into(), section_id: "s".into() },
            Fem2dCommand::AddBeam { start: "n1".into(), end: "n2".into(), material_id: "m".into(), section_id: "s".into() },
            Fem2dCommand::AddMaterial { name: "Steel".into(), e: 2.1e11 },
            Fem2dCommand::AddSection { name: "Sec".into(), area: 0.01, iy: 0.0001 },
            Fem2dCommand::AddSupport { node_id: "n1".into(), fixed: vec![] },
            Fem2dCommand::AddNodalLoad { node_id: "n1".into(), dof: fem2d::FemDof::Ty, value: 0.0, case_id: None },
            Fem2dCommand::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: 0.0, case_id: None },
            Fem2dCommand::AddAreaLoad { region_id: "r1".into(), pressure: 0.0, case_id: None },
            Fem2dCommand::AddRegion { x: 0.0, y: 0.0, width: 1.0, height: 1.0, material_id: "m".into(), thickness: None, mesh_size: None },
            Fem2dCommand::AddLoadCase { name: "Case".into(), self_weight: false },
            Fem2dCommand::AddCombination { name: "Combo".into(), terms: vec![] },
            Fem2dCommand::SetSelfWeight { case_id: "case".into(), enabled: true },
            Fem2dCommand::SetAnalysisSettings { modal_count: None, buckling_count: None, deformation_scale: None },
            Fem2dCommand::RemoveSelection { ids: vec![] },
            Fem2dCommand::SetActiveExample { example_id: "default".into() },
            Fem2dCommand::SetCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            Fem2dCommand::SetResultDisplay { source_id: None, mode: "static".into(), mode_index: 0 },
            Fem2dCommand::SetLocale { value: "en-US".into() },
        ];
        for command in &samples {
            let id = app.command_id(command);
            assert!(definition.actions.iter().any(|action| action.id == id), "command_id {id} (for {command:?}) must be a declared action");
        }
    }
    //#endregion 🔖️CommandId

    //#region 🔖️ConfigAndIo
    #[test]
    fn config_spec_declares_no_fields() {
        assert!(Fem2dPlayApp::default().config_spec().fields.is_empty());
    }

    #[test]
    fn manifest_declares_config_io_and_computation_artifact_kind() {
        let definition = create_fem2d_app().definition;
        assert!(definition.config.fields.is_empty());
        assert_eq!(definition.io.document_schema, fem2d::FEM_2D_SCHEMA);
        let computation_kind = definition.artifact_kinds.iter().find(|kind| kind.id == "computation.fem2d").expect("computation.fem2d artifact kind declared");
        assert_eq!(computation_kind.media_type.class, MediaClass::Computation);
        assert_eq!(computation_kind.media_type.form, MediaForm::Value);
    }

    #[test]
    fn app_io_forwards_the_engine_declared_ports() {
        let io = Fem2dPlayApp::default().io().expect("io declared");
        assert!(io.ports.iter().any(|port| port.id == "geometry:in"));
        assert!(io.ports.iter().any(|port| port.id == "results:out"));
    }
    //#endregion 🔖️ConfigAndIo

    //#region 🔖️ExportImportMedia
    #[test]
    fn export_media_document_out_round_trips_via_import_media_document_in() {
        let app = Fem2dPlayApp::default();
        let projection: Fem2dDocument = Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let media = app.export_media("document:out", &doc).expect("document:out exports");
        assert_eq!(media.media_type.class, MediaClass::TwoD);
        assert_eq!(media.media_type.form, MediaForm::Vector);
        let empty_projection = fem2d_engine::empty_fem2d_projection();
        let empty_history = history_view();
        let empty_doc = DocumentView { projection: &empty_projection, history: &empty_history };
        let emit = app.import_media("document:in", &media, &empty_doc).expect("document:in imports");
        assert_eq!(emit.document_operations.len(), 1);
        match &emit.document_operations[0] {
            Fem2dOperation::SetDocument { document } => assert_eq!(document, &projection),
            _ => panic!("expected SetDocument"),
        }
    }

    #[test]
    fn export_media_results_out_returns_json_with_every_case_and_combination() {
        let app = Fem2dPlayApp::default();
        let projection: Fem2dDocument = Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let media = app.export_media("results:out", &doc).expect("results:out exports");
        assert_eq!(media.media_type.class, MediaClass::Data);
        assert_eq!(media.media_type.form, MediaForm::Value);
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "computation.fem2d");
                let value: Value = serde_json::from_str(&json).expect("results:out payload is valid JSON");
                for case_id in ["dead", "live", "uls"] {
                    let result = value.get(case_id).unwrap_or_else(|| panic!("missing {case_id} in results:out payload: {value}"));
                    assert!(result.get("displacements").is_some());
                    assert!(result.get("reactions").is_some());
                    assert!(result.get("checks").is_some());
                }
            }
            MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
        }
    }

    #[test]
    fn export_media_results_out_errors_when_no_load_cases_are_defined() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let error = app.export_media("results:out", &doc).expect_err("no load cases means no results to export");
        match error {
            MediaError::Payload(port, _) => assert_eq!(port, "results:out"),
            other => panic!("expected MediaError::Payload, got {other:?}"),
        }
    }

    #[test]
    fn export_media_unknown_port_is_not_implemented() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        assert!(matches!(app.export_media("bogus:out", &doc), Err(MediaError::NotImplemented)));
    }

    #[test]
    fn import_media_geometry_in_builds_a_new_region_from_the_first_material() {
        let app = Fem2dPlayApp::default();
        let mut projection = fem2d_engine::empty_fem2d_projection();
        projection.materials.push(fem2d::FemMaterial { id: "steel".into(), name: "Steel".into(), e: 2.1e11, nu: 0.3, rho: 7850.0 });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let payload = json!({ "outline": [[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], "holes": [] }).to_string();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "geometry".into(), json: payload } };
        let emit = app.import_media("geometry:in", &media, &doc).expect("geometry:in imports");
        assert_eq!(emit.document_operations.len(), 1);
        match &emit.document_operations[0] {
            Fem2dOperation::SetRegion { region, .. } => {
                assert_eq!(region.outline, vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]]);
                assert!(region.holes.is_empty());
                assert_eq!(region.material_id, "steel");
            }
            _ => panic!("expected SetRegion"),
        }
    }

    #[test]
    fn import_media_geometry_in_falls_back_to_unassigned_material_when_none_exists() {
        let app = Fem2dPlayApp::default();
        let projection = fem2d_engine::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let payload = json!({ "outline": [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]] }).to_string();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "geometry".into(), json: payload } };
        let emit = app.import_media("geometry:in", &media, &doc).expect("geometry:in imports");
        match &emit.document_operations[0] {
            Fem2dOperation::SetRegion { region, .. } => assert_eq!(region.material_id, "unassigned"),
            _ => panic!("expected SetRegion"),
        }
    }
    //#endregion 🔖️ExportImportMedia
}
//#endregion 🧪️Tests
