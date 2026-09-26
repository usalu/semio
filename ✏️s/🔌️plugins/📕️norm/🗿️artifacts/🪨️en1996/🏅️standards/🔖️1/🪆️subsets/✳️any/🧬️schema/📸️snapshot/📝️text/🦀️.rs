//! 📜️ EN 1996 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::En1996Snapshot;

/// 🗄️ The load-bearing-wall example fixture, handcrafted in `en1996`'s DSL (`store::ArtifactDsl`):
/// an EN-annex masonry class 2 wall check under a transient design situation, distinct from
/// `En1996Snapshot::default()`'s DE-annex/persistent values so the grammar's non-default branches
/// (annex, masonry class, design situation, exposure, mortar) are exercised too.
pub const EN1996_LOADBEARING_WALL_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🧱️loadbearing-wall/🧱️loadbearing-wall/🗣️.dsl.semio");

/// 📖️ Parses `.en1996` DSL text into a `En1996Snapshot`.
pub fn parse_dsl(text: &str) -> Result<En1996Snapshot, store::TextError> {
    <En1996Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `En1996Snapshot` back to `.en1996` DSL text.
pub fn print_dsl(document: &En1996Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type En1996SnapshotText = String;
//#endregion 🚚️Carrier

/// 🖼️ Write committed DSL + pack assets for the catalogue examples (invoked by 📜️script.ts).
pub fn write_committed_example_assets(assets_root: &std::path::Path) -> std::io::Result<()> {
    use crate::En1996Snapshot;
    for (folder, doc) in [
        ("🧱️loadbearing-wall", En1996Snapshot::compliant_clay_wall()),
        ("❌️multi-fail-masonry", En1996Snapshot::noncompliant_multi_fail()),
    ] {
        let dir = assets_root.join(folder).join(folder);
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join("🗣️.dsl.semio"), print_dsl(&doc))?;
        let pack = <En1996Snapshot as store::ArtifactPack>::encode_pack(&doc);
        std::fs::write(dir.join("🎒️.pack.semio"), pack)?;
    }
    Ok(())
}
