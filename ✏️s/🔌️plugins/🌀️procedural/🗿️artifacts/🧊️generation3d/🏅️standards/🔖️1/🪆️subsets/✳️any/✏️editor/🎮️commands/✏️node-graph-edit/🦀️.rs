//! 🕸️ 🕸️ Generation3d play app commands command — `node-graph-edit`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::standards::v1::subsets::any::schema::{commit_host_document, with_host};
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::FlowHostDocument;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "graph-edit")]
pub struct NodeGraphEdit {
    pub operations_json: String,
}

/// 🌉️ `operations_json` is a locally-defined array of sub-operation descriptors (not a framework
/// boundary type) — parsed generically via `pack::json`'s raw tree, not `serde_json`.
fn parse_sub_operations(text: &str) -> Vec<dsl::json::Value> {
    dsl::json::parse(text).ok().and_then(|value| value.as_array().cloned()).unwrap_or_default()
}

/// 🧾️ The six sub-operations the node-graph surfaces actually dispatch, and who dispatches each:
/// `setFixture` (the wasm flow canvas, after every committed gesture — `🕸️NodeGraph/🟦️.tsx`'s
/// `onFixtureChanged`), `move` / `connect` (the SSR `Diagram` fallback's `onNodeDragStop` /
/// `onConnect`), `disconnect` (a cut wire), and `deleteSelection` (the row/keyboard delete path).
/// `move` and `disconnect` used to fall through the `_ => {}` arm, so a fallback node drag and every
/// wire cut were silent no-ops that still spent a whole retained command
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gaps #3/#10). `move` is also the ONE home of the node
/// move verb now — the redundant `moveMediaNode` command it duplicated is gone.
///
/// `setSlider` is the inline slider overlay's value, LIVE while the user drags. It arrives once per
/// coalesced round trip rather than once per pointer move (`🕸️NodeGraph/🟦️.tsx`'s
/// `useGraphSliderLanes`), carries the `gesture` identity of the press it belongs to, and the last
/// one of a press carries `commit`. Its whole press folds into ONE undoable edit: every tick emits
/// under the gesture's coalesce key, so the history shows the drag, not its 60 samples
/// (`📓️slider-preview-update-2026-09-15.md`). Before this, the overlay dispatched an undeclared
/// `setGraphParameter` action that the shell dropped, so the document — and the preview — never moved
/// at all.
fn apply_operations(fixture: &FlowHostDocument, sub_operations: &[dsl::json::Value], selected: &[String]) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
    let operations = with_host(fixture, |host| {
        for operation in sub_operations {
            match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                "setHostDocument" => {
                    if let Some(new_fixture) = operation.get("hostDocumentJson").and_then(|value| value.as_str()).and_then(|json| semio_framework_os_flow::os_pack::json::from_json_str::<FlowHostDocument>(json).ok()) {
                        host.replace_host_document(new_fixture);
                    }
                }
                "deleteSelection" => {
                    for id in selected {
                        let _ = host.remove_widget(id);
                    }
                }
                "connect" => {
                    let from = operation.get("sourceNodeId").and_then(|value| value.as_str());
                    let from_port = operation.get("sourcePortId").and_then(|value| value.as_str());
                    let to = operation.get("targetNodeId").and_then(|value| value.as_str());
                    let to_port = operation.get("targetPortId").and_then(|value| value.as_str());
                    if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                        let _ = host.connect_ports(from, from_port, to, to_port);
                    }
                }
                "disconnect" => {
                    if let Some(synapse_id) = operation.get("synapseId").and_then(|value| value.as_str()) {
                        let _ = host.disconnect(synapse_id);
                    }
                }
                "move" => {
                    let node_id = operation.get("nodeId").and_then(|value| value.as_str());
                    let x = operation.get("x").and_then(dsl::json::Value::as_f64);
                    let y = operation.get("y").and_then(dsl::json::Value::as_f64);
                    if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                        let _ = host.move_widget(node_id, x, y);
                    }
                }
                "setSlider" => {
                    let widget_id = operation.get("widgetId").and_then(|value| value.as_str());
                    let value = operation.get("value").and_then(dsl::json::Value::as_f64);
                    if let (Some(widget_id), Some(value)) = (widget_id, value) {
                        host.set_slider_value(widget_id, value);
                    }
                }
                _ => {}
            }
        }
        commit_host_document(fixture, &host.host_document)
    });
    Emit { artifact_mutations: operations, coalesce_key: gesture_coalesce_key(sub_operations), ..Default::default() }
}

/// 🎚️ The coalesce key one continuous gesture's edits fold under, or `None` for a discrete edit.
///
/// A key is minted only when EVERY sub-operation of this dispatch belongs to the SAME press — the
/// releasing tick included, so a whole drag is one undo step. A mixed batch, or a `setSlider` with no
/// press identity, is a described edit of its own. `AmendLast` keys on the string alone, so a press
/// identified only by its widget would swallow every later press of that widget into one undo step.
fn gesture_coalesce_key(sub_operations: &[dsl::json::Value]) -> Option<String> {
    let mut gesture: Option<&str> = None;
    for operation in sub_operations {
        if operation.get("operation").and_then(|value| value.as_str()) != Some("setSlider") {
            return None;
        }
        let key = operation.get("gesture").and_then(|value| value.as_str())?;
        if gesture.is_some_and(|current| current != key) {
            return None;
        }
        gesture = Some(key);
    }
    gesture.map(|key| format!("graph-slider:{key}"))
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, ctx)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// reachable only through that macro-generated path (`Generation3dPlayApp::handle` always routes this
/// command through `apply` below instead), so `"deleteSelection"` sub-operations degrade to treating
/// the selection as empty.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let sub_operations = parse_sub_operations(&payload.operations_json);
    Ok(apply_operations(&doc.snapshot.host_document, &sub_operations, &[]))
}

/// 🕹️ `"deleteSelection"` reads the `graph` domain's current selection instead of a deleted config
/// field — no config mutation needed afterwards, the framework auto-prunes the deleted ids out of
/// `graph`'s selection.
pub fn apply(
    payload: &NodeGraphEdit,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    _cfg: &ConfigView<'_, Generation3dConfig>,
    interaction: &InteractionView<'_>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let sub_operations = parse_sub_operations(&payload.operations_json);
    Ok(apply_operations(&doc.snapshot.host_document, &sub_operations, &interaction.selection("graph").ids))
}

/// 🕹️ Retained-command-job entry point (`generation3d_retained_reduce`, editor `🦀️.rs`) — same real-selection
/// behavior as `apply` above, but callable without an `app::InteractionView` (which plugin code cannot
/// construct; its fields are `pub(crate)` to the framework crate). `selected` is read straight off
/// `protocol::InteractionState` by the caller.
pub(crate) fn apply_selected(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Generation3dSnapshot>, selected: &[String]) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
    let sub_operations = parse_sub_operations(&payload.operations_json);
    apply_operations(&doc.snapshot.host_document, &sub_operations, selected)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
