//! 🧪️ Neutral Equation field edits checked by the independent serde JSON implementation.
use crate::diff::{EquationPointEdit,EquationPointsDelta};
use crate::{EquationDiff,EquationPoint};
use protocol::{DiffText,DiffBinary};
#[test]
fn owned_geometry_diff_matches_neutral_text_and_native_roundtrip() {
    let expected:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../../../✳️any/🚪️io/📝️text/🔺️diff/🧪️tests/🧫️fixtures/📍️geometry/🔣.json")).unwrap();
    let diff=EquationDiff{points:Some(EquationPointsDelta{edits:vec![EquationPointEdit::Set{at:0,point:EquationPoint{x:2.0,y:3.0}}]}),..Default::default()};
    let text=diff.print_diff();
    let independent:serde_json::Value=serde_json::from_str(&text).unwrap();
    assert_eq!(independent,expected);
    assert_eq!(EquationDiff::parse_diff(&text).unwrap(),diff);
    assert_eq!(EquationDiff::decode_diff(&diff.encode_diff().unwrap()).unwrap(),diff);
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
