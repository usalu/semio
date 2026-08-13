//! 🧭️ 🧭️ Procedural3d play app commands command — `rotate-selection`.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::{FlowEvalSession, FlowFixture, FlowHost};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

use crate::artifacts::procedural3d::schema::{
    commit_fixture, ensure_gumball_node, gumball_rotate_params_json, gumball_scale_params_json, gumball_translate_params_json, gumball_widget_number_param, gumball_widget_offset, host_from_fixture};

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "rotate-selection")]
pub struct RotateSelection {
    pub node_ids: Vec<String>,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    pub angle: f64}

pub fn handle(payload: &RotateSelection, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let ids = mesh_selection_ids_typed(&payload.node_ids, &cfg.snapshot.selected_node_ids);
    let (ax, ay, az, angle) = (payload.ax, payload.ay, payload.az, payload.angle);
    match gumball_transform(fixture, &ids, "rotate", move |host, transform_id| {
        let current_angle = gumball_widget_number_param(host, transform_id, "angle", 0.0);
        host.set_neuron_params(transform_id, &gumball_rotate_params_json([ax, ay, az], current_angle + angle)).is_ok()
    }) {
        Some((operations, new_selection)) => Ok(Emit { artifact_mutations: operations, config_mutations: vec![Procedural3dConfigMutation::SetSelection { node_ids: new_selection }], coalesce_key: Some("gumball-rotate".into()), ..Default::default() }),
        None => Ok(Emit::default())}
}
