//! fem3d -> csv. `stdio.csv`'s real `CsvSnapshot` shape (`has_header` + `records: Vec<CsvRecord>`
//! of `CsvField{value, quoted}`) landed after this leaf was first written — lagging call site
//! fixed to match (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-
//! RETIREMENT W5a), same single-blob-payload shape as before, just through the current fields.
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};

pub async fn register() {}

pub async fn serialize(snapshot: &Fem3dSnapshot) -> Result<CsvSnapshot, store::TextError> {
    Ok(CsvSnapshot {
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        has_header: true,
        records: vec![
            CsvRecord { fields: vec![CsvField { value: "payload".into(), quoted: false }] },
            CsvRecord { fields: vec![CsvField { value: <Fem3dSnapshot as store::ArtifactDsl>::print_dsl(snapshot), quoted: true }] },
        ],
    })
}

pub async fn serialize_bytes(snapshot: &Fem3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
