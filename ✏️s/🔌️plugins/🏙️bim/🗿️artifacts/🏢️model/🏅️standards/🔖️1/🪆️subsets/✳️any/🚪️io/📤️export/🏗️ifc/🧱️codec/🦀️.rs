//! 🧱️ The one place that names the IFC codecs of the `s.stdio.ifc` artifact: Part-21 documents in and out of file bytes, validated against the schema header (IFC2X3 through the stdio 2x3 codec, IFC4 through the generic Part-21 reader and writer).

use semio_s_artifact_stdio_ifc::part21::{parse_part21, write_part21, Part21Document};
use semio_s_artifact_stdio_ifc::standards::v2x3::engine::{decode_ifc2x3, encode_ifc2x3};
use semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::snapshot::{Ifc2x3Snapshot, STDIO_IFC2X3_DOCUMENT_SCHEMA};
use std::collections::BTreeSet;

/// 📤️ The file bytes of an IFC 2x3 document (CRLF layout, validated header and unique instance ids).
pub fn encode_document(document: Part21Document) -> Result<Vec<u8>, String> {
    encode_ifc2x3(&Ifc2x3Snapshot { schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document, edm_preamble: None })
}

/// 📥️ The Part-21 document of IFC 2x3 file bytes.
pub fn decode_document(bytes: &[u8]) -> Result<Part21Document, String> {
    decode_ifc2x3(bytes).map(|snapshot| snapshot.document)
}

fn declares_ifc4(document: &Part21Document) -> bool {
    document.header.file_schema.iter().any(|value| value.as_list().is_some_and(|items| items.iter().any(|item| item.as_str() == Some("IFC4"))))
}

/// 📤️ The file bytes of an IFC4 document: the header must declare `IFC4` and the instance ids must be unique.
pub fn encode_ifc4(document: &Part21Document) -> Result<Vec<u8>, String> {
    if !declares_ifc4(document) {
        return Err("ifc4: FILE_SCHEMA does not declare IFC4".to_string());
    }
    let mut ids = BTreeSet::new();
    if let Some(instance) = document.instances.iter().find(|instance| !ids.insert(instance.id)) {
        return Err(format!("ifc4: the instance #{} is written twice", instance.id));
    }
    Ok(write_part21(document).into_bytes())
}

/// 📥️ The Part-21 document of IFC4 file bytes; a file that does not declare `IFC4` is refused.
pub fn decode_ifc4(bytes: &[u8]) -> Result<Part21Document, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| format!("ifc4: not valid utf-8: {error}"))?;
    let document = parse_part21(text).map_err(|error| format!("ifc4 parse: {error}"))?;
    if !declares_ifc4(&document) {
        return Err("ifc4: FILE_SCHEMA does not declare IFC4".to_string());
    }
    Ok(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
