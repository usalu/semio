//! ⚡️ FEM 3D artifact — operation enum + laws (constitutional: op).

use crate::artifacts::fem3d::diff::{index_of, Fem3dDiff};
use crate::artifacts::fem3d::{element_id, Fem3dSnapshot, FemAnalysisSettings, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


// #region 🔖️Operations
/// 🧮️ Fem-3d operation: id-keyed collection edits over nodes/elements/materials/sections/supports/load
/// cases, each with a true inverse via `backwards`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Fem3dMutation {
    SetNode {
        index: usize,
        node: FemNode,
    },
    RemoveNode {
        id: String,
    },
    // `FemElement` is a `DslEnum` (tagged, data-carrying variants), not a `DslRecord`, so it has no
    // `DslField` impl of its own — a bare scalar field can't bind it directly. `#[dsl(statements)]`
    // on a `Box<T>` is the engine's "exactly one required tagged value" shape for that case.
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
        material: FemMaterial,
    },
    RemoveMaterial {
        id: String,
    },
    SetSection {
        index: usize,
        section: FemSection,
    },
    RemoveSection {
        id: String,
    },
    SetSolid {
        index: usize,
        solid: FemSolid,
    },
    RemoveSolid {
        id: String,
    },
    SetSupport {
        index: usize,
        support: FemSupport,
    },
    RemoveSupport {
        id: String,
    },
    SetLoadCase {
        index: usize,
        load_case: FemLoadCase,
    },
    RemoveLoadCase {
        id: String,
    },
    SetCombination {
        index: usize,
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
        snapshot: Fem3dSnapshot,
    },
}





impl Mutation<Fem3dSnapshot> for Fem3dMutation {
    type Diff = Fem3dDiff;

    fn diff(&self, base: &Fem3dSnapshot) -> Fem3dDiff {
        match self {
            Fem3dMutation::SetNode { index, node } => crate::artifacts::fem3d::diff::diff_set_node(*index, node.clone(), base),
            Fem3dMutation::RemoveNode { id } => crate::artifacts::fem3d::diff::diff_remove_node(id.clone()),
            Fem3dMutation::SetElement { index, element } => crate::artifacts::fem3d::diff::diff_set_element(*index, (**element).clone(), base),
            Fem3dMutation::RemoveElement { id } => crate::artifacts::fem3d::diff::diff_remove_element(id.clone()),
            Fem3dMutation::SetMaterial { index, material } => crate::artifacts::fem3d::diff::diff_set_material(*index, material.clone(), base),
            Fem3dMutation::RemoveMaterial { id } => crate::artifacts::fem3d::diff::diff_remove_material(id.clone()),
            Fem3dMutation::SetSection { index, section } => crate::artifacts::fem3d::diff::diff_set_section(*index, section.clone(), base),
            Fem3dMutation::RemoveSection { id } => crate::artifacts::fem3d::diff::diff_remove_section(id.clone()),
            Fem3dMutation::SetSupport { index, support } => crate::artifacts::fem3d::diff::diff_set_support(*index, support.clone(), base),
            Fem3dMutation::RemoveSupport { id } => crate::artifacts::fem3d::diff::diff_remove_support(id.clone()),
            Fem3dMutation::SetLoadCase { index, load_case } => crate::artifacts::fem3d::diff::diff_set_load_case(*index, load_case.clone(), base),
            Fem3dMutation::RemoveLoadCase { id } => crate::artifacts::fem3d::diff::diff_remove_load_case(id.clone()),
            Fem3dMutation::SetSolid { index, solid } => crate::artifacts::fem3d::diff::diff_set_solid(*index, solid.clone(), base),
            Fem3dMutation::RemoveSolid { id } => crate::artifacts::fem3d::diff::diff_remove_solid(id.clone()),
            Fem3dMutation::SetCombination { index, combination } => crate::artifacts::fem3d::diff::diff_set_combination(*index, combination.clone(), base),
            Fem3dMutation::RemoveCombination { id } => crate::artifacts::fem3d::diff::diff_remove_combination(id.clone()),
            Fem3dMutation::SetAnalysisSettings { settings } => crate::artifacts::fem3d::diff::diff_set_analysis(settings.clone()),
            Fem3dMutation::SetSnapshot { snapshot } => crate::artifacts::fem3d::diff::diff_set_snapshot(snapshot.clone()),
        }
    }

    fn inverse(&self, snapshot: &Fem3dSnapshot) -> Vec<Self> {
        match self {
            Fem3dMutation::SetNode { node, .. } => match index_of(&snapshot.nodes, &node.id) {
                Some(index) => vec![Fem3dMutation::SetNode { index, node: snapshot.nodes[index].clone() }],
                None => vec![Fem3dMutation::RemoveNode { id: node.id.clone() }],
            },
            Fem3dMutation::RemoveNode { id } => index_of(&snapshot.nodes, id).map(|index| vec![Fem3dMutation::SetNode { index, node: snapshot.nodes[index].clone() }]).unwrap_or_default(),
            Fem3dMutation::SetElement { element, .. } => match index_of(&snapshot.elements, element_id(element)) {
                Some(index) => vec![Fem3dMutation::SetElement { index, element: Box::new(snapshot.elements[index].clone()) }],
                None => vec![Fem3dMutation::RemoveElement { id: element_id(element).to_string() }],
            },
            Fem3dMutation::RemoveElement { id } => index_of(&snapshot.elements, id).map(|index| vec![Fem3dMutation::SetElement { index, element: Box::new(snapshot.elements[index].clone()) }]).unwrap_or_default(),
            Fem3dMutation::SetMaterial { material, .. } => match index_of(&snapshot.materials, &material.id) {
                Some(index) => vec![Fem3dMutation::SetMaterial { index, material: snapshot.materials[index].clone() }],
                None => vec![Fem3dMutation::RemoveMaterial { id: material.id.clone() }],
            },
            Fem3dMutation::RemoveMaterial { id } => index_of(&snapshot.materials, id).map(|index| vec![Fem3dMutation::SetMaterial { index, material: snapshot.materials[index].clone() }]).unwrap_or_default(),
            Fem3dMutation::SetSection { section, .. } => match index_of(&snapshot.sections, &section.id) {
                Some(index) => vec![Fem3dMutation::SetSection { index, section: snapshot.sections[index].clone() }],
                None => vec![Fem3dMutation::RemoveSection { id: section.id.clone() }],
            },
            Fem3dMutation::RemoveSection { id } => index_of(&snapshot.sections, id).map(|index| vec![Fem3dMutation::SetSection { index, section: snapshot.sections[index].clone() }]).unwrap_or_default(),
            Fem3dMutation::SetSupport { support, .. } => match index_of(&snapshot.supports, &support.id) {
                Some(index) => vec![Fem3dMutation::SetSupport { index, support: snapshot.supports[index].clone() }],
                None => vec![Fem3dMutation::RemoveSupport { id: support.id.clone() }],
            },
            Fem3dMutation::RemoveSupport { id } => index_of(&snapshot.supports, id).map(|index| vec![Fem3dMutation::SetSupport { index, support: snapshot.supports[index].clone() }]).unwrap_or_default(),
            Fem3dMutation::SetLoadCase { load_case, .. } => match index_of(&snapshot.load_cases, &load_case.id) {
                Some(index) => vec![Fem3dMutation::SetLoadCase { index, load_case: snapshot.load_cases[index].clone() }],
                None => vec![Fem3dMutation::RemoveLoadCase { id: load_case.id.clone() }],
            },
            Fem3dMutation::RemoveLoadCase { id } => index_of(&snapshot.load_cases, id).map(|index| vec![Fem3dMutation::SetLoadCase { index, load_case: snapshot.load_cases[index].clone() }]).unwrap_or_default(),
            Fem3dMutation::SetSolid { solid, .. } => match index_of(&snapshot.solids, &solid.id) {
                Some(index) => vec![Fem3dMutation::SetSolid { index, solid: snapshot.solids[index].clone() }],
                None => vec![Fem3dMutation::RemoveSolid { id: solid.id.clone() }],
            },
            Fem3dMutation::RemoveSolid { id } => index_of(&snapshot.solids, id).map(|index| vec![Fem3dMutation::SetSolid { index, solid: snapshot.solids[index].clone() }]).unwrap_or_default(),
            Fem3dMutation::SetCombination { combination, .. } => match index_of(&snapshot.combinations, &combination.id) {
                Some(index) => vec![Fem3dMutation::SetCombination { index, combination: snapshot.combinations[index].clone() }],
                None => vec![Fem3dMutation::RemoveCombination { id: combination.id.clone() }],
            },
            Fem3dMutation::RemoveCombination { id } => index_of(&snapshot.combinations, id).map(|index| vec![Fem3dMutation::SetCombination { index, combination: snapshot.combinations[index].clone() }]).unwrap_or_default(),
            Fem3dMutation::SetAnalysisSettings { .. } => vec![Fem3dMutation::SetAnalysisSettings { settings: snapshot.analysis.clone() }],
            Fem3dMutation::SetSnapshot { .. } => vec![Fem3dMutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}
// #endregion 🔖️Operations

pub type Fem3dEnvelope = ArtifactEnvelope<Fem3dSnapshot, Fem3dMutation>;
pub type Fem3dStore = ArtifactStore<Fem3dSnapshot, Fem3dMutation>;

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::fem3d::FemDof;
    use protocol::MutationDiff;
    use std::collections::BTreeMap;

    // #region 🔖️Fixtures
    fn cantilever_fixture() -> (Fem3dSnapshot, f64, f64, f64, f64, f64) {
        let e = 210e9;
        let g = 80.77e9;
        let a = 0.00538;
        let iy = 0.0000369;
        let iz = 0.0000133;
        let j = 0.00000060;
        let l = 3.0;
        let p = 5000.0;
        let doc = Fem3dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: l, y: 0.0, z: 0.0 }],
            elements: vec![FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e, g, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "hea200".into(), name: "HEA200".into(), area: a, iy, iz, j }],
            solids: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() }],
            load_cases: vec![FemLoadCase { id: "point".into(), name: "Point Load".into(), loads: vec![crate::artifacts::fem3d::FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -p }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        };
        (doc, e, iy, l, p, iz)
    }

    /// 🧱️ A 2m x 1m x 0.5m slab footprint at the origin, meshed at `mesh_size`, with all 4 footprint
    /// corners as pre-placed document nodes fully fixed in translation (`Tet4` has no rotational DOF) —
    /// mirrors `fem_2d`'s `rectangle_region_doc` fixture pattern for `FemSolid`.
    fn solid_slab_doc() -> Fem3dSnapshot {
        Fem3dSnapshot {
            nodes: vec![FemNode { id: "sc0".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "sc1".into(), x: 2.0, y: 0.0, z: 0.0 }, FemNode { id: "sc2".into(), x: 2.0, y: 1.0, z: 0.0 }, FemNode { id: "sc3".into(), x: 0.0, y: 1.0, z: 0.0 }],
            elements: vec![],
            materials: vec![FemMaterial { id: "concrete".into(), name: "Concrete".into(), e: 30e9, g: 12.5e9, nu: 0.2, rho: 2400.0 }],
            sections: vec![],
            solids: vec![FemSolid { id: "sol1".into(), name: "Slab".into(), outline: vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]], holes: vec![], base_z: 0.0, height: 0.5, layers: 1, mesh_size: 1.0, material_id: "concrete".into() }],
            supports: vec![
                FemSupport { id: "s1".into(), node_id: "sc0".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s2".into(), node_id: "sc1".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s3".into(), node_id: "sc2".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s4".into(), node_id: "sc3".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
            ],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "Self Weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }
    // #endregion 🔖️Fixtures

    // #region 🔖️OpRoundTrip
    fn round_trip(snapshot: &Fem3dSnapshot, operation: &Fem3dMutation) -> Fem3dSnapshot {
        let forward = vcs::apply_mutation(snapshot, operation);
        let mut restored = forward.clone();
        for back in operation.inverse(snapshot) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, snapshot, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn node_set_and_remove_round_trip() {
        let base = Fem3dSnapshot::default();
        let node = FemNode { id: "n1".into(), x: 1.0, y: 2.0, z: 3.0 };
        let after_set = round_trip(&base, &Fem3dMutation::SetNode { index: 0, node: node.clone() });
        assert_eq!(after_set.nodes, vec![node.clone()]);
        round_trip(&after_set, &Fem3dMutation::RemoveNode { id: node.id });
    }

    #[test]
    fn element_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let updated = FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.5 };
        let after_set = round_trip(&base, &Fem3dMutation::SetElement { index: 0, element: Box::new(updated) });
        round_trip(&after_set, &Fem3dMutation::RemoveElement { id: "e1".into() });
    }

    #[test]
    fn material_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let material = FemMaterial { id: "steel".into(), name: "Steel Updated".into(), e: 200e9, g: 79e9, nu: 0.3, rho: 7900.0 };
        let after_set = round_trip(&base, &Fem3dMutation::SetMaterial { index: 0, material });
        round_trip(&after_set, &Fem3dMutation::RemoveMaterial { id: "steel".into() });
    }

    #[test]
    fn section_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let section = FemSection { id: "hea200".into(), name: "HEA200 Updated".into(), area: 0.006, iy: 4e-5, iz: 1.5e-5, j: 7e-7 };
        let after_set = round_trip(&base, &Fem3dMutation::SetSection { index: 0, section });
        round_trip(&after_set, &Fem3dMutation::RemoveSection { id: "hea200".into() });
    }

    #[test]
    fn support_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let support = FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] };
        let after_set = round_trip(&base, &Fem3dMutation::SetSupport { index: 0, support });
        round_trip(&after_set, &Fem3dMutation::RemoveSupport { id: "s1".into() });
    }

    #[test]
    fn load_case_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let load_case = FemLoadCase { id: "point".into(), name: "Point Load Updated".into(), loads: vec![crate::artifacts::fem3d::FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -9000.0 }], self_weight: false };
        let after_set = round_trip(&base, &Fem3dMutation::SetLoadCase { index: 0, load_case });
        round_trip(&after_set, &Fem3dMutation::RemoveLoadCase { id: "point".into() });
    }

    #[test]
    fn combination_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let combination = FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35)]) };
        let after_set = round_trip(&base, &Fem3dMutation::SetCombination { index: 0, combination });
        round_trip(&after_set, &Fem3dMutation::RemoveCombination { id: "uls".into() });
    }

    #[test]
    fn analysis_settings_set_round_trips() {
        let base = Fem3dSnapshot::default();
        let settings = FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 25.0 };
        round_trip(&base, &Fem3dMutation::SetAnalysisSettings { settings });
    }

    #[test]
    fn document_op_round_trips() {
        let (base, ..) = cantilever_fixture();
        let replacement = solid_slab_doc();
        let after = round_trip(&base, &Fem3dMutation::SetSnapshot { snapshot: replacement.clone() });
        assert_eq!(after, replacement);
    }

    #[test]
    fn document_diff_absorb_wins_over_granular_changes() {
        let (base, ..) = cantilever_fixture();
        let replacement = solid_slab_doc();
        let mut diff = Fem3dMutation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 1, buckling_count: 1, deformation_scale: 1.0 } }.diff(&base);
        diff.absorb(Fem3dMutation::SetSnapshot { snapshot: replacement.clone() }.diff(&base));
        assert_eq!(diff.apply(&base), replacement);
    }

    #[test]
    fn solid_op_round_trips() {
        let base = solid_slab_doc();
        let updated = FemSolid { id: "sol1".into(), name: "Slab Updated".into(), outline: base.solids[0].outline.clone(), holes: vec![], base_z: 0.0, height: 0.8, layers: 2, mesh_size: 0.5, material_id: "concrete".into() };
        let after_set = round_trip(&base, &Fem3dMutation::SetSolid { index: 0, solid: updated });
        assert_eq!(after_set.solids[0].height, 0.8);
        round_trip(&after_set, &Fem3dMutation::RemoveSolid { id: "sol1".into() });
    }
    // #endregion 🔖️OpRoundTrip

    // #region 🔖️OpText
    #[test]
    fn fem3d_op_text_round_trips_every_variant() {
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::SetNode { index: 0, node: FemNode { id: "n1".into(), x: 1.0, y: 2.0, z: 3.0 } });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::RemoveNode { id: "n1".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::SetElement {
            index: 0,
            element: Box::new(FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.5 }),
        });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::SetElement { index: 0, element: Box::new(FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }) });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::RemoveElement { id: "e1".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::SetMaterial { index: 0, material: FemMaterial { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 } });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::RemoveMaterial { id: "steel".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::SetSection { index: 0, section: FemSection { id: "hea200".into(), name: "HEA200".into(), area: 0.00538, iy: 3.69e-5, iz: 1.33e-5, j: 6.0e-7 } });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::RemoveSection { id: "hea200".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::SetSolid {
            index: 0,
            solid: FemSolid {
                id: "sol1".into(),
                name: "Slab".into(),
                outline: vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]],
                holes: vec![vec![[0.5, 0.25], [1.5, 0.25], [1.5, 0.75]]],
                base_z: 0.0,
                height: 0.5,
                layers: 2,
                mesh_size: 0.5,
                material_id: "concrete".into(),
            },
        });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::RemoveSolid { id: "sol1".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::SetSupport { index: 0, support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() } });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::RemoveSupport { id: "s1".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::SetLoadCase {
            index: 0,
            load_case: FemLoadCase {
                id: "point".into(),
                name: "Point Load".into(),
                loads: vec![
                    crate::artifacts::fem3d::FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -5000.0 },
                    crate::artifacts::fem3d::FemLoad::MemberUdl { id: "l2".into(), element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -800.0 },
                    crate::artifacts::fem3d::FemLoad::Area { id: "l3".into(), solid_id: "sol1".into(), pressure: 800.0 },
                ],
                self_weight: true,
            },
        });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::RemoveLoadCase { id: "point".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::SetCombination { index: 0, combination: FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35), ("live".into(), 1.5)]) } });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::RemoveCombination { id: "uls".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        let (cantilever, ..) = cantilever_fixture();
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::SetSnapshot { snapshot: cantilever });
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



pub fn apply_fem3d_mutation(snapshot: &mut Fem3dSnapshot, mutation: &Fem3dMutation) {
    *snapshot = vcs::apply_mutation(snapshot, mutation);
}

pub fn inverse_fem3d_mutation(snapshot: &Fem3dSnapshot, mutation: &Fem3dMutation) -> Vec<Fem3dMutation> {
    mutation.inverse(snapshot)
}
