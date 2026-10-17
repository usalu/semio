//! 🔤️ Operation text and pack codec of the norm compliance core: the field escaping, the artifact record trait and the snapshot dsl and pack codec. See [`super`].

use super::*;

// #region 🔖️OpText
/// ✂️ Escapes `\`, `"`, `\n` for embedding arbitrary (possibly multi-line) text inside a single quoted
/// op-text field. Mirrors vcs's own private `escape_text_field`/`unescape_text_field` convention
/// exactly (same three escapes, same order) so escaping behaves identically repo-wide, even though vcs
/// does not expose those helpers for reuse.
pub fn escape_op_text_field(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            _ => out.push(ch),
        }
    }
    out
}

/// ✂️ Inverts [`escape_op_text_field`] for artifact snapshot commands.
pub fn unescape_op_text_field(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('"') => out.push('"'),
                Some('\\') => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

//#region 🔖️SnapshotDslPackCodec
/// 🧬️ P6: `dsl::DslRecord`/`dsl::DslArtifact` emit the inherent `__dsl_spec`/`__dsl_to_record`/
/// `__dsl_from_record` trio only — never `pack::snapshot::ArtifactDsl`/`pack::snapshot::ArtifactPack` themselves (a
/// permanent framework decision, see `🗣️dsl/✨️derive/🦀️.rs`'s "P6" header comment and every
/// other P6-migrated document kind, e.g. `🪐️space`, `🔁️workflow`, `📖️playbook`). Every one of norm's
/// fifteen family snapshots therefore needs the identical envelope-wrap/unwrap glue around that
/// trio; W5a (ticket 26/08/11/SEMIO-ARTIFACT-…) found it hand-duplicated verbatim in fifteen
/// `📸️snapshot/🦀️.rs` files' `HandcraftedArtifactCodecs` regions and consolidated it here —
/// this trait plus the four functions below are norm's single copy. Each snapshot type now needs
/// only a five-line `NormArtifactRecord` impl (its two identifying strings, delegating straight to
/// its own derive-generated `__dsl_*` trio) plus two one-line-bodied `ArtifactDsl`/`ArtifactPack`
/// impls that call through here — Rust has no blanket `impl<T> Trait for T` escape for a foreign
/// trait for a foreign-per-crate reason (orphan rule) here, so the per-type impl shells stay, but
/// every byte of actual encode/decode logic now lives exactly once.
pub trait NormArtifactRecord: Sized {
    /// 🏷️ File extension this family's DSL text uses (`ArtifactDsl::EXTENSION`).
    const EXTENSION: &'static str;
    /// 🆔️ Semio envelope id (`"norm.<family>"`) both the text and pack wire wrap themselves in.
    const ENVELOPE_ID: &'static str;
    fn dsl_spec() -> semio_framework_dsl_record::RecordSpec;
    fn dsl_to_record(&self) -> semio_framework_dsl_record::RecordValue;
    fn dsl_from_record(record: &semio_framework_dsl_record::RecordValue) -> Result<Self, TextError>;
}

/// 📖️ Shared `ArtifactDsl::parse_dsl` body: strip the optional semio-format text preamble, parse the
/// remaining body against `T`'s record spec, then lower the parsed record back into `T`.
pub fn norm_parse_dsl<T: NormArtifactRecord>(text: &str) -> Result<T, TextError> {
    let body = match store::semio_format::split_text_preamble(text) {
        Ok((_, rest)) => rest,
        Err(_) => text,
    };
    let record = semio_framework_dsl_record::parse(body, &T::dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
    T::dsl_from_record(&record)
}

/// 🖨️ Shared `ArtifactDsl::print_dsl` body: print `T`'s record, then wrap it in its semio envelope.
pub fn norm_print_dsl<T: NormArtifactRecord>(value: &T) -> String {
    let body = semio_framework_dsl_record::print(&value.dsl_to_record(), &T::dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
    let envelope = store::semio_format::SemioEnvelope::from_envelope_id(T::ENVELOPE_ID, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
    store::semio_format::wrap_text(&envelope, &body)
}

/// 📦️ Shared `ArtifactPack::encode_pack_with` body: pack-encode `T`'s record, then envelope-wrap it.
pub fn norm_encode_pack<T: NormArtifactRecord>(value: &T, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
    let inner = store::pack_rt::encode_document(&T::dsl_spec(), &value.dsl_to_record(), options)?;
    let envelope = store::semio_format::SemioEnvelope::from_envelope_id(T::ENVELOPE_ID, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
    Ok(store::semio_format::wrap_binary(&envelope, &inner))
}

/// 📦️ Shared `ArtifactPack::decode_pack_with` body: unwrap the envelope (checking it matches `T`'s
/// own), pack-decode the inner record, then lower it back into `T`.
pub fn norm_decode_pack<T: NormArtifactRecord>(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<T, store::PackError> {
    let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
    if !envelope.matches_identity(T::ENVELOPE_ID, store::semio_format::Component::Pack, 1) {
        return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", T::ENVELOPE_ID, envelope.binary_token()))));
    }
    let (record, _report) = store::pack_rt::decode_document(&inner, &T::dsl_spec(), options)?;
    T::dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
}
//#endregion 🔖️SnapshotDslPackCodec
// #endregion 🔖️OpText
