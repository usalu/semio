//! 🗣️ Animate presentation artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::{PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};

/// 📄️ The handcrafted `.presentation` DSL-text fixture — a multi-tile deck exercising every field
/// (including the optional `source-aspect`), embedded at compile time as the permanent proof that
/// the checked-in fixture still parses and round trips.
pub const PRESENTATION_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.presentation` DSL text into a `PresentationSnapshot`.
pub fn parse_dsl(text: &str) -> Result<PresentationSnapshot, semio_framework_diagnostic::TextError> {
    <PresentationSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `PresentationSnapshot` back to `.presentation` DSL text.
pub fn print_dsl(deck: &PresentationSnapshot) -> String {
    store::ArtifactDsl::print_dsl(deck)
}

//#region 🔖️HandcraftedArtifactDsl
/// 🚪️ Moved from `../../../🧬️schema/📸️snapshot/🦀️.rs` per design.md's CORRECTION — the
/// native codec sits directly under `🚪️io/<facet>/<representation>/`, unsplit (one bidirectional
/// trait impl, not an import/export mirror).
impl store::ArtifactDsl for PresentationSnapshot {
    const EXTENSION: &'static str = "presentation";
    fn envelope_id() -> &'static str {
        PRESENTATION_DOCUMENT_SCHEMA
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests


#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use crate::{AnimationChild, PresentationChild, PRESENTATION_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;

pub fn build_tile_morph_prompt(source: &crate::FigureTileSource, drafts: &[crate::FigureTileDraft]) -> String {
    fn format_frame(frame: &crate::FigureTileFrame) -> String {
        format!("{{ x: {:.6}, y: {:.6}, width: {:.6}, height: {:.6} }}", frame.x, frame.y, frame.width, frame.height)
    }
    let kind = if source.kind.is_empty() { "figure" } else { source.kind.as_str() };
    let mut lines = vec![
        "Wire a one-to-many morph for animate presentation deck tiles using the parameters below.".into(),
        String::new(),
        "## Source media".into(),
        format!("- kind: {kind}"),
        format!("- src: {}", semio_framework_pack_json::to_json_string(&source.src)),
    ];
    if let Some(aspect) = source.source_aspect {
        lines.push(format!("- sourceAspect: {aspect}"));
    }
    if kind == "pdf" {
        if let Some(page) = source.pdf_page {
            lines.push(format!("- pdfPage: {page}"));
        }
    }
    lines.push(format!("- frame: {}", format_frame(&source.frame)));
    lines.push(String::new());
    lines.push("## Tiles (normalized source crops; overlap allowed)".into());
    for draft in drafts {
        lines.push(format!("- {} ({}): crop {}", draft.name, draft.id, format_frame(&draft.crop)));
    }
    let embodiment_hint = match kind {
        "video" => "Use video embodiments for tile participants and the source clip.",
        "pdf" => "Use pdf embodiments for tile participants and the source document page.",
        _ => "Register one participant per tile with a tile figure embodiment using each crop above.",
    };
    lines.push(String::new());
    lines.push("## Task".into());
    lines.push(format!("1. {embodiment_hint}"));
    lines.push("2. On the source slide, place the full media with morphTo slots pointing at each tile participant.".into());
    lines.push("3. Use reveal.js auto-animate; morph from the actual disposition including ephemeral modifications.".into());
    lines.join("\n")
}
}
pub use snapshot_codec::*;
