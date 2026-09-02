//! 🧭️ 🧭️ Procedural3d play app commands command — `scale-selection`.

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::{FlowEvalSession, FlowFixture, FlowHost};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

use crate::artifacts::procedural3d::schema::{commit_fixture, ensure_gumball_node, gumball_scale_params_json, gumball_widget_number_param, host_from_fixture};
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
fn gumball_transform(fixture: &FlowFixture, ids: &[String], operation: &str, apply: impl Fn(&mut FlowHost, &str) -> bool) -> Option<(Vec<Procedural3dMutation>, Vec<String>)> {
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
#[dsl(keyword = "scale-selection")]
pub struct ScaleSelection {
    pub node_ids: Vec<String>,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}

fn scale_ids(fixture: &FlowFixture, ids: &[String], uniform_factor: f64) -> Emit<Procedural3dMutation, Procedural3dConfigMutation> {
    match gumball_transform(fixture, ids, "scale", move |host, transform_id| {
        let current_factor = gumball_widget_number_param(host, transform_id, "factor", 1.0);
        host.set_neuron_params(transform_id, &gumball_scale_params_json(current_factor * uniform_factor)).is_ok()
    }) {
        Some((operations, _new_selection)) => Emit { artifact_mutations: operations, coalesce_key: Some("gumball-scale".into()), ..Default::default() },
        None => Emit::default(),
    }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, ctx)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// reachable only through that macro-generated path (`Procedural3dPlayApp::handle` always routes this
/// command through `apply` below instead), so an ids-less payload degrades to a no-op transform.
pub fn handle(payload: &ScaleSelection, doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, &[]);
    Ok(scale_ids(&doc.snapshot.fixture, &ids, (payload.sx + payload.sy + payload.sz) / 3.0))
}

/// 🕹️ Falls back to the `graph` domain's current selection instead of a deleted config field when the
/// command carries no explicit ids.
pub fn apply(
    payload: &ScaleSelection,
    doc: &ArtifactView<'_, Procedural3dSnapshot>,
    _cfg: &ConfigView<'_, Procedural3dConfig>,
    interaction: &InteractionView<'_>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, &interaction.selection("graph").ids);
    Ok(scale_ids(&doc.snapshot.fixture, &ids, (payload.sx + payload.sy + payload.sz) / 3.0))
}
