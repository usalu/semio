use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::transform::from_value;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::transform::to_value;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::transform::DslValue;
use serde_json::Value as Json;

#[test]
fn pptx_transform_wire_vectors_match_independent_serde_signed64() {
    let corpus: Json = serde_json::from_str(&crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/📸️snapshot/🧭️transform/🔣️.json")).unwrap();
    let rows = corpus["vectors"].as_array().unwrap();
    assert_eq!(rows.len(), 16);
    for row in rows {
        let wire = &row["wire"];
        let value = match wire {
            Json::String(text) => DslValue::String(text.clone()),
            Json::Number(number) => DslValue::int(number.as_i64().unwrap()),
            Json::Null => DslValue::Null,
            _ => panic!("closed scalar corpus"),
        };
        let reference = wire.as_str().and_then(|text| serde_json::from_str::<i64>(text).ok().filter(|value| serde_json::to_string(value).unwrap() == text));
        let decoded = from_value(value);
        let expected = row["codecAccepted"].as_bool().unwrap();
        assert_eq!(reference.is_some(), expected, "independent {}", row["id"]);
        assert_eq!(decoded.is_ok(), expected, "owned {}", row["id"]);
        if let Some(reference) = reference {
            let decoded = decoded.unwrap();
            assert_eq!(decoded, reference);
            assert_eq!(to_value(&decoded), DslValue::String(serde_json::to_string(&reference).unwrap()));
        }
    }
    println!("pptx-transform-wire: all16 actual owned scalar codecs match independent Serde signed64");
}
