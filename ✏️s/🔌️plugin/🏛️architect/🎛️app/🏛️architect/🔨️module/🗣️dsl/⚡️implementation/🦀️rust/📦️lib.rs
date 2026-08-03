//! 🗣️ Architect app document DSL surface (constitutional: dsl).

use architect::Program;

//#region 🔖️DocumentDsl
pub use architect::ARCHITECT_EXAMPLE_TEXT;

/// 🗣️ Parses an Architect document from its textual DSL representation.
pub fn parse(text: &str) -> Result<Program, store::TextError> {
    <Program as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints an Architect document in its canonical textual DSL representation.
pub fn print(document: &Program) -> String {
    store::DocumentDsl::print_dsl(document)
}
//#endregion 🔖️DocumentDsl
