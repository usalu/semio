//! 🏗️ FEM WASM plugin: `fem2d-play` and `fem3d-play` apps registered as one hot-swappable component.

use fem_core::{Dof, ElementResult};
use semio_framework_plugin::{
    build_canvas_2d_scene, build_world_3d_scene, create_default_layout, ui_stack_vertical, ui_text, world3d_default_camera,
    world3d_default_selection_json, world3d_meshes_json_from_kinds, world3d_scene, AppLabelsOverlay, ActionArgDef,
    ActionArgOption, ActionEmit, App, Canvas2dScene, DocumentApp, DocumentView, SurfaceKind, UiNode,
    ViewState, WorldSunConfig,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use vcs::DocumentDsl;

//#region 🔖Constants
const FEM2D_APP_ID: &str = "fem2d-play";
const FEM2D_WINDOW_MODEL: &str = "fem2d-model";
const FEM2D_WINDOW_RESULTS: &str = "fem2d-results";
const FEM2D_BODY_MODEL: &str = "fem2d.play.model";
const FEM2D_BODY_RESULTS: &str = "fem2d.play.results";

const FEM3D_APP_ID: &str = "fem3d-play";
const FEM3D_WINDOW_MODEL: &str = "fem3d-model";
const FEM3D_WINDOW_RESULTS: &str = "fem3d-results";
const FEM3D_BODY_MODEL: &str = "fem3d.play.model";
const FEM3D_BODY_RESULTS: &str = "fem3d.play.results";

/// 📦 The `fem2d-play`/`fem3d-play` "default" examples, embedded at compile time as handcrafted
/// `.fem2d`/`.fem3d` DSL text (see `fem_2d`/`fem_3d`'s `🔖Dsl` regions) — shared by the manifest's
/// `.example(...)` registration, the `setActiveExample` handler, and every test fixture.
const FEM2D_EXAMPLE_DSL: &str = include_str!("../../2d/example/default.fem2d");
const FEM3D_EXAMPLE_DSL: &str = include_str!("../../3d/example/default.fem3d");

/// 📐 Model-meters -> screen-pixels scale for the 2D canvas (a 6m span shouldn't render as 6px wide).
const SCALE_2D: f64 = 20.0;
/// 📐 Screen-space origin offset so a structure anchored at (0,0) isn't drawn at the canvas corner.
const ORIGIN_2D: f64 = 40.0;
/// 📐 Exaggeration factor for offsetting the moment-diagram polyline perpendicular to a member.
const MOMENT_SCALE_2D: f64 = 0.001;

/// 📐 Mode shapes are normalized to unit peak displacement (see `normalize_mode_shape`), so a single
/// ratio of the model's own extent gives a visually consistent, deterministic amplitude for both
/// 2D and 3D modal/buckling overlays regardless of the eigen-solver's arbitrary shape normalization.
const MODE_SHAPE_AMPLITUDE_RATIO: f64 = 0.1;

/// 🧊 Half-extent-ish scale of the small box instance drawn at each node.
const NODE_SIZE_3D: f64 = 0.05;
/// 🧊 Cross-section (x/y) thickness of the oriented box prism drawn for each `Bar`/`Frame` member —
/// a fixed visual thickness, not the member's actual section dimensions (see `fem3d_structural_instances`).
const MEMBER_THICKNESS_3D: f64 = 0.05;

/// 🎨 Blue→green→yellow→red banded ramp for von Mises stress contour fill colors, low to high.
const VON_MISES_BANDS: [&str; 8] = ["#1d4ed8", "#2563eb", "#0ea5e9", "#22c55e", "#eab308", "#f97316", "#ef4444", "#b91c1c"];
/// 🎨 Muted color for the mesh-edge preview overlay drawn under the model window's members.
const MESH_EDGE_COLOR: &str = "#475569";
//#endregion 🔖Constants

//#region 🔖Shared
/// 🪪 Finds the smallest `"{prefix}{n}"` id not already present in `existing`.
fn next_id(existing: impl Iterator<Item = String>, prefix: &str) -> String {
    let ids: std::collections::HashSet<String> = existing.collect();
    let mut i = ids.len();
    loop {
        let candidate = format!("{prefix}{i}");
        if !ids.contains(&candidate) {
            return candidate;
        }
        i += 1;
    }
}

/// 🎨 Parses a `"#rrggbb"` hex color into 0..1 float components for a Canvas2d `fill.color` array.
fn hex_to_rgb01(hex: &str) -> (f64, f64, f64) {
    let h = hex.trim_start_matches('#');
    let component = |slice: &str| u8::from_str_radix(slice, 16).unwrap_or(0) as f64 / 255.0;
    (component(&h[0..2]), component(&h[2..4]), component(&h[4..6]))
}

/// 📐 Rescales a node-id-keyed displacement map in place so its largest translational magnitude
/// (`sqrt(tx²+ty²+tz²)`) becomes exactly 1.0 — mode shapes from `subspace_iteration` are mass/Kg-
/// orthonormalized (arbitrary physical magnitude), so this gives a deterministic, comparable-across-
/// modes amplitude before scaling by `MODE_SHAPE_AMPLITUDE_RATIO * model_extent`. A near-zero shape
/// (degenerate/rigid mode) is left untouched rather than divided by a near-zero magnitude.
fn normalize_mode_shape(disp_map: &mut HashMap<String, [f64; 6]>) {
    let peak = disp_map
        .values()
        .map(|d| (d[Dof::Tx.index()].powi(2) + d[Dof::Ty.index()].powi(2) + d[Dof::Tz.index()].powi(2)).sqrt())
        .fold(0.0_f64, f64::max);
    if peak < 1e-12 {
        return;
    }
    for values in disp_map.values_mut() {
        for v in values.iter_mut() {
            *v /= peak;
        }
    }
}

/// 🌡️ Maps `value` within `[min, max]` onto one of `VON_MISES_BANDS`' 8 hex colors, low to high.
fn von_mises_color(value: f64, min: f64, max: f64) -> &'static str {
    let span = (max - min).max(1e-9);
    let t = ((value - min) / span).clamp(0.0, 1.0);
    let index = ((t * (VON_MISES_BANDS.len() - 1) as f64).round() as usize).min(VON_MISES_BANDS.len() - 1);
    VON_MISES_BANDS[index]
}

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
//#endregion 🔖Shared

//#region 🔖Fem2dRender
fn screen_2d(x: f64, y: f64) -> (f64, f64) {
    (x * SCALE_2D + ORIGIN_2D, -y * SCALE_2D + ORIGIN_2D)
}

fn find_node_2d<'a>(nodes: &'a [fem_2d::FemNode], id: &str) -> Option<&'a fem_2d::FemNode> {
    nodes.iter().find(|n| n.id == id)
}

fn fem2d_element_endpoints(element: &fem_2d::FemElement) -> (&str, &str) {
    match element {
        fem_2d::FemElement::Bar { start, end, .. } | fem_2d::FemElement::Beam { start, end, .. } => (start.as_str(), end.as_str()),
    }
}

/// 🔎 Finds the load case an incoming load/self-weight edit should target: the named `case_id` if it
/// exists, else the first case, else a freshly synthesized `"case-1"` — shared by `addNodalLoad`,
/// `addMemberUdl`, `addAreaLoad`, and `setSelfWeight` so every load-mutating action resolves its
/// target case the same way. Returns the case's collection index (`load_cases.len()` for a fresh one)
/// alongside an owned clone ready to be mutated and re-emitted via `SetLoadCase`.
fn fem2d_resolve_load_case(doc: &fem_2d::Fem2dDocument, case_id: Option<&str>) -> (usize, fem_2d::FemLoadCase) {
    let named = case_id.and_then(|id| doc.load_cases.iter().find(|lc| lc.id == id).cloned());
    let load_case = named
        .or_else(|| doc.load_cases.first().cloned())
        .unwrap_or_else(|| fem_2d::FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: Vec::new(), self_weight: false });
    let index = doc.load_cases.iter().position(|lc| lc.id == load_case.id).unwrap_or(doc.load_cases.len());
    (index, load_case)
}

/// 📐 Bounding-box diagonal (in model meters) over every node plus every region outline vertex — the
/// reference length `MODE_SHAPE_AMPLITUDE_RATIO` scales a normalized mode shape against. Falls back to
/// `1.0` for a degenerate (empty or point-like) model so mode-shape rendering never divides by zero.
fn fem2d_model_extent(doc: &fem_2d::Fem2dDocument) -> f64 {
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
fn fem2d_structure_layers(doc: &fem_2d::Fem2dDocument, node_color: &str, line_color: &str, support_color: &str) -> Vec<Value> {
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
            layers.push(json!({ "kind": "line", "id": format!("el-{}", fem_2d::element_id(element)), "x0": x0, "y0": y0, "x1": x1, "y1": y1, "color": line_color }));
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
fn fem2d_region_triangles(doc: &fem_2d::Fem2dDocument) -> Vec<(String, [(f64, f64); 3])> {
    let mut out = Vec::new();
    let Ok(meshes) = fem_2d::fem2d_mesh_preview(doc) else { return out };
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
fn fem2d_region_mesh_triangles(doc: &fem_2d::Fem2dDocument) -> Vec<(String, [(f64, f64); 3], [String; 3])> {
    let mut out = Vec::new();
    let Ok(meshes) = fem_2d::fem2d_mesh_preview(doc) else { return out };
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
fn fem2d_deformed_shape_layers(doc: &fem_2d::Fem2dDocument, disp_map: &HashMap<String, [f64; 6]>, deform_scale: f64) -> Vec<Value> {
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
            "id": format!("deformed-{}", fem_2d::element_id(element)),
            "points": [[x0 + dx0, y0 + dy0], [x1 + dx1, y1 + dy1]],
            "color": "#f472b6",
        }));
    }
    layers
}

fn render_fem2d_model(doc: &fem_2d::Fem2dDocument) -> UiNode {
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
    build_canvas_2d_scene(FEM2D_BODY_MODEL, FEM2D_APP_ID, Canvas2dScene { camera_x: doc.camera.x, camera_y: doc.camera.y, zoom: doc.camera.zoom, layers_json })
}

/// 📊 Results window dispatcher — picks the static/modal/buckling render based on `display`.
fn render_fem2d_results(doc: &fem_2d::Fem2dDocument, display: &ResultDisplay) -> UiNode {
    match display.mode {
        DisplayMode::Static => render_fem2d_results_static(doc, display.source_id.as_deref()),
        DisplayMode::Modal(mode_index) => render_fem2d_results_modal(doc, mode_index),
        DisplayMode::Buckling(mode_index) => render_fem2d_results_buckling(doc, display.source_id.as_deref(), mode_index),
    }
}

/// 📊 Static results: undeformed structure faintly, plus a deformed-shape polyline, text labels at
/// every support reaction, (for beams) a moment-diagram polyline, and (for meshed regions) a
/// nodal-averaged, marching-triangle-banded von-Mises stress contour with a color-swatch legend.
/// `source_id` selects a `fem2d_solve_all` case/combination id, falling back to the first load case
/// when `None`/unknown (preserves v0's default behavior).
fn render_fem2d_results_static(doc: &fem_2d::Fem2dDocument, source_id: Option<&str>) -> UiNode {
    let results = match fem_2d::fem2d_solve_all(doc) {
        Ok(results) => results,
        Err(e) => return ui_text(format!("Analysis error: {e}")),
    };
    let case_id = source_id
        .filter(|id| results.contains_key(*id))
        .map(str::to_string)
        .or_else(|| doc.load_cases.first().map(|c| c.id.clone()));
    let Some(case_id) = case_id else {
        return ui_text("No load case defined");
    };
    let Some(result) = results.get(&case_id) else {
        return ui_text(format!("Result not found: {case_id}"));
    };

    let mut layers = fem2d_structure_layers(doc, "#334155", "#334155", "#334155");
    let mut disp_map: HashMap<String, [f64; 6]> = HashMap::new();
    for d in &result.displacements {
        disp_map.insert(d.node_id.clone(), d.values);
    }
    layers.extend(fem2d_deformed_shape_layers(doc, &disp_map, doc.analysis.deformation_scale));

    //#region 🔖ReactionLabels
    for reaction in &result.reactions {
        let Some(node) = find_node_2d(&doc.nodes, &reaction.node_id) else { continue };
        let (sx, sy) = screen_2d(node.x, node.y);
        layers.push(json!({
            "id": format!("reaction-{}-{:?}", reaction.node_id, reaction.dof),
            "transform": [1.0, 0.0, 0.0, 1.0, sx + 8.0, sy + 14.0],
            "text": { "content": format!("{:?}: {:.0} N", reaction.dof, reaction.value), "size": 10.0 },
        }));
    }
    //#endregion 🔖ReactionLabels

    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(n1), Some(n2)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let (x0, y0) = screen_2d(n1.x, n1.y);
        let (x1, y1) = screen_2d(n2.x, n2.y);
        if let Some((_, ElementResult::Beam { stations })) = result.elements.iter().find(|(id, _)| id.as_str() == fem_2d::element_id(element)) {
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
                "id": format!("moment-{}", fem_2d::element_id(element)),
                "points": points,
                "color": "#fbbf24",
            }));
        }
    }

    //#region 🔖StressContour
    let nodal_von_mises = fem_2d::fem2d_nodal_von_mises(doc, &case_id).unwrap_or_default();
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
    //#endregion 🔖StressContour

    let layers_json = serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into());
    build_canvas_2d_scene(FEM2D_BODY_RESULTS, FEM2D_APP_ID, Canvas2dScene { camera_x: doc.camera.x, camera_y: doc.camera.y, zoom: doc.camera.zoom, layers_json })
}

/// 📊 Modal mode-shape overlay: undeformed structure faintly plus the selected mode's deformed-shape
/// polyline (normalized to unit peak, then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own
/// extent — see `normalize_mode_shape`) and a frequency caption.
fn render_fem2d_results_modal(doc: &fem_2d::Fem2dDocument, mode_index: usize) -> UiNode {
    let (freq_hz, mut disp_map) = match fem_2d::fem2d_modal_mode_values(doc, mode_index) {
        Ok(values) => values,
        Err(e) => return ui_text(format!("Modal analysis error: {e}")),
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
    build_canvas_2d_scene(FEM2D_BODY_RESULTS, FEM2D_APP_ID, Canvas2dScene { camera_x: doc.camera.x, camera_y: doc.camera.y, zoom: doc.camera.zoom, layers_json })
}

/// 📊 Buckling mode-shape overlay: undeformed structure faintly plus the selected mode's deformed-shape
/// polyline (normalized to unit peak, then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own
/// extent — see `normalize_mode_shape`) and a load-factor caption. `source_id` selects the reference
/// load case, falling back to the first load case when `None`.
fn render_fem2d_results_buckling(doc: &fem_2d::Fem2dDocument, source_id: Option<&str>, mode_index: usize) -> UiNode {
    let Some(case_id) = source_id.map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone())) else {
        return ui_text("No load case defined");
    };
    let (factor, mut disp_map) = match fem_2d::fem2d_buckling_mode_values(doc, &case_id, mode_index) {
        Ok(values) => values,
        Err(e) => return ui_text(format!("Buckling analysis error: {e}")),
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
    build_canvas_2d_scene(FEM2D_BODY_RESULTS, FEM2D_APP_ID, Canvas2dScene { camera_x: doc.camera.x, camera_y: doc.camera.y, zoom: doc.camera.zoom, layers_json })
}
//#endregion 🔖Fem2dRender

//#region 🔖ResultDisplay
/// 👁️ Ephemeral (non-document) view state selecting what the results window shows — which
/// `fem2d_solve_all`/`fem3d_solve_all` case-or-combination id (`source_id`) and which `DisplayMode`.
/// Mutated by the `setResultDisplay` VIEW action (`ActionEmit::default()`, no operations — never recorded in
/// history) and lives directly on the app struct, per `DocumentApp::handle_action`'s `&mut self`.
#[derive(Clone, Debug, Default)]
struct ResultDisplay {
    source_id: Option<String>,
    mode: DisplayMode,
}

/// 👁️ Which analysis result the results window renders: the static solve, or the `n`-th modal/buckling
/// mode shape (0-indexed).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum DisplayMode {
    #[default]
    Static,
    Modal(usize),
    Buckling(usize),
}

/// 👁️ Parses `setResultDisplay`'s `{"sourceId"?, "mode": "static"|"modal"|"buckling", "modeIndex"?}`
/// args into a `ResultDisplay` — unknown/missing `mode` falls back to `Static`.
fn parse_result_display(args: Option<&Value>) -> ResultDisplay {
    let source_id = args.and_then(|v| v.get("sourceId")).and_then(Value::as_str).map(str::to_string);
    let mode_index = args.and_then(|v| v.get("modeIndex")).and_then(Value::as_u64).unwrap_or(0) as usize;
    let mode = match args.and_then(|v| v.get("mode")).and_then(Value::as_str) {
        Some("modal") => DisplayMode::Modal(mode_index),
        Some("buckling") => DisplayMode::Buckling(mode_index),
        _ => DisplayMode::Static,
    };
    ResultDisplay { source_id, mode }
}
//#endregion 🔖ResultDisplay

//#region 🔖Fem2dPlayApp
/// 🧮 v0 design: results are never persisted or cached — `fem2d_solve`/`fem2d_solve_all` run fresh
/// inside `render()` whenever the results window is drawn. At v0 scale (≤10 nodes) this is cheap and
/// correct-by-construction (no cache-invalidation bugs to get wrong). There is no `RunAnalysis` operation:
/// solving is a pure function of the document. `result_display` is ephemeral view state (see
/// `ResultDisplay`'s doc) — mutated by the `setResultDisplay` view action, defaulting to the first
/// load case in `Static` mode.
#[derive(Default)]
struct Fem2dPlayApp {
    result_display: ResultDisplay,
}

impl DocumentApp for Fem2dPlayApp {
    type Projection = fem_2d::Fem2dDocument;
    type Operation = fem_2d::Fem2dOperation;

    fn app_id(&self) -> &str {
        FEM2D_APP_ID
    }

    fn document_schema(&self) -> &str {
        fem_2d::FEM_2D_SCHEMA
    }

    fn initial_projection(&self) -> fem_2d::Fem2dDocument {
        fem_2d::empty_fem2d_projection()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, fem_2d::Fem2dDocument>, _view_state: &ViewState) -> ActionEmit<fem_2d::Fem2dOperation> {
        match action {
            "addNode" => {
                if let (Some(x), Some(y)) = (args.and_then(|v| v.get("x")).and_then(Value::as_f64), args.and_then(|v| v.get("y")).and_then(Value::as_f64)) {
                    let id = next_id(doc.projection.nodes.iter().map(|n| n.id.clone()), "n");
                    let index = doc.projection.nodes.len();
                    return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetNode { index, node: fem_2d::FemNode { id, x, y } }]);
                }
            }
            "addBar" | "addBeam" => {
                if let (Some(start), Some(end), Some(material_id), Some(section_id)) = (
                    args.and_then(|v| v.get("start")).and_then(Value::as_str),
                    args.and_then(|v| v.get("end")).and_then(Value::as_str),
                    args.and_then(|v| v.get("materialId")).and_then(Value::as_str),
                    args.and_then(|v| v.get("sectionId")).and_then(Value::as_str),
                ) {
                    let id = next_id(doc.projection.elements.iter().map(|e| fem_2d::element_id(e).to_string()), "e");
                    let index = doc.projection.elements.len();
                    let element = if action == "addBar" {
                        fem_2d::FemElement::Bar { id, start: start.into(), end: end.into(), material_id: material_id.into(), section_id: section_id.into() }
                    } else {
                        fem_2d::FemElement::Beam { id, start: start.into(), end: end.into(), material_id: material_id.into(), section_id: section_id.into() }
                    };
                    return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetElement { index, element }]);
                }
            }
            "addMaterial" => {
                if let (Some(name), Some(e)) = (args.and_then(|v| v.get("name")).and_then(Value::as_str), args.and_then(|v| v.get("e")).and_then(Value::as_f64)) {
                    let id = next_id(doc.projection.materials.iter().map(|m| m.id.clone()), "m");
                    let index = doc.projection.materials.len();
                    return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetMaterial { index, material: fem_2d::FemMaterial { id, name: name.into(), e, nu: 0.3, rho: 7850.0 } }]);
                }
            }
            "addSection" => {
                if let (Some(name), Some(area), Some(iy)) = (
                    args.and_then(|v| v.get("name")).and_then(Value::as_str),
                    args.and_then(|v| v.get("area")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("iy")).and_then(Value::as_f64),
                ) {
                    let id = next_id(doc.projection.sections.iter().map(|s| s.id.clone()), "s");
                    let index = doc.projection.sections.len();
                    return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetSection { index, section: fem_2d::FemSection { id, name: name.into(), area, iy } }]);
                }
            }
            "addSupport" => {
                if let Some(node_id) = args.and_then(|v| v.get("nodeId")).and_then(Value::as_str) {
                    let fixed: Vec<Dof> = args.and_then(|v| v.get("fixed")).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                    let id = next_id(doc.projection.supports.iter().map(|s| s.id.clone()), "sup");
                    let index = doc.projection.supports.len();
                    return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetSupport { index, support: fem_2d::FemSupport { id, node_id: node_id.into(), fixed } }]);
                }
            }
            "addNodalLoad" => {
                if let (Some(node_id), Some(dof), Some(value)) = (
                    args.and_then(|v| v.get("nodeId")).and_then(Value::as_str),
                    args.and_then(|v| v.get("dof")).and_then(|v| serde_json::from_value::<Dof>(v.clone()).ok()),
                    args.and_then(|v| v.get("value")).and_then(Value::as_f64),
                ) {
                    let case_id = args.and_then(|v| v.get("caseId")).and_then(Value::as_str);
                    let (index, mut load_case) = fem2d_resolve_load_case(doc.projection, case_id);
                    let load_id = next_id(load_case.loads.iter().map(|l| fem_2d::load_id(l).to_string()), "l");
                    load_case.loads.push(fem_2d::FemLoad::Nodal { id: load_id, node_id: node_id.into(), dof, value });
                    return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetLoadCase { index, load_case }]);
                }
            }
            "addMemberUdl" => {
                if let (Some(element_id), Some(wx), Some(wy)) = (
                    args.and_then(|v| v.get("elementId")).and_then(Value::as_str),
                    args.and_then(|v| v.get("wx")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("wy")).and_then(Value::as_f64),
                ) {
                    let case_id = args.and_then(|v| v.get("caseId")).and_then(Value::as_str);
                    let (index, mut load_case) = fem2d_resolve_load_case(doc.projection, case_id);
                    let load_id = next_id(load_case.loads.iter().map(|l| fem_2d::load_id(l).to_string()), "l");
                    load_case.loads.push(fem_2d::FemLoad::MemberUdl { id: load_id, element_id: element_id.into(), wx, wy });
                    return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetLoadCase { index, load_case }]);
                }
            }
            "addAreaLoad" => {
                if let (Some(region_id), Some(pressure)) = (args.and_then(|v| v.get("regionId")).and_then(Value::as_str), args.and_then(|v| v.get("pressure")).and_then(Value::as_f64)) {
                    let case_id = args.and_then(|v| v.get("caseId")).and_then(Value::as_str);
                    let (index, mut load_case) = fem2d_resolve_load_case(doc.projection, case_id);
                    let load_id = next_id(load_case.loads.iter().map(|l| fem_2d::load_id(l).to_string()), "l");
                    load_case.loads.push(fem_2d::FemLoad::Area { id: load_id, region_id: region_id.into(), pressure });
                    return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetLoadCase { index, load_case }]);
                }
            }
            "addRegion" => {
                if let (Some(x), Some(y), Some(width), Some(height), Some(material_id)) = (
                    args.and_then(|v| v.get("x")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("y")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("width")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("height")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("materialId")).and_then(Value::as_str),
                ) {
                    let thickness = args.and_then(|v| v.get("thickness")).and_then(Value::as_f64).unwrap_or(0.02);
                    let mesh_size = args.and_then(|v| v.get("meshSize")).and_then(Value::as_f64).unwrap_or(0.25);
                    let id = next_id(doc.projection.regions.iter().map(|r| r.id.clone()), "r");
                    let index = doc.projection.regions.len();
                    let outline = vec![[x, y], [x + width, y], [x + width, y + height], [x, y + height]];
                    return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetRegion { index, region: fem_2d::FemRegion { id, name: "Region".into(), outline, holes: Vec::new(), thickness, material_id: material_id.into(), mesh_size } }]);
                }
            }
            "addLoadCase" => {
                if let Some(name) = args.and_then(|v| v.get("name")).and_then(Value::as_str) {
                    let self_weight = args.and_then(|v| v.get("selfWeight")).and_then(Value::as_bool).unwrap_or(false);
                    let id = next_id(doc.projection.load_cases.iter().map(|lc| lc.id.clone()), "case-");
                    let index = doc.projection.load_cases.len();
                    return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetLoadCase { index, load_case: fem_2d::FemLoadCase { id, name: name.into(), loads: Vec::new(), self_weight } }]);
                }
            }
            "addCombination" => {
                if let (Some(name), Some(terms_json)) = (args.and_then(|v| v.get("name")).and_then(Value::as_str), args.and_then(|v| v.get("terms")).and_then(Value::as_str)) {
                    if let Ok(terms) = serde_json::from_str::<Vec<(String, f64)>>(terms_json) {
                        let id = next_id(doc.projection.combinations.iter().map(|c| c.id.clone()), "c");
                        let index = doc.projection.combinations.len();
                        return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetCombination { index, combination: fem_2d::FemCombination { id, name: name.into(), terms } }]);
                    }
                }
            }
            "setSelfWeight" => {
                if let (Some(case_id), Some(enabled)) = (args.and_then(|v| v.get("caseId")).and_then(Value::as_str), args.and_then(|v| v.get("enabled")).and_then(Value::as_bool)) {
                    if let Some(index) = doc.projection.load_cases.iter().position(|lc| lc.id == case_id) {
                        let mut load_case = doc.projection.load_cases[index].clone();
                        load_case.self_weight = enabled;
                        return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetLoadCase { index, load_case }]);
                    }
                }
            }
            "setAnalysisSettings" => {
                let current = &doc.projection.analysis;
                let modal_count = args.and_then(|v| v.get("modalCount")).and_then(Value::as_u64).map(|n| n as usize).unwrap_or(current.modal_count);
                let buckling_count = args.and_then(|v| v.get("bucklingCount")).and_then(Value::as_u64).map(|n| n as usize).unwrap_or(current.buckling_count);
                let deformation_scale = args.and_then(|v| v.get("deformationScale")).and_then(Value::as_f64).unwrap_or(current.deformation_scale);
                return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetAnalysisSettings { settings: fem_2d::FemAnalysisSettings { modal_count, buckling_count, deformation_scale } }]);
            }
            "removeSelection" => {
                let ids: Vec<String> = args.and_then(|v| v.get("ids")).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                let mut operations = Vec::new();
                for id in ids {
                    if doc.projection.nodes.iter().any(|n| n.id == id) {
                        operations.push(fem_2d::Fem2dOperation::RemoveNode { id });
                    } else if doc.projection.elements.iter().any(|e| fem_2d::element_id(e) == id) {
                        operations.push(fem_2d::Fem2dOperation::RemoveElement { id });
                    } else if doc.projection.materials.iter().any(|m| m.id == id) {
                        operations.push(fem_2d::Fem2dOperation::RemoveMaterial { id });
                    } else if doc.projection.sections.iter().any(|s| s.id == id) {
                        operations.push(fem_2d::Fem2dOperation::RemoveSection { id });
                    } else if doc.projection.supports.iter().any(|s| s.id == id) {
                        operations.push(fem_2d::Fem2dOperation::RemoveSupport { id });
                    } else if doc.projection.load_cases.iter().any(|l| l.id == id) {
                        operations.push(fem_2d::Fem2dOperation::RemoveLoadCase { id });
                    } else if doc.projection.regions.iter().any(|r| r.id == id) {
                        operations.push(fem_2d::Fem2dOperation::RemoveRegion { id });
                    } else if doc.projection.combinations.iter().any(|c| c.id == id) {
                        operations.push(fem_2d::Fem2dOperation::RemoveCombination { id });
                    }
                }
                if !operations.is_empty() {
                    return ActionEmit::operations(operations);
                }
            }
            "setCamera" => {
                if let (Some(x), Some(y), Some(zoom)) = (
                    args.and_then(|v| v.get("x")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("y")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("zoom")).and_then(Value::as_f64),
                ) {
                    return ActionEmit::amend(vec![fem_2d::Fem2dOperation::SetCamera { camera: fem_2d::FemCamera { x, y, zoom } }], "camera");
                }
            }
            "setResultDisplay" => {
                self.result_display = parse_result_display(args);
            }
            "setActiveExample" => {
                let example_id = args.and_then(|v| v.get("exampleId")).and_then(Value::as_str).unwrap_or("");
                let document = if example_id == "default" {
                    fem_2d::Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap_or_else(|_| fem_2d::empty_fem2d_projection())
                } else {
                    fem_2d::empty_fem2d_projection()
                };
                self.result_display = ResultDisplay::default();
                return ActionEmit::operations(vec![fem_2d::Fem2dOperation::SetDocument { document }]);
            }
            _ => {}
        }
        ActionEmit::default()
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, fem_2d::Fem2dDocument>, _view_state: &ViewState) -> UiNode {
        match body_key {
            FEM2D_BODY_MODEL => render_fem2d_model(doc.projection),
            FEM2D_BODY_RESULTS => render_fem2d_results(doc.projection, &self.result_display),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = fem2d_labels(view_state);
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        AppLabelsOverlay {
            window_kind_labels: HashMap::from([
                (FEM2D_WINDOW_MODEL.to_string(), labels.window_model.to_string()),
                (FEM2D_WINDOW_RESULTS.to_string(), labels.window_results.to_string()),
            ]),
            panel_tab_labels: HashMap::new(),
            mode_labels: HashMap::from([("edit".to_string(), labels.mode_edit.to_string())]),
            action_labels: fem2d_action_labels(is_de),
            utility_labels: HashMap::new(),
            example_labels: HashMap::from([("default".to_string(), labels.example_default.to_string())]),
            action_arg_labels: HashMap::new(),
            dialog_labels: HashMap::new(),
            introduction_labels: HashMap::new(),
            group_labels: HashMap::new(),
        }
    }
}
//#endregion 🔖Fem2dPlayApp

//#region 🔖Fem2dTerminology
struct Fem2dLabels {
    window_model: &'static str,
    window_results: &'static str,
    mode_edit: &'static str,
    example_default: &'static str,
}

const FEM2D_LABELS_EN: Fem2dLabels = Fem2dLabels { window_model: "Model", window_results: "Results", mode_edit: "Edit", example_default: "Default" };
const FEM2D_LABELS_DE: Fem2dLabels = Fem2dLabels { window_model: "Modell", window_results: "Ergebnisse", mode_edit: "Bearbeiten", example_default: "Standard" };

fn fem2d_labels(view_state: &ViewState) -> &'static Fem2dLabels {
    if view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de")) {
        &FEM2D_LABELS_DE
    } else {
        &FEM2D_LABELS_EN
    }
}

fn fem2d_action_labels(is_de: bool) -> HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("addNode", "Add Node", "Knoten hinzufügen"),
        ("addBar", "Add Bar", "Stab hinzufügen"),
        ("addBeam", "Add Beam", "Balken hinzufügen"),
        ("addMaterial", "Add Material", "Material hinzufügen"),
        ("addSection", "Add Section", "Querschnitt hinzufügen"),
        ("addSupport", "Add Support", "Lager hinzufügen"),
        ("addNodalLoad", "Add Nodal Load", "Knotenlast hinzufügen"),
        ("addMemberUdl", "Add Member UDL", "Streckenlast hinzufügen"),
        ("addAreaLoad", "Add Area Load", "Flächenlast hinzufügen"),
        ("addRegion", "Add Region", "Bereich hinzufügen"),
        ("addLoadCase", "Add Load Case", "Lastfall hinzufügen"),
        ("addCombination", "Add Combination", "Kombination hinzufügen"),
        ("setSelfWeight", "Set Self Weight", "Eigengewicht festlegen"),
        ("setAnalysisSettings", "Set Analysis Settings", "Analyseeinstellungen festlegen"),
        ("removeSelection", "Remove Selection", "Auswahl entfernen"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("setResultDisplay", "Set Result Display", "Ergebnisanzeige festlegen"),
    ];
    ENTRIES.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
}
//#endregion 🔖Fem2dTerminology

//#region 🔖Fem3dRender
fn find_node_3d<'a>(nodes: &'a [fem_3d::FemNode], id: &str) -> Option<&'a fem_3d::FemNode> {
    nodes.iter().find(|n| n.id == id)
}

fn fem3d_element_endpoints(element: &fem_3d::FemElement) -> (&str, &str) {
    match element {
        fem_3d::FemElement::Bar { start, end, .. } | fem_3d::FemElement::Frame { start, end, .. } => (start.as_str(), end.as_str()),
    }
}

fn fem3d_element_id(element: &fem_3d::FemElement) -> &str {
    match element {
        fem_3d::FemElement::Bar { id, .. } | fem_3d::FemElement::Frame { id, .. } => id.as_str(),
    }
}

/// 🔎 3D counterpart of `fem2d_resolve_load_case` — see its doc.
fn fem3d_resolve_load_case(doc: &fem_3d::Fem3dDocument, case_id: Option<&str>) -> (usize, fem_3d::FemLoadCase) {
    let named = case_id.and_then(|id| doc.load_cases.iter().find(|lc| lc.id == id).cloned());
    let load_case = named
        .or_else(|| doc.load_cases.first().cloned())
        .unwrap_or_else(|| fem_3d::FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: Vec::new(), self_weight: false });
    let index = doc.load_cases.iter().position(|lc| lc.id == load_case.id).unwrap_or(doc.load_cases.len());
    (index, load_case)
}

/// 📐 Bounding-box diagonal (in model meters) over every node plus every solid's footprint/height —
/// see `fem2d_model_extent`'s doc for why this drives mode-shape amplitude. Falls back to `1.0` for a
/// degenerate model.
fn fem3d_model_extent(doc: &fem_3d::Fem3dDocument) -> f64 {
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    let mut expand = |x: f64, y: f64, z: f64| {
        min[0] = min[0].min(x);
        min[1] = min[1].min(y);
        min[2] = min[2].min(z);
        max[0] = max[0].max(x);
        max[1] = max[1].max(y);
        max[2] = max[2].max(z);
    };
    for node in &doc.nodes {
        expand(node.x, node.y, node.z);
    }
    for solid in &doc.solids {
        for p in &solid.outline {
            expand(p[0], p[1], solid.base_z);
            expand(p[0], p[1], solid.base_z + solid.height);
        }
    }
    if min[0] > max[0] {
        return 1.0;
    }
    let d = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1.0)
}

/// 🧭 Hamilton quaternion product `a * b`, both `[x,y,z,w]` — applying `b`'s rotation first, then `a`'s.
fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    let (bx, by, bz, bw) = (b[0], b[1], b[2], b[3]);
    [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]
}

/// 🧭 Rotation of `roll` radians about the LOCAL +Z axis — applied before `quat_z_to` reorients +Z to
/// the member direction, so this spins the box prism about its own long axis (matches `Frame3`'s roll).
fn quat_roll_z(roll: f64) -> [f64; 4] {
    let h = roll / 2.0;
    [0.0, 0.0, h.sin(), h.cos()]
}

/// 🧭 Shortest-arc rotation taking local `+Z` (the `"box"` mesh's long axis) onto unit direction `dir`
/// — the standard "rotate A onto B" quaternion (`axis = cross(from,to)`, `angle = acos(dot(from,to))`),
/// specialized for `from = (0,0,1)` so `cross` reduces to `(-dir.y, dir.x, 0)`. Handles the antiparallel
/// case (`dir ≈ (0,0,-1)`) with a fixed 180° flip about the X axis, since `cross` degenerates to zero there.
fn quat_z_to(dir: [f64; 3]) -> [f64; 4] {
    let dot = dir[2].clamp(-1.0, 1.0);
    if dot > 0.999_999 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    if dot < -0.999_999 {
        return [1.0, 0.0, 0.0, 0.0];
    }
    let axis = [-dir[1], dir[0], 0.0];
    let axis_len = (axis[0] * axis[0] + axis[1] * axis[1]).sqrt();
    let axis_n = [axis[0] / axis_len, axis[1] / axis_len, 0.0];
    let half = dot.acos() / 2.0;
    let s = half.sin();
    [axis_n[0] * s, axis_n[1] * s, axis_n[2] * s, half.cos()]
}

/// 🧊 Node-position resolver shared by every 3D instance/mesh builder: `displacements` (node id -> 6-DOF
/// values), when present, offsets a node's position by its solved displacement scaled by `deform_scale`.
fn fem3d_deformed_position(pos: [f64; 3], node_id: &str, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64) -> [f64; 3] {
    let mut p = pos;
    if let Some(map) = displacements {
        if let Some(d) = map.get(node_id) {
            p[0] += d[Dof::Tx.index()] * deform_scale;
            p[1] += d[Dof::Ty.index()] * deform_scale;
            p[2] += d[Dof::Tz.index()] * deform_scale;
        }
    }
    p
}

/// 🧊 One small box instance per node, plus one ORIENTED box prism per `Bar`/`Frame` member — position
/// at the (possibly deformed) midpoint, `scale=[t,t,length]` so the mesh's own long (local Z) axis
/// stretches along the member, `rotation` a quaternion aligning that axis to the member's direction
/// (composed with a `Frame`'s own `roll` about its own axis; `Bar`s have no roll).
fn fem3d_structural_instances(doc: &fem_3d::Fem3dDocument, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64) -> Vec<Value> {
    let node_pos = |node: &fem_3d::FemNode| fem3d_deformed_position([node.x, node.y, node.z], &node.id, displacements, deform_scale);

    let mut instances: Vec<Value> = Vec::new();
    for node in &doc.nodes {
        let p = node_pos(node);
        instances.push(json!({
            "id": format!("node-{}", node.id),
            "meshId": "box",
            "position": p,
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [NODE_SIZE_3D, NODE_SIZE_3D, NODE_SIZE_3D],
            "label": node.id,
        }));
    }
    for element in &doc.elements {
        let (start, end) = fem3d_element_endpoints(element);
        let (Some(n1), Some(n2)) = (find_node_3d(&doc.nodes, start), find_node_3d(&doc.nodes, end)) else { continue };
        let p1 = node_pos(n1);
        let p2 = node_pos(n2);
        let d = [p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2]];
        let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1e-9);
        let dir = [d[0] / length, d[1] / length, d[2] / length];
        let roll = match element {
            fem_3d::FemElement::Frame { roll, .. } => *roll,
            fem_3d::FemElement::Bar { .. } => 0.0,
        };
        let rotation = quat_mul(quat_z_to(dir), quat_roll_z(roll));
        let mid = [(p1[0] + p2[0]) / 2.0, (p1[1] + p2[1]) / 2.0, (p1[2] + p2[2]) / 2.0];
        let id = fem3d_element_id(element);
        instances.push(json!({
            "id": format!("el-{id}"),
            "meshId": "box",
            "position": mid,
            "rotation": rotation,
            "scale": [MEMBER_THICKNESS_3D, MEMBER_THICKNESS_3D, length],
            "label": id,
        }));
    }
    instances
}

/// 🧱 Every `FemSolid`'s boundary surface as a custom `meshes_json` entry (flat per-face normals, one
/// duplicated vertex triple per triangle) plus its one identity-transform instance — `nodal_stress`,
/// when present, colors each vertex by `von_mises_color` (min/max taken across ALL solids' averaged
/// values), driving the react renderer's vertex-color contour (see `PaintTexturedMesh`). `displacements`
/// deforms vertex positions the same way `fem3d_structural_instances` deforms node/member instances.
fn fem3d_solid_mesh_entries(doc: &fem_3d::Fem3dDocument, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64, nodal_stress: Option<&HashMap<String, f64>>) -> (Vec<Value>, Vec<Value>) {
    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    let Ok(solid_meshes) = fem_3d::fem3d_mesh_preview(doc) else { return (meshes, instances) };
    let (min, max) = match nodal_stress {
        Some(map) if !map.is_empty() => (map.values().cloned().fold(f64::INFINITY, f64::min), map.values().cloned().fold(f64::NEG_INFINITY, f64::max)),
        _ => (0.0, 1.0),
    };

    for solid in &solid_meshes {
        let mut positions: Vec<f64> = Vec::with_capacity(solid.boundary_tris.len() * 9);
        let mut normals: Vec<f64> = Vec::with_capacity(solid.boundary_tris.len() * 9);
        let mut colors: Vec<f64> = Vec::with_capacity(solid.boundary_tris.len() * 9);
        let mut indices: Vec<u32> = Vec::with_capacity(solid.boundary_tris.len() * 3);

        let vertex_pos = |idx: u32| -> [f64; 3] { fem3d_deformed_position(solid.points[idx as usize], &solid.node_ids[idx as usize], displacements, deform_scale) };
        let vertex_color = |idx: u32| -> (f64, f64, f64) {
            let Some(stress_map) = nodal_stress else { return (0.78, 0.78, 0.8) };
            let value = stress_map.get(&solid.node_ids[idx as usize]).copied().unwrap_or(min);
            hex_to_rgb01(von_mises_color(value, min, max))
        };

        for &[a, b, c] in &solid.boundary_tris {
            let (pa, pb, pc) = (vertex_pos(a), vertex_pos(b), vertex_pos(c));
            let e0 = [pb[0] - pa[0], pb[1] - pa[1], pb[2] - pa[2]];
            let e1 = [pc[0] - pa[0], pc[1] - pa[1], pc[2] - pa[2]];
            let raw = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
            let raw_len = (raw[0] * raw[0] + raw[1] * raw[1] + raw[2] * raw[2]).sqrt().max(1e-12);
            let n = [raw[0] / raw_len, raw[1] / raw_len, raw[2] / raw_len];
            let base = (positions.len() / 3) as u32;
            for (idx, p) in [(a, pa), (b, pb), (c, pc)] {
                positions.extend_from_slice(&p);
                normals.extend_from_slice(&n);
                let (r, g, bl) = vertex_color(idx);
                colors.extend_from_slice(&[r, g, bl]);
            }
            indices.extend_from_slice(&[base, base + 1, base + 2]);
        }

        let mesh_id = format!("solid-{}", solid.solid_id);
        meshes.push(json!({ "id": mesh_id, "data": { "positions": positions, "normals": normals, "colors": colors, "indices": indices } }));
        instances.push(json!({
            "id": format!("solid-inst-{}", solid.solid_id),
            "meshId": mesh_id,
            "position": [0.0, 0.0, 0.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0],
            "label": solid.solid_id,
        }));
    }
    (meshes, instances)
}

/// 🧊 Builds the FULL `(meshes_json, instances_json)` pair for a 3D scene: the `"box"` primitive mesh
/// plus every `FemSolid`'s custom surface mesh, and every node/member/solid instance — shared by the
/// model window and every results view (static/modal/buckling).
fn fem3d_scene_parts(doc: &fem_3d::Fem3dDocument, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64, nodal_stress: Option<&HashMap<String, f64>>) -> (String, String) {
    let mut meshes: Vec<Value> = serde_json::from_str(&world3d_meshes_json_from_kinds(&["box".to_string()])).unwrap_or_default();
    let mut instances = fem3d_structural_instances(doc, displacements, deform_scale);
    let (solid_meshes, solid_instances) = fem3d_solid_mesh_entries(doc, displacements, deform_scale, nodal_stress);
    meshes.extend(solid_meshes);
    instances.extend(solid_instances);
    (serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()))
}

fn fem3d_camera_json(doc: &fem_3d::Fem3dDocument) -> String {
    if doc.camera.json == "{}" {
        world3d_default_camera()
    } else {
        doc.camera.json.clone()
    }
}

/// 🏷️ Wraps a `World3d` scene node with a text caption above it — `World3dScene` itself has no text
/// field, so a vertical `UiNode` stack (already how the shell composes surfaces) is the idiomatic way
/// to show a frequency/load-factor/case caption in-scene, mirroring the 2D results window's caption layer.
fn with_caption(scene: UiNode, caption: String) -> UiNode {
    ui_stack_vertical(vec![ui_text(caption), scene])
}

fn render_fem3d_model(doc: &fem_3d::Fem3dDocument) -> UiNode {
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, None, doc.analysis.deformation_scale, None);
    build_world_3d_scene(
        FEM3D_BODY_MODEL,
        FEM3D_APP_ID,
        world3d_scene(fem3d_camera_json(doc), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()),
    )
}

/// 📊 Results window dispatcher — picks the static/modal/buckling render based on `display`.
fn render_fem3d_results(doc: &fem_3d::Fem3dDocument, display: &ResultDisplay) -> UiNode {
    match display.mode {
        DisplayMode::Static => render_fem3d_results_static(doc, display.source_id.as_deref()),
        DisplayMode::Modal(mode_index) => render_fem3d_results_modal(doc, mode_index),
        DisplayMode::Buckling(mode_index) => render_fem3d_results_buckling(doc, display.source_id.as_deref(), mode_index),
    }
}

/// 📊 Static results: solved fresh on every render (see `Fem3dPlayApp` doc comment) — same node/member/
/// solid instances as the model window, offset by the solved displacements, solids additionally colored
/// by nodal-averaged von Mises stress. `source_id` selects a `fem3d_solve_all` case/combination id,
/// falling back to the first load case when `None`/unknown. Caption names the active case.
fn render_fem3d_results_static(doc: &fem_3d::Fem3dDocument, source_id: Option<&str>) -> UiNode {
    let results = match fem_3d::fem3d_solve_all(doc) {
        Ok(results) => results,
        Err(e) => return ui_text(format!("Analysis error: {e}")),
    };
    let case_id = source_id
        .filter(|id| results.contains_key(*id))
        .map(str::to_string)
        .or_else(|| doc.load_cases.first().map(|c| c.id.clone()));
    let Some(case_id) = case_id else {
        return ui_text("No load case defined");
    };
    let Some(result) = results.get(&case_id) else {
        return ui_text(format!("Result not found: {case_id}"));
    };
    let mut disp_map: HashMap<String, [f64; 6]> = HashMap::new();
    for d in &result.displacements {
        disp_map.insert(d.node_id.clone(), d.values);
    }
    let nodal_stress = fem_3d::fem3d_nodal_von_mises(doc, &case_id).ok();
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), doc.analysis.deformation_scale, nodal_stress.as_ref());
    let scene = build_world_3d_scene(
        FEM3D_BODY_RESULTS,
        FEM3D_APP_ID,
        world3d_scene(fem3d_camera_json(doc), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()),
    );
    with_caption(scene, format!("Case: {case_id}"))
}

/// 📊 Modal mode-shape overlay: instances offset by the selected mode's shape, normalized to unit peak
/// then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent (see `normalize_mode_shape`),
/// with a frequency caption.
fn render_fem3d_results_modal(doc: &fem_3d::Fem3dDocument, mode_index: usize) -> UiNode {
    let (freq_hz, mut disp_map) = match fem_3d::fem3d_modal_mode_values(doc, mode_index) {
        Ok(values) => values,
        Err(e) => return ui_text(format!("Modal analysis error: {e}")),
    };
    normalize_mode_shape(&mut disp_map);
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), fem3d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO, None);
    let scene = build_world_3d_scene(
        FEM3D_BODY_RESULTS,
        FEM3D_APP_ID,
        world3d_scene(fem3d_camera_json(doc), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()),
    );
    with_caption(scene, format!("Mode {}: {freq_hz:.3} Hz", mode_index + 1))
}

/// 📊 Buckling mode-shape overlay: instances offset by the selected mode's shape, normalized to unit
/// peak then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent (see
/// `normalize_mode_shape`). `source_id` selects the reference load case, falling back to the first
/// load case when `None`. Caption names the mode and its load factor.
fn render_fem3d_results_buckling(doc: &fem_3d::Fem3dDocument, source_id: Option<&str>, mode_index: usize) -> UiNode {
    let Some(case_id) = source_id.map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone())) else {
        return ui_text("No load case defined");
    };
    let (factor, mut disp_map) = match fem_3d::fem3d_buckling_mode_values(doc, &case_id, mode_index) {
        Ok(values) => values,
        Err(e) => return ui_text(format!("Buckling analysis error: {e}")),
    };
    normalize_mode_shape(&mut disp_map);
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), fem3d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO, None);
    let scene = build_world_3d_scene(
        FEM3D_BODY_RESULTS,
        FEM3D_APP_ID,
        world3d_scene(fem3d_camera_json(doc), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()),
    );
    with_caption(scene, format!("Buckling mode {}: factor {factor:.3}", mode_index + 1))
}
//#endregion 🔖Fem3dRender

//#region 🔖Fem3dPlayApp
/// 🧮 v0 design: mirrors `Fem2dPlayApp` — results are recomputed fresh inside `render()`, no cache, no
/// `RunAnalysis` operation. `result_display` is ephemeral view state (see `ResultDisplay`'s doc), defaulting
/// to the first load case in `Static` mode.
#[derive(Default)]
struct Fem3dPlayApp {
    result_display: ResultDisplay,
}

impl DocumentApp for Fem3dPlayApp {
    type Projection = fem_3d::Fem3dDocument;
    type Operation = fem_3d::Fem3dOperation;

    fn app_id(&self) -> &str {
        FEM3D_APP_ID
    }

    fn document_schema(&self) -> &str {
        fem_3d::FEM_3D_SCHEMA
    }

    fn initial_projection(&self) -> fem_3d::Fem3dDocument {
        fem_3d::empty_fem3d_projection()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, fem_3d::Fem3dDocument>, _view_state: &ViewState) -> ActionEmit<fem_3d::Fem3dOperation> {
        match action {
            "addNode" => {
                if let (Some(x), Some(y), Some(z)) = (
                    args.and_then(|v| v.get("x")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("y")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("z")).and_then(Value::as_f64),
                ) {
                    let id = next_id(doc.projection.nodes.iter().map(|n| n.id.clone()), "n");
                    let index = doc.projection.nodes.len();
                    return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetNode { index, node: fem_3d::FemNode { id, x, y, z } }]);
                }
            }
            "addBar" => {
                if let (Some(start), Some(end), Some(material_id), Some(section_id)) = (
                    args.and_then(|v| v.get("start")).and_then(Value::as_str),
                    args.and_then(|v| v.get("end")).and_then(Value::as_str),
                    args.and_then(|v| v.get("materialId")).and_then(Value::as_str),
                    args.and_then(|v| v.get("sectionId")).and_then(Value::as_str),
                ) {
                    let id = next_id(doc.projection.elements.iter().map(|e| fem3d_element_id(e).to_string()), "e");
                    let index = doc.projection.elements.len();
                    let element = fem_3d::FemElement::Bar { id, start: start.into(), end: end.into(), material_id: material_id.into(), section_id: section_id.into() };
                    return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetElement { index, element }]);
                }
            }
            "addFrame" => {
                if let (Some(start), Some(end), Some(material_id), Some(section_id)) = (
                    args.and_then(|v| v.get("start")).and_then(Value::as_str),
                    args.and_then(|v| v.get("end")).and_then(Value::as_str),
                    args.and_then(|v| v.get("materialId")).and_then(Value::as_str),
                    args.and_then(|v| v.get("sectionId")).and_then(Value::as_str),
                ) {
                    let roll = args.and_then(|v| v.get("roll")).and_then(Value::as_f64).unwrap_or(0.0);
                    let id = next_id(doc.projection.elements.iter().map(|e| fem3d_element_id(e).to_string()), "e");
                    let index = doc.projection.elements.len();
                    let element = fem_3d::FemElement::Frame { id, start: start.into(), end: end.into(), material_id: material_id.into(), section_id: section_id.into(), roll };
                    return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetElement { index, element }]);
                }
            }
            "addMaterial" => {
                if let (Some(name), Some(e), Some(g)) = (
                    args.and_then(|v| v.get("name")).and_then(Value::as_str),
                    args.and_then(|v| v.get("e")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("g")).and_then(Value::as_f64),
                ) {
                    let id = next_id(doc.projection.materials.iter().map(|m| m.id.clone()), "m");
                    let index = doc.projection.materials.len();
                    return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetMaterial { index, material: fem_3d::FemMaterial { id, name: name.into(), e, g, nu: 0.3, rho: 7850.0 } }]);
                }
            }
            "addSection" => {
                if let (Some(name), Some(area), Some(iy), Some(iz), Some(j)) = (
                    args.and_then(|v| v.get("name")).and_then(Value::as_str),
                    args.and_then(|v| v.get("area")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("iy")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("iz")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("j")).and_then(Value::as_f64),
                ) {
                    let id = next_id(doc.projection.sections.iter().map(|s| s.id.clone()), "s");
                    let index = doc.projection.sections.len();
                    return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetSection { index, section: fem_3d::FemSection { id, name: name.into(), area, iy, iz, j } }]);
                }
            }
            "addSupport" => {
                if let Some(node_id) = args.and_then(|v| v.get("nodeId")).and_then(Value::as_str) {
                    let fixed: Vec<Dof> = args.and_then(|v| v.get("fixed")).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                    let id = next_id(doc.projection.supports.iter().map(|s| s.id.clone()), "sup");
                    let index = doc.projection.supports.len();
                    return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetSupport { index, support: fem_3d::FemSupport { id, node_id: node_id.into(), fixed } }]);
                }
            }
            "addNodalLoad" => {
                if let (Some(node_id), Some(dof), Some(value)) = (
                    args.and_then(|v| v.get("nodeId")).and_then(Value::as_str),
                    args.and_then(|v| v.get("dof")).and_then(|v| serde_json::from_value::<Dof>(v.clone()).ok()),
                    args.and_then(|v| v.get("value")).and_then(Value::as_f64),
                ) {
                    let case_id = args.and_then(|v| v.get("caseId")).and_then(Value::as_str);
                    let (index, mut load_case) = fem3d_resolve_load_case(doc.projection, case_id);
                    let load_id = next_id(load_case.loads.iter().map(|l| fem_3d::load_id(l).to_string()), "l");
                    load_case.loads.push(fem_3d::FemLoad::Nodal { id: load_id, node_id: node_id.into(), dof, value });
                    return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetLoadCase { index, load_case }]);
                }
            }
            "addMemberUdl" => {
                if let (Some(element_id), Some(wx), Some(wy), Some(wz)) = (
                    args.and_then(|v| v.get("elementId")).and_then(Value::as_str),
                    args.and_then(|v| v.get("wx")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("wy")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("wz")).and_then(Value::as_f64),
                ) {
                    let case_id = args.and_then(|v| v.get("caseId")).and_then(Value::as_str);
                    let (index, mut load_case) = fem3d_resolve_load_case(doc.projection, case_id);
                    let load_id = next_id(load_case.loads.iter().map(|l| fem_3d::load_id(l).to_string()), "l");
                    load_case.loads.push(fem_3d::FemLoad::MemberUdl { id: load_id, element_id: element_id.into(), wx, wy, wz });
                    return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetLoadCase { index, load_case }]);
                }
            }
            "addAreaLoad" => {
                if let (Some(solid_id), Some(pressure)) = (args.and_then(|v| v.get("solidId")).and_then(Value::as_str), args.and_then(|v| v.get("pressure")).and_then(Value::as_f64)) {
                    let case_id = args.and_then(|v| v.get("caseId")).and_then(Value::as_str);
                    let (index, mut load_case) = fem3d_resolve_load_case(doc.projection, case_id);
                    let load_id = next_id(load_case.loads.iter().map(|l| fem_3d::load_id(l).to_string()), "l");
                    load_case.loads.push(fem_3d::FemLoad::Area { id: load_id, solid_id: solid_id.into(), pressure });
                    return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetLoadCase { index, load_case }]);
                }
            }
            "addSolid" => {
                if let (Some(x), Some(y), Some(width), Some(depth), Some(height), Some(material_id)) = (
                    args.and_then(|v| v.get("x")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("y")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("width")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("depth")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("height")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("materialId")).and_then(Value::as_str),
                ) {
                    let base_z = args.and_then(|v| v.get("baseZ")).and_then(Value::as_f64).unwrap_or(0.0);
                    let layers = args.and_then(|v| v.get("layers")).and_then(Value::as_u64).unwrap_or(1) as usize;
                    let mesh_size = args.and_then(|v| v.get("meshSize")).and_then(Value::as_f64).unwrap_or(0.5);
                    let id = next_id(doc.projection.solids.iter().map(|s| s.id.clone()), "sol");
                    let index = doc.projection.solids.len();
                    let outline = vec![[x, y], [x + width, y], [x + width, y + depth], [x, y + depth]];
                    return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetSolid { index, solid: fem_3d::FemSolid { id, name: "Solid".into(), outline, holes: Vec::new(), base_z, height, layers, mesh_size, material_id: material_id.into() } }]);
                }
            }
            "addLoadCase" => {
                if let Some(name) = args.and_then(|v| v.get("name")).and_then(Value::as_str) {
                    let self_weight = args.and_then(|v| v.get("selfWeight")).and_then(Value::as_bool).unwrap_or(false);
                    let id = next_id(doc.projection.load_cases.iter().map(|lc| lc.id.clone()), "case-");
                    let index = doc.projection.load_cases.len();
                    return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetLoadCase { index, load_case: fem_3d::FemLoadCase { id, name: name.into(), loads: Vec::new(), self_weight } }]);
                }
            }
            "addCombination" => {
                if let (Some(name), Some(terms_json)) = (args.and_then(|v| v.get("name")).and_then(Value::as_str), args.and_then(|v| v.get("terms")).and_then(Value::as_str)) {
                    if let Ok(terms) = serde_json::from_str::<Vec<(String, f64)>>(terms_json) {
                        let id = next_id(doc.projection.combinations.iter().map(|c| c.id.clone()), "c");
                        let index = doc.projection.combinations.len();
                        return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetCombination { index, combination: fem_3d::FemCombination { id, name: name.into(), terms } }]);
                    }
                }
            }
            "setSelfWeight" => {
                if let (Some(case_id), Some(enabled)) = (args.and_then(|v| v.get("caseId")).and_then(Value::as_str), args.and_then(|v| v.get("enabled")).and_then(Value::as_bool)) {
                    if let Some(index) = doc.projection.load_cases.iter().position(|lc| lc.id == case_id) {
                        let mut load_case = doc.projection.load_cases[index].clone();
                        load_case.self_weight = enabled;
                        return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetLoadCase { index, load_case }]);
                    }
                }
            }
            "setAnalysisSettings" => {
                let current = &doc.projection.analysis;
                let modal_count = args.and_then(|v| v.get("modalCount")).and_then(Value::as_u64).map(|n| n as usize).unwrap_or(current.modal_count);
                let buckling_count = args.and_then(|v| v.get("bucklingCount")).and_then(Value::as_u64).map(|n| n as usize).unwrap_or(current.buckling_count);
                let deformation_scale = args.and_then(|v| v.get("deformationScale")).and_then(Value::as_f64).unwrap_or(current.deformation_scale);
                return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetAnalysisSettings { settings: fem_3d::FemAnalysisSettings { modal_count, buckling_count, deformation_scale } }]);
            }
            "removeSelection" => {
                let ids: Vec<String> = args.and_then(|v| v.get("ids")).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                let mut operations = Vec::new();
                for id in ids {
                    if doc.projection.nodes.iter().any(|n| n.id == id) {
                        operations.push(fem_3d::Fem3dOperation::RemoveNode { id });
                    } else if doc.projection.elements.iter().any(|e| fem3d_element_id(e) == id) {
                        operations.push(fem_3d::Fem3dOperation::RemoveElement { id });
                    } else if doc.projection.materials.iter().any(|m| m.id == id) {
                        operations.push(fem_3d::Fem3dOperation::RemoveMaterial { id });
                    } else if doc.projection.sections.iter().any(|s| s.id == id) {
                        operations.push(fem_3d::Fem3dOperation::RemoveSection { id });
                    } else if doc.projection.supports.iter().any(|s| s.id == id) {
                        operations.push(fem_3d::Fem3dOperation::RemoveSupport { id });
                    } else if doc.projection.load_cases.iter().any(|l| l.id == id) {
                        operations.push(fem_3d::Fem3dOperation::RemoveLoadCase { id });
                    } else if doc.projection.solids.iter().any(|s| s.id == id) {
                        operations.push(fem_3d::Fem3dOperation::RemoveSolid { id });
                    } else if doc.projection.combinations.iter().any(|c| c.id == id) {
                        operations.push(fem_3d::Fem3dOperation::RemoveCombination { id });
                    }
                }
                if !operations.is_empty() {
                    return ActionEmit::operations(operations);
                }
            }
            "setCamera" => {
                if let Some(json_str) = args.and_then(|v| v.get("json")).and_then(Value::as_str) {
                    return ActionEmit::amend(vec![fem_3d::Fem3dOperation::SetCamera { camera: fem_3d::FemCamera { json: json_str.into() } }], "camera");
                }
            }
            "setResultDisplay" => {
                self.result_display = parse_result_display(args);
            }
            "setActiveExample" => {
                let example_id = args.and_then(|v| v.get("exampleId")).and_then(Value::as_str).unwrap_or("");
                let document = if example_id == "default" {
                    fem_3d::Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap_or_else(|_| fem_3d::empty_fem3d_projection())
                } else {
                    fem_3d::empty_fem3d_projection()
                };
                self.result_display = ResultDisplay::default();
                return ActionEmit::operations(vec![fem_3d::Fem3dOperation::SetDocument { document }]);
            }
            _ => {}
        }
        ActionEmit::default()
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, fem_3d::Fem3dDocument>, _view_state: &ViewState) -> UiNode {
        match body_key {
            FEM3D_BODY_MODEL => render_fem3d_model(doc.projection),
            FEM3D_BODY_RESULTS => render_fem3d_results(doc.projection, &self.result_display),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = fem3d_labels(view_state);
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        AppLabelsOverlay {
            window_kind_labels: HashMap::from([
                (FEM3D_WINDOW_MODEL.to_string(), labels.window_model.to_string()),
                (FEM3D_WINDOW_RESULTS.to_string(), labels.window_results.to_string()),
            ]),
            panel_tab_labels: HashMap::new(),
            mode_labels: HashMap::from([("edit".to_string(), labels.mode_edit.to_string())]),
            action_labels: fem3d_action_labels(is_de),
            utility_labels: HashMap::new(),
            example_labels: HashMap::from([("default".to_string(), labels.example_default.to_string())]),
            action_arg_labels: HashMap::new(),
            dialog_labels: HashMap::new(),
            introduction_labels: HashMap::new(),
            group_labels: HashMap::new(),
        }
    }
}
//#endregion 🔖Fem3dPlayApp

//#region 🔖Fem3dTerminology
struct Fem3dLabels {
    window_model: &'static str,
    window_results: &'static str,
    mode_edit: &'static str,
    example_default: &'static str,
}

const FEM3D_LABELS_EN: Fem3dLabels = Fem3dLabels { window_model: "Model", window_results: "Results", mode_edit: "Edit", example_default: "Default" };
const FEM3D_LABELS_DE: Fem3dLabels = Fem3dLabels { window_model: "Modell", window_results: "Ergebnisse", mode_edit: "Bearbeiten", example_default: "Standard" };

fn fem3d_labels(view_state: &ViewState) -> &'static Fem3dLabels {
    if view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de")) {
        &FEM3D_LABELS_DE
    } else {
        &FEM3D_LABELS_EN
    }
}

fn fem3d_action_labels(is_de: bool) -> HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("addNode", "Add Node", "Knoten hinzufügen"),
        ("addBar", "Add Bar", "Stab hinzufügen"),
        ("addFrame", "Add Frame", "Rahmen hinzufügen"),
        ("addMaterial", "Add Material", "Material hinzufügen"),
        ("addSection", "Add Section", "Querschnitt hinzufügen"),
        ("addSupport", "Add Support", "Lager hinzufügen"),
        ("addNodalLoad", "Add Nodal Load", "Knotenlast hinzufügen"),
        ("addMemberUdl", "Add Member UDL", "Streckenlast hinzufügen"),
        ("addAreaLoad", "Add Area Load", "Flächenlast hinzufügen"),
        ("addSolid", "Add Solid", "Volumenkörper hinzufügen"),
        ("addLoadCase", "Add Load Case", "Lastfall hinzufügen"),
        ("addCombination", "Add Combination", "Kombination hinzufügen"),
        ("setSelfWeight", "Set Self Weight", "Eigengewicht festlegen"),
        ("setAnalysisSettings", "Set Analysis Settings", "Analyseeinstellungen festlegen"),
        ("removeSelection", "Remove Selection", "Auswahl entfernen"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("setResultDisplay", "Set Result Display", "Ergebnisanzeige festlegen"),
    ];
    ENTRIES.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
}
//#endregion 🔖Fem3dTerminology

//#region 🔖Manifest
fn create_fem2d_app() -> App {
    App::from_builder(
        App::builder(FEM2D_APP_ID, "FEM 2D")
            .document(["semio", "fem", "fem2d"])
            .icon_id("fem-app")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(FEM2D_WINDOW_MODEL, "Model", FEM2D_BODY_MODEL, SurfaceKind::Canvas2d, "fem-model")
            .window_kind(FEM2D_WINDOW_RESULTS, "Results", FEM2D_BODY_RESULTS, SurfaceKind::Canvas2d, "bar-chart-3")
            .default_layout(create_default_layout(
                &[FEM2D_WINDOW_MODEL.into(), FEM2D_WINDOW_RESULTS.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Model".into(), "Results".into()]),
            ))
            .operation("addNode", "Add Node")
            .action_args("addNode", vec![ActionArgDef::number("x", "X").required(), ActionArgDef::number("y", "Y").required()])
            .operation("addBar", "Add Bar")
            .operation("addBeam", "Add Beam")
            .operation("addMaterial", "Add Material")
            .operation("addSection", "Add Section")
            .operation("addSupport", "Add Support")
            .operation("addNodalLoad", "Add Nodal Load")
            .action_args("addNodalLoad", vec![ActionArgDef::text("caseId", "Case")])
            .operation("addMemberUdl", "Add Member UDL")
            .action_args("addMemberUdl", vec![ActionArgDef::text("caseId", "Case")])
            .operation("addAreaLoad", "Add Area Load")
            .action_args("addAreaLoad", vec![
                ActionArgDef::text("regionId", "Region").required(),
                ActionArgDef::number("pressure", "Pressure").required(),
                ActionArgDef::text("caseId", "Case"),
            ])
            .operation("addRegion", "Add Region")
            .action_args("addRegion", vec![
                ActionArgDef::number("x", "X").required(),
                ActionArgDef::number("y", "Y").required(),
                ActionArgDef::number("width", "Width").required(),
                ActionArgDef::number("height", "Height").required(),
                ActionArgDef::text("materialId", "Material").required(),
                ActionArgDef::number("thickness", "Thickness").default_value(0.02),
                ActionArgDef::number("meshSize", "Mesh Size").default_value(0.25),
            ])
            .operation("addLoadCase", "Add Load Case")
            .action_args("addLoadCase", vec![ActionArgDef::text("name", "Name").required(), ActionArgDef::toggle("selfWeight", "Self Weight").default_value(false)])
            .operation("addCombination", "Add Combination")
            .action_args("addCombination", vec![ActionArgDef::text("name", "Name").required(), ActionArgDef::text("terms", "Terms").required()])
            .operation("setSelfWeight", "Set Self Weight")
            .action_args("setSelfWeight", vec![ActionArgDef::text("caseId", "Case").required(), ActionArgDef::toggle("enabled", "Enabled").required()])
            .operation("setAnalysisSettings", "Set Analysis Settings")
            .action_args("setAnalysisSettings", vec![
                ActionArgDef::number("modalCount", "Modal Count"),
                ActionArgDef::number("bucklingCount", "Buckling Count"),
                ActionArgDef::number("deformationScale", "Deformation Scale"),
            ])
            .operation("removeSelection", "Remove Selection")
            .operation("setCamera", "Set Camera")
            .operation("setActiveExample", "Set Active Example")
            .action_args("setActiveExample", vec![ActionArgDef::select("exampleId", "Example", vec![ActionArgOption::new("default", "Default")]).default_value("default")])
            .view_action("setResultDisplay", "Set Result Display")
            .action_args("setResultDisplay", result_display_action_args()),
    )
    .example("default", "Family House", FEM2D_EXAMPLE_DSL)
    .program("fem2d", "FEM 2D", "structure")
}

fn create_fem3d_app() -> App {
    App::from_builder(
        App::builder(FEM3D_APP_ID, "FEM 3D")
            .document(["semio", "fem", "fem3d"])
            .icon_id("fem-app")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(FEM3D_WINDOW_MODEL, "Model", FEM3D_BODY_MODEL, SurfaceKind::World3d, "fem-model")
            .window_kind(FEM3D_WINDOW_RESULTS, "Results", FEM3D_BODY_RESULTS, SurfaceKind::World3d, "bar-chart-3")
            .default_layout(create_default_layout(
                &[FEM3D_WINDOW_MODEL.into(), FEM3D_WINDOW_RESULTS.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Model".into(), "Results".into()]),
            ))
            .operation("addNode", "Add Node")
            .action_args("addNode", vec![
                ActionArgDef::number("x", "X").required(),
                ActionArgDef::number("y", "Y").required(),
                ActionArgDef::number("z", "Z").required(),
            ])
            .operation("addBar", "Add Bar")
            .operation("addFrame", "Add Frame")
            .operation("addMaterial", "Add Material")
            .operation("addSection", "Add Section")
            .operation("addSupport", "Add Support")
            .operation("addNodalLoad", "Add Nodal Load")
            .action_args("addNodalLoad", vec![ActionArgDef::text("caseId", "Case")])
            .operation("addMemberUdl", "Add Member UDL")
            .action_args("addMemberUdl", vec![ActionArgDef::text("caseId", "Case")])
            .operation("addAreaLoad", "Add Area Load")
            .action_args("addAreaLoad", vec![
                ActionArgDef::text("solidId", "Solid").required(),
                ActionArgDef::number("pressure", "Pressure").required(),
                ActionArgDef::text("caseId", "Case"),
            ])
            .operation("addSolid", "Add Solid")
            .action_args("addSolid", vec![
                ActionArgDef::number("x", "X").required(),
                ActionArgDef::number("y", "Y").required(),
                ActionArgDef::number("width", "Width").required(),
                ActionArgDef::number("depth", "Depth").required(),
                ActionArgDef::number("height", "Height").required(),
                ActionArgDef::text("materialId", "Material").required(),
                ActionArgDef::number("baseZ", "Base Z").default_value(0.0),
                ActionArgDef::number("layers", "Layers").default_value(1),
                ActionArgDef::number("meshSize", "Mesh Size").default_value(0.5),
            ])
            .operation("addLoadCase", "Add Load Case")
            .action_args("addLoadCase", vec![ActionArgDef::text("name", "Name").required(), ActionArgDef::toggle("selfWeight", "Self Weight").default_value(false)])
            .operation("addCombination", "Add Combination")
            .action_args("addCombination", vec![ActionArgDef::text("name", "Name").required(), ActionArgDef::text("terms", "Terms").required()])
            .operation("setSelfWeight", "Set Self Weight")
            .action_args("setSelfWeight", vec![ActionArgDef::text("caseId", "Case").required(), ActionArgDef::toggle("enabled", "Enabled").required()])
            .operation("setAnalysisSettings", "Set Analysis Settings")
            .action_args("setAnalysisSettings", vec![
                ActionArgDef::number("modalCount", "Modal Count"),
                ActionArgDef::number("bucklingCount", "Buckling Count"),
                ActionArgDef::number("deformationScale", "Deformation Scale"),
            ])
            .operation("removeSelection", "Remove Selection")
            .operation("setCamera", "Set Camera")
            .operation("setActiveExample", "Set Active Example")
            .action_args("setActiveExample", vec![ActionArgDef::select("exampleId", "Example", vec![ActionArgOption::new("default", "Default")]).default_value("default")])
            .view_action("setResultDisplay", "Set Result Display")
            .action_args("setResultDisplay", result_display_action_args()),
    )
    .example("default", "Family House", FEM3D_EXAMPLE_DSL)
    .program("fem3d", "FEM 3D", "structure")
}

/// 📝 Shared `setResultDisplay` arg declarations for both apps' builders — `sourceId` (a case/
/// combination id), `mode` (static/modal/buckling), and `modeIndex` (0-based, only meaningful for
/// modal/buckling).
fn result_display_action_args() -> Vec<ActionArgDef> {
    vec![
        ActionArgDef::text("sourceId", "Source"),
        ActionArgDef::select(
            "mode",
            "Mode",
            vec![
                ActionArgOption::new("static", "Static"),
                ActionArgOption::new("modal", "Modal"),
                ActionArgOption::new("buckling", "Buckling"),
            ],
        ),
        ActionArgDef::number("modeIndex", "Mode Index"),
    ]
}

fn register_fem_exports() {}

semio_framework_plugin::semio_plugin! {
    id: "fem", label: "FEM", version: "0.1.0",
    setup: register_fem_exports,
    apps: [ create_fem2d_app => Fem2dPlayApp, create_fem3d_app => Fem3dPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    fn history_view() -> HistoryView {
        HistoryView { columns: Vec::new(), can_undo: false, can_redo: false, active_alternative_id: None, current_checkpoint_id: None }
    }

    //#region 🔖RendersScenes
    #[test]
    fn renders_fem2d_model_scene() {
        let app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM2D_BODY_MODEL, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn renders_fem2d_results_scene() {
        let app = Fem2dPlayApp::default();
                let projection: fem_2d::Fem2dDocument = fem_2d::Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM2D_BODY_RESULTS, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn renders_fem3d_model_scene() {
        let app = Fem3dPlayApp::default();
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM3D_BODY_MODEL, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn renders_fem3d_results_scene() {
        let app = Fem3dPlayApp::default();
                let projection: fem_3d::Fem3dDocument = fem_3d::Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }
    //#endregion 🔖RendersScenes

    //#region 🔖AddNodeAction
    #[test]
    fn add_node_action_emits_op_2d() {
        let mut app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "x": 1.0, "y": 2.0 });
        let emit = app.handle_action("addNode", Some(&args), &doc, &ViewState::default());
        assert_eq!(emit.operations.len(), 1);
        match &emit.operations[0] {
            fem_2d::Fem2dOperation::SetNode { node, .. } => {
                assert_eq!(node.x, 1.0);
                assert_eq!(node.y, 2.0);
            }
            _ => panic!("expected SetNode"),
        }
    }

    #[test]
    fn add_node_action_emits_op_3d() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "x": 1.0, "y": 2.0, "z": 3.0 });
        let emit = app.handle_action("addNode", Some(&args), &doc, &ViewState::default());
        assert_eq!(emit.operations.len(), 1);
        match &emit.operations[0] {
            fem_3d::Fem3dOperation::SetNode { node, .. } => {
                assert_eq!(node.x, 1.0);
                assert_eq!(node.y, 2.0);
                assert_eq!(node.z, 3.0);
            }
            _ => panic!("expected SetNode"),
        }
    }
    //#endregion 🔖AddNodeAction

    //#region 🔖SolverErrorSurfaced
    #[test]
    fn results_window_surfaces_solver_error_without_panicking_2d() {
        let app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let _ = app.render(FEM2D_BODY_RESULTS, &doc, &ViewState::default());
    }

    #[test]
    fn results_window_surfaces_solver_error_without_panicking_3d() {
        let app = Fem3dPlayApp::default();
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let _ = app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
    }
    //#endregion 🔖SolverErrorSurfaced

    //#region 🔖ExampleFixtureRenders
    #[test]
    fn example_fixture_renders_2d() {
        let app = Fem2dPlayApp::default();
                let projection: fem_2d::Fem2dDocument = fem_2d::Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let _ = app.render(FEM2D_BODY_MODEL, &doc, &ViewState::default());
        let _ = app.render(FEM2D_BODY_RESULTS, &doc, &ViewState::default());
    }

    #[test]
    fn example_fixture_renders_3d() {
        let app = Fem3dPlayApp::default();
                let projection: fem_3d::Fem3dDocument = fem_3d::Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let _ = app.render(FEM3D_BODY_MODEL, &doc, &ViewState::default());
        let _ = app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
    }
    //#endregion 🔖ExampleFixtureRenders

    //#region 🔖MeshPreviewRender
    #[test]
    fn mesh_preview_renders_region_edges() {
        let app = Fem2dPlayApp::default();
                let projection: fem_2d::Fem2dDocument = fem_2d::Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM2D_BODY_MODEL, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("mesh-edge-"), "expected mesh-edge preview layers in the model scene");
    }
    //#endregion 🔖MeshPreviewRender

    //#region 🔖ResultDisplayAction
    #[test]
    fn set_result_display_is_a_view_action() {
        let mut app = Fem2dPlayApp::default();
                let projection: fem_2d::Fem2dDocument = fem_2d::Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "sourceId": "dead", "mode": "modal", "modeIndex": 0 });
        let emit = app.handle_action("setResultDisplay", Some(&args), &doc, &ViewState::default());
        assert!(emit.operations.is_empty(), "setResultDisplay must not emit operations (it's ephemeral view state)");
        assert_eq!(app.result_display.mode, DisplayMode::Modal(0));
        assert_eq!(app.result_display.source_id.as_deref(), Some("dead"));
    }
    //#endregion 🔖ResultDisplayAction

    //#region 🔖SetActiveExample
    #[test]
    fn set_active_example_loads_default_fixture_2d() {
        let mut app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("setActiveExample", Some(&json!({ "exampleId": "default" })), &doc, &ViewState::default());
        assert_eq!(emit.operations.len(), 1);
        match &emit.operations[0] {
            fem_2d::Fem2dOperation::SetDocument { document } => assert!(!document.nodes.is_empty(), "expected the default fixture's nodes"),
            _ => panic!("expected SetDocument"),
        }
    }

    #[test]
    fn set_active_example_unknown_id_yields_empty_document_2d() {
        let mut app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("setActiveExample", Some(&json!({ "exampleId": "" })), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem_2d::Fem2dOperation::SetDocument { document } => assert_eq!(document, &fem_2d::empty_fem2d_projection()),
            _ => panic!("expected SetDocument"),
        }
    }

    #[test]
    fn set_active_example_loads_default_fixture_3d() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("setActiveExample", Some(&json!({ "exampleId": "default" })), &doc, &ViewState::default());
        assert_eq!(emit.operations.len(), 1);
        match &emit.operations[0] {
            fem_3d::Fem3dOperation::SetDocument { document } => assert!(!document.nodes.is_empty(), "expected the default fixture's nodes"),
            _ => panic!("expected SetDocument"),
        }
    }

    /// 🧬 `setActiveExample` replaces document content via `SetDocument` operations, so it MUST be declared as
    /// an Operation, not a View/Shell action — the framework's "View/Shell actions must not emit
    /// operations" guard would otherwise reject it (mirrors `gis/plugin/rs`'s equivalent test).
    #[test]
    fn set_active_example_is_declared_as_operation_2d() {
        use semio_framework_plugin::ActionKind;
        let definition = create_fem2d_app().definition;
        let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, ActionKind::Operation), "loading an example emits SetDocument operations, so it is an Operation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
    }

    #[test]
    fn set_active_example_is_declared_as_operation_3d() {
        use semio_framework_plugin::ActionKind;
        let definition = create_fem3d_app().definition;
        let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, ActionKind::Operation), "loading an example emits SetDocument operations, so it is an Operation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
    }
    //#endregion 🔖SetActiveExample

    //#region 🔖ContourRender
    #[test]
    fn results_window_renders_contour_for_region() {
        let app = Fem2dPlayApp { result_display: ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Static } };
                let projection: fem_2d::Fem2dDocument = fem_2d::Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM2D_BODY_RESULTS, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        // `layers_json` is itself a JSON string embedded inside `UiNode`'s own serialization, so its
        // quotes come out backslash-escaped in `json` — match on the unescaped substrings instead.
        assert!(json.contains("fill"), "expected filled-path contour layers for the region's Tri3Cst elements: {json}");
        assert!(json.contains("contour-"), "expected contour-prefixed layer ids: {json}");
    }
    //#endregion 🔖ContourRender

    //#region 🔖ReactionLabels
    #[test]
    fn results_window_renders_reaction_labels_2d() {
        let app = Fem2dPlayApp { result_display: ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Static } };
                let projection: fem_2d::Fem2dDocument = fem_2d::Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM2D_BODY_RESULTS, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("reaction-"), "expected reaction-prefixed text label layers: {json}");
    }
    //#endregion 🔖ReactionLabels

    //#region 🔖ModeShapeRender
    #[test]
    fn results_window_renders_modal_mode_shape() {
        let app_2d = Fem2dPlayApp { result_display: ResultDisplay { source_id: None, mode: DisplayMode::Modal(0) } };
                let projection_2d: fem_2d::Fem2dDocument = fem_2d::Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc_2d = DocumentView { projection: &projection_2d, history: &history };
        let node_2d = app_2d.render(FEM2D_BODY_RESULTS, &doc_2d, &ViewState::default());
        let json_2d = serde_json::to_string(&node_2d).unwrap();
        assert!(json_2d.contains("canvas-2d"), "expected a valid canvas-2d scene, got: {json_2d}");
        assert!(!json_2d.contains("Modal analysis error"), "unexpected modal error: {json_2d}");

        let app_3d = Fem3dPlayApp { result_display: ResultDisplay { source_id: None, mode: DisplayMode::Modal(0) } };
                let projection_3d: fem_3d::Fem3dDocument = fem_3d::Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let doc_3d = DocumentView { projection: &projection_3d, history: &history };
        let node_3d = app_3d.render(FEM3D_BODY_RESULTS, &doc_3d, &ViewState::default());
        let json_3d = serde_json::to_string(&node_3d).unwrap();
        assert!(json_3d.contains("world-3d"), "expected a valid world-3d scene, got: {json_3d}");
        assert!(!json_3d.contains("Modal analysis error"), "unexpected modal error: {json_3d}");
    }

    #[test]
    fn results_window_renders_buckling_mode_shape() {
        let app_2d = Fem2dPlayApp { result_display: ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Buckling(0) } };
                let projection_2d: fem_2d::Fem2dDocument = fem_2d::Fem2dDocument::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc_2d = DocumentView { projection: &projection_2d, history: &history };
        let node_2d = app_2d.render(FEM2D_BODY_RESULTS, &doc_2d, &ViewState::default());
        let json_2d = serde_json::to_string(&node_2d).unwrap();
        assert!(json_2d.contains("canvas-2d"), "expected a valid canvas-2d scene, got: {json_2d}");
        assert!(!json_2d.contains("Buckling analysis error"), "unexpected buckling error: {json_2d}");

        let app_3d = Fem3dPlayApp { result_display: ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Buckling(0) } };
                let projection_3d: fem_3d::Fem3dDocument = fem_3d::Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let doc_3d = DocumentView { projection: &projection_3d, history: &history };
        let node_3d = app_3d.render(FEM3D_BODY_RESULTS, &doc_3d, &ViewState::default());
        let json_3d = serde_json::to_string(&node_3d).unwrap();
        assert!(json_3d.contains("world-3d"), "expected a valid world-3d scene, got: {json_3d}");
        assert!(!json_3d.contains("Buckling analysis error"), "unexpected buckling error: {json_3d}");
    }
    //#endregion 🔖ModeShapeRender

    //#region 🔖SolidRenderAndCaptions
    #[test]
    fn model_scene_renders_solid_mesh_and_oriented_member_instances_3d() {
        let app = Fem3dPlayApp::default();
                let projection: fem_3d::Fem3dDocument = fem_3d::Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM3D_BODY_MODEL, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("solid-sol1"), "expected a solid- mesh/instance id for the example fixture's solid: {json}");
        assert!(json.contains("el-e1"), "expected a single oriented box instance per member (no -{{i}} sphere chain): {json}");
        assert!(!json.contains("\\\"sphere\\\""), "sphere markers should be gone: {json}");
    }

    #[test]
    fn results_scene_includes_solid_vertex_colors_3d() {
        let app = Fem3dPlayApp { result_display: ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Static } };
                let projection: fem_3d::Fem3dDocument = fem_3d::Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("solid-sol1"), "expected the solid mesh in the results scene: {json}");
        assert!(json.contains("\\\"colors\\\""), "expected a vertex colors array on the solid mesh data: {json}");
        assert!(json.contains("Case: dead"), "expected a case-id caption: {json}");
    }

    #[test]
    fn results_scene_captions_name_mode_and_factor_3d() {
        let app_modal = Fem3dPlayApp { result_display: ResultDisplay { source_id: None, mode: DisplayMode::Modal(0) } };
                let projection: fem_3d::Fem3dDocument = fem_3d::Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node_modal = app_modal.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
        let json_modal = serde_json::to_string(&node_modal).unwrap();
        assert!(json_modal.contains("Hz"), "expected a frequency caption: {json_modal}");

        let app_buckling = Fem3dPlayApp { result_display: ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Buckling(0) } };
        let node_buckling = app_buckling.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
        let json_buckling = serde_json::to_string(&node_buckling).unwrap();
        assert!(json_buckling.contains("factor"), "expected a load-factor caption: {json_buckling}");
    }
    //#endregion 🔖SolidRenderAndCaptions

    //#region 🔖StructureActions
    #[test]
    fn add_region_action_emits_set_region_2d() {
        let mut app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "x": 0.0, "y": 0.0, "width": 4.0, "height": 2.0, "materialId": "steel" });
        let emit = app.handle_action("addRegion", Some(&args), &doc, &ViewState::default());
        assert_eq!(emit.operations.len(), 1);
        match &emit.operations[0] {
            fem_2d::Fem2dOperation::SetRegion { region, .. } => {
                assert_eq!(region.outline, vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]]);
                assert_eq!(region.thickness, 0.02);
            }
            _ => panic!("expected SetRegion"),
        }
    }

    #[test]
    fn add_solid_action_emits_set_solid_3d() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "x": 0.0, "y": 0.0, "width": 2.0, "depth": 1.0, "height": 0.5, "materialId": "concrete" });
        let emit = app.handle_action("addSolid", Some(&args), &doc, &ViewState::default());
        assert_eq!(emit.operations.len(), 1);
        match &emit.operations[0] {
            fem_3d::Fem3dOperation::SetSolid { solid, .. } => {
                assert_eq!(solid.outline, vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
                assert_eq!(solid.height, 0.5);
                assert_eq!(solid.layers, 1);
            }
            _ => panic!("expected SetSolid"),
        }
    }

    #[test]
    fn remove_selection_covers_regions_solids_combinations() {
        let mut app_2d = Fem2dPlayApp::default();
        let mut projection_2d = fem_2d::empty_fem2d_projection();
        projection_2d.regions.push(fem_2d::FemRegion { id: "r1".into(), name: "R".into(), outline: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 0.5 });
        projection_2d.combinations.push(fem_2d::FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![("dead".into(), 1.35)] });
        let history = history_view();
        let doc_2d = DocumentView { projection: &projection_2d, history: &history };
        let emit_2d = app_2d.handle_action("removeSelection", Some(&json!({ "ids": ["r1", "uls"] })), &doc_2d, &ViewState::default());
        assert_eq!(emit_2d.operations.len(), 2);
        assert!(matches!(emit_2d.operations[0], fem_2d::Fem2dOperation::RemoveRegion { .. }));
        assert!(matches!(emit_2d.operations[1], fem_2d::Fem2dOperation::RemoveCombination { .. }));

        let mut app_3d = Fem3dPlayApp::default();
        let mut projection_3d = fem_3d::empty_fem3d_projection();
        projection_3d.solids.push(fem_3d::FemSolid { id: "sol1".into(), name: "S".into(), outline: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], holes: vec![], base_z: 0.0, height: 1.0, layers: 1, mesh_size: 0.5, material_id: "concrete".into() });
        let doc_3d = DocumentView { projection: &projection_3d, history: &history };
        let emit_3d = app_3d.handle_action("removeSelection", Some(&json!({ "ids": ["sol1"] })), &doc_3d, &ViewState::default());
        assert_eq!(emit_3d.operations.len(), 1);
        assert!(matches!(emit_3d.operations[0], fem_3d::Fem3dOperation::RemoveSolid { .. }));
    }
    //#endregion 🔖StructureActions

    //#region 🔖LoadCaseActions
    #[test]
    fn add_load_case_and_combination_emit_ops_2d() {
        let mut app = Fem2dPlayApp::default();
        let mut projection = fem_2d::empty_fem2d_projection();
        projection.load_cases.push(fem_2d::FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };

        let emit_case = app.handle_action("addLoadCase", Some(&json!({ "name": "Live", "selfWeight": false })), &doc, &ViewState::default());
        match &emit_case.operations[0] {
            fem_2d::Fem2dOperation::SetLoadCase { load_case, .. } => assert_eq!(load_case.name, "Live"),
            _ => panic!("expected SetLoadCase"),
        }

        let emit_combo = app.handle_action("addCombination", Some(&json!({ "name": "ULS", "terms": "[[\"dead\",1.35]]" })), &doc, &ViewState::default());
        match &emit_combo.operations[0] {
            fem_2d::Fem2dOperation::SetCombination { combination, .. } => assert_eq!(combination.terms, vec![("dead".to_string(), 1.35)]),
            _ => panic!("expected SetCombination"),
        }
    }

    #[test]
    fn add_area_load_targets_named_case_2d() {
        let mut app = Fem2dPlayApp::default();
        let mut projection = fem_2d::empty_fem2d_projection();
        projection.load_cases.push(fem_2d::FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: false });
        projection.load_cases.push(fem_2d::FemLoadCase { id: "live".into(), name: "Live".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("addAreaLoad", Some(&json!({ "regionId": "r1", "pressure": 5000.0, "caseId": "live" })), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem_2d::Fem2dOperation::SetLoadCase { index, load_case } => {
                assert_eq!(*index, 1);
                assert_eq!(load_case.id, "live");
                assert!(matches!(load_case.loads[0], fem_2d::FemLoad::Area { .. }));
            }
            _ => panic!("expected SetLoadCase"),
        }
    }

    #[test]
    fn set_self_weight_toggles_case_2d() {
        let mut app = Fem2dPlayApp::default();
        let mut projection = fem_2d::empty_fem2d_projection();
        projection.load_cases.push(fem_2d::FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("setSelfWeight", Some(&json!({ "caseId": "dead", "enabled": true })), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem_2d::Fem2dOperation::SetLoadCase { load_case, .. } => assert!(load_case.self_weight),
            _ => panic!("expected SetLoadCase"),
        }
    }

    #[test]
    fn set_analysis_settings_partial_args_keep_current_2d() {
        let mut app = Fem2dPlayApp::default();
        let mut projection = fem_2d::empty_fem2d_projection();
        projection.analysis = fem_2d::FemAnalysisSettings { modal_count: 4, buckling_count: 6, deformation_scale: 50.0 };
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("setAnalysisSettings", Some(&json!({ "deformationScale": 300.0 })), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem_2d::Fem2dOperation::SetAnalysisSettings { settings } => {
                assert_eq!(settings.modal_count, 4);
                assert_eq!(settings.buckling_count, 6);
                assert_eq!(settings.deformation_scale, 300.0);
            }
            _ => panic!("expected SetAnalysisSettings"),
        }
    }

    #[test]
    fn add_member_udl_action_emits_op_3d() {
        let mut app = Fem3dPlayApp::default();
        let mut projection = fem_3d::empty_fem3d_projection();
        projection.load_cases.push(fem_3d::FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "elementId": "e1", "wx": 0.0, "wy": 0.0, "wz": -2000.0 });
        let emit = app.handle_action("addMemberUdl", Some(&args), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem_3d::Fem3dOperation::SetLoadCase { load_case, .. } => assert!(matches!(load_case.loads[0], fem_3d::FemLoad::MemberUdl { .. })),
            _ => panic!("expected SetLoadCase"),
        }
    }
    //#endregion 🔖LoadCaseActions

    //#region 🔖SharedHelpers
    #[test]
    fn next_id_retries_past_collisions() {
        let existing = vec!["n0".to_string(), "n2".to_string()];
        assert_eq!(next_id(existing.into_iter(), "n"), "n3");
    }

    #[test]
    fn hex_to_rgb01_parses_pure_colors() {
        assert_eq!(hex_to_rgb01("#ffffff"), (1.0, 1.0, 1.0));
        assert_eq!(hex_to_rgb01("#000000"), (0.0, 0.0, 0.0));
        assert_eq!(hex_to_rgb01("#ff0000"), (1.0, 0.0, 0.0));
    }

    #[test]
    fn von_mises_color_maps_extremes_midpoint_and_clamps() {
        assert_eq!(von_mises_color(0.0, 0.0, 100.0), VON_MISES_BANDS[0]);
        assert_eq!(von_mises_color(100.0, 0.0, 100.0), VON_MISES_BANDS[VON_MISES_BANDS.len() - 1]);
        assert_eq!(von_mises_color(50.0, 0.0, 100.0), VON_MISES_BANDS[VON_MISES_BANDS.len() / 2]);
        assert_eq!(von_mises_color(-10.0, 0.0, 100.0), VON_MISES_BANDS[0]);
        assert_eq!(von_mises_color(200.0, 0.0, 100.0), VON_MISES_BANDS[VON_MISES_BANDS.len() - 1]);
    }

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
    fn quat_z_to_identity_for_parallel_direction() {
        assert_eq!(quat_z_to([0.0, 0.0, 1.0]), [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn quat_z_to_handles_antiparallel_direction() {
        assert_eq!(quat_z_to([0.0, 0.0, -1.0]), [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn fem2d_resolve_load_case_synthesizes_case_when_none_exist() {
        let projection = fem_2d::empty_fem2d_projection();
        let (index, load_case) = fem2d_resolve_load_case(&projection, None);
        assert_eq!(index, 0);
        assert_eq!(load_case.id, "case-1");
    }

    #[test]
    fn fem3d_resolve_load_case_synthesizes_case_when_none_exist() {
        let projection = fem_3d::empty_fem3d_projection();
        let (index, load_case) = fem3d_resolve_load_case(&projection, None);
        assert_eq!(index, 0);
        assert_eq!(load_case.id, "case-1");
    }

    #[test]
    fn fem2d_model_extent_degenerate_model_returns_one() {
        assert_eq!(fem2d_model_extent(&fem_2d::empty_fem2d_projection()), 1.0);
    }

    #[test]
    fn fem3d_model_extent_degenerate_model_returns_one() {
        assert_eq!(fem3d_model_extent(&fem_3d::empty_fem3d_projection()), 1.0);
    }

    #[test]
    fn parse_result_display_unknown_mode_falls_back_to_static() {
        assert_eq!(parse_result_display(Some(&json!({ "mode": "bogus" }))).mode, DisplayMode::Static);
    }

    #[test]
    fn parse_result_display_missing_args_defaults_to_static_with_no_source() {
        let display = parse_result_display(None);
        assert_eq!(display.mode, DisplayMode::Static);
        assert!(display.source_id.is_none());
    }
    //#endregion 🔖SharedHelpers

    //#region 🔖UnknownBodyAndGermanLabels
    #[test]
    fn render_unknown_body_key_returns_placeholder_text_2d() {
        let app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let json = serde_json::to_string(&app.render("nonsense", &doc, &ViewState::default())).unwrap();
        assert!(json.contains("Unknown body: nonsense"));
    }

    #[test]
    fn render_unknown_body_key_returns_placeholder_text_3d() {
        let app = Fem3dPlayApp::default();
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let json = serde_json::to_string(&app.render("nonsense", &doc, &ViewState::default())).unwrap();
        assert!(json.contains("Unknown body: nonsense"));
    }

    #[test]
    fn app_labels_use_german_locale_2d() {
        let app = Fem2dPlayApp::default();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let labels = app.app_labels(&view_state);
        assert_eq!(labels.window_kind_labels.get(FEM2D_WINDOW_MODEL).map(String::as_str), Some("Modell"));
        assert_eq!(labels.action_labels.get("addNode").map(String::as_str), Some("Knoten hinzufügen"));
    }

    #[test]
    fn app_labels_use_german_locale_3d() {
        let app = Fem3dPlayApp::default();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let labels = app.app_labels(&view_state);
        assert_eq!(labels.window_kind_labels.get(FEM3D_WINDOW_MODEL).map(String::as_str), Some("Modell"));
        assert_eq!(labels.action_labels.get("addFrame").map(String::as_str), Some("Rahmen hinzufügen"));
    }

    #[test]
    fn results_window_buckling_with_no_load_case_shows_placeholder_2d() {
        let app = Fem2dPlayApp { result_display: ResultDisplay { source_id: None, mode: DisplayMode::Buckling(0) } };
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let json = serde_json::to_string(&app.render(FEM2D_BODY_RESULTS, &doc, &ViewState::default())).unwrap();
        assert!(json.contains("No load case defined"), "{json}");
    }

    #[test]
    fn results_window_buckling_with_no_load_case_shows_placeholder_3d() {
        let app = Fem3dPlayApp { result_display: ResultDisplay { source_id: None, mode: DisplayMode::Buckling(0) } };
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let json = serde_json::to_string(&app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default())).unwrap();
        assert!(json.contains("No load case defined"), "{json}");
    }
    //#endregion 🔖UnknownBodyAndGermanLabels

    //#region 🔖MoreStructureAndLoadActions
    #[test]
    fn add_bar_and_add_beam_actions_emit_ops_2d() {
        let mut app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "start": "n1", "end": "n2", "materialId": "m1", "sectionId": "s1" });

        let emit_bar = app.handle_action("addBar", Some(&args), &doc, &ViewState::default());
        match &emit_bar.operations[0] {
            fem_2d::Fem2dOperation::SetElement { element, .. } => assert!(matches!(element, fem_2d::FemElement::Bar { .. })),
            _ => panic!("expected SetElement"),
        }

        let emit_beam = app.handle_action("addBeam", Some(&args), &doc, &ViewState::default());
        match &emit_beam.operations[0] {
            fem_2d::Fem2dOperation::SetElement { element, .. } => assert!(matches!(element, fem_2d::FemElement::Beam { .. })),
            _ => panic!("expected SetElement"),
        }
    }

    #[test]
    fn add_material_action_emits_op_2d() {
        let mut app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("addMaterial", Some(&json!({ "name": "Steel", "e": 2.1e11 })), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem_2d::Fem2dOperation::SetMaterial { material, .. } => {
                assert_eq!(material.name, "Steel");
                assert_eq!(material.e, 2.1e11);
            }
            _ => panic!("expected SetMaterial"),
        }
    }

    #[test]
    fn add_material_action_emits_op_3d() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "name": "Steel", "e": 2.1e11, "g": 8.1e10 });
        let emit = app.handle_action("addMaterial", Some(&args), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem_3d::Fem3dOperation::SetMaterial { material, .. } => assert_eq!(material.g, 8.1e10),
            _ => panic!("expected SetMaterial"),
        }
    }

    #[test]
    fn add_section_action_emits_op_2d() {
        let mut app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "name": "HEA200", "area": 0.00538, "iy": 0.0000369 });
        let emit = app.handle_action("addSection", Some(&args), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem_2d::Fem2dOperation::SetSection { section, .. } => assert_eq!(section.name, "HEA200"),
            _ => panic!("expected SetSection"),
        }
    }

    #[test]
    fn add_section_action_emits_op_3d() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "name": "HEA200", "area": 0.00538, "iy": 0.0000369, "iz": 0.0000133, "j": 0.0000006 });
        let emit = app.handle_action("addSection", Some(&args), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem_3d::Fem3dOperation::SetSection { section, .. } => assert_eq!(section.j, 0.0000006),
            _ => panic!("expected SetSection"),
        }
    }

    #[test]
    fn add_support_action_emits_op_with_fixed_dofs_2d() {
        let mut app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "nodeId": "n1", "fixed": ["Tx", "Ty"] });
        let emit = app.handle_action("addSupport", Some(&args), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem_2d::Fem2dOperation::SetSupport { support, .. } => assert_eq!(support.fixed, vec![Dof::Tx, Dof::Ty]),
            _ => panic!("expected SetSupport"),
        }
    }

    #[test]
    fn add_nodal_load_action_targets_named_case_2d() {
        let mut app = Fem2dPlayApp::default();
        let mut projection = fem_2d::empty_fem2d_projection();
        projection.load_cases.push(fem_2d::FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: false });
        projection.load_cases.push(fem_2d::FemLoadCase { id: "live".into(), name: "Live".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "nodeId": "n1", "dof": "Ty", "value": -5000.0, "caseId": "live" });
        let emit = app.handle_action("addNodalLoad", Some(&args), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem_2d::Fem2dOperation::SetLoadCase { index, load_case } => {
                assert_eq!(*index, 1);
                assert!(matches!(load_case.loads[0], fem_2d::FemLoad::Nodal { .. }));
            }
            _ => panic!("expected SetLoadCase"),
        }
    }

    #[test]
    fn add_member_udl_action_emits_op_2d() {
        let mut app = Fem2dPlayApp::default();
        let mut projection = fem_2d::empty_fem2d_projection();
        projection.load_cases.push(fem_2d::FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "elementId": "e1", "wx": 0.0, "wy": -500.0 });
        let emit = app.handle_action("addMemberUdl", Some(&args), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem_2d::Fem2dOperation::SetLoadCase { load_case, .. } => assert!(matches!(load_case.loads[0], fem_2d::FemLoad::MemberUdl { .. })),
            _ => panic!("expected SetLoadCase"),
        }
    }

    #[test]
    fn add_frame_action_emits_op_3d() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "start": "n1", "end": "n2", "materialId": "m1", "sectionId": "s1", "roll": 0.5 });
        let emit = app.handle_action("addFrame", Some(&args), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem_3d::Fem3dOperation::SetElement { element, .. } => match element {
                fem_3d::FemElement::Frame { roll, .. } => assert_eq!(*roll, 0.5),
                _ => panic!("expected Frame"),
            },
            _ => panic!("expected SetElement"),
        }
    }

    #[test]
    fn set_camera_action_amends_2d() {
        let mut app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("setCamera", Some(&json!({ "x": 1.0, "y": 2.0, "zoom": 1.5 })), &doc, &ViewState::default());
        assert_eq!(emit.coalesce_key.as_deref(), Some("camera"));
        match &emit.operations[0] {
            fem_2d::Fem2dOperation::SetCamera { camera } => {
                assert_eq!(camera.x, 1.0);
                assert_eq!(camera.zoom, 1.5);
            }
            _ => panic!("expected SetCamera"),
        }
    }

    #[test]
    fn set_camera_action_amends_3d() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("setCamera", Some(&json!({ "json": "{\"x\":1}" })), &doc, &ViewState::default());
        assert_eq!(emit.coalesce_key.as_deref(), Some("camera"));
        match &emit.operations[0] {
            fem_3d::Fem3dOperation::SetCamera { camera } => assert_eq!(camera.json, "{\"x\":1}"),
            _ => panic!("expected SetCamera"),
        }
    }

    #[test]
    fn remove_selection_covers_nodes_elements_materials_sections_supports_load_cases_2d() {
        let mut app = Fem2dPlayApp::default();
        let mut projection = fem_2d::empty_fem2d_projection();
        projection.nodes.push(fem_2d::FemNode { id: "n1".into(), x: 0.0, y: 0.0 });
        projection.elements.push(fem_2d::FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n1".into(), material_id: "m1".into(), section_id: "s1".into() });
        projection.materials.push(fem_2d::FemMaterial { id: "m1".into(), name: "Steel".into(), e: 2.1e11, nu: 0.3, rho: 7850.0 });
        projection.sections.push(fem_2d::FemSection { id: "s1".into(), name: "Sec".into(), area: 0.01, iy: 0.0001 });
        projection.supports.push(fem_2d::FemSupport { id: "sup1".into(), node_id: "n1".into(), fixed: vec![Dof::Tx] });
        projection.load_cases.push(fem_2d::FemLoadCase { id: "case1".into(), name: "Case".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "ids": ["n1", "e1", "m1", "s1", "sup1", "case1"] });
        let emit = app.handle_action("removeSelection", Some(&args), &doc, &ViewState::default());
        assert_eq!(emit.operations.len(), 6);
        assert!(matches!(emit.operations[0], fem_2d::Fem2dOperation::RemoveNode { .. }));
        assert!(matches!(emit.operations[1], fem_2d::Fem2dOperation::RemoveElement { .. }));
        assert!(matches!(emit.operations[2], fem_2d::Fem2dOperation::RemoveMaterial { .. }));
        assert!(matches!(emit.operations[3], fem_2d::Fem2dOperation::RemoveSection { .. }));
        assert!(matches!(emit.operations[4], fem_2d::Fem2dOperation::RemoveSupport { .. }));
        assert!(matches!(emit.operations[5], fem_2d::Fem2dOperation::RemoveLoadCase { .. }));
    }

    #[test]
    fn add_node_action_missing_args_yields_no_operation_2d() {
        let mut app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("addNode", Some(&json!({ "x": 1.0 })), &doc, &ViewState::default());
        assert!(emit.operations.is_empty());
    }

    #[test]
    fn add_combination_invalid_terms_json_yields_no_operation_2d() {
        let mut app = Fem2dPlayApp::default();
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "name": "Bad", "terms": "not-json" });
        let emit = app.handle_action("addCombination", Some(&args), &doc, &ViewState::default());
        assert!(emit.operations.is_empty());
    }
    //#endregion 🔖MoreStructureAndLoadActions
}
//#endregion 🧪Tests
