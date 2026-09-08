
use super::*;
use crate::{FemAnalysisSettings, FemCombination, FemCombinationTerm, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport, element_id, load_id};
use protocol::MutationDiff;
use protocol::SemanticMutation;

// #region 🔖️Fixtures
fn simply_supported_beam_doc() -> Fem2dSnapshot {
    Fem2dSnapshot {
        nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 6.0, y: 0.0 }],
        elements: vec![FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
        regions: vec![],
        materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }, FemMaterial { id: "timber_spare".into(), name: "GL24h".into(), e: 11.5e9, nu: 0.3, rho: 420.0 }],
        sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }, FemSection { id: "shs_spare".into(), name: "SHS 100x5".into(), area: 0.00184, iy: 2.79e-6 }],
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
    let forward = vcs::apply_mutation(snapshot, operation).expect("valid mutation").0;
    let mut restored = forward.clone();
    for back in operation.inverse(snapshot) {
        restored = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation").0;
    }
    assert_eq!(&restored, snapshot, "inverse() must restore the pre-mutation document");
    forward
}

#[semio_framework_async_macros::async_test]
async fn node_create_and_delete_round_trip() {
    let base = Fem2dSnapshot::default();
    let node = FemNode { id: "n1".into(), x: 1.0, y: 2.0 };
    let after_create = round_trip(&base, &Fem2dMutation::CreateNode(create_node::CreateNode { node: node.clone() }));
    assert_eq!(after_create.nodes, vec![node.clone()]);
    round_trip(&after_create, &Fem2dMutation::DeleteNode(delete_node::DeleteNode { id: node.id }));
}

#[semio_framework_async_macros::async_test]
async fn element_create_replace_and_delete_round_trip() {
    let base = simply_supported_beam_doc();
    let updated = FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() };
    let after_replace = round_trip(&base, &Fem2dMutation::ReplaceElement(replace_element::ReplaceElement { id: "e1".into(), new_element: Box::new(updated) }));
    assert_eq!(element_id(&after_replace.elements[0]), "e1");
    let new_element = FemElement::Bar { id: "e2".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() };
    let after_create = round_trip(&after_replace, &Fem2dMutation::CreateElement(create_element::CreateElement { element: Box::new(new_element) }));
    round_trip(&after_create, &Fem2dMutation::DeleteElement(delete_element::DeleteElement { id: "e2".into() }));
}

#[semio_framework_async_macros::async_test]
async fn material_create_replace_and_delete_round_trip() {
    let base = simply_supported_beam_doc();
    let replaced = FemMaterial { id: "steel".into(), name: "Steel Updated".into(), e: 200e9, nu: 0.3, rho: 7900.0 };
    let after_replace = round_trip(&base, &Fem2dMutation::ReplaceMaterial(replace_material::ReplaceMaterial { id: "steel".into(), new_material: replaced }));
    // 🔗️ Deletes the TRAILING SPARE, not `steel`: since this ticket's referential-integrity wave a
    // `delete-material` naming a grade seven members still point at is refused with
    // `mutation.target-referenced` (see `guards::material_referrers`), and a trailing record is
    // also the only one whose `create-`-shaped inverse round-trips to a byte-identical vec order.
    round_trip(&after_replace, &Fem2dMutation::DeleteMaterial(delete_material::DeleteMaterial { id: "timber_spare".into() }));
}

#[semio_framework_async_macros::async_test]
async fn section_create_replace_and_delete_round_trip() {
    let base = simply_supported_beam_doc();
    let replaced = FemSection { id: "ipe300".into(), name: "IPE300 Updated".into(), area: 0.01, iy: 1e-4 };
    let after_replace = round_trip(&base, &Fem2dMutation::ReplaceSection(replace_section::ReplaceSection { id: "ipe300".into(), new_section: replaced }));
    // 🔗️ Same reason as the material twin above: `ipe300` is still carried by beam `e1`, so its
    // deletion is now `mutation.target-referenced`; the trailing spare is the deletable one.
    round_trip(&after_replace, &Fem2dMutation::DeleteSection(delete_section::DeleteSection { id: "shs_spare".into() }));
}

#[semio_framework_async_macros::async_test]
async fn support_create_replace_and_delete_round_trip() {
    let base = simply_supported_beam_doc();
    let replaced = FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Ty] };
    round_trip(&base, &Fem2dMutation::ReplaceSupport(replace_support::ReplaceSupport { id: "s1".into(), new_support: replaced }));
    // 🧮️ Deletes the LAST support: `apply_delta`'s `added` handling (`↩️inverse` recreates via
    // `create-support`, which has no `index` field) re-appends at the end of the collection, so
    // only a last-position delete round-trips to a byte-identical vec order — id-keyed collections
    // with no display order (📓️derivation-rules.md rule 2) don't guarantee position preservation
    // for a non-last delete+recreate, matching `create_support`'s fem3d sibling precedent.
    round_trip(&base, &Fem2dMutation::DeleteSupport(delete_support::DeleteSupport { id: "s2".into() }));
}

#[semio_framework_async_macros::async_test]
async fn region_create_replace_and_delete_round_trip() {
    let base = rectangle_region_doc();
    let updated = FemRegion { id: "r1".into(), name: "slab v2".into(), outline: vec![[0.0, 0.0], [5.0, 0.0], [5.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.03, material_id: "steel".into(), mesh_size: 0.5 };
    let after_replace = round_trip(&base, &Fem2dMutation::ReplaceRegion(replace_region::ReplaceRegion { id: "r1".into(), new_region: updated }));
    assert_eq!(after_replace.regions[0].thickness, 0.03);
    round_trip(&after_replace, &Fem2dMutation::DeleteRegion(delete_region::DeleteRegion { id: "r1".into() }));
}

#[semio_framework_async_macros::async_test]
async fn load_case_create_and_delete_round_trip() {
    let base = simply_supported_beam_doc();
    let load_case = FemLoadCase { id: "wind".into(), name: "Wind Load".into(), loads: vec![], self_weight: false };
    let after_create = round_trip(&base, &Fem2dMutation::CreateLoadCase(create_load_case::CreateLoadCase { load_case }));
    round_trip(&after_create, &Fem2dMutation::DeleteLoadCase(delete_load_case::DeleteLoadCase { id: "wind".into() }));
}

#[semio_framework_async_macros::async_test]
async fn add_load_and_remove_load_round_trip() {
    let base = simply_supported_beam_doc();
    let load = FemLoad::MemberUdl { id: "l2".into(), element_id: "e1".into(), wx: 0.0, wy: -900.0 };
    let after_add = round_trip(&base, &Fem2dMutation::AddLoad(add_load::AddLoad { case_id: "dead".into(), load: Box::new(load.clone()) }));
    assert_eq!(after_add.load_cases[0].loads.len(), 2);
    round_trip(&after_add, &Fem2dMutation::RemoveLoad(remove_load::RemoveLoad { case_id: "dead".into(), load_id: load_id(&load).to_string() }));
}

#[semio_framework_async_macros::async_test]
async fn change_load_case_self_weight_round_trips() {
    let base = simply_supported_beam_doc();
    round_trip(&base, &Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: "dead".into(), new_self_weight: true }));
}

#[semio_framework_async_macros::async_test]
async fn combination_create_and_delete_round_trip() {
    let mut base = simply_supported_beam_doc();
    base.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }] });
    let combination = FemCombination { id: "sls".into(), name: "SLS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.0 }] };
    let after_create = round_trip(&base, &Fem2dMutation::CreateCombination(create_combination::CreateCombination { combination }));
    round_trip(&after_create, &Fem2dMutation::DeleteCombination(delete_combination::DeleteCombination { id: "sls".into() }));
}

#[semio_framework_async_macros::async_test]
async fn analysis_settings_update_round_trips() {
    let base = simply_supported_beam_doc();
    let settings = FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 25.0 };
    round_trip(&base, &Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings { settings }));
}

#[semio_framework_async_macros::async_test]
async fn missing_target_inverse_and_diff_are_no_ops() {
    let base = Fem2dSnapshot::default();
    assert!(Fem2dMutation::DeleteNode(delete_node::DeleteNode { id: "ghost".into() }).inverse(&base).is_empty());
    assert!(Fem2dMutation::ReplaceMaterial(replace_material::ReplaceMaterial { id: "ghost".into(), new_material: FemMaterial { id: "ghost".into(), name: "x".into(), e: 1.0, nu: 0.3, rho: 1.0 } }).inverse(&base).is_empty());
    assert!(Fem2dMutation::RemoveLoad(remove_load::RemoveLoad { case_id: "ghost".into(), load_id: "ghost".into() }).inverse(&base).is_empty());
    assert_eq!(*Fem2dMutation::AddLoad(add_load::AddLoad { case_id: "ghost".into(), load: Box::new(FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: FemDof::Ty, value: 1.0 }) }).diff(&base).diff(), Fem2dDiff::default());
}
// #endregion 🔖️OpRoundTrip

// #region 🔖️OpText
#[semio_framework_async_macros::async_test]
async fn fem2d_op_text_round_trips_every_variant() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateNode(create_node::CreateNode { node: FemNode { id: "n1".into(), x: 1.0, y: 2.0 } }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteNode(delete_node::DeleteNode { id: "n1".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateElement(create_element::CreateElement {
        element: Box::new(FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }),
    }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::ReplaceElement(replace_element::ReplaceElement {
        id: "e1".into(),
        new_element: Box::new(FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }),
    }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteElement(delete_element::DeleteElement { id: "e1".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateMaterial(create_material::CreateMaterial {
        material: FemMaterial { id: "steel".into(), name: "Steel S235".into(), e: 210e9, nu: 0.3, rho: 7850.0 },
    }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::ReplaceMaterial(replace_material::ReplaceMaterial {
        id: "steel".into(),
        new_material: FemMaterial { id: "steel".into(), name: "Steel S235".into(), e: 210e9, nu: 0.3, rho: 7850.0 },
    }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteMaterial(delete_material::DeleteMaterial { id: "steel".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateSection(create_section::CreateSection { section: FemSection { id: "ipe300".into(), name: "IPE 300".into(), area: 0.005381, iy: 8.356e-5 } }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::ReplaceSection(replace_section::ReplaceSection {
        id: "ipe300".into(),
        new_section: FemSection { id: "ipe300".into(), name: "IPE 300".into(), area: 0.005381, iy: 8.356e-5 },
    }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteSection(delete_section::DeleteSection { id: "ipe300".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateSupport(create_support::CreateSupport { support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] } }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteSupport(delete_support::DeleteSupport { id: "s1".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateRegion(create_region::CreateRegion {
        region: FemRegion { id: "r1".into(), name: "Slab".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![vec![[1.0, 1.0], [2.0, 1.0], [2.0, 1.5]]], thickness: 0.02, material_id: "steel".into(), mesh_size: 0.5 },
    }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteRegion(delete_region::DeleteRegion { id: "r1".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateLoadCase(create_load_case::CreateLoadCase {
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
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteLoadCase(delete_load_case::DeleteLoadCase { id: "dead".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::AddLoad(add_load::AddLoad {
        case_id: "dead".into(),
        load: Box::new(FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: FemDof::Ty, value: -1000.0 }),
    }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveLoad(remove_load::RemoveLoad { case_id: "dead".into(), load_id: "l1".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: "dead".into(), new_self_weight: true }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateCombination(create_combination::CreateCombination {
        combination: FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] },
    }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteCombination(delete_combination::DeleteCombination { id: "uls".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings {
        settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 },
    }));
}
// #endregion 🔖️OpText

// #region 🔖️MutationLaws
#[semio_framework_async_macros::async_test]
async fn mutation_law_create_node_inverse_and_diff_absorb() {
    let base = Fem2dSnapshot::default();
    let mutation = Fem2dMutation::CreateNode(create_node::CreateNode { node: FemNode { id: "n1".into(), x: 1.0, y: 2.0 } });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let after = d1.apply(&base).expect("valid mutation diff");
    let d2 = Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: "none".into(), new_self_weight: true }).diff(&after).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn mutation_law_replace_material_inverse() {
    let base = simply_supported_beam_doc();
    let mutation = Fem2dMutation::ReplaceMaterial(replace_material::ReplaceMaterial { id: "steel".into(), new_material: FemMaterial { id: "steel".into(), name: "Steel 2".into(), e: 200e9, nu: 0.3, rho: 7900.0 } });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn mutation_law_add_load_inverse_and_diff_absorb() {
    let base = simply_supported_beam_doc();
    // 🔗️ A member UDL on the beam this document actually carries: since this ticket's wave
    // `add-load` resolves the load's own target exactly as `create-load-case` does, so the
    // area pressure over the region `r1` this fixture never had would now be refused.
    let mutation = Fem2dMutation::AddLoad(add_load::AddLoad { case_id: "dead".into(), load: Box::new(FemLoad::MemberUdl { id: "l9".into(), element_id: "e1".into(), wx: 0.0, wy: -400.0 }) });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let after = d1.apply(&base).expect("valid mutation diff");
    let d2 = Fem2dMutation::DeleteCombination(delete_combination::DeleteCombination { id: "none".into() }).diff(&after).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn every_mutation_registers_a_semantic_descriptor() {
    register_fem2d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
    let kinds = <Fem2dMutation as SemanticMutation<Fem2dSnapshot>>::kinds();
    assert_eq!(kinds.len(), 25, "every semantic mutation kind must be registered exactly once");
    for descriptor in kinds {
        assert!(protocol::is_approved_verb(descriptor.verb), "verb '{}' must be in APPROVED_VERBS", descriptor.verb);
    }
}
// #endregion 🔖️MutationLaws

//#region 🔖️OutcomeLaws
/// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
/// one `assert_missing_target_is_error`/Fatal check per verb family this facet implements
/// (create/delete/replace/add/remove/change).
#[semio_framework_async_macros::async_test]
async fn create_node_duplicate_id_is_fatal() {
    let base = simply_supported_beam_doc();
    let existing_id = base.nodes.first().unwrap().id.clone();
    let outcome = Fem2dMutation::CreateNode(create_node::CreateNode { node: FemNode { id: existing_id, x: 0.0, y: 0.0 } }).diff(&base);
    protocol::os_spr::testkit::assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn create_support_missing_node_is_error() {
    let base = Fem2dSnapshot::default();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &Fem2dMutation::CreateSupport(create_support::CreateSupport { support: FemSupport { id: "s1".into(), node_id: "ghost".into(), fixed: vec![] } })).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_node_missing_target_is_error() {
    let base = Fem2dSnapshot::default();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &Fem2dMutation::DeleteNode(delete_node::DeleteNode { id: "ghost".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn replace_material_missing_target_is_error() {
    let base = simply_supported_beam_doc();
    protocol::os_spr::testkit::assert_missing_target_is_error(
        &base,
        &Fem2dMutation::ReplaceMaterial(replace_material::ReplaceMaterial { id: "ghost".into(), new_material: FemMaterial { id: "ghost".into(), name: "x".into(), e: 1.0, nu: 0.3, rho: 1.0 } }),
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn add_load_missing_target_is_error() {
    let base = simply_supported_beam_doc();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &Fem2dMutation::AddLoad(add_load::AddLoad { case_id: "ghost".into(), load: Box::new(FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: FemDof::Ty, value: 1.0 }) })).await;
}

#[semio_framework_async_macros::async_test]
async fn remove_load_missing_target_is_error() {
    let base = simply_supported_beam_doc();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &Fem2dMutation::RemoveLoad(remove_load::RemoveLoad { case_id: "ghost".into(), load_id: "ghost".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn change_load_case_self_weight_missing_target_is_error() {
    let base = simply_supported_beam_doc();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: "ghost".into(), new_self_weight: true })).await;
}
//#endregion 🔖️OutcomeLaws

//#region 🛡️GuardLaws
/// 🧾️ The refusal a guard raised, as `(code, level, target)` — every assertion below reads it.
fn refusal(base: &Fem2dSnapshot, mutation: &Fem2dMutation) -> (String, protocol::Severity, Vec<String>) {
    let outcome = mutation.diff(base);
    assert_eq!(outcome.diff(), &Fem2dDiff::default(), "a refusing diff builder must carry the empty diff, never a half-built delta");
    let messages = outcome.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    (messages[0].code.0.clone(), messages[0].level, messages[0].target.clone())
}

/// 🪪️ A `replace-` may not rename its target — that would orphan every referrer silently.
#[semio_framework_async_macros::async_test]
async fn replace_material_rename_is_id_mismatch() {
    let base = simply_supported_beam_doc();
    let renamed = FemMaterial { id: "steel_v2".into(), name: "Steel S355".into(), e: 210e9, nu: 0.3, rho: 7850.0 };
    let (code, level, target) = refusal(&base, &Fem2dMutation::ReplaceMaterial(replace_material::ReplaceMaterial { id: "steel".into(), new_material: renamed }));
    assert_eq!(code, "mutation.id-mismatch");
    assert_eq!(level, protocol::Severity::Fatal, "a rename through a replace is an identity breach no merge policy may absorb");
    assert_eq!(target, vec!["steel".to_string(), "steel_v2".to_string()], "the diagnostic addresses the selected id first and the impostor second");
}

/// 🔗️ A `delete-` refuses while referrers exist, and names every one of them.
#[semio_framework_async_macros::async_test]
async fn delete_material_still_referenced_is_error() {
    let base = simply_supported_beam_doc();
    let (code, level, target) = refusal(&base, &Fem2dMutation::DeleteMaterial(delete_material::DeleteMaterial { id: "steel".into() }));
    assert_eq!(code, "mutation.target-referenced");
    assert_eq!(level, protocol::Severity::Error, "another base may well have no referrers, so this is an Error, not a Fatal");
    assert_eq!(target, vec!["steel".to_string(), "e1".to_string()], "the target comes first, then every referrer");
}

/// 🔗️ The same law through the load lane: a case a combination still weights cannot leave.
#[semio_framework_async_macros::async_test]
async fn delete_load_case_still_combined_is_error() {
    let mut base = simply_supported_beam_doc();
    base.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }] });
    let (code, _, target) = refusal(&base, &Fem2dMutation::DeleteLoadCase(delete_load_case::DeleteLoadCase { id: "dead".into() }));
    assert_eq!(code, "mutation.target-referenced");
    assert_eq!(target, vec!["dead".to_string(), "uls".to_string()]);
}

/// 🧱️ A Poisson ratio at or above 0.5 makes the plane constitutive matrix singular.
#[semio_framework_async_macros::async_test]
async fn create_material_implausible_poisson_is_fatal() {
    let base = simply_supported_beam_doc();
    let implausible = FemMaterial { id: "rubber".into(), name: "Rubber".into(), e: 1e7, nu: 0.5, rho: 1100.0 };
    let (code, level, target) = refusal(&base, &Fem2dMutation::CreateMaterial(create_material::CreateMaterial { material: implausible }));
    assert_eq!(code, "mutation.invariant");
    assert_eq!(level, protocol::Severity::Fatal, "an inadmissible property is wrong on every base, so no merge policy may absorb it");
    assert_eq!(target, vec!["rubber".to_string()]);
}

/// 📏️ A zero area gives the member zero axial stiffness.
#[semio_framework_async_macros::async_test]
async fn create_section_zero_area_is_fatal() {
    let base = simply_supported_beam_doc();
    let implausible = FemSection { id: "void".into(), name: "Void".into(), area: 0.0, iy: 1e-5 };
    let (code, level, _) = refusal(&base, &Fem2dMutation::CreateSection(create_section::CreateSection { section: implausible }));
    assert_eq!(code, "mutation.invariant");
    assert_eq!(level, protocol::Severity::Fatal);
}

/// 🕳️ A hole must be cut FROM the outline, not float beside it.
#[semio_framework_async_macros::async_test]
async fn create_region_hole_outside_outline_is_fatal() {
    let base = rectangle_region_doc();
    let loose =
        FemRegion { id: "r2".into(), name: "Loose hole".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![vec![[9.0, 9.0], [10.0, 9.0], [10.0, 10.0]]], thickness: 0.02, material_id: "steel".into(), mesh_size: 0.5 };
    let (code, level, target) = refusal(&base, &Fem2dMutation::CreateRegion(create_region::CreateRegion { region: loose }));
    assert_eq!(code, "mutation.invariant");
    assert_eq!(level, protocol::Severity::Fatal);
    assert_eq!(target, vec!["r2".to_string()]);
}

/// 📐️ An outline of two points encloses nothing there is anything to mesh.
#[semio_framework_async_macros::async_test]
async fn create_region_degenerate_outline_is_fatal() {
    let base = rectangle_region_doc();
    let degenerate = FemRegion { id: "r3".into(), name: "Line".into(), outline: vec![[0.0, 0.0], [4.0, 0.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 0.5 };
    let (code, _, _) = refusal(&base, &Fem2dMutation::CreateRegion(create_region::CreateRegion { region: degenerate }));
    assert_eq!(code, "mutation.invariant");
}

/// ⚙️ Asking for zero modes asks the solver for an empty spectrum.
#[semio_framework_async_macros::async_test]
async fn update_analysis_settings_zero_modes_is_fatal() {
    let base = simply_supported_beam_doc();
    let settings = FemAnalysisSettings { modal_count: 0, buckling_count: 3, deformation_scale: 50.0 };
    let (code, level, target) = refusal(&base, &Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings { settings }));
    assert_eq!(code, "mutation.invariant");
    assert_eq!(level, protocol::Severity::Fatal);
    assert!(target.is_empty(), "the analysis facet has no id to address, got {target:?}");
}

/// 🔗️ `add-load` and `create-load-case` are two doors into the same collection, and now agree.
#[semio_framework_async_macros::async_test]
async fn add_load_dangling_node_is_error() {
    let base = simply_supported_beam_doc();
    let load = FemLoad::Nodal { id: "l9".into(), node_id: "ghost".into(), dof: FemDof::Ty, value: -1000.0 };
    let through_add = refusal(&base, &Fem2dMutation::AddLoad(add_load::AddLoad { case_id: "dead".into(), load: Box::new(load.clone()) }));
    let case = FemLoadCase { id: "seismic".into(), name: "Seismic".into(), loads: vec![load], self_weight: false };
    let through_create = refusal(&base, &Fem2dMutation::CreateLoadCase(create_load_case::CreateLoadCase { load_case: case }));
    assert_eq!(through_add, through_create, "the same load on the same missing node must be refused identically through both verbs");
    assert_eq!(through_add.0, "mutation.target-missing");
    assert_eq!(through_add.2, vec!["ghost".to_string()]);
}

/// 🔗️ `replace-element` re-resolves all four foreign keys, exactly as `create-element` does.
#[semio_framework_async_macros::async_test]
async fn replace_element_dangling_section_is_error() {
    let base = simply_supported_beam_doc();
    let dangling = FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ghost".into() };
    let (code, level, target) = refusal(&base, &Fem2dMutation::ReplaceElement(replace_element::ReplaceElement { id: "e1".into(), new_element: Box::new(dangling) }));
    assert_eq!(code, "mutation.target-missing");
    assert_eq!(level, protocol::Severity::Error);
    assert_eq!(target, vec!["ghost".to_string()]);
}
//#endregion 🛡️GuardLaws
