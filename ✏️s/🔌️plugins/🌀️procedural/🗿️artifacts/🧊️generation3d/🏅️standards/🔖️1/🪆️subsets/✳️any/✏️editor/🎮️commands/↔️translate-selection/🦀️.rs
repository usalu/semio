//! 🧭️ 🧭️ Generation3d play app commands command — `translate-selection`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::FlowFixture;
use semio_framework_os_flow::{FlowEvalSession, FlowHost};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

use crate::standards::v1::subsets::any::schema::{commit_fixture, ensure_gumball_node, gumball_translate_params_json, gumball_widget_offset, host_from_fixture};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shared
/// 🎯️ The typed-command counterpart of the pre-migration JSON-args `mesh_selection_ids` — falls back
/// to the current config selection when the command carries no explicit ids.
fn mesh_selection_ids_typed(ids: &[String], fallback: &[String]) -> Vec<String> {
    if ids.is_empty() {
        fallback.to_vec()
    } else {
        ids.to_vec()
    }
}

/// 🧭️ Runs a gumball transform (translate/rotate/scale) as a fixture operation, splicing transform
/// neurons via `ensure_gumball_node` and re-selecting the resulting transform widgets. `None` when no
/// transform actually changed anything (nothing to commit).
fn gumball_transform(fixture: &FlowFixture, ids: &[String], operation: &str, apply: impl Fn(&mut FlowHost, &str) -> bool) -> Option<(Vec<Generation3dMutation>, Vec<String>)> {
    let mut host = host_from_fixture(fixture);
    let mut new_selection = Vec::new();
    let mut changed = false;
    for id in ids {
        if let Ok(transform_id) = ensure_gumball_node(&mut host, id, operation) {
            if apply(&mut host, &transform_id) {
                new_selection.push(transform_id);
                changed = true;
            }
        }
    }
    if changed {
        Some((commit_fixture(fixture, &host.fixture), new_selection))
    } else {
        None
    }
}
//#endregion 🔖️Shared

//#region 🔖️TranslateSelection
//#endregion 🔖️TranslateSelection

//#region 🔖️RotateSelection
//#endregion 🔖️RotateSelection

//#region 🔖️ScaleSelection
//#endregion 🔖️ScaleSelection

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "translate-selection")]
pub struct TranslateSelection {
    pub node_ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

fn translate_ids(fixture: &FlowFixture, ids: &[String], dx: f64, dy: f64, dz: f64) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
    match gumball_transform(fixture, ids, "translate", move |host, transform_id| {
        let current = gumball_widget_offset(host, transform_id);
        let next = [current[0] + dx, current[1] + dy, current[2] + dz];
        host.set_neuron_params(transform_id, &gumball_translate_params_json(next)).is_ok()
    }) {
        Some((operations, _new_selection)) => Emit { artifact_mutations: operations, coalesce_key: Some("gumball-translate".into()), ..Default::default() },
        None => Emit::default(),
    }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, ctx)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// reachable only through that macro-generated path (`Generation3dPlayApp::handle` always routes this
/// command through `apply` below instead), so an ids-less payload degrades to a no-op transform.
pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, &[]);
    Ok(translate_ids(&doc.snapshot.fixture, &ids, payload.dx, payload.dy, payload.dz))
}

/// 🕹️ Falls back to the `graph` domain's current selection instead of a deleted config field when the
/// command carries no explicit ids. The created gumball transform widget is no longer auto-reselected
/// — the framework, not this app, owns `graph`'s selection now, and no `Emit` channel can write it
/// directly.
pub fn apply(
    payload: &TranslateSelection,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    _cfg: &ConfigView<'_, Generation3dConfig>,
    interaction: &InteractionView<'_>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, &interaction.selection("graph").ids);
    Ok(translate_ids(&doc.snapshot.fixture, &ids, payload.dx, payload.dy, payload.dz))
}

/// 🕹️ Retained-command-job entry point (`generation3d_retained_reduce`, editor `🦀️.rs`) — same real-selection
/// behavior as `apply` above, but callable without an `app::InteractionView` (which plugin code cannot
/// construct; its fields are `pub(crate)` to the framework crate). `selected` is read straight off
/// `protocol::InteractionState` by the caller.
pub(crate) fn apply_selected(payload: &TranslateSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, selected: &[String]) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, selected);
    translate_ids(&doc.snapshot.fixture, &ids, payload.dx, payload.dy, payload.dz)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
