/// 📤️📥️ LAW: exporting a document and importing those exact bytes back yields a BYTE-IDENTICAL
/// document, for every shipped example — including the one above the transport budget, because the
/// projection and the import decode are the same law the transport only carries. The decode reads the
/// whole file the framework hands the import (`puzzle5d_decode_document`).
#[test]
fn export_import_round_trips_every_shipped_example_byte_for_byte() {
    for document in [concrete_forest_example_document(), nakagin_example_document(), capsule_dream_example_document()] {
        // 🚦️ Non-vacuity: an EMPTY document round-trips trivially, so this law would pass while proving
        // nothing the moment the shipped examples regress (see the diagnosis law above).
        assert!(!document.parts.is_empty(), "a round trip over an empty document proves nothing");
        let exported = export_fixture::puzzle5d_export_json(&document);
        let root = import_fixture::puzzle5d_decode_document(&exported).expect("the whole export decodes as a puzzle 5d document");
        let reimported: Puzzle5dDocument = serde_json::from_value(root).expect("the decoded root is a puzzle 5d document");
        assert_eq!(export_fixture::puzzle5d_export_json(&reimported), exported, "{} did not round-trip byte-for-byte", document.label.clone().unwrap_or_default());
    }
}
