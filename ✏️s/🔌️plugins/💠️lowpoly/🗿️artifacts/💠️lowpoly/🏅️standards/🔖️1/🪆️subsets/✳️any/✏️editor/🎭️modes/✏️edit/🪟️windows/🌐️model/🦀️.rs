//! 🌐️ Lowpoly play app — the Model window: the live 3D world-3d mesh scene (every mesh-editing/
//! transform/UV-unwrap operation runs here; paint operations are scoped on BOTH this window and the UV
//! window since the paint utilities apply to both).

use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::engine::LowpolyDocument;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{document_object_row_id, euler_degrees_to_quaternion, is_paint_utility, LowpolyView, LowpolyWorldSelection, MESH_GRANULARITY_OBJECT, MESH_INTERACTION_DOMAIN};
use semio_framework_3d::mesh::{EdgeId, FaceId, VertexId};
use crate::editor::lowpoly::{lowpoly_window_engagement, lowpoly_window_measures};
use crate::schema::mesh_data_from_transfer;
use semio_framework_plugin::{scene_surface, world3d_camera_json, world3d_scene, InteractionRef, PluginAssemblyError, SurfaceKind, UtilityRef, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions};
use std::collections::HashMap;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_WINDOW_MAIN: &str = "lowpoly-main";
pub const LOWPOLY_PLAY_BODY_MAIN: &str = "lowpoly.play.main";
const LOWPOLY_PLAY_SURFACE_MAIN: &str = "lowpoly.play.main";
/// 🧰️ The transform gumball utility a Model window falls back to when the host hasn't set an active utility.
pub const LOWPOLY_TRANSFORM_UTILITY_DEFAULT: &str = "move";

/// 📇️ Every action this window scopes — mesh-editing/transform/UV-unwrap operations plus the paint
/// operations it shares with the UV window.
pub const LOWPOLY_MAIN_ACTIONS: &[&str] = &[
    "addPrimitive",
    "patchObject",
    "extrude",
    "inset",
    "bevel",
    "loopCut",
    "subdivide",
    "triangulate",
    "mirror",
    "decimate",
    "flipFaces",
    "merge",
    "dissolve",
    "snap",
    "toggleSmooth",
    "unwrapActive",
    "markUvSeam",
    "clearSeam",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "transformBegin",
    "transformEnd",
    "addPaintLayer",
    "paintStrokeEnd",
    "paintFill",
    "fillBucket",
    "exportMesh",
    "loadMeshRequest",
    "importMeshFile",
];
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::lowpoly::create_lowpoly_app`.
pub fn definition() -> WindowKindDefinition {
    let projection = crate::schema::default_snapshot();
    let config = LowpolyConfig::default();
    let labels = semio_framework_plugin::resolve_labels::<LowpolyLabels>(&semio_framework_plugin::ViewModel::default());
    let engagement = lowpoly_window_engagement(LowpolyView { snapshot: &projection, config: &config }, LOWPOLY_TRANSFORM_UTILITY_DEFAULT, labels);
    WindowKindDefinition {
        id: LOWPOLY_PLAY_WINDOW_MAIN.into(),
        label: semio_framework_plugin::LocalizedLabel::native("Model", "Modell"),
        body_key: LOWPOLY_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "lowpoly-model".into(),
        // 🎚️ `measures` stays empty here: measures are config-derived per frame by
        // `ArtifactApp::window_measures`, never frozen into the manifest.
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::Some(engagement) },
        actions: Vec::new(),
        utilities: ["move", "rotate", "scale", "brush", "eraser", "fill", "eyedropper"].iter().map(|id| UtilityRef::from(*id)).collect(),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "mesh" interaction domain —
        // only the Model window selects/hovers mesh components; the UV window paints textures.
        interactions: vec![InteractionRef::new(MESH_INTERACTION_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from the app-level `🛠️options/*` shared by
/// both windows (see the master ticket's TEMPLATE.md §12.2 pattern).
pub fn window_measures(config: &LowpolyConfig, labels: &LowpolyLabels) -> Vec<WindowMeasure> {
    lowpoly_window_measures(config, labels, &crate::editor::lowpoly::options::select::SelectState::default())
}
//#endregion 🔖️Definition

//#region 🔖️Scene
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the mesh domain's selection/hover is
/// framework-owned `InteractionState` now, never `LowpolyConfig` — and `ArtifactApp::render` (unlike
/// `handle`/`copy_fragment`/`cut_operations`) is not threaded an `InteractionView` this wave, so this
/// scene JSON can no longer embed a live selection/hover/gumball summary itself (deleted:
/// `granularity`/`targets`/`componentIds`/`selectionMode`/`selectionMergeMode`/`hoveredComponent`/
/// `gumballActive`/`gumballTarget`, plus the per-instance `selected`/`hovered` flags below). The shell
/// renders every peer's (and the local) selection/hover generically off the SAME "mesh" domain this
/// window declares via `.window_kind_interactions` — see `📋️master.md`'s UI section ("scene payloads
/// fed from InteractionView") — so this app never needs to re-embed it.
///
/// 🕹️ 2026-09-17 (ticket 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS): `render_with_request_context`
/// now threads the live `InteractionView`, so the scene carries what `World3dHost` needs to pick and
/// paint the mesh domain — `targets`/`selectionMode` (the granularity the next pick addresses),
/// `componentIds` on the active object, the selected object `ids`, and the gumball anchor.
fn world_selection_json_for(view: LowpolyView<'_>, loaded: &LowpolyDocument, active_utility: &str, selection: &LowpolyWorldSelection) -> String {
    let config = view.config;
    let paint = is_paint_utility(active_utility);
    let interaction_mode = if paint { "paint" } else { "model" };
    let granularity = selection.granularity.as_str();
    let object_level = granularity == MESH_GRANULARITY_OBJECT;
    let host_mode = if object_level { "mesh" } else { granularity };
    let targets = dsl::DslValue::object([
        ("mesh".to_string(), dsl::DslValue::Bool(object_level)),
        ("vertex".to_string(), dsl::DslValue::Bool(granularity == "vertex")),
        ("edge".to_string(), dsl::DslValue::Bool(granularity == "edge")),
        ("face".to_string(), dsl::DslValue::Bool(granularity == "face")),
    ]);
    let pivot = gumball_pivot(view, loaded, selection);
    let mut entries = vec![
        ("transformMode".to_string(), dsl::DslValue::String(active_utility.to_string())),
        ("interactionMode".to_string(), dsl::DslValue::String(interaction_mode.to_string())),
        ("activeObjectId".to_string(), dsl::DslValue::String(selection.active_object_id.clone())),
        ("showEdges".to_string(), dsl::DslValue::Bool(config.show_edges)),
        ("targets".to_string(), targets),
        ("selectionMode".to_string(), dsl::DslValue::String(host_mode.to_string())),
        ("granularity".to_string(), dsl::DslValue::String(host_mode.to_string())),
        ("ids".to_string(), dsl::DslValue::Array(selection.object_ids.iter().cloned().map(dsl::DslValue::String).collect())),
        ("componentIds".to_string(), dsl::DslValue::Array(selection.component_ids.iter().map(|id| dsl::DslValue::Number(dsl::Number::UInt(u64::from(*id)))).collect())),
        ("gumballActive".to_string(), dsl::DslValue::Bool(!paint && pivot.is_some())),
    ];
    if let Some(pivot) = pivot {
        entries.push(("gumballTarget".to_string(), dsl::ToValue::to_value(&pivot)));
    }
    // 🧲️ The composable gumball: every handle group the window toggles left on shows at once
    // (`UnifiedGumball` reads `gumballConfig` over the single-mode `transformMode` fallback).
    let handles = crate::editor::lowpoly::options::gumball::GumballHandles::from_config(config);
    entries.push((
        "gumballConfig".to_string(),
        dsl::DslValue::object([
            ("moveAxes".to_string(), dsl::DslValue::Bool(handles.r#move)),
            ("movePlanes".to_string(), dsl::DslValue::Bool(handles.r#move)),
            ("rotate".to_string(), dsl::DslValue::Bool(handles.rotate)),
            ("scaleAxes".to_string(), dsl::DslValue::Bool(handles.scale)),
            ("scalePlanes".to_string(), dsl::DslValue::Bool(handles.scale)),
            ("scaleUniform".to_string(), dsl::DslValue::Bool(handles.scale)),
        ]),
    ));
    // 🖱️ The mesh domain's pointer hover, echoed for the host's overlays: the hovered instance and,
    // when the pointer is over a component, `{objectId, mode, id}` for its vertex/edge/face highlight.
    if let Some(hovered) = &selection.hovered_object_id {
        entries.push(("hoveredId".to_string(), dsl::DslValue::String(hovered.clone())));
    }
    if let Some(component) = &selection.hovered_component {
        entries.push((
            "hoveredComponent".to_string(),
            dsl::DslValue::object([
                ("objectId".to_string(), dsl::DslValue::String(component.object_id.clone())),
                ("mode".to_string(), dsl::DslValue::String(component.mode.clone())),
                ("id".to_string(), dsl::DslValue::Number(dsl::Number::UInt(u64::from(component.id)))),
            ]),
        ));
    }
    dsl::json::to_json_string(&dsl::DslValue::Object(entries))
}

/// 🧲️ World-space centroid of the current selection (selected components on the active object, else
/// the selected objects' vertices), `None` when nothing is selected — the gumball's anchor.
fn gumball_pivot(view: LowpolyView<'_>, loaded: &LowpolyDocument, selection: &LowpolyWorldSelection) -> Option<[f64; 3]> {
    let mut sum = [0.0_f64; 3];
    let mut count = 0_usize;
    let mut add = |object_index: usize, vertices: &[VertexId]| {
        let (Some(object), Some(mesh)) = (view.snapshot.objects.get(object_index), loaded.mesh_at(object_index)) else { return };
        let rotation = euler_degrees_to_quaternion(object.transform.rotation);
        for vertex in vertices {
            let Ok(position) = mesh.vertex_position(*vertex) else { continue };
            let scaled = [f64::from(position.0[0] * object.transform.scale[0]), f64::from(position.0[1] * object.transform.scale[1]), f64::from(position.0[2] * object.transform.scale[2])];
            let rotated = rotate(rotation, scaled);
            for axis in 0..3 {
                sum[axis] += rotated[axis] + f64::from(object.transform.position[axis]);
            }
            count += 1;
        }
    };
    if selection.granularity == MESH_GRANULARITY_OBJECT {
        for object_id in &selection.object_ids {
            let Some(index) = view.snapshot.objects.iter().position(|object| &object.id == object_id) else { continue };
            let Some(mesh) = loaded.mesh_at(index) else { continue };
            let vertices: Vec<VertexId> = (0..mesh.vertex_count()).map(|vertex| VertexId(vertex as u32)).collect();
            add(index, &vertices);
        }
    } else if let Some(index) = view.snapshot.objects.iter().position(|object| object.id == selection.active_object_id) {
        if let Some(mesh) = loaded.mesh_at(index) {
            let mut vertices = Vec::new();
            for id in &selection.component_ids {
                match selection.granularity.as_str() {
                    "vertex" => vertices.push(VertexId(*id)),
                    "edge" => vertices.extend(mesh.edge_endpoints(EdgeId(*id)).ok().map(|(a, b)| [a, b]).into_iter().flatten()),
                    "face" => vertices.extend(mesh.face_vertex_ids(FaceId(*id)).unwrap_or_default()),
                    _ => {}
                }
            }
            vertices.sort_by_key(|vertex| vertex.0);
            vertices.dedup_by_key(|vertex| vertex.0);
            add(index, &vertices);
        }
    }
    (count > 0).then(|| sum.map(|value| value / count as f64))
}

fn rotate(q: [f64; 4], v: [f64; 3]) -> [f64; 3] {
    let [x, y, z, w] = q;
    let t = [2.0 * (y * v[2] - z * v[1]), 2.0 * (z * v[0] - x * v[2]), 2.0 * (x * v[1] - y * v[0])];
    [v[0] + w * t[0] + (y * t[2] - z * t[1]), v[1] + w * t[1] + (z * t[0] - x * t[2]), v[2] + w * t[2] + (x * t[1] - y * t[0])]
}

fn world_meshes_json(doc: &LowpolyDocument, texture_cache: &HashMap<String, String>) -> String {
    let items: Vec<dsl::DslValue> = dsl::json::from_json_str(&doc.tessellate_all_json().unwrap_or_else(|_| "[]".into())).unwrap_or_default();
    let meshes: Vec<dsl::DslValue> = items
        .iter()
        .filter_map(|item| {
            let id = item.get("id")?.as_str()?;
            let tessellation = item.get("tessellation")?;
            let texture = texture_cache.get(id).cloned();
            Some(dsl::DslValue::object([("id".to_string(), dsl::DslValue::String(id.to_string())), ("data".to_string(), dsl::ToValue::to_value(&mesh_data_from_transfer(tessellation, texture)))]))
        })
        .collect();
    dsl::json::to_json_string(&meshes)
}

/// 🕹️ `selected`/`hovered` per-instance flags are DELETED — see `world_selection_json_for`'s doc: the
/// shell overlays the mesh domain's live selection/hover generically now.
fn world_instances_json(view: LowpolyView<'_>) -> String {
    let instances: Vec<dsl::DslValue> = view
        .snapshot
        .objects
        .iter()
        .map(|object| {
            let rotation = euler_degrees_to_quaternion(object.transform.rotation);
            let position: [f64; 3] = [object.transform.position[0] as f64, object.transform.position[1] as f64, object.transform.position[2] as f64];
            let scale: [f64; 3] = [object.transform.scale[0] as f64, object.transform.scale[1] as f64, object.transform.scale[2] as f64];
            dsl::DslValue::object([
                ("id".to_string(), dsl::DslValue::String(object.id.clone())),
                ("meshId".to_string(), dsl::DslValue::String(object.id.clone())),
                ("interactionId".to_string(), dsl::DslValue::String(document_object_row_id(&object.id))),
                ("interactionGranularityId".to_string(), dsl::DslValue::String(MESH_GRANULARITY_OBJECT.to_string())),
                ("position".to_string(), dsl::ToValue::to_value(&position)),
                ("rotation".to_string(), dsl::ToValue::to_value(&rotation)),
                ("scale".to_string(), dsl::ToValue::to_value(&scale)),
                ("label".to_string(), dsl::DslValue::String(object.name.clone())),
                ("smoothShading".to_string(), dsl::DslValue::Bool(object.smooth_shading)),
            ])
        })
        .collect();
    dsl::json::to_json_string(&instances)
}

pub fn render(view: LowpolyView<'_>, loaded: Option<&LowpolyDocument>, active_utility: &str, texture_cache: &HashMap<String, String>, selection: &LowpolyWorldSelection) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let config = view.config;
    match loaded {
        Some(loaded) => {
            let mut scene = world3d_scene(
                world3d_camera_json(config.world_camera_position, config.world_camera_target, config.world_camera_fov),
                world_meshes_json(loaded, texture_cache),
                world_instances_json(view),
                world_selection_json_for(view, loaded, active_utility, selection),
                &crate::editor::lowpoly::config::lowpoly_sun_config(config),
            );
            // 🕹️ Bound to the "mesh" domain: object hits resolve through each instance's `interactionId`,
            // component hits through `<interactionId>.<granularity>.<id>` (`World3dHost`).
            scene.domain_id = Some(MESH_INTERACTION_DOMAIN.into());
            scene.domain_granularity_id = Some(MESH_GRANULARITY_OBJECT.into());
            scene_surface(LOWPOLY_PLAY_SURFACE_MAIN, semio_framework_ui_contract::SurfaceKind::World3d, &scene)
        }
        None => semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data("Failed to load lowpoly document")).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly main window failed-load text admission failed")),
    }
}
//#endregion 🔖️Scene

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
