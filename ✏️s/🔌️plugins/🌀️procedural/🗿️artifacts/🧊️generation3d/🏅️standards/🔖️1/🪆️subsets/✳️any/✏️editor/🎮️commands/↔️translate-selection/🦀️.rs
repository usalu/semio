//! 🧭️ 🧭️ Generation3d play app commands command — `translate-selection`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use crate::editor::generation3d::transform_commands::{apply as gumball_transform, selection_ids as mesh_selection_ids_typed};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

use crate::standards::v1::subsets::any::schema::{gumball_translate_params_json, gumball_widget_offset, with_host};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "translate-selection")]
pub struct TranslateSelection {
    pub node_ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

fn translate_ids(host_snapshot: &FlowHostSnapshot, ids: &[String], dx: f64, dy: f64, dz: f64) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    gumball_transform(host_snapshot, ids, "translate", move |host, transform_id| {
        let current = gumball_widget_offset(host, transform_id);
        let next = [current[0] + dx, current[1] + dy, current[2] + dz];
        if next.iter().any(|value| !value.is_finite()) { return Err("Translation values must be finite".into()); }
        host.set_neuron_params(transform_id, &gumball_translate_params_json(next)).map_err(|error| error.to_string())
    })
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, ctx)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// reachable only through that macro-generated path (`Generation3dPlayApp::handle` always routes this
/// command through `apply` below instead), so an ids-less payload degrades to a no-op transform.
pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, &[]);
    translate_ids(&doc.snapshot.host_snapshot, &ids, payload.dx, payload.dy, payload.dz)
}

/// 🕹️ Falls back to the `graph` domain's current selection instead of a deleted config field when the
/// command carries no explicit ids. The resulting transform remains selected through interaction writes.
pub fn apply(
    payload: &TranslateSelection,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    _cfg: &ConfigView<'_, Generation3dConfig>,
    interaction: &InteractionView<'_>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, &interaction.selection("graph").ids);
    translate_ids(&doc.snapshot.host_snapshot, &ids, payload.dx, payload.dy, payload.dz)
}

/// 🕹️ Retained-command-job entry point (`generation3d_retained_reduce`, editor `🦀️.rs`) — same real-selection
/// behavior as `apply` above, but callable without an `app::InteractionView` (which plugin code cannot
/// construct; its fields are `pub(crate)` to the framework crate). `selected` is read straight off
/// `protocol::InteractionState` by the caller.
pub(crate) fn apply_selected(payload: &TranslateSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, selected: &[String]) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, selected);
    translate_ids(&doc.snapshot.host_snapshot, &ids, payload.dx, payload.dy, payload.dz)
}

/// 🧩️ Applies viewport deltas to the selected components, retaining their topology identifiers.
pub(crate) fn apply_components(payload: &TranslateSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, selected: &[String]) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    crate::editor::generation3d::transform_commands::validate_component_gesture(&doc.snapshot.host_snapshot, &payload.node_ids, selected)?;
    translate_ids(&doc.snapshot.host_snapshot, selected, payload.dx, payload.dy, payload.dz)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
