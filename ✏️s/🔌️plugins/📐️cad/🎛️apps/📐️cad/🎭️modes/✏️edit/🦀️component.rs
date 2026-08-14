//! ✏️ CAD play app — the `edit` mode: the quad world-3d layout (shape/building over
//! energy/structure-classic) plus the world-scene, selection-overlay and engagement-HUD builders its
//! four windows share. Each window binds these to its own pane; nothing here is pane-specific.

use crate::apps::cad::modes::edit::windows::{building, energy, shape, structure_classic};
use crate::apps::cad::terminology::CadLabels;
use crate::apps::cad::{cad_action, cad_pane_camera_runtime, cad_pane_suffix, camera_json, CadPlayRuntime, CadPlayView, CAD_DISLOCATE_UTILITY_ID, CAD_FALLBACK_MESH_KIND, CAD_INTERACTION_DOMAIN, CAD_PLAY_APP_ID};
use crate::apps::cad::config::CadDislocateOptions;
use crate::apps::cad::engine::interaction::{keyed_transitions, list_interactions_for_model_definition, preview_display_items};
use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::{collect_mesh_urls, object_mesh_data, object_scale_json, resolve_object_mesh_url};
use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::{CadGeometry, CadObject};
use crate::artifacts::cad::{CadPaneId, CadSnapshot};
use semio_framework_plugin::{
    build_world_3d_scene, mesh_from_kind, world3d_chunking_json, world3d_environment_json, world3d_mesh_id_from_url, world3d_scene_extended, world3d_selection_json, LocalizedLabel, ModeDefinition, UiNode, WindowEngagement,
    WindowEngagementInput, WindowEngagementPossible, WindowEngagementStatus, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode,
};
use serde_json::{json, Value};

pub const CAD_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::cad::create_cad_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: CAD_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One quadrant of the quad layout: a stack holding a single window kind.
fn cad_window_stack(window_kind_id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size,
        active_window_kind_id: None,
        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None }],
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
pub fn world_instances_json(objects: &[CadObject], runtime: &CadPlayRuntime) -> String {
    let instances: Vec<Value> = objects
        .iter()
        .filter(|object| object.visible)
        .map(|object| {
            let mesh_id = resolve_object_mesh_url(object).map_or_else(|| object.id.clone(), |url| world3d_mesh_id_from_url(&url));
            let selected = false;
            let hovered = instance_is_component_hovered(runtime, &object.id);
            json!({
                "id": object.id,
                "meshId": mesh_id,
                "position": [
                    object.origin.first().copied().unwrap_or(0.0),
                    object.origin.get(1).copied().unwrap_or(0.0),
                    object.origin.get(2).copied().unwrap_or(0.0),
                ],
                "rotation": object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": object_scale_json(object),
                "label": object.label,
                "color": if selected { "#3b82f6" } else { "#64748b" },
                "selected": selected,
                "hovered": hovered,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

pub fn world_meshes_json(objects: &[CadObject], geometry: Option<&CadGeometry>) -> String {
    let urls = collect_mesh_urls(objects);
    if !urls.is_empty() {
        return semio_framework_plugin::world3d_meshes_json_from_urls(&urls);
    }
    let meshes: Vec<Value> = objects
        .iter()
        .filter(|object| object.visible)
        .map(|object| {
            let data = object_mesh_data(object, geometry);
            json!({ "id": object.id, "data": data })
        })
        .collect();
    if meshes.is_empty() {
        let data = mesh_from_kind(CAD_FALLBACK_MESH_KIND);
        return serde_json::to_string(&[json!({ "id": CAD_FALLBACK_MESH_KIND, "data": data })]).unwrap_or_else(|_| "[]".into());
    }
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

/// ⚠️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): mesh object/vertex/edge/face
/// selection AND hover are the framework-owned `"cad"` interaction domain now, unreachable at this
/// render boundary (see `instance_is_component_hovered`'s doc comment) — `selectionMode`/
/// `granularity`/`targets`/`componentIds`/`activeObjectId`/`hoveredComponent` are no longer emitted
/// here (the client no longer needs them from this payload either: `interaction_domain`-bound UI
/// gets its presence stamped by the framework wrapper post-render). `gumball_active` is always
/// `false` for the same reason.
pub fn world_selection_json(_document: &CadSnapshot, runtime: &CadPlayRuntime, active_utility: Option<&str>, options: CadDislocateOptions) -> String {
    let mut value: Value = serde_json::from_str(&world3d_selection_json("rectangle", &[], None)).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        let active = gumball_active(runtime, active_utility, options);
        if active_utility == Some(CAD_DISLOCATE_UTILITY_ID) {
            object.insert("transformMode".into(), json!("transform"));
            object.insert(
                "gumballConfig".into(),
                json!({
                    "moveAxes": options.move_enabled,
                    "movePlanes": options.move_enabled,
                    "rotate": options.rotate_enabled,
                    "scaleAxes": false,
                    "scalePlanes": false,
                    "scaleUniform": false,
                }),
            );
        }
        object.insert("gumballActive".into(), json!(active));
        object.insert("engagementSessionActive".into(), json!(runtime.engagement_session.is_some()));
        object.insert("showEdges".into(), json!(true));
        if let Some(reference_id) = runtime.selected_reference_id.as_deref() {
            object.insert("referenceSelectedId".into(), json!(reference_id));
        }
    }
    value.to_string()
}

pub fn world_references_json(document: &CadSnapshot, pane: CadPaneId) -> Option<String> {
    let references = document.references_by_model_definition_id.get(pane.model_definition_id())?;
    if references.is_empty() {
        return None;
    }
    let records: Vec<Value> = references
        .iter()
        .filter(|reference| !reference.hidden)
        .map(|reference| {
            json!({
                "id": reference.id,
                "url": reference.source_url,
                "origin": reference.origin,
                "widthWorld": if reference.width_world > 0.0 { reference.width_world } else { 1.0 },
                "locked": reference.locked,
                "hidden": reference.hidden,
                "opacity": reference.opacity.unwrap_or(1.0),
            })
        })
        .collect();
    Some(serde_json::to_string(&records).unwrap_or_else(|_| "[]".into()))
}

/// ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `cad_pane_objects`/
/// `cad_pane_geometry` are retired — a pane's object/geometry data lives inside its composed
/// `s.stdio.semio.model` CHILD document now (unresolved at this render boundary; see
/// `🔖️Composition` in `🏪️store/🦀️component.rs`). Renders an empty object list per pane until a
/// resolved-child-content render path exists; `world_instances_json`/`world_meshes_json` themselves
/// are untouched real functions, just fed an empty slice here.
pub fn build_world_scene_for_pane(envelope: &CadPlayView, pane: CadPaneId, surface_id: &str, active_utility: Option<&str>, options: CadDislocateOptions) -> UiNode {
    let objects: &[CadObject] = &[];
    let preview = envelope.runtime.engagement_session.as_ref().filter(|session| session.pane == pane).map(preview_display_items).filter(|items| !items.is_empty()).map(|items| serde_json::to_string(&items).unwrap_or_else(|_| "[]".into()));
    build_world_3d_scene(
        surface_id,
        CAD_PLAY_APP_ID,
        world3d_scene_extended(
            camera_json(cad_pane_camera_runtime(&envelope.runtime, pane)),
            world_meshes_json(objects, None),
            world_instances_json(objects, &envelope.runtime),
            world_selection_json(&envelope.document, &envelope.runtime, active_utility, options),
            None,
            None,
            None,
            world_references_json(&envelope.document, pane),
            None,
            None,
            preview,
            None,
            Some(world3d_chunking_json(256.0, 8000.0)),
            Some(world3d_environment_json(&envelope.runtime.sun)),
            None,
            None,
            None,
            None,
            None,
            // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): all four CAD panes bind the
            // same `CAD_INTERACTION_DOMAIN` (see `create_cad_app`'s `.window_kind_interactions` calls)
            // — a plain whole-object pick/hover on this shared world-3d surface targets that domain's
            // `"object"` granularity, not the OS's own bare `world` board domain.
            Some(CAD_INTERACTION_DOMAIN.into()),
            Some("object".into()),
        ),
    )
}
//#endregion 🔖️WorldScene

//#region 🔖️Engagement
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
                    Some(json!({
                        "pane": cad_pane_suffix(pane),
                        "possibleId": transition.event_kind,
                    })),
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
                action: Some(cad_action("engagementPossibleSelect", Some(json!({ "pane": cad_pane_suffix(pane), "possibleId": entry.id.clone() })))),
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
            on_change: Some(cad_action("engagementInput", Some(json!({ "pane": cad_pane_suffix(pane) })))),
            on_submit: Some(cad_action("engagementSubmit", Some(json!({ "pane": cad_pane_suffix(pane) })))),
            on_repeat_last: Some(cad_action("engagementRepeatLast", Some(json!({ "pane": cad_pane_suffix(pane) })))),
            on_abort: Some(cad_action("engagementAbort", Some(json!({ "pane": cad_pane_suffix(pane) })))),
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
