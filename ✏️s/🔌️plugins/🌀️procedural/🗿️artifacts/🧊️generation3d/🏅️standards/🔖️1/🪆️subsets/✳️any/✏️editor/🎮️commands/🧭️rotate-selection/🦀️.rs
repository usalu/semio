//! 🧭️ 🧭️ Generation3d play app commands command — `rotate-selection`.

use crate::artifacts::generation3d::op::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::{FlowEvalSession, FlowFixture, FlowHost};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

use crate::artifacts::generation3d::schema::{commit_fixture, ensure_gumball_node, gumball_rotate_params_json, gumball_widget_number_param, host_from_fixture};
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
#[dsl(keyword = "rotate-selection")]
pub struct RotateSelection {
    pub node_ids: Vec<String>,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    pub angle: f64,
}

fn rotate_ids(fixture: &FlowFixture, ids: &[String], ax: f64, ay: f64, az: f64, angle: f64) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
    match gumball_transform(fixture, ids, "rotate", move |host, transform_id| {
        let current_angle = gumball_widget_number_param(host, transform_id, "angle", 0.0);
        host.set_neuron_params(transform_id, &gumball_rotate_params_json([ax, ay, az], current_angle + angle)).is_ok()
    }) {
        Some((operations, _new_selection)) => Emit { artifact_mutations: operations, coalesce_key: Some("gumball-rotate".into()), ..Default::default() },
        None => Emit::default(),
    }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, ctx)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// reachable only through that macro-generated path (`Generation3dPlayApp::handle` always routes this
/// command through `apply` below instead), so an ids-less payload degrades to a no-op transform.
pub fn handle(payload: &RotateSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, &[]);
    Ok(rotate_ids(&doc.snapshot.fixture, &ids, payload.ax, payload.ay, payload.az, payload.angle))
}

/// 🕹️ Falls back to the `graph` domain's current selection instead of a deleted config field when the
/// command carries no explicit ids.
pub fn apply(
    payload: &RotateSelection,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    _cfg: &ConfigView<'_, Generation3dConfig>,
    interaction: &InteractionView<'_>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, &interaction.selection("graph").ids);
    Ok(rotate_ids(&doc.snapshot.fixture, &ids, payload.ax, payload.ay, payload.az, payload.angle))
}
