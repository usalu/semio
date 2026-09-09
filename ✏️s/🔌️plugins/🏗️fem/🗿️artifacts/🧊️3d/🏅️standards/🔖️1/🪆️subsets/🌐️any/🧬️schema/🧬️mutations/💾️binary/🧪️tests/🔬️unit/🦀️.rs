use super::*;
use crate::standards::v1::subsets::any::schema;
use crate::standards::v1::subsets::any::schema::mutations::update_analysis_settings;
use crate::{FemAnalysisSettings, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport};
use store::{create_document_envelope, ArtifactCommand};

fn cantilever_fixture() -> crate::Fem3dSnapshot {
    crate::Fem3dSnapshot {
        nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: 3.0, y: 0.0, z: 0.0 }],
        elements: vec![FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.0 }],
        materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "hea200".into(), name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.00000060 }],
        solids: vec![],
        supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() }],
        load_cases: vec![FemLoadCase { id: "point".into(), name: "Point Load".into(), loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -5000.0 }], self_weight: false }],
        combinations: vec![],
        analysis: FemAnalysisSettings::default(),
    }
}

#[test]
fn op_binary_round_trips_and_agrees_with_text() {
    let operation = Fem3dMutation::UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
    semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

/// 🧬️ The whole-document `SetSnapshot` mutation that used to seed this store round-trip test is
/// banned outright (`📓️taxonomy.md`'s forbidden vocabulary — no replacement mutation). Builds the
/// same `cantilever_fixture` content through a real sequence of semantic mutations instead, still
/// exercising every collection kind (id-keyed create + a nested load) for the text/pack codecs.
#[semio_framework_async_macros::async_test]
async fn fem3d_document_text_round_trips_through_the_store() {
    let fixture = cantilever_fixture();
    let mut store = semio_framework_plugin::resolve_ready(schema::mutations::Fem3dStore::new(create_document_envelope(crate::FEM_3D_SCHEMA, "fem3d", schema::empty_fem3d_snapshot(), None))).expect("valid store");
    let mutations = vec![
        Fem3dMutation::CreateMaterial(schema::mutations::create_material::CreateMaterial { material: fixture.materials[0].clone() }),
        Fem3dMutation::CreateSection(schema::mutations::create_section::CreateSection { section: fixture.sections[0].clone() }),
        Fem3dMutation::CreateNode(schema::mutations::create_node::CreateNode { node: fixture.nodes[0].clone() }),
        Fem3dMutation::CreateNode(schema::mutations::create_node::CreateNode { node: fixture.nodes[1].clone() }),
        Fem3dMutation::CreateElement(schema::mutations::create_element::CreateElement { element: Box::new(fixture.elements[0].clone()) }),
        Fem3dMutation::CreateSupport(schema::mutations::create_support::CreateSupport { support: fixture.supports[0].clone() }),
        Fem3dMutation::CreateLoadCase(schema::mutations::create_load_case::CreateLoadCase { load_case: fixture.load_cases[0].clone() }),
    ];
    store.dispatch(ArtifactCommand::Apply { mutations, description: None }).await.expect("apply");
    assert_eq!(store.snapshot().expect("snapshot"), fixture);
    semio_framework_os_kernel::os_store::test_support::assert_document_text_round_trip(&store).await;
    semio_framework_os_kernel::os_store::test_support::assert_document_pack_round_trip(&store).await;
}
