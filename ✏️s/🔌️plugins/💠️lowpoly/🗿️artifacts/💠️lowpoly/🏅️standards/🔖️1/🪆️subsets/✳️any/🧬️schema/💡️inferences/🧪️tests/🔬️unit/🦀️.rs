use super::*;
use protocol::Inference;

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::snapshot_from_mesh_json("{}", "o1", "Object 1");
    assert_eq!(LowpolyInference::infer(&snapshot).expect("valid materialized inference fixture"), LowpolyInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(LowpolyInference::infer(&LowpolySnapshot::default()).expect("valid materialized inference fixture"), LowpolyInference::default());
}
//#endregion 🧪️InferenceLaws

/// 🔮️ Neutral vectors independently checked with Three XYZ Euler/quaternion mathematics.
#[semio_framework_async_macros::async_test]
async fn semantic_transform_neutral_three_oracle_law() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔄️transform.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let triple = |value: &serde_json::Value| -> [f32; 3] { std::array::from_fn(|i| value[i].as_f64().unwrap() as f32) };
        let transform = LowpolyTransform { position: triple(&case["transform"]["position"]), rotation: triple(&case["transform"]["rotation"]), scale: triple(&case["transform"]["scale"]) };
        let quaternion = euler_degrees_to_quaternion(transform.rotation);
        let world = apply_transform(&transform, triple(&case["local"]));
        for (index, value) in quaternion.iter().enumerate() {
            assert!((value - case["quaternion"][index].as_f64().unwrap()).abs() < 1e-12, "{} quaternion {index}", case["id"]);
        }
        for (index, value) in world.iter().enumerate() {
            assert!((value - case["world"][index].as_f64().unwrap()).abs() < 1e-10, "{} world {index}", case["id"]);
        }
        eprintln!("[DEBUG] Lowpoly semantic transform Three oracle {} quaternion={quaternion:?} world={world:?}", case["id"]);
    }
    assert_eq!(triangle_normal([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]), [0.0, 0.0, 1.0]);
    assert_eq!(triangle_normal([0.0; 3], [0.0; 3], [0.0; 3]), [0.0; 3]);
}
