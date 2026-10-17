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
    value: Option<i64>,
    availability: String,
}

#[derive(value_derive::FromValue)]
struct Contract {
    vectors: Vec<Vector>,
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_unsigned(source: &str, infer_leaf: for<'a> fn(&GltfGeometryContext<'a>) -> GltfMeasure<u64>, unavailable_leaf: fn(&[String]) -> GltfMeasure<u64>) {
    let contract: Contract = semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for vector in contract.vectors {
        let result = if vector.context.valid {
            let policy = crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::policy();
            let context = GltfGeometryContext::new(&vector.context.points, &vector.context.triangles, &policy).unwrap();
            infer_leaf(&context)
        } else {
            unavailable_leaf(&["missing-position".into()])
        };
        assert_eq!(result.value.map(|value| value as i64), vector.value);
        assert_eq!(format!("{:?}", result.availability).to_ascii_lowercase(), vector.availability);
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_signed(source: &str, infer_leaf: for<'a> fn(&GltfGeometryContext<'a>) -> GltfMeasure<i64>, unavailable_leaf: fn(&[String]) -> GltfMeasure<i64>) {
    let contract: Contract = semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for vector in contract.vectors {
        let result = if vector.context.valid {
            let policy = crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::policy();
            let context = GltfGeometryContext::new(&vector.context.points, &vector.context.triangles, &policy).unwrap();
            infer_leaf(&context)
        } else {
            unavailable_leaf(&["missing-position".into()])
        };
        assert_eq!(result.value, vector.value);
        assert_eq!(format!("{:?}", result.availability).to_ascii_lowercase(), vector.availability);
    }
}

#[test]
fn every_topology_leaf_executes_its_shared_vectors() {
    assert_unsigned(runtime_fixture_text("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🕸️topology/➰️boundary-loops/🔣️.json"), boundary_loops::infer, boundary_loops::unavailable_measure);
    assert_signed(runtime_fixture_text("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🕸️topology/🧮️euler-characteristic/🔣️.json"), euler_characteristic::infer, euler_characteristic::unavailable_measure);
    assert_unsigned(runtime_fixture_text("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🕸️topology/🔢️genus/🔣️.json"), genus::infer, genus::unavailable_measure);
    assert_unsigned(runtime_fixture_text("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🕸️topology/🥯️handles/🔣️.json"), handles::infer, handles::unavailable_measure);
    assert_unsigned(runtime_fixture_text("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🕸️topology/🕳️holes/🔣️.json"), holes::infer, holes::unavailable_measure);
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
