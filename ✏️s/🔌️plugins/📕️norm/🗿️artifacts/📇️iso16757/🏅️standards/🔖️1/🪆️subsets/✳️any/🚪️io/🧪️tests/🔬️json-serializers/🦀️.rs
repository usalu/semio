use super::*;
use crate::Iso16757Snapshot;

#[semio_framework_async_macros::async_test]
async fn catalogue_json_round_trip() {
    let doc = Iso16757Snapshot::default();
    let json = io::catalogue_to_json(&doc.catalogue).expect("json");
    let restored = io::catalogue_from_json(&json).expect("restore");
    assert_eq!(restored.id, doc.catalogue.id);
}

#[semio_framework_async_macros::async_test]
async fn dictionary_json_round_trip() {
    let doc = Iso16757Snapshot::default();
    let json = io::dictionary_to_json(&doc.dictionary).expect("json");
    assert!(json.contains("hvac-dict"));
}
