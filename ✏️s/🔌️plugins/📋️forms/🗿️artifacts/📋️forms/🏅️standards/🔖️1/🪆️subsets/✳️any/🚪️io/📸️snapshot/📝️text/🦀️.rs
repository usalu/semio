//! 📜️ Forms artifact — textual document grammar surface + laws (constitutional: dsl). Ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (design.md §1 CORRECTION): `store::ArtifactDsl
//! for FormsSnapshot` now lives HERE (moved from `🧬️schema/📸️snapshot`, which keeps only the struct)
//! — the native codec is one bidirectional thing and sits directly under `🚪️io/<facet>/<representation>`,
//! unsplit. This component owns the real `parse_dsl`/`print_dsl` impl plus the thin artifact-facing
//! wrappers and the canonical example fixtures and their round-trip laws.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::FormsSnapshot;

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ `ArtifactDsl` over the derived spec-driven text of `FormsSnapshot::__dsl_spec()`, the same record
/// the pack encodes; a parsed document must also pass `FormsSnapshot::validate`.
impl store::ArtifactDsl for FormsSnapshot {
    const EXTENSION: &'static str = "forms";
    fn envelope_id() -> &'static str {
        crate::FORMS_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((envelope, rest)) => {
                if !envelope.matches_identity(Self::envelope_id(), store::semio_format::Component::Dsl, 1) {
                    return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Forms text envelope mismatch", semio_framework_diagnostic::TextSpan::at(1, 1)));
                }
                rest
            },
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &crate::schema::snapshot::native_pack::record_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        let snapshot = crate::schema::snapshot::native_pack::reconstruct_record(&record).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))?;
        snapshot.validate().map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&crate::schema::snapshot::native_pack::record(self).expect("valid Forms native state"), &crate::schema::snapshot::native_pack::record_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

/// 📄️ The building-component fixture, handcrafted in the `.forms` DSL.
pub const BUILDING_COMPONENT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📇️ The Contact template in the native saved-document format.
pub const DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/📇️contact/🗣️.dsl.semio");

/// 🌱️ The Onboarding template in the native saved-document format.
pub const ONBOARDING_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🌱️onboarding/🗣️.dsl.semio");

/// 📖️ Parses `.forms` DSL text into a `FormsSnapshot` — `FormsSnapshot`'s OWN persisted wire
/// format, the derived text of its own `dsl::DslRecord` spec.
pub fn parse_dsl(text: &str) -> Result<FormsSnapshot, semio_framework_diagnostic::TextError> {
    <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `FormsSnapshot` back to `.forms` DSL text.
pub fn print_dsl(document: &FormsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ExternalBridges
/// 📖️ Parses `.forms` DSL text with a plain-`String` error, reachable from OUTSIDE this crate —
/// `store` is a private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named
/// by the exhaustive mutation case's test adapter that has to read the committed
/// `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_forms_dsl(text: &str) -> Result<FormsSnapshot, String> {
    <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ Prints a [`FormsSnapshot`] back to `.forms` DSL text under a name an external caller can reach, paired
/// with [`parse_forms_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_forms_dsl(snapshot: &FormsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️ExternalBridges
