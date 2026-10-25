use super::*;

/// 🤝️ Independently authored node properties remain independent during replay and inverse replay.
#[test]
fn concurrent_node_fields_preserve_both_authors_intent() {
    let base = Fem3dSnapshot { nodes: vec![semio_s_fem_node::volumetric::FemNodeValue { id: "target".into(), x: 0.0, y: 0.0, z: 0.0 }], ..Default::default() };
    let lower = |field: &str, value: &str| {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&base, &history);
        let config = NoConfig::default();
        handle(&PatchNode { id: "target".into(), field: field.into(), value: value.into() }, &doc, &ConfigView { snapshot: &config, window: None }).expect("field intent").artifact_mutations.remove(0)
    };
    let first = lower("x", "3");
    let second = lower("y", "4");
    for sequence in [[&first, &second], [&second, &first]] {
        let mut snapshot = base.clone();
        for mutation in sequence { semio_s_artifact_fem_3d_mutations::central_apply::apply_fem3d_mutation(&mut snapshot, mutation).expect("independent field replay"); }
        assert_eq!(snapshot.nodes[0].x, 3.0);
        assert_eq!(snapshot.nodes[0].y, 4.0);
        for inverse in protocol::ColdMutation::inverse(&first, &base).expect("field inverse").iter().rev() {
            semio_s_artifact_fem_3d_mutations::central_apply::apply_fem3d_mutation(&mut snapshot, inverse).expect("independent field inverse replay");
        }
        assert_eq!(snapshot.nodes[0].x, base.nodes[0].x);
        assert_eq!(snapshot.nodes[0].y, 4.0);
    }
}
use crate::context::dispatch;
use crate::context::fem3d_demo_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
#[cfg(test)] use semio_framework_plugin::component::ArtifactView;
#[cfg(test)] use semio_framework_plugin::component::NoConfig;
#[cfg(test)] use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
#[cfg(test)] use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
#[cfg(test)] use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_node::PatchNode;
#[cfg(test)] use semio_framework_plugin::component::Emit;
#[cfg(test)] use semio_framework_plugin::component::NoConfigMutation;
#[cfg(test)] use semio_framework::Fault;

fn demo() -> Fem3dSnapshot {
    semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot()
}

fn emit(snapshot: &Fem3dSnapshot, payload: PatchNode) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_node_moves_an_ordinate_on_the_live_demo_3d() {
    let mut app = fem3d_demo_app().await;
    dispatch(&mut app, Fem3dCommand::PatchNode(PatchNode { id: "n20_l1".into(), field: "z".into(), value: "3.1".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let node = snapshot.nodes.iter().find(|node| node.id == "n20_l1").expect("n20_l1 survives the patch");
    assert_eq!((node.x, node.y, node.z), (8.0, 0.0, 3.1), "the sparse authored ordinate preserves untouched coordinates");
}

#[semio_framework_async_macros::async_test]
async fn patch_node_emits_only_the_authored_ordinate_3d() {
    let emitted = emit(&demo(), PatchNode { id: "n00_l2".into(), field: "y".into(), value: "0.5".into() }).expect("handle");
    let [Fem3dMutation::ChangeNode(change)] = emitted.artifact_mutations.as_slice() else { panic!("one sparse change-node") };
    assert_eq!(change.id, "n00_l2");
    assert_eq!(change.patch, semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dNodePatch { y: Some(0.5), ..Default::default() });
}

#[semio_framework_async_macros::async_test]
async fn patch_node_rejects_an_unknown_field_an_unparsable_value_and_a_missing_node_3d() {
    let demo = demo();
    assert!(emit(&demo, PatchNode { id: "n00_g".into(), field: "w".into(), value: "1".into() }).is_err());
    assert!(emit(&demo, PatchNode { id: "n00_g".into(), field: "x".into(), value: "over there".into() }).is_err());
    assert!(emit(&demo, PatchNode { id: "nowhere".into(), field: "x".into(), value: "1".into() }).is_err());
    assert!(emit(&demo, PatchNode { id: "n00_g".into(), field: "x".into(), value: "0".into() }).expect("handle").artifact_mutations.is_empty(), "an edit that changes nothing must not open a revision");
}
