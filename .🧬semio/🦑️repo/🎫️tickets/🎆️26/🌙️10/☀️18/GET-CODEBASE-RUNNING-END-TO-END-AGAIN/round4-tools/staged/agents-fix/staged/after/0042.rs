//! 🪪️ Law requests retain exact snapshot ownership and agree with an independent JSON reference.
use super::*;

#[test]
fn request_builder_borrows_the_original_snapshot_and_returns_its_exact_registry_lease() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("./🧫️fixtures/🧾️request.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let (root, mut request) = one_item_preparation_law_request(row["snapshot"].as_bool().unwrap(), row["mutation"].as_bool().unwrap());
        assert!(std::ptr::eq(root.as_ref(), request.base.get()));
        let actual = serde_json::json!({"snapshot": request.base.get(), "mutation": request.mutation, "operation": request.operation.0, "generation": request.generation.0});
        assert_eq!(actual, row["expected"]);
        assert_eq!(request.base_revision, [0; 32]);
        assert_eq!(request.authority.operation(), request.operation);
        assert_eq!(request.authority.generation(), request.generation);
        assert_eq!(request.authority.base_revision(), request.base_revision);
        assert!(request.base.return_to_registry());
    }
}
