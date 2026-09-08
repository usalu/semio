
use super::*;

const VALID: &str = include_str!("../../🧪️fixtures/✅️valid/🔣️.json");
const INVALID: &str = include_str!("../../🧪️fixtures/❌️invalid/🔣️.json");

#[test]
fn package_schema_matches_third_party_json_oracle() {
    let ours = package_from_schema(VALID).expect("first-party parser");
    let oracle: serde_json::Value = serde_json::from_str(VALID).expect("third-party parser");
    assert_eq!(ours.id, oracle["id"].as_str().expect("oracle id"));
    assert_eq!(ours.rust_package, oracle["rust_package"].as_str().expect("oracle Rust package"));
    assert!(package_from_schema(INVALID).is_err());
    assert!(serde_json::from_str::<serde_json::Value>(INVALID).is_ok());
}
