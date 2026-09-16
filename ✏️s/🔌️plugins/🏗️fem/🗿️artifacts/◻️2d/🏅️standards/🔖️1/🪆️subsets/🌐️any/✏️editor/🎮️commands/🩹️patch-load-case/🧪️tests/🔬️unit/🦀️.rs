use super::*;
use crate::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app};
use crate::editor::fem2d::Fem2dCommand;
use store::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("the bundled demo fixture parses")
}

fn emit(snapshot: &Fem2dSnapshot, payload: PatchLoadCase) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_load_case_renames_and_toggles_self_weight_on_the_demo_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchLoadCase(PatchLoadCase { id: "live".into(), field: "name".into(), value: "Imposed Load".into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchLoadCase(PatchLoadCase { id: "live".into(), field: "selfWeight".into(), value: "true".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let case = snapshot.load_cases.iter().find(|case| case.id == "live").expect("live survives the patch");
    assert_eq!(case.name, "Imposed Load");
    assert!(case.self_weight);
    assert_eq!(case.loads.len(), 2, "a narrow change never rewrites the case's own load collection");
}

#[semio_framework_async_macros::async_test]
async fn patch_load_case_picks_the_narrow_change_its_field_names_2d() {
    let demo = demo();
    let renamed = emit(&demo, PatchLoadCase { id: "dead".into(), field: "name".into(), value: "Permanent".into() }).expect("handle");
    assert!(matches!(renamed.artifact_mutations.as_slice(), [Fem2dMutation::ChangeLoadCaseName(_)]));
    let toggled = emit(&demo, PatchLoadCase { id: "dead".into(), field: "selfWeight".into(), value: "false".into() }).expect("handle");
    assert!(matches!(toggled.artifact_mutations.as_slice(), [Fem2dMutation::ChangeLoadCaseSelfWeight(_)]));
}

#[semio_framework_async_macros::async_test]
async fn patch_load_case_rejects_an_unknown_field_an_unparsable_flag_and_a_missing_case_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchLoadCase { id: "dead".into(), field: "loads".into(), value: "[]".into() }).is_err());
    assert!(emit(&demo, PatchLoadCase { id: "dead".into(), field: "selfWeight".into(), value: "sometimes".into() }).is_err());
    assert!(emit(&demo, PatchLoadCase { id: "nowhere".into(), field: "name".into(), value: "X".into() }).is_err());
}

#[semio_framework_async_macros::async_test]
async fn patch_load_case_with_the_current_value_emits_nothing_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchLoadCase { id: "dead".into(), field: "name".into(), value: "Dead Load".into() }).expect("handle").artifact_mutations.is_empty());
    assert!(emit(&demo, PatchLoadCase { id: "dead".into(), field: "selfWeight".into(), value: "true".into() }).expect("handle").artifact_mutations.is_empty());
}
