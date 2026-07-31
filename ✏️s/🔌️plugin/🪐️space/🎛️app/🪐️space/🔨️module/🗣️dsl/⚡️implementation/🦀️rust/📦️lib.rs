//! 📜️ S Studio app — textual document grammar surface + laws (constitutional: dsl).
//!
//! 🕳️ Wraps `semio_framework_os::OsProjection` (os-core's own `DocumentDsl` derive), not a locally-owned
//! document type — see `space_op`'s doc comment for why this app owns no document/operation type.

use semio_framework_os::OsProjection;

/// 📖️ Parses `OsProjection` DSL text (the `.s` studio grammar) into an `OsProjection`.
pub fn parse_dsl(text: &str) -> Result<OsProjection, store::TextError> {
    <OsProjection as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints an `OsProjection` back to `.s` studio DSL text.
pub fn print_dsl(projection: &OsProjection) -> String {
    store::DocumentDsl::print_dsl(projection)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use space_shared::demo_space_projection;

    #[test]
    fn demo_document_dsl_text_round_trips() {
        let projection = demo_space_projection();
        store::test_support::assert_dsl_round_trip(&projection);
    }

    #[test]
    fn parse_dsl_print_dsl_agree_on_demo_fixture() {
        let projection = demo_space_projection();
        let printed = print_dsl(&projection);
        let reparsed = parse_dsl(&printed).expect("reparse printed dsl");
        assert_eq!(reparsed, projection);
    }
}
//#endregion 🧪️Tests
