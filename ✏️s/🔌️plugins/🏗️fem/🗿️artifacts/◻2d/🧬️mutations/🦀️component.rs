//! ⚡️ FEM 2D artifact — operation enum + laws (constitutional: op).

use crate::artifacts::fem2d::diff::{index_of, Fem2dDiff};
use crate::artifacts::fem2d::{element_id, Fem2dDocument, FemAnalysisSettings, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


// #region 🔖️Operation
/// 🧮️ Fem-2d operation: id-keyed document-collection edits, each with a true inverse computed from
/// the pre-operation projection.
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
    SetDocument {
        #[dsl(block)]
        document: Fem2dDocument,
    },
}





impl Mutation<Fem2dDocument> for Fem2dMutation {
    type Diff = Fem2dDiff;

    fn diff(&self, _projection: &Fem2dDocument) -> Fem2dDiff {
        let mut diff = Fem2dDiff::default();
        match self {
            Fem2dMutation::SetNode { index, node } => diff.nodes.set.push((*index, node.clone())),
            Fem2dMutation::RemoveNode { id } => diff.nodes.removed.push(id.clone()),
            Fem2dMutation::SetElement { index, element } => diff.elements.set.push((*index, (**element).clone())),
            Fem2dMutation::RemoveElement { id } => diff.elements.removed.push(id.clone()),
            Fem2dMutation::SetMaterial { index, material } => diff.materials.set.push((*index, material.clone())),
            Fem2dMutation::RemoveMaterial { id } => diff.materials.removed.push(id.clone()),
            Fem2dMutation::SetSection { index, section } => diff.sections.set.push((*index, section.clone())),
            Fem2dMutation::RemoveSection { id } => diff.sections.removed.push(id.clone()),
            Fem2dMutation::SetSupport { index, support } => diff.supports.set.push((*index, support.clone())),
            Fem2dMutation::RemoveSupport { id } => diff.supports.removed.push(id.clone()),
            Fem2dMutation::SetLoadCase { index, load_case } => diff.load_cases.set.push((*index, load_case.clone())),
            Fem2dMutation::RemoveLoadCase { id } => diff.load_cases.removed.push(id.clone()),
            Fem2dMutation::SetRegion { index, region } => diff.regions.set.push((*index, region.clone())),
            Fem2dMutation::RemoveRegion { id } => diff.regions.removed.push(id.clone()),
            Fem2dMutation::SetCombination { index, combination } => diff.combinations.set.push((*index, combination.clone())),
            Fem2dMutation::RemoveCombination { id } => diff.combinations.removed.push(id.clone()),
            Fem2dMutation::SetAnalysisSettings { settings } => diff.analysis = Some(settings.clone()),
            Fem2dMutation::SetDocument { document } => diff.document = Some(document.clone()),
        }
        diff
    }

    fn inverse(&self, projection: &Fem2dDocument) -> Vec<Self> {
        match self {
            Fem2dMutation::SetNode { node, .. } => match index_of(&projection.nodes, &node.id) {
                Some(index) => vec![Fem2dMutation::SetNode { index, node: projection.nodes[index].clone() }],
                None => vec![Fem2dMutation::RemoveNode { id: node.id.clone() }],
            },
            Fem2dMutation::RemoveNode { id } => index_of(&projection.nodes, id).map(|index| vec![Fem2dMutation::SetNode { index, node: projection.nodes[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetElement { element, .. } => match index_of(&projection.elements, element_id(element)) {
                Some(index) => vec![Fem2dMutation::SetElement { index, element: Box::new(projection.elements[index].clone()) }],
                None => vec![Fem2dMutation::RemoveElement { id: element_id(element).to_string() }],
            },
            Fem2dMutation::RemoveElement { id } => index_of(&projection.elements, id).map(|index| vec![Fem2dMutation::SetElement { index, element: Box::new(projection.elements[index].clone()) }]).unwrap_or_default(),
            Fem2dMutation::SetMaterial { material, .. } => match index_of(&projection.materials, &material.id) {
                Some(index) => vec![Fem2dMutation::SetMaterial { index, material: projection.materials[index].clone() }],
                None => vec![Fem2dMutation::RemoveMaterial { id: material.id.clone() }],
            },
            Fem2dMutation::RemoveMaterial { id } => index_of(&projection.materials, id).map(|index| vec![Fem2dMutation::SetMaterial { index, material: projection.materials[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetSection { section, .. } => match index_of(&projection.sections, &section.id) {
                Some(index) => vec![Fem2dMutation::SetSection { index, section: projection.sections[index].clone() }],
                None => vec![Fem2dMutation::RemoveSection { id: section.id.clone() }],
            },
            Fem2dMutation::RemoveSection { id } => index_of(&projection.sections, id).map(|index| vec![Fem2dMutation::SetSection { index, section: projection.sections[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetSupport { support, .. } => match index_of(&projection.supports, &support.id) {
                Some(index) => vec![Fem2dMutation::SetSupport { index, support: projection.supports[index].clone() }],
                None => vec![Fem2dMutation::RemoveSupport { id: support.id.clone() }],
            },
            Fem2dMutation::RemoveSupport { id } => index_of(&projection.supports, id).map(|index| vec![Fem2dMutation::SetSupport { index, support: projection.supports[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetLoadCase { load_case, .. } => match index_of(&projection.load_cases, &load_case.id) {
                Some(index) => vec![Fem2dMutation::SetLoadCase { index, load_case: projection.load_cases[index].clone() }],
                None => vec![Fem2dMutation::RemoveLoadCase { id: load_case.id.clone() }],
            },
            Fem2dMutation::RemoveLoadCase { id } => index_of(&projection.load_cases, id).map(|index| vec![Fem2dMutation::SetLoadCase { index, load_case: projection.load_cases[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetRegion { region, .. } => match index_of(&projection.regions, &region.id) {
                Some(index) => vec![Fem2dMutation::SetRegion { index, region: projection.regions[index].clone() }],
                None => vec![Fem2dMutation::RemoveRegion { id: region.id.clone() }],
            },
            Fem2dMutation::RemoveRegion { id } => index_of(&projection.regions, id).map(|index| vec![Fem2dMutation::SetRegion { index, region: projection.regions[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetCombination { combination, .. } => match index_of(&projection.combinations, &combination.id) {
                Some(index) => vec![Fem2dMutation::SetCombination { index, combination: projection.combinations[index].clone() }],
                None => vec![Fem2dMutation::RemoveCombination { id: combination.id.clone() }],
            },
            Fem2dMutation::RemoveCombination { id } => index_of(&projection.combinations, id).map(|index| vec![Fem2dMutation::SetCombination { index, combination: projection.combinations[index].clone() }]).unwrap_or_default(),
            Fem2dMutation::SetAnalysisSettings { .. } => vec![Fem2dMutation::SetAnalysisSettings { settings: projection.analysis.clone() }],
            Fem2dMutation::SetDocument { .. } => vec![Fem2dMutation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type Fem2dEnvelope = DocumentEnvelope<Fem2dDocument, Fem2dMutation>;
pub type Fem2dStore = DocumentStore<Fem2dDocument, Fem2dMutation>;
// #endregion 🔖️Operation

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // #region 🔖️Fixtures
    fn simply_supported_beam_doc() -> Fem2dDocument {
        Fem2dDocument {
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

    fn rectangle_region_doc() -> Fem2dDocument {
        Fem2dDocument {
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
    fn round_trip(projection: &Fem2dDocument, operation: &Fem2dMutation) -> Fem2dDocument {
        let forward = vcs::apply_mutation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.inverse(projection) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation document");
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
        let after = round_trip(&base, &Fem2dMutation::SetDocument { document: replacement.clone() });
        assert_eq!(after, replacement);
    }
    // #endregion 🔖️OpRoundTrip

    // #region 🔖️OpText
    #[test]
    fn fem2d_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetNode { index: 0, node: FemNode { id: "n1".into(), x: 1.0, y: 2.0 } });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveNode { id: "n1".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetElement { index: 0, element: Box::new(FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }) });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetElement { index: 0, element: Box::new(FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }) });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveElement { id: "e1".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetMaterial { index: 0, material: FemMaterial { id: "steel".into(), name: "Steel S235".into(), e: 210e9, nu: 0.3, rho: 7850.0 } });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveMaterial { id: "steel".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetSection { index: 0, section: FemSection { id: "ipe300".into(), name: "IPE 300".into(), area: 0.005381, iy: 8.356e-5 } });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveSection { id: "ipe300".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetSupport { index: 0, support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] } });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveSupport { id: "s1".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetLoadCase {
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
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveLoadCase { id: "dead".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetRegion {
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
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveRegion { id: "r1".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetCombination {
            index: 0,
            combination: FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![crate::artifacts::fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, crate::artifacts::fem2d::FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] },
        });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveCombination { id: "uls".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        store::test_support::assert_op_line_round_trip(&Fem2dMutation::SetDocument { document: simply_supported_beam_doc() });
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



pub fn apply_fem2d_mutation(projection: &mut Fem2dDocument, mutation: &Fem2dMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_fem2d_mutation(projection: &Fem2dDocument, mutation: &Fem2dMutation) -> Vec<Fem2dMutation> {
    mutation.inverse(projection)
}
