//! 🔍 Verify the loaded PDF 1.4 subset fixture roster through the actual SDK acceptance reader.

use semio_plugin::EditorApp;
use semio_plugin_testing_acceptance::acceptance_cases;
use semio_s_artifact_stdio_pdf_schema_5::editor::{pdf14a::Pdf14AEditor, pdf14x::Pdf14XEditor};
use std::path::PathBuf;

#[test]
fn profile_editor_fixture_snapshots_and_operation_inputs_resolve() {
    let root = PathBuf::from(std::env::var_os("SEMIO_PDF14_HISTORY_PROBE").unwrap());
    assert!(acceptance_cases::<EditorApp<Pdf14AEditor>>(&root.join("missing")).is_empty());
    for name in ["a", "x"] {
        use semio_framework_value::FromValue;
        use semio_os_kernel::Mutation;
        use semio_s_artifact_stdio_pdf_schema_5::standards::v1_4::subsets::base::schema::{mutations::PdfMutation, snapshot::PdfSnapshot};
        let text = std::fs::read_to_string(root.join(name).join("case").join("🦠️mutation").join("🔣️.json")).unwrap();
        let raw = semio_framework_pack_json::parse(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let wire = semio_framework_pack_json::to_dsl_value(&raw);
        assert!(PdfSnapshot::from_value(wire.get("before").unwrap().clone()).is_ok(), "{name} snapshot: {:?}", PdfSnapshot::from_value(wire.get("before").unwrap().clone()).err());
        let op = PdfMutation::from_value(wire.get("mutation").unwrap().clone()).unwrap();
        assert!(op.input_schema().is_some(), "{name} has an authored input schema");
        assert!(!op.may_emit_foreign_steps());
        op.retire_cold();
    }
    let archive = acceptance_cases::<EditorApp<Pdf14AEditor>>(&root.join("a"));
    let press = acceptance_cases::<EditorApp<Pdf14XEditor>>(&root.join("x"));
    assert_eq!(archive.len(), 1);
    assert_eq!(press.len(), 1);
    assert!(archive[0].base.contains("Shelf inventory"));
    assert!(press[0].base.contains("Print proof"));
    for row in archive.into_iter().chain(press) {
        use semio_os_kernel::Mutation;
        row.op.retire_cold();
    }
}
