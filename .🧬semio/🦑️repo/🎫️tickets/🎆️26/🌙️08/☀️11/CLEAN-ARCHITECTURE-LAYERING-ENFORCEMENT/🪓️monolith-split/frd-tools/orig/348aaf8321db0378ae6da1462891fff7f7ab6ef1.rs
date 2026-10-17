use super::*;

#[derive(value_derive::FromValue)]
#[value(rename_all = "camelCase")]
struct Context {
    points: Vec<[f64; 3]>,
    triangles: Vec<[usize; 3]>,
    valid: bool,
}

#[derive(value_derive::FromValue)]
struct Vector {
    context: Context,
    value: Option<f64>,
    availability: String,
}

#[derive(value_derive::FromValue)]
struct Contract {
    vectors: Vec<Vector>,
}

#[semio_framework_async_macros::async_test]
async fn descriptor_is_versioned_and_cacheable() {
    assert_eq!(descriptor().id, "s.stdio.gltf.inference.elongation.v1");
    assert_eq!(descriptor().algorithm_version, 1);
}

#[semio_framework_async_macros::async_test]
async fn shared_vectors_execute_the_rust_leaf() {
    let contract: Contract = semio_framework_pack_json::from_json_str(runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/💡️inferences/📏️proportion/↔️elongation/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for vector in contract.vectors {
        let result = if vector.context.valid {
            let policy = crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::policy();
            let context = GltfGeometryContext::new(&vector.context.points, &vector.context.triangles, &policy).unwrap();
            infer(&context)
        } else {
            unavailable_measure(&["missing-position".into()])
        };
        assert_eq!(result.value, vector.value);
        assert_eq!(format!("{:?}", result.availability).to_ascii_lowercase(), vector.availability);
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
