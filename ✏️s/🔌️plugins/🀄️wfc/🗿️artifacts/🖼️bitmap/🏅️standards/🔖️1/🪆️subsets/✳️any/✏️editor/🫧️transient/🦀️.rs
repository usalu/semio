//! 🫧️ Bitmap app-local computed solve state shared by every window of one editor instance. The
//! INFERRED output bitmap lives here and only here: it is never a snapshot field, never an edit,
//! never undoable — exactly the law the schema tree states. A `Solve` command recomputes it; every
//! other command leaves it alone, which is why a stale render is visibly stale rather than silently
//! wrong.

/// ✉️ The envelope id must be DOTTED (`plugin.artifact`). `DslArtifact` falls back to the extension
/// when no `id` is given, and `SemioEnvelope::from_envelope_id` refuses an id without a `.` — so a
/// bare `wfcbitmaptransient` made `print_dsl`'s own `expect` PANIC the guest the first time the solve
/// was published on the transient lane, trapping every later dispatch in the shell
/// (found live on the bitmap playground, 2026-09-18).
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(id = "wfc.bitmaptransient")]
#[artifact(extension = "wfcbitmaptransient")]
#[dsl(layout = "lines")]
pub struct BitmapTransient {
    /// 🖼️ Base64 palette indices of the last solved output, row-major. `None` until a solve ran.
    pub output_pixels: Option<String>,
    /// 🩺 Whether the last solve ended in a contradiction rather than a complete assignment.
    pub contradiction: bool,
    /// 📐️ The output extent the cached pixels were solved for, so a render never reshapes a stale
    /// buffer into the document's current extent and shows a plausible lie.
    pub output_width: u32,
    pub output_height: u32,
}

impl store::ArtifactDsl for BitmapTransient {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid bitmap transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for BitmapTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl protocol::MutationDiff<BitmapTransient> for BitmapTransient {
    fn apply(&self, _base: &BitmapTransient) -> protocol::MutationApplyResult<BitmapTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
