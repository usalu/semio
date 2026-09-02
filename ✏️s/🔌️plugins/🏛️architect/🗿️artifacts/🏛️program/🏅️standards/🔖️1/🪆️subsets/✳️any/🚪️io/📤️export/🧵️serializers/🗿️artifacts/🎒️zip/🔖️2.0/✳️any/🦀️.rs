//! program -> zip
use crate::artifacts::program::ProgramSnapshot;
use semio_s_plugin_stdio::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA;
pub use semio_s_plugin_stdio::artifacts::zip::{ZipEntry, ZipSnapshot};

pub async fn register() {}

pub async fn serialize(snapshot: &ProgramSnapshot) -> Result<ZipSnapshot, store::TextError> {
    let tables = crate::artifacts::program::io::program_export_tables(snapshot).await.map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))?;
    let entries = tables
        .into_iter()
        .map(|table| {
            let rows = dsl::DslValue::Array(table.rows.into_iter().map(dsl::DslValue::Object).collect());
            ZipEntry { name: format!("{}.json", table.name), data: dsl::json::to_json_string(&rows).into_bytes() }
        })
        .collect::<Vec<_>>();
    Ok(ZipSnapshot { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries, comment: "s.architect.program@1/*".into() })
}

pub async fn serialize_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<ZipSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot).await?))
}

pub async fn serialize_raw_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, store::TextError> {
    let archive = serialize(snapshot).await?;
    semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::any::io::encode_zip(&archive).map_err(|error| store::TextError::new(format!("program->zip: {error}"), dsl::TextSpan::at(1, 1)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::any::io::encode_zip;
    use std::io::Read;

    #[semio_framework_async_macros::async_test]
    async fn exports_every_program_table_to_a_real_archive() {
        let program = crate::artifacts::program::sample_plugin().await;
        let archive = serialize(&program).await.expect("serialize program archive");
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
}
