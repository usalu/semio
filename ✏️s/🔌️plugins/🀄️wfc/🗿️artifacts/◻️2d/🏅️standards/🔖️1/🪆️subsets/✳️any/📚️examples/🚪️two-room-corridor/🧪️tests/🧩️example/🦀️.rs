//! 🧪️ Example round trip — the PRINTED document parses back to the Rust authority it came from.

#[test]
fn printed_text_parses_back_to_the_authored_document() {
    let document = crate::examples::two_room_corridor::document();
    let text = store::ArtifactDsl::print_dsl(&document);
    let parsed: crate::Wfc2dSnapshot = <crate::Wfc2dSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("the printed example parses");
    assert_eq!(parsed, document, "the printed text is not the document it was printed from");
}

#[test]
fn the_example_source_carries_the_printed_document() {
    let source = crate::examples::two_room_corridor::source();
    assert_eq!(source.id(), crate::examples::two_room_corridor::ID);
    assert!(source.document_json().len() > 8, "the example payload must not be empty");
}
