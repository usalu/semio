//! ✏️ CAD play app — the `edit` mode: the quad world-3d layout (shape/building over
//! energy/structure-classic) plus the world-scene, selection-overlay and engagement-HUD builders its
//! four windows share. Each window binds these to its own pane; nothing here is pane-specific.

use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::{CadGeometry, CadObject};
use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::{collect_mesh_urls, object_mesh_data, object_scale_json, resolve_object_mesh_url};
use crate::artifacts::cad::{CadPaneId, CadSnapshot, CadWorkingScene};
use crate::editor::cad::config::CadDislocateOptions;
use crate::editor::cad::engine::interaction::{keyed_transitions, list_interactions_for_model_definition, preview_display_items};
use crate::editor::cad::modes::edit::windows::{building, energy, shape, structure_classic};
use crate::editor::cad::terminology::CadLabels;
use crate::editor::cad::{cad_pane_camera_runtime, cad_pane_suffix, camera_json, CadPlayRuntime, CadPlayView, CAD_DISLOCATE_UTILITY_ID, CAD_FALLBACK_MESH_KIND, CAD_INTERACTION_DOMAIN, CAD_PLAY_APP_ID};
use semio_framework_plugin::app::WindowKit;
use semio_framework_plugin::{
    mesh_from_kind, world3d_mesh_id_from_url, world3d_selection_json, ActionDescriptor, BuiltNode, LocalizedLabel, MeshView, MeshWindowKit, ModeDefinition, UiAssemblyResult, WindowEngagement, WindowEngagementInput,
    WindowEngagementPossible, WindowEngagementStatus, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode,
};
use protocol::DslValue;

pub const CAD_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::cad::create_cad_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: CAD_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One quadrant of the quad layout: a stack holding a single window kind.
fn cad_window_stack(window_kind_id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size,
        active_window_kind_id: None,
        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }],
    })
}

/// @emoji 🪟️ Quad play layout: shape/building left column, energy/structure classic right column.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Axis(WindowLayoutAxisNode { kind: "column".into(), size: Some(0.5), children: vec![cad_window_stack(shape::WINDOW_KIND_ID, "Shape", Some(0.5)), cad_window_stack(building::WINDOW_KIND_ID, "Building", Some(0.5))] }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "column".into(),
                    size: Some(0.5),
                    children: vec![cad_window_stack(energy::WINDOW_KIND_ID, "Energy", Some(0.5)), cad_window_stack(structure_classic::WINDOW_KIND_ID, "Structure Classic", Some(0.5))],
                }),
            ],
        }),
    }
}
//#endregion 🔖️Definition

//#region 🔖️WorldScene
/// 🐁️ ⚠️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): mesh hover is the framework-owned
/// `"cad"` interaction domain now, and `ArtifactApp::render` (unlike `handle`/`copy_fragment`/
/// `cut_operations`) has NO `InteractionView` parameter — a per-object hover tint in the World3d
/// scene payload is unreachable at this render boundary. Documented reduced-fidelity gap, matching
/// this file's own pre-existing `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` gap notes; always `false` until
/// a future wave threads render-time interaction state through.
pub fn instance_is_component_hovered(_runtime: &CadPlayRuntime, _object_id: &str) -> bool {
    false
}

/// @emoji 🕹️ Whether this window's active Dislocate utility has a visible handle for the selection.
/// ⚠️ Same `render`-has-no-`InteractionView` gap as `instance_is_component_hovered` — the gumball
/// cannot know the current mesh selection here, so it never shows. Documented gap.
pub fn gumball_active(_runtime: &CadPlayRuntime, _active_utility: Option<&str>, _options: CadDislocateOptions) -> bool {
    false
}

/// ⚠️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `selected` is always `false` here — see
/// `instance_is_component_hovered`'s doc comment for why `render` cannot know the current mesh
/// selection. Documented reduced-fidelity gap.
/// 🌉️ `f64_array_value` — small local helper turning a fixed-size point/vector array into a
/// `DslValue::Array` of floats (mirrors `vec3_json` in `⚙️engine/🕹️interaction/🦀️.rs`).
fn f64_array_value(values: &[f64]) -> DslValue {
    DslValue::Array(values.iter().map(|v| DslValue::float(*v)).collect())
}

/// 🌉️ `MeshData` (`semio_framework_plugin`) carries its own first-party `From<MeshData> for
/// pack::json::Value` — reached here through `protocol`'s `os_pack` re-export of the same `pack`
/// crate, never `serde_json`. Bridged once, here, at the point each mesh payload is assembled.
fn mesh_data_to_dsl(data: &semio_framework_plugin::MeshData) -> DslValue {
    protocol::os_pack::json::to_dsl_value(&protocol::os_pack::json::Value::from(data.clone()))
}

pub(crate) fn world_instances_json(objects: &[CadObject], runtime: &CadPlayRuntime) -> String {
    let instances: Vec<DslValue> = objects
        .iter()
        .filter(|object| object.visible)
        .map(|object| {
            let mesh_id = resolve_object_mesh_url(object).map_or_else(|| object.id.clone(), |url| world3d_mesh_id_from_url(&url));
            let selected = false;
            let hovered = instance_is_component_hovered(runtime, &object.id);
            DslValue::object([
                ("id".to_string(), DslValue::String(object.id.clone())),
                ("meshId".to_string(), DslValue::String(mesh_id)),
                ("position".to_string(), f64_array_value(&object.origin)),
                ("rotation".to_string(), f64_array_value(&object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]))),
                ("scale".to_string(), f64_array_value(&object_scale_json(object))),
                ("label".to_string(), DslValue::String(object.label.clone())),
                ("color".to_string(), DslValue::String(if selected { "#3b82f6" } else { "#64748b" }.to_string())),
                ("selected".to_string(), DslValue::Bool(selected)),
                ("hovered".to_string(), DslValue::Bool(hovered)),
            ])
        })
        .collect();
    protocol::json::to_json_string(&instances)
}

pub(crate) fn world_meshes_json(objects: &[CadObject], geometry: Option<&CadGeometry>) -> String {
    let urls = collect_mesh_urls(objects);
    if !urls.is_empty() {
        return semio_framework_plugin::world3d_meshes_json_from_urls(&urls);
    }
    let meshes: Vec<DslValue> = objects
        .iter()
        .filter(|object| object.visible)
        .map(|object| {
            let data = object_mesh_data(object, geometry);
            DslValue::object([("id".to_string(), DslValue::String(object.id.clone())), ("data".to_string(), mesh_data_to_dsl(&data))])
        })
        .collect();
    if meshes.is_empty() {
        let data = mesh_from_kind(CAD_FALLBACK_MESH_KIND);
        let fallback = vec![DslValue::object([("id".to_string(), DslValue::String(CAD_FALLBACK_MESH_KIND.to_string())), ("data".to_string(), mesh_data_to_dsl(&data))])];
        return protocol::json::to_json_string(&fallback);
    }
    protocol::json::to_json_string(&meshes)
}

/// ⚠️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): mesh object/vertex/edge/face
/// selection AND hover are the framework-owned `"cad"` interaction domain now, unreachable at this
/// render boundary (see `instance_is_component_hovered`'s doc comment) — `selectionMode`/
/// `granularity`/`targets`/`componentIds`/`activeObjectId`/`hoveredComponent` are no longer emitted
/// here (the client no longer needs them from this payload either: `interaction_domain`-bound UI
/// gets its presence stamped by the framework wrapper post-render). `gumball_active` is always
/// `false` for the same reason.
/// 🌉️ Insert-or-overwrite into a `DslValue::Object`'s entry list — `DslValue::Object` is a plain
/// `Vec<(String, DslValue)>` (no `Map`-like `.insert`), so this is the mutable-upsert primitive
/// every JSON-mutation site in this file shares.
fn dsl_object_upsert(entries: &mut Vec<(String, DslValue)>, key: &str, value: DslValue) {
    if let Some(existing) = entries.iter_mut().find(|(k, _)| k == key) {
        existing.1 = value;
    } else {
        entries.push((key.to_string(), value));
    }
}

pub fn world_selection_json(_document: &CadSnapshot, runtime: &CadPlayRuntime, active_utility: Option<&str>, options: CadDislocateOptions) -> String {
    let mut value: DslValue = protocol::json::from_json_str(&world3d_selection_json("rectangle", &[], None)).unwrap_or_else(|_| DslValue::object(Vec::new()));
    if let DslValue::Object(entries) = &mut value {
        let active = gumball_active(runtime, active_utility, options);
        if active_utility == Some(CAD_DISLOCATE_UTILITY_ID) {
            dsl_object_upsert(entries, "transformMode", DslValue::String("transform".into()));
            dsl_object_upsert(
                entries,
                "gumballConfig",
                DslValue::object([
                    ("moveAxes".to_string(), DslValue::Bool(options.move_enabled)),
                    ("movePlanes".to_string(), DslValue::Bool(options.move_enabled)),
                    ("rotate".to_string(), DslValue::Bool(options.rotate_enabled)),
                    ("scaleAxes".to_string(), DslValue::Bool(false)),
                    ("scalePlanes".to_string(), DslValue::Bool(false)),
                    ("scaleUniform".to_string(), DslValue::Bool(false)),
                ]),
            );
        }
        dsl_object_upsert(entries, "gumballActive", DslValue::Bool(active));
        dsl_object_upsert(entries, "engagementSessionActive", DslValue::Bool(runtime.engagement_session.is_some()));
        dsl_object_upsert(entries, "showEdges", DslValue::Bool(true));
        if let Some(reference_id) = runtime.selected_reference_id.as_deref() {
            dsl_object_upsert(entries, "referenceSelectedId", DslValue::String(reference_id.to_string()));
        }
    }
    protocol::json::to_json_string(&value)
}

pub fn world_references_json(document: &CadSnapshot, pane: CadPaneId) -> Option<String> {
    let references = document.references_by_model_definition_id.get(pane.model_definition_id())?;
    if references.is_empty() {
        return None;
    }
    let records: Vec<DslValue> = references
        .iter()
        .filter(|reference| !reference.hidden)
        .map(|reference| {
            DslValue::object([
                ("id".to_string(), DslValue::String(reference.id.clone())),
                ("url".to_string(), DslValue::String(reference.source_url.clone())),
                ("origin".to_string(), f64_array_value(&reference.origin)),
                ("widthWorld".to_string(), DslValue::float(if reference.width_world > 0.0 { reference.width_world } else { 1.0 })),
                ("locked".to_string(), DslValue::Bool(reference.locked)),
                ("hidden".to_string(), DslValue::Bool(reference.hidden)),
                ("opacity".to_string(), DslValue::float(reference.opacity.unwrap_or(1.0))),
            ])
        })
        .collect();
    Some(protocol::json::to_json_string(&records))
}

/// 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: a pane's object/geometry data
/// lives inside its composed `s.stdio.semio.model` CHILD document now — no host-level child
/// resolver exists yet (see `🔖️Composition` in `🏪️store/🦀️.rs`), but the handle's own
/// `ArtifactChild::local_owner` (the same in-process materialization seam `flow`/`dag`/`jack`/
/// `wires`/`sequence` already rely on) carries the `CadWorkingScene` a document builder such as
/// `forest_play_document` attached when it minted the handle. `pane`'s objects/geometry come from
/// there; a handle with no local owner (or none at all) renders an empty pane, never a fabricated one.
pub(crate) fn cad_pane_working_scene(document: &CadSnapshot, pane: CadPaneId) -> Option<std::sync::Arc<CadWorkingScene>> {
    let child = match pane {
        CadPaneId::Shape => document.shape_model.as_ref(),
        CadPaneId::Building => document.building_model.as_ref(),
        CadPaneId::Energy => document.energy_model.as_ref(),
        CadPaneId::StructureClassic => document.structure_classic_model.as_ref(),
    }?;
    child.local_owner::<CadWorkingScene>()
}

pub(crate) fn cad_pane_working_objects(scene: &CadWorkingScene, pane: CadPaneId) -> (&[CadObject], Option<&CadGeometry>) {
    match pane {
        CadPaneId::Shape => (&scene.objects, scene.geometry.as_ref()),
        CadPaneId::Building => (&scene.building_objects, scene.building_geometry.as_ref()),
        CadPaneId::Energy => (&scene.energy_objects, scene.energy_geometry.as_ref()),
        CadPaneId::StructureClassic => (&scene.structure_classic_objects, scene.structure_classic_geometry.as_ref()),
    }
}

pub fn build_world_scene_for_pane(envelope: &CadPlayView, pane: CadPaneId, _surface_id: &str, active_utility: Option<&str>, options: CadDislocateOptions) -> UiAssemblyResult<BuiltNode> {
    let working_scene = cad_pane_working_scene(&envelope.document, pane);
    let empty: &[CadObject] = &[];
    let (objects, geometry) = working_scene.as_deref().map_or((empty, None), |scene| cad_pane_working_objects(scene, pane));
    MeshWindowKit::render(&MeshView {
        camera_json: camera_json(cad_pane_camera_runtime(&envelope.runtime, pane)),
        meshes_json: world_meshes_json(objects, geometry),
        instances_json: world_instances_json(objects, &envelope.runtime),
        selection_json: world_selection_json(&envelope.document, &envelope.runtime, active_utility, options),
    })
}
//#endregion 🔖️WorldScene

//#region 🔖️Engagement
fn cad_action(action: &str, args: Option<DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: CAD_PLAY_APP_ID.into(), action: action.into(), args }
}

pub fn cad_window_engagement(envelope: &CadPlayView, pane: CadPaneId, labels: &CadLabels) -> WindowEngagement {
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): mesh selection is framework-owned
    // now and unreachable at this render boundary (see `instance_is_component_hovered`'s doc
    // comment) — the status HUD can no longer report a live selected-object count. Documented gap.
    let selected_count = 0;
    let model_definition_id = pane.model_definition_id();
    let session_active = envelope.runtime.engagement_session.is_some();
    let possible_engagements: Vec<WindowEngagementPossible> = if let Some(session) = envelope.runtime.engagement_session.as_ref() {
        keyed_transitions(session)
            .into_iter()
            .map(|transition| WindowEngagementPossible {
                id: transition.event_kind.clone(),
                label: transition.label,
                detail: Some(transition.key),
                action: Some(cad_action(
                    "engagementPossibleSelect",
                    Some(DslValue::object([
                        ("pane".to_string(), DslValue::String(cad_pane_suffix(pane).to_string())),
                        ("possibleId".to_string(), DslValue::String(transition.event_kind.clone())),
                    ])),
                )),
            })
            .collect()
    } else {
        list_interactions_for_model_definition(model_definition_id)
            .into_iter()
            .map(|entry| WindowEngagementPossible {
                id: entry.id.clone(),
                label: entry.label.clone(),
                detail: Some(entry.key.clone()),
                action: Some(cad_action(
                    "engagementPossibleSelect",
                    Some(DslValue::object([("pane".to_string(), DslValue::String(cad_pane_suffix(pane).to_string())), ("possibleId".to_string(), DslValue::String(entry.id.clone()))])),
                )),
            })
            .collect()
    };
    let step_text = envelope.runtime.engagement_session.as_ref().map_or_else(|| envelope.runtime.engagement_step.clone(), |session| session.state.clone());
    WindowEngagement {
        session_active: Some(session_active),
        // 🧰️ The move/rotate/scale transform switcher now lives in the framework utility bar (derived
        // from `UtilityDefinition`s + `ViewModel::active_utility_id`); the engagement HUD no longer
        // duplicates it — utilities must have exactly one surface.
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("engagement-input".into()),
            value: Some(envelope.runtime.engagement_input.clone()),
            placeholder: Some(labels.action_placeholder.into()),
            disabled: None,
            on_change: Some(cad_action("engagementInput", Some(DslValue::object([("pane".to_string(), DslValue::String(cad_pane_suffix(pane).to_string()))])))),
            on_submit: Some(cad_action("engagementSubmit", Some(DslValue::object([("pane".to_string(), DslValue::String(cad_pane_suffix(pane).to_string()))])))),
            on_repeat_last: Some(cad_action("engagementRepeatLast", Some(DslValue::object([("pane".to_string(), DslValue::String(cad_pane_suffix(pane).to_string()))])))),
            on_abort: Some(cad_action("engagementAbort", Some(DslValue::object([("pane".to_string(), DslValue::String(cad_pane_suffix(pane).to_string()))])))),
        }),
        control: None,
        controls: None,
        status: Some(vec![
            WindowEngagementStatus { id: "cad-status".into(), text: format!("{selected_count} {}", labels.selected.as_str()) },
            WindowEngagementStatus { id: "cad-step".into(), text: format!("{}: {step_text}", labels.step.as_str()) },
            WindowEngagementStatus { id: "cad-response".into(), text: envelope.runtime.engagement_session.as_ref().and_then(|session| session.last_response.clone()).unwrap_or_else(|| labels.ok.into()) },
        ]),
        possible_engagements: Some(possible_engagements),
    }
}
//#endregion 🔖️Engagement
