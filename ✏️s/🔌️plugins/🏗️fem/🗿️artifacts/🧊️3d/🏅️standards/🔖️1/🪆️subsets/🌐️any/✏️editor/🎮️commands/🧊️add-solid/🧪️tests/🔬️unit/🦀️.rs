use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_solid::*;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::NoConfig;
use crate::context::dispatch;
use crate::context::fem3d_demo_app;

fn payload(material: &str, width: f64) -> AddSolid {
    AddSolid { x: 0.0, y: 0.0, width, depth: 2.0, height: 0.5, material_id: material.into(), base_z: None, layers: None, mesh_size: None, axis: None }
}

fn refusal(payload: &AddSolid) -> String {
    let snapshot = semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io::fem3d_boot_snapshot().expect("boot snapshot");
    let snapshot = crate::context::Retiring::new(snapshot);
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = NoConfig::default();
    format!("{:?}", handle(payload, &doc, &ConfigView { snapshot: &config, window: None }).expect_err("an invalid solid is refused with a fault"))
}

/// 🧊️ LAW: `addSolid` refuses an unknown material and a non-positive extent with a visible fault, and a valid material still adds the solid.
#[semio_framework_async_macros::async_test]
async fn add_solid_refuses_an_unknown_material_and_a_degenerate_extent() {
    assert!(refusal(&payload("unobtainium", 4.0)).contains("fem3d.solid.material-unknown"));
    let mut app = fem3d_demo_app().await;
    let material = app.snapshot().expect("snapshot").materials[0].id.as_str().to_owned();
    assert!(refusal(&payload(&material, 0.0)).contains("fem3d.solid.extent"));
    let before = app.snapshot().expect("snapshot").solids.len();
    dispatch(&mut app, Fem3dCommand::AddSolid(payload(&material, 4.0))).await;
    assert_eq!(app.snapshot().expect("snapshot").solids.len(), before + 1);
}

/// 🧊️ LAW: the `addSolid` refusals reach the shell as readable EN and DE notices, not as a bare code — the fault carries the `material` param and the editor's notice table fills it.
#[test]
fn add_solid_refusals_reach_the_shell_as_localized_notices_3d() {
    let snapshot = crate::context::Retiring::new(semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io::fem3d_boot_snapshot().expect("boot snapshot"));
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = NoConfig::default();
    let fault = handle(&payload("unobtainium", 4.0), &doc, &ConfigView { snapshot: &config, window: None }).expect_err("refused");
    assert_eq!(fault.code.0.as_str(), "fem3d.solid.material-unknown");
    let table = semio_framework::fault_notice_definitions(<semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dPlayApp as semio_plugin::ArtifactEditor>::fault_notices());
    assert!(semio_framework::validate_fault_notices(&table).is_empty());
    for (locale, expected) in [(semio_ui_locale::Locale::En, "'unobtainium' is not a material"), (semio_ui_locale::Locale::De, "'unobtainium' ist kein Material")] {
        let shown = semio_framework::fault_notice_text(&table, "fem3d.solid.material-unknown", fault.params.as_deref(), semio_ui_locale::Terminology::Native, locale).expect("notice");
        assert!(shown.contains(expected), "{locale:?}: {shown}");
    }
}
