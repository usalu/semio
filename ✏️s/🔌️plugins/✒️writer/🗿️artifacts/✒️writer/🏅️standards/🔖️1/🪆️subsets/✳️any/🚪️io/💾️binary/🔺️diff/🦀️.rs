//! binary rep for stdio.json 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::WriterArtifact;
use crate::{document_child_handle_with_text, WriterSnapshot};
use protocol::MutationDiff;
use crate::schema::diff::*;

impl protocol::DiffBinary for WriterDiff {
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    Ok(self.print_diff().into_bytes())
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let line = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: error.to_string() })?;
    Self::parse_diff(line).map_err(|error| protocol::ProtocolError::Malformed { what: "diff json", offset: 0, detail: error.to_string() })
}
}
}
pub use diff_codec::*;
