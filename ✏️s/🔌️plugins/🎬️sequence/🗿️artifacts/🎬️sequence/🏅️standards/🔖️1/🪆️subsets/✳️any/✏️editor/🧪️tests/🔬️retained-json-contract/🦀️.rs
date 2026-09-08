
use super::*;

fn assert_measure<T: SequenceRetainedJson + dsl::ToValue>(value: &T) {
    let encoded = dsl::os_pack::to_json_string(value);
    let oracle: Value = serde_json::from_str(&encoded).expect("independent JSON parser");
    let expected = serde_json::to_vec(&oracle).expect("independent JSON writer").len();
    assert_eq!(sequence_bounded_serialized_bytes(value, expected).expect("exact admitted cap"), expected);
    assert!(sequence_bounded_serialized_bytes(value, expected - 1).is_err());
}

#[test]
fn sequence_retained_json_measure_matches_the_json_oracle() {
    let vectors: Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔁️retained-json.json")).expect("neutral retained vectors");
    for row in vectors["mutations"].as_array().expect("mutations") {
        let mutation: SequenceMutation = dsl::os_pack::from_json_str(&row.to_string()).expect("owned mutation decoder");
        assert_measure(&mutation);
        neural_engine::ColdRetire::retire_cold(mutation);
    }
    let config: Value = serde_json::from_str(include_str!("../../🎚️config/🧪️fixtures/🔁️mutation-contracts.json")).expect("neutral config vectors");
    for row in config["cases"].as_array().expect("config cases") {
        let mutation: SequenceConfigMutation = dsl::os_pack::from_json_str(&row["mutation"].to_string()).expect("owned config decoder");
        assert_measure(&mutation);
    }
    let carrier: Value = serde_json::from_str(include_str!("../../../🚪️io/🧪️fixtures/🔁️carrier-contracts.json")).expect("neutral carrier vectors");
    for row in carrier["cases"].as_array().expect("carrier cases") {
        let fixture: SequenceFixture = dsl::os_pack::from_json_str(&row["fixture"].to_string()).expect("owned fixture decoder");
        let scene = (fixture.steps, fixture.edges);
        assert_measure(&scene);
        neural_engine::ColdRetire::retire_cold(scene.0);
    }
    let escaped = String::from("\u{0000}\u{0008}\u{000c}\n\r\t\"\\Grüße");
    assert_measure(&escaped);
}
