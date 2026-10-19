use super::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
use crate::context::dispatch;
use crate::context::fem3d_demo_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchSupport) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_support_toggles_dofs_in_canonical_order_and_repoints_the_node_3d() {
    let mut app = fem3d_demo_app().await;
    dispatch(&mut app, Fem3dCommand::PatchSupport(PatchSupport { id: "ss_0".into(), field: "rz".into(), value: "true".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchSupport(PatchSupport { id: "ss_0".into(), field: "tx".into(), value: "false".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let support = snapshot.supports.iter().find(|support| support.id == "ss_0").expect("ss_0");
    assert_eq!(support.fixed, vec![FemDof::Ty, FemDof::Tz, FemDof::Rz]);
    let emitted = emit(&snapshot, PatchSupport { id: "ss_0".into(), field: "nodeId".into(), value: "sc1".into() }).expect("handle");
    let [Fem3dMutation::ChangeSupport(change)] = emitted.artifact_mutations.as_slice() else { panic!("one change-support") };
    assert_eq!(change.patch.node_id.as_deref().unwrap(), "sc1");
    assert!(emit(&snapshot, PatchSupport { id: "ss_0".into(), field: "tz".into(), value: "maybe".into() }).is_err());
    assert!(emit(&snapshot, PatchSupport { id: "ss_0".into(), field: "tw".into(), value: "true".into() }).is_err());
    assert!(emit(&snapshot, PatchSupport { id: "ss_0".into(), field: "ty".into(), value: "true".into() }).expect("handle").artifact_mutations.is_empty());
}

/// 👥️ Two inspector commands authored on one base compose their independent intent in either replay order.
#[test]
fn sibling_field_commands_and_inverse_preserve_concurrent_intent(){
    let base=semio_framework_pack_json::from_json_str::<Fem3dSnapshot>(include_str!("../../../../../../🛡️boundary/🧫️fixtures/🧬️mutations/🩹️change-support/🩹️changes/📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral fixture");
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