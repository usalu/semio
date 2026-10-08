use crate::standards::v1::subsets::any::io::export::serializers::artifacts::csv::v_rfc4180::any::export_relationships_csv;
use super::*;
use crate::standards::v1::subsets::any::io::text::inferences::{export_json,import_json};
use crate::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn json_round_trip() {
    let program = sample_plugin();
    let json = export_json(&program).expect("export");
    let imported = import_json(&json).expect("import");
    assert_eq!(imported.elements.len(), program.elements.len());
    assert_eq!(imported.adjacencies.len(), program.adjacencies.len());
}

#[semio_framework_async_macros::async_test]
async fn relationships_csv_round_trips_via_stdio_codec() {
    let program = sample_plugin();
    let csv = export_relationships_csv(&program).expect("relationships csv export");
    let snapshot = semio_s_artifact_stdio_csv::standards::v_rfc4180::subsets::any::io::text::snapshot::decode_csv_with(&csv, true);
    assert_eq!(snapshot.records.len(), program.relationships.len() + 1, "header + one row per relationship");
}
