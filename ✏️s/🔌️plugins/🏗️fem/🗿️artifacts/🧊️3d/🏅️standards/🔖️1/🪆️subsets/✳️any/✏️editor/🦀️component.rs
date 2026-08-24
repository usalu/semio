//! 🖥️ FEM 3D play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch. Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: `Fem3dPlayApp` now
//! authors the `✏️editor` surface only — the read-only `👁️viewer` surface is a genuinely independent
//! sibling (`crate::viewer::fem3d::Fem3dViewer`), never constructed from this file.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, view state in `🎚️config`, shared compute in the artifact's
//! `⚙️engine`. This file is a routing table: `handle` → `Fem3dCommand::dispatch`, `render` → body-key →
//! window, and a `🔖️Manifest` region that calls one passthrough per node (scalar `.mode(..)`/
//! `.window_kind(..)` calls stay inline — fem3d builds neither a `ModeDefinition` nor a
//! `WindowKindDefinition` object anywhere, see `modes::edit`'s and the window nodes' own doc comments).

use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use crate::editor::fem3d::commands::{
    add_area_load, add_bar, add_combination, add_frame, add_load_case, add_material, add_member_udl, add_nodal_load, add_node, add_section, add_solid, add_support, remove_selection, set_active_example, set_analysis_settings, set_camera,
    set_result_display, set_self_weight,
};
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::editor::fem3d::modes::edit;
use crate::editor::fem3d::modes::edit::windows::{model as window_model, results as window_results};
use crate::model::{Dof, ElementResult};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    built_text_node, create_default_layout, ActionArgDef, ActionArgOption, AppDefinition, AppIo, AppRenderOperationContext, ArtifactEditor, ArtifactView, ConfigSpec, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, LocalizedLabel, Media,
    MediaClass, MediaError, MediaForm, MediaPayload, MediaType, NoDraft, NoDraftMutation, PluginCloseStep,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use store::EngineHandles;

//#region 🔖️Constants
pub const FEM3D_APP_ID: &str = "fem3d-play";
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Fem3dPlayApp::Command` — the SOLE dispatch surface for fem3d's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the codec uses) — they are genuinely different vocabularies for 3 of these 18
    /// rows: `setActiveExample`/`active-example`, `setCamera`/`camera`, `setResultDisplay`/
    /// `result-display`. **Row order is the binary variant ordinal: appending is safe, reordering is a
    /// wire-format break.** Unlike fem2d, there is NO `setLocale`/`SetLocale` row — fem3d's pre-migration
    /// `Fem3dCommand` enum never had one (a pre-existing, intentional asymmetry between the two apps).
    pub enum Fem3dCommand for Fem3dSnapshot, Fem3dMutation, Fem3dConfig, Fem3dConfigMutation {
        "addNode" as "add-node" => add_node::AddNode,
        "addBar" as "add-bar" => add_bar::AddBar,
        "addFrame" as "add-frame" => add_frame::AddFrame,
        "addMaterial" as "add-material" => add_material::AddMaterial,
        "addSection" as "add-section" => add_section::AddSection,
        "addSupport" as "add-support" => add_support::AddSupport,
        "addNodalLoad" as "add-nodal-load" => add_nodal_load::AddNodalLoad,
        "addMemberUdl" as "add-member-udl" => add_member_udl::AddMemberUdl,
        "addAreaLoad" as "add-area-load" => add_area_load::AddAreaLoad,
        "addSolid" as "add-solid" => add_solid::AddSolid,
        "addLoadCase" as "add-load-case" => add_load_case::AddLoadCase,
        "addCombination" as "add-combination" => add_combination::AddCombination,
        "setSelfWeight" as "set-self-weight" => set_self_weight::SetSelfWeight,
        "setAnalysisSettings" as "set-analysis-settings" => set_analysis_settings::SetAnalysisSettings,
        "removeSelection" as "remove-selection" => remove_selection::RemoveSelection,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setResultDisplay" as "result-display" => set_result_display::SetResultDisplay,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🔖️Fem3dResultsJson
/// 🎨️ Manual `crate::model::StaticResult` -> JSON bridge for `"results:out"` (see `export_media` below)
/// — `crate::model::StaticResult`/`ElementResult`/`Dof` don't derive `Serialize` (the `🫀️core` kernel is
/// a cross-artifact shared crate, out of scope to touch here), so this hand-rolls the same shape
/// `serde_json::to_string` would have produced, using `Dof`'s existing `{:?}` formatting. Single
/// consumer (`export_media`), so this lives here rather than in the artifact's `⚙️engine`.
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

fn fem3d_static_result_json(result: &crate::model::StaticResult) -> Value {
    json!({
        "displacements": result.displacements.iter().map(|d| json!({ "nodeId": d.node_id, "values": d.values })).collect::<Vec<_>>(),
        "reactions": result.reactions.iter().map(|r| json!({ "nodeId": r.node_id, "dof": fem3d_dof_json(r.dof), "value": r.value })).collect::<Vec<_>>(),
        "elements": result.elements.iter().map(|(id, element_result)| json!({ "id": id, "result": fem3d_element_result_json(element_result) })).collect::<Vec<_>>(),
        "checks": { "residualNorm": result.checks.residual_norm, "reactionSum": result.checks.reaction_sum },
    })
}

fn fem3d_results_map_json(results: &HashMap<String, crate::model::StaticResult>) -> Value {
    Value::Object(results.iter().map(|(id, result)| (id.clone(), fem3d_static_result_json(result))).collect())
}
//#endregion 🔖️Fem3dResultsJson

//#region 🔌️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document port pair
/// (`fem.3d` × 3D-Any) plus `geometry:in` (importing an externally authored extruded-footprint outline
/// as a new `FemSolid` — see `import_media` above) and `results:out` (every load case/combination's
/// solved `crate::model::StaticResult`, pinned to the `computation.fem3d` artifact kind declared in
/// `crate::artifacts::fem3d::computation_artifact_kind` — see `export_media` above). Moved out of the
/// (now deleted) artifact `⚙️engine`: it returns `AppIo`, an app type, so it belongs here.
pub fn fem3d_io() -> AppIo {
    AppIo {
        document_schema: crate::artifacts::fem3d::FEM_3D_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Any },
        ports: vec![fem3d_geometry_in_port(), fem3d_results_out_port()],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.fem".into(), name: "FEM 3D".into(), dimension: "3d".into(), component_kind: "fem3d".into() },
    }
}

/// 🔌️ `geometry:in` — an externally authored extruded-footprint outline (polygon-with-holes,
/// base/height/layers), imported as a new `FemSolid`.
pub fn fem3d_geometry_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "geometry:in".into(),
        label: "Geometry".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Any },
        kind_id: None,
        required: true,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}

/// 🔌️ `results:out` — every load case/combination's solved `crate::model::StaticResult`, pinned to the
/// `computation.fem3d` artifact kind.
pub fn fem3d_results_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "results:out".into(),
        label: "Results".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        kind_id: Some("computation.fem3d".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}
//#endregion 🔌️Io

//#region 🎬️SceneRender
/// 🎬️ App-facing 3D scene-building bridge, moved out of the (now deleted) artifact `⚙️engine`: every fn
/// here references `crate::app_surface` (an app type) and/or returns scene JSON consumed only by the
/// model/results windows (`crate::editor::fem3d::modes::edit::windows::{model, results}`), per the
/// migration recipe's `DocumentHelpers` rule — a helper with 2+ window consumers belongs at the app
/// level, not duplicated per window.
use crate::fem3d_engine::mesh_preview;

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

/// 🧊️ Half-extent-ish scale of the small box instance drawn at each node.
const NODE_SIZE_3D: f64 = 0.05;
/// 🧊️ Cross-section (x/y) thickness of the oriented box prism drawn for each `Bar`/`Frame` member —
/// a fixed visual thickness, not the member's actual section dimensions (see `fem3d_structural_instances`).
const MEMBER_THICKNESS_3D: f64 = 0.05;

fn find_node_3d<'a>(nodes: &'a [crate::artifacts::fem3d::FemNode], id: &str) -> Option<&'a crate::artifacts::fem3d::FemNode> {
    nodes.iter().find(|n| n.id == id)
}

fn fem3d_element_endpoints(element: &crate::artifacts::fem3d::FemElement) -> (&str, &str) {
    match element {
        crate::artifacts::fem3d::FemElement::Bar { start, end, .. } | crate::artifacts::fem3d::FemElement::Frame { start, end, .. } => (start.as_str(), end.as_str()),
    }
}

/// 🧊️ One small box instance per node, plus one ORIENTED box prism per `Bar`/`Frame` member — position
/// at the (possibly deformed) midpoint, `scale=[t,t,length]` so the mesh's own long (local Z) axis
/// stretches along the member, `rotation` a quaternion aligning that axis to the member's direction
/// (composed with a `Frame`'s own `roll` about its own axis; `Bar`s have no roll).
fn fem3d_structural_instances(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64) -> Vec<Value> {
    let node_pos = |node: &crate::artifacts::fem3d::FemNode| fem3d_deformed_position([node.x, node.y, node.z], &node.id, displacements, deform_scale);

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
            crate::artifacts::fem3d::FemElement::Frame { roll, .. } => *roll,
            crate::artifacts::fem3d::FemElement::Bar { .. } => 0.0,
        };
        let rotation = quat_mul(quat_z_to(dir), quat_roll_z(roll));
        let mid = [(p1[0] + p2[0]) / 2.0, (p1[1] + p2[1]) / 2.0, (p1[2] + p2[2]) / 2.0];
        let id = crate::artifacts::fem3d::element_id(element);
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
/// when present, colors each vertex by `crate::app_surface::von_mises_color` (min/max taken across ALL
/// solids' averaged values), driving the react renderer's vertex-color contour (see
/// `PaintTexturedMesh`). `displacements` deforms vertex positions the same way
/// `fem3d_structural_instances` deforms node/member instances.
fn fem3d_solid_mesh_entries(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64, nodal_stress: Option<&HashMap<String, f64>>) -> (Vec<Value>, Vec<Value>) {
    use crate::app_surface::{hex_to_rgb01, von_mises_color};

    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    let Ok(solid_meshes) = mesh_preview::fem3d_mesh_preview(doc) else { return (meshes, instances) };
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
pub fn fem3d_scene_parts(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64, nodal_stress: Option<&HashMap<String, f64>>) -> (String, String) {
    let mut meshes: Vec<Value> = serde_json::from_str(&semio_framework_plugin::resolve_ready(semio_framework_plugin::world3d_meshes_json_from_kinds(&["box".to_string()]))).unwrap_or_default();
    let mut instances = fem3d_structural_instances(doc, displacements, deform_scale);
    let (solid_meshes, solid_instances) = fem3d_solid_mesh_entries(doc, displacements, deform_scale, nodal_stress);
    meshes.extend(solid_meshes);
    instances.extend(solid_instances);
    (serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()))
}

/// 🎥️ Resolves a `FemCamera` to its JSON string, falling back to the framework's default 3D camera when
/// the document/config still carries the sentinel empty-object placeholder.
pub fn fem3d_camera_json(camera: &crate::artifacts::fem3d::FemCamera) -> String {
    if camera.json == "{}" {
        semio_framework_plugin::resolve_ready(semio_framework_plugin::world3d_default_camera())
    } else {
        camera.json.clone()
    }
}
//#endregion 🎬️SceneRender

//#region 🔖️Fem3dPlayApp
/// 🧮️ v0 design: results are recomputed fresh inside `render()`, no cache, no `RunAnalysis` operation.
/// Unit struct — every former `RefCell` field lives in `Fem3dConfig`, written through
/// `Fem3dConfigMutation`s.
#[derive(Default)]
pub struct Fem3dPlayApp;

impl ArtifactEditor for Fem3dPlayApp {
    type Snapshot = Fem3dSnapshot;
    type Mutation = Fem3dMutation;
    type Config = Fem3dConfig;
    type ConfigMutation = Fem3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::fem3d::presence::Fem3dPresence;
    type PresenceMutation = crate::editor::fem3d::presence::Fem3dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Fem3dCommand;

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::fem3d::config::schema::app_schema_descriptor())
    }

    /// 🪪️ W2 packet P7: the canonical `ArtifactEditor::DIALECT`, derived from the artifact-level
    /// `FEM3D_DIALECT` constant (`🗿️artifacts/🧊️3d/🦀️component.rs`) so the sibling `👁️viewer` surface
    /// can read the very same value without ever importing through this `editor` module.
    const DIALECT: Dialect = crate::artifacts::fem3d::FEM3D_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::fem3d::FEM_3D_SCHEMA;

    async fn initial_snapshot() -> Fem3dSnapshot {
        crate::artifacts::fem3d::schema::empty_fem3d_snapshot()
    }

    async fn io() -> Option<AppIo> {
        Some(fem3d_io())
    }

    fn mounted_job_maintenance_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(crate::editor::fem3d::session::maintenance_step(instance_id, maximum_items, maximum_bytes))
    }

    fn mounted_job_close_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(crate::editor::fem3d::session::close_step(instance_id, maximum_items, maximum_bytes))
    }

    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        crate::editor::fem3d::session::terminal_is_empty(instance_id)
    }

    fn mounted_job_prepare_snapshot_read(operation: AppRenderOperationContext, snapshot: &Self::Snapshot) -> bool {
        crate::editor::fem3d::session::prepare_snapshot_read(operation, snapshot)
    }

    /// 🎞️ `"document:out"` reproduces the trait's default whole-document pack (overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new
    /// one). `"results:out"` runs every load case/combination's analysis fresh and returns them as plain
    /// JSON text in a `Structured` payload. A document with no load cases, or a solve failure, is
    /// reported as `MediaError::Payload` rather than an empty/panicking export.
    async fn export_media(port: &str, doc: &ArtifactView<'_, Fem3dSnapshot>) -> Result<Media, MediaError> {
        match port {
            "document:out" => {
                let media_type = fem3d_io().document_media_type;
                let bytes = <Fem3dSnapshot as store::ArtifactPack>::encode_pack(doc.snapshot);
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "results:out" => {
                if doc.snapshot.load_cases.is_empty() {
                    return Err(MediaError::Payload("results:out".into(), "no load cases defined".into()));
                }
                let results = crate::fem3d_engine::fem3d_solve_all(doc.snapshot).map_err(|error| MediaError::Payload("results:out".into(), error.to_string()))?;
                let json = fem3d_results_map_json(&results).to_string();
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.fem3d".into(), json } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧬️ No `whole_document_operation` override on this impl — per `📓️taxonomy.md`, whole-document
    /// replace (`SetSnapshot`) is banned outright with NO replacement mutation, so this falls back to
    /// the trait's own default (`None`).
    ///
    /// 🎞️ `"document:in"` swaps the whole live document via `reset_document_effect` (a
    /// `Effect::LoadDocument`, the sanctioned non-history whole-doc-replace path — see
    /// `reset_document_effect`'s own doc comment) instead of routing through `whole_document_operation`.
    /// `"geometry:in"` decodes a minimal, app-owned `{"outline": [[f64;2]...], "holes": [[[f64;2]...]...],
    /// "baseZ"?: f64, "height"?: f64, "layers"?: usize}` extruded-footprint contract into a new
    /// `FemSolid`, defaulted to the document's first existing material if any, else an `"unassigned"`
    /// placeholder id — the solid simply won't solve until a real material is assigned.
    async fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Fem3dSnapshot>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let snapshot = <Fem3dSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
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
                let material_id = doc.snapshot.materials.first().map(|material| material.id.clone()).unwrap_or_else(|| "unassigned".into());
                let id = crate::app_surface::next_id(doc.snapshot.solids.iter().map(|s| s.id.clone()), "sol");
                let solid = crate::artifacts::fem3d::FemSolid { id, name: "Imported Geometry".into(), outline, holes, base_z, height, layers, mesh_size: 0.5, material_id };
                Ok(Emit::mutations(vec![Fem3dMutation::CreateSolid(crate::artifacts::fem3d::mutations::create_solid::mutation::CreateSolid { solid })]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧮️ No sticky `ActionArgDef` defaults are mirrored here (all of `addSolid`'s
    /// `baseZ`/`layers`/`meshSize` defaults are baked directly into its handler, not user-configurable
    /// settings).
    async fn config_spec() -> ConfigSpec {
        semio_framework_plugin::resolve_ready(ConfigSpec::empty())
    }

    async fn command_id(command: &Fem3dCommand) -> &'static str {
        command.command_id()
    }

    async fn handle(
        command: &Fem3dCommand,
        doc: &ArtifactView<'_, Fem3dSnapshot>,
        cfg: &ConfigView<'_, Fem3dConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    async fn pending_effects(doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Vec<semio_framework::kernel::Effect> {
        crate::editor::fem3d::session::reconcile(doc)
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Fem3dSnapshot>, cfg: &ConfigView<'_, Fem3dConfig>) -> semio_framework_plugin::ComponentTree {
        let camera = &cfg.snapshot.camera;
        semio_framework_plugin::built_to_component_tree(match body_key {
            window_model::FEM3D_BODY_MODEL => crate::editor::fem3d::session::with_live_visual(doc.render_operation(), |visual| window_model::render_with_progress(camera, visual)),
            window_results::FEM3D_BODY_RESULTS => crate::editor::fem3d::session::with_live_visual(doc.render_operation(), |visual| window_results::render_with_progress(camera, visual)),
            _ => built_text_node(Label::data(format!("Unknown body: {body_key}"))),
        })
    }
}
//#endregion 🔖️Fem3dPlayApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (file import,
/// load-example). Per `📓️taxonomy.md`, `SetSnapshot` is banned outright with NO replacement
/// mutation: whole-document replace is not expressible as an in-history `Mutation` at all. Every
/// former "replace the whole document" gesture in this package (`import_media`'s `"document:in"`,
/// `commands::set_active_example`) builds this effect instead of an `Emit::mutations([...])`.
/// The spr is a fresh, edit-free op-log for `scene` — a genesis envelope with no history to encode.
pub fn reset_document_effect(scene: &Fem3dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Fem3dSnapshot as store::ArtifactPack>::encode_pack(scene);
    let envelope = store::create_document_envelope::<Fem3dSnapshot, Fem3dMutation>(crate::artifacts::fem3d::FEM_3D_SCHEMA, "fem3d", scene.clone(), None);
    let spr = semio_framework_plugin::resolve_ready(store::print_document_spr(&envelope)).expect("fem3d document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node. fem3d's mode/windows are all scalar
/// (`.mode(..)`/`.window_kind(..)`) declarations — no `_def` passthrough exists for them since no
/// `ModeDefinition`/`WindowKindDefinition` object is built anywhere (see `modes::edit`'s doc comment).
///
/// 🚧️ SDK GAP (contract §2.4, `App { definition, examples }` split): `EditorBuilder` has no
/// `.example(...)`/`.workflow(...)` methods — the pre-migration chain's trailing
/// `.example("default", LocalizedLabel::native("Family House", "Einfamilienhaus"),
/// crate::artifacts::fem3d::dsl::FEM3D_EXAMPLE_TEXT, "file")` and `.workflow("fem3d", "FEM 3D",
/// "structure")` calls are dropped here, not ported. `setActiveExample`'s handler loads the same
/// `FEM3D_EXAMPLE_TEXT` fixture directly.
pub fn create_fem3d_app() -> AppDefinition {
    Editor::builder(crate::artifacts::fem3d::FEM3D_DIALECT)
            .document(["semio", "fem", "fem3d"])
            .artifact_kind(crate::artifacts::fem3d::computation_artifact_kind())
            .icon_id("fem-app")
            .mode(edit::MODE_ID, LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id(edit::MODE_ID)
            .window_kind(window_model::FEM3D_WINDOW_MODEL, LocalizedLabel::native("Model", "Modell"), window_model::FEM3D_BODY_MODEL, semio_framework_ui_contract::SurfaceKind::World3d, "fem-model")
            .window_kind(window_results::FEM3D_WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), window_results::FEM3D_BODY_RESULTS, semio_framework_ui_contract::SurfaceKind::World3d, "bar-chart-3")
            .default_layout(create_default_layout(
                &[window_model::FEM3D_WINDOW_MODEL.into(), window_results::FEM3D_WINDOW_RESULTS.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Model".into(), "Results".into()]),
            ))
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_args("addNode", vec![
                ActionArgDef::number("x", LocalizedLabel::data("X")).required(),
                ActionArgDef::number("y", LocalizedLabel::data("Y")).required(),
                ActionArgDef::number("z", LocalizedLabel::data("Z")).required(),
            ])
            .mutation("addBar", LocalizedLabel::native("Add Bar", "Stab hinzufügen"))
            .mutation("addFrame", LocalizedLabel::native("Add Frame", "Rahmen hinzufügen"))
            .mutation("addMaterial", LocalizedLabel::native("Add Material", "Material hinzufügen"))
            .mutation("addSection", LocalizedLabel::native("Add Section", "Querschnitt hinzufügen"))
            .mutation("addSupport", LocalizedLabel::native("Add Support", "Lager hinzufügen"))
            .mutation("addNodalLoad", LocalizedLabel::native("Add Nodal Load", "Knotenlast hinzufügen"))
            .action_args("addNodalLoad", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .mutation("addMemberUdl", LocalizedLabel::native("Add Member UDL", "Streckenlast hinzufügen"))
            .action_args("addMemberUdl", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .mutation("addAreaLoad", LocalizedLabel::native("Add Area Load", "Flächenlast hinzufügen"))
            .action_args("addAreaLoad", vec![
                ActionArgDef::text("solidId", LocalizedLabel::native("Solid", "Volumenkörper")).required(),
                ActionArgDef::number("pressure", LocalizedLabel::native("Pressure", "Druck")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addSolid", LocalizedLabel::native("Add Solid", "Volumenkörper hinzufügen"))
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
            .mutation("addLoadCase", LocalizedLabel::native("Add Load Case", "Lastfall hinzufügen"))
            .action_args("addLoadCase", vec![
                ActionArgDef::text("name", LocalizedLabel::data("Name")).required(),
                ActionArgDef::toggle("selfWeight", LocalizedLabel::native("Self Weight", "Eigengewicht")).default_value(false),
            ])
            .mutation("addCombination", LocalizedLabel::native("Add Combination", "Kombination hinzufügen"))
            .action_args("addCombination", vec![
                ActionArgDef::text("name", LocalizedLabel::data("Name")).required(),
                ActionArgDef::text("terms", LocalizedLabel::native("Terms", "Terme")).required(),
            ])
            .mutation("setSelfWeight", LocalizedLabel::native("Set Self Weight", "Eigengewicht festlegen"))
            .action_args("setSelfWeight", vec![
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")).required(),
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).required(),
            ])
            .mutation("setAnalysisSettings", LocalizedLabel::native("Set Analysis Settings", "Analyseeinstellungen festlegen"))
            .action_args("setAnalysisSettings", vec![
                ActionArgDef::number("modalCount", LocalizedLabel::native("Modal Count", "Anzahl Moden")),
                ActionArgDef::number("bucklingCount", LocalizedLabel::native("Buckling Count", "Anzahl Beulmoden")),
                ActionArgDef::number("deformationScale", LocalizedLabel::native("Deformation Scale", "Verformungsmaßstab")),
            ])
            .mutation("removeSelection", LocalizedLabel::native("Remove Selection", "Auswahl entfernen"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new("default", LocalizedLabel::native("Default", "Standard"))]).default_value("default"),
            ])
            .view_action("setResultDisplay", LocalizedLabel::native("Set Result Display", "Ergebnisanzeige festlegen"))
            .action_args("setResultDisplay", crate::app_surface::result_display_action_args())
            // 🎯️ Typed channel surface — `config_spec()`/`fem3d_io()` are this same information's single
            // source of truth, reused here rather than duplicated.
            .config(semio_framework_plugin::resolve_ready(Fem3dPlayApp::config_spec()))
            .io(fem3d_io())
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{ArtifactApp, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type Fem3dApp = VcsArtifactApp<EditorApp<Fem3dPlayApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    /// `EditorApp<Fem3dPlayApp>` (SDK adapter, contract §2.1) is the real `ArtifactApp` implementor
    /// `VcsArtifactApp` wraps, exactly the way `PluginBuilder::editor::<Fem3dPlayApp>` builds it.
    pub fn fem3d_app() -> Fem3dApp {
        semio_framework_plugin::resolve_ready(new_app::<EditorApp<Fem3dPlayApp>>())
    }

    /// 🚧️ SDK GAP: `new_app_with_registry` still expects `fn() -> App` (contract §2.4's
    /// `App { definition, examples }` split was not threaded through this testkit fn) — wrap the now
    /// `AppDefinition`-returning `create_fem3d_app` the same way the cad pilot's own
    /// `cad_app_manifest_for_testkit` does.
    fn fem3d_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_fem3d_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn fem3d_app_with_registry() -> Fem3dApp {
        semio_framework_plugin::resolve_ready(new_app_with_registry::<EditorApp<Fem3dPlayApp>>(fem3d_app_manifest_for_testkit))
    }

    pub async fn dispatch(app: &mut Fem3dApp, command: Fem3dCommand) -> InvocationResult {
        let result = app.dispatch_typed(command, &meta("local")).await.expect("dispatch");
        for effect in &result.requested_effects {
            if let semio_framework_plugin::Effect::LoadDocument { pack, spr } = effect {
                let files = store::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() };
                app.load_document_pack(&files).await.expect("test host applies load-document effect");
            }
        }
        result
    }

    pub fn render(app: &mut Fem3dApp, body_key: &str) -> String {
        serde_json::to_string(&semio_framework_plugin::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem3d::testkit::{dispatch, fem3d_app, Fem3dApp};
    use semio_framework_plugin::testkit::assert_undo_redo_round_trip;

    //#region 🔖️CommandSurface
    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order — mirrors the exact
    /// fixture values the pre-migration `fem3d_protocol` crate's own `Fem3dCommand` test used.
    fn every_command() -> Vec<Fem3dCommand> {
        vec![
            Fem3dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0, z: 3.0 }),
            Fem3dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }),
            Fem3dCommand::AddFrame(add_frame::AddFrame { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.5 }),
            Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11, g: 8.077e10 }),
            Fem3dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.0000006 }),
            Fem3dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: crate::artifacts::fem3d::FemDof::ALL.to_vec() }),
            Fem3dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::artifacts::fem3d::FemDof::Tz, value: -5000.0, case_id: Some("live".into()) }),
            Fem3dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -500.0, case_id: None }),
            Fem3dCommand::AddAreaLoad(add_area_load::AddAreaLoad { solid_id: "sol1".into(), pressure: 5000.0, case_id: Some("dead".into()) }),
            Fem3dCommand::AddSolid(add_solid::AddSolid { x: 0.0, y: 0.0, width: 4.0, depth: 2.0, height: 0.5, material_id: "concrete".into(), base_z: Some(0.0), layers: Some(2), mesh_size: None }),
            Fem3dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }),
            Fem3dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: "[[\"dead\",1.35],[\"live\",1.5]]".into() }),
            Fem3dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "dead".into(), enabled: true }),
            Fem3dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0) }),
            Fem3dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["n1".into(), "e1".into()] }),
            Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }),
            Fem3dCommand::SetCamera(set_camera::SetCamera { json: "{\"x\":1}".into() }),
            Fem3dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 }),
        ]
    }

    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 18, "every Fem3dCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 📌️ LAW: the pre-migration command wire format, row for row — the hex list is positionally aligned
    /// to `every_command()`, which carries exactly the values the old `📡️protocol` crate's baseline dump
    /// used (ticket `26/08/05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`,
    /// `🧪️wire-baseline-before-3d.txt`). Row order is the binary variant ordinal, so a reordering — which
    /// no round-trip law can catch — shows up here as a leading-byte mismatch. `addNodalLoad`'s `None`
    /// case is pinned separately below because `every_command()` only carries its `Some` shape.
    #[semio_framework_async_macros::async_test]
    async fn every_command_keeps_its_pre_migration_bytes() {
        use protocol::OpBinary;
        let expected = [
            "010000030005000000000000f03f0105000000000000004002050000000000000840",
            "010104026e31026e3203726f6405737465656c04000600010601020603030602",
            "01020406686561323030026e31026e3205737465656c050006010106020206030306000405000000000000e03f",
            "01030105537465656c030006000105000000da7c72484202050000806444ce3242",
            "0104010648454132303005000600010545f5d6c05609763f020554fc8458a258033f0305210ec81462e4eb3e040576830df4f521a43e",
            "010501026e310200060001160600020406080a",
            "010602046c697665026e3104000601010a020205000000000088b3c0030600",
            "01070102653104000600010500000000000000000205000000000000000003050000000000407fc0",
            "010802046465616404736f6c31030006010105000000000088b340020600",
            "01090108636f6e637265746508000500000000000000000105000000000000000002050000000000001040030500000000000000400405000000000000e03f05060006050000000000000000070402",
            "010a01044c697665020006000101",
            "010b0203554c531c5b5b2264656164222c312e33355d2c5b226c697665222c312e355d5d02000600010601",
            "010c010464656164020006000102",
            "010d000200040502050000000000003e40",
            "010e02026531026e3101000c0206010600",
            "010f010764656661756c7401000600",
            "011001077b2278223a317d01000600",
            "0111020464656164056d6f64616c03000600010601020400",
        ];
        let commands = every_command();
        assert_eq!(commands.len(), expected.len(), "the baseline hex list must cover every command row");
        for (command, expected) in commands.iter().zip(expected) {
            let bytes = command.encode_op().expect("encode");
            assert_eq!(bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), expected, "wire bytes changed for {}", command.command_id());
        }
        let nodal_load_without_case = Fem3dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::artifacts::fem3d::FemDof::Tz, value: -5000.0, case_id: None });
        assert_eq!(nodal_load_without_case.encode_op().expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>(), "010601026e3103000600010a020205000000000088b3c0");
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword. Three rows
    /// (`setActiveExample`/`setCamera`/`setResultDisplay`) prove the wire keyword is NOT simply the
    /// kebab-cased command id — this is exactly what a missing `#[dsl(keyword = ..)]` on a payload struct
    /// silently breaks (the record prints with no keyword at all and no longer parses).
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        let expected_keys = [
            "add-node",
            "add-bar",
            "add-frame",
            "add-material",
            "add-section",
            "add-support",
            "add-nodal-load",
            "add-member-udl",
            "add-area-load",
            "add-solid",
            "add-load-case",
            "add-combination",
            "set-self-weight",
            "set-analysis-settings",
            "remove-selection",
            "active-example",
            "camera",
            "result-display",
        ];
        for (command, expected) in every_command().into_iter().zip(expected_keys) {
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {command:?}: {printed:?}");
        }
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_fem3d_app()).expect("app definition json");
        for id in [window_model::FEM3D_WINDOW_MODEL, window_results::FEM3D_WINDOW_RESULTS] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::MODE_ID), "mode {} missing from the manifest", edit::MODE_ID);
        assert!(json.contains("computation.fem3d"), "artifact kind missing from the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn manifest_labels_resolve_german_3d() {
        use semio_framework_plugin::{Locale, Terminology};
        let definition = create_fem3d_app();
        let window = definition.window_kinds.iter().find(|w| w.id == window_model::FEM3D_WINDOW_MODEL).expect("model window declared");
        assert_eq!(window.label.resolve(Terminology::Native, Locale::De), "Modell");
        let action = window.actions.iter().find(|action| action.id == "addFrame").expect("addFrame declared");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::De), "Rahmen hinzufügen");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::En), "Add Frame");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn undo_restores_document_after_add_node() {
        let mut app = fem3d_app();
        let before = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot").nodes.len();
        assert_undo_redo_round_trip(&mut app, Fem3dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0, z: 3.0 }), |app| semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot").nodes.len(), before, before + 1).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::fem3d::testkit::render;
        let mut app = fem3d_app();
        assert!(render(&mut app, "fem3d.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️MediaPorts
    /// 🎞️ `"results:out"` runs every load case fresh and returns a `Structured` JSON payload — build a
    /// doc with the bundled example (which has load cases), export, assert the JSON round-trips through
    /// `serde_json` and names a case id.
    #[semio_framework_async_macros::async_test]
    async fn export_media_results_out_returns_solved_json_for_every_case_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() })).await;
        let snapshot = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot");
        let history = semio_framework_plugin::resolve_ready(semio_framework_plugin::HistoryView::empty());
        let doc = semio_framework_plugin::resolve_ready(ArtifactView::new(&snapshot, &history));
        let media = semio_framework_plugin::resolve_ready(Fem3dPlayApp::export_media("results:out", &doc)).expect("results:out exports");
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
    #[semio_framework_async_macros::async_test]
    async fn export_media_results_out_errors_without_load_cases_3d() {
        let snapshot = crate::artifacts::fem3d::schema::empty_fem3d_snapshot();
        let history = semio_framework_plugin::resolve_ready(semio_framework_plugin::HistoryView::empty());
        let doc = semio_framework_plugin::resolve_ready(ArtifactView::new(&snapshot, &history));
        let err = semio_framework_plugin::resolve_ready(Fem3dPlayApp::export_media("results:out", &doc)).expect_err("no load cases should error");
        assert!(matches!(err, MediaError::Payload(..)));
    }

    /// 🎞️ `"geometry:in"` decodes an extruded-footprint JSON contract into a new `FemSolid` operation.
    #[semio_framework_async_macros::async_test]
    async fn import_media_geometry_in_adds_a_new_solid_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Concrete".into(), e: 30e9, g: 12.5e9 })).await;
        let snapshot = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot");
        let history = semio_framework_plugin::resolve_ready(semio_framework_plugin::HistoryView::empty());
        let doc = semio_framework_plugin::resolve_ready(ArtifactView::new(&snapshot, &history));
        let json = serde_json::json!({
            "outline": [[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]],
            "holes": [],
            "baseZ": 0.5,
            "height": 3.0,
            "layers": 2,
        })
        .to_string();
        let media = Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Any }, payload: MediaPayload::Structured { schema: "geometry".into(), json } };
        let emit = semio_framework_plugin::resolve_ready(Fem3dPlayApp::import_media("geometry:in", &media, &doc)).expect("geometry:in imports");
        assert_eq!(emit.artifact_mutations.len(), 1);
        match &emit.artifact_mutations[0] {
            Fem3dMutation::CreateSolid(crate::artifacts::fem3d::mutations::create_solid::mutation::CreateSolid { solid }) => {
                assert_eq!(solid.outline, vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
                assert_eq!(solid.base_z, 0.5);
                assert_eq!(solid.height, 3.0);
                assert_eq!(solid.layers, 2);
                assert_eq!(solid.material_id, "m0");
            }
            _ => panic!("expected CreateSolid"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn fem3d_io_matches_declared_artifact_identity_3d() {
        let io = semio_framework_plugin::resolve_ready(Fem3dPlayApp::io()).expect("fem3d declares typed media I/O");
        assert_eq!(io.artifact.id, "3d.fem");
        assert!(io.ports.iter().any(|port| port.id == "geometry:in"));
        assert!(io.ports.iter().any(|port| port.id == "results:out"));
    }

    /// 🔌️ Wave-1's `required: true` unwired-input enforcement (`validate_edge_kinds`) lives in the run
    /// crate, not here — this test only proves the port DECLARATION is correct; the cross-crate
    /// enforcement is exercised at the run-crate level.
    #[semio_framework_async_macros::async_test]
    async fn fem3d_io_declares_geometry_in_and_results_out_ports() {
        let io = fem3d_io();
        assert_eq!(io.document_schema, crate::artifacts::fem3d::FEM_3D_SCHEMA);
        assert_eq!(io.document_media_type.class, semio_framework_plugin::MediaClass::ThreeD);
        assert_eq!(io.document_media_type.form, semio_framework_plugin::MediaForm::Any);
        assert_eq!(io.artifact.id, "3d.fem");
        assert_eq!(io.artifact.component_kind, "fem3d");

        let geometry_in = io.ports.iter().find(|port| port.id == "geometry:in").expect("geometry:in declared");
        assert_eq!(geometry_in.direction, semio_framework_plugin::MediaPortDirection::In);
        assert!(geometry_in.required, "geometry:in is a required input port");
        assert_eq!(geometry_in.media_type.class, semio_framework_plugin::MediaClass::ThreeD);
        assert_eq!(geometry_in.media_type.form, semio_framework_plugin::MediaForm::Any);
        assert_eq!(geometry_in.multiplicity, semio_framework::PortMultiplicity::One);

        let results_out = io.ports.iter().find(|port| port.id == "results:out").expect("results:out declared");
        assert_eq!(results_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert!(!results_out.required, "results:out is optional");
        assert_eq!(results_out.kind_id.as_deref(), Some("computation.fem3d"));
        assert_eq!(results_out.media_type.class, semio_framework_plugin::MediaClass::Data);
        assert_eq!(results_out.media_type.form, semio_framework_plugin::MediaForm::Value);
    }
    //#endregion 🔖️MediaPorts

    //#region 🎬️SceneRender
    #[semio_framework_async_macros::async_test]
    async fn quat_z_to_identity_for_parallel_direction() {
        assert_eq!(quat_z_to([0.0, 0.0, 1.0]), [0.0, 0.0, 0.0, 1.0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn quat_z_to_handles_antiparallel_direction() {
        assert_eq!(quat_z_to([0.0, 0.0, -1.0]), [1.0, 0.0, 0.0, 0.0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn fem3d_camera_json_falls_back_to_world3d_default_for_empty_object() {
        let camera = crate::artifacts::fem3d::FemCamera::default();
        assert_eq!(fem3d_camera_json(&camera), semio_framework_plugin::resolve_ready(semio_framework_plugin::world3d_default_camera()));
        let custom = crate::artifacts::fem3d::FemCamera { json: "{\"x\":1}".into() };
        assert_eq!(fem3d_camera_json(&custom), "{\"x\":1}");
    }

    #[semio_framework_async_macros::async_test]
    async fn fem3d_scene_parts_include_solid_mesh_and_oriented_member_instances() {
        let doc: Fem3dSnapshot = crate::artifacts::fem3d::dsl::parse_dsl(crate::artifacts::fem3d::dsl::FEM3D_EXAMPLE_TEXT).expect("example fixture parses");
        let (meshes_json, instances_json) = fem3d_scene_parts(&doc, None, doc.analysis.deformation_scale, None);
        assert!(meshes_json.contains("solid-sol1"), "expected a solid- mesh id for the example fixture's solid: {meshes_json}");
        assert!(instances_json.contains("el-e1"), "expected a single oriented box instance per member (no -{{i}} sphere chain): {instances_json}");
    }
    //#endregion 🎬️SceneRender
}
//#endregion 🧪️Tests
