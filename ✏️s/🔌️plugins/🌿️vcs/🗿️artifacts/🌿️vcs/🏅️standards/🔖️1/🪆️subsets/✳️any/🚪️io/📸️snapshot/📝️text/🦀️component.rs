//! 📜️ VCS artifact — native `.vcs` DSL text codec (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §1 CORRECTION: the native codec is
//! one bidirectional thing and sits directly under `🚪️io/<facet>/<representation>/`, unsplit).
//! Relocated here from `🧬️schema/📸️snapshot/📝️text` — the real `store::ArtifactDsl for VcsSnapshot`
//! impl moved with it; `🧬️schema` keeps only the `VcsSnapshot` type. `store::ArtifactPack`'s twin
//! impl sits in the sibling `💾️binary` facet.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::vcs::VcsSnapshot;

//#region 🔖️ArtifactDslCodec
impl store::ArtifactDsl for VcsSnapshot {
    const EXTENSION: &'static str = "vcs";
    fn envelope_id() -> &'static str {
        "vcs.vcs"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions {
                limits: dsl::Limits::default(),
                mode: dsl::SourceMode::Document,
            },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️ArtifactDslCodec

//#region 🔖️Example
/// 📄️ The `demo` example checkpoint, handcrafted in the `.vcsdemo` DSL — a mid-review structural
/// change with a non-zero counter, freeform notes, an in-progress status, and a few tags.
pub const VCS_DEMO_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.vcsdemo` DSL text into a `VcsSnapshot`.
pub fn parse_dsl(text: &str) -> Result<VcsSnapshot, store::TextError> {
    <VcsSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `VcsSnapshot` back to `.vcsdemo` DSL text.
pub fn print_dsl(projection: &VcsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(projection)
}
//#endregion 🔖️Example

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcs_demo_projection_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&crate::artifacts::vcs::standards::v1::subsets::any::schema::empty_vcs_snapshot());
    }

    #[test]
    fn default_example_dsl_round_trips() {
        let document = parse_dsl(VCS_DEMO_DEFAULT_EXAMPLE_TEXT).expect("parse default .vcsdemo example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
