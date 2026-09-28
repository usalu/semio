//! 🧭️ 🧭️ Generation3d play app commands command — `scale-selection`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use crate::editor::generation3d::transform_commands::{apply as gumball_transform, selection_ids as mesh_selection_ids_typed};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

use crate::standards::v1::subsets::any::schema::{gumball_scale_params_json, transforms::compose_scale, gumball_widget_json, with_host};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "scale-selection")]
pub struct ScaleSelection {
    pub node_ids: Vec<String>,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}

fn scale_ids(host_snapshot: &FlowHostSnapshot, ids: &[String], factors: [f64; 3]) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    gumball_transform(host_snapshot, ids, "scale", move |host, transform_id| {
        let widget = gumball_widget_json(host, transform_id);
        let factor = widget.as_ref().and_then(|value| value.get("params")).and_then(|value| value.get("factor"));
        let current = ["x", "y", "z"].map(|key| factor.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_f64).unwrap_or(1.0));
        let params = gumball_scale_params_json(compose_scale(current, factors).map_err(str::to_string)?);
        host.set_neuron_params(transform_id, &params).map_err(|error| error.to_string())
    })
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, ctx)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// reachable only through that macro-generated path (`Generation3dPlayApp::handle` always routes this
/// command through `apply` below instead), so an ids-less payload degrades to a no-op transform.
pub fn handle(payload: &ScaleSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, &[]);
    scale_ids(&doc.snapshot.host_snapshot, &ids, [payload.sx, payload.sy, payload.sz])
}

/// 🕹️ Falls back to the `graph` domain's current selection instead of a deleted config field when the
/// command carries no explicit ids.
pub fn apply(
    payload: &ScaleSelection,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    _cfg: &ConfigView<'_, Generation3dConfig>,
    interaction: &InteractionView<'_>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, &interaction.selection("graph").ids);
    scale_ids(&doc.snapshot.host_snapshot, &ids, [payload.sx, payload.sy, payload.sz])
}

/// 🕹️ Retained-command-job entry point (`generation3d_retained_reduce`, editor `🦀️.rs`) — same real-selection
/// behavior as `apply` above, but callable without an `app::InteractionView` (which plugin code cannot
/// construct; its fields are `pub(crate)` to the framework crate). `selected` is read straight off
/// `protocol::InteractionState` by the caller.
pub(crate) fn apply_selected(payload: &ScaleSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, selected: &[String]) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, selected);
    scale_ids(&doc.snapshot.host_snapshot, &ids, [payload.sx, payload.sy, payload.sz])
}

/// 🧩️ Applies viewport deltas to the selected components, retaining their topology identifiers.
pub(crate) fn apply_components(payload: &ScaleSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, selected: &[String]) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    crate::editor::generation3d::transform_commands::validate_component_gesture(&doc.snapshot.host_snapshot, &payload.node_ids, selected)?;
    scale_ids(&doc.snapshot.host_snapshot, selected, [payload.sx, payload.sy, payload.sz])
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
