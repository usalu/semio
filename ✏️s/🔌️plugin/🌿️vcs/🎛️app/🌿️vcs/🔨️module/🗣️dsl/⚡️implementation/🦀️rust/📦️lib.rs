//! 📜️ VCS app — textual document grammar surface + laws (constitutional: dsl).

use vcs::VcsDemoProjection;

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
    #[test]
    fn vcs_demo_projection_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&vcs_engine::empty_vcs_demo_projection());
    }
}
//#endregion 🧪️Tests
