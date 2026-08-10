//! 📜️ FEM 2D artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


/// 📦️ The `fem2d-play` "default" example, embedded at compile time as handcrafted `.fem2d` DSL text —
/// shared by the manifest's `.example(...)` registration, the `setActiveExample` handler, and every
/// test fixture.
pub const FEM2D_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.fem2d` DSL text into a `Fem2dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Fem2dSnapshot, store::TextError> {
    <Fem2dSnapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Fem2dSnapshot` back to `.fem2d` DSL text.
pub fn print_dsl(document: &Fem2dSnapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::fem2d::{FemAnalysisSettings, FemCombination, FemCombinationTerm, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};

    // #region 🔖️Fixtures
    fn simply_supported_beam_doc() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 6.0, y: 0.0 }],
            elements: vec![FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    fn truss_doc() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 4.0, y: 0.0 }, FemNode { id: "n3".into(), x: 4.0, y: 3.0 }],
            elements: vec![
                FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "e2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
            ],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "rod".into(), name: "rod".into(), area: 0.001, iy: 0.0 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }],
            load_cases: vec![FemLoadCase {
                id: "dead".into(),
                name: "dead".into(),
                loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n3".into(), dof: FemDof::Ty, value: -1000.0 }, FemLoad::Nodal { id: "l2".into(), node_id: "n3".into(), dof: FemDof::Tx, value: -500.0 }],
                self_weight: false,
            }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    fn rectangle_region_doc() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![FemNode { id: "c0".into(), x: 0.0, y: 0.0 }, FemNode { id: "c1".into(), x: 4.0, y: 0.0 }, FemNode { id: "c2".into(), x: 4.0, y: 2.0 }, FemNode { id: "c3".into(), x: 0.0, y: 2.0 }],
            elements: vec![],
            regions: vec![FemRegion { id: "r1".into(), name: "slab".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 1.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "c0".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "c1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "self weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    fn rectangle_with_hole_region_doc() -> Fem2dSnapshot {
        let mut doc = rectangle_region_doc();
        doc.regions[0].holes = vec![vec![[1.5, 0.75], [2.5, 0.75], [2.5, 1.25], [1.5, 1.25]]];
        doc
    }
    // #endregion 🔖️Fixtures

    #[test]
    fn fem2d_dsl_round_trips_bundled_default_example() {
        let document = parse_dsl(FEM2D_EXAMPLE_TEXT).expect("parse default example");

        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn fem2d_dsl_round_trips_fixture_documents() {
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&Fem2dSnapshot::default());
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&simply_supported_beam_doc());
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&truss_doc());
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&rectangle_with_hole_region_doc());
        let mut with_combination = simply_supported_beam_doc();
        with_combination.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] });
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&with_combination);
    }
}
// #endregion 🧪️Tests

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[test]
    fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}

