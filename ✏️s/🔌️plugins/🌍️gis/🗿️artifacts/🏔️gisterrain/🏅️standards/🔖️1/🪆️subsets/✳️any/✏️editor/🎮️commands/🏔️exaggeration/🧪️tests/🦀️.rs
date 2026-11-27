//! 🧪️ Checks the exaggeration command against the language-agnostic fixture that the Ajv and mathjs cross-check also reads.

use super::*;
use crate::editor::gis3d::{unit_tests::context, Gis3dCommand, Gis3dPlayApp, GIS3D_EXAGGERATION_MAXIMUM, GIS3D_EXAGGERATION_MINIMUM, GIS3D_EXAGGERATION_STAGED_DEFAULT};
use crate::mutations::GisTerrainMutation;
use semio_framework::ArgSchema;
use semio_framework_value::DslValue;
use semio_plugin::{ArtifactEditor, HistoryView};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🎮️commands/🏔️exaggeration/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()
}

fn action(json: &Value) -> Option<DslValue> {
    Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&json.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()))
}

#[test]
fn every_fixture_case_becomes_one_invertible_exaggeration_edit() {
    use protocol::ColdMutation;
    let fixture = fixture();
    let before = GisTerrainSnapshot::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&before, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    for case in fixture["cases"].as_array().unwrap() {
        let expected = case["exaggeration"].as_f64().unwrap();
        let command = Gis3dPlayApp::command_from_action("setExaggeration", &mut action(&case["action"]), &mut None).unwrap();
        assert!(matches!(&command, Gis3dCommand::SetExaggeration(payload) if payload.exaggeration == expected), "{case}");
        let emit = command.dispatch(&doc, &cfg).unwrap();
        let [mutation] = emit.artifact_mutations.as_slice() else { panic!("expected one artifact mutation: {case}") };
        assert!(matches!(mutation, GisTerrainMutation::ChangeExaggeration(change) if change.new_exaggeration == expected), "{case}");
        let next = protocol::apply_diff_cold(&mutation.diff(&before).into_parts().0, &before).unwrap();
        assert_eq!(next.exaggeration, expected, "{case}");
        let heights = fixture["heightsMillimetres"].as_array().unwrap().iter().map(|height| (height.as_f64().unwrap() * next.exaggeration).round());
        assert_eq!(heights.collect::<Vec<_>>(), case["scaledMillimetres"].as_array().unwrap().iter().map(|scaled| scaled.as_f64().unwrap()).collect::<Vec<_>>(), "{case}");
    }
}

#[test]
fn the_slider_range_and_the_absent_default_are_the_fixture_bounds() {
    let bounds = &fixture()["bounds"];
    assert_eq!(GIS3D_EXAGGERATION_MINIMUM, bounds["minimum"].as_f64().unwrap());
    assert_eq!(GIS3D_EXAGGERATION_MAXIMUM, bounds["maximum"].as_f64().unwrap());
    assert_eq!(GIS3D_EXAGGERATION_STAGED_DEFAULT, bounds["staged"].as_f64().unwrap());
    let manifest = context::gis3d_app_manifest_for_tests();
    let slider = manifest.definition.actions.iter().find(|action| action.id == "setExaggeration").unwrap().args.iter().find(|arg| arg.id == "value").unwrap();
    assert_eq!(slider.schema, ArgSchema::number(Some(GIS3D_EXAGGERATION_MINIMUM), Some(GIS3D_EXAGGERATION_MAXIMUM), None, false));
    let absent = Gis3dPlayApp::command_from_action("setExaggeration", &mut action(&serde_json::json!({})), &mut None).unwrap();
    assert!(matches!(absent, Gis3dCommand::SetExaggeration(payload) if payload.exaggeration == bounds["absent"].as_f64().unwrap()));
}
