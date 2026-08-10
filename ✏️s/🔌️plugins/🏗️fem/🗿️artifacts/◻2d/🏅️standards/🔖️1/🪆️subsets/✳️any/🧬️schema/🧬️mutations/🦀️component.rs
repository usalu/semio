//! ⚡️ FEM 2D artifact — operation enum + laws (constitutional: op).

use crate::artifacts::fem2d::diff::{index_of, Fem2dDiff};
use crate::artifacts::fem2d::{element_id, Fem2dSnapshot, FemAnalysisSettings, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


// #region 🔖️Operation
/// 🧮️ Fem-2d operation: id-keyed document-collection edits, each with a true inverse computed from
/// the pre-operation snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Fem2dMutation {
    SetNode {
        index: usize,
        #[dsl(block)]
        node: FemNode,
    },
    RemoveNode {
        id: String,
    },
    SetElement {
        index: usize,
        #[dsl(statements)]
        element: Box<FemElement>,
    },
    RemoveElement {
        id: String,
    },
    SetMaterial {
        index: usize,
        #[dsl(block)]
        material: FemMaterial,
    },
    RemoveMaterial {
        id: String,
    },
    SetSection {
        index: usize,
        #[dsl(block)]
        section: FemSection,
    },
    RemoveSection {
        id: String,
    },
    SetSupport {
        index: usize,
        #[dsl(block)]
        support: FemSupport,
    },
    RemoveSupport {
        id: String,
    },
    SetLoadCase {
        index: usize,
        #[dsl(block)]
        load_case: FemLoadCase,
    },
    RemoveLoadCase {
        id: String,
    },
    SetRegion {
        index: usize,
        #[dsl(block)]
        region: FemRegion,
    },
    RemoveRegion {
        id: String,
    },
    SetCombination {
        index: usize,
        #[dsl(block)]
        combination: FemCombination,
    },
    RemoveCombination {
        id: String,
    },
    SetAnalysisSettings {
        #[dsl(block)]
        settings: FemAnalysisSettings,
    },
    /// 🌍️ Replaces the whole document (example import / reset).
    SetSnapshot {
        #[dsl(block)]
        snapshot: Fem2dSnapshot,
    },
}





impl Mutation<Fem2dSnapshot> for Fem2dMutation {
    type Diff = Fem2dDiff;

    fn diff(&self, base: &Fem2dSnapshot) -> Fem2dDiff {
        match self {
            Fem2dMutation::SetNode { index, node } => crate::artifacts::fem2d::diff::diff_set_node(*index, node.clone(), base),
            Fem2dMutation::RemoveNode { id } => crate::artifacts::fem2d::diff::diff_remove_node(id.clone()),
            Fem2dMutation::SetElement { index, element } => crate::artifacts::fem2d::diff::diff_set_element(*index, (**element).clone(), base),
            Fem2dMutation::RemoveElement { id } => crate::artifacts::fem2d::diff::diff_remove_element(id.clone()),
            Fem2dMutation::SetMaterial { index, material } => crate::artifacts::fem2d::diff::diff_set_material(*index, material.clone(), base),
            Fem2dMutation::RemoveMaterial { id } => crate::artifacts::fem2d::diff::diff_remove_material(id.clone()),
            Fem2dMutation::SetSection { index, section } => crate::artifacts::fem2d::diff::diff_set_section(*index, section.clone(), base),
            Fem2dMutation::RemoveSection { id } => crate::artifacts::fem2d::diff::diff_remove_section(id.clone()),
            Fem2dMutation::SetSupport { index, support } => crate::artifacts::fem2d::diff::diff_set_support(*index, support.clone(), base),
            Fem2dMutation::RemoveSupport { id } => crate::artifacts::fem2d::diff::diff_remove_support(id.clone()),
            Fem2dMutation::SetLoadCase { index, load_case } => crate::artifacts::fem2d::diff::diff_set_load_case(*index, load_case.clone(), base),
            Fem2dMutation::RemoveLoadCase { id } => crate::artifacts::fem2d::diff::diff_remove_load_case(id.clone()),
            Fem2dMutation::SetRegion { index, region } => crate::artifacts::fem2d::diff::diff_set_region(*index, region.clone(), base),
            Fem2dMutation::RemoveRegion { id } => crate::artifacts::fem2d::diff::diff_remove_region(id.clone()),
            Fem2dMutation::SetCombination { index, combination } => crate::artifacts::fem2d::diff::diff_set_combination(*index, combination.clone(), base),
            Fem2dMutation::RemoveCombination { id } => crate::artifacts::fem2d::diff::diff_remove_combination(id.clone()),
            Fem2dMutation::SetAnalysisSettings { settings } => crate::artifacts::fem2d::diff::diff_set_analysis(settings.clone()),
            Fem2dMutation::SetSnapshot { snapshot } => crate::artifacts::fem2d::diff::diff_set_snapshot(snapshot.clone()),
        }
    }

    fn inverse(&self, snapshot: &Fem2dSnapshot) -> Vec<Self> {
        match self {
            Fem2dMutation::SetNode { node, .. } => match index_of(&snapshot.nodes, &node.id) {
                Some(index) => vec![Fem2dMutation::SetNode { index, node: snapshot.nodes[index].clone() }],
                None => vec![Fem2dMutation::RemoveNode { id: node.id.clone() }],
            },
            Fem2dMutation::RemoveNode { id } => index_of(&snapshot.nodes, id).map(|index| vec![Fem2dMutation::SetNode { index, node: snapshot.nodes[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetElement { element, .. } => match index_of(&snapshot.elements, element_id(element)) {
                Some(index) => vec![Fem2dMutation::SetElement { index, element: Box::new(snapshot.elements[index].clone()) }],
                None => vec![Fem2dMutation::RemoveElement { id: element_id(element).to_string() }],
            },
            Fem2dMutation::RemoveElement { id } => index_of(&snapshot.elements, id).map(|index| vec![Fem2dMutation::SetElement { index, element: Box::new(snapshot.elements[index].clone()) }]).unwrap_or_default(),
            Fem2dMutation::SetMaterial { material, .. } => match index_of(&snapshot.materials, &material.id) {
                Some(index) => vec![Fem2dMutation::SetMaterial { index, material: snapshot.materials[index].clone() }],
                None => vec![Fem2dMutation::RemoveMaterial { id: material.id.clone() }],
            },
            Fem2dMutation::RemoveMaterial { id } => index_of(&snapshot.materials, id).map(|index| vec![Fem2dMutation::SetMaterial { index, material: snapshot.materials[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetSection { section, .. } => match index_of(&snapshot.sections, &section.id) {
                Some(index) => vec![Fem2dMutation::SetSection { index, section: snapshot.sections[index].clone() }],
                None => vec![Fem2dMutation::RemoveSection { id: section.id.clone() }],
            },
            Fem2dMutation::RemoveSection { id } => index_of(&snapshot.sections, id).map(|index| vec![Fem2dMutation::SetSection { index, section: snapshot.sections[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetSupport { support, .. } => match index_of(&snapshot.supports, &support.id) {
                Some(index) => vec![Fem2dMutation::SetSupport { index, support: snapshot.supports[index].clone() }],
                None => vec![Fem2dMutation::RemoveSupport { id: support.id.clone() }],
            },
            Fem2dMutation::RemoveSupport { id } => index_of(&snapshot.supports, id).map(|index| vec![Fem2dMutation::SetSupport { index, support: snapshot.supports[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetLoadCase { load_case, .. } => match index_of(&snapshot.load_cases, &load_case.id) {
                Some(index) => vec![Fem2dMutation::SetLoadCase { index, load_case: snapshot.load_cases[index].clone() }],
                None => vec![Fem2dMutation::RemoveLoadCase { id: load_case.id.clone() }],
            },
            Fem2dMutation::RemoveLoadCase { id } => index_of(&snapshot.load_cases, id).map(|index| vec![Fem2dMutation::SetLoadCase { index, load_case: snapshot.load_cases[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetRegion { region, .. } => match index_of(&snapshot.regions, &region.id) {
                Some(index) => vec![Fem2dMutation::SetRegion { index, region: snapshot.regions[index].clone() }],
                None => vec![Fem2dMutation::RemoveRegion { id: region.id.clone() }],
            },
            Fem2dMutation::RemoveRegion { id } => index_of(&snapshot.regions, id).map(|index| vec![Fem2dMutation::SetRegion { index, region: snapshot.regions[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetCombination { combination, .. } => match index_of(&snapshot.combinations, &combination.id) {
                Some(index) => vec![Fem2dMutation::SetCombination { index, combination: snapshot.combinations[index].clone() }],
                None => vec![Fem2dMutation::RemoveCombination { id: combination.id.clone() }],
            },
            Fem2dMutation::RemoveCombination { id } => index_of(&snapshot.combinations, id).map(|index| vec![Fem2dMutation::SetCombination { index, combination: snapshot.combinations[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetAnalysisSettings { .. } => vec![Fem2dMutation::SetAnalysisSettings { settings: snapshot.analysis.clone() }],
            Fem2dMutation::SetSnapshot { .. } => vec![Fem2dMutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}

pub type Fem2dEnvelope = ArtifactEnvelope<Fem2dSnapshot, Fem2dMutation>;
pub type Fem2dStore = ArtifactStore<Fem2dSnapshot, Fem2dMutation>;
// #endregion 🔖️Operation

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // #region 🔖️Fixtures
    fn simply_supported_beam_doc() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 6.0, y: 0.0 }],
            elements: vec![FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![crate::artifacts::fem2d::FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
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
            supports: vec![FemSupport { id: "s1".into(), node_id: "c0".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "c1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "self weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }
    // #endregion 🔖️Fixtures

    // #region 🔖️OpRoundTrip
    fn round_trip(snapshot: &Fem2dSnapshot, operation: &Fem2dMutation) -> Fem2dSnapshot {
        let forward = vcs::apply_mutation(snapshot, operation);
        let mut restored = forward.clone();
        for back in operation.inverse(snapshot) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, snapshot, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn node_op_round_trips() {
        let base = simply_supported_beam_doc();
        let after = round_trip(&base, &Fem2dMutation::SetNode { index: 0, node: FemNode { id: "n1".into(), x: 1.0, y: 1.0 } });
        assert_eq!(after.nodes[0].x, 1.0);
        round_trip(&base, &Fem2dMutation::RemoveNode { id: "n1".into() });
    }

    #[test]
    fn element_op_round_trips() {
        let base = simply_supported_beam_doc();
        let updated = FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() };
        round_trip(&base, &Fem2dMutation::SetElement { index: 0, element: Box::new(updated) });
        round_trip(&base, &Fem2dMutation::RemoveElement { id: "e1".into() });
    }

    #[test]
    fn material_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dMutation::SetMaterial { index: 0, material: FemMaterial { id: "steel".into(), name: "steel".into(), e: 200e9, nu: 0.3, rho: 7850.0 } });
        round_trip(&base, &Fem2dMutation::RemoveMaterial { id: "steel".into() });
    }

    #[test]
    fn section_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dMutation::SetSection { index: 0, section: FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.01, iy: 1e-4 } });
        round_trip(&base, &Fem2dMutation::RemoveSection { id: "ipe300".into() });
    }

    #[test]
    fn support_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dMutation::SetSupport { index: 0, support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Ty] } });
        round_trip(&base, &Fem2dMutation::RemoveSupport { id: "s1".into() });
    }

    #[test]
    fn load_case_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dMutation::SetLoadCase { index: 0, load_case: FemLoadCase { id: "dead".into(), name: "dead 2".into(), loads: vec![], self_weight: true } });
        round_trip(&base, &Fem2dMutation::RemoveLoadCase { id: "dead".into() });
    }

    #[test]
    fn region_op_round_trips() {
        let base = rectangle_region_doc();
        let updated = FemRegion { id: "r1".into(), name: "slab v2".into(), outline: vec![[0.0, 0.0], [5.0, 0.0], [5.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.03, material_id: "steel".into(), mesh_size: 0.5 };
        let after = round_trip(&base, &Fem2dMutation::SetRegion { index: 0, region: updated });
        assert_eq!(after.regions[0].thickness, 0.03);
        round_trip(&base, &Fem2dMutation::RemoveRegion { id: "r1".into() });
    }

    #[test]
    fn combination_op_round_trips() {
        let mut base = simply_supported_beam_doc();
        base.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![crate::artifacts::fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }] });
        let updated = FemCombination { id: "uls".into(), name: "ULS v2".into(), terms: vec![crate::artifacts::fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.4 }] };
        let after = round_trip(&base, &Fem2dMutation::SetCombination { index: 0, combination: updated });
        assert_eq!(after.combinations[0].terms[0].factor, 1.4);
        round_trip(&base, &Fem2dMutation::RemoveCombination { id: "uls".into() });
    }

    #[test]
    fn analysis_settings_op_round_trips() {
        let base = simply_supported_beam_doc();
        let after = round_trip(&base, &Fem2dMutation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        assert_eq!(after.analysis.modal_count, 5);
    }

    #[test]
    fn document_op_round_trips() {
        let base = simply_supported_beam_doc();
        let replacement = rectangle_region_doc();
        let after = round_trip(&base, &Fem2dMutation::SetSnapshot { snapshot: replacement.clone() });
        assert_eq!(after, replacement);
    }
    // #endregion 🔖️OpRoundTrip

    // #region 🔖️OpText
    #[test]
    fn fem2d_op_text_round_trips_every_variant() {
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetNode { index: 0, node: FemNode { id: "n1".into(), x: 1.0, y: 2.0 } });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveNode { id: "n1".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetElement { index: 0, element: Box::new(FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }) });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetElement { index: 0, element: Box::new(FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }) });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveElement { id: "e1".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetMaterial { index: 0, material: FemMaterial { id: "steel".into(), name: "Steel S235".into(), e: 210e9, nu: 0.3, rho: 7850.0 } });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveMaterial { id: "steel".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetSection { index: 0, section: FemSection { id: "ipe300".into(), name: "IPE 300".into(), area: 0.005381, iy: 8.356e-5 } });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveSection { id: "ipe300".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetSupport { index: 0, support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] } });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveSupport { id: "s1".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetLoadCase {
            index: 0,
            load_case: FemLoadCase {
                id: "dead".into(),
                name: "Dead Load".into(),
                loads: vec![
                    crate::artifacts::fem2d::FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: crate::artifacts::fem2d::FemDof::Ty, value: -1000.0 },
                    crate::artifacts::fem2d::FemLoad::MemberUdl { id: "l2".into(), element_id: "e1".into(), wx: 0.0, wy: -5000.0 },
                    crate::artifacts::fem2d::FemLoad::Area { id: "l3".into(), region_id: "r1".into(), pressure: 800.0 },
                ],
                self_weight: true,
            },
        });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveLoadCase { id: "dead".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetRegion {
            index: 0,
            region: FemRegion {
                id: "r1".into(),
                name: "Slab".into(),
                outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]],
                holes: vec![vec![[1.0, 1.0], [2.0, 1.0], [2.0, 1.5]]],
                thickness: 0.02,
                material_id: "steel".into(),
                mesh_size: 0.5,
            },
        });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveRegion { id: "r1".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetCombination {
            index: 0,
            combination: FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![crate::artifacts::fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, crate::artifacts::fem2d::FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] },
        });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveCombination { id: "uls".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetSnapshot { snapshot: simply_supported_beam_doc() });
    }
    // #endregion 🔖️OpText
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



pub fn apply_fem2d_mutation(snapshot: &mut Fem2dSnapshot, mutation: &Fem2dMutation) {
    *snapshot = vcs::apply_mutation(snapshot, mutation);
}

pub fn inverse_fem2d_mutation(snapshot: &Fem2dSnapshot, mutation: &Fem2dMutation) -> Vec<Fem2dMutation> {
    mutation.inverse(snapshot)
}
