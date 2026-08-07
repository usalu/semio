//! 🧭️ Procedural3d play app commands — the 3D preview transform gumball (translate/rotate/scale),
//! splicing a persisted transform neuron into the flow graph on first grab and accumulating on repeat
//! drags (coalesced into one undoable edit per gesture).

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigOperation};
use crate::artifacts::procedural3d::engine::{
    commit_fixture, ensure_gumball_node, gumball_rotate_params_json, gumball_scale_params_json, gumball_translate_params_json, gumball_widget_number_param, gumball_widget_offset, host_from_fixture,
};
use crate::artifacts::procedural3d::op::Procedural3dOperation;
use crate::artifacts::procedural3d::Procedural3dDocument;
use flow::{FlowEvalSession, FlowFixture, FlowHost};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

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
fn gumball_transform(fixture: &FlowFixture, ids: &[String], operation: &str, apply: impl Fn(&mut FlowHost, &str) -> bool) -> Option<(Vec<Procedural3dOperation>, Vec<String>)> {
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
pub mod translate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "translate-selection")]
    pub struct TranslateSelection {
        pub node_ids: Vec<String>,
        pub dx: f64,
        pub dy: f64,
        pub dz: f64,
    }

    pub fn handle(payload: &TranslateSelection, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let fixture = &doc.projection.fixture;
        let ids = mesh_selection_ids_typed(&payload.node_ids, &cfg.projection.selected_node_ids);
        let (dx, dy, dz) = (payload.dx, payload.dy, payload.dz);
        match gumball_transform(fixture, &ids, "translate", move |host, transform_id| {
            let current = gumball_widget_offset(host, transform_id);
            let next = [current[0] + dx, current[1] + dy, current[2] + dz];
            host.set_neuron_params(transform_id, &gumball_translate_params_json(next)).is_ok()
        }) {
            Some((operations, new_selection)) => Ok(Emit { document_operations: operations, config_operations: vec![Procedural3dConfigOperation::SetSelection { node_ids: new_selection }], coalesce_key: Some("gumball-translate".into()), ..Default::default() }),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️TranslateSelection

//#region 🔖️RotateSelection
pub mod rotate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "rotate-selection")]
    pub struct RotateSelection {
        pub node_ids: Vec<String>,
        pub ax: f64,
        pub ay: f64,
        pub az: f64,
        pub angle: f64,
    }

    pub fn handle(payload: &RotateSelection, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let fixture = &doc.projection.fixture;
        let ids = mesh_selection_ids_typed(&payload.node_ids, &cfg.projection.selected_node_ids);
        let (ax, ay, az, angle) = (payload.ax, payload.ay, payload.az, payload.angle);
        match gumball_transform(fixture, &ids, "rotate", move |host, transform_id| {
            let current_angle = gumball_widget_number_param(host, transform_id, "angle", 0.0);
            host.set_neuron_params(transform_id, &gumball_rotate_params_json([ax, ay, az], current_angle + angle)).is_ok()
        }) {
            Some((operations, new_selection)) => Ok(Emit { document_operations: operations, config_operations: vec![Procedural3dConfigOperation::SetSelection { node_ids: new_selection }], coalesce_key: Some("gumball-rotate".into()), ..Default::default() }),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️RotateSelection

//#region 🔖️ScaleSelection
pub mod scale_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "scale-selection")]
    pub struct ScaleSelection {
        pub node_ids: Vec<String>,
        pub sx: f64,
        pub sy: f64,
        pub sz: f64,
    }

    pub fn handle(payload: &ScaleSelection, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let fixture = &doc.projection.fixture;
        let ids = mesh_selection_ids_typed(&payload.node_ids, &cfg.projection.selected_node_ids);
        let uniform_factor = (payload.sx + payload.sy + payload.sz) / 3.0;
        match gumball_transform(fixture, &ids, "scale", move |host, transform_id| {
            let current_factor = gumball_widget_number_param(host, transform_id, "factor", 1.0);
            host.set_neuron_params(transform_id, &gumball_scale_params_json(current_factor * uniform_factor)).is_ok()
        }) {
            Some((operations, new_selection)) => Ok(Emit { document_operations: operations, config_operations: vec![Procedural3dConfigOperation::SetSelection { node_ids: new_selection }], coalesce_key: Some("gumball-scale".into()), ..Default::default() }),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ScaleSelection

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, dispatch};
    use crate::apps::procedural3d::Procedural3dCommand;
    use crate::artifacts::procedural3d::widget_id;
    use flow::Widget;

    #[test]
    fn translate_selection_persists_transform_into_flow_graph() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        let before = app.projection().expect("projection");
        assert!(before.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"));
        dispatch(&mut app, Procedural3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0 }));
        let projection = app.projection().expect("projection");
        let transform_id = "extrude__gumball_translate";
        let transform = projection.fixture.widgets.iter().find(|widget| widget_id(widget) == transform_id).expect("transform neuron created");
        assert!(matches!(transform, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.xform.translate"));
        let offset = gumball_widget_offset(&host_from_fixture(&projection.fixture), transform_id);
        assert_eq!(offset, [1.0, 2.0, 3.0]);

        // Re-grabbing the same transform accumulates the delta instead of creating a second node.
        dispatch(&mut app, Procedural3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids: vec![transform_id.into()], dx: 1.0, dy: 0.0, dz: 0.0 }));
        let projection2 = app.projection().expect("projection");
        assert_eq!(projection2.fixture.widgets.iter().filter(|widget| widget_id(widget) == transform_id).count(), 1);
        assert_eq!(gumball_widget_offset(&host_from_fixture(&projection2.fixture), transform_id), [2.0, 2.0, 3.0]);
    }

    #[test]
    fn rotate_and_scale_selection_persist_into_flow_graph() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut rotate_app = app();
        dispatch(&mut rotate_app, Procedural3dCommand::RotateSelection(rotate_selection::RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::FRAC_PI_2 }));
        let rotated = rotate_app.projection().expect("projection");
        let rotate_id = "extrude__gumball_rotate";
        assert!(rotated.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == rotate_id && neuron_kind == "brep.xform.rotate")));

        let mut scale_app = app();
        dispatch(&mut scale_app, Procedural3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids: vec!["extrude".into()], sx: 2.0, sy: 2.0, sz: 2.0 }));
        let scaled = scale_app.projection().expect("projection");
        let scale_id = "extrude__gumball_scale";
        assert!(scaled.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == scale_id && neuron_kind == "brep.xform.scale")));
    }
}
//#endregion 🧪️Tests
