//! 🧪️ Canonical Generic collection specimen preserves the typed fixture fixpoint law.
use crate::*;
use store::ArtifactDsl as _;
#[test]
fn canonical_owned_fixture_text_laws() {
    let text = include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🧹️conformance/🗣️.dsl.semio");
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🧹️conformance/🔣️.json")).expect("independent Generic fixture oracle");
    let (envelope, _) = store::os_store::semio_format::split_text_preamble(text).expect("canonical envelope");
    assert_eq!(envelope.envelope_id(), expected["envelope"].as_str().unwrap());
    assert_eq!(CollectionSnapshot::envelope_id(), expected["envelope"].as_str().unwrap());
    let snapshot = CollectionSnapshot::parse_dsl(text).expect("actual Generic snapshot parsing");
    assert_eq!(serde_json::to_value(&snapshot).unwrap(), expected["snapshot"]);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&snapshot)).unwrap(), expected["snapshot"]);
    let printed = snapshot.print_dsl();
    let reparsed = CollectionSnapshot::parse_dsl(&printed).expect("actual Generic snapshot reparsing");
    assert_eq!(snapshot, reparsed);
    assert_eq!(printed, reparsed.print_dsl());
    store::os_store::test_support::check_dsl_fixture_text_laws::<CollectionSnapshot>(text).expect("original typed fixture law");
}
