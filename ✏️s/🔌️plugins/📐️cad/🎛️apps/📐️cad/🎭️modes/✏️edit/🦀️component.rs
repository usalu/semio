//! ✏️ CAD play app — the `edit` mode: the quad world-3d layout (shape/building over
//! energy/structure-classic) plus the world-scene, selection-overlay and engagement-HUD builders its
//! four windows share. Each window binds these to its own pane; nothing here is pane-specific.

use crate::apps::cad::modes::edit::windows::{building, energy, shape, structure_classic};
use crate::apps::cad::terminology::CadLabels;
use crate::apps::cad::{cad_action, cad_pane_camera_runtime, cad_pane_suffix, camera_json, resolve_active_object_id, CadPlayRuntime, CadPlayView, CAD_DISLOCATE_UTILITY_ID, CAD_FALLBACK_MESH_KIND, CAD_PLAY_APP_ID};
use crate::apps::cad::config::CadDislocateOptions;
use crate::artifacts::cad::engine::interaction::{keyed_transitions, list_interactions_for_model_definition, preview_display_items};
use crate::artifacts::cad::engine::{collect_mesh_urls, object_mesh_data, object_scale_json, resolve_object_mesh_url};
use crate::artifacts::cad::{cad_all_objects, cad_pane_geometry, cad_pane_objects, CadGeometry, CadObject, CadPaneId, CadScene};
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
pub fn instance_is_component_hovered(runtime: &CadPlayRuntime, object_id: &str) -> bool {
    runtime.hovered_target.as_ref().map_or_else(|| runtime.hovered_object_id.as_deref() == Some(object_id), |target| target.mode.as_deref() == Some("mesh") && target.object_id.as_deref() == Some(object_id))
}

/// @emoji 🕹️ Whether this window's active Dislocate utility has a visible handle for the selection.
pub fn gumball_active(runtime: &CadPlayRuntime, active_utility: Option<&str>, options: CadDislocateOptions) -> bool {
    active_utility == Some(CAD_DISLOCATE_UTILITY_ID) && (options.move_enabled || options.rotate_enabled) && (!runtime.selected_object_ids.is_empty() || !runtime.component_selection.ids.is_empty())
}

/// @emoji 🎯️ World-space pivot for the gumball: centroid of selected objects across all panes.
pub fn gumball_target_for(document: &CadScene, selected_ids: &[String]) -> Option<[f64; 3]> {
    let mut sum = [0.0; 3];
    let mut count = 0usize;
    for (object, _) in cad_all_objects(document) {
        if selected_ids.contains(&object.id) {
            sum[0] += object.origin[0];
            sum[1] += object.origin[1];
            sum[2] += object.origin[2];
            count += 1;
        }
    }
    if count == 0 {
        return None;
    }
    let n = count as f64;
    Some([sum[0] / n, sum[1] / n, sum[2] / n])
}

pub fn world_instances_json(objects: &[CadObject], runtime: &CadPlayRuntime) -> String {
    let instances: Vec<Value> = objects
        .iter()
        .filter(|object| object.visible)
        .map(|object| {
            let mesh_id = resolve_object_mesh_url(object).map_or_else(|| object.id.clone(), |url| world3d_mesh_id_from_url(&url));
            let selected = runtime.selected_object_ids.contains(&object.id);
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

pub fn world_selection_json(document: &CadScene, runtime: &CadPlayRuntime, active_utility: Option<&str>, options: CadDislocateOptions) -> String {
    let mut value: Value = serde_json::from_str(&world3d_selection_json(&runtime.selection_method, runtime.selected_object_ids.as_slice(), runtime.hovered_object_id.as_deref())).unwrap_or_else(|_| json!({}));
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
        object.insert("selectionMode".into(), json!(runtime.component_selection.mode));
        object.insert("granularity".into(), json!(runtime.component_selection.mode));
        object.insert("targets".into(), json!(runtime.component_selection.targets));
        object.insert("componentIds".into(), json!(runtime.component_selection.ids));
        if let Some(active) = resolve_active_object_id(runtime) {
            object.insert("activeObjectId".into(), json!(active));
        }
        if let Some(target) = runtime.hovered_target.as_ref() {
            object.insert("hoveredComponent".into(), json!(target));
        }
        if let Some(reference_id) = runtime.selected_reference_id.as_deref() {
            object.insert("referenceSelectedId".into(), json!(reference_id));
        }
        if active {
            if let Some(target) = gumball_target_for(document, runtime.selected_object_ids.as_slice()) {
                object.insert("gumballTarget".into(), json!(target));
            }
        }
    }
    value.to_string()
}

pub fn world_references_json(document: &CadScene, pane: CadPaneId) -> Option<String> {
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

pub fn build_world_scene_for_pane(envelope: &CadPlayView, pane: CadPaneId, surface_id: &str, active_utility: Option<&str>, options: CadDislocateOptions) -> UiNode {
    let objects = cad_pane_objects(&envelope.document, pane);
    let preview = envelope.runtime.engagement_session.as_ref().filter(|session| session.pane == pane).map(preview_display_items).filter(|items| !items.is_empty()).map(|items| serde_json::to_string(&items).unwrap_or_else(|_| "[]".into()));
    build_world_3d_scene(
        surface_id,
        CAD_PLAY_APP_ID,
        world3d_scene_extended(
            camera_json(cad_pane_camera_runtime(&envelope.runtime, pane)),
            world_meshes_json(objects, cad_pane_geometry(&envelope.document, pane)),
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
        ),
    )
}
//#endregion 🔖️WorldScene

//#region 🔖️Engagement
pub fn cad_window_engagement(envelope: &CadPlayView, pane: CadPaneId, labels: &CadLabels) -> WindowEngagement {
    let selected_count = envelope.runtime.selected_object_ids.len();
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
        // from `UtilityDefinition`s + `ViewState::active_utility_id`); the engagement HUD no longer
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
