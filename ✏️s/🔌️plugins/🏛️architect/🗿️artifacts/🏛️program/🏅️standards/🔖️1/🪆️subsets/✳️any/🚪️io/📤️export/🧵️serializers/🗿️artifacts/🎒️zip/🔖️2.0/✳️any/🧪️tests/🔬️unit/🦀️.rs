use super::*;
use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::encode_zip;
use std::io::Read;

#[semio_framework_async_macros::async_test]
async fn exports_every_program_table_to_a_real_archive() {
    let program = crate::sample_plugin();
    let archive = serialize(&program).expect("serialize program archive");
    assert_eq!(archive.entries.len(), 70);
    let elements = archive.entries.iter().find(|entry| entry.name == "elements.json").expect("elements entry");
    let element_rows: serde_json::Value = serde_json::from_slice(&elements.data).expect("elements JSON");
    assert_eq!(element_rows.as_array().expect("elements rows").len(), 2);
    let risks = archive.entries.iter().find(|entry| entry.name == "risks.json").expect("risks entry");
    assert_eq!(risks.data, b"[]".to_vec());

    let raw = encode_zip(&archive).expect("encode real ZIP");
    let mut observed = zip::ZipArchive::new(std::io::Cursor::new(raw)).expect("zip crate reads ZIP");
    assert_eq!(observed.len(), 70);
    let mut element_json = String::new();
    observed.by_name("elements.json").expect("zip crate observed elements entry").read_to_string(&mut element_json).expect("zip crate reads elements entry");
    let observed_elements: serde_json::Value = serde_json::from_str(&element_json).expect("independently read elements JSON");
    assert_eq!(observed_elements.as_array().expect("observed element rows").len(), 2);
}
