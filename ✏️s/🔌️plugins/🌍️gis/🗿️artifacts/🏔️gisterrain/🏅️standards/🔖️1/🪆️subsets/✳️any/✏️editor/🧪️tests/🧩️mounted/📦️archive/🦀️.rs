//! 📦️ Reopens modified Terrain through the registered document archive owner.

use super::artifact::editor::gis3d::{Gis3dCommand, Gis3dPlayApp};
use super::artifact::editor::gis3d::commands::overlays::add_overlay_position::AddOverlayPosition;
use super::context::{app, close, dispatch, history_verb};
use semio_framework_pack_json::{JsonMemberPolicy, from_json_str};
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::DslValue;
use semio_plugin::artifact_app_laws as laws;
use semio_plugin::{ArtifactEditor, PluginApp};
use serde_json::Value;

/// 🗃️ Restores both authored data and reversible History from the registered archive.
#[semio_framework_async_macros::async_test]
async fn edited_terrain_archive_reopens_with_replayable_raster_and_overlay() {
    const OPERATION: u64 = 0x4754_4152;
    let fixture: Value = serde_json::from_str(fixture_text!("../../../🎮️commands/🏔️elevation-source/➕️create-grid/🧪️tests/🔣️.json")).unwrap();
    let args = from_json_str::<DslValue>(&fixture["command"].to_string(), JsonMemberPolicy::Reject).unwrap();
    let command = Gis3dPlayApp::command_from_action("createElevationGrid", &mut Some(args), &mut None).unwrap();
    let mut app = app().await;
    let initial = app.snapshot().unwrap();
    dispatch(&mut app, command).await;
    let raster = app.snapshot().unwrap();
    assert_ne!(raster.elevation_source, initial.elevation_source);

    dispatch(&mut app, Gis3dCommand::AddOverlayPosition(AddOverlayPosition {
        id: "archive-survey".into(),
        longitude: -1.25,
        latitude: 49.85,
        label: Some("Archive Survey".into()),
        elevation: None,
    })).await;
    let edited = app.snapshot().unwrap();
    assert_ne!(edited.imported_map, raster.imported_map);
    let archive = PluginApp::document_archive(&*app.0).await.expect("export edited Terrain archive");

    history_verb(&mut app, "undo").await;
    assert_eq!(app.snapshot().unwrap(), raster);
    laws::admit_document_archive_load(&mut *app.0, OPERATION, archive, app.1.cx()).expect("admit edited Terrain archive");
    let result = laws::settle_document_archive_load(&mut *app.0, OPERATION, &mut laws::fixture_identity(), app.1.cx()).await;
    result.unwrap_or_else(|failure| panic!("reopen edited Terrain archive: {failure:?}; {}", app.close_diagnostic()));
    assert_eq!(app.snapshot().unwrap(), edited);

    for (verb, snapshot) in [("undo", &raster), ("undo", &initial), ("redo", &raster), ("redo", &edited)] {
        history_verb(&mut app, verb).await;
        assert_eq!(&app.snapshot().unwrap(), snapshot);
    }
    close(&mut app);
}
