use super::*;
use crate::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app};
use crate::editor::fem2d::Fem2dCommand;
use store::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("the bundled demo fixture parses")
}

fn emit(snapshot: &Fem2dSnapshot, payload: PatchRegion) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_region_edits_the_thickness_on_the_demo_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchRegion(PatchRegion { id: "r1".into(), field: "thickness".into(), value: "0.25".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let region = snapshot.regions.iter().find(|region| region.id == "r1").expect("r1 survives the patch");
    assert_eq!(region.thickness, 0.25);
    assert_eq!(region.outline.len(), 4, "the polygon is carried through untouched");
    assert_eq!(region.material_id, "concrete");
}

#[semio_framework_async_macros::async_test]
async fn patch_region_emits_one_whole_record_replace_2d() {
    let emitted = emit(&demo(), PatchRegion { id: "r1".into(), field: "meshSize".into(), value: "0.5".into() }).expect("handle");
    let [Fem2dMutation::ReplaceRegion(replace)] = emitted.artifact_mutations.as_slice() else { panic!("one replace-region") };
    assert_eq!(replace.id, "r1");
    assert_eq!(replace.new_region.mesh_size, 0.5);
    assert_eq!(replace.new_region.outline.len(), 4, "the polygon is carried through untouched");
}

#[semio_framework_async_macros::async_test]
async fn patch_region_rejects_an_unknown_field_an_unparsable_value_and_a_missing_region_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchRegion { id: "r1".into(), field: "outline".into(), value: "[]".into() }).is_err());
    assert!(emit(&demo, PatchRegion { id: "r1".into(), field: "meshSize".into(), value: "fine".into() }).is_err());
    assert!(emit(&demo, PatchRegion { id: "nowhere".into(), field: "thickness".into(), value: "1".into() }).is_err());
}

#[semio_framework_async_macros::async_test]
async fn patch_region_with_the_current_value_emits_nothing_2d() {
    let emitted = emit(&demo(), PatchRegion { id: "r1".into(), field: "materialId".into(), value: "concrete".into() }).expect("handle");
    assert!(emitted.artifact_mutations.is_empty());
}
