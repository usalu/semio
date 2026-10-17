#[derive(value_derive::FromValue)]
struct Value {
    min: [f64; 3],
    max: [f64; 3],
    dimensions: [f64; 3],
}
#[derive(value_derive::FromValue)]
struct Vector {
    value: Option<Value>,
}
#[derive(value_derive::FromValue)]
struct Contract {
    vectors: Vec<Vector>,
}
#[semio_framework_async_macros::async_test]
async fn shared_analytic_unavailable_and_deterministic_bounds_vectors_are_typed() {
    let contract: Contract = semio_framework_pack_json::from_json_str(runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/💡️inferences/📦️size/↔️axis-aligned-bounds/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(contract.vectors[0].value.as_ref().unwrap().dimensions, [3.0, 4.0, 5.0]);
    assert_eq!(contract.vectors[0].value.as_ref().unwrap().min, [1.0, 2.0, 3.0]);
    assert_eq!(contract.vectors[0].value.as_ref().unwrap().max, [4.0, 6.0, 8.0]);
    assert!(contract.vectors[1].value.is_none());
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
