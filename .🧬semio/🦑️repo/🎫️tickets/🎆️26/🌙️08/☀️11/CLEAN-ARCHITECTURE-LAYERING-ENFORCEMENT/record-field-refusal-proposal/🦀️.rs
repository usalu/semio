//! 🧪️ Typed field refusal identity through direct and boxed owned bindings.
use crate::{DslField, FieldValue, RecordValue, Shape};
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, ValueError, ValueRefusalKind};

#[derive(Debug)]
struct RefusingField(ValueError);

fn kind(text: &str) -> ValueRefusalKind {
    match text {
        "InvalidValue" => ValueRefusalKind::InvalidValue,
        "Canceled" => ValueRefusalKind::Canceled,
        "OwnershipLimit" => ValueRefusalKind::OwnershipLimit,
        "AllocationFailed" => ValueRefusalKind::AllocationFailed,
        "WorkLimit" => ValueRefusalKind::WorkLimit,
        "DepthLimit" => ValueRefusalKind::DepthLimit,
        "UnsupportedOwner" => ValueRefusalKind::UnsupportedOwner,
        "InvariantViolated" => ValueRefusalKind::InvariantViolated,
        _ => panic!("unknown authored refusal kind"),
    }
}

fn refusal(value: &FieldValue) -> ValueError {
    let FieldValue::Text(text) = value else { panic!("authored refusal carrier") };
    let data: serde_json::Value = serde_json::from_str(text).unwrap();
    ValueError::new(kind(data["kind"].as_str().unwrap()), data["message"].as_str().unwrap())
}

impl DslField for RefusingField {
    fn shape() -> Shape { Shape::Text }
    fn to_value(&self) -> FieldValue { panic!("unchecked projection is outside this law") }
    fn from_value(_: &FieldValue) -> Result<Self, String> { panic!("unchecked construction is outside this law") }
    fn to_value_controlled(&self, _: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> { Err(self.0.clone()) }
    fn to_record_controlled(&self, _: &mut NativeEncodeControl<'_>) -> Result<RecordValue, ValueError> { Err(self.0.clone()) }
    fn from_value_controlled(value: &FieldValue, _: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> { Err(refusal(value)) }
    fn from_record_controlled(record: &RecordValue, _: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> { Err(refusal(&record.fields[&1])) }
}

#[test]
fn controlled_field_projection_and_construction_retain_kind_through_boxed_paths() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪆️refusal/🔣️.json")).unwrap();
    for name in corpus["kinds"].as_array().unwrap() {
        for row in corpus["paths"].as_array().unwrap() {
            let expected_kind = kind(name.as_str().unwrap());
            let carrier = FieldValue::Text(serde_json::to_string(&serde_json::json!({"kind": name, "message": row["message"]})).unwrap());
            let mut record = RecordValue::default();
            record.fields.insert(1, carrier);
            let field = RefusingField(ValueError::new(expected_kind, row["message"].as_str().unwrap()));
            let mut decode_callback = |_| true;
            let mut encode_callback = |_| true;
            let mut decode = NativeDecodeControl::new(1024, &mut decode_callback);
            let mut encode = NativeEncodeControl::new(1024, &mut encode_callback);
            let error = match (row["method"].as_str().unwrap(), row["boxed"].as_bool().unwrap()) {
                ("projectValue", false) => field.to_value_controlled(&mut encode).unwrap_err(),
                ("projectRecord", false) => field.to_record_controlled(&mut encode).unwrap_err(),
                ("constructValue", false) => RefusingField::from_value_controlled(&record.fields[&1], &mut decode).unwrap_err(),
                ("constructRecord", false) => RefusingField::from_record_controlled(&record, &mut decode).unwrap_err(),
                ("constructValue", true) => Box::<RefusingField>::from_value_controlled(&record.fields[&1], &mut decode).unwrap_err(),
                ("constructRecord", true) => Box::<RefusingField>::from_record_controlled(&record, &mut decode).unwrap_err(),
                _ => panic!("unknown authored method and ownership path"),
            }.under(row["under"].as_str().unwrap());
            assert_eq!(error.kind, expected_kind);
            assert_eq!(error.message, row["expectedMessage"].as_str().unwrap());
        }
    }
}
