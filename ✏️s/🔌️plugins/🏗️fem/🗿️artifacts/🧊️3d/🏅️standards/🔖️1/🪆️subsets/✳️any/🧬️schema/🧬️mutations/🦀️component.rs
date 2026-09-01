//! ⚡️ FEM 3D artifact — semantic mutation dispatch enum + laws (constitutional: op). Every variant
//! wraps exactly one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Fem3dSnapshot, Fem3dMutation>`; `#[derive(dsl::Mutations)]` below
//! generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` for the reference shape.

use crate::artifacts::fem3d::diff::Fem3dDiff;
use crate::artifacts::fem3d::Fem3dSnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the fem3d document, derived per
/// `📓️derivation-rules.md` from `Fem3dSnapshot`'s shape (8 id-keyed collections + one inseparable
/// analysis-settings facet). Every generic `Set*`/`Remove*` variant this facet used to carry —
/// including the banned `SetSnapshot` whole-document-replace variant — is gone; whole-document
/// replace is not an in-history mutation at all (routed through `ArtifactStore::reset` /
/// `Effect::LoadDocument`, see `Fem3dPlayApp::whole_document_operation` returning `None` now and
/// `editor::fem3d::reset_document_effect`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Fem3dSnapshot, diff = Fem3dDiff, schema = "fem.fem3d")]
pub enum Fem3dMutation {
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
    CreateSolid(create_solid::mutation::CreateSolid),
    DeleteSolid(delete_solid::mutation::DeleteSolid),
    ReplaceSolid(replace_solid::mutation::ReplaceSolid),
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
use super::create_section;
use super::create_solid;
use super::create_support;
use super::delete_combination;
use super::delete_element;
use super::delete_load_case;
use super::delete_material;
use super::delete_node;
use super::delete_section;
use super::delete_solid;
use super::delete_support;
use super::remove_load;
use super::replace_element;
use super::replace_material;
use super::replace_section;
use super::replace_solid;
use super::replace_support;
use super::update_analysis_settings;
//#endregion 🔖️LeafImports

pub type Fem3dEnvelope = ArtifactEnvelope<Fem3dSnapshot, Fem3dMutation>;
pub type Fem3dStore = ArtifactStore<Fem3dSnapshot, Fem3dMutation>;

//#region 🔖️GenericDelegates
/// 🌉️ Thin delegates to the derive-generated `protocol::Mutation` impl — kept because
/// `🏗️builder/🦀️component.rs` (an artifact-generic caller, no per-variant knowledge) and
/// `📝️text/🦀️component.rs`'s re-export both call these by name.
pub fn apply_fem3d_mutation(snapshot: &mut Fem3dSnapshot, mutation: &Fem3dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = semio_framework_plugin::resolve_ready(vcs::apply_mutation(snapshot, mutation))?;

    *snapshot = next;
    Ok(())
}

pub fn inverse_fem3d_mutation(snapshot: &Fem3dSnapshot, mutation: &Fem3dMutation) -> Vec<Fem3dMutation> {
    mutation.inverse(snapshot)
}
//#endregion 🔖️GenericDelegates

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::fem3d::{element_id, load_id, FemAnalysisSettings, FemCombination, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
    use protocol::MutationDiff;
    use protocol::SemanticMutation;
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
            load_cases: vec![FemLoadCase { id: "point".into(), name: "Point Load".into(), loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -p }], self_weight: false }],
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
        let forward = semio_framework_plugin::resolve_ready(vcs::apply_mutation(snapshot, operation)).expect("valid mutation").0;
        let mut restored = forward.clone();
        for back in operation.inverse(snapshot) {
            restored = semio_framework_plugin::resolve_ready(vcs::apply_mutation(&restored, &back)).expect("valid inverse mutation").0;
        }
        assert_eq!(&restored, snapshot, "inverse() must restore the pre-mutation document");
        forward
    }

    #[semio_framework_async_macros::async_test]
    async fn node_create_and_delete_round_trip() {
        let base = Fem3dSnapshot::default();
        let node = FemNode { id: "n1".into(), x: 1.0, y: 2.0, z: 3.0 };
        let after_create = round_trip(&base, &Fem3dMutation::CreateNode(create_node::mutation::CreateNode { node: node.clone() }));
        assert_eq!(after_create.nodes, vec![node.clone()]);
        round_trip(&after_create, &Fem3dMutation::DeleteNode(delete_node::mutation::DeleteNode { id: node.id }));
    }

    #[semio_framework_async_macros::async_test]
    async fn element_create_replace_and_delete_round_trip() {
        let (base, ..) = cantilever_fixture();
        let updated = FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.5 };
        let after_replace = round_trip(&base, &Fem3dMutation::ReplaceElement(replace_element::mutation::ReplaceElement { id: "e1".into(), new_element: Box::new(updated) }));
        assert_eq!(element_id(&after_replace.elements[0]), "e1");
        let new_element = FemElement::Bar { id: "e2".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into() };
        let after_create = round_trip(&after_replace, &Fem3dMutation::CreateElement(create_element::mutation::CreateElement { element: Box::new(new_element) }));
        round_trip(&after_create, &Fem3dMutation::DeleteElement(delete_element::mutation::DeleteElement { id: "e2".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn material_create_replace_and_delete_round_trip() {
        let (base, ..) = cantilever_fixture();
        let replaced = FemMaterial { id: "steel".into(), name: "Steel Updated".into(), e: 200e9, g: 79e9, nu: 0.3, rho: 7900.0 };
        let after_replace = round_trip(&base, &Fem3dMutation::ReplaceMaterial(replace_material::mutation::ReplaceMaterial { id: "steel".into(), new_material: replaced }));
        round_trip(&after_replace, &Fem3dMutation::DeleteMaterial(delete_material::mutation::DeleteMaterial { id: "steel".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn section_create_replace_and_delete_round_trip() {
        let (base, ..) = cantilever_fixture();
        let replaced = FemSection { id: "hea200".into(), name: "HEA200 Updated".into(), area: 0.006, iy: 4e-5, iz: 1.5e-5, j: 7e-7 };
        let after_replace = round_trip(&base, &Fem3dMutation::ReplaceSection(replace_section::mutation::ReplaceSection { id: "hea200".into(), new_section: replaced }));
        round_trip(&after_replace, &Fem3dMutation::DeleteSection(delete_section::mutation::DeleteSection { id: "hea200".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn support_create_replace_and_delete_round_trip() {
        let (base, ..) = cantilever_fixture();
        let replaced = FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] };
        let after_replace = round_trip(&base, &Fem3dMutation::ReplaceSupport(replace_support::mutation::ReplaceSupport { id: "s1".into(), new_support: replaced }));
        round_trip(&after_replace, &Fem3dMutation::DeleteSupport(delete_support::mutation::DeleteSupport { id: "s1".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn load_case_create_and_delete_round_trip() {
        let (base, ..) = cantilever_fixture();
        let load_case = FemLoadCase { id: "wind".into(), name: "Wind Load".into(), loads: vec![], self_weight: false };
        let after_create = round_trip(&base, &Fem3dMutation::CreateLoadCase(create_load_case::mutation::CreateLoadCase { load_case }));
        round_trip(&after_create, &Fem3dMutation::DeleteLoadCase(delete_load_case::mutation::DeleteLoadCase { id: "wind".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_load_and_remove_load_round_trip() {
        let (base, ..) = cantilever_fixture();
        let load = FemLoad::MemberUdl { id: "l2".into(), element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -900.0 };
        let after_add = round_trip(&base, &Fem3dMutation::AddLoad(add_load::mutation::AddLoad { case_id: "point".into(), load: Box::new(load.clone()) }));
        assert_eq!(after_add.load_cases[0].loads.len(), 2);
        round_trip(&after_add, &Fem3dMutation::RemoveLoad(remove_load::mutation::RemoveLoad { case_id: "point".into(), load_id: load_id(&load).to_string() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_load_case_self_weight_round_trips() {
        let (base, ..) = cantilever_fixture();
        round_trip(&base, &Fem3dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::mutation::ChangeLoadCaseSelfWeight { case_id: "point".into(), new_self_weight: true }));
    }

    #[semio_framework_async_macros::async_test]
    async fn combination_create_and_delete_round_trip() {
        let (base, ..) = cantilever_fixture();
        let combination = FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35)]) };
        let after_create = round_trip(&base, &Fem3dMutation::CreateCombination(create_combination::mutation::CreateCombination { combination }));
        round_trip(&after_create, &Fem3dMutation::DeleteCombination(delete_combination::mutation::DeleteCombination { id: "uls".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn analysis_settings_update_round_trips() {
        let base = Fem3dSnapshot::default();
        let settings = FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 25.0 };
        round_trip(&base, &Fem3dMutation::UpdateAnalysisSettings(update_analysis_settings::mutation::UpdateAnalysisSettings { settings }));
    }

    #[semio_framework_async_macros::async_test]
    async fn solid_create_replace_and_delete_round_trip() {
        let base = solid_slab_doc();
        let updated = FemSolid { id: "sol1".into(), name: "Slab Updated".into(), outline: base.solids[0].outline.clone(), holes: vec![], base_z: 0.0, height: 0.8, layers: 2, mesh_size: 0.5, material_id: "concrete".into() };
        let after_replace = round_trip(&base, &Fem3dMutation::ReplaceSolid(replace_solid::mutation::ReplaceSolid { id: "sol1".into(), new_solid: updated }));
        assert_eq!(after_replace.solids[0].height, 0.8);
        round_trip(&after_replace, &Fem3dMutation::DeleteSolid(delete_solid::mutation::DeleteSolid { id: "sol1".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_target_inverse_and_diff_are_no_ops() {
        let base = Fem3dSnapshot::default();
        assert!(Fem3dMutation::DeleteNode(delete_node::mutation::DeleteNode { id: "ghost".into() }).inverse(&base).is_empty());
        assert!(Fem3dMutation::ReplaceMaterial(replace_material::mutation::ReplaceMaterial { id: "ghost".into(), new_material: FemMaterial { id: "ghost".into(), name: "x".into(), e: 1.0, g: 1.0, nu: 0.3, rho: 1.0 } }).inverse(&base).is_empty());
        assert!(Fem3dMutation::RemoveLoad(remove_load::mutation::RemoveLoad { case_id: "ghost".into(), load_id: "ghost".into() }).inverse(&base).is_empty());
        assert_eq!(*Fem3dMutation::AddLoad(add_load::mutation::AddLoad { case_id: "ghost".into(), load: Box::new(FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: FemDof::Tz, value: 1.0 }) }).diff(&base).diff(), Fem3dDiff::default());
    }
    // #endregion 🔖️OpRoundTrip

    // #region 🔖️OpText
    #[semio_framework_async_macros::async_test]
    async fn fem3d_op_text_round_trips_every_variant() {
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::CreateNode(create_node::mutation::CreateNode { node: FemNode { id: "n1".into(), x: 1.0, y: 2.0, z: 3.0 } }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::DeleteNode(delete_node::mutation::DeleteNode { id: "n1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::CreateElement(create_element::mutation::CreateElement {
            element: Box::new(FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.5 }),
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::ReplaceElement(replace_element::mutation::ReplaceElement {
            id: "e1".into(),
            new_element: Box::new(FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }),
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::DeleteElement(delete_element::mutation::DeleteElement { id: "e1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::CreateMaterial(create_material::mutation::CreateMaterial {
            material: FemMaterial { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::ReplaceMaterial(replace_material::mutation::ReplaceMaterial {
            id: "steel".into(),
            new_material: FemMaterial { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::DeleteMaterial(delete_material::mutation::DeleteMaterial { id: "steel".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::CreateSection(create_section::mutation::CreateSection {
            section: FemSection { id: "hea200".into(), name: "HEA200".into(), area: 0.00538, iy: 3.69e-5, iz: 1.33e-5, j: 6.0e-7 },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::ReplaceSection(replace_section::mutation::ReplaceSection {
            id: "hea200".into(),
            new_section: FemSection { id: "hea200".into(), name: "HEA200".into(), area: 0.00538, iy: 3.69e-5, iz: 1.33e-5, j: 6.0e-7 },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::DeleteSection(delete_section::mutation::DeleteSection { id: "hea200".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::CreateSolid(create_solid::mutation::CreateSolid {
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
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::DeleteSolid(delete_solid::mutation::DeleteSolid { id: "sol1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::CreateSupport(create_support::mutation::CreateSupport {
            support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::DeleteSupport(delete_support::mutation::DeleteSupport { id: "s1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::CreateLoadCase(create_load_case::mutation::CreateLoadCase {
            load_case: FemLoadCase {
                id: "point".into(),
                name: "Point Load".into(),
                loads: vec![
                    FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -5000.0 },
                    FemLoad::MemberUdl { id: "l2".into(), element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -800.0 },
                    FemLoad::Area { id: "l3".into(), solid_id: "sol1".into(), pressure: 800.0 },
                ],
                self_weight: true,
            },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::DeleteLoadCase(delete_load_case::mutation::DeleteLoadCase { id: "point".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::AddLoad(add_load::mutation::AddLoad {
            case_id: "point".into(),
            load: Box::new(FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -5000.0 }),
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::RemoveLoad(remove_load::mutation::RemoveLoad { case_id: "point".into(), load_id: "l1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::mutation::ChangeLoadCaseSelfWeight { case_id: "point".into(), new_self_weight: true }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::CreateCombination(create_combination::mutation::CreateCombination {
            combination: FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35), ("live".into(), 1.5)]) },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::DeleteCombination(delete_combination::mutation::DeleteCombination { id: "uls".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dMutation::UpdateAnalysisSettings(update_analysis_settings::mutation::UpdateAnalysisSettings {
            settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 },
        }));
    }
    // #endregion 🔖️OpText

    // #region 🔖️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn mutation_law_create_node_inverse_and_diff_absorb() {
        let base = Fem3dSnapshot::default();
        let mutation = Fem3dMutation::CreateNode(create_node::mutation::CreateNode { node: FemNode { id: "n1".into(), x: 1.0, y: 2.0, z: 3.0 } });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
        let d1 = mutation.diff(&base).diff().clone();
        let after = d1.apply(&base).expect("valid mutation diff");
        let d2 = Fem3dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::mutation::ChangeLoadCaseSelfWeight { case_id: "none".into(), new_self_weight: true }).diff(&after).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_law_replace_material_inverse() {
        let (base, ..) = cantilever_fixture();
        let mutation = Fem3dMutation::ReplaceMaterial(replace_material::mutation::ReplaceMaterial { id: "steel".into(), new_material: FemMaterial { id: "steel".into(), name: "Steel 2".into(), e: 200e9, g: 79e9, nu: 0.3, rho: 7900.0 } });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_law_add_load_inverse_and_diff_absorb() {
        let (base, ..) = cantilever_fixture();
        let mutation = Fem3dMutation::AddLoad(add_load::mutation::AddLoad { case_id: "point".into(), load: Box::new(FemLoad::Area { id: "l9".into(), solid_id: "sol1".into(), pressure: 400.0 }) });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
        let d1 = mutation.diff(&base).diff().clone();
        let after = d1.apply(&base).expect("valid mutation diff");
        let d2 = Fem3dMutation::DeleteCombination(delete_combination::mutation::DeleteCombination { id: "none".into() }).diff(&after).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn every_mutation_registers_a_semantic_descriptor() {
        register_fem3d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
        let kinds = <Fem3dMutation as protocol::SemanticMutation<Fem3dSnapshot>>::kinds();
        assert_eq!(kinds.len(), 25, "every semantic mutation kind must be registered exactly once");
        for descriptor in kinds {
            assert!(protocol::is_approved_verb(descriptor.verb), "verb '{}' must be in APPROVED_VERBS", descriptor.verb);
        }
    }
    // #endregion 🔖️MutationLaws

    //#region 🔖️OutcomeLaws
    /// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
    /// one `assert_missing_target_is_error` per verb family this facet implements (create/delete/
    /// replace/add/remove/change), plus one `assert_fatal_never_applies` check for a
    /// create-duplicate-id case.
    #[semio_framework_async_macros::async_test]
    async fn create_support_missing_node_is_error() {
        let base = Fem3dSnapshot::default();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &Fem3dMutation::CreateSupport(create_support::mutation::CreateSupport { support: FemSupport { id: "s1".into(), node_id: "ghost".into(), fixed: vec![] } })).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_node_missing_target_is_error() {
        let base = Fem3dSnapshot::default();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &Fem3dMutation::DeleteNode(delete_node::mutation::DeleteNode { id: "ghost".into() })).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_material_missing_target_is_error() {
        let base = Fem3dSnapshot::default();
        protocol::os_spr::testkit::assert_missing_target_is_error(
            &base,
            &Fem3dMutation::ReplaceMaterial(replace_material::mutation::ReplaceMaterial { id: "ghost".into(), new_material: FemMaterial { id: "ghost".into(), name: "x".into(), e: 1.0, g: 1.0, nu: 0.3, rho: 1.0 } }),
        )
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn add_load_missing_case_is_error() {
        let base = Fem3dSnapshot::default();
        protocol::os_spr::testkit::assert_missing_target_is_error(
            &base,
            &Fem3dMutation::AddLoad(add_load::mutation::AddLoad { case_id: "ghost".into(), load: Box::new(FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: FemDof::Tz, value: 1.0 }) }),
        )
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_load_missing_case_is_error() {
        let base = Fem3dSnapshot::default();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &Fem3dMutation::RemoveLoad(remove_load::mutation::RemoveLoad { case_id: "ghost".into(), load_id: "ghost".into() })).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn change_load_case_self_weight_missing_target_is_error() {
        let base = Fem3dSnapshot::default();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &Fem3dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::mutation::ChangeLoadCaseSelfWeight { case_id: "ghost".into(), new_self_weight: true })).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn create_node_duplicate_id_is_fatal() {
        let (base, ..) = cantilever_fixture();
        let outcome = Fem3dMutation::CreateNode(create_node::mutation::CreateNode { node: FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 } }).diff(&base);
        protocol::os_spr::testkit::assert_fatal_never_applies(&outcome).await;
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }
    //#endregion 🔖️OutcomeLaws
}
// #endregion 🧪️Tests

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `Fem3dMutation` variant, in declaration order — the vocabulary the `fem3d-1-any` mutation catalog
/// (`../../🧪️oracle/🔣️.json`) declares and the `mutate-fem3d-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] = &[
    "create-node",
    "delete-node",
    "create-element",
    "delete-element",
    "replace-element",
    "create-material",
    "delete-material",
    "replace-material",
    "create-section",
    "delete-section",
    "replace-section",
    "create-support",
    "delete-support",
    "replace-support",
    "create-solid",
    "delete-solid",
    "replace-solid",
    "create-load-case",
    "delete-load-case",
    "add-load",
    "remove-load",
    "change-load-case-self-weight",
    "create-combination",
    "delete-combination",
    "update-analysis-settings",
];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no third-party codec or `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `Fem3dMutation` nor
/// `Fem3dSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🧪️oracle/🔣️.json — the catalog and the recorded no-oracle decision.
pub fn fem3d_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<Fem3dSnapshot, String> {
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: Fem3dMutation = dsl::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <Fem3dMutation as Mutation<Fem3dSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <Fem3dMutation as Mutation<Fem3dSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <Fem3dMutation as Mutation<Fem3dSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = dsl::DslValue::object([
    ("base".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&base))),
    ("expectedSnapshot".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&expected))),
    ("snapshot".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&applied))),
    ("diff".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(forward.diff()))),
    ("messages".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(forward.messages()))),
    ("inverseSteps".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&inverse))),
    ("inverseSnapshot".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&undone))),
    ("inverseMessages".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&inverse_messages))),
    ]);
    Ok(dsl::json::to_json_string(&report))
}
//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
mod kinds_conformance {
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of them must appear in the committed oracle
    /// manifest's catalog. The framework never parses Rust, so this is what keeps the declaration
    /// honest in both directions at once.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <Fem3dMutation as protocol::SemanticMutation<Fem3dSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsConformance
