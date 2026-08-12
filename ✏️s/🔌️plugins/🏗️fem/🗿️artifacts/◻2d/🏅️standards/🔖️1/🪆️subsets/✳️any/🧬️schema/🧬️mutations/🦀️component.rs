//! ⚡️ FEM 2D artifact — semantic mutation dispatch enum + laws (constitutional: op). Every variant
//! wraps exactly one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Fem2dSnapshot, Fem2dMutation>`; `#[derive(dsl::Mutations)]` below
//! generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` for the reference shape.

use crate::artifacts::fem2d::diff::Fem2dDiff;
use crate::artifacts::fem2d::Fem2dSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the fem2d document, derived per
/// `📓️derivation-rules.md` from `Fem2dSnapshot`'s shape (8 id-keyed collections + one inseparable
/// analysis-settings facet). Every generic `Set*`/`Remove*` variant this facet used to carry —
/// including the banned `SetSnapshot` whole-document-replace variant — is gone; whole-document
/// replace is not an in-history mutation at all (routed through `HostEffect::LoadDocument`, see
/// `Fem2dPlayApp::whole_document_operation` returning `None` now and `apps::fem2d::reset_document_effect`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Fem2dSnapshot, diff = Fem2dDiff, schema = "fem.fem2d")]
pub enum Fem2dMutation {
    CreateNode(create_node::mutation::CreateNode),
    DeleteNode(delete_node::mutation::DeleteNode),
    CreateElement(create_element::mutation::CreateElement),
    DeleteElement(delete_element::mutation::DeleteElement),
    ReplaceElement(replace_element::mutation::ReplaceElement),
    CreateMaterial(create_material::mutation::CreateMaterial),
    DeleteMaterial(delete_material::mutation::DeleteMaterial),
    ReplaceMaterial(replace_material::mutation::ReplaceMaterial),
    CreateSection(create_section::mutation::CreateSection),
    DeleteSection(delete_section::mutation::DeleteSection),
    ReplaceSection(replace_section::mutation::ReplaceSection),
    CreateSupport(create_support::mutation::CreateSupport),
    DeleteSupport(delete_support::mutation::DeleteSupport),
    ReplaceSupport(replace_support::mutation::ReplaceSupport),
    CreateRegion(create_region::mutation::CreateRegion),
    DeleteRegion(delete_region::mutation::DeleteRegion),
    ReplaceRegion(replace_region::mutation::ReplaceRegion),
    CreateLoadCase(create_load_case::mutation::CreateLoadCase),
    DeleteLoadCase(delete_load_case::mutation::DeleteLoadCase),
    AddLoad(add_load::mutation::AddLoad),
    RemoveLoad(remove_load::mutation::RemoveLoad),
    ChangeLoadCaseSelfWeight(change_load_case_self_weight::mutation::ChangeLoadCaseSelfWeight),
    CreateCombination(create_combination::mutation::CreateCombination),
    DeleteCombination(delete_combination::mutation::DeleteCombination),
    UpdateAnalysisSettings(update_analysis_settings::mutation::UpdateAnalysisSettings),
}
//#endregion 🔖️Mutations

//#region 🔖️LeafImports
/// 🌉️ Brings every triad leaf's `mutation` submodule into this file's own scope (declared as
/// siblings back in `📦️glue.rs`, not inside this file) — required for the dispatch enum's bare
/// `create_node::mutation::CreateNode`-style variant field paths above to resolve.
use super::add_load;
use super::change_load_case_self_weight;
use super::create_combination;
use super::create_element;
use super::create_load_case;
use super::create_material;
use super::create_node;
use super::create_region;
use super::create_section;
use super::create_support;
use super::delete_combination;
use super::delete_element;
use super::delete_load_case;
use super::delete_material;
use super::delete_node;
use super::delete_region;
use super::delete_section;
use super::delete_support;
use super::remove_load;
use super::replace_element;
use super::replace_material;
use super::replace_region;
use super::replace_section;
use super::replace_support;
use super::update_analysis_settings;
//#endregion 🔖️LeafImports

pub type Fem2dEnvelope = ArtifactEnvelope<Fem2dSnapshot, Fem2dMutation>;
pub type Fem2dStore = ArtifactStore<Fem2dSnapshot, Fem2dMutation>;

//#region 🔖️GenericDelegates
/// 🌉️ Thin delegates to the derive-generated `protocol::Mutation` impl — kept because
/// `🏗️builder/🦀️component.rs` (an artifact-generic caller, no per-variant knowledge) and
/// `📝️text/🦀️component.rs`'s re-export both call these by name.
pub fn apply_fem2d_mutation(snapshot: &mut Fem2dSnapshot, mutation: &Fem2dMutation) {
    *snapshot = vcs::apply_mutation(snapshot, mutation);
}

pub fn inverse_fem2d_mutation(snapshot: &Fem2dSnapshot, mutation: &Fem2dMutation) -> Vec<Fem2dMutation> {
    mutation.inverse(snapshot)
}
//#endregion 🔖️GenericDelegates

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::SemanticMutation;
    use crate::artifacts::fem2d::{element_id, load_id, FemAnalysisSettings, FemCombination, FemCombinationTerm, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
    use protocol::MutationDiff;

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
    // #endregion 🔖️Fixtures

    // #region 🔖️OpRoundTrip
    fn round_trip(snapshot: &Fem2dSnapshot, operation: &Fem2dMutation) -> Fem2dSnapshot {
        let forward = vcs::apply_mutation(snapshot, operation);
        let mut restored = forward.clone();
        for back in operation.inverse(snapshot) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, snapshot, "inverse() must restore the pre-mutation document");
        forward
    }

    #[test]
    fn node_create_and_delete_round_trip() {
        let base = Fem2dSnapshot::default();
        let node = FemNode { id: "n1".into(), x: 1.0, y: 2.0 };
        let after_create = round_trip(&base, &Fem2dMutation::CreateNode(create_node::mutation::CreateNode { node: node.clone() }));
        assert_eq!(after_create.nodes, vec![node.clone()]);
        round_trip(&after_create, &Fem2dMutation::DeleteNode(delete_node::mutation::DeleteNode { id: node.id }));
    }

    #[test]
    fn element_create_replace_and_delete_round_trip() {
        let base = simply_supported_beam_doc();
        let updated = FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() };
        let after_replace = round_trip(&base, &Fem2dMutation::ReplaceElement(replace_element::mutation::ReplaceElement { id: "e1".into(), new_element: Box::new(updated) }));
        assert_eq!(element_id(&after_replace.elements[0]), "e1");
        let new_element = FemElement::Bar { id: "e2".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() };
        let after_create = round_trip(&after_replace, &Fem2dMutation::CreateElement(create_element::mutation::CreateElement { element: Box::new(new_element) }));
        round_trip(&after_create, &Fem2dMutation::DeleteElement(delete_element::mutation::DeleteElement { id: "e2".into() }));
    }

    #[test]
    fn material_create_replace_and_delete_round_trip() {
        let base = simply_supported_beam_doc();
        let replaced = FemMaterial { id: "steel".into(), name: "Steel Updated".into(), e: 200e9, nu: 0.3, rho: 7900.0 };
        let after_replace = round_trip(&base, &Fem2dMutation::ReplaceMaterial(replace_material::mutation::ReplaceMaterial { id: "steel".into(), new_material: replaced }));
        round_trip(&after_replace, &Fem2dMutation::DeleteMaterial(delete_material::mutation::DeleteMaterial { id: "steel".into() }));
    }

    #[test]
    fn section_create_replace_and_delete_round_trip() {
        let base = simply_supported_beam_doc();
        let replaced = FemSection { id: "ipe300".into(), name: "IPE300 Updated".into(), area: 0.01, iy: 1e-4 };
        let after_replace = round_trip(&base, &Fem2dMutation::ReplaceSection(replace_section::mutation::ReplaceSection { id: "ipe300".into(), new_section: replaced }));
        round_trip(&after_replace, &Fem2dMutation::DeleteSection(delete_section::mutation::DeleteSection { id: "ipe300".into() }));
    }

    #[test]
    fn support_create_replace_and_delete_round_trip() {
        let base = simply_supported_beam_doc();
        let replaced = FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Ty] };
        round_trip(&base, &Fem2dMutation::ReplaceSupport(replace_support::mutation::ReplaceSupport { id: "s1".into(), new_support: replaced }));
        // 🧮️ Deletes the LAST support: `apply_delta`'s `added` handling (`↩️inverse` recreates via
        // `create-support`, which has no `index` field) re-appends at the end of the collection, so
        // only a last-position delete round-trips to a byte-identical vec order — id-keyed collections
        // with no display order (📓️derivation-rules.md rule 2) don't guarantee position preservation
        // for a non-last delete+recreate, matching `create_support`'s fem3d sibling precedent.
        round_trip(&base, &Fem2dMutation::DeleteSupport(delete_support::mutation::DeleteSupport { id: "s2".into() }));
    }

    #[test]
    fn region_create_replace_and_delete_round_trip() {
        let base = rectangle_region_doc();
        let updated = FemRegion { id: "r1".into(), name: "slab v2".into(), outline: vec![[0.0, 0.0], [5.0, 0.0], [5.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.03, material_id: "steel".into(), mesh_size: 0.5 };
        let after_replace = round_trip(&base, &Fem2dMutation::ReplaceRegion(replace_region::mutation::ReplaceRegion { id: "r1".into(), new_region: updated }));
        assert_eq!(after_replace.regions[0].thickness, 0.03);
        round_trip(&after_replace, &Fem2dMutation::DeleteRegion(delete_region::mutation::DeleteRegion { id: "r1".into() }));
    }

    #[test]
    fn load_case_create_and_delete_round_trip() {
        let base = simply_supported_beam_doc();
        let load_case = FemLoadCase { id: "wind".into(), name: "Wind Load".into(), loads: vec![], self_weight: false };
        let after_create = round_trip(&base, &Fem2dMutation::CreateLoadCase(create_load_case::mutation::CreateLoadCase { load_case }));
        round_trip(&after_create, &Fem2dMutation::DeleteLoadCase(delete_load_case::mutation::DeleteLoadCase { id: "wind".into() }));
    }

    #[test]
    fn add_load_and_remove_load_round_trip() {
        let base = simply_supported_beam_doc();
        let load = FemLoad::MemberUdl { id: "l2".into(), element_id: "e1".into(), wx: 0.0, wy: -900.0 };
        let after_add = round_trip(&base, &Fem2dMutation::AddLoad(add_load::mutation::AddLoad { case_id: "dead".into(), load: Box::new(load.clone()) }));
        assert_eq!(after_add.load_cases[0].loads.len(), 2);
        round_trip(&after_add, &Fem2dMutation::RemoveLoad(remove_load::mutation::RemoveLoad { case_id: "dead".into(), load_id: load_id(&load).to_string() }));
    }

    #[test]
    fn change_load_case_self_weight_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::mutation::ChangeLoadCaseSelfWeight { case_id: "dead".into(), new_self_weight: true }));
    }

    #[test]
    fn combination_create_and_delete_round_trip() {
        let mut base = simply_supported_beam_doc();
        base.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }] });
        let combination = FemCombination { id: "sls".into(), name: "SLS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.0 }] };
        let after_create = round_trip(&base, &Fem2dMutation::CreateCombination(create_combination::mutation::CreateCombination { combination }));
        round_trip(&after_create, &Fem2dMutation::DeleteCombination(delete_combination::mutation::DeleteCombination { id: "sls".into() }));
    }

    #[test]
    fn analysis_settings_update_round_trips() {
        let base = simply_supported_beam_doc();
        let settings = FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 25.0 };
        round_trip(&base, &Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::mutation::UpdateAnalysisSettings { settings }));
    }

    #[test]
    fn missing_target_inverse_and_diff_are_no_ops() {
        let base = Fem2dSnapshot::default();
        assert!(Fem2dMutation::DeleteNode(delete_node::mutation::DeleteNode { id: "ghost".into() }).inverse(&base).is_empty());
        assert!(Fem2dMutation::ReplaceMaterial(replace_material::mutation::ReplaceMaterial { id: "ghost".into(), new_material: FemMaterial { id: "ghost".into(), name: "x".into(), e: 1.0, nu: 0.3, rho: 1.0 } }).inverse(&base).is_empty());
        assert!(Fem2dMutation::RemoveLoad(remove_load::mutation::RemoveLoad { case_id: "ghost".into(), load_id: "ghost".into() }).inverse(&base).is_empty());
        assert_eq!(Fem2dMutation::AddLoad(add_load::mutation::AddLoad { case_id: "ghost".into(), load: Box::new(FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: FemDof::Ty, value: 1.0 }) }).diff(&base), Fem2dDiff::default());
    }
    // #endregion 🔖️OpRoundTrip

    // #region 🔖️OpText
    #[test]
    fn fem2d_op_text_round_trips_every_variant() {
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateNode(create_node::mutation::CreateNode { node: FemNode { id: "n1".into(), x: 1.0, y: 2.0 } }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteNode(delete_node::mutation::DeleteNode { id: "n1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateElement(create_element::mutation::CreateElement { element: Box::new(FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }) }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::ReplaceElement(replace_element::mutation::ReplaceElement { id: "e1".into(), new_element: Box::new(FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }) }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteElement(delete_element::mutation::DeleteElement { id: "e1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateMaterial(create_material::mutation::CreateMaterial { material: FemMaterial { id: "steel".into(), name: "Steel S235".into(), e: 210e9, nu: 0.3, rho: 7850.0 } }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::ReplaceMaterial(replace_material::mutation::ReplaceMaterial { id: "steel".into(), new_material: FemMaterial { id: "steel".into(), name: "Steel S235".into(), e: 210e9, nu: 0.3, rho: 7850.0 } }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteMaterial(delete_material::mutation::DeleteMaterial { id: "steel".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateSection(create_section::mutation::CreateSection { section: FemSection { id: "ipe300".into(), name: "IPE 300".into(), area: 0.005381, iy: 8.356e-5 } }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::ReplaceSection(replace_section::mutation::ReplaceSection { id: "ipe300".into(), new_section: FemSection { id: "ipe300".into(), name: "IPE 300".into(), area: 0.005381, iy: 8.356e-5 } }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteSection(delete_section::mutation::DeleteSection { id: "ipe300".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateSupport(create_support::mutation::CreateSupport { support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] } }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteSupport(delete_support::mutation::DeleteSupport { id: "s1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateRegion(create_region::mutation::CreateRegion {
            region: FemRegion { id: "r1".into(), name: "Slab".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![vec![[1.0, 1.0], [2.0, 1.0], [2.0, 1.5]]], thickness: 0.02, material_id: "steel".into(), mesh_size: 0.5 },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteRegion(delete_region::mutation::DeleteRegion { id: "r1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateLoadCase(create_load_case::mutation::CreateLoadCase {
            load_case: FemLoadCase {
                id: "dead".into(),
                name: "Dead Load".into(),
                loads: vec![
                    FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: FemDof::Ty, value: -1000.0 },
                    FemLoad::MemberUdl { id: "l2".into(), element_id: "e1".into(), wx: 0.0, wy: -5000.0 },
                    FemLoad::Area { id: "l3".into(), region_id: "r1".into(), pressure: 800.0 },
                ],
                self_weight: true,
            },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteLoadCase(delete_load_case::mutation::DeleteLoadCase { id: "dead".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::AddLoad(add_load::mutation::AddLoad { case_id: "dead".into(), load: Box::new(FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: FemDof::Ty, value: -1000.0 }) }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveLoad(remove_load::mutation::RemoveLoad { case_id: "dead".into(), load_id: "l1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::mutation::ChangeLoadCaseSelfWeight { case_id: "dead".into(), new_self_weight: true }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateCombination(create_combination::mutation::CreateCombination {
            combination: FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteCombination(delete_combination::mutation::DeleteCombination { id: "uls".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::mutation::UpdateAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } }));
    }
    // #endregion 🔖️OpText

    // #region 🔖️MutationLaws
    #[test]
    fn mutation_law_create_node_inverse_and_diff_absorb() {
        let base = Fem2dSnapshot::default();
        let mutation = Fem2dMutation::CreateNode(create_node::mutation::CreateNode { node: FemNode { id: "n1".into(), x: 1.0, y: 2.0 } });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let after = d1.apply(&base);
        let d2 = Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::mutation::ChangeLoadCaseSelfWeight { case_id: "none".into(), new_self_weight: true }).diff(&after);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn mutation_law_replace_material_inverse() {
        let base = simply_supported_beam_doc();
        let mutation = Fem2dMutation::ReplaceMaterial(replace_material::mutation::ReplaceMaterial { id: "steel".into(), new_material: FemMaterial { id: "steel".into(), name: "Steel 2".into(), e: 200e9, nu: 0.3, rho: 7900.0 } });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn mutation_law_add_load_inverse_and_diff_absorb() {
        let base = simply_supported_beam_doc();
        let mutation = Fem2dMutation::AddLoad(add_load::mutation::AddLoad { case_id: "dead".into(), load: Box::new(FemLoad::Area { id: "l9".into(), region_id: "r1".into(), pressure: 400.0 }) });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let after = d1.apply(&base);
        let d2 = Fem2dMutation::DeleteCombination(delete_combination::mutation::DeleteCombination { id: "none".into() }).diff(&after);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn every_mutation_registers_a_semantic_descriptor() {
        register_fem2d_mutation_descriptors();
        let kinds = <Fem2dMutation as protocol::SemanticMutation<Fem2dSnapshot>>::kinds();
        assert_eq!(kinds.len(), 25, "every semantic mutation kind must be registered exactly once");
        for descriptor in kinds {
            assert!(protocol::is_approved_verb(descriptor.verb), "verb '{}' must be in APPROVED_VERBS", descriptor.verb);
        }
    }
    // #endregion 🔖️MutationLaws
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
