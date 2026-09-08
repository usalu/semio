
use super::*;

#[test]
fn bounded_chunk_values_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔣️chunks.json")).unwrap();
    for vector in vectors.as_array().unwrap() {
        let value = vector.get("value").cloned().unwrap_or_else(|| serde_json::Value::String(vector["text"].as_str().unwrap().repeat(vector["repeat"].as_u64().unwrap() as usize)));
        let encoded = value.to_string();
        let actual = dsl::os_pack::json::from_json_str::<ChunkAddressableJson>(&encoded);
        let oracle = serde_json::from_str::<ChunkAddressableJson>(&encoded);
        assert_eq!(actual.is_ok(), vector["accepted"].as_bool().unwrap());
        assert_eq!(actual.is_ok(), oracle.is_ok());
        if let (Ok(actual), Ok(oracle)) = (actual, oracle) {
            assert_eq!(actual, oracle);
            assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::os_pack::json::to_json_string(&actual)).unwrap(), value);
        }
    }
}
