use super::*;
use crate::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app};
use crate::editor::fem2d::Fem2dCommand;
use store::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("the bundled demo fixture parses")
}

fn emit(snapshot: &Fem2dSnapshot, payload: PatchSection) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_section_edits_the_area_on_the_demo_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchSection(PatchSection { id: "rafter".into(), field: "area".into(), value: "0.02".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let section = snapshot.sections.iter().find(|section| section.id == "rafter").expect("rafter survives the patch");
    assert_eq!(section.area, 0.02);
    assert_eq!(section.iy, 0.000039, "every field but the edited one is carried through");
}

#[semio_framework_async_macros::async_test]
async fn patch_section_rejects_an_unknown_field_an_unparsable_value_and_a_missing_section_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchSection { id: "rafter".into(), field: "iz".into(), value: "1".into() }).is_err());
    assert!(emit(&demo, PatchSection { id: "rafter".into(), field: "iy".into(), value: "thick".into() }).is_err());
    assert!(emit(&demo, PatchSection { id: "nowhere".into(), field: "area".into(), value: "1".into() }).is_err());
}

#[semio_framework_async_macros::async_test]
async fn patch_section_with_the_current_value_emits_nothing_2d() {
    let emitted = emit(&demo(), PatchSection { id: "rafter".into(), field: "name".into(), value: "Timber Rafter 80x180".into() }).expect("handle");
    assert!(emitted.artifact_mutations.is_empty());
}
