//! 🛡️ The pptx diff text and binary codecs refuse a record whose value is not a typed diff.

use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::diff::*;
use protocol::{DiffBinary, DiffCodec, DiffText};

fn canonical_diff_fixture() -> serde_json::Value {
    serde_json::from_str(runtime_fixture_text("../../../🧪️tests/🧫️fixtures/🧬️canonical-diff/🔣️.json")).expect("language-neutral canonical field replacement laws")
}

#[test]
fn diff_codecs_refuse_untyped_xml_owners() {
    let fixture = canonical_diff_fixture();
    let value = semio_framework_value::DslValue::Object(vec![(fixture["malformedField"].as_str().unwrap().into(), semio_framework_value::DslValue::String(fixture["malformedValue"].as_str().unwrap().into()))]);
    let bytes = store::pack_rt::encode_wire_value(&value);
    assert!(PptxDiff::decode_diff(&bytes).is_err());
    let record = PptxDiffRecord { value };
    let text = semio_framework_dsl_record::print(&record.__dsl_to_record(), &PptxDiffRecord::__dsl_spec(), semio_framework_dsl_record::JoinMode::Inline);
    assert!(PptxDiff::parse_diff(&text).is_err());
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
