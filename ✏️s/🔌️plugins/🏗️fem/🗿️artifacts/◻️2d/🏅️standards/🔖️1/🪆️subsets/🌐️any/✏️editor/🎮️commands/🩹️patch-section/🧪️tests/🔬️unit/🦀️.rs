use crate::context::{dispatch, fem2d_app};
use semio_os_kernel as protocol;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_section::{PatchSection, handle};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSection;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::{Fem2dSnapshot, Fem2dSnapshotRetiring, Fem2dSnapshotValue};
use semio_s_artifact_fem_2d_mutations::cold_laws::apply_cold_mutation;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;

/// 🤝️ Independently authored section properties remain independent during replay and inverse replay.
#[test]
fn concurrent_section_fields_preserve_both_authors_intent() {
    let base = Fem2dSnapshotValue { sections: vec![FemSection { id: "target".into(), name: "Original".into(), area: 0.01, iy: 1e-6 }], ..Default::default() };
    let accepted = Fem2dSnapshotRetiring::new(Fem2dSnapshot::from_reference(base.clone()).expect("admit the section fixture"));
    let lower = |field: &str, value: &str| {
        let history = semio_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&*accepted, &history);
        let config = NoConfig::default();
        handle(&PatchSection { id: "target".into(), field: field.into(), value: value.into() }, &doc, &ConfigView { snapshot: &config, window: None }).expect("field intent").artifact_mutations.remove(0)
    };
    let first = lower("name", "Edited Section");
    let second = lower("area", "0.02");
    for sequence in [[&first, &second], [&second, &first]] {
        let mut snapshot = base.clone();
        for mutation in sequence { snapshot = apply_cold_mutation(&snapshot, mutation).expect("independent field replay").0; }
        assert_eq!(snapshot.sections[0].name, "Edited Section");
        assert_eq!(snapshot.sections[0].area, 0.02);
        for inverse in protocol::ColdMutation::inverse(&first, &base).expect("field inverse").iter().rev() {
            snapshot = apply_cold_mutation(&snapshot, inverse).expect("independent field inverse replay").0;
        }
        assert_eq!(snapshot.sections[0].name, base.sections[0].name);
        assert_eq!(snapshot.sections[0].area, 0.02);
    }
}

fn demo() -> Fem2dSnapshotRetiring {
    Fem2dSnapshotRetiring::new(default_fem2d_snapshot().expect("the bundled demo fixture parses"))
}

fn emit(snapshot: &Fem2dSnapshot, payload: PatchSection) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_section_edits_the_area_on_the_demo_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::PatchSection(PatchSection { id: "rafter".into(), field: "area".into(), value: "0.02".into() })).await;
    let snapshot = app.snapshot_ref();
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
