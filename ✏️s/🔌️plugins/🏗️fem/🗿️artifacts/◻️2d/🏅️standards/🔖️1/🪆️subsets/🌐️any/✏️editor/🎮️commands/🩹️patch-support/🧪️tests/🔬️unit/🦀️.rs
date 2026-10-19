use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use super::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::context::dispatch;
use crate::context::fem2d_app;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use pack::snapshot::ArtifactDsl;
#[cfg(test)] use semio_framework_plugin::component::ArtifactView;
#[cfg(test)] use semio_framework_plugin::component::NoConfig;
#[cfg(test)] use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
#[cfg(test)] use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_support::PatchSupport;
#[cfg(test)] use semio_framework_plugin::component::Emit;
#[cfg(test)] use semio_framework_plugin::component::NoConfigMutation;
#[cfg(test)] use semio_framework::Fault;

fn demo() -> Fem2dSnapshot {
    semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).expect("the bundled demo fixture parses")
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
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: semio_s_artifact_fem_2d_io::examples::demo::ID.into() })).await;
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
    let [Fem2dMutation::ChangeSupport(change)] = emitted.artifact_mutations.as_slice() else { panic!("one change-support") };
    assert_eq!(change.patch.node_id.as_deref().unwrap(), "p0_l1");
    assert_eq!(change.id, "s2");
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

/// 👥️ Two inspector commands authored on one base compose their independent intent in either replay order.
#[test]
fn sibling_field_commands_and_inverse_preserve_concurrent_intent(){
    let base=semio_framework_pack_json::from_json_str::<Fem2dSnapshot>(include_str!("../../../../../../🛡️boundary/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-support/🩹️changes/📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral fixture");
    let first=emit(&base,PatchSupport{id:"s1".into(),field:"tx".into(),value:"true".into()}).unwrap().artifact_mutations.remove(0);
    let second=emit(&base,PatchSupport{id:"s1".into(),field:"ty".into(),value:"true".into()}).unwrap().artifact_mutations.remove(0);
    let first_then_second=vcs::apply_mutation(&vcs::apply_mutation(&base,&first).unwrap().0,&second).unwrap().0;
    let combined=vcs::apply_mutation(&vcs::apply_mutation(&base,&second).unwrap().0,&first).unwrap().0;
    assert_eq!(combined,first_then_second);
    assert_eq!(combined.supports[0].fixed,vec![crate::FemDof::Tx,crate::FemDof::Ty,crate::FemDof::Rz]);
    let mut undo=combined.clone();for inverse in protocol::Mutation::inverse(&first,&base).unwrap().iter().rev(){undo=vcs::apply_mutation(&undo,inverse).unwrap().0;}
    assert_eq!(undo,vcs::apply_mutation(&base,&second).unwrap().0,"undo owns no sibling field");
    let first_diff=protocol::Mutation::diff(&first,&base).into_parts().0;let second_diff=protocol::Mutation::diff(&second,&base).into_parts().0;
    let mut sum=first_diff.clone();protocol::ColdMutationDiff::absorb(&mut sum,second_diff);
    assert_eq!(protocol::apply_diff_cold(&sum,&base).unwrap(),combined);
    assert_eq!(protocol::apply_diff_cold(&protocol::DiffAlgebra::inverse(&sum,&base),&combined).unwrap(),base);
    eprintln!("[DEBUG] support same-base inspector fields and sparse inverse preserve concurrent sibling intent");
}