//#region 🔖Pack
/// @emoji 📦 Binary counterpart of `🔖Text` above — see the wave-1 design at
/// `.repo/🎫/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/` for the full container-format
/// contract. `pack`'s own `EncodeOptions`/`DecodeOptions`/`VerificationLevel` are re-exported under
/// a `Pack`-prefixed name (not a plain re-export — `dsl_derive`'s emitted `DocumentPack` impl and
/// every downstream caller spell them `store::PackEncodeOptions`/`store::PackDecodeOptions`/
/// `store::PackVerificationLevel`, so there is exactly one spelling repo-wide).
pub use pack::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions, PackError, VerificationLevel as PackVerificationLevel};

/// @emoji 🧵 Thin runtime bridge to `pack::{encode_document, decode_document}`, resolved as
/// `::store::pack_rt::...` by `dsl_derive`'s generated `DocumentPack` impl (app crates depend on
/// `vcs`, never on `pack` directly — same seam `::dsl::RecordSpec`/`RecordValue` already use). Also
/// hosts the schema-less JSON bridge behind `impl DocumentPack for serde_json::Value` below.
pub mod pack_rt {
    use super::{PackDecodeOptions, PackEncodeOptions, PackError};
    use dsl::{DslValue, FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordValue, Shape};
    use std::collections::HashMap;

    /// @emoji 🚪 Forwards to `pack::encode_document`.
    pub fn encode_document(spec: &RecordSpec, record: &RecordValue, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        pack::encode_document(spec, record, options)
    }

    /// @emoji 🚪 Forwards to `pack::decode_document`.
    pub fn decode_document(bytes: &[u8], spec: &RecordSpec, options: &PackDecodeOptions) -> Result<(RecordValue, pack::DecodeReport), PackError> {
        pack::decode_document(bytes, spec, options)
    }

    /// @emoji 🌱 Field id the JSON bridge's synthetic single-field record wraps a whole
    /// `serde_json::Value` payload in — mirrors `dsl::DslField for serde_json::Value`'s
    /// `Shape::Value` escape hatch (`dsl/rs/lib.rs`), lifted one level from "one field" to "one
    /// whole document" so schema-less apps (puzzle plugins, compose kit) get a pack encoding too.
    const JSON_BRIDGE_FIELD_ID: u16 = 1;

    fn json_bridge_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(JSON_BRIDGE_FIELD_ID, "value", Shape::Value)])
    }

    /// @emoji 🌱 Encodes an arbitrary `serde_json::Value` as a complete pack file. Infallible for any
    /// well-formed JSON value — mirrors `DocumentDsl::print_dsl`'s infallible signature; a
    /// `LimitExceeded` on a pathologically huge value is the one way this can panic, same ceiling
    /// `pack_value`'s own encoder enforces.
    pub fn encode_json_value(value: &serde_json::Value) -> Vec<u8> {
        let mut fields = HashMap::new();
        fields.insert(JSON_BRIDGE_FIELD_ID, FieldValue::Value(DslValue::from(value.clone())));
        let record = RecordValue { fields };
        encode_document(&json_bridge_spec(), &record, &PackEncodeOptions::default())
            .expect("json bridge encode is infallible for a well-formed DslValue")
    }

    /// @emoji 🌱 Inverse of `encode_json_value`.
    pub fn decode_json_value(bytes: &[u8]) -> Result<serde_json::Value, PackError> {
        let (record, _report) = decode_document(bytes, &json_bridge_spec(), &PackDecodeOptions::default())?;
        match record.get(JSON_BRIDGE_FIELD_ID) {
            Some(FieldValue::Value(dsl_value)) => Ok(serde_json::Value::from(dsl_value.clone())),
            _ => Ok(serde_json::Value::Null),
        }
    }
}

/// @emoji 📦 Binary counterpart to `DocumentDsl` — same shape, opposite face. LAW: `P::decode_pack(
/// &p.encode_pack())` recovers an equal `p`, AND (structurally, not just by test) `decode_pack(
/// encode_pack(p)) == parse_dsl(print_dsl(p))` — dsl and pack are two encodings of the identical
/// `(RecordSpec, RecordValue)` pair keyed by the same stable `u16` field ids `dsl_derive` assigns,
/// never two independent sources of truth. The `_with` methods are required (the seam
/// `dsl_derive`'s generated impl calls through `::store::pack_rt`); the plain names are provided
/// defaults over `Pack{Encode,Decode}Options::default()`.
pub trait DocumentPack: Sized {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError>;
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError>;

    /// @emoji 📦 `encode_pack_with` at default options — infallible in practice (mirrors
    /// `DocumentDsl::print_dsl`'s infallible signature); panics only on a `PackLimits` overflow.
    fn encode_pack(&self) -> Vec<u8> {
        self.encode_pack_with(&PackEncodeOptions::default()).expect("default-options pack encode is infallible")
    }

    /// @emoji 📦 `decode_pack_with` at default (Standard) verification.
    fn decode_pack(bytes: &[u8]) -> Result<Self, PackError> {
        Self::decode_pack_with(bytes, &PackDecodeOptions::default())
    }
}

/// @emoji 📦 Binary counterpart to `DocumentTextFiles`: `pack` is the encoded initial projection
/// (whole `.spk` container bytes), `ops` stays the op-log TEXT — the op grammar is format-invariant,
/// only the initial-projection encoding differs between the text and pack file pairs.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocumentPackFiles {
    pub pack: Vec<u8>,
    pub ops: String,
}

/// @emoji 🌱 Pack counterpart of the schema-less `serde_json::Value` escape hatch (puzzle-plugin/
/// compose-kit apps stay on `serde_json::Value` end to end): delegates to `pack_rt`'s JSON bridge.
impl DocumentPack for serde_json::Value {
    fn encode_pack_with(&self, _options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        Ok(pack_rt::encode_json_value(self))
    }
    fn decode_pack_with(bytes: &[u8], _options: &PackDecodeOptions) -> Result<Self, PackError> {
        pack_rt::decode_json_value(bytes)
    }
}

/// @emoji 🔀 The closest `PackError` variant to "a text-format failure surfaced through a pack-facing
/// API" (e.g. `dsl_derive`'s generated `decode_pack_with`, whose `__dsl_from_record` step returns
/// `TextError`). A free function, not `impl From<TextError> for PackError`: both types are
/// re-exports of foreign crates (`dsl_core`/`pack_core`) through `vcs`, so a blanket `From` impl
/// here would violate the orphan rule — neither type is actually local to this crate.
pub fn text_error_to_pack_error(error: TextError) -> PackError {
    PackError::Schema(error.to_string())
}
//#endregion 🔖Pack
