//! 📜️ EN 1998 app — textual document grammar surface + laws (constitutional: dsl).

use crate::artifacts::en1998::Document;

/// 🗄️ The seismic-rc-frame example fixture, handcrafted in `en1998`'s DSL (`store::DocumentDsl`): a
/// high-importance dual-system RC building in seismic zone 3 on ground type D, resolved under the EN
/// annex's Type 2 spectrum on EN ground type C, with an isolated-bridge bearing check, a near-collapse
/// KL3 retrofit assessment, and companion silo/tank/tower/foundation/retaining-wall subsystem checks —
/// distinct from `Document::default()`'s DE-annex/CC2/moment-frame/KL2/significant-damage values so the
/// grammar's non-default branches (annex, importance class, structural system, ground types, spectrum
/// type, retrofit knowledge level and limit state, redundancy and chimney booleans) are exercised too.
pub const EN1998_SEISMIC_RC_FRAME_EXAMPLE_TEXT: &str = include_str!("../../📚️examples/📕️seismic-rc-frame/🗣️dsls/📕️seismic-rc-frame/🧬️component.norm.en1998.dsl.semio");

/// 📖️ Parses `.en1998` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1998` DSL text.
pub fn print_dsl(document: &Document) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&Document::default());
    }

    #[test]
    fn seismic_rc_frame_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(EN1998_SEISMIC_RC_FRAME_EXAMPLE_TEXT).expect("parse seismic rc frame example");
        assert_eq!(document.annex, "en");
        assert_eq!(document.importance_class, "cc3");
        assert_eq!(document.structural_system, "dual_system");
        assert_eq!(document.en_spectrum_type, "type2");
        assert_eq!(document.retrofit_limit_state, "near_collapse");
        assert!(!document.multiple_resisting_systems);
        assert!(!document.tower_is_chimney);
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
