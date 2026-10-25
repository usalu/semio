use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_combination::*;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::replace_combination;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use crate::context::dispatch;
use crate::context::fem3d_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
#[cfg(test)] use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchCombination) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_combination_reweights_adds_and_removes_terms_3d() {
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::PatchCombination(PatchCombination { id: "uls".into(), field: format!("{TERM_FIELD_PREFIX}live"), value: "1.65".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchCombination(PatchCombination { id: "uls".into(), field: REMOVE_TERM_FIELD.into(), value: "dead".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let uls = snapshot.combinations.iter().find(|combination| combination.id == "uls").expect("uls");
    assert_eq!(uls.terms.iter().map(|(case, factor)| (case.as_str(), *factor)).collect::<Vec<_>>(), vec![("live", 1.65)]);
    dispatch(&mut app, Fem3dCommand::PatchCombination(PatchCombination { id: "uls".into(), field: ADD_TERM_FIELD.into(), value: "dead".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    assert_eq!(snapshot.combinations[0].terms.get("dead"), Some(&ADDED_TERM_FACTOR));
    assert!(emit(&snapshot, PatchCombination { id: "uls".into(), field: ADD_TERM_FIELD.into(), value: "".into() }).is_err());
    let future=emit(&snapshot, PatchCombination { id: "uls".into(), field: format!("{TERM_FIELD_PREFIX}snow"), value: "1".into() }).expect("keyed upsert");
    assert!(matches!(future.artifact_mutations.as_slice(),[Fem3dMutation::ChangeCombinationTerm(ChangeCombinationTerm{case_id,factor:Some(1.0),..})] if case_id=="snow"));
    assert!(emit(&snapshot, PatchCombination { id: "uls".into(), field: "colour".into(), value: "x".into() }).is_err());
    assert!(emit(&snapshot, PatchCombination { id: "uls".into(), field: format!("{TERM_FIELD_PREFIX}live"), value: "heavy".into() }).is_err());
}

/// 👥️ Keyed term changes and combination name commands preserve independently authored intent.
#[test]
fn combination_name_and_terms_preserve_independent_intent(){
    let base=semio_framework_pack_json::from_json_str::<Fem3dSnapshot>(include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/🩹️renames/📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let name=emit(&base,PatchCombination{id:"c1".into(),field:"name".into(),value:"Independent".into()}).unwrap().artifact_mutations.remove(0);
    let first=emit(&base,PatchCombination{id:"c1".into(),field:format!("{TERM_FIELD_PREFIX}lc1"),value:"2".into()}).unwrap().artifact_mutations.remove(0);
    let second=emit(&base,PatchCombination{id:"c1".into(),field:format!("{TERM_FIELD_PREFIX}lc2"),value:"3".into()}).unwrap().artifact_mutations.remove(0);
    let after_first=vcs::apply_mutation(&base,&first).unwrap().0;
    let after_second=vcs::apply_mutation(&base,&second).unwrap().0;
    let terms=vcs::apply_mutation(&after_first,&second).unwrap().0;
    assert_eq!(terms,vcs::apply_mutation(&after_second,&first).unwrap().0);
    let combined=vcs::apply_mutation(&terms,&name).unwrap().0;
    let mut opposite=vcs::apply_mutation(&base,&name).unwrap().0;opposite=vcs::apply_mutation(&opposite,&second).unwrap().0;opposite=vcs::apply_mutation(&opposite,&first).unwrap().0;
    assert_eq!(combined,opposite);assert_eq!(combined.combinations[0].name,"Independent");
    assert_eq!(combined.combinations[0].terms.get("lc1"),Some(&2.0));assert_eq!(combined.combinations[0].terms.get("lc2"),Some(&3.0));
    let mut undone=combined.clone();for inverse in protocol::ColdMutation::inverse(&first,&base).unwrap().iter().rev(){undone=vcs::apply_mutation(&undone,inverse).unwrap().0;}
    assert_eq!(undone,vcs::apply_mutation(&after_second,&name).unwrap().0);
    let mut sum=protocol::ColdMutation::diff(&first,&base).into_parts().0;protocol::ColdMutationDiff::absorb(&mut sum,protocol::ColdMutation::diff(&second,&base).into_parts().0);protocol::ColdMutationDiff::absorb(&mut sum,protocol::ColdMutation::diff(&name,&base).into_parts().0);
    assert_eq!(protocol::apply_diff_cold(&sum,&base).unwrap(),combined);
    assert_eq!(protocol::apply_diff_cold(&protocol::DiffAlgebra::inverse(&sum,&base),&combined).unwrap(),base);
    eprintln!("[DEBUG] combination name and independent keyed terms preserve all authored fields and sparse undo");
}