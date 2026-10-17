//! 🧵️ Surface-neutral preview evaluation — the ONE read-only `previewEval` tool run both generation3d
//! surfaces run (`📋️tool-run-contract.md` §2, §3.7; ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS).
//!
//! ⏯️ The framework owns the lifecycle: a surface's `pending_effects` starts the run once an attached
//! preview window owes an evaluation, and start, pause, single step, abort and finalize are the
//! framework-reserved actions. [`PreviewEvalRunJob`] owns the scheduling: each algorithm unit is ONE
//! chain hop it hands the host through its [`ToolRunJobPort`] (`flowEvalTick` addressed at a preview
//! window), after which it waits until a hop settles. The hop advances the retained
//! [`FlowEvalSession`], publishes the evaluation into the addressed window's retained transient and
//! parks an [`semio_framework_plugin::ExtensionInvocation`] (`evaluate` while operators are still
//! pending, then `tessellate`); `flowEvalResolve` / `flowTessellateResolve` fold the answers and wake
//! the job. Every observed node transition becomes a trace record keyed by the node's entity id. Abort
//! is host-driven by construction: the job's close quiesces the session and hands the host one
//! `flowEvalRelease` hop that reaches the geometry extension's own retained kernel jobs. No surface
//! may run a synchronous
//! `FlowEvalSession::tick` loop instead: the brep/math operators are CONTRIBUTED by the host at
//! runtime (`setContributions`) and are never linked into the guest, so an in-guest tick can only
//! ever fault (`📓️audit-window-inventory-2026-09-12.md` §4 P0 item 1).
//!
//! 🪆️ This module lives at the SUBSET level, beside `✏️editor` and `👁️viewer` rather than inside
//! either, because both run the identical chain and `policyViewerPurityBreaches` forbids a viewer
//! file reaching through `::editor::`. Everything a surface owns alone — its config type, its
//! interaction marks, its payload assembly, its window chrome — stays on that surface; everything
//! the chain itself is made of lives here exactly once (CLAUDE.md: repeated code MUST be close to
//! each other). Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
#![allow(clippy::result_large_err)]

use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, StepContext};
use semio_s_artifact_procedural_3d_model::host_with_session;
use semio_framework_os_flow::{FlowEvalPublication, FlowEvalSession};
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin::Effect;
use semio_framework_plugin::ExtensionInvocation;
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MeshData;
use semio_framework_plugin::ToolDefinition;
use semio_framework_plugin::ToolRunJobPort;
use semio_framework_plugin::ToolRunView;
use semio_framework_plugin::ViewModel;
use semio_framework_tool_run::{
    ToolRunCounter, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTickWriter, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID,
    TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_FINALIZE_ACTION_ID, TOOL_RUN_START_ACTION_ID,
};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::{BTreeMap, HashSet};
#[path="📥️tessellation/🦀️.rs"]
mod tessellation_receiving;
pub use tessellation_receiving::{PreviewTessellateReceive,PreviewEvalReceive};

#[path = "🪟️window-addressing/🦀️.rs"]
mod window_addressing;
pub use window_addressing::*;

#[path = "🧊️preview-geometry/🦀️.rs"]
mod preview_geometry;
pub use preview_geometry::*;

#[path = "⏱️tick-and-resolve/🦀️.rs"]
mod tick_and_resolve;
pub use tick_and_resolve::*;

#[path = "📈️status-json/🦀️.rs"]
mod status_json;
pub use status_json::*;

//#region ⏱️Tick

pub use semio_framework_os_flow::flow_inference_dependency_json as geometry_dependency_json;

//#endregion ⏱️Tick

//#region ⏯️Run
#[path = "⏯️tool-run/🦀️.rs"]
mod tool_run;
pub use tool_run::*;
//#endregion ⏯️Run

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

pub fn dag_host_snapshot_to_workflow(host_snapshot: &semio_framework_os_infinite_directed_presentation::DagHostSnapshot) -> (Vec<semio_framework_ui::wgpu::NodeGraphNodeRecord>, Vec<semio_framework_ui::wgpu::NodeGraphEdgeRecord>) {
    let nodes: Vec<semio_framework_ui::wgpu::NodeGraphNodeRecord> = host_snapshot
        .nodes
        .iter()
        .map(|node| semio_framework_ui::wgpu::NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| semio_framework_ui::wgpu::NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), value_type: port.value_type.clone(), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| semio_framework_ui::wgpu::NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), value_type: port.value_type.clone(), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<semio_framework_ui::wgpu::NodeGraphEdgeRecord> = host_snapshot
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            semio_framework_ui::wgpu::NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    (nodes, edges)
}

/// 🪪️ Reads exact retained channel packs without display filtering or another kernel evaluation.
pub fn export_meshes_from_evaluation(snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, session: &FlowEvalSession, widget_id: Option<&str>) -> Result<Vec<semio_framework_plugin::MeshData>, semio_framework_diagnostic::TextError> {
    use semio_s_artifact_procedural_3d_mesh_bridge::io_error;
    if session.preview_chain_status().working || session.preview_eval_status().in_flight > 0 || session.preview_tessellate_status().in_flight > 0 { return Err(io_error("generation3d export is waiting for the current geometry evaluation")); }
    let sources = semio_s_artifact_procedural_3d_mesh_bridge::export_source_channels(snapshot, widget_id)?;
    let eval = semio_framework_pack_json::parse(session.eval_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| io_error(error.to_string()))?;
    if let Some(error) = eval.get("error").and_then(semio_framework_pack_json::Value::as_str) { return Err(io_error(error)); }
    let mut meshes = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for (source, channel) in sources {
        for item in preview_channel_items_for_widget(&eval, &source).into_iter().filter(|item| channel.as_ref().is_none_or(|channel| *channel == item.channel)) {
            let key = if item.handle.is_empty() { format!("{source}@{}#{}", item.channel, item.index) } else { item.handle.clone() };
            if !seen.insert(key) { continue; }
            let mesh = match item.inline {
                Some(PreviewInlineGeometry::Point { x, y, z }) => Some(point_marker_mesh(x, y, z)),
                Some(PreviewInlineGeometry::Vector { .. }) => continue,
                Some(PreviewInlineGeometry::Mesh { preview, .. }) => decode_preview_mesh_pack(&preview),
                None => session_preview_mesh(&item.handle, session),
            }.ok_or_else(|| io_error(format!("generation3d export geometry '{source}@{}' is unavailable", item.channel)))?;
            meshes.push(mesh);
        }
    }
    if meshes.is_empty() { return Err(io_error("generation3d export has no connected geometry")); }
    Ok(meshes)
}
