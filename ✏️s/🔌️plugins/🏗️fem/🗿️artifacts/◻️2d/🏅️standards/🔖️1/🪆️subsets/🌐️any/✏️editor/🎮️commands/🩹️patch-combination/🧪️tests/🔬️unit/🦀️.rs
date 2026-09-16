use super::*;
use crate::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app};
use crate::editor::fem2d::Fem2dCommand;
use store::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("the bundled demo fixture parses")
}

fn emit(snapshot: &Fem2dSnapshot, payload: PatchCombination) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
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
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchCombination(PatchCombination { id: "uls".into(), field: format!("{TERM_FIELD_PREFIX}live"), value: "1.6".into() })).await;
    assert_eq!(terms(&app.snapshot().expect("snapshot")), vec![("dead".to_string(), 1.35), ("live".to_string(), 1.6)]);
}

#[semio_framework_async_macros::async_test]
async fn patch_combination_drops_a_term_dialed_to_zero_2d() {
    let emitted = emit(&demo(), PatchCombination { id: "uls".into(), field: format!("{TERM_FIELD_PREFIX}live"), value: "0".into() }).expect("handle");
    let [Fem2dMutation::ReplaceCombination(replace)] = emitted.artifact_mutations.as_slice() else { panic!("one replace-combination") };
    assert_eq!(replace.new_combination.terms.len(), 1, "a zero factor removes the term instead of pinning the case at no weight");
    assert_eq!(replace.new_combination.terms[0].case_id, "dead");
}

#[semio_framework_async_macros::async_test]
async fn patch_combination_appends_an_unknown_case_and_the_add_term_select_2d() {
    let demo = demo();
    let appended = emit(&demo, PatchCombination { id: "uls".into(), field: format!("{TERM_FIELD_PREFIX}wind"), value: "0.6".into() }).expect("handle");
    let [Fem2dMutation::ReplaceCombination(replace)] = appended.artifact_mutations.as_slice() else { panic!("one replace-combination") };
    assert_eq!(replace.new_combination.terms.last().expect("appended term").case_id, "wind");
    let mut single = demo.clone();
    single.combinations[0].terms.retain(|term| term.case_id == "dead");
    let added = emit(&single, PatchCombination { id: "uls".into(), field: ADD_TERM_FIELD.into(), value: "live".into() }).expect("handle");
    let [Fem2dMutation::ReplaceCombination(replace)] = added.artifact_mutations.as_slice() else { panic!("one replace-combination") };
    assert_eq!(replace.new_combination.terms.last().expect("appended term"), &FemCombinationTerm { case_id: "live".into(), factor: APPENDED_TERM_FACTOR });
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
