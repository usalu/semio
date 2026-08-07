//! 📜️ Forms artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `store::DocumentDsl for FormSpec` is implemented directly in the shared `playbook` kernel crate; see
//! `🗿️artifacts/📋️forms/🦀️component.rs` for why. This component only adds the thin artifact-facing
//! `parse_dsl`/`print_dsl` wrappers plus the canonical example fixtures and their round-trip laws.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::forms::FormSpec;

/// 📄️ The building-component fixture, handcrafted in the `.forms` DSL.
pub const BUILDING_COMPONENT_EXAMPLE_TEXT: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📄️ The `default` (Contact) fixture — a minimal single-step form, in the `.forms` DSL.
pub const DEFAULT_EXAMPLE_TEXT: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📄️ The `onboarding` fixture — a multi-step form exercising every built-in question kind and a
/// conditional block, in the `.forms` DSL.
pub const ONBOARDING_EXAMPLE_TEXT: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.forms` DSL text into a `FormSpec`.
pub fn parse_dsl(text: &str) -> Result<FormSpec, store::TextError> {
    <FormSpec as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `FormSpec` back to `.forms` DSL text.
pub fn print_dsl(document: &FormSpec) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use store::test_support::assert_dsl_round_trip;

    #[test]
    fn building_component_fixture_dsl_round_trips() {
        let spec = parse_dsl(BUILDING_COMPONENT_EXAMPLE_TEXT).expect("📋️building-component.forms parses");
        assert_eq!(spec.id, "building-component");
        assert_eq!(spec.steps.len(), 2);
        assert_dsl_round_trip(&spec);
    }

    #[test]
    fn default_fixture_dsl_round_trips() {
        let spec = parse_dsl(DEFAULT_EXAMPLE_TEXT).expect("📋️default.forms parses");
        assert_eq!(spec.id, "default");
        assert_eq!(spec.steps.len(), 1);
        assert_dsl_round_trip(&spec);
    }

    #[test]
    fn onboarding_fixture_dsl_round_trips() {
        let spec = parse_dsl(ONBOARDING_EXAMPLE_TEXT).expect("📋️onboarding.forms parses");
        assert_eq!(spec.id, "onboarding");
        assert_eq!(spec.steps.len(), 3);
        assert_dsl_round_trip(&spec);
    }
}
//#endregion 🧪️Tests
