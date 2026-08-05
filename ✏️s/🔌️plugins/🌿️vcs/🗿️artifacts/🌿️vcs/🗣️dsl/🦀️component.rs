//! 📜️ VCS artifact — textual document grammar surface + laws (was: constitutional `dsl`).

use crate::artifacts::vcs::VcsDemoProjection;

/// 📄️ The `demo` example checkpoint, handcrafted in the `.vcsdemo` DSL — a mid-review structural
/// change with a non-zero counter, freeform notes, an in-progress status, and a few tags.
pub const VCS_DEMO_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌿️demo.vcsdemo");

/// 📖️ Parses `.vcsdemo` DSL text into a `VcsDemoProjection`.
pub fn parse_dsl(text: &str) -> Result<VcsDemoProjection, store::TextError> {
    <VcsDemoProjection as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `VcsDemoProjection` back to `.vcsdemo` DSL text.
pub fn print_dsl(projection: &VcsDemoProjection) -> String {
    store::DocumentDsl::print_dsl(projection)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcs_demo_projection_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&crate::artifacts::vcs::engine::empty_vcs_demo_projection());
    }

    #[test]
    fn default_example_dsl_round_trips() {
        let document = parse_dsl(VCS_DEMO_DEFAULT_EXAMPLE_TEXT).expect("parse default .vcsdemo example");
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
