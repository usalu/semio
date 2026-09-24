//! 📦️ Writer artifact — binary document surface + laws (constitutional: pack). Owns the
//! `store::ArtifactPack` impl for `WriterSnapshot`, encoded through its derived `dsl::DslRecord` spec.

use crate::WriterSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

//#region 🔖️ArtifactPack
/// 📦️ Derived-record pack: the `dsl::DslRecord` spec of `WriterSnapshot` (composed `document` child
/// included) is both the byte layout and the `record_spec()` pack-schema identity.
impl store::ArtifactPack for WriterSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        let mut snapshot = Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?;
        crate::attach_writer_document_text(&mut snapshot.document, &snapshot.text.clone());
        Ok(snapshot)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactPack

/// 📦️ Encodes a `WriterSnapshot` to its binary pack form.
pub fn encode(projection: &WriterSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(projection)
}

/// 📖️ Decodes a `WriterSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<WriterSnapshot, PackError> {
    <WriterSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-protocol-conformance/🦀️.rs"]
mod semio_protocol_conformance;
