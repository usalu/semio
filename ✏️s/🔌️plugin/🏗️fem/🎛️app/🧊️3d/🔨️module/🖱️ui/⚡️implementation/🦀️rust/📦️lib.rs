//! 🖼️ FEM 3D app — `DocumentApp` impl, render, manifest (constitutional: ui).

use fem3d::{Fem3dDocument, FemCamera};
use fem_core::Dof;
use fem_shared::{hex_to_rgb01, next_id, normalize_mode_shape, parse_result_display, result_display_action_args, von_mises_color, DisplayMode, ResultDisplay, MODE_SHAPE_AMPLITUDE_RATIO};
use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, ui_stack_vertical, ui_text, world3d_default_camera, world3d_default_selection_json, world3d_meshes_json_from_kinds, world3d_scene,
    ActionArgDef, ActionArgOption, ActionEmit, App, AppLabelsOverlay, DocumentApp, DocumentView, SurfaceKind, UiNode, ViewState, WorldSunConfig,
};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use store::DocumentDsl;

//#region 🔖️Constants
const FEM3D_APP_ID: &str = "fem3d-play";
const FEM3D_WINDOW_MODEL: &str = "fem3d-model";
const FEM3D_WINDOW_RESULTS: &str = "fem3d-results";
const FEM3D_BODY_MODEL: &str = "fem3d.play.model";
const FEM3D_BODY_RESULTS: &str = "fem3d.play.results";

/// 📦️ The `fem3d-play` "default" example — shared by the manifest's `.example(...)` registration, the
/// `setActiveExample` handler, and every test fixture. See `fem3d_dsl`'s `🔖️Dsl` region.
const FEM3D_EXAMPLE_DSL: &str = fem3d_dsl::FEM3D_EXAMPLE_TEXT;

/// 🧊️ Half-extent-ish scale of the small box instance drawn at each node.
const NODE_SIZE_3D: f64 = 0.05;
/// 🧊️ Cross-section (x/y) thickness of the oriented box prism drawn for each `Bar`/`Frame` member —
/// a fixed visual thickness, not the member's actual section dimensions (see `fem3d_structural_instances`).
const MEMBER_THICKNESS_3D: f64 = 0.05;
//#endregion 🔖️Constants

//#region 🔖️Fem3dRender
fn find_node_3d<'a>(nodes: &'a [fem3d::FemNode], id: &str) -> Option<&'a fem3d::FemNode> {
    nodes.iter().find(|n| n.id == id)
}

fn fem3d_element_endpoints(element: &fem3d::FemElement) -> (&str, &str) {
    match element {
        fem3d::FemElement::Bar { start, end, .. } | fem3d::FemElement::Frame { start, end, .. } => (start.as_str(), end.as_str()),
    }
}

fn fem3d_element_id(element: &fem3d::FemElement) -> &str {
    match element {
        fem3d::FemElement::Bar { id, .. } | fem3d::FemElement::Frame { id, .. } => id.as_str(),
    }
}

/// 🔎️ 3D counterpart of `fem2d_resolve_load_case` — see its doc.
fn fem3d_resolve_load_case(doc: &Fem3dDocument, case_id: Option<&str>) -> (usize, fem3d::FemLoadCase) {
    let named = case_id.and_then(|id| doc.load_cases.iter().find(|lc| lc.id == id).cloned());
    let load_case = named
        .or_else(|| doc.load_cases.first().cloned())
        .unwrap_or_else(|| fem3d::FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: Vec::new(), self_weight: false });
    let index = doc.load_cases.iter().position(|lc| lc.id == load_case.id).unwrap_or(doc.load_cases.len());
    (index, load_case)
}

/// 📐️ Bounding-box diagonal (in model meters) over every node plus every solid's footprint/height —
/// see `fem2d_model_extent`'s doc for why this drives mode-shape amplitude. Falls back to `1.0` for a
/// degenerate model.
fn fem3d_model_extent(doc: &Fem3dDocument) -> f64 {
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

/// 🧭️ Hamilton quaternion product `a * b`, both `[x,y,z,w]` — applying `b`'s rotation first, then `a`'s.
fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    let (bx, by, bz, bw) = (b[0], b[1], b[2], b[3]);
    [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]
}

/// 🧭️ Rotation of `roll` radians about the LOCAL +Z axis — applied before `quat_z_to` reorients +Z to
/// the member direction, so this spins the box prism about its own long axis (matches `Frame3`'s roll).
fn quat_roll_z(roll: f64) -> [f64; 4] {
    let h = roll / 2.0;
    [0.0, 0.0, h.sin(), h.cos()]
}

/// 🧭️ Shortest-arc rotation taking local `+Z` (the `"box"` mesh's long axis) onto unit direction `dir`
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

/// 🧊️ Node-position resolver shared by every 3D instance/mesh builder: `displacements` (node id -> 6-DOF
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

/// 🧊️ One small box instance per node, plus one ORIENTED box prism per `Bar`/`Frame` member — position
/// at the (possibly deformed) midpoint, `scale=[t,t,length]` so the mesh's own long (local Z) axis
/// stretches along the member, `rotation` a quaternion aligning that axis to the member's direction
/// (composed with a `Frame`'s own `roll` about its own axis; `Bar`s have no roll).
fn fem3d_structural_instances(doc: &Fem3dDocument, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64) -> Vec<Value> {
    let node_pos = |node: &fem3d::FemNode| fem3d_deformed_position([node.x, node.y, node.z], &node.id, displacements, deform_scale);

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
            fem3d::FemElement::Frame { roll, .. } => *roll,
            fem3d::FemElement::Bar { .. } => 0.0,
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

/// 🧱️ Every `FemSolid`'s boundary surface as a custom `meshes_json` entry (flat per-face normals, one
/// duplicated vertex triple per triangle) plus its one identity-transform instance — `nodal_stress`,
/// when present, colors each vertex by `von_mises_color` (min/max taken across ALL solids' averaged
/// values), driving the react renderer's vertex-color contour (see `PaintTexturedMesh`). `displacements`
/// deforms vertex positions the same way `fem3d_structural_instances` deforms node/member instances.
fn fem3d_solid_mesh_entries(doc: &Fem3dDocument, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64, nodal_stress: Option<&HashMap<String, f64>>) -> (Vec<Value>, Vec<Value>) {
    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    let Ok(solid_meshes) = fem3d_engine::fem3d_mesh_preview(doc) else { return (meshes, instances) };
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

/// 🧊️ Builds the FULL `(meshes_json, instances_json)` pair for a 3D scene: the `"box"` primitive mesh
/// plus every `FemSolid`'s custom surface mesh, and every node/member/solid instance — shared by the
/// model window and every results view (static/modal/buckling).
fn fem3d_scene_parts(doc: &Fem3dDocument, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64, nodal_stress: Option<&HashMap<String, f64>>) -> (String, String) {
    let mut meshes: Vec<Value> = serde_json::from_str(&world3d_meshes_json_from_kinds(&["box".to_string()])).unwrap_or_default();
    let mut instances = fem3d_structural_instances(doc, displacements, deform_scale);
    let (solid_meshes, solid_instances) = fem3d_solid_mesh_entries(doc, displacements, deform_scale, nodal_stress);
    meshes.extend(solid_meshes);
    instances.extend(solid_instances);
    (serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()))
}

fn fem3d_camera_json(camera: &FemCamera) -> String {
    if camera.json == "{}" {
        world3d_default_camera()
    } else {
        camera.json.clone()
    }
}

/// 🏷️ Wraps a `World3d` scene node with a text caption above it — `World3dScene` itself has no text
/// field, so a vertical `UiNode` stack (already how the shell composes surfaces) is the idiomatic way
/// to show a frequency/load-factor/case caption in-scene, mirroring the 2D results window's caption layer.
fn with_caption(scene: UiNode, caption: String) -> UiNode {
    ui_stack_vertical(vec![ui_text(caption), scene])
}

fn render_fem3d_model(doc: &Fem3dDocument, camera: &FemCamera) -> UiNode {
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, None, doc.analysis.deformation_scale, None);
    build_world_3d_scene(
        FEM3D_BODY_MODEL,
        FEM3D_APP_ID,
        world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()),
    )
}

/// 📊️ Results window dispatcher — picks the static/modal/buckling render based on `display`.
fn render_fem3d_results(doc: &Fem3dDocument, display: &ResultDisplay, camera: &FemCamera) -> UiNode {
    match display.mode {
        DisplayMode::Static => render_fem3d_results_static(doc, display.source_id.as_deref(), camera),
        DisplayMode::Modal(mode_index) => render_fem3d_results_modal(doc, mode_index, camera),
        DisplayMode::Buckling(mode_index) => render_fem3d_results_buckling(doc, display.source_id.as_deref(), mode_index, camera),
    }
}

/// 📊️ Static results: solved fresh on every render (see `Fem3dPlayApp` doc comment) — same node/member/
/// solid instances as the model window, offset by the solved displacements, solids additionally colored
/// by nodal-averaged von Mises stress. `source_id` selects a `fem3d_solve_all` case/combination id,
/// falling back to the first load case when `None`/unknown. Caption names the active case.
fn render_fem3d_results_static(doc: &Fem3dDocument, source_id: Option<&str>, camera: &FemCamera) -> UiNode {
    let results = match fem3d_engine::fem3d_solve_all(doc) {
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
    let nodal_stress = fem3d_engine::fem3d_nodal_von_mises(doc, &case_id).ok();
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), doc.analysis.deformation_scale, nodal_stress.as_ref());
    let scene = build_world_3d_scene(
        FEM3D_BODY_RESULTS,
        FEM3D_APP_ID,
        world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()),
    );
    with_caption(scene, format!("Case: {case_id}"))
}

/// 📊️ Modal mode-shape overlay: instances offset by the selected mode's shape, normalized to unit peak
/// then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent (see `normalize_mode_shape`),
/// with a frequency caption.
fn render_fem3d_results_modal(doc: &Fem3dDocument, mode_index: usize, camera: &FemCamera) -> UiNode {
    let (freq_hz, mut disp_map) = match fem3d_engine::fem3d_modal_mode_values(doc, mode_index) {
        Ok(values) => values,
        Err(e) => return ui_text(format!("Modal analysis error: {e}")),
    };
    normalize_mode_shape(&mut disp_map);
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), fem3d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO, None);
    let scene = build_world_3d_scene(
        FEM3D_BODY_RESULTS,
        FEM3D_APP_ID,
        world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()),
    );
    with_caption(scene, format!("Mode {}: {freq_hz:.3} Hz", mode_index + 1))
}

/// 📊️ Buckling mode-shape overlay: instances offset by the selected mode's shape, normalized to unit
/// peak then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent (see
/// `normalize_mode_shape`). `source_id` selects the reference load case, falling back to the first
/// load case when `None`. Caption names the mode and its load factor.
fn render_fem3d_results_buckling(doc: &Fem3dDocument, source_id: Option<&str>, mode_index: usize, camera: &FemCamera) -> UiNode {
    let Some(case_id) = source_id.map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone())) else {
        return ui_text("No load case defined");
    };
    let (factor, mut disp_map) = match fem3d_engine::fem3d_buckling_mode_values(doc, &case_id, mode_index) {
        Ok(values) => values,
        Err(e) => return ui_text(format!("Buckling analysis error: {e}")),
    };
    normalize_mode_shape(&mut disp_map);
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), fem3d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO, None);
    let scene = build_world_3d_scene(
        FEM3D_BODY_RESULTS,
        FEM3D_APP_ID,
        world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()),
    );
    with_caption(scene, format!("Buckling mode {}: factor {factor:.3}", mode_index + 1))
}
//#endregion 🔖️Fem3dRender

//#region 🔖️Fem3dPlayApp
/// 🧮️ v0 design: mirrors `Fem2dPlayApp` — results are recomputed fresh inside `render()`, no cache, no
/// `RunAnalysis` operation. `result_display` is ephemeral view state (see `fem_shared::ResultDisplay`'s
/// doc), defaulting to the first load case in `Static` mode.
pub struct Fem3dPlayApp {
    result_display: RefCell<ResultDisplay>,
    camera: RefCell<FemCamera>,
}

impl Default for Fem3dPlayApp {
    fn default() -> Self {
        Self {
            result_display: RefCell::new(ResultDisplay::default()),
            camera: RefCell::new(FemCamera::default()),
        }
    }
}

impl DocumentApp for Fem3dPlayApp {
    type Projection = Fem3dDocument;
    type Operation = fem3d_op::Fem3dOperation;
        type Config = semio_framework_plugin::NoConfig;
        type ConfigOperation = semio_framework_plugin::NoConfigOperation;

    fn app_id(&self) -> &str {
        FEM3D_APP_ID
    }

    fn document_schema(&self) -> &str {
        fem3d::FEM_3D_SCHEMA
    }

    fn initial_projection(&self) -> Fem3dDocument {
        fem3d_engine::empty_fem3d_projection()
    }

    fn handle_action(
        &self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, Fem3dDocument>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        _view_state: &ViewState,
    ) -> ActionEmit<fem3d_op::Fem3dOperation> {
        match action {
            "addNode" => {
                if let (Some(x), Some(y), Some(z)) = (
                    args.and_then(|v| v.get("x")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("y")).and_then(Value::as_f64),
                    args.and_then(|v| v.get("z")).and_then(Value::as_f64),
                ) {
                    let id = next_id(doc.projection.nodes.iter().map(|n| n.id.clone()), "n");
                    let index = doc.projection.nodes.len();
                    return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetNode { index, node: fem3d::FemNode { id, x, y, z } }]);
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
                    let element = fem3d::FemElement::Bar { id, start: start.into(), end: end.into(), material_id: material_id.into(), section_id: section_id.into() };
                    return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetElement { index, element: Box::new(element) }]);
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
                    let element = fem3d::FemElement::Frame { id, start: start.into(), end: end.into(), material_id: material_id.into(), section_id: section_id.into(), roll };
                    return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetElement { index, element: Box::new(element) }]);
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
                    return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetMaterial { index, material: fem3d::FemMaterial { id, name: name.into(), e, g, nu: 0.3, rho: 7850.0 } }]);
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
                    return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetSection { index, section: fem3d::FemSection { id, name: name.into(), area, iy, iz, j } }]);
                }
            }
            "addSupport" => {
                if let Some(node_id) = args.and_then(|v| v.get("nodeId")).and_then(Value::as_str) {
                    let fixed: Vec<Dof> = args.and_then(|v| v.get("fixed")).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                    let fixed: Vec<fem3d::FemDof> = fixed.into_iter().map(Into::into).collect();
                    let id = next_id(doc.projection.supports.iter().map(|s| s.id.clone()), "sup");
                    let index = doc.projection.supports.len();
                    return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetSupport { index, support: fem3d::FemSupport { id, node_id: node_id.into(), fixed } }]);
                }
            }
            "addNodalLoad" => {
                if let (Some(node_id), Some(dof), Some(value)) = (
                    args.and_then(|v| v.get("nodeId")).and_then(Value::as_str),
                    args.and_then(|v| v.get("dof")).and_then(|v| serde_json::from_value::<fem3d::FemDof>(v.clone()).ok()),
                    args.and_then(|v| v.get("value")).and_then(Value::as_f64),
                ) {
                    let case_id = args.and_then(|v| v.get("caseId")).and_then(Value::as_str);
                    let (index, mut load_case) = fem3d_resolve_load_case(doc.projection, case_id);
                    let load_id = next_id(load_case.loads.iter().map(|l| fem3d::load_id(l).to_string()), "l");
                    load_case.loads.push(fem3d::FemLoad::Nodal { id: load_id, node_id: node_id.into(), dof, value });
                    return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetLoadCase { index, load_case }]);
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
                    let load_id = next_id(load_case.loads.iter().map(|l| fem3d::load_id(l).to_string()), "l");
                    load_case.loads.push(fem3d::FemLoad::MemberUdl { id: load_id, element_id: element_id.into(), wx, wy, wz });
                    return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetLoadCase { index, load_case }]);
                }
            }
            "addAreaLoad" => {
                if let (Some(solid_id), Some(pressure)) = (args.and_then(|v| v.get("solidId")).and_then(Value::as_str), args.and_then(|v| v.get("pressure")).and_then(Value::as_f64)) {
                    let case_id = args.and_then(|v| v.get("caseId")).and_then(Value::as_str);
                    let (index, mut load_case) = fem3d_resolve_load_case(doc.projection, case_id);
                    let load_id = next_id(load_case.loads.iter().map(|l| fem3d::load_id(l).to_string()), "l");
                    load_case.loads.push(fem3d::FemLoad::Area { id: load_id, solid_id: solid_id.into(), pressure });
                    return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetLoadCase { index, load_case }]);
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
                    return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetSolid { index, solid: fem3d::FemSolid { id, name: "Solid".into(), outline, holes: Vec::new(), base_z, height, layers, mesh_size, material_id: material_id.into() } }]);
                }
            }
            "addLoadCase" => {
                if let Some(name) = args.and_then(|v| v.get("name")).and_then(Value::as_str) {
                    let self_weight = args.and_then(|v| v.get("selfWeight")).and_then(Value::as_bool).unwrap_or(false);
                    let id = next_id(doc.projection.load_cases.iter().map(|lc| lc.id.clone()), "case-");
                    let index = doc.projection.load_cases.len();
                    return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetLoadCase { index, load_case: fem3d::FemLoadCase { id, name: name.into(), loads: Vec::new(), self_weight } }]);
                }
            }
            "addCombination" => {
                if let (Some(name), Some(terms_json)) = (args.and_then(|v| v.get("name")).and_then(Value::as_str), args.and_then(|v| v.get("terms")).and_then(Value::as_str)) {
                    if let Ok(terms) = serde_json::from_str::<Vec<(String, f64)>>(terms_json) {
                        let terms: std::collections::BTreeMap<String, f64> = terms.into_iter().collect();
                        let id = next_id(doc.projection.combinations.iter().map(|c| c.id.clone()), "c");
                        let index = doc.projection.combinations.len();
                        return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetCombination { index, combination: fem3d::FemCombination { id, name: name.into(), terms } }]);
                    }
                }
            }
            "setSelfWeight" => {
                if let (Some(case_id), Some(enabled)) = (args.and_then(|v| v.get("caseId")).and_then(Value::as_str), args.and_then(|v| v.get("enabled")).and_then(Value::as_bool)) {
                    if let Some(index) = doc.projection.load_cases.iter().position(|lc| lc.id == case_id) {
                        let mut load_case = doc.projection.load_cases[index].clone();
                        load_case.self_weight = enabled;
                        return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetLoadCase { index, load_case }]);
                    }
                }
            }
            "setAnalysisSettings" => {
                let current = &doc.projection.analysis;
                let modal_count = args.and_then(|v| v.get("modalCount")).and_then(Value::as_u64).map(|n| n as usize).unwrap_or(current.modal_count);
                let buckling_count = args.and_then(|v| v.get("bucklingCount")).and_then(Value::as_u64).map(|n| n as usize).unwrap_or(current.buckling_count);
                let deformation_scale = args.and_then(|v| v.get("deformationScale")).and_then(Value::as_f64).unwrap_or(current.deformation_scale);
                return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetAnalysisSettings { settings: fem3d::FemAnalysisSettings { modal_count, buckling_count, deformation_scale } }]);
            }
            "removeSelection" => {
                let ids: Vec<String> = args.and_then(|v| v.get("ids")).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                let mut operations = Vec::new();
                for id in ids {
                    if doc.projection.nodes.iter().any(|n| n.id == id) {
                        operations.push(fem3d_op::Fem3dOperation::RemoveNode { id });
                    } else if doc.projection.elements.iter().any(|e| fem3d_element_id(e) == id) {
                        operations.push(fem3d_op::Fem3dOperation::RemoveElement { id });
                    } else if doc.projection.materials.iter().any(|m| m.id == id) {
                        operations.push(fem3d_op::Fem3dOperation::RemoveMaterial { id });
                    } else if doc.projection.sections.iter().any(|s| s.id == id) {
                        operations.push(fem3d_op::Fem3dOperation::RemoveSection { id });
                    } else if doc.projection.supports.iter().any(|s| s.id == id) {
                        operations.push(fem3d_op::Fem3dOperation::RemoveSupport { id });
                    } else if doc.projection.load_cases.iter().any(|l| l.id == id) {
                        operations.push(fem3d_op::Fem3dOperation::RemoveLoadCase { id });
                    } else if doc.projection.solids.iter().any(|s| s.id == id) {
                        operations.push(fem3d_op::Fem3dOperation::RemoveSolid { id });
                    } else if doc.projection.combinations.iter().any(|c| c.id == id) {
                        operations.push(fem3d_op::Fem3dOperation::RemoveCombination { id });
                    }
                }
                if !operations.is_empty() {
                    return ActionEmit::operations(operations);
                }
            }
            "setCamera" => {
                if let Some(json_str) = args.and_then(|v| v.get("json")).and_then(Value::as_str) {
                    *self.camera.borrow_mut() = FemCamera { json: json_str.into() };
                }
            }
            "setResultDisplay" => {
                *self.result_display.borrow_mut() = parse_result_display(args);
            }
            "setActiveExample" => {
                let example_id = args.and_then(|v| v.get("exampleId")).and_then(Value::as_str).unwrap_or("");
                let document = if example_id == "default" {
                    Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap_or_else(|_| fem3d_engine::empty_fem3d_projection())
                } else {
                    fem3d_engine::empty_fem3d_projection()
                };
                *self.result_display.borrow_mut() = ResultDisplay::default();
                *self.camera.borrow_mut() = FemCamera::default();
                return ActionEmit::operations(vec![fem3d_op::Fem3dOperation::SetDocument { document }]);
            }
            _ => {}
        }
        ActionEmit::default()
    }

    fn render(
        &self,
        body_key: &str,
        doc: &DocumentView<'_, Fem3dDocument>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        _view_state: &ViewState,
    ) -> UiNode {
        match body_key {
            FEM3D_BODY_MODEL => render_fem3d_model(doc.projection, &*self.camera.borrow()),
            FEM3D_BODY_RESULTS => render_fem3d_results(doc.projection, &*self.result_display.borrow(), &*self.camera.borrow()),
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
            tutorial_labels: HashMap::new(),
            group_labels: HashMap::new(),
        }
    }
}
//#endregion 🔖️Fem3dPlayApp

//#region 🔖️Fem3dTerminology
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
//#endregion 🔖️Fem3dTerminology

//#region 🔖️Manifest
pub fn create_fem3d_app() -> App {
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
            .view_action("setCamera", "Set Camera")
            .operation("setActiveExample", "Set Active Example")
            .action_args("setActiveExample", vec![ActionArgDef::select("exampleId", "Example", vec![ActionArgOption::new("default", "Default")]).default_value("default")])
            .view_action("setResultDisplay", "Set Result Display")
            .action_args("setResultDisplay", result_display_action_args()),
    )
    .example("default", "Family House", FEM3D_EXAMPLE_DSL)
    .workflow("fem3d", "FEM 3D", "structure")
}
//#endregion 🔖️Manifest

// #region 🔖️WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use fem3d_op::{Fem3dEnvelope, Fem3dStore};
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Fem3dDocumentVcs {
        store: RefCell<Fem3dStore>,
    }

    #[wasm_bindgen]
    impl Fem3dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Fem3dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Fem3dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Fem3dStore::new(envelope)
                }
                None => Fem3dStore::new(create_document_envelope(fem3d::FEM_3D_SCHEMA, "fem3d", fem3d_engine::empty_fem3d_projection(), None)),
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
    use semio_framework_plugin::HistoryView;

    fn history_view() -> HistoryView {
        HistoryView::empty()
    }

    //#region 🔖️RendersScenes
    #[test]
    fn renders_fem3d_model_scene() {
        let app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM3D_BODY_MODEL, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn renders_fem3d_results_scene() {
        let app = Fem3dPlayApp::default();
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }
    //#endregion 🔖️RendersScenes

    //#region 🔖️AddNodeAction
    #[test]
    fn add_node_action_emits_op_3d() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "x": 1.0, "y": 2.0, "z": 3.0 });
        let emit = app.handle_action("addNode", Some(&args), &doc, &ViewState::default());
        assert_eq!(emit.operations.len(), 1);
        match &emit.operations[0] {
            fem3d_op::Fem3dOperation::SetNode { node, .. } => {
                assert_eq!(node.x, 1.0);
                assert_eq!(node.y, 2.0);
                assert_eq!(node.z, 3.0);
            }
            _ => panic!("expected SetNode"),
        }
    }
    //#endregion 🔖️AddNodeAction

    //#region 🔖️SolverErrorSurfaced
    #[test]
    fn results_window_surfaces_solver_error_without_panicking_3d() {
        let app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let _ = app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
    }
    //#endregion 🔖️SolverErrorSurfaced

    //#region 🔖️ExampleFixtureRenders
    #[test]
    fn example_fixture_renders_3d() {
        let app = Fem3dPlayApp::default();
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let _ = app.render(FEM3D_BODY_MODEL, &doc, &ViewState::default());
        let _ = app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
    }
    //#endregion 🔖️ExampleFixtureRenders

    //#region 🔖️SetActiveExample
    #[test]
    fn set_active_example_loads_default_fixture_3d() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("setActiveExample", Some(&json!({ "exampleId": "default" })), &doc, &ViewState::default());
        assert_eq!(emit.operations.len(), 1);
        match &emit.operations[0] {
            fem3d_op::Fem3dOperation::SetDocument { document } => assert!(!document.nodes.is_empty(), "expected the default fixture's nodes"),
            _ => panic!("expected SetDocument"),
        }
    }

    /// 🧬️ `setActiveExample` replaces document content via `SetDocument` operations, so it MUST be declared as
    /// an Operation, not a View/Shell action — the framework's "View/Shell actions must not emit
    /// operations" guard would otherwise reject it.
    #[test]
    fn set_active_example_is_declared_as_operation_3d() {
        use semio_framework_plugin::ActionKind;
        let definition = create_fem3d_app().definition;
        let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, ActionKind::Operation), "loading an example emits SetDocument operations, so it is an Operation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
    }
    //#endregion 🔖️SetActiveExample

    //#region 🔖️ModeShapeRender
    #[test]
    fn results_window_renders_modal_mode_shape_3d() {
        let app = Fem3dPlayApp { result_display: ResultDisplay { source_id: None, mode: DisplayMode::Modal(0) }, camera: FemCamera::default() };
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"), "expected a valid world-3d scene, got: {json}");
        assert!(!json.contains("Modal analysis error"), "unexpected modal error: {json}");
    }

    #[test]
    fn results_window_renders_buckling_mode_shape_3d() {
        let app = Fem3dPlayApp { result_display: ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Buckling(0) }, camera: FemCamera::default() };
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"), "expected a valid world-3d scene, got: {json}");
        assert!(!json.contains("Buckling analysis error"), "unexpected buckling error: {json}");
    }
    //#endregion 🔖️ModeShapeRender

    //#region 🔖️SolidRenderAndCaptions
    #[test]
    fn model_scene_renders_solid_mesh_and_oriented_member_instances_3d() {
        let app = Fem3dPlayApp::default();
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
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
        let app = Fem3dPlayApp { result_display: ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Static }, camera: FemCamera::default() };
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
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
        let app_modal = Fem3dPlayApp { result_display: ResultDisplay { source_id: None, mode: DisplayMode::Modal(0) }, camera: FemCamera::default() };
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let node_modal = app_modal.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
        let json_modal = serde_json::to_string(&node_modal).unwrap();
        assert!(json_modal.contains("Hz"), "expected a frequency caption: {json_modal}");

        let app_buckling = Fem3dPlayApp { result_display: ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Buckling(0) }, camera: FemCamera::default() };
        let node_buckling = app_buckling.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default());
        let json_buckling = serde_json::to_string(&node_buckling).unwrap();
        assert!(json_buckling.contains("factor"), "expected a load-factor caption: {json_buckling}");
    }
    //#endregion 🔖️SolidRenderAndCaptions

    //#region 🔖️StructureActions
    #[test]
    fn add_solid_action_emits_set_solid_3d() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "x": 0.0, "y": 0.0, "width": 2.0, "depth": 1.0, "height": 0.5, "materialId": "concrete" });
        let emit = app.handle_action("addSolid", Some(&args), &doc, &ViewState::default());
        assert_eq!(emit.operations.len(), 1);
        match &emit.operations[0] {
            fem3d_op::Fem3dOperation::SetSolid { solid, .. } => {
                assert_eq!(solid.outline, vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
                assert_eq!(solid.height, 0.5);
                assert_eq!(solid.layers, 1);
            }
            _ => panic!("expected SetSolid"),
        }
    }

    #[test]
    fn remove_selection_covers_solids_3d() {
        let mut app = Fem3dPlayApp::default();
        let mut projection = fem3d_engine::empty_fem3d_projection();
        projection.solids.push(fem3d::FemSolid { id: "sol1".into(), name: "S".into(), outline: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], holes: vec![], base_z: 0.0, height: 1.0, layers: 1, mesh_size: 0.5, material_id: "concrete".into() });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("removeSelection", Some(&json!({ "ids": ["sol1"] })), &doc, &ViewState::default());
        assert_eq!(emit.operations.len(), 1);
        assert!(matches!(emit.operations[0], fem3d_op::Fem3dOperation::RemoveSolid { .. }));
    }
    //#endregion 🔖️StructureActions

    //#region 🔖️LoadCaseActions
    #[test]
    fn add_member_udl_action_emits_op_3d() {
        let mut app = Fem3dPlayApp::default();
        let mut projection = fem3d_engine::empty_fem3d_projection();
        projection.load_cases.push(fem3d::FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "elementId": "e1", "wx": 0.0, "wy": 0.0, "wz": -2000.0 });
        let emit = app.handle_action("addMemberUdl", Some(&args), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem3d_op::Fem3dOperation::SetLoadCase { load_case, .. } => assert!(matches!(load_case.loads[0], fem3d::FemLoad::MemberUdl { .. })),
            _ => panic!("expected SetLoadCase"),
        }
    }
    //#endregion 🔖️LoadCaseActions

    //#region 🔖️SharedHelpers
    #[test]
    fn quat_z_to_identity_for_parallel_direction() {
        assert_eq!(quat_z_to([0.0, 0.0, 1.0]), [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn quat_z_to_handles_antiparallel_direction() {
        assert_eq!(quat_z_to([0.0, 0.0, -1.0]), [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn fem3d_resolve_load_case_synthesizes_case_when_none_exist() {
        let projection = fem3d_engine::empty_fem3d_projection();
        let (index, load_case) = fem3d_resolve_load_case(&projection, None);
        assert_eq!(index, 0);
        assert_eq!(load_case.id, "case-1");
    }

    #[test]
    fn fem3d_model_extent_degenerate_model_returns_one() {
        assert_eq!(fem3d_model_extent(&fem3d_engine::empty_fem3d_projection()), 1.0);
    }
    //#endregion 🔖️SharedHelpers

    //#region 🔖️UnknownBodyAndGermanLabels
    #[test]
    fn render_unknown_body_key_returns_placeholder_text_3d() {
        let app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let json = serde_json::to_string(&app.render("nonsense", &doc, &ViewState::default())).unwrap();
        assert!(json.contains("Unknown body: nonsense"));
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
    fn results_window_buckling_with_no_load_case_shows_placeholder_3d() {
        let app = Fem3dPlayApp { result_display: ResultDisplay { source_id: None, mode: DisplayMode::Buckling(0) }, camera: FemCamera::default() };
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let json = serde_json::to_string(&app.render(FEM3D_BODY_RESULTS, &doc, &ViewState::default())).unwrap();
        assert!(json.contains("No load case defined"), "{json}");
    }
    //#endregion 🔖️UnknownBodyAndGermanLabels

    //#region 🔖️MoreStructureAndLoadActions
    #[test]
    fn add_material_action_emits_op_3d() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "name": "Steel", "e": 2.1e11, "g": 8.1e10 });
        let emit = app.handle_action("addMaterial", Some(&args), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem3d_op::Fem3dOperation::SetMaterial { material, .. } => assert_eq!(material.g, 8.1e10),
            _ => panic!("expected SetMaterial"),
        }
    }

    #[test]
    fn add_section_action_emits_op_3d() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "name": "HEA200", "area": 0.00538, "iy": 0.0000369, "iz": 0.0000133, "j": 0.0000006 });
        let emit = app.handle_action("addSection", Some(&args), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem3d_op::Fem3dOperation::SetSection { section, .. } => assert_eq!(section.j, 0.0000006),
            _ => panic!("expected SetSection"),
        }
    }

    #[test]
    fn add_frame_action_emits_op_3d() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let args = json!({ "start": "n1", "end": "n2", "materialId": "m1", "sectionId": "s1", "roll": 0.5 });
        let emit = app.handle_action("addFrame", Some(&args), &doc, &ViewState::default());
        match &emit.operations[0] {
            fem3d_op::Fem3dOperation::SetElement { element, .. } => match element.as_ref() {
                fem3d::FemElement::Frame { roll, .. } => assert_eq!(*roll, 0.5),
                _ => panic!("expected Frame"),
            },
            _ => panic!("expected SetElement"),
        }
    }

    #[test]
    fn set_camera_action_writes_runtime_not_operations() {
        let mut app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("setCamera", Some(&json!({ "json": "{\"x\":1}" })), &doc, &ViewState::default());
        assert!(emit.operations.is_empty(), "setCamera must not emit a VCS operation");
        assert_eq!(app.camera.json, "{\"x\":1}");
    }
    //#endregion 🔖️MoreStructureAndLoadActions
}
//#endregion 🧪️Tests
