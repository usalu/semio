//! 🚚️ Layer drop fixtures preserve identifiers and reject invalid hierarchy edits.
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
#[allow(unused_imports)] use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_editor_commands::move_layer::*;
#[test]
fn layer_drop_fixtures() {
    let mut layers = vec![semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("A")).to_string().into()).expect("nonempty authored identity"), "A"), semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("B")).to_string().into()).expect("nonempty authored identity"), "B"), semio_s_artifact_draw_drawing_model::create_drawing_group_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("G")).to_string().into()).expect("nonempty authored identity"), "G")];
    for (layer, id) in layers.iter_mut().zip(["a.a", "b.b", "g.g"]) { semio_s_artifact_draw_drawing_model::layer_base_mut(layer).id = id.into(); }
    let document = DrawingSnapshot { layers:layers.into(), ..Default::default() };
    let cases: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../🚚️move-layer/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let result = plan(&document, &MoveLayer { layer_id: case["source"].as_str().unwrap().into(), target_row_id: case["target"].as_str().unwrap().into(), drop_position: case["position"].as_str().unwrap().into() });
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{case}");
        if let Some(index) = case["index"].as_u64() { let DrawingMutation::ReorderLayer(payload) = result.unwrap() else { panic!("Expected a reorder") }; assert_eq!(payload.index, index as usize); }
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
