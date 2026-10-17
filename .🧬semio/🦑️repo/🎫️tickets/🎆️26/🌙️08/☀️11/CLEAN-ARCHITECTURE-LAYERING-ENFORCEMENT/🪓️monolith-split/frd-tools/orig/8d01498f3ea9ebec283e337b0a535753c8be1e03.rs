#[derive(value_derive::FromValue)]
struct Vector {
    value: Option<f64>,
    availability: String,
}
#[derive(value_derive::FromValue)]
struct Contract {
    vectors: Vec<Vector>,
}
#[semio_framework_async_macros::async_test]
async fn shared_analytic_unavailable_and_deterministic_vectors_are_typed() {
    let contract: Contract = semio_framework_pack_json::from_json_str(runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/💡️inferences/📦️size/📏️overall-size/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(contract.vectors[0].value, Some(5.0));
    assert_eq!(contract.vectors[0].availability, "available");
    assert_eq!(contract.vectors[1].value, None);
    assert_eq!(contract.vectors[1].availability, "unavailable");
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
