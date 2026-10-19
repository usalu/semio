use super::*;

/// 🤝️ Independently authored section properties remain independent during replay and inverse replay.
#[test]
fn concurrent_section_fields_preserve_both_authors_intent() {
    let base = Fem3dSnapshot { sections: vec![semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSection { id: "target".into(), name: "Original".into(), area: 0.01, iy: 1e-6, iz: 1e-6, j: 1e-6 }], ..Default::default() };
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
        for mutation in sequence { semio_s_artifact_fem_3d_mutations::central_apply::apply_fem3d_mutation(&mut snapshot, mutation).expect("independent field replay"); }
        assert_eq!(snapshot.sections[0].name, "Edited Section");
        assert_eq!(snapshot.sections[0].area, 0.02);
        for inverse in protocol::Mutation::inverse(&first, &base).expect("field inverse").iter().rev() {
            semio_s_artifact_fem_3d_mutations::central_apply::apply_fem3d_mutation(&mut snapshot, inverse).expect("independent field inverse replay");
        }
        assert_eq!(snapshot.sections[0].name, base.sections[0].name);
        assert_eq!(snapshot.sections[0].area, 0.02);
    }
}
use crate::context::dispatch;
use crate::context::fem3d_demo_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchSection) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_section_edits_one_property_on_the_live_demo_3d() {
    let mut app = fem3d_demo_app().await;
    dispatch(&mut app, Fem3dCommand::PatchSection(PatchSection { id: "hea200".into(), field: "j".into(), value: "0.0000012".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let section = snapshot.sections.iter().find(|section| section.id == "hea200").expect("hea200");
    assert_eq!((section.j, section.area), (0.0000012, 0.00538));
    assert!(emit(&snapshot, PatchSection { id: "hea200".into(), field: "iy".into(), value: "wide".into() }).is_err());
    assert!(emit(&snapshot, PatchSection { id: "hea200".into(), field: "width".into(), value: "1".into() }).is_err());
    assert!(emit(&snapshot, PatchSection { id: "ghost".into(), field: "area".into(), value: "1".into() }).is_err());
}
