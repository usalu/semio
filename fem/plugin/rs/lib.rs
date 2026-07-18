//! 🏗️ FEM WASM plugin: `fem2d-play` and `fem3d-play` apps registered as one hot-swappable component.

use fem_core::{Dof, ElementResult};
use semio_framework_plugin::{
    build_canvas_2d_scene, build_world_3d_scene, create_default_layout, ui_text, world3d_default_camera,
    world3d_default_selection_json, world3d_meshes_json_from_kinds, world3d_scene, AppLabelsOverlay, ActionArgDef,
    ActionEmit, App, Canvas2dScene, DocumentApp, DocumentView, HistoryView, SurfaceKind, UiNode, ViewState,
    WorldSunConfig,
};
use serde_json::{json, Value};
use std::collections::HashMap;

// #region 🔖Constants
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

/// 📐 Model-meters -> screen-pixels scale for the 2D canvas (a 6m span shouldn't render as 6px wide).
const SCALE_2D: f64 = 20.0;
/// 📐 Screen-space origin offset so a structure anchored at (0,0) isn't drawn at the canvas corner.
const ORIGIN_2D: f64 = 40.0;
/// 📐 Exaggeration factor for rendering (tiny, meter-scale) displacements as a visible deformed shape.
const DEFORM_SCALE_2D: f64 = 500.0;
/// 📐 Exaggeration factor for offsetting the moment-diagram polyline perpendicular to a member.
const MOMENT_SCALE_2D: f64 = 0.001;

/// 🧊 Exaggeration factor for rendering (tiny, meter-scale) 3D displacements.
const DEFORM_SCALE_3D: f64 = 200.0;
/// 🧊 Half-extent-ish scale of the small box instance drawn at each node.
const NODE_SIZE_3D: f64 = 0.05;
/// 🧊 Scale of each small sphere marker used to approximate a member's extent (see `fem3d_instances_json`).
const MEMBER_MARKER_SIZE_3D: f64 = 0.03;
/// 🧊 Number of sphere markers interpolated along a member's length.
const MEMBER_SEGMENTS_3D: usize = 5;
// #endregion 🔖Constants

// #region 🔖Shared
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
// #endregion 🔖Shared

// #region 🔖Fem2dRender
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

fn render_fem2d_model(doc: &fem_2d::Fem2dDocument) -> UiNode {
    let layers = fem2d_structure_layers(doc, "#38bdf8", "#94a3b8", "#f97316");
    let layers_json = serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into());
    build_canvas_2d_scene(FEM2D_BODY_MODEL, FEM2D_APP_ID, Canvas2dScene { camera_x: doc.camera.x, camera_y: doc.camera.y, zoom: doc.camera.zoom, layers_json })
}

/// 📊 Results window: solved fresh on every render (see `Fem2dPlayApp` doc comment) — undeformed
/// structure faintly, plus a deformed-shape polyline and (for beams) a moment-diagram polyline per
/// element. v0 limitation: reaction values are not rendered as text labels.
fn render_fem2d_results(doc: &fem_2d::Fem2dDocument) -> UiNode {
    let Some(case) = doc.load_cases.first() else {
        return ui_text("No load case defined");
    };
    let result = match fem_2d::fem2d_solve(doc, &case.id) {
        Ok(result) => result,
        Err(e) => return ui_text(format!("Analysis error: {e}")),
    };

    let mut layers = fem2d_structure_layers(doc, "#334155", "#334155", "#334155");
    let mut disp_map: HashMap<String, [f64; 6]> = HashMap::new();
    for d in &result.displacements {
        disp_map.insert(d.node_id.clone(), d.values);
    }

    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(n1), Some(n2)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let (x0, y0) = screen_2d(n1.x, n1.y);
        let (x1, y1) = screen_2d(n2.x, n2.y);
        let d1 = disp_map.get(&n1.id).copied().unwrap_or([0.0; 6]);
        let d2 = disp_map.get(&n2.id).copied().unwrap_or([0.0; 6]);
        let dx0 = d1[Dof::Tx.index()] * DEFORM_SCALE_2D * SCALE_2D;
        let dy0 = -d1[Dof::Ty.index()] * DEFORM_SCALE_2D * SCALE_2D;
        let dx1 = d2[Dof::Tx.index()] * DEFORM_SCALE_2D * SCALE_2D;
        let dy1 = -d2[Dof::Ty.index()] * DEFORM_SCALE_2D * SCALE_2D;
        layers.push(json!({
            "kind": "polyline",
            "id": format!("deformed-{}", fem_2d::element_id(element)),
            "points": [[x0 + dx0, y0 + dy0], [x1 + dx1, y1 + dy1]],
            "color": "#f472b6",
        }));

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

    let layers_json = serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into());
    build_canvas_2d_scene(FEM2D_BODY_RESULTS, FEM2D_APP_ID, Canvas2dScene { camera_x: doc.camera.x, camera_y: doc.camera.y, zoom: doc.camera.zoom, layers_json })
}
// #endregion 🔖Fem2dRender

// #region 🔖Fem2dPlayApp
/// 🧮 v0 design: results are never persisted or cached — `fem2d_solve` runs fresh inside `render()`
/// whenever the results window is drawn. At v0 scale (≤10 nodes) this is cheap and correct-by-
/// construction (no cache-invalidation bugs to get wrong). There is no `RunAnalysis` op: solving is a
/// pure function of the document. The active load case is always `load_cases.first()`; a load-case
/// switcher UI is future work.
#[derive(Default)]
struct Fem2dPlayApp;

impl DocumentApp for Fem2dPlayApp {
    type Projection = fem_2d::Fem2dDocument;
    type Op = fem_2d::Fem2dOp;

    fn app_id(&self) -> &str {
        FEM2D_APP_ID
    }

    fn document_schema(&self) -> &str {
        fem_2d::FEM_2D_SCHEMA
    }

    fn initial_projection(&self) -> fem_2d::Fem2dDocument {
        fem_2d::empty_fem2d_projection()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, fem_2d::Fem2dDocument>, _view_state: &ViewState) -> ActionEmit<fem_2d::Fem2dOp> {
        match action {
            "addNode" => {
                if let (Some(x), Some(y)) = (args.and_then(|v| v.get("x")).and_then(Value::as_f64), args.and_then(|v| v.get("y")).and_then(Value::as_f64)) {
                    let id = next_id(doc.projection.nodes.iter().map(|n| n.id.clone()), "n");
                    let index = doc.projection.nodes.len();
                    return ActionEmit::ops(vec![fem_2d::Fem2dOp::SetNode { index, node: fem_2d::FemNode { id, x, y } }]);
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
                    return ActionEmit::ops(vec![fem_2d::Fem2dOp::SetElement { index, element }]);
                }
            }
            "addMaterial" => {
                if let (Some(name), Some(e)) = (args.and_then(|v| v.get("name")).and_then(Value::as_str), args.and_then(|v| v.get("e")).and_then(Value::as_f64)) {
                    let id = next_id(doc.projection.materials.iter().map(|m| m.id.clone()), "m");
                    let index = doc.projection.materials.len();
                    return ActionEmit::ops(vec![fem_2d::Fem2dOp::SetMaterial { index, material: fem_2d::FemMaterial { id, name: name.into(), e } }]);
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
                    return ActionEmit::ops(vec![fem_2d::Fem2dOp::SetSection { index, section: fem_2d::FemSection { id, name: name.into(), area, iy } }]);
                }
            }
            "addSupport" => {
                if let Some(node_id) = args.and_then(|v| v.get("nodeId")).and_then(Value::as_str) {
                    let fixed: Vec<Dof> = args.and_then(|v| v.get("fixed")).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                    let id = next_id(doc.projection.supports.iter().map(|s| s.id.clone()), "sup");
                    let index = doc.projection.supports.len();
                    return ActionEmit::ops(vec![fem_2d::Fem2dOp::SetSupport { index, support: fem_2d::FemSupport { id, node_id: node_id.into(), fixed } }]);
                }
            }
            "addNodalLoad" => {
                if let (Some(node_id), Some(dof), Some(value)) = (
                    args.and_then(|v| v.get("nodeId")).and_then(Value::as_str),
                    args.and_then(|v| v.get("dof")).and_then(|v| serde_json::from_value::<Dof>(v.clone()).ok()),
                    args.and_then(|v| v.get("value")).and_then(Value::as_f64),
                ) {
                    let mut load_case = doc.projection.load_cases.first().cloned().unwrap_or_else(|| fem_2d::FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: Vec::new() });
                    let load_id = next_id(load_case.loads.iter().map(|l| fem_2d::load_id(l).to_string()), "l");
                    load_case.loads.push(fem_2d::FemLoad::Nodal { id: load_id, node_id: node_id.into(), dof, value });
                    let index = doc.projection.load_cases.iter().position(|lc| lc.id == load_case.id).unwrap_or(doc.projection.load_cases.len());
                    return ActionEmit::ops(vec![fem_2d::Fem2dOp::SetLoadCase { index, load_case }]);
                }
            }
            "addMemberUdl" => {
                if let (Some(element_id), Some(wx), Some(wy)) = (
                    args.and_then(|v| v.get("elementId")).and_then(Value::as_str),
                    args.and_then(|v| v.get("wx")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("wy")).and_then(Value::as_f64),
                ) {
                    let mut load_case = doc.projection.load_cases.first().cloned().unwrap_or_else(|| fem_2d::FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: Vec::new() });
                    let load_id = next_id(load_case.loads.iter().map(|l| fem_2d::load_id(l).to_string()), "l");
                    load_case.loads.push(fem_2d::FemLoad::MemberUdl { id: load_id, element_id: element_id.into(), wx, wy });
                    let index = doc.projection.load_cases.iter().position(|lc| lc.id == load_case.id).unwrap_or(doc.projection.load_cases.len());
                    return ActionEmit::ops(vec![fem_2d::Fem2dOp::SetLoadCase { index, load_case }]);
                }
            }
            "removeSelection" => {
                let ids: Vec<String> = args.and_then(|v| v.get("ids")).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                let mut ops = Vec::new();
                for id in ids {
                    if doc.projection.nodes.iter().any(|n| n.id == id) {
                        ops.push(fem_2d::Fem2dOp::RemoveNode { id });
                    } else if doc.projection.elements.iter().any(|e| fem_2d::element_id(e) == id) {
                        ops.push(fem_2d::Fem2dOp::RemoveElement { id });
                    } else if doc.projection.materials.iter().any(|m| m.id == id) {
                        ops.push(fem_2d::Fem2dOp::RemoveMaterial { id });
                    } else if doc.projection.sections.iter().any(|s| s.id == id) {
                        ops.push(fem_2d::Fem2dOp::RemoveSection { id });
                    } else if doc.projection.supports.iter().any(|s| s.id == id) {
                        ops.push(fem_2d::Fem2dOp::RemoveSupport { id });
                    } else if doc.projection.load_cases.iter().any(|l| l.id == id) {
                        ops.push(fem_2d::Fem2dOp::RemoveLoadCase { id });
                    }
                }
                if !ops.is_empty() {
                    return ActionEmit::ops(ops);
                }
            }
            "setCamera" => {
                if let (Some(x), Some(y), Some(zoom)) = (
                    args.and_then(|v| v.get("x")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("y")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("zoom")).and_then(Value::as_f64),
                ) {
                    return ActionEmit::amend(vec![fem_2d::Fem2dOp::SetCamera { camera: fem_2d::FemCamera { x, y, zoom } }], "camera");
                }
            }
            _ => {}
        }
        ActionEmit::default()
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, fem_2d::Fem2dDocument>, _view_state: &ViewState) -> UiNode {
        match body_key {
            FEM2D_BODY_MODEL => render_fem2d_model(doc.projection),
            FEM2D_BODY_RESULTS => render_fem2d_results(doc.projection),
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
        }
    }
}
// #endregion 🔖Fem2dPlayApp

// #region 🔖Fem2dTerminology
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
        ("removeSelection", "Remove Selection", "Auswahl entfernen"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
    ];
    ENTRIES.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
}
// #endregion 🔖Fem2dTerminology

// #region 🔖Fem3dRender
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

fn lerp3(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// 🧊 Builds World3d `instances_json`: one small box per node, plus a chain of small sphere markers
/// interpolated along each member's length. `displacements` (node id -> 6-DOF values), when present,
/// offsets every node position before building instances — used by the results window's deformed view.
/// v0 simplification: members are approximated by interpolated sphere markers rather than an oriented
/// extruded prism (a proper stretched/rotated box transform is future work).
fn fem3d_instances_json(doc: &fem_3d::Fem3dDocument, displacements: Option<&HashMap<String, [f64; 6]>>) -> String {
    let node_pos = |node: &fem_3d::FemNode| -> [f64; 3] {
        let mut p = [node.x, node.y, node.z];
        if let Some(map) = displacements {
            if let Some(d) = map.get(&node.id) {
                p[0] += d[Dof::Tx.index()] * DEFORM_SCALE_3D;
                p[1] += d[Dof::Ty.index()] * DEFORM_SCALE_3D;
                p[2] += d[Dof::Tz.index()] * DEFORM_SCALE_3D;
            }
        }
        p
    };

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
        let id = fem3d_element_id(element);
        for i in 0..=MEMBER_SEGMENTS_3D {
            let t = i as f64 / MEMBER_SEGMENTS_3D as f64;
            let p = lerp3(p1, p2, t);
            instances.push(json!({
                "id": format!("el-{id}-{i}"),
                "meshId": "sphere",
                "position": p,
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [MEMBER_MARKER_SIZE_3D, MEMBER_MARKER_SIZE_3D, MEMBER_MARKER_SIZE_3D],
                "label": id,
            }));
        }
    }
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn fem3d_camera_json(doc: &fem_3d::Fem3dDocument) -> String {
    if doc.camera.json == "{}" {
        world3d_default_camera()
    } else {
        doc.camera.json.clone()
    }
}

fn render_fem3d_model(doc: &fem_3d::Fem3dDocument) -> UiNode {
    let meshes_json = world3d_meshes_json_from_kinds(&["box".to_string(), "sphere".to_string()]);
    let instances_json = fem3d_instances_json(doc, None);
    build_world_3d_scene(
        FEM3D_BODY_MODEL,
        FEM3D_APP_ID,
        world3d_scene(fem3d_camera_json(doc), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()),
    )
}

/// 📊 Results window: solved fresh on every render (see `Fem3dPlayApp` doc comment) — same node/member
/// instances as the model window, offset by the solved displacements.
fn render_fem3d_results(doc: &fem_3d::Fem3dDocument) -> UiNode {
    let Some(case) = doc.load_cases.first() else {
        return ui_text("No load case defined");
    };
    let result = match fem_3d::fem3d_solve(doc, &case.id) {
        Ok(result) => result,
        Err(e) => return ui_text(format!("Analysis error: {e}")),
    };
    let mut disp_map: HashMap<String, [f64; 6]> = HashMap::new();
    for d in &result.displacements {
        disp_map.insert(d.node_id.clone(), d.values);
    }
    let meshes_json = world3d_meshes_json_from_kinds(&["box".to_string(), "sphere".to_string()]);
    let instances_json = fem3d_instances_json(doc, Some(&disp_map));
    build_world_3d_scene(
        FEM3D_BODY_RESULTS,
        FEM3D_APP_ID,
        world3d_scene(fem3d_camera_json(doc), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()),
    )
}
// #endregion 🔖Fem3dRender

// #region 🔖Fem3dPlayApp
/// 🧮 v0 design: mirrors `Fem2dPlayApp` — results are recomputed fresh inside `render()`, no cache, no
/// `RunAnalysis` op, active load case is always `load_cases.first()`.
#[derive(Default)]
struct Fem3dPlayApp;

impl DocumentApp for Fem3dPlayApp {
    type Projection = fem_3d::Fem3dDocument;
    type Op = fem_3d::Fem3dOp;

    fn app_id(&self) -> &str {
        FEM3D_APP_ID
    }

    fn document_schema(&self) -> &str {
        fem_3d::FEM_3D_SCHEMA
    }

    fn initial_projection(&self) -> fem_3d::Fem3dDocument {
        fem_3d::empty_fem3d_projection()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, fem_3d::Fem3dDocument>, _view_state: &ViewState) -> ActionEmit<fem_3d::Fem3dOp> {
        match action {
            "addNode" => {
                if let (Some(x), Some(y), Some(z)) = (
                    args.and_then(|v| v.get("x")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("y")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("z")).and_then(Value::as_f64),
                ) {
                    let id = next_id(doc.projection.nodes.iter().map(|n| n.id.clone()), "n");
                    let index = doc.projection.nodes.len();
                    return ActionEmit::ops(vec![fem_3d::Fem3dOp::SetNode { index, node: fem_3d::FemNode { id, x, y, z } }]);
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
                    return ActionEmit::ops(vec![fem_3d::Fem3dOp::SetElement { index, element }]);
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
                    return ActionEmit::ops(vec![fem_3d::Fem3dOp::SetElement { index, element }]);
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
                    return ActionEmit::ops(vec![fem_3d::Fem3dOp::SetMaterial { index, material: fem_3d::FemMaterial { id, name: name.into(), e, g } }]);
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
                    return ActionEmit::ops(vec![fem_3d::Fem3dOp::SetSection { index, section: fem_3d::FemSection { id, name: name.into(), area, iy, iz, j } }]);
                }
            }
            "addSupport" => {
                if let Some(node_id) = args.and_then(|v| v.get("nodeId")).and_then(Value::as_str) {
                    let fixed: Vec<Dof> = args.and_then(|v| v.get("fixed")).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                    let id = next_id(doc.projection.supports.iter().map(|s| s.id.clone()), "sup");
                    let index = doc.projection.supports.len();
                    return ActionEmit::ops(vec![fem_3d::Fem3dOp::SetSupport { index, support: fem_3d::FemSupport { id, node_id: node_id.into(), fixed } }]);
                }
            }
            "addNodalLoad" => {
                if let (Some(node_id), Some(dof), Some(value)) = (
                    args.and_then(|v| v.get("nodeId")).and_then(Value::as_str),
                    args.and_then(|v| v.get("dof")).and_then(|v| serde_json::from_value::<Dof>(v.clone()).ok()),
                    args.and_then(|v| v.get("value")).and_then(Value::as_f64),
                ) {
                    let mut load_case = doc.projection.load_cases.first().cloned().unwrap_or_else(|| fem_3d::FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: Vec::new() });
                    let load_id = next_id(load_case.loads.iter().map(|l| l.id.clone()), "l");
                    load_case.loads.push(fem_3d::FemNodalLoad { id: load_id, node_id: node_id.into(), dof, value });
                    let index = doc.projection.load_cases.iter().position(|lc| lc.id == load_case.id).unwrap_or(doc.projection.load_cases.len());
                    return ActionEmit::ops(vec![fem_3d::Fem3dOp::SetLoadCase { index, load_case }]);
                }
            }
            "removeSelection" => {
                let ids: Vec<String> = args.and_then(|v| v.get("ids")).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                let mut ops = Vec::new();
                for id in ids {
                    if doc.projection.nodes.iter().any(|n| n.id == id) {
                        ops.push(fem_3d::Fem3dOp::RemoveNode { id });
                    } else if doc.projection.elements.iter().any(|e| fem3d_element_id(e) == id) {
                        ops.push(fem_3d::Fem3dOp::RemoveElement { id });
                    } else if doc.projection.materials.iter().any(|m| m.id == id) {
                        ops.push(fem_3d::Fem3dOp::RemoveMaterial { id });
                    } else if doc.projection.sections.iter().any(|s| s.id == id) {
                        ops.push(fem_3d::Fem3dOp::RemoveSection { id });
                    } else if doc.projection.supports.iter().any(|s| s.id == id) {
                        ops.push(fem_3d::Fem3dOp::RemoveSupport { id });
                    } else if doc.projection.load_cases.iter().any(|l| l.id == id) {
                        ops.push(fem_3d::Fem3dOp::RemoveLoadCase { id });
                    }
                }
                if !ops.is_empty() {
                    return ActionEmit::ops(ops);
                }
            }
            "setCamera" => {
                if let Some(json_str) = args.and_then(|v| v.get("json")).and_then(Value::as_str) {
                    return ActionEmit::amend(vec![fem_3d::Fem3dOp::SetCamera { camera: fem_3d::FemCamera { json: json_str.into() } }], "camera");
                }
            }
            _ => {}
        }
        ActionEmit::default()
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, fem_3d::Fem3dDocument>, _view_state: &ViewState) -> UiNode {
        match body_key {
            FEM3D_BODY_MODEL => render_fem3d_model(doc.projection),
            FEM3D_BODY_RESULTS => render_fem3d_results(doc.projection),
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
        }
    }
}
// #endregion 🔖Fem3dPlayApp

// #region 🔖Fem3dTerminology
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
        ("removeSelection", "Remove Selection", "Auswahl entfernen"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
    ];
    ENTRIES.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
}
// #endregion 🔖Fem3dTerminology

// #region 🔖Manifest
fn create_fem2d_app() -> App {
    App::from_builder(
        App::builder(FEM2D_APP_ID, "FEM 2D")
            .document(["semio", "fem", "fem2d"])
            .icon_id("fem")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(FEM2D_WINDOW_MODEL, "Model", FEM2D_BODY_MODEL, SurfaceKind::Canvas2d)
            .window_kind(FEM2D_WINDOW_RESULTS, "Results", FEM2D_BODY_RESULTS, SurfaceKind::Canvas2d)
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
            .operation("addMemberUdl", "Add Member UDL")
            .operation("removeSelection", "Remove Selection")
            .operation("setCamera", "Set Camera"),
    )
    .example("default", "Default", include_str!("../../2d/example/default.fem2d.json"))
    .program("fem2d", "FEM 2D", "structure")
}

fn create_fem3d_app() -> App {
    App::from_builder(
        App::builder(FEM3D_APP_ID, "FEM 3D")
            .document(["semio", "fem", "fem3d"])
            .icon_id("fem")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(FEM3D_WINDOW_MODEL, "Model", FEM3D_BODY_MODEL, SurfaceKind::World3d)
            .window_kind(FEM3D_WINDOW_RESULTS, "Results", FEM3D_BODY_RESULTS, SurfaceKind::World3d)
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
            .operation("removeSelection", "Remove Selection")
            .operation("setCamera", "Set Camera"),
    )
    .example("default", "Default", include_str!("../../3d/example/default.fem3d.json"))
    .program("fem3d", "FEM 3D", "structure")
}

fn register_fem_exports() {}

semio_framework_plugin::semio_plugin! {
    id: "fem", label: "FEM", version: "0.1.0",
    setup: register_fem_exports,
    apps: [ create_fem2d_app => Fem2dPlayApp, create_fem3d_app => Fem3dPlayApp ],
}
// #endregion 🔖Manifest

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn history_view() -> HistoryView {
        HistoryView { columns: Vec::new(), can_undo: false, can_redo: false, active_alternative_id: None, current_checkpoint_id: None }
    }

    // #region 🔖RendersScenes
    #[test]
    fn renders_fem2d_model_scene() {
        let app = Fem2dPlayApp;
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM2D_BODY_MODEL, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn renders_fem2d_results_scene() {
        let app = Fem2dPlayApp;
        let json_fixture = include_str!("../../2d/example/default.fem2d.json");
        let projection: fem_2d::Fem2dDocument = serde_json::from_str(json_fixture).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM2D_BODY_RESULTS, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn renders_fem3d_model_scene() {
        let app = Fem3dPlayApp;
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM3D_BODY_MODEL, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn renders_fem3d_results_scene() {
        let app = Fem3dPlayApp;
        let json_fixture = include_str!("../../3d/example/default.fem3d.json");
        let projection: fem_3d::Fem3dDocument = serde_json::from_str(json_fixture).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }
    // #endregion 🔖RendersScenes

    // #region 🔖AddNodeAction
    #[test]
    fn add_node_action_emits_op_2d() {
        let mut app = Fem2dPlayApp;
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "x": 1.0, "y": 2.0 });
        let emit = app.handle_action("addNode", Some(&args), &doc, &ViewState::default());
        assert_eq!(emit.ops.len(), 1);
        match &emit.ops[0] {
            fem_2d::Fem2dOp::SetNode { node, .. } => {
                assert_eq!(node.x, 1.0);
                assert_eq!(node.y, 2.0);
            }
            _ => panic!("expected SetNode"),
        }
    }

    #[test]
    fn add_node_action_emits_op_3d() {
        let mut app = Fem3dPlayApp;
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "x": 1.0, "y": 2.0, "z": 3.0 });
        let emit = app.handle_action("addNode", Some(&args), &doc, &ViewState::default());
        assert_eq!(emit.ops.len(), 1);
        match &emit.ops[0] {
            fem_3d::Fem3dOp::SetNode { node, .. } => {
                assert_eq!(node.x, 1.0);
                assert_eq!(node.y, 2.0);
                assert_eq!(node.z, 3.0);
            }
            _ => panic!("expected SetNode"),
        }
    }
    // #endregion 🔖AddNodeAction

    // #region 🔖SolverErrorSurfaced
    #[test]
    fn results_window_surfaces_solver_error_without_panicking_2d() {
        let app = Fem2dPlayApp;
        let projection = fem_2d::empty_fem2d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let _ = app.render(FEM2D_BODY_RESULTS, &doc, &ViewState::default());
    }

    #[test]
    fn results_window_surfaces_solver_error_without_panicking_3d() {
        let app = Fem3dPlayApp;
        let projection = fem_3d::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let _ = app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
    }
    // #endregion 🔖SolverErrorSurfaced

    // #region 🔖ExampleFixtureRenders
    #[test]
    fn example_fixture_renders_2d() {
        let app = Fem2dPlayApp;
        let json_fixture = include_str!("../../2d/example/default.fem2d.json");
        let projection: fem_2d::Fem2dDocument = serde_json::from_str(json_fixture).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let _ = app.render(FEM2D_BODY_MODEL, &doc, &ViewState::default());
        let _ = app.render(FEM2D_BODY_RESULTS, &doc, &ViewState::default());
    }

    #[test]
    fn example_fixture_renders_3d() {
        let app = Fem3dPlayApp;
        let json_fixture = include_str!("../../3d/example/default.fem3d.json");
        let projection: fem_3d::Fem3dDocument = serde_json::from_str(json_fixture).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let _ = app.render(FEM3D_BODY_MODEL, &doc, &ViewState::default());
        let _ = app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
    }
    // #endregion 🔖ExampleFixtureRenders
}
// #endregion 🔖Tests
