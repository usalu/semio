//! 🧪️ Portable representation witnesses and independent serde JSON comparison.

use semio_s_artifact_procedural_3d_model::catalogue::Localized;
use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::WidgetInputValue;
use semio_s_artifact_procedural_3d_mut_generations::select_generation::SelectGeneration;
use semio_framework_value::{FromValue, ToValue};

#[test]
fn value_codec_portable_witnesses() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../🔣️value/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let value = |key: &str| semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&fixture[key].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let oracle = |projection: &semio_framework_value::DslValue| serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(projection))).unwrap();
    assert_eq!(oracle(&Localized::from_value(value("localized")).unwrap().to_value()), fixture["localized"]);
    assert!(Localized::from_value(value("localizedUnknown")).is_err());
    for key in ["selectionOmitted", "selectionNull"] {
        assert_eq!(SelectGeneration::from_value(value(key)).unwrap().to_value(), value("selectionNull"));
    }
    assert_eq!(oracle(&SelectGeneration::from_value(value("selectionNamed")).unwrap().to_value()), fixture["selectionNamed"]);
    for key in ["inputNumber", "inputVector", "inputPlane"] {
        assert_eq!(oracle(&WidgetInputValue::from_value(value(key)).unwrap().to_value()), fixture[key]);
    }
    for key in ["inputMissing", "inputForged", "inputShortVector"] {
        assert!(WidgetInputValue::from_value(value(key)).is_err(), "{key}");
    }
    let snapshot = semio_s_artifact_procedural_3d_model::empty_generation3d_snapshot();
    let encoded = snapshot.to_value();
    let decoded = semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot::from_value(encoded).unwrap();
    assert_eq!(decoded, snapshot);
    decoded.retire_cold();
    snapshot.retire_cold();
    assert!(semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot::from_value(value("snapshotMissingGeneration")).is_err());
}

#[test]
fn widget_descriptor_io_admission_precedes_native_creation() {
    use semio_s_artifact_procedural_3d_model::empty_generation3d_snapshot;
    use semio_s_artifact_procedural_3d_model::with_host;
    use semio_s_artifact_procedural_3d_mutations::GraphEditor;
    use semio_framework_artifact_flow_flow::WidgetDescriptor;
    let _serial = semio_s_artifact_procedural_3d_testkit::test_serial::lock();
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../🔣️value/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let base = empty_generation3d_snapshot();
    for row in fixture["descriptors"].as_array().unwrap() {
        let text = row["value"].to_string();
        let accepted = row["accepted"].as_bool().unwrap();
        assert_eq!(serde_json::from_str::<WidgetDescriptor>(&text).is_ok(), accepted, "independent descriptor witness");
        with_host(&base.host_snapshot, |host| {
            let mut editor = GraphEditor::new(host);
            assert_eq!(editor.add_widget_json(&text, 10.0, 20.0).is_ok(), accepted, "receiving IO admission");
            assert_eq!(editor.snapshot().widgets.len(), usize::from(accepted), "refused descriptors author no widget");
            let leaves = editor.finish();
            assert_eq!(leaves.len(), if accepted { 2 } else { 0 }, "only admitted descriptors author creation and placement");
            for leaf in leaves { leaf.retire_cold(); }
        });
    }
    base.retire_cold();
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
