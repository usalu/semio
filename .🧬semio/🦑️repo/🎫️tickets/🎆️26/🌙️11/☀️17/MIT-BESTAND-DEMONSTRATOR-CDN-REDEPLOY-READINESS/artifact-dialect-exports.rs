//! 🌿️ Exercise the artifact-reference exports that release consumers import.
#[path = "../../../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/🦀️.rs"]
mod artifact_reference;
pub use artifact_reference::*;

fn view<'a>(row: &'a &'static str) -> (&'a str, &'a str, &'a str) { ("kind", "standard", *row) }

fn main() {
    let all = SubsetId::ANY.0;
    assert_eq!(most_general_dialect(vec!["doors", all], view), MostGeneralDialect::One(all));
    assert_eq!(most_general_dialect(vec!["doors", "walls"], view), MostGeneralDialect::Ambiguous);
    assert_eq!(most_general_dialect(vec![], view), MostGeneralDialect::Empty);
}
