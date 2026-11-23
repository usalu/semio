use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_material::*;
use semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::replace_material;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use crate::context::dispatch;
use crate::context::fem3d_demo_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
#[cfg(test)] use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

fn demo() -> crate::context::Retiring<Fem3dSnapshot> {
    crate::context::Retiring::new(Fem3dSnapshot::from_reference(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue { materials: vec![semio_s_fem_material::volumetric::FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80e9, nu: 0.3, rho: 7850.0 }], ..Default::default() }).expect("demo snapshot"))
}

fn emit(snapshot: &Fem3dSnapshot, payload: PatchMaterial) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

/// 🤝️ Commands authored on the same base retain both independently edited material fields in either order.
#[test]
fn concurrent_material_fields_preserve_both_authors_intent() {
    let base = demo();
    let name = emit(&base, PatchMaterial { id: "steel".into(), field: "name".into(), value: "Steel S355".into() }).expect("name intent").artifact_mutations.remove(0);
    let elasticity = emit(&base, PatchMaterial { id: "steel".into(), field: "e".into(), value: "190000000000".into() }).expect("elasticity intent").artifact_mutations.remove(0);
    for sequence in [[&name, &elasticity], [&elasticity, &name]] {
        let mut snapshot = crate::context::Retiring::new(base.clone());
        for mutation in sequence { snapshot = crate::context::applied(&snapshot, mutation); }
        let material = snapshot.materials.iter().find(|material| material.id.as_str() == "steel").expect("steel");
        assert_eq!(material.name.as_str(), "Steel S355");
        assert_eq!(material.e, 190000000000.0);
    }
    let mut snapshot = crate::context::Retiring::new(base.clone());
    for mutation in [&name, &elasticity] { snapshot = crate::context::applied(&snapshot, mutation); }
    let inverse = protocol::ColdMutation::inverse(&name, &*base).expect("field inverse");
    for mutation in inverse.iter().rev() { snapshot = crate::context::applied(&snapshot, mutation); }
    let material = snapshot.materials.iter().find(|material| material.id.as_str() == "steel").expect("steel");
    assert_eq!(material.name, base.materials.iter().find(|material| material.id.as_str() == "steel").expect("base steel").name.clone());
    assert_eq!(material.e, 190000000000.0);
}

#[semio_framework_async_macros::async_test]
async fn patch_material_edits_one_property_on_the_live_demo_3d() {
    let mut app = fem3d_demo_app().await;
    dispatch(&mut app, Fem3dCommand::PatchMaterial(PatchMaterial { id: "concrete".into(), field: "nu".into(), value: "0.25".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchMaterial(PatchMaterial { id: "concrete".into(), field: "name".into(), value: "C35/45".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let material = snapshot.materials.iter().find(|material| material.id.as_str() == "concrete").expect("concrete");
    assert_eq!((material.nu, material.name.as_str(), material.e), (0.25, "C35/45", 33e9));
    assert!(emit(&snapshot, PatchMaterial { id: "concrete".into(), field: "e".into(), value: "soft".into() }).is_err());
    assert!(emit(&snapshot, PatchMaterial { id: "concrete".into(), field: "colour".into(), value: "grey".into() }).is_err());
    assert!(emit(&snapshot, PatchMaterial { id: "concrete".into(), field: "nu".into(), value: "0.25".into() }).expect("handle").artifact_mutations.is_empty());
}

/// ⏱️ Same-field material edits resolve through chronological replay without restoring stale siblings.
#[test]
fn concurrent_material_same_field_last_replay_wins() {
    let base = demo();
    let first = emit(&base, PatchMaterial { id: "steel".into(), field: "name".into(), value: "First Author".into() }).expect("first intent").artifact_mutations.remove(0);
    let second = emit(&base, PatchMaterial { id: "steel".into(), field: "name".into(), value: "Second Author".into() }).expect("second intent").artifact_mutations.remove(0);
    for (sequence, expected) in [([&first, &second], "Second Author"), ([&second, &first], "First Author")] {
        let mut snapshot = crate::context::Retiring::new(base.clone());
        for mutation in sequence { snapshot = crate::context::applied(&snapshot, mutation); }
        assert_eq!(snapshot.materials.iter().find(|material| material.id.as_str() == "steel").expect("steel").name.as_str(), expected);
    }
}
