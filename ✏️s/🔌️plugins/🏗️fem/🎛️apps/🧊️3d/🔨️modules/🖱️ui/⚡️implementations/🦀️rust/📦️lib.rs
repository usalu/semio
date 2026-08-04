//! 🖼️ FEM 3D app — `DocumentApp` impl, render, manifest (constitutional: ui). B1: brings this app up
//! from the pre-B1 stringly-typed `{action,args}` `handle_action`/`ViewState`-carrying `render`/deleted
//! `AppLabelsOverlay` shape to the same pure-trait design `fem2d_ui::Fem2dPlayApp` already pioneers —
//! every former `Fem3dPlayApp` `RefCell` field (`result_display`, `camera`) now lives in
//! `fem3d_engine::Fem3dConfig`, written via `fem3d_op::Fem3dConfigOperation`s (real `backwards`, no ad
//! hoc `InverseAction`); every action dispatches through the single typed `fem3d_protocol::Fem3dCommand`
//! channel via `DocumentApp::handle` — this crate now mirrors `fem2d`'s `_engine`-config/
//! `_op`-config-operation/`_protocol`-command crate split exactly, plus the `geometry:in`/`results:out`
//! typed media ports (see `🔖️Fem3dPlayApp`'s `io`/`export_media`/`import_media` overrides).
//!
//! Also migrates every manifest/UI label to the compile-time-checked `Label`/`LocalizedLabel` types:
//! static manifest text (window/mode/action/example names) is now declared once via
//! `LocalizedLabel::native(en, de)`/`::data(...)` and resolved by the shell per active
//! locale/terminology; the old runtime `is_de`-branching `Fem3dLabels`/`fem3d_action_labels` table and
//! the `app_labels()` trait override it fed are both gone — there is no OTHER runtime consumer of
//! locale in this app (every render-time caption below was always English regardless of locale even
//! before this migration), so no local `Locale`/`Terminology` shim is needed here at all.

use fem3d::{Fem3dDocument, FemCamera};
use fem3d_engine::Fem3dConfig;
use fem3d_op::{Fem3dConfigOperation, Fem3dOperation};
use fem3d_protocol::Fem3dCommand;
use fem_core::{Dof, ElementResult};
use fem_shared::{hex_to_rgb01, next_id, normalize_mode_shape, result_display_action_args, von_mises_color, DisplayMode, ResultDisplay, MODE_SHAPE_AMPLITUDE_RATIO};
use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, ui_stack_vertical, ui_text, world3d_default_camera, world3d_default_selection_json, world3d_meshes_json_from_kinds, world3d_scene, ActionArgDef, ActionArgOption, App, AppIo, ArtifactKindSpec,
    ConfigSpec, ConfigView, DocumentApp, DocumentView, Emit, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, SurfaceKind, UiNode, WorldSunConfig,
};
use serde_json::{json, Value};
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
    let load_case = named.or_else(|| doc.load_cases.first().cloned()).unwrap_or_else(|| fem3d::FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: Vec::new(), self_weight: false });
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
/// `caption` is genuine runtime data (a case id, mode index, frequency, …), so it is wrapped via
/// `Label::data` rather than any `LocalizedLabel` — see this file's header doc.
fn with_caption(scene: UiNode, caption: String) -> UiNode {
    ui_stack_vertical(vec![ui_text(Label::data(caption)), scene])
}

fn render_fem3d_model(doc: &Fem3dDocument, camera: &FemCamera) -> UiNode {
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, None, doc.analysis.deformation_scale, None);
    build_world_3d_scene(FEM3D_BODY_MODEL, FEM3D_APP_ID, world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()))
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
        Err(e) => return ui_text(Label::data(format!("Analysis error: {e}"))),
    };
    let case_id = source_id.filter(|id| results.contains_key(*id)).map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone()));
    let Some(case_id) = case_id else {
        return ui_text(Label::data("No load case defined"));
    };
    let Some(result) = results.get(&case_id) else {
        return ui_text(Label::data(format!("Result not found: {case_id}")));
    };
    let mut disp_map: HashMap<String, [f64; 6]> = HashMap::new();
    for d in &result.displacements {
        disp_map.insert(d.node_id.clone(), d.values);
    }
    let nodal_stress = fem3d_engine::fem3d_nodal_von_mises(doc, &case_id).ok();
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), doc.analysis.deformation_scale, nodal_stress.as_ref());
    let scene = build_world_3d_scene(FEM3D_BODY_RESULTS, FEM3D_APP_ID, world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()));
    with_caption(scene, format!("Case: {case_id}"))
}

/// 📊️ Modal mode-shape overlay: instances offset by the selected mode's shape, normalized to unit peak
/// then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent (see `normalize_mode_shape`),
/// with a frequency caption.
fn render_fem3d_results_modal(doc: &Fem3dDocument, mode_index: usize, camera: &FemCamera) -> UiNode {
    let (freq_hz, mut disp_map) = match fem3d_engine::fem3d_modal_mode_values(doc, mode_index) {
        Ok(values) => values,
        Err(e) => return ui_text(Label::data(format!("Modal analysis error: {e}"))),
    };
    normalize_mode_shape(&mut disp_map);
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), fem3d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO, None);
    let scene = build_world_3d_scene(FEM3D_BODY_RESULTS, FEM3D_APP_ID, world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()));
    with_caption(scene, format!("Mode {}: {freq_hz:.3} Hz", mode_index + 1))
}

/// 📊️ Buckling mode-shape overlay: instances offset by the selected mode's shape, normalized to unit
/// peak then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent (see
/// `normalize_mode_shape`). `source_id` selects the reference load case, falling back to the first
/// load case when `None`. Caption names the mode and its load factor.
fn render_fem3d_results_buckling(doc: &Fem3dDocument, source_id: Option<&str>, mode_index: usize, camera: &FemCamera) -> UiNode {
    let Some(case_id) = source_id.map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone())) else {
        return ui_text(Label::data("No load case defined"));
    };
    let (factor, mut disp_map) = match fem3d_engine::fem3d_buckling_mode_values(doc, &case_id, mode_index) {
        Ok(values) => values,
        Err(e) => return ui_text(Label::data(format!("Buckling analysis error: {e}"))),
    };
    normalize_mode_shape(&mut disp_map);
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), fem3d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO, None);
    let scene = build_world_3d_scene(FEM3D_BODY_RESULTS, FEM3D_APP_ID, world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()));
    with_caption(scene, format!("Buckling mode {}: factor {factor:.3}", mode_index + 1))
}
//#endregion 🔖️Fem3dRender

//#region 🔖️Fem3dConfigHelpers
/// 👁️ B1: `cfg`-driven counterpart of the deleted `ResultDisplay` `RefCell` — converts the flat
/// `Fem3dConfig` result-display fields back into `fem_shared::ResultDisplay`/`DisplayMode` so the
/// existing `render_fem3d_results` pipeline (built around those shared types) needs no changes.
fn config_result_display(cfg: &Fem3dConfig) -> ResultDisplay {
    let mode = match cfg.result_mode.as_str() {
        "modal" => DisplayMode::Modal(cfg.result_mode_index as usize),
        "buckling" => DisplayMode::Buckling(cfg.result_mode_index as usize),
        _ => DisplayMode::Static,
    };
    ResultDisplay { source_id: cfg.result_source_id.clone(), mode }
}
//#endregion 🔖️Fem3dConfigHelpers

//#region 🔖️Fem3dResultsJson
/// 🎨️ Manual `fem_core::StaticResult` -> JSON bridge for `"results:out"` (see
/// `Fem3dPlayApp::export_media`) — `fem_core::StaticResult`/`ElementResult`/`Dof` don't derive
/// `Serialize` (out of this ticket's scope: `🫀️core` is a shared crate), so this hand-rolls the same
/// shape `serde_json::to_string` would have produced, using `Dof`'s existing `{:?}` formatting (already
/// used for the reaction-label layers in `fem2d_ui`'s equivalent). Mirrors `fem2d_ui`'s identical
/// `results:out` JSON encoding one dimension up.
fn fem3d_dof_json(dof: Dof) -> Value {
    json!(format!("{dof:?}"))
}

fn fem3d_element_result_json(result: &ElementResult) -> Value {
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

fn fem3d_static_result_json(result: &fem_core::StaticResult) -> Value {
    json!({
        "displacements": result.displacements.iter().map(|d| json!({ "nodeId": d.node_id, "values": d.values })).collect::<Vec<_>>(),
        "reactions": result.reactions.iter().map(|r| json!({ "nodeId": r.node_id, "dof": fem3d_dof_json(r.dof), "value": r.value })).collect::<Vec<_>>(),
        "elements": result.elements.iter().map(|(id, element_result)| json!({ "id": id, "result": fem3d_element_result_json(element_result) })).collect::<Vec<_>>(),
        "checks": { "residualNorm": result.checks.residual_norm, "reactionSum": result.checks.reaction_sum },
    })
}

fn fem3d_results_map_json(results: &HashMap<String, fem_core::StaticResult>) -> Value {
    Value::Object(results.iter().map(|(id, result)| (id.clone(), fem3d_static_result_json(result))).collect())
}
//#endregion 🔖️Fem3dResultsJson

//#region 🔖️Fem3dPlayApp
/// 🧮️ v0 design: mirrors `Fem2dPlayApp` — results are recomputed fresh inside `render()`, no cache, no
/// `RunAnalysis` operation. B1: unit struct — every former `RefCell` field now lives in `Fem3dConfig`
/// (see `DocumentApp::Config`), written through `Fem3dConfigOperation`s.
#[derive(Default)]
pub struct Fem3dPlayApp;

impl DocumentApp for Fem3dPlayApp {
    type Projection = Fem3dDocument;
    type Operation = Fem3dOperation;
    type Config = Fem3dConfig;
    type ConfigOperation = Fem3dConfigOperation;
    type Command = Fem3dCommand;

    fn app_id(&self) -> &str {
        FEM3D_APP_ID
    }

    fn document_schema(&self) -> &str {
        fem3d::FEM_3D_SCHEMA
    }

    fn initial_projection(&self) -> Fem3dDocument {
        fem3d_engine::empty_fem3d_projection()
    }

    fn io(&self) -> Option<AppIo> {
        Some(fem3d_engine::fem3d_io())
    }

    /// 🎞️ `"document:out"` reproduces the trait's default whole-document pack (overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new
    /// one). `"results:out"` runs every load case/combination's analysis fresh and returns them as plain
    /// JSON text in a `Structured` payload, mirroring `fem2d_ui::Fem2dPlayApp::export_media`'s identical
    /// judgment call one dimension down. A document with no load cases, or a solve failure, is reported
    /// as `MediaError::Payload` rather than an empty/panicking export.
    fn export_media(&self, port: &str, doc: &DocumentView<'_, Fem3dDocument>) -> Result<Media, MediaError> {
        match port {
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = <Fem3dDocument as store::DocumentPack>::encode_pack(doc.projection);
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "results:out" => {
                if doc.projection.load_cases.is_empty() {
                    return Err(MediaError::Payload("results:out".into(), "no load cases defined".into()));
                }
                let results = fem3d_engine::fem3d_solve_all(doc.projection).map_err(|error| MediaError::Payload("results:out".into(), error.to_string()))?;
                let json = fem3d_results_map_json(&results).to_string();
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.fem3d".into(), json } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn whole_document_operation(&self, projection: Fem3dDocument) -> Option<Fem3dOperation> {
        Some(Fem3dOperation::SetDocument { document: projection })
    }

    /// 🎞️ `"document:in"` reproduces the trait's default whole-document-pack importer (overriding
    /// `import_media` shadows it for every port). `"geometry:in"` decodes a minimal, app-owned
    /// `{"outline": [[f64;2]...], "holes": [[[f64;2]...]...], "baseZ"?: f64, "height"?: f64, "layers"?:
    /// usize}` extruded-footprint contract (mirroring `fem2d_ui`'s `geometry:in` polygon-with-holes
    /// contract, extended with the 3D extrusion fields `FemSolid` needs) into a new `FemSolid`, defaulted
    /// to the document's first existing material if any, else an `"unassigned"` placeholder id (mirrors
    /// `fem2d_ui::Fem2dPlayApp::import_media`'s identical judgment call: the solid simply won't solve
    /// until a real material is assigned).
    fn import_media(&self, port: &str, media: &Media, doc: &DocumentView<'_, Fem3dDocument>) -> Result<Emit<Fem3dOperation, Fem3dConfigOperation>, MediaError> {
        match port {
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let projection = <Fem3dDocument as store::DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
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
                let base_z = value.get("baseZ").and_then(Value::as_f64).unwrap_or(0.0);
                let height = value.get("height").and_then(Value::as_f64).unwrap_or(1.0);
                let layers = value.get("layers").and_then(Value::as_u64).map(|v| v as usize).unwrap_or(1);
                let material_id = doc.projection.materials.first().map(|material| material.id.clone()).unwrap_or_else(|| "unassigned".into());
                let id = next_id(doc.projection.solids.iter().map(|s| s.id.clone()), "sol");
                let index = doc.projection.solids.len();
                let solid = fem3d::FemSolid { id, name: "Imported Geometry".into(), outline, holes, base_z, height, layers, mesh_size: 0.5, material_id };
                Ok(Emit::operations(vec![Fem3dOperation::SetSolid { index, solid }]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧮️ No sticky `ActionArgDef` defaults are mirrored here (all of `addSolid`'s
    /// `baseZ`/`layers`/`meshSize` defaults are baked directly into `handle`, not user-configurable
    /// settings), matching `fem2d_ui::Fem2dPlayApp::config_spec`'s identical judgment call.
    fn config_spec(&self) -> ConfigSpec {
        ConfigSpec::empty()
    }

    /// 🏷️ Maps each `Fem3dCommand` variant back to the action id it was declared under in
    /// `create_fem3d_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &Fem3dCommand) -> &str {
        match command {
            Fem3dCommand::AddNode { .. } => "addNode",
            Fem3dCommand::AddBar { .. } => "addBar",
            Fem3dCommand::AddFrame { .. } => "addFrame",
            Fem3dCommand::AddMaterial { .. } => "addMaterial",
            Fem3dCommand::AddSection { .. } => "addSection",
            Fem3dCommand::AddSupport { .. } => "addSupport",
            Fem3dCommand::AddNodalLoad { .. } => "addNodalLoad",
            Fem3dCommand::AddMemberUdl { .. } => "addMemberUdl",
            Fem3dCommand::AddAreaLoad { .. } => "addAreaLoad",
            Fem3dCommand::AddSolid { .. } => "addSolid",
            Fem3dCommand::AddLoadCase { .. } => "addLoadCase",
            Fem3dCommand::AddCombination { .. } => "addCombination",
            Fem3dCommand::SetSelfWeight { .. } => "setSelfWeight",
            Fem3dCommand::SetAnalysisSettings { .. } => "setAnalysisSettings",
            Fem3dCommand::RemoveSelection { .. } => "removeSelection",
            Fem3dCommand::SetActiveExample { .. } => "setActiveExample",
            Fem3dCommand::SetCamera { .. } => "setCamera",
            Fem3dCommand::SetResultDisplay { .. } => "setResultDisplay",
        }
    }

    /// 🧩️ B1: the pure heart of the app — a total, side-effect-free function from
    /// `(command, document, config)` to an `Emit`. Every former `handle_action` match arm keeps working,
    /// just through this typed channel instead of the `{action, args}` JSON channel.
    fn handle(&self, command: &Fem3dCommand, doc: &DocumentView<'_, Fem3dDocument>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dOperation, Fem3dConfigOperation>, Fault> {
        let projection = doc.projection;
        match command {
            Fem3dCommand::AddNode { x, y, z } => {
                let id = next_id(projection.nodes.iter().map(|n| n.id.clone()), "n");
                let index = projection.nodes.len();
                Ok(Emit::operations(vec![Fem3dOperation::SetNode { index, node: fem3d::FemNode { id, x: *x, y: *y, z: *z } }])
            }
            Fem3dCommand::AddBar { start, end, material_id, section_id } => {
                let id = next_id(projection.elements.iter().map(|e| fem3d_element_id(e).to_string()), "e");
                let index = projection.elements.len();
                let element = fem3d::FemElement::Bar { id, start: start.clone(), end: end.clone(), material_id: material_id.clone(), section_id: section_id.clone() };
                Ok(Emit::operations(vec![Fem3dOperation::SetElement { index, element: Box::new(element) }])
            }
            Fem3dCommand::AddFrame { start, end, material_id, section_id, roll } => {
                let id = next_id(projection.elements.iter().map(|e| fem3d_element_id(e).to_string()), "e");
                let index = projection.elements.len();
                let element = fem3d::FemElement::Frame { id, start: start.clone(), end: end.clone(), material_id: material_id.clone(), section_id: section_id.clone(), roll: *roll };
                Ok(Emit::operations(vec![Fem3dOperation::SetElement { index, element: Box::new(element) }])
            }
            Fem3dCommand::AddMaterial { name, e, g } => {
                let id = next_id(projection.materials.iter().map(|m| m.id.clone()), "m");
                let index = projection.materials.len();
                Ok(Emit::operations(vec![Fem3dOperation::SetMaterial { index, material: fem3d::FemMaterial { id, name: name.clone(), e: *e, g: *g, nu: 0.3, rho: 7850.0 } }])
            }
            Fem3dCommand::AddSection { name, area, iy, iz, j } => {
                let id = next_id(projection.sections.iter().map(|s| s.id.clone()), "s");
                let index = projection.sections.len();
                Ok(Emit::operations(vec![Fem3dOperation::SetSection { index, section: fem3d::FemSection { id, name: name.clone(), area: *area, iy: *iy, iz: *iz, j: *j } }])
            }
            Fem3dCommand::AddSupport { node_id, fixed } => {
                let id = next_id(projection.supports.iter().map(|s| s.id.clone()), "sup");
                let index = projection.supports.len();
                Ok(Emit::operations(vec![Fem3dOperation::SetSupport { index, support: fem3d::FemSupport { id, node_id: node_id.clone(), fixed: fixed.clone() } }])
            }
            Fem3dCommand::AddNodalLoad { node_id, dof, value, case_id } => {
                let (index, mut load_case) = fem3d_resolve_load_case(projection, case_id.as_deref());
                let load_id = next_id(load_case.loads.iter().map(|l| fem3d::load_id(l).to_string()), "l");
                load_case.loads.push(fem3d::FemLoad::Nodal { id: load_id, node_id: node_id.clone(), dof: *dof, value: *value });
                Ok(Emit::operations(vec![Fem3dOperation::SetLoadCase { index, load_case }])
            }
            Fem3dCommand::AddMemberUdl { element_id, wx, wy, wz, case_id } => {
                let (index, mut load_case) = fem3d_resolve_load_case(projection, case_id.as_deref());
                let load_id = next_id(load_case.loads.iter().map(|l| fem3d::load_id(l).to_string()), "l");
                load_case.loads.push(fem3d::FemLoad::MemberUdl { id: load_id, element_id: element_id.clone(), wx: *wx, wy: *wy, wz: *wz });
                Ok(Emit::operations(vec![Fem3dOperation::SetLoadCase { index, load_case }])
            }
            Fem3dCommand::AddAreaLoad { solid_id, pressure, case_id } => {
                let (index, mut load_case) = fem3d_resolve_load_case(projection, case_id.as_deref());
                let load_id = next_id(load_case.loads.iter().map(|l| fem3d::load_id(l).to_string()), "l");
                load_case.loads.push(fem3d::FemLoad::Area { id: load_id, solid_id: solid_id.clone(), pressure: *pressure });
                Ok(Emit::operations(vec![Fem3dOperation::SetLoadCase { index, load_case }])
            }
            Fem3dCommand::AddSolid { x, y, width, depth, height, material_id, base_z, layers, mesh_size } => {
                let id = next_id(projection.solids.iter().map(|s| s.id.clone()), "sol");
                let index = projection.solids.len();
                let outline = vec![[*x, *y], [x + width, *y], [x + width, y + depth], [*x, y + depth]];
                let solid = fem3d::FemSolid {
                    id,
                    name: "Solid".into(),
                    outline,
                    holes: Vec::new(),
                    base_z: base_z.unwrap_or(0.0),
                    height: *height,
                    layers: layers.map(|v| v as usize).unwrap_or(1),
                    mesh_size: mesh_size.unwrap_or(0.5),
                    material_id: material_id.clone(),
                };
                Ok(Emit::operations(vec![Fem3dOperation::SetSolid { index, solid }])
            }
            Fem3dCommand::AddLoadCase { name, self_weight } => {
                let id = next_id(projection.load_cases.iter().map(|lc| lc.id.clone()), "case-");
                let index = projection.load_cases.len();
                Ok(Emit::operations(vec![Fem3dOperation::SetLoadCase { index, load_case: fem3d::FemLoadCase { id, name: name.clone(), loads: Vec::new(), self_weight: *self_weight } }])
            }
            Fem3dCommand::AddCombination { name, terms } => match serde_json::from_str::<Vec<(String, f64)>>(terms) {
                Ok(parsed) => {
                    let terms: std::collections::BTreeMap<String, f64> = parsed.into_iter().collect();
                    let id = next_id(projection.combinations.iter().map(|c| c.id.clone()), "c");
                    let index = projection.combinations.len();
                    Ok(Emit::operations(vec![Fem3dOperation::SetCombination { index, combination: fem3d::FemCombination { id, name: name.clone(), terms } }])
                }
                Err(_) => Ok(Emit::default()),
            },
            Fem3dCommand::SetSelfWeight { case_id, enabled } => match projection.load_cases.iter().position(|lc| &lc.id == case_id) {
                Some(index) => {
                    let mut load_case = projection.load_cases[index].clone();
                    load_case.self_weight = *enabled;
                    Ok(Emit::operations(vec![Fem3dOperation::SetLoadCase { index, load_case }])
                }
                None => Ok(Emit::default()),
            },
            Fem3dCommand::SetAnalysisSettings { modal_count, buckling_count, deformation_scale } => {
                let current = &projection.analysis;
                let settings = fem3d::FemAnalysisSettings {
                    modal_count: modal_count.map(|value| value as usize).unwrap_or(current.modal_count),
                    buckling_count: buckling_count.map(|value| value as usize).unwrap_or(current.buckling_count),
                    deformation_scale: deformation_scale.unwrap_or(current.deformation_scale),
                };
                Ok(Emit::operations(vec![Fem3dOperation::SetAnalysisSettings { settings }])
            }
            Fem3dCommand::RemoveSelection { ids } => {
                let mut operations = Vec::new();
                for id in ids {
                    if projection.nodes.iter().any(|n| &n.id == id) {
                        operations.push(Fem3dOperation::RemoveNode { id: id.clone() });
                    } else if projection.elements.iter().any(|e| fem3d_element_id(e) == id) {
                        operations.push(Fem3dOperation::RemoveElement { id: id.clone() });
                    } else if projection.materials.iter().any(|m| &m.id == id) {
                        operations.push(Fem3dOperation::RemoveMaterial { id: id.clone() });
                    } else if projection.sections.iter().any(|s| &s.id == id) {
                        operations.push(Fem3dOperation::RemoveSection { id: id.clone() });
                    } else if projection.supports.iter().any(|s| &s.id == id) {
                        operations.push(Fem3dOperation::RemoveSupport { id: id.clone() });
                    } else if projection.load_cases.iter().any(|l| &l.id == id) {
                        operations.push(Fem3dOperation::RemoveLoadCase { id: id.clone() });
                    } else if projection.solids.iter().any(|s| &s.id == id) {
                        operations.push(Fem3dOperation::RemoveSolid { id: id.clone() });
                    } else if projection.combinations.iter().any(|c| &c.id == id) {
                        operations.push(Fem3dOperation::RemoveCombination { id: id.clone() });
                    }
                }
                if operations.is_empty() {
                    Ok(Emit::default()
                } else {
                    Ok(Emit::operations(operations)
                }
            }
            Fem3dCommand::SetActiveExample { example_id } => {
                let document = if example_id == "default" { Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap_or_else(|_| fem3d_engine::empty_fem3d_projection()) } else { fem3d_engine::empty_fem3d_projection() };
                Emit { document_operations: vec![Fem3dOperation::SetDocument { document }], config_operations: vec![Fem3dConfigOperation::Snapshot { config: Fem3dConfig::default() }], ..Default::default() }
            }
            // 🎥️ Config-only: the world-3d camera never touches the document.
            Fem3dCommand::SetCamera { json } => Ok(Emit::config(vec![Fem3dConfigOperation::SetCamera { camera: FemCamera { json: json.clone() } }])),
            // 👁️ Config-only: which case/mode the results window shows never touches the document.
            Fem3dCommand::SetResultDisplay { source_id, mode, mode_index } => Ok(Emit::config(vec![Fem3dConfigOperation::SetResultDisplay { source_id: source_id.clone(), mode: mode.clone(), mode_index: *mode_index }])),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Fem3dDocument>, cfg: &ConfigView<'_, Fem3dConfig>) -> UiNode {
        let camera = &cfg.projection.camera;
        match body_key {
            FEM3D_BODY_MODEL => render_fem3d_model(doc.projection, camera),
            FEM3D_BODY_RESULTS => render_fem3d_results(doc.projection, &config_result_display(cfg.projection), camera),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Fem3dPlayApp

//#region 🔖️Manifest
pub fn create_fem3d_app() -> App {
    App::from_builder(
        App::builder(FEM3D_APP_ID, LocalizedLabel::data("FEM 3D"))
            .document(["semio", "fem", "fem3d"])
            // 🔌️ The computed-results output artifact (`results:out`'s `kind_id`, see `fem3d_engine::fem3d_io`)
            // — the OS-catalog-level resource descriptor for `computation.fem3d`; deliberately a
            // different `media_type` (`Computation`×`Value`) than the PORT's wire-level `Data`×`Value`
            // (see `fem2d_ui::create_fem2d_app`'s identical `computation.fem2d` pattern one dimension down).
            .artifact_kind(ArtifactKindSpec {
                id: "computation.fem3d".into(),
                name: "FEM 3D Results".into(),
                source_format: "computation.fem3d".into(),
                component_kind: "fem3d-results".into(),
                dimension: "computation".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
                schema: "computation.fem3d".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("fem-app")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind(FEM3D_WINDOW_MODEL, LocalizedLabel::native("Model", "Modell"), FEM3D_BODY_MODEL, SurfaceKind::World3d, "fem-model")
            .window_kind(FEM3D_WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), FEM3D_BODY_RESULTS, SurfaceKind::World3d, "bar-chart-3")
            .default_layout(create_default_layout(
                &[FEM3D_WINDOW_MODEL.into(), FEM3D_WINDOW_RESULTS.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Model".into(), "Results".into()]),
            ))
            .operation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_args("addNode", vec![
                ActionArgDef::number("x", LocalizedLabel::data("X")).required(),
                ActionArgDef::number("y", LocalizedLabel::data("Y")).required(),
                ActionArgDef::number("z", LocalizedLabel::data("Z")).required(),
            ])
            .operation("addBar", LocalizedLabel::native("Add Bar", "Stab hinzufügen"))
            .operation("addFrame", LocalizedLabel::native("Add Frame", "Rahmen hinzufügen"))
            .operation("addMaterial", LocalizedLabel::native("Add Material", "Material hinzufügen"))
            .operation("addSection", LocalizedLabel::native("Add Section", "Querschnitt hinzufügen"))
            .operation("addSupport", LocalizedLabel::native("Add Support", "Lager hinzufügen"))
            .operation("addNodalLoad", LocalizedLabel::native("Add Nodal Load", "Knotenlast hinzufügen"))
            .action_args("addNodalLoad", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .operation("addMemberUdl", LocalizedLabel::native("Add Member UDL", "Streckenlast hinzufügen"))
            .action_args("addMemberUdl", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .operation("addAreaLoad", LocalizedLabel::native("Add Area Load", "Flächenlast hinzufügen"))
            .action_args("addAreaLoad", vec![
                ActionArgDef::text("solidId", LocalizedLabel::native("Solid", "Volumenkörper")).required(),
                ActionArgDef::number("pressure", LocalizedLabel::native("Pressure", "Druck")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .operation("addSolid", LocalizedLabel::native("Add Solid", "Volumenkörper hinzufügen"))
            .action_args("addSolid", vec![
                ActionArgDef::number("x", LocalizedLabel::data("X")).required(),
                ActionArgDef::number("y", LocalizedLabel::data("Y")).required(),
                ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required(),
                ActionArgDef::number("depth", LocalizedLabel::native("Depth", "Tiefe")).required(),
                ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::data("Material")).required(),
                ActionArgDef::number("baseZ", LocalizedLabel::native("Base Z", "Basis Z")).default_value(0.0),
                ActionArgDef::number("layers", LocalizedLabel::native("Layers", "Schichten")).default_value(1),
                ActionArgDef::number("meshSize", LocalizedLabel::native("Mesh Size", "Netzgröße")).default_value(0.5),
            ])
            .operation("addLoadCase", LocalizedLabel::native("Add Load Case", "Lastfall hinzufügen"))
            .action_args("addLoadCase", vec![
                ActionArgDef::text("name", LocalizedLabel::data("Name")).required(),
                ActionArgDef::toggle("selfWeight", LocalizedLabel::native("Self Weight", "Eigengewicht")).default_value(false),
            ])
            .operation("addCombination", LocalizedLabel::native("Add Combination", "Kombination hinzufügen"))
            .action_args("addCombination", vec![
                ActionArgDef::text("name", LocalizedLabel::data("Name")).required(),
                ActionArgDef::text("terms", LocalizedLabel::native("Terms", "Terme")).required(),
            ])
            .operation("setSelfWeight", LocalizedLabel::native("Set Self Weight", "Eigengewicht festlegen"))
            .action_args("setSelfWeight", vec![
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")).required(),
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).required(),
            ])
            .operation("setAnalysisSettings", LocalizedLabel::native("Set Analysis Settings", "Analyseeinstellungen festlegen"))
            .action_args("setAnalysisSettings", vec![
                ActionArgDef::number("modalCount", LocalizedLabel::native("Modal Count", "Anzahl Moden")),
                ActionArgDef::number("bucklingCount", LocalizedLabel::native("Buckling Count", "Anzahl Beulmoden")),
                ActionArgDef::number("deformationScale", LocalizedLabel::native("Deformation Scale", "Verformungsmaßstab")),
            ])
            .operation("removeSelection", LocalizedLabel::native("Remove Selection", "Auswahl entfernen"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new("default", LocalizedLabel::native("Default", "Standard"))]).default_value("default"),
            ])
            .view_action("setResultDisplay", LocalizedLabel::native("Set Result Display", "Ergebnisanzeige festlegen"))
            .action_args("setResultDisplay", result_display_action_args())
            // 🎯️ Typed channel surface — `config_spec()`/`fem3d_io()` are this same information's
            // single source of truth, reused here rather than duplicated (mirrors
            // `fem2d_ui::create_fem2d_app`'s identical wiring).
            .config(Fem3dPlayApp::default().config_spec())
            .io(fem3d_engine::fem3d_io()),
    )
    .example("default", LocalizedLabel::native("Family House", "Einfamilienhaus"), FEM3D_EXAMPLE_DSL, "file")
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
    use semio_framework_plugin::{ActionKind, HistoryView, Locale, Terminology};

    fn history_view() -> HistoryView {
        HistoryView::empty()
    }

    fn default_config() -> Fem3dConfig {
        Fem3dConfig::default()
    }

    //#region 🔖️RendersScenes
    #[test]
    fn renders_fem3d_model_scene() {
        let app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let node = app.render(FEM3D_BODY_MODEL, &doc, &cfg);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn renders_fem3d_results_scene() {
        let app = Fem3dPlayApp::default();
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let node = app.render(FEM3D_BODY_RESULTS, &doc, &cfg);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }
    //#endregion 🔖️RendersScenes

    //#region 🔖️AddNodeAction
    #[test]
    fn add_node_action_emits_op_3d() {
        let app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let emit = app.handle(&Fem3dCommand::AddNode { x: 1.0, y: 2.0, z: 3.0 }, &doc, &cfg);
        assert_eq!(emit.document_operations.len(), 1);
        match &emit.document_operations[0] {
            Fem3dOperation::SetNode { node, .. } => {
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
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let _ = app.render(FEM3D_BODY_RESULTS, &doc, &cfg);
    }
    //#endregion 🔖️SolverErrorSurfaced

    //#region 🔖️ExampleFixtureRenders
    #[test]
    fn example_fixture_renders_3d() {
        let app = Fem3dPlayApp::default();
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let _ = app.render(FEM3D_BODY_MODEL, &doc, &cfg);
        let _ = app.render(FEM3D_BODY_RESULTS, &doc, &cfg);
    }
    //#endregion 🔖️ExampleFixtureRenders

    //#region 🔖️SetActiveExample
    #[test]
    fn set_active_example_loads_default_fixture_3d() {
        let app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let emit = app.handle(&Fem3dCommand::SetActiveExample { example_id: "default".into() }, &doc, &cfg);
        assert_eq!(emit.document_operations.len(), 1);
        // 🧮️ Also resets the config back to its default (mirrors the pre-B1 `result_display`/`camera`
        // resets on `setActiveExample`) — a single whole-record `Snapshot`.
        assert_eq!(emit.config_operations, vec![Fem3dConfigOperation::Snapshot { config: Fem3dConfig::default() }]);
        match &emit.document_operations[0] {
            Fem3dOperation::SetDocument { document } => assert!(!document.nodes.is_empty(), "expected the default fixture's nodes"),
            _ => panic!("expected SetDocument"),
        }
    }

    /// 🧬️ `setActiveExample` replaces document content via `SetDocument` operations, so it MUST be declared as
    /// an Operation, not a View/Shell action — the framework's "View/Shell actions must not emit
    /// operations" guard would otherwise reject it.
    #[test]
    fn set_active_example_is_declared_as_operation_3d() {
        let definition = create_fem3d_app().definition;
        let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, ActionKind::Operation), "loading an example emits SetDocument operations, so it is an Operation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
    }
    //#endregion 🔖️SetActiveExample

    //#region 🔖️ModeShapeRender
    #[test]
    fn results_window_renders_modal_mode_shape_3d() {
        let app = Fem3dPlayApp::default();
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = Fem3dConfig { result_mode: "modal".into(), result_mode_index: 0, ..Fem3dConfig::default() };
        let cfg = ConfigView { projection: &config };
        let node = app.render(FEM3D_BODY_RESULTS, &doc, &cfg);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"), "expected a valid world-3d scene, got: {json}");
        assert!(!json.contains("Modal analysis error"), "unexpected modal error: {json}");
    }

    #[test]
    fn results_window_renders_buckling_mode_shape_3d() {
        let app = Fem3dPlayApp::default();
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = Fem3dConfig { result_source_id: Some("dead".into()), result_mode: "buckling".into(), result_mode_index: 0, ..Fem3dConfig::default() };
        let cfg = ConfigView { projection: &config };
        let node = app.render(FEM3D_BODY_RESULTS, &doc, &cfg);
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
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let node = app.render(FEM3D_BODY_MODEL, &doc, &cfg);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("solid-sol1"), "expected a solid- mesh/instance id for the example fixture's solid: {json}");
        assert!(json.contains("el-e1"), "expected a single oriented box instance per member (no -{{i}} sphere chain): {json}");
        assert!(!json.contains("\\\"sphere\\\""), "sphere markers should be gone: {json}");
    }

    #[test]
    fn results_scene_includes_solid_vertex_colors_3d() {
        let app = Fem3dPlayApp::default();
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = Fem3dConfig { result_source_id: Some("dead".into()), result_mode: "static".into(), ..Fem3dConfig::default() };
        let cfg = ConfigView { projection: &config };
        let node = app.render(FEM3D_BODY_RESULTS, &doc, &cfg);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("solid-sol1"), "expected the solid mesh in the results scene: {json}");
        assert!(json.contains("\\\"colors\\\""), "expected a vertex colors array on the solid mesh data: {json}");
        assert!(json.contains("Case: dead"), "expected a case-id caption: {json}");
    }

    #[test]
    fn results_scene_captions_name_mode_and_factor_3d() {
        let app = Fem3dPlayApp::default();
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };

        let config_modal = Fem3dConfig { result_mode: "modal".into(), result_mode_index: 0, ..Fem3dConfig::default() };
        let cfg_modal = ConfigView { projection: &config_modal };
        let node_modal = app.render(FEM3D_BODY_RESULTS, &doc, &cfg_modal);
        let json_modal = serde_json::to_string(&node_modal).unwrap();
        assert!(json_modal.contains("Hz"), "expected a frequency caption: {json_modal}");

        let config_buckling = Fem3dConfig { result_source_id: Some("dead".into()), result_mode: "buckling".into(), result_mode_index: 0, ..Fem3dConfig::default() };
        let cfg_buckling = ConfigView { projection: &config_buckling };
        let node_buckling = app.render(FEM3D_BODY_RESULTS, &doc, &cfg_buckling);
        let json_buckling = serde_json::to_string(&node_buckling).unwrap();
        assert!(json_buckling.contains("factor"), "expected a load-factor caption: {json_buckling}");
    }
    //#endregion 🔖️SolidRenderAndCaptions

    //#region 🔖️StructureActions
    #[test]
    fn add_solid_action_emits_set_solid_3d() {
        let app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let command = Fem3dCommand::AddSolid { x: 0.0, y: 0.0, width: 2.0, depth: 1.0, height: 0.5, material_id: "concrete".into(), base_z: None, layers: None, mesh_size: None };
        let emit = app.handle(&command, &doc, &cfg);
        assert_eq!(emit.document_operations.len(), 1);
        match &emit.document_operations[0] {
            Fem3dOperation::SetSolid { solid, .. } => {
                assert_eq!(solid.outline, vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
                assert_eq!(solid.height, 0.5);
                assert_eq!(solid.layers, 1);
            }
            _ => panic!("expected SetSolid"),
        }
    }

    #[test]
    fn remove_selection_covers_solids_3d() {
        let app = Fem3dPlayApp::default();
        let mut projection = fem3d_engine::empty_fem3d_projection();
        projection.solids.push(fem3d::FemSolid {
            id: "sol1".into(),
            name: "S".into(),
            outline: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            holes: vec![],
            base_z: 0.0,
            height: 1.0,
            layers: 1,
            mesh_size: 0.5,
            material_id: "concrete".into(),
        });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let emit = app.handle(&Fem3dCommand::RemoveSelection { ids: vec!["sol1".into()] }, &doc, &cfg);
        assert_eq!(emit.document_operations.len(), 1);
        assert!(matches!(emit.document_operations[0], Fem3dOperation::RemoveSolid { .. }));
    }
    //#endregion 🔖️StructureActions

    //#region 🔖️LoadCaseActions
    #[test]
    fn add_member_udl_action_emits_op_3d() {
        let app = Fem3dPlayApp::default();
        let mut projection = fem3d_engine::empty_fem3d_projection();
        projection.load_cases.push(fem3d::FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: false });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let command = Fem3dCommand::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -2000.0, case_id: None };
        let emit = app.handle(&command, &doc, &cfg);
        match &emit.document_operations[0] {
            Fem3dOperation::SetLoadCase { load_case, .. } => assert!(matches!(load_case.loads[0], fem3d::FemLoad::MemberUdl { .. })),
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

    //#region 🔖️UnknownBodyAndManifestLabels
    #[test]
    fn render_unknown_body_key_returns_placeholder_text_3d() {
        let app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let json = serde_json::to_string(&app.render("nonsense", &doc, &cfg)).unwrap();
        assert!(json.contains("Unknown body: nonsense"));
    }

    /// 🗣️ Static manifest text (window/action names) is now declared once via `LocalizedLabel::native`
    /// and resolved by the shell per active locale/terminology — replaces the deleted runtime
    /// `app_labels()`/`AppLabelsOverlay` overlay this app used to build by hand.
    #[test]
    fn manifest_labels_resolve_german_3d() {
        let definition = create_fem3d_app().definition;
        let window = definition.window_kinds.iter().find(|w| w.id == FEM3D_WINDOW_MODEL).expect("model window declared");
        assert_eq!(window.label.resolve(Terminology::Native, Locale::De), "Modell");
        let action = definition.actions.iter().find(|a| a.id == "addFrame").expect("addFrame declared");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::De), "Rahmen hinzufügen");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::En), "Add Frame");
    }
    //#endregion 🔖️UnknownBodyAndManifestLabels

    //#region 🔖️MoreStructureAndLoadActions
    #[test]
    fn add_material_action_emits_op_3d() {
        let app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let command = Fem3dCommand::AddMaterial { name: "Steel".into(), e: 2.1e11, g: 8.1e10 };
        let emit = app.handle(&command, &doc, &cfg);
        match &emit.document_operations[0] {
            Fem3dOperation::SetMaterial { material, .. } => assert_eq!(material.g, 8.1e10),
            _ => panic!("expected SetMaterial"),
        }
    }

    #[test]
    fn add_section_action_emits_op_3d() {
        let app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let command = Fem3dCommand::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.0000006 };
        let emit = app.handle(&command, &doc, &cfg);
        match &emit.document_operations[0] {
            Fem3dOperation::SetSection { section, .. } => assert_eq!(section.j, 0.0000006),
            _ => panic!("expected SetSection"),
        }
    }

    #[test]
    fn add_frame_action_emits_op_3d() {
        let app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let command = Fem3dCommand::AddFrame { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into(), roll: 0.5 };
        let emit = app.handle(&command, &doc, &cfg);
        match &emit.document_operations[0] {
            Fem3dOperation::SetElement { element, .. } => match element.as_ref() {
                fem3d::FemElement::Frame { roll, .. } => assert_eq!(*roll, 0.5),
                _ => panic!("expected Frame"),
            },
            _ => panic!("expected SetElement"),
        }
    }

    #[test]
    fn set_camera_action_writes_config_not_document_operations() {
        let app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = default_config();
        let cfg = ConfigView { projection: &config };
        let emit = app.handle(&Fem3dCommand::SetCamera { json: "{\"x\":1}".into() }, &doc, &cfg);
        assert!(emit.document_operations.is_empty(), "setCamera must not emit a VCS document operation");
        assert_eq!(emit.config_operations, vec![Fem3dConfigOperation::SetCamera { camera: FemCamera { json: "{\"x\":1}".into() } }]);
    }
    //#endregion 🔖️MoreStructureAndLoadActions

    //#region 🔖️MediaPorts
    /// 🎞️ `"results:out"` runs every load case fresh and returns a `Structured` JSON payload — build a
    /// doc with one load case, export, assert the JSON round-trips through `serde_json` and names the
    /// case id. Mirrors `fem2d_ui`'s equivalent end-to-end `results:out` test one dimension down.
    #[test]
    fn export_media_results_out_returns_solved_json_for_every_case_3d() {
        let app = Fem3dPlayApp::default();
        let projection: Fem3dDocument = Fem3dDocument::parse_dsl(FEM3D_EXAMPLE_DSL).unwrap();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let media = app.export_media("results:out", &doc).expect("results:out exports");
        assert_eq!(media.media_type.class, MediaClass::Data);
        assert_eq!(media.media_type.form, MediaForm::Value);
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected a Structured payload") };
        assert_eq!(schema, "computation.fem3d");
        let value: Value = serde_json::from_str(&json).expect("results:out payload is valid JSON");
        assert!(value.get("dead").is_some(), "expected the example fixture's dead case in the results map: {json}");
        assert!(value["dead"].get("displacements").is_some(), "expected a displacements array: {json}");
    }

    /// 🎞️ `"results:out"` on a document with no load cases errors rather than panicking or returning an
    /// empty payload.
    #[test]
    fn export_media_results_out_errors_without_load_cases_3d() {
        let app = Fem3dPlayApp::default();
        let projection = fem3d_engine::empty_fem3d_projection();
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let err = app.export_media("results:out", &doc).expect_err("no load cases should error");
        assert!(matches!(err, MediaError::Payload(..)));
    }

    /// 🎞️ `"geometry:in"` decodes an extruded-footprint JSON contract into a new `FemSolid` operation —
    /// build a geometry JSON payload, import, assert a new solid landed in the resulting operation.
    /// Mirrors `fem2d_ui`'s equivalent `geometry:in` test one dimension up (extra `baseZ`/`height`/
    /// `layers` fields).
    #[test]
    fn import_media_geometry_in_adds_a_new_solid_3d() {
        let app = Fem3dPlayApp::default();
        let mut projection = fem3d_engine::empty_fem3d_projection();
        projection.materials.push(fem3d::FemMaterial { id: "concrete".into(), name: "Concrete".into(), e: 30e9, g: 12.5e9, nu: 0.2, rho: 2400.0 });
        let history = history_view();
        let doc = DocumentView { projection: &projection, history: &history };
        let json = serde_json::json!({
            "outline": [[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]],
            "holes": [],
            "baseZ": 0.5,
            "height": 3.0,
            "layers": 2,
        })
        .to_string();
        let media = Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Any }, payload: MediaPayload::Structured { schema: "geometry".into(), json } };
        let emit = app.import_media("geometry:in", &media, &doc).expect("geometry:in imports");
        assert_eq!(emit.document_operations.len(), 1);
        match &emit.document_operations[0] {
            Fem3dOperation::SetSolid { solid, .. } => {
                assert_eq!(solid.outline, vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
                assert_eq!(solid.base_z, 0.5);
                assert_eq!(solid.height, 3.0);
                assert_eq!(solid.layers, 2);
                assert_eq!(solid.material_id, "concrete");
            }
            _ => panic!("expected SetSolid"),
        }
    }

    #[test]
    fn fem3d_io_matches_declared_artifact_identity_3d() {
        let app = Fem3dPlayApp::default();
        let io = app.io().expect("fem3d declares typed media I/O");
        assert_eq!(io.artifact.id, "3d.fem");
        assert!(io.ports.iter().any(|port| port.id == "geometry:in"));
        assert!(io.ports.iter().any(|port| port.id == "results:out"));
    }
    //#endregion 🔖️MediaPorts
}
//#endregion 🧪️Tests
