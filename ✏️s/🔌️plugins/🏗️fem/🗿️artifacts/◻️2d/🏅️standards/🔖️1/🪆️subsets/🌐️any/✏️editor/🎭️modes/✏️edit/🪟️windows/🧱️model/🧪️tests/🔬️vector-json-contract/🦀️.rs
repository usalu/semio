
use super::*;
#[test]
fn vector_layer_vectors_match_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️vectors.json")).expect("neutral vector layers");
    for row in fixture["cases"].as_array().expect("vector cases") {
        let origin = (row["origin"][0].as_f64().unwrap(), row["origin"][1].as_f64().unwrap());
        let vector = [row["vector"][0].as_f64().unwrap(), row["vector"][1].as_f64().unwrap()];
        let layer = vector_layer(row["id"].as_str().unwrap().into(), origin, vector, row["color"].as_str().unwrap());
        let actual: serde_json::Value = serde_json::from_str(&dsl::json::to_string(&layer)).expect("independent layer decoder");
        assert_eq!(actual, row["expected"]);
    }
}
