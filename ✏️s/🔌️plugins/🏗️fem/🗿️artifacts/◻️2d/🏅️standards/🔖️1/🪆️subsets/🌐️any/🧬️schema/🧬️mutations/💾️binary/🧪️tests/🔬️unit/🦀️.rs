
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::update_analysis_settings;
use crate::standards::v1::subsets::any::schema;
use crate::{FemAnalysisSettings, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport};
use store::{ArtifactCommand, create_document_envelope};

fn simply_supported_beam_doc() -> crate::Fem2dSnapshot {
    crate::Fem2dSnapshot {
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

#[test]
fn op_binary_round_trips_and_agrees_with_text() {
    let operation = Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
    semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

/// 🧬️ The whole-document `SetSnapshot` mutation that used to seed this store round-trip test is
/// banned outright (`📓️taxonomy.md`'s forbidden vocabulary — no replacement mutation). Builds the
/// same `simply_supported_beam_doc` content through a real sequence of semantic mutations instead,
/// still exercising every collection kind (id-keyed create + a nested load) for the text/pack codecs.
#[semio_framework_async_macros::async_test]
async fn fem2d_document_text_round_trips_through_the_store() {
    let fixture = simply_supported_beam_doc();
    let mut store = semio_framework_plugin::resolve_ready(schema::mutations::Fem2dStore::new(create_document_envelope(crate::FEM_2D_SCHEMA, "fem2d", schema::empty_fem2d_snapshot(), None))).expect("valid store");
    let mutations = vec![
        Fem2dMutation::CreateMaterial(schema::mutations::create_material::CreateMaterial { material: fixture.materials[0].clone() }),
        Fem2dMutation::CreateSection(schema::mutations::create_section::CreateSection { section: fixture.sections[0].clone() }),
        Fem2dMutation::CreateNode(schema::mutations::create_node::CreateNode { node: fixture.nodes[0].clone() }),
        Fem2dMutation::CreateNode(schema::mutations::create_node::CreateNode { node: fixture.nodes[1].clone() }),
        Fem2dMutation::CreateElement(schema::mutations::create_element::CreateElement { element: Box::new(fixture.elements[0].clone()) }),
        Fem2dMutation::CreateSupport(schema::mutations::create_support::CreateSupport { support: fixture.supports[0].clone() }),
        Fem2dMutation::CreateSupport(schema::mutations::create_support::CreateSupport { support: fixture.supports[1].clone() }),
        Fem2dMutation::CreateLoadCase(schema::mutations::create_load_case::CreateLoadCase { load_case: fixture.load_cases[0].clone() }),
    ];
    store.dispatch(ArtifactCommand::Apply { mutations, description: None }).await.expect("apply");
    assert_eq!(store.snapshot().expect("snapshot"), fixture);
    semio_framework_os_kernel::os_store::test_support::assert_document_text_round_trip(&store).await;
    semio_framework_os_kernel::os_store::test_support::assert_document_pack_round_trip(&store).await;
}
