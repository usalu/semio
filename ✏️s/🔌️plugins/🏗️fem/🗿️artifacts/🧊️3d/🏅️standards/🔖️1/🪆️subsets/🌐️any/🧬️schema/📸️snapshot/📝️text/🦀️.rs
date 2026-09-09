//! 📜️ FEM 3D artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::Fem3dSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📦️ The `fem3d-play` "default" example, embedded at compile time as handcrafted `.fem3d` DSL text —
/// shared by the manifest's `.example(...)` registration, the `setActiveExample` handler, and every
/// test fixture.
pub const FEM3D_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.fem3d` DSL text into a `Fem3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Fem3dSnapshot, store::TextError> {
    <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Fem3dSnapshot` back to `.fem3d` DSL text.
pub fn print_dsl(document: &Fem3dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

/// 🚀️ The document every `fem3d` surface boots with — the bundled `default` example, the only built-in
/// fixture that carries nodes/elements/solids, so the `World3d` Model window meshes real geometry on
/// first paint instead of an empty scene. Shared by `Fem3dPlayApp::initial_snapshot` and
/// `Fem3dViewer::initial_snapshot` (the viewer must never import through the sibling editor module, so
/// the shared boot document lives here, beside the fixture text itself). Falls back to the empty
/// document if the embedded fixture ever stops parsing — a boot must never fault on a fixture.
pub fn fem3d_boot_snapshot() -> Fem3dSnapshot {
    parse_dsl(FEM3D_EXAMPLE_TEXT).unwrap_or_else(|_| crate::standards::v1::subsets::any::schema::empty_fem3d_snapshot())
}

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Fem3dSnapshotText = String;
//#endregion 🚚️Carrier
