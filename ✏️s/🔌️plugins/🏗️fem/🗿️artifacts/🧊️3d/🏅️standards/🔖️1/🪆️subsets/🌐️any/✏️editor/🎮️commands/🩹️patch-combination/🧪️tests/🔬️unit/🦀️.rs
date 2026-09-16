use super::*;
use crate::editor::fem3d::unit_tests::context::{dispatch, fem3d_app};
use crate::editor::fem3d::Fem3dCommand;

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
    assert!(emit(&snapshot, PatchCombination { id: "uls".into(), field: ADD_TERM_FIELD.into(), value: "".into() }).expect("handle").artifact_mutations.is_empty());
    assert!(emit(&snapshot, PatchCombination { id: "uls".into(), field: format!("{TERM_FIELD_PREFIX}snow"), value: "1".into() }).is_err(), "a term the combination does not carry cannot be re-weighted");
    assert!(emit(&snapshot, PatchCombination { id: "uls".into(), field: "colour".into(), value: "x".into() }).is_err());
    assert!(emit(&snapshot, PatchCombination { id: "uls".into(), field: format!("{TERM_FIELD_PREFIX}live"), value: "heavy".into() }).is_err());
}
