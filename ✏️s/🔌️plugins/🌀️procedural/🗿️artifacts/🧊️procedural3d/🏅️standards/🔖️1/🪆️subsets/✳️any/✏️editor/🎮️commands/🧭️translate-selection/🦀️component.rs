//! 🧭️ 🧭️ Procedural3d play app commands command — `translate-selection`.

use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::{FlowEvalSession, FlowFixture, FlowHost};
use semio_framework_plugin::{app::InteractionView, ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

use crate::artifacts::procedural3d::schema::{
    commit_fixture, ensure_gumball_node, gumball_translate_params_json, gumball_widget_offset, host_from_fixture};

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
#[dsl(keyword = "translate-selection")]
pub struct TranslateSelection {
    pub node_ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64}

fn translate_ids(fixture: &FlowFixture, ids: &[String], dx: f64, dy: f64, dz: f64) -> Emit<Procedural3dMutation, Procedural3dConfigMutation> {
    match gumball_transform(fixture, ids, "translate", move |host, transform_id| {
        let current = gumball_widget_offset(host, transform_id);
        let next = [current[0] + dx, current[1] + dy, current[2] + dz];
        host.set_neuron_params(transform_id, &gumball_translate_params_json(next)).is_ok()
    }) {
        Some((operations, _new_selection)) => Emit { artifact_mutations: operations, coalesce_key: Some("gumball-translate".into()), ..Default::default() },
        None => Emit::default()}
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, ctx)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// reachable only through that macro-generated path (`Procedural3dPlayApp::handle` always routes this
/// command through `apply` below instead), so an ids-less payload degrades to a no-op transform.
pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, &[]);
    Ok(translate_ids(&doc.snapshot.fixture, &ids, payload.dx, payload.dy, payload.dz))
}

/// 🕹️ Falls back to the `graph` domain's current selection instead of a deleted config field when the
/// command carries no explicit ids. The created gumball transform widget is no longer auto-reselected
/// — the framework, not this app, owns `graph`'s selection now, and no `Emit` channel can write it
/// directly.
pub fn apply(payload: &TranslateSelection, doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, interaction: &InteractionView<'_>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let ids = mesh_selection_ids_typed(&payload.node_ids, &interaction.selection("graph").ids);
    Ok(translate_ids(&doc.snapshot.fixture, &ids, payload.dx, payload.dy, payload.dz))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::testkit::{app, dispatch};
    use crate::editor::procedural3d::Procedural3dCommand;
    use crate::editor::procedural3d::commands::{rotate_selection, scale_selection};
    use crate::artifacts::procedural3d::widget_id;
    use flow::Widget;

    #[test]
    fn translate_selection_persists_transform_into_flow_graph() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        assert!(before.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"));
        dispatch(&mut app, Procedural3dCommand::TranslateSelection(TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0 }));
        let projection = app.snapshot().expect("snapshot");
        let transform_id = "extrude__gumball_translate";
        let transform = projection.fixture.widgets.iter().find(|widget| widget_id(widget) == transform_id).expect("transform neuron created");
        assert!(matches!(transform, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.xform.translate"));
        let offset = gumball_widget_offset(&host_from_fixture(&projection.fixture), transform_id);
        assert_eq!(offset, [1.0, 2.0, 3.0]);

        // Re-grabbing the same transform accumulates the delta instead of creating a second node.
        dispatch(&mut app, Procedural3dCommand::TranslateSelection(TranslateSelection { node_ids: vec![transform_id.into()], dx: 1.0, dy: 0.0, dz: 0.0 }));
        let projection2 = app.snapshot().expect("snapshot");
        assert_eq!(projection2.fixture.widgets.iter().filter(|widget| widget_id(widget) == transform_id).count(), 1);
        assert_eq!(gumball_widget_offset(&host_from_fixture(&projection2.fixture), transform_id), [2.0, 2.0, 3.0]);
    }

    #[test]
    fn rotate_and_scale_selection_persist_into_flow_graph() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut rotate_app = app();
        dispatch(&mut rotate_app, Procedural3dCommand::RotateSelection(rotate_selection::RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::FRAC_PI_2 }));
        let rotated = rotate_app.snapshot().expect("snapshot");
        let rotate_id = "extrude__gumball_rotate";
        assert!(rotated.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == rotate_id && neuron_kind == "brep.xform.rotate")));

        let mut scale_app = app();
        dispatch(&mut scale_app, Procedural3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids: vec!["extrude".into()], sx: 2.0, sy: 2.0, sz: 2.0 }));
        let scaled = scale_app.snapshot().expect("snapshot");
        let scale_id = "extrude__gumball_scale";
        assert!(scaled.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == scale_id && neuron_kind == "brep.xform.scale")));
    }
}
//#endregion 🧪️Tests
