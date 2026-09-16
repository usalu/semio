#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🌲️concrete-forest/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️ExampleLaws
use crate::standards::v1::subsets::any::schema::inferences::Process3dInference;
use crate::Process3dSnapshot;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn example_parses_to_the_reference_stock() {
    let snapshot = <Process3dSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::concrete_forest::PRIMARY_TEXT).expect("concrete forest example parses");
    assert_eq!(snapshot.stock_payload.solid, crate::WorkingSolid::Reference { reference_id: crate::REFERENCE_SOLID_CONCRETE_FOREST_LEFT.into() });
    assert_eq!(snapshot.step_payloads.len(), 7);
    assert_eq!(Process3dInference::infer(&snapshot), Process3dInference::infer(&snapshot));
}
//#endregion 🧪️ExampleLaws

//#region 🧪️GenesisChildren
/// 🌱️ Every composed member the example declares is derivable from its inline records — the stock's
/// real 57-face B-Rep, the seven-node timeline flow, and one tool solid per cut/attach step — so a
/// member-less archive load (the react shell's `loadDocumentPair`) closes completely.
#[semio_framework_async_macros::async_test]
async fn every_composed_child_of_the_example_has_a_genesis_pack() {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
    use store::ArtifactPack;
    let snapshot = <Process3dSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::concrete_forest::PRIMARY_TEXT).expect("concrete forest example parses");
    let stock = crate::genesis_process3d_child_pack(&snapshot, "stockSolid", &snapshot.stock_solid.child_id).expect("stock child pack");
    let brep = <SemioBrepSnapshot as ArtifactPack>::decode_pack(&stock).expect("stock brep decodes");
    assert_eq!((brep.faces.len(), brep.edges.len(), brep.vertices.len()), (57, 126, 71));
    let steps = crate::genesis_process3d_child_pack(&snapshot, "steps", &snapshot.steps.child_id).expect("steps child pack");
    let flow = <SemioFlowSnapshot as ArtifactPack>::decode_pack(&steps).expect("steps flow decodes");
    assert_eq!(flow.nodes.len(), 7);
    assert_eq!(snapshot.tool_solids.len(), 5, "five cut/attach steps mint a tool solid; the two drills mint none");
    for tool in &snapshot.tool_solids {
        let pack = crate::genesis_process3d_child_pack(&snapshot, "toolSolids", &tool.child_id).unwrap_or_else(|| panic!("tool child pack {}", tool.child_id));
        assert!(<SemioBrepSnapshot as ArtifactPack>::decode_pack(&pack).is_ok_and(|brep| !brep.faces.is_empty()));
    }
    assert!(crate::genesis_process3d_child_pack(&snapshot, "stockSolid", "not-a-child").is_none());
}
//#endregion 🧪️GenesisChildren
