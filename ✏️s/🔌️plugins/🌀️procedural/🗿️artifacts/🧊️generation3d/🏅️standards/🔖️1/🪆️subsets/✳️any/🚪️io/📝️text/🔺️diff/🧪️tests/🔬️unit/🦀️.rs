use crate::standards::v1::subsets::any::schema::diff::{Generation3dDiff, Generation3dDiffRead};
use semio_framework_value::{FromValue, ToValue};

/// 🧫️ Original sparse diff text/binary records preserve complete field values against serde_json.
#[test]
fn generation3d_original_diff_record_preserves_sparse_domain_fields_and_serde_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["contract"]["fieldIds"], serde_json::json!({"schema":0,"camera":1,"widgets":2,"synapses":3,"layout":4,"generations":5,"selectedGeneration":6,"previewText":7}));
    for case in fixture["cases"].as_array().unwrap() {
        let source = Generation3dDiffRead::new(semio_framework_pack_json::from_json_str(&case["value"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
        let source_json = semio_framework_pack_json::to_json_string(&*source);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&source_json).unwrap(), case["value"]);
        let text = protocol::DiffText::print_diff(&*source);
        let parsed = Generation3dDiffRead::new(<Generation3dDiff as protocol::DiffText>::parse_diff(&text).unwrap());
        assert_eq!(parsed.to_value(), source.to_value(), "{} text preserves complete sparse field values", case["id"]);
        let bytes = protocol::DiffBinary::encode_diff(&*source).unwrap();
        let decoded = Generation3dDiffRead::new(<Generation3dDiff as protocol::DiffBinary>::decode_diff(&bytes).unwrap());
        let actual = semio_framework_pack_json::to_json_string(&*decoded);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&actual).unwrap(), case["value"], "{} binary agrees with serde_json", case["id"]);
        let record = source.__dsl_to_record();
        for (id, key) in [(0, "schema"), (1, "camera"), (2, "widgets"), (3, "synapses"), (4, "layout"), (5, "generations"), (6, "selectedGeneration"), (7, "previewText")] {
            assert_eq!(record.fields.contains_key(&id), !case["value"][key].is_null());
        }
        semio_framework_dsl_record::native_encoding::retire_field(semio_framework_dsl_record::FieldValue::Record(record));
        eprintln!("[DEBUG] original generation3d diff record case={} exactSparseFields=true textBinary=true independentSerdeJson=true", case["id"]);
    }
    assert!(Generation3dDiff::from_value(semio_framework_value::DslValue::Object(vec![("widgets".into(), semio_framework_value::DslValue::String("wrong field shape".into()))])).is_err());
}
