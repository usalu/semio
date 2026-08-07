//! 🗣️ CAD artifact — the textual `.cad` document grammar surface: `parse_dsl`/`print_dsl` over the
//! derive-generated `store::DocumentDsl`, plus the handcrafted `default` example the app registers.

use crate::artifacts::cad::CadProjection;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📄️ The `default` example scene, handcrafted in the `.cad` DSL — a small structural column with
/// a two-vertex/one-edge/one-wire/one-face/one-shell/one-solid brep, a site-photo reference, and
/// objects across the shape/building/structure-classic panes.
pub const CAD_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../📚️examples/♻️default/🗣️dsls/♻️default/🧬️component.cad.cad.dsl.semio");

/// 📖️ Parses `.cad` DSL text into a `CadProjection`.
pub fn parse_dsl(text: &str) -> Result<CadProjection, store::TextError> {
    <CadProjection as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `CadProjection` back to `.cad` DSL text.
pub fn print_dsl(document: &CadProjection) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::cad::testkit::{sample_geometry, sample_scene};

    #[test]
    fn default_example_dsl_round_trips() {
        let document = parse_dsl(CAD_DEFAULT_EXAMPLE_TEXT).expect("parse default .cad example");
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn cad_scene_round_trips_through_dsl_document() {
        store::test_support::assert_dsl_round_trip(&sample_scene());
    }

    #[test]
    fn cad_scene_with_all_geometry_panes_round_trips_through_dsl_document() {
        let mut scene = sample_scene();
        scene.building_geometry = Some(sample_geometry());
        scene.energy_geometry = Some(sample_geometry());
        scene.structure_classic_geometry = Some(sample_geometry());
        store::test_support::assert_dsl_round_trip(&scene);
    }
}
//#endregion 🧪️Tests
