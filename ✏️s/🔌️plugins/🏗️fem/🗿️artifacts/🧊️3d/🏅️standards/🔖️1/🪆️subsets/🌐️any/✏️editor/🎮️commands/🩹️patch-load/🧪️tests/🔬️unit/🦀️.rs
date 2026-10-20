use super::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use crate::context::dispatch;
use crate::context::fem3d_demo_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
#[cfg(test)] use semio_framework_plugin::component::ArtifactView;
#[cfg(test)] use semio_framework_plugin::component::NoConfig;
#[cfg(test)] use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
#[cfg(test)] use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
#[cfg(test)] use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_load::PatchLoad;
#[cfg(test)] use semio_framework_plugin::component::Emit;
#[cfg(test)] use semio_framework_plugin::component::NoConfigMutation;
#[cfg(test)] use semio_framework::Fault;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchLoad) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_load_edits_each_variant_through_its_owning_case_3d() {
    let mut app = fem3d_demo_app().await;
    dispatch(&mut app, Fem3dCommand::PatchLoad(PatchLoad { id: semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_load_target("live", "l2").unwrap(), field: "value".into(), value: "-7500".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchLoad(PatchLoad { id: semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_load_target("live", "l3").unwrap(), field: "pressure".into(), value: "2000".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let live = snapshot.load_cases.iter().find(|case| case.id == "live").expect("live");
    assert!(matches!(&live.loads[0], FemLoad::Nodal { value, .. } if *value == -7500.0));
    assert!(matches!(&live.loads[1], FemLoad::Area { pressure, .. } if *pressure == 2000.0));
    let emitted = emit(&snapshot, PatchLoad { id: semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_load_target("live", "l2").unwrap(), field: "dof".into(), value: "Tx".into() }).expect("handle");
    let [Fem3dMutation::ChangeLoad(change)] = emitted.artifact_mutations.as_slice() else { panic!("one change-load") };
    assert_eq!((change.case_id.as_str(), change.id.as_str()), ("live", "l2"));
    assert!(matches!(change.patch, Fem3dLoadPatch::Nodal { dof: Some(FemDof::Tx), node_id: None, value: None }));
    assert!(emit(&snapshot, PatchLoad { id: semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_load_target("live", "l2").unwrap(), field: "wx".into(), value: "1".into() }).is_err(), "a nodal load has no wx");
    assert!(emit(&snapshot, PatchLoad { id: semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_load_target("live", "l2").unwrap(), field: "dof".into(), value: "Tw".into() }).is_err());
    assert!(emit(&snapshot, PatchLoad { id: "ghost".into(), field: "value".into(), value: "1".into() }).is_err());
}

/// 👥️ Same-base field commands preserve both nodal edits and undo only their authored field.
#[test]
fn sibling_load_commands_preserve_independent_intent(){
    let base=semio_framework_pack_json::from_json_str::<Fem3dSnapshot>(include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/🩹️changes/📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let first=emit(&base,PatchLoad{id:semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_load_target("lc1", "l1").unwrap(),field:"value".into(),value:"20".into()}).unwrap().artifact_mutations.remove(0);
    let second=emit(&base,PatchLoad{id:semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_load_target("lc1", "l1").unwrap(),field:"nodeId".into(),value:"n2".into()}).unwrap().artifact_mutations.remove(0);
    let combined=vcs::apply_mutation(&vcs::apply_mutation(&base,&first).unwrap().0,&second).unwrap().0;
    assert_eq!(combined,vcs::apply_mutation(&vcs::apply_mutation(&base,&second).unwrap().0,&first).unwrap().0);
    let FemLoad::Nodal{node_id,value,..}=&combined.load_cases[0].loads[0] else{panic!("nodal variant preserved")};
    assert_eq!(node_id,"n2");assert_eq!(*value,20.0);
    let mut undone=combined.clone();for inverse in protocol::Mutation::inverse(&first,&base).unwrap().iter().rev(){undone=vcs::apply_mutation(&undone,inverse).unwrap().0;}
    assert_eq!(undone,vcs::apply_mutation(&base,&second).unwrap().0);
    let mut sum=protocol::Mutation::diff(&first,&base).into_parts().0;protocol::ColdMutationDiff::absorb(&mut sum,protocol::Mutation::diff(&second,&base).into_parts().0);
    assert_eq!(protocol::apply_diff_cold(&sum,&base).unwrap(),combined);
    assert_eq!(protocol::apply_diff_cold(&protocol::DiffAlgebra::inverse(&sum,&base),&combined).unwrap(),base);
    eprintln!("[DEBUG] nodal magnitude/reference inspector intents compose and undo without sibling loss");
}