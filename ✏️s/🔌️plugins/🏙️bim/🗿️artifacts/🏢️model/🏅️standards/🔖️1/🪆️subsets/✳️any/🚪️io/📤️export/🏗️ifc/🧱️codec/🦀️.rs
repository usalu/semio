//! 🧱️ The one place that names the IFC 2x3 codec of the `s.stdio.ifc` artifact: Part-21 documents in and out of file bytes, validated against the IFC2X3 schema header.

use semio_s_artifact_stdio_ifc::part21::Part21Document;
use semio_s_artifact_stdio_ifc::standards::v2x3::engine::{decode_ifc2x3, encode_ifc2x3};
use semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::snapshot::{Ifc2x3Snapshot, STDIO_IFC2X3_DOCUMENT_SCHEMA};

/// 📤️ The file bytes of an IFC 2x3 document (CRLF layout, validated header and unique instance ids).
pub fn encode_document(document: Part21Document) -> Result<Vec<u8>, String> {
    encode_ifc2x3(&Ifc2x3Snapshot { schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document, edm_preamble: None })
}

/// 📥️ The Part-21 document of IFC 2x3 file bytes.
pub fn decode_document(bytes: &[u8]) -> Result<Part21Document, String> {
    decode_ifc2x3(bytes).map(|snapshot| snapshot.document)
}
