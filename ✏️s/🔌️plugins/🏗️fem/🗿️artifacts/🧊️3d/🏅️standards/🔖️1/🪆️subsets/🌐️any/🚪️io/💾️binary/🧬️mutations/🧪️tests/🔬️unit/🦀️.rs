use crate::standards::v1::subsets::any::io::binary::mutations::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema;
use semio_s_artifact_fem_3d_analysis::standards::v1::subsets::any::schema::mutations::update_analysis_settings;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_s_fem_material::volumetric::FemMaterialValue;
use semio_s_fem_node::volumetric::FemNodeValue;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSection;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSupport;
use store::os_store::document_materialize::create_document_envelope;
use store::os_store::store_schema::ArtifactCommand;

fn cantilever_fixture() -> semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot {
    semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot {
        nodes: vec![FemNodeValue { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNodeValue { id: "n2".into(), x: 3.0, y: 0.0, z: 0.0 }],
        elements: vec![FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.0 }],
        materials: vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 }],
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
    let operation = Fem3dMutation::UpdateAnalysisSettings(semio_s_artifact_fem_3d_analysis::standards::v1::subsets::any::schema::mutations::update_analysis_settings::UpdateAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
    semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

/// 🩹️ Typed field intent remains sparse through both mutation protocol representations.
#[test]
fn material_field_intent_binary_round_trips_and_agrees_with_text() {
    let operation = Fem3dMutation::ChangeMaterial(schema::mutations::change_material::ChangeMaterial {
        id: "steel".into(),
        patch: schema::diff::Fem3dMaterialPatch { e: Some(190e9), ..Default::default() }
    });
    semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode sparse material field intent");
    assert_eq!(decode_op(&bytes).expect("decode sparse material field intent"), operation);
}

/// 🧬️ The whole-document `SetSnapshot` mutation that used to seed this store round-trip test is
/// banned outright (`📓️taxonomy.md`'s forbidden vocabulary — no replacement mutation). Builds the
/// same `cantilever_fixture` content through a real sequence of semantic mutations instead, still
/// exercising every collection kind (id-keyed create + a nested load) for the text/pack codecs.
#[semio_framework_async_macros::async_test]
async fn fem3d_document_text_round_trips_through_the_store() {
    let fixture = cantilever_fixture();
    let mut store = ::semio_framework_async::poll::resolve_ready(schema::mutations::Fem3dStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(create_document_envelope(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FEM_3D_SCHEMA, "fem3d", schema::empty_fem3d_snapshot(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))).expect("valid store");
    store.install_document_store_owners_exact(semio_framework_os_kernel::os_store::document_store_owners::funded_bounded_artifact_store_owners::<semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot, Fem3dMutation>().expect("funded bounded document owners")).unwrap_or_else(|(error, _)| panic!("bounded document owners install refused: {error}"));
    let mutations = vec![
        Fem3dMutation::CreateMaterial(schema::mutations::create_material::CreateMaterial { material: fixture.materials[0].clone(), index: None }),
        Fem3dMutation::CreateSection(schema::mutations::create_section::CreateSection { section: fixture.sections[0].clone(), index: None }),
        Fem3dMutation::CreateNode(schema::mutations::create_node::CreateNode { node: fixture.nodes[0].clone(), index: None }),
        Fem3dMutation::CreateNode(schema::mutations::create_node::CreateNode { node: fixture.nodes[1].clone(), index: None }),
        Fem3dMutation::CreateElement(schema::mutations::create_element::CreateElement { element: Box::new(fixture.elements[0].clone()), index: None }),
        Fem3dMutation::CreateSupport(schema::mutations::create_support::CreateSupport { support: fixture.supports[0].clone(), index: None }),
        Fem3dMutation::CreateLoadCase(schema::mutations::create_load_case::CreateLoadCase { load_case: fixture.load_cases[0].clone(), index: None }),
    ];
    store.dispatch(ArtifactCommand::Apply { mutations, transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    assert_eq!(store.snapshot().expect("snapshot"), fixture);
    semio_framework_os_kernel::os_store::test_support::assert_document_text_round_trip(&store).await;
    semio_framework_os_kernel::os_store::test_support::assert_document_pack_round_trip(&store).await;
    store.close_owned_unscheduled().expect("fem3d document store closes through its exact bounded owners");
}

/// 📨️ Every field-intent event retains the same authored payload across text and binary history codecs.
#[test]
fn sparse_field_intent_text_binary_equivalence(){
    let vectors=[
        r#"{"mutation":"changeNode","id":"n1","patch":{"x":4}}"#,
        r#"{"mutation":"changeSection","id":"sec1","patch":{"area":0.03}}"#,
        r#"{"mutation":"changeSolid","id":"geometry","patch":{"holes":[]}}"#,
        r#"{"mutation":"changeSupport","id":"s1","patch":{"tx":true}}"#,
        r#"{"mutation":"changeElement","id":"e1","patch":{"kind":"frame"}}"#,
        r#"{"mutation":"changeLoad","caseId":"lc1","id":"l1","patch":{"kind":"nodal","value":20}}"#,
        r#"{"mutation":"changeCombinationName","id":"c1","name":"New"}"#,
        r#"{"mutation":"changeCombinationTerm","id":"c1","caseId":"lc1","factor":2}"#,
        r#"{"mutation":"changeCombinationTerm","id":"c1","caseId":"lc1","factor":null}"#,
    ];
    for wire in vectors{let operation=semio_framework_pack_json::from_json_str::<Fem3dMutation>(wire, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored field event");semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&operation);}
    for wire in[r#"{"mutation":"changeCombinationTerm","id":"c1","caseId":"lc1"}"#,r#"{"mutation":"changeLoad","caseId":"lc1","id":"l1","patch":{"kind":"nodal","value":null}}"#]{assert!(semio_framework_pack_json::from_json_str::<Fem3dMutation>(wire, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());}
    eprintln!("[DEBUG] nine sparse semantic payloads preserve authored fields including empty geometry and nullable term removal through text/binary history codecs");
}