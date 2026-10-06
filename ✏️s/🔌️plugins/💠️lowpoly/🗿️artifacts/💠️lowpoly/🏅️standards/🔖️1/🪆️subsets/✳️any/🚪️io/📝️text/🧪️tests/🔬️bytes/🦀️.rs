use super::*;
use semio_framework_value::{FromValue, ToValue};

/// 🧫️ Intrinsic octets and physical base64 agree with the independent JSON/base64 oracle.
#[test]
fn lowpoly_intrinsic_bytes_have_one_semantic_owner_and_physical_json_codec() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️bytes/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let run = crate::schema::PixelRun { offset: case["offset"].as_u64().unwrap() as u32, bytes: case["octets"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect() };
        let value = run.to_value();
        assert!(matches!(semio_framework_value::DslValue::into_object(value.clone()).unwrap().into_iter().find(|(name, _)| name == "bytes").unwrap().1, semio_framework_value::DslValue::Bytes(_)));
        assert_eq!(crate::schema::PixelRun::from_value(value).unwrap(), run);
        let text = lowpoly_json_encode(&run);
        let oracle: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(oracle, serde_json::from_str::<serde_json::Value>(case["text"].as_str().unwrap()).unwrap());
        assert_eq!(oracle["bytes"].as_str().unwrap(), base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &run.bytes));
        assert_eq!(lowpoly_json_decode::<crate::schema::PixelRun>(&text).unwrap(), run);
    }
    assert!(lowpoly_json_decode::<crate::schema::PixelRun>("{\"offset\":0,\"bytes\":\"?\"}").is_err());
    eprintln!("[DEBUG] lowpoly-intrinsic-bytes semantic=octets physical=base64 oracle=serde_json+base64");
}
