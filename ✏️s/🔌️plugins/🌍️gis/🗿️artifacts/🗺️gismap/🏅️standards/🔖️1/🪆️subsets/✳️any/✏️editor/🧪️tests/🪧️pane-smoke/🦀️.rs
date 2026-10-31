//! 🪧️ Verfolgen pane smoke: the bundled GIS map demo loads, one `AddFeature` applies, undo restores the loaded snapshot and the app closes through its exact retained close.

use crate::unit_tests::context::app;
use semio_s_artifact_gis_gismap_gis2d::editor::gis2d::commands::features::add_feature;
use semio_s_artifact_gis_gismap_gis2d::editor::gis2d::Gis2dCommand;

/// 🌍️ LAW: the Verfolgen pane loads its bundled demo map, adds one position, undoes it and closes with the store drop witness satisfied.
#[semio_framework_async_macros::async_test]
async fn verfolgen_pane_loads_applies_undoes_and_closes() {
    let mut app = app().await;
    let command = Gis2dCommand::AddFeature(add_feature::AddFeature { collection: "positions".into(), label: "Depot".into(), lon: 7.5, lat: 51.0, span: 0.01 });
    let (loaded, applied) = semio_framework_plugin::artifact_app_laws::assert_pane_smoke(&mut app.0, semio_s_artifact_gis_gismap_demo_asset::DEMO_DSL, command, |app| app.snapshot().expect("projection"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    assert!(!loaded.positions.is_empty(), "the loaded demo carries its positions");
    assert_eq!(applied.positions.len(), loaded.positions.len() + 1);
}
