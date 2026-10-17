//! 🖥️ Flow editor host bridge: rebuilds the stateful `FlowHost` from the document projection, view config and eval session, and maps host results back onto content-child leaves.

use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use crate::{FlowMutation, FlowSnapshot, FlowWorkingScene};
use flow::{flow_host_with_session, FlowEvalSession, FlowHost, FLOW_LOD_MODE_AUTOMATIC};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_os_infinite_directed_presentation::DagDrawLod;
use semio_framework_plugin::app::ChildEmitPreparation;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation};
use semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;

//#region 🔖️Host
pub fn seed_host_catalogue(host: &mut FlowHost, extra_sections_json: &str) {
    let mut sections = flow::flow_catalogue_sections();
    if let Ok(extra) = serde_json::from_str::<Vec<flow::CatalogueSection>>(extra_sections_json) {
        sections.extend(extra);
    }
    host.set_host_catalogue_json(&serde_json::to_string(&sections).unwrap_or_else(|_| "[]".into()));
}

/// 🎚️ Pushes the view-state canvas options (LOD mode, proximity distance, grid) onto a freshly built host.
pub fn apply_canvas_options(host: &mut FlowHost, config: &FlowMainWindowConfig) {
    if config.lod_mode != FLOW_LOD_MODE_AUTOMATIC && DagDrawLod::from_id(&config.lod_mode).is_some() {
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label(&config.lod_mode);
    } else {
        host.dag.set_automatic_lod(true);
    }
    host.dag.set_proximity_distance(config.proximity_distance);
    host.set_grid_visible(config.grid_visible);
    host.set_grid_snap_enabled(config.grid_snap_enabled);
    let _ = host.set_grid_factor(config.grid_factor);
}

/// 🏗️ Rebuilds the stateful `FlowHost` from the document projection + view config + eval session — the
/// single entry point every command handler and every window renderer goes through.
/// 🎟️ The generous grant every synchronous cold session path debits: those paths run to completion inside one command turn.
pub fn cold_grant() -> semio_framework_value::RetainedCloneGrant {
    semio_framework_value::RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 26, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 28, maximum_depth: 4096 }
}

/// 🏠️ Seeds a host from `live` and installs `session`'s converged baseline under the cold grant.
pub fn host_with_session(live: semio_framework_artifact_flow_flow::FlowHostSnapshot, session: &FlowEvalSession) -> FlowHost {
    match flow_host_with_session(FlowHost::from_host_snapshot(live).with_history_store_factory(::semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY), session, cold_grant()) {
        Ok((host, _)) => host,
        Err((error, host)) => {
            host.retire_cold();
            panic!("flow cold host baseline refused: {error}")
        }
    }
}

pub fn host_from_snapshot(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession) -> FlowHost {
    let mut host = host_with_session(snapshot.to_host_snapshot(), session);
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    apply_canvas_options(&mut host, config);
    host
}

/// 🏠️ Runs `body` against a host rebuilt by [`host_from_snapshot`], then retires that host — the ONE
/// shape every render, probe and evaluation step that only needs a host for the length of a call must
/// use. A `FlowHost` owns a `FlowHostSnapshot` whose `layout: OrderedMap<WidgetLayout>` aborts the
/// guest on a bare drop, so a host is closed, never dropped (the twin of `FlowHost::with_host_snapshot`).
pub fn with_host_from_snapshot<R>(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, body: impl FnOnce(&mut FlowHost) -> R) -> R {
    let mut host = host_from_snapshot(snapshot, config, session);
    let result = body(&mut host);
    host.retire_cold();
    result
}

/// 📸️ Runs `body` against the live host projection of `snapshot`, then retires that projection — a
/// `FlowHostSnapshot` owns an `OrderedMap` layout root and widget payloads that refuse a bare drop.
pub fn with_live_host_snapshot<R>(snapshot: &FlowSnapshot, body: impl FnOnce(&semio_framework_artifact_flow_flow::FlowHostSnapshot) -> R) -> R {
    let live = snapshot.to_host_snapshot();
    let result = body(&live);
    live.retire_cold();
    result
}

/// 🪆️ The content child `composed` names, as the exact scene its working-scene owner carries.
pub fn flow_composed_content(composed: &FlowSnapshot) -> Result<SemioFlowSnapshot, Fault> {
    let scene = composed.content.local_owner::<FlowWorkingScene>().ok_or_else(|| Fault::from("flow-edit-scene-owner-missing"))?;
    Ok(crate::flow_content_snapshot_from_working(&scene.widgets, &scene.synapses, &scene.layout))
}

/// 📮️ `leaves` as ONE group on the content child `child_id`; nothing when there are none.
pub fn flow_content_leaves_emit(child_id: &str, leaves: Vec<SemioFlowMutation>) -> Emit<FlowMutation, NoConfigMutation> {
    match leaves.is_empty() {
        true => Emit::default(),
        false => Emit { child_preparations: std::collections::VecDeque::from([ChildEmitPreparation::of::<SemioFlowSnapshot, _>("content", child_id, leaves)]), ui_scope: UiDirtyScope::Full, ..Default::default() },
    }
}

/// 🗑️ The child leaves that delete `node_ids` and `edge_ids` from `child`: every named edge and every edge touching a named
/// node first (child order, so no edge ever dangles), then every named node.
pub fn flow_removal_leaves(child: &SemioFlowSnapshot, node_ids: &[String], edge_ids: &[String]) -> Vec<SemioFlowMutation> {
    let mut edit = crate::editor::flow::edit_rules::ContentEdit::new(child.clone());
    edit.remove(node_ids, edge_ids);
    edit.leaves
}
//#endregion 🔖️Host

//#region 🔖️Selection
pub fn sync_host_selection(host: &mut FlowHost, selected: &[String]) {
    sync_host_selection_domains(host, selected, &[], &[]);
}

pub fn sync_host_selection_domains(host: &mut FlowHost, nodes: &[String], edges: &[String], handles: &[String]) {
    if nodes.is_empty() && edges.is_empty() && handles.is_empty() {
        let _ = host.dag.cancel_area_select();
        return;
    }
    host.dag.set_selection_domains(&infinite_canvas::board::schema::dag_input::DagSelectionDomains {nodes:nodes.to_vec(),edges:edges.to_vec(),handles:handles.to_vec()});
}

/// 🔍️ The camera that frames the given node selection (the "graph" domain's live selection, read by
/// the caller via `InteractionView` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), or
/// `None` when nothing is selected.
pub fn focus_selection_camera(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, selected_node_ids: &[String]) -> Option<CameraJson> {
    if selected_node_ids.is_empty() {
        return None;
    }
    with_host_from_snapshot(snapshot, config, session, |host| {
        host.dag.set_viewport(1280, 800, 1.0);
        host.dag.set_selection(selected_node_ids);
        host.focus_selection_camera(1.2)
    })
}
//#endregion 🔖️Selection

//#region 🧯️Faults
/// 🧯️ Surfaces a retained-ownership refusal as an app fault carrying its message.
pub fn value_fault(error: semio_framework_value::ValueError) -> Fault { Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.retained"), error.into_message()) }
//#endregion 🧯️Faults
