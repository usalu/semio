use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use super::*;

/// 🤝️ Independently authored section properties remain independent during replay and inverse replay.
#[test]
fn concurrent_section_fields_preserve_both_authors_intent() {
    let base = Fem2dSnapshot { sections: vec![semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSection { id: "target".into(), name: "Original".into(), area: 0.01, iy: 1e-6 }], ..Default::default() };
    let lower = |field: &str, value: &str| {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&base, &history);
        let config = NoConfig::default();
        handle(&PatchSection { id: "target".into(), field: field.into(), value: value.into() }, &doc, &ConfigView { snapshot: &config, window: None }).expect("field intent").artifact_mutations.remove(0)
    };
    let first = lower("name", "Edited Section");
    let second = lower("area", "0.02");
    for sequence in [[&first, &second], [&second, &first]] {
        let mut snapshot = base.clone();
        for mutation in sequence { semio_s_artifact_fem_2d_mutations::central_apply::apply_fem2d_mutation(&mut snapshot, mutation).expect("independent field replay"); }
        assert_eq!(snapshot.sections[0].name, "Edited Section");
        assert_eq!(snapshot.sections[0].area, 0.02);
        for inverse in protocol::ColdMutation::inverse(&first, &base).expect("field inverse").iter().rev() {
            semio_s_artifact_fem_2d_mutations::central_apply::apply_fem2d_mutation(&mut snapshot, inverse).expect("independent field inverse replay");
        }
        assert_eq!(snapshot.sections[0].name, base.sections[0].name);
        assert_eq!(snapshot.sections[0].area, 0.02);
    }
}
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::context::dispatch;
use crate::context::fem2d_app;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use pack::snapshot::ArtifactDsl;
#[cfg(test)] use semio_framework_plugin::component::ArtifactView;
#[cfg(test)] use semio_framework_plugin::component::NoConfig;
#[cfg(test)] use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
#[cfg(test)] use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_section::PatchSection;
#[cfg(test)] use semio_framework_plugin::component::Emit;
#[cfg(test)] use semio_framework_plugin::component::NoConfigMutation;
#[cfg(test)] use semio_framework::Fault;

fn demo() -> Fem2dSnapshot {
    semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).expect("the bundled demo fixture parses")
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
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: semio_s_artifact_fem_2d_io::examples::demo::ID.into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchSection(PatchSection { id: "rafter".into(), field: "area".into(), value: "0.02".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let section = snapshot.sections.iter().find(|section| section.id == "rafter").expect("rafter survives the patch");
    assert_eq!(section.area, 0.02);
    assert_eq!(section.iy, 0.000039, "every field but the edited one is carried through");
}

#[semio_framework_async_macros::async_test]
async fn patch_section_emits_one_property_intent_2d() {
    let emitted = emit(&demo(), PatchSection { id: "post140".into(), field: "iy".into(), value: "0.00005".into() }).expect("handle");
    let [Fem2dMutation::ChangeSection(change)] = emitted.artifact_mutations.as_slice() else { panic!("one change-section") };
    assert_eq!(change.id, "post140");
    assert_eq!((change.patch.area, change.patch.iy), (None, Some(0.00005)));
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
