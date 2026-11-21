use super::*;
#[test]
fn vector_layer_vectors_match_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️vectors.json")).expect("neutral vector layers");
    for row in fixture["cases"].as_array().expect("vector cases") {
        let origin = (row["origin"][0].as_f64().unwrap(), row["origin"][1].as_f64().unwrap());
        let vector = [row["vector"][0].as_f64().unwrap(), row["vector"][1].as_f64().unwrap()];
        let layer = vector_layer(row["id"].as_str().unwrap(), origin, vector, row["color"].as_str().unwrap());
        let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&layer)).expect("independent layer decoder");
        assert_eq!(actual, row["expected"]);
    }
}

#[test]
fn analysis_caption_has_the_catalog_text_paint() {
    let layer = fem2d_canvas_text_layer("analysis-progress", 10.0, 20.0, "Converged", 11.0);
    let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&layer)).expect("independent layer decoder");
    assert_eq!(actual, serde_json::json!({
        "id": "analysis-progress",
        "transform": [1.0, 0.0, 0.0, 1.0, 10.0, 20.0],
        "fill": { "color": [0.43, 0.48, 0.55, 1.0] },
        "text": { "content": "Converged", "size": 11.0, "fontFamily": "Share Tech Mono" },
    }));
}
