//! 🧪️ Checks Esri ASCII Grid admission, raster alignment and Terrain mutation publication.

use super::*;
use crate::editor::gis3d::{Gis3dCommand, Gis3dPlayApp};
use semio_framework_value::ToValue;
use semio_plugin::{ArtifactEditor, Effect, HistoryView};
use semio_framework_test_fixtures::fixture_text;

static SLOPE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../🎮️commands/📥️import-elevation/🧪️tests/🧫️fixtures/🔼️slope.asc"));
#[test]
fn esri_ascii_picker_routes_the_result_to_one_invertible_terrain_edit() {
    use protocol::ColdMutation;

    let before = GisTerrainSnapshot::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&before, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let picked = handle(&ImportElevationGrid {}, &doc, &cfg).unwrap();
    assert!(matches!(picked.effects.as_slice(), [Effect::RequestFileOpen { accept, read_as: Some(kind), import_action, max_file_bytes: Some(MAX_FILE_BYTES), multiple: false, .. }] if accept == ".asc" && kind == "text" && import_action == FILE_ACTION));
    let mut args = Some(DslValue::object([
        ("payload".into(), (*SLOPE).to_value()),
        ("name".into(), "slope.asc".to_value()),
        ("chunk".into(), 0_u32.to_value()),
        ("chunkCount".into(), 1_u32.to_value()),
    ]));
    let command = Gis3dPlayApp::command_from_action(FILE_ACTION, &mut args, &mut None).unwrap();
    assert!(matches!(command, Gis3dCommand::ImportElevationGridFile(_)));
    assert_eq!(args.as_ref().and_then(|args| args.get("payload")).and_then(DslValue::as_str), Some(""));
    let emit = command.dispatch(&doc, &cfg).unwrap();
    let [mutation] = emit.artifact_mutations.as_slice() else { panic!("expected exactly one artifact mutation") };
    let next = protocol::apply_diff_cold(&mutation.diff(&before).into_parts().0, &before).unwrap();
    assert_eq!(next.elevation_source, Some(ascii_grid::decode(*SLOPE).unwrap()));
    let [undo]: [GisTerrainMutation; 1] = mutation.inverse(&before).unwrap().try_into().expect("one inverse");
    let restored = protocol::apply_diff_cold(&undo.diff(&next).into_parts().0, &next).unwrap();
    assert_eq!(restored, before);
}
