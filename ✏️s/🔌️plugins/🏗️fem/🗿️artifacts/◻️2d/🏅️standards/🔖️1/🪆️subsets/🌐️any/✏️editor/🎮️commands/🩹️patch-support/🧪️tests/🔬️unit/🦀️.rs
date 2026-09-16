use super::*;
use crate::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app};
use crate::editor::fem2d::Fem2dCommand;
use store::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("the bundled demo fixture parses")
}

fn emit(snapshot: &Fem2dSnapshot, payload: PatchSupport) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_support_restrains_and_releases_a_dof_on_the_demo_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchSupport(PatchSupport { id: "s1".into(), field: "rz".into(), value: "true".into() })).await;
    let fixed = app.snapshot().expect("snapshot").supports.iter().find(|support| support.id == "s1").expect("s1").fixed.clone();
    assert_eq!(fixed, vec![FemDof::Tx, FemDof::Ty, FemDof::Rz], "a restrained set is always emitted in canonical DOF order");
    dispatch(&mut app, Fem2dCommand::PatchSupport(PatchSupport { id: "s1".into(), field: "tx".into(), value: "false".into() })).await;
    let fixed = app.snapshot().expect("snapshot").supports.iter().find(|support| support.id == "s1").expect("s1").fixed.clone();
    assert_eq!(fixed, vec![FemDof::Ty, FemDof::Rz]);
}

#[semio_framework_async_macros::async_test]
async fn patch_support_retargets_the_node_2d() {
    let emitted = emit(&demo(), PatchSupport { id: "s2".into(), field: "nodeId".into(), value: "p0_l1".into() }).expect("handle");
    let [Fem2dMutation::ReplaceSupport(replace)] = emitted.artifact_mutations.as_slice() else { panic!("one replace-support") };
    assert_eq!(replace.new_support.node_id, "p0_l1");
    assert_eq!(replace.new_support.id, "s2");
}

#[semio_framework_async_macros::async_test]
async fn patch_support_rejects_an_unknown_field_an_unparsable_flag_and_a_missing_support_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchSupport { id: "s1".into(), field: "tz".into(), value: "true".into() }).is_err());
    assert!(emit(&demo, PatchSupport { id: "s1".into(), field: "tx".into(), value: "maybe".into() }).is_err());
    assert!(emit(&demo, PatchSupport { id: "nowhere".into(), field: "tx".into(), value: "true".into() }).is_err());
}

#[semio_framework_async_macros::async_test]
async fn patch_support_restraining_an_already_restrained_dof_emits_nothing_2d() {
    let emitted = emit(&demo(), PatchSupport { id: "s1".into(), field: "ty".into(), value: "true".into() }).expect("handle");
    assert!(emitted.artifact_mutations.is_empty());
}
