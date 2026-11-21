use crate::context::{dispatch, fem2d_app};
use semio_os_kernel as protocol;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_combination::{ADD_TERM_FIELD, APPENDED_TERM_FACTOR, PatchCombination, TERM_FIELD_PREFIX, handle};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::{Fem2dSnapshot, Fem2dSnapshotRetiring, Fem2dSnapshotValue};
use semio_s_artifact_fem_2d_mutations::cold_laws::apply_cold_mutation;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;

fn demo() -> Fem2dSnapshotRetiring {
    Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot().expect("the bundled demo fixture parses"))
}

fn emit(snapshot: &Fem2dSnapshot, payload: PatchCombination) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

fn terms(snapshot: &Fem2dSnapshot) -> Vec<(String, f64)> {
    snapshot.combinations.iter().find(|combination| combination.id == "uls").expect("uls survives the patch").terms.iter().map(|term| (term.case_id.clone(), term.factor)).collect()
}

#[semio_framework_async_macros::async_test]
async fn patch_combination_reweights_a_term_on_the_demo_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::PatchCombination(PatchCombination { id: "uls".into(), field: format!("{TERM_FIELD_PREFIX}live"), value: "1.6".into() })).await;
    assert_eq!(terms(app.snapshot_ref()), vec![("dead".to_string(), 1.35), ("live".to_string(), 1.6)]);
}

#[semio_framework_async_macros::async_test]
async fn patch_combination_drops_a_term_dialed_to_zero_2d() {
    let emitted = emit(&demo(), PatchCombination { id: "uls".into(), field: format!("{TERM_FIELD_PREFIX}live"), value: "0".into() }).expect("handle");
    let [Fem2dMutation::ChangeCombinationTerm(change)] = emitted.artifact_mutations.as_slice() else { panic!("one keyed term intent") };
    assert_eq!(change.case_id, "live");
    assert_eq!(change.factor, None, "zero authors only this term removal");
}

#[semio_framework_async_macros::async_test]
async fn patch_combination_appends_an_unknown_case_and_the_add_term_select_2d() {
    let demo = demo();
    let appended = emit(&demo, PatchCombination { id: "uls".into(), field: format!("{TERM_FIELD_PREFIX}wind"), value: "0.6".into() }).expect("handle");
    let [Fem2dMutation::ChangeCombinationTerm(change)] = appended.artifact_mutations.as_slice() else { panic!("one keyed term intent") };
    assert_eq!(change.case_id, "wind");
    assert_eq!(change.factor, Some(0.6));
    let mut single = demo.to_reference();
    single.combinations[0].terms.retain(|term| term.case_id == "dead");
    let single = Fem2dSnapshotRetiring::new(Fem2dSnapshot::from_reference(single).expect("admit the single-term fixture"));
    let added = emit(&single, PatchCombination { id: "uls".into(), field: ADD_TERM_FIELD.into(), value: "live".into() }).expect("handle");
    let [Fem2dMutation::ChangeCombinationTerm(change)] = added.artifact_mutations.as_slice() else { panic!("one keyed term intent") };
    assert_eq!(change.case_id, "live");
    assert_eq!(change.factor, Some(APPENDED_TERM_FACTOR));
}

#[semio_framework_async_macros::async_test]
async fn patch_combination_rejects_an_unknown_field_an_unparsable_factor_and_a_missing_combination_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchCombination { id: "uls".into(), field: "terms".into(), value: "[]".into() }).is_err());
    assert!(emit(&demo, PatchCombination { id: "uls".into(), field: TERM_FIELD_PREFIX.into(), value: "1".into() }).is_err());
    assert!(emit(&demo, PatchCombination { id: "uls".into(), field: format!("{TERM_FIELD_PREFIX}live"), value: "heavy".into() }).is_err());
    assert!(emit(&demo, PatchCombination { id: "nowhere".into(), field: "name".into(), value: "X".into() }).is_err());
}

#[semio_framework_async_macros::async_test]
async fn patch_combination_with_the_current_factor_emits_nothing_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchCombination { id: "uls".into(), field: format!("{TERM_FIELD_PREFIX}dead"), value: "1.35".into() }).expect("handle").artifact_mutations.is_empty());
    assert!(emit(&demo, PatchCombination { id: "uls".into(), field: ADD_TERM_FIELD.into(), value: "dead".into() }).expect("handle").artifact_mutations.is_empty());
}

/// 👥️ Keyed term changes and combination name commands preserve independently authored intent.
#[test]
fn combination_name_and_terms_preserve_independent_intent(){
    let base=semio_framework_pack_json::from_json_str::<Fem2dSnapshotValue>(include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/🩹️renames/📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let accepted = Fem2dSnapshotRetiring::new(Fem2dSnapshot::from_reference(base.clone()).expect("admit the neutral fixture"));
    let name=emit(&accepted,PatchCombination{id:"c1".into(),field:"name".into(),value:"Independent".into()}).unwrap().artifact_mutations.remove(0);
    let first=emit(&accepted,PatchCombination{id:"c1".into(),field:format!("{TERM_FIELD_PREFIX}lc1"),value:"2".into()}).unwrap().artifact_mutations.remove(0);
    let second=emit(&accepted,PatchCombination{id:"c1".into(),field:format!("{TERM_FIELD_PREFIX}lc2"),value:"3".into()}).unwrap().artifact_mutations.remove(0);
    let after_first=apply_cold_mutation(&base,&first).unwrap().0;
    let after_second=apply_cold_mutation(&base,&second).unwrap().0;
    let terms=apply_cold_mutation(&after_first,&second).unwrap().0;
    assert_eq!(terms,apply_cold_mutation(&after_second,&first).unwrap().0);
    let combined=apply_cold_mutation(&terms,&name).unwrap().0;
    let mut opposite=apply_cold_mutation(&base,&name).unwrap().0;opposite=apply_cold_mutation(&opposite,&second).unwrap().0;opposite=apply_cold_mutation(&opposite,&first).unwrap().0;
    assert_eq!(combined,opposite);assert_eq!(combined.combinations[0].name,"Independent");
    assert_eq!(combined.combinations[0].terms.iter().map(|term|(term.case_id.as_str(),term.factor)).collect::<Vec<_>>(),vec![("lc1",2.0),("lc2",3.0)]);
    let mut undone=combined.clone();for inverse in protocol::ColdMutation::inverse(&first,&base).unwrap().iter().rev(){undone=apply_cold_mutation(&undone,inverse).unwrap().0;}
    assert_eq!(undone,apply_cold_mutation(&after_second,&name).unwrap().0);
    let mut sum=protocol::ColdMutation::diff(&first,&base).into_parts().0;protocol::ColdMutationDiff::<Fem2dSnapshotValue>::absorb(&mut sum,protocol::ColdMutation::diff(&second,&base).into_parts().0);protocol::ColdMutationDiff::<Fem2dSnapshotValue>::absorb(&mut sum,protocol::ColdMutation::diff(&name,&base).into_parts().0);
    assert_eq!(protocol::apply_diff_cold(&sum,&base).unwrap(),combined);
    assert_eq!(protocol::apply_diff_cold(&protocol::ColdDiffAlgebra::inverse(&sum,&base),&combined).unwrap(),base);
}
