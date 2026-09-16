use super::*;
use crate::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app};
use crate::editor::fem2d::Fem2dCommand;
use store::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("the bundled demo fixture parses")
}

fn emit(snapshot: &Fem2dSnapshot, payload: PatchMaterial) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_material_edits_the_poisson_ratio_on_the_demo_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchMaterial(PatchMaterial { id: "timber".into(), field: "nu".into(), value: "0.25".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let material = snapshot.materials.iter().find(|material| material.id == "timber").expect("timber survives the patch");
    assert_eq!(material.nu, 0.25);
    assert_eq!(material.rho, 450.0, "every field but the edited one is carried through");
}

#[semio_framework_async_macros::async_test]
async fn patch_material_renames_without_touching_the_identity_2d() {
    let emitted = emit(&demo(), PatchMaterial { id: "steel".into(), field: "name".into(), value: "Steel S355".into() }).expect("handle");
    let [Fem2dMutation::ReplaceMaterial(replace)] = emitted.artifact_mutations.as_slice() else { panic!("one replace-material") };
    assert_eq!(replace.new_material.id, "steel", "a display-name edit may never rename the record's identity");
    assert_eq!(replace.new_material.name, "Steel S355");
}

#[semio_framework_async_macros::async_test]
async fn patch_material_rejects_an_unknown_field_an_unparsable_value_and_a_missing_material_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchMaterial { id: "steel".into(), field: "colour".into(), value: "grey".into() }).is_err());
    assert!(emit(&demo, PatchMaterial { id: "steel".into(), field: "e".into(), value: "stiff".into() }).is_err());
    assert!(emit(&demo, PatchMaterial { id: "nowhere".into(), field: "e".into(), value: "1".into() }).is_err());
}

#[semio_framework_async_macros::async_test]
async fn patch_material_with_the_current_value_emits_nothing_2d() {
    let emitted = emit(&demo(), PatchMaterial { id: "steel".into(), field: "nu".into(), value: "0.3".into() }).expect("handle");
    assert!(emitted.artifact_mutations.is_empty());
}
