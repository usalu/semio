use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use super::*;

/// 🤝️ Independently authored node properties remain independent during replay and inverse replay.
#[test]
fn concurrent_node_fields_preserve_both_authors_intent() {
    let base = Fem2dSnapshot { nodes: vec![semio_s_fem_node::planar::FemNodeValue { id: "target".into(), x: 0.0, y: 0.0 }], ..Default::default() };
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
        for mutation in sequence { semio_s_artifact_fem_2d_mutations::central_apply::apply_fem2d_mutation(&mut snapshot, mutation).expect("independent field replay"); }
        assert_eq!(snapshot.nodes[0].x, 3.0);
        assert_eq!(snapshot.nodes[0].y, 4.0);
        for inverse in protocol::Mutation::inverse(&first, &base).expect("field inverse").iter().rev() {
            semio_s_artifact_fem_2d_mutations::central_apply::apply_fem2d_mutation(&mut snapshot, inverse).expect("independent field inverse replay");
        }
        assert_eq!(snapshot.nodes[0].x, base.nodes[0].x);
        assert_eq!(snapshot.nodes[0].y, 4.0);
    }
}
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::context::dispatch;
use crate::context::fem2d_app;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use pack::snapshot::ArtifactDsl;
#[cfg(test)] use semio_framework_plugin::component::ArtifactView;
#[cfg(test)] use semio_framework_plugin::component::NoConfig;
#[cfg(test)] use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
#[cfg(test)] use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_node::PatchNode;
#[cfg(test)] use semio_framework_plugin::component::Emit;
#[cfg(test)] use semio_framework_plugin::component::NoConfigMutation;
#[cfg(test)] use semio_framework::Fault;

fn demo() -> Fem2dSnapshot {
    semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).expect("the bundled demo fixture parses")
}

fn emit(snapshot: &Fem2dSnapshot, payload: PatchNode) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_node_moves_an_ordinate_on_the_demo_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: semio_s_artifact_fem_2d_io::examples::demo::ID.into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchNode(PatchNode { id: "n2".into(), field: "x".into(), value: "1.5".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let node = snapshot.nodes.iter().find(|node| node.id == "n2").expect("n2 survives the patch");
    assert_eq!(node.x, 1.5);
    assert_eq!(node.y, 0.0, "the untouched ordinate is carried through the whole-record replace");
}

#[semio_framework_async_macros::async_test]
async fn patch_node_emits_one_coordinate_intent_2d() {
    let emitted = emit(&demo(), PatchNode { id: "ridge".into(), field: "y".into(), value: "8.2".into() }).expect("handle");
    let [Fem2dMutation::ChangeNode(change)] = emitted.artifact_mutations.as_slice() else { panic!("one change-node") };
    assert_eq!(change.id, "ridge");
    assert_eq!((change.patch.x, change.patch.y), (None, Some(8.2)));
}

#[semio_framework_async_macros::async_test]
async fn patch_node_rejects_an_unknown_field_an_unparsable_value_and_a_missing_node_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchNode { id: "n2".into(), field: "z".into(), value: "1".into() }).is_err());
    assert!(emit(&demo, PatchNode { id: "n2".into(), field: "x".into(), value: "over there".into() }).is_err());
    assert!(emit(&demo, PatchNode { id: "nowhere".into(), field: "x".into(), value: "1".into() }).is_err());
}

#[semio_framework_async_macros::async_test]
async fn patch_node_with_the_current_ordinate_emits_nothing_2d() {
    let emitted = emit(&demo(), PatchNode { id: "n2".into(), field: "y".into(), value: "0".into() }).expect("handle");
    assert!(emitted.artifact_mutations.is_empty(), "an edit that changes nothing must not open a revision");
}
