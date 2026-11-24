#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🖼️assets/🌲️concrete-forest/🗣️.dsl.semio")).expect("../../../../🖼️assets/🌲️concrete-forest/🗣️.dsl.semio").leak();
    assert!(text.len() > 8);
}

//#region 🧪️ExampleLaws
use semio_s_artifact_process_process3d_inferences::Process3dInference;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn example_parses_to_the_reference_stock() {
    let snapshot = <Process3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(crate::examples::concrete_forest::PRIMARY_TEXT).expect("concrete forest example parses");
    assert_eq!(snapshot.stock_payload.solid, semio_s_artifact_process_process3d_model::snapshot::WorkingSolid::Reference { reference_id: semio_s_artifact_process_process3d_model::snapshot::REFERENCE_SOLID_CONCRETE_FOREST_LEFT.into() });
    assert_eq!(snapshot.step_payloads.len(), 7);
    assert_eq!(Process3dInference::infer(&snapshot).expect("valid materialized inference fixture"), Process3dInference::infer(&snapshot).expect("valid materialized inference fixture"));
}
//#endregion 🧪️ExampleLaws

//#region 🧪️GenesisChildren
/// 🌱️ Every composed member the example declares is derivable from its inline records — the stock's
/// real 57-face B-Rep, the seven-node timeline flow, and one tool solid per cut/attach step — so a
/// member-less archive load (the react shell's `loadDocumentPair`) closes completely.
#[semio_framework_async_macros::async_test]
async fn every_composed_child_of_the_example_has_a_genesis_pack() {
    use semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot;
    use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
    use pack::snapshot::ArtifactPack;
    let snapshot = <Process3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(crate::examples::concrete_forest::PRIMARY_TEXT).expect("concrete forest example parses");
    let stock = semio_s_artifact_process_process3d_model::snapshot::genesis_process3d_child_pack(&snapshot, "stockSolid", &snapshot.stock_solid.child_id).expect("stock child pack");
    let brep = <SemioBrepSnapshot as ArtifactPack>::decode_pack(&stock).expect("stock brep decodes");
    assert_eq!((brep.faces.len(), brep.edges.len(), brep.vertices.len()), (57, 126, 71));
    let steps = semio_s_artifact_process_process3d_model::snapshot::genesis_process3d_child_pack(&snapshot, "steps", &snapshot.steps.child_id).expect("steps child pack");
    let flow = <SemioFlowSnapshot as ArtifactPack>::decode_pack(&steps).expect("steps flow decodes");
    assert_eq!(flow.nodes.len(), 7);
    assert_eq!(snapshot.tool_solids.len(), 5, "five cut/attach steps mint a tool solid; the two drills mint none");
    for tool in &snapshot.tool_solids {
        let pack = semio_s_artifact_process_process3d_model::snapshot::genesis_process3d_child_pack(&snapshot, "toolSolids", &tool.child_id).unwrap_or_else(|| panic!("tool child pack {}", tool.child_id));
        assert!(<SemioBrepSnapshot as ArtifactPack>::decode_pack(&pack).is_ok_and(|brep| !brep.faces.is_empty()));
    }
    assert!(semio_s_artifact_process_process3d_model::snapshot::genesis_process3d_child_pack(&snapshot, "stockSolid", "not-a-child").is_none());
}
//#endregion 🧪️GenesisChildren

//#region 🧪️ForestBrepSeedBudget
/// 🧮️ Vacant receiver bytes one forest stock brep open seeds: every row of every collection births one native record receiver whose whole vacant image is charged as copy and fuel.
fn forest_brep_seed_bytes(brep: &semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot) -> u64 {
    use semio_framework_dsl_record::RecordValue;
    use semio_framework_value_native_owned::native_decoding::owned::NativeReceive;
    use semio_s_artifact_stdio_semio_brep::schema::snapshot::*;
    fn receiver<T: NativeReceive<RecordValue>>() -> u64 { std::mem::size_of::<<T as NativeReceive<RecordValue>>::Receiver>() as u64 }
    let loop_edges: usize = brep.loops.iter().map(|l| l.edges.len()).sum();
    let shell_faces: usize = brep.shells.iter().map(|s| s.faces.len()).sum();
    brep.vertices.len() as u64 * receiver::<BrepVertex>() + brep.edges.len() as u64 * receiver::<BrepEdge>() + brep.loops.len() as u64 * receiver::<BrepLoop>() + brep.faces.len() as u64 * receiver::<BrepFace>() + brep.coedges.len() as u64 * receiver::<BrepCoedge>() + loop_edges as u64 * receiver::<BrepLoopEdge>() + shell_faces as u64 * receiver::<BrepShellFace>()
}

/// 📸️ The decoded concrete forest stock brep and its pinned budget law.
fn forest_brep_and_law() -> (semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot, u64, serde_json::Value) {
    use semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot;
    use pack::snapshot::ArtifactPack;
    let budget: serde_json::Value = serde_json::from_str(include_str!("🪙️budget.json")).expect("forest brep budget parses");
    let snapshot = <Process3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(crate::examples::concrete_forest::PRIMARY_TEXT).expect("concrete forest example parses");
    let stock = semio_s_artifact_process_process3d_model::snapshot::genesis_process3d_child_pack(&snapshot, "stockSolid", &snapshot.stock_solid.child_id).expect("stock child pack");
    let brep = <SemioBrepSnapshot as ArtifactPack>::decode_pack(&stock).expect("stock brep decodes");
    (brep, stock.len() as u64, budget["forestBrep"].clone())
}

/// 🔨️ LAW: the concrete forest stock brep keeps its pinned row counts and never seeds more receiver bytes than the ceiling measured before its compaction. The budget file is shared with the TS twin.
#[semio_framework_async_macros::async_test]
async fn forest_stock_brep_seed_bytes_never_exceed_their_pinned_ceiling() {
    let (brep, pack_bytes, law) = forest_brep_and_law();
    let n = |key: &str| law["rows"][key].as_u64().expect("row count");
    assert_eq!(pack_bytes, law["packBytes"].as_u64().unwrap());
    assert_eq!((brep.vertices.len() as u64, brep.edges.len() as u64, brep.loops.len() as u64, brep.faces.len() as u64, brep.coedges.len() as u64), (n("vertices"), n("edges"), n("loops"), n("faces"), n("coedges")));
    let seeded = forest_brep_seed_bytes(&brep);
    assert!(seeded <= law["ceilingSeedBytes"].as_u64().unwrap(), "forest stock brep seeds {seeded} receiver bytes above its pinned ceiling");
}

/// 🎯️ LAW: the concrete forest stock brep seeds few enough receiver bytes for every load of a pane smoke to fit the host lifetime fuel. Red until the receiver slot or the brep rows are compacted.
#[semio_framework_async_macros::async_test]
async fn forest_stock_brep_fits_the_host_lifetime_fuel_for_every_pane_load() {
    let (brep, _, law) = forest_brep_and_law();
    let seeded = forest_brep_seed_bytes(&brep);
    let target = law["targetSeedBytes"].as_u64().unwrap();
    assert!(target * law["loadsPerLifetime"].as_u64().unwrap() <= law["hostLifetimeFuel"].as_u64().unwrap());
    assert!(seeded <= target, "forest stock brep seeds {seeded} receiver bytes against a target of {target}");
}
/// 🔨️ LAW: one native receive slot never exceeds its pinned size, whatever child it carries, because a record seeds one slot per non-scalar field. The budget file is shared with the TS twin.
#[test]
fn receive_slot_never_exceeds_its_pinned_size() {
    use semio_framework_value_native_owned::native_decoding::owned::{tree::NativeValueSource, NativeReceiveSlot};
    let budget: serde_json::Value = serde_json::from_str(include_str!("🪙️budget.json")).expect("budget parses");
    let ceiling = budget["receiveSlot"]["ceilingBytes"].as_u64().unwrap() as usize;
    let (string, byte) = (std::mem::size_of::<NativeReceiveSlot<dyn NativeValueSource, String>>(), std::mem::size_of::<NativeReceiveSlot<dyn NativeValueSource, u8>>());
    assert!(string <= ceiling && byte <= ceiling, "receive slot is {string} B (String) and {byte} B (u8) against a ceiling of {ceiling} B");
}
/// 🔨️ LAW: every brep row receiver stays within its pinned size, because a row seeds one whole receiver: its record state, its one shared frame and one slot per distinct field type. The budget file is shared with the TS twin.
#[test]
fn brep_row_receivers_never_exceed_their_pinned_size() {
    use semio_framework_dsl_record::RecordValue;
    use semio_framework_value_native_owned::native_decoding::owned::NativeReceive;
    use semio_s_artifact_stdio_semio_brep::schema::snapshot::*;
    fn receiver<T: NativeReceive<RecordValue>>() -> u64 { std::mem::size_of::<<T as NativeReceive<RecordValue>>::Receiver>() as u64 }
    let budget: serde_json::Value = serde_json::from_str(include_str!("🪙️budget.json")).expect("budget parses");
    let ceilings = &budget["recordReceiver"]["ceilingBytes"];
    let actual = [("BrepVertex", receiver::<BrepVertex>()), ("BrepEdge", receiver::<BrepEdge>()), ("BrepCoedge", receiver::<BrepCoedge>()), ("BrepFace", receiver::<BrepFace>()), ("BrepLoop", receiver::<BrepLoop>()), ("BrepLoopEdge", receiver::<BrepLoopEdge>())];
    let over: Vec<String> = actual.iter().filter(|(name, size)| *size > ceilings[*name].as_u64().unwrap()).map(|(name, size)| format!("{name} {size} B against {}", ceilings[*name])).collect();
    assert!(over.is_empty(), "row receivers above their ceilings: {over:?}");
}
//#endregion 🧪️ForestBrepSeedBudget
#[test]
fn debug_nested_receivers() {
    use semio_framework_dsl_record::RecordValue;
    use semio_framework_value_native_owned::native_decoding::owned::NativeReceive;
    use semio_s_artifact_stdio_semio_brep::schema::snapshot::*;
    use semio_s_artifact_stdio_semio_base::schema::geometry::{SemioPoint2,SemioPoint3};
    fn r<T: NativeReceive<RecordValue>>() -> usize { std::mem::size_of::<<T as NativeReceive<RecordValue>>::Receiver>() }
    eprintln!("[DEBUG] p3={} p2={} curve={} curve2={} surface={} optcurve2={} tuple={} string={} f64={} bool={} vecloopedge={} vecstring={} loopedge={} shellface={} vertex={} edge={} face={}", r::<SemioPoint3>(), r::<SemioPoint2>(), r::<BrepCurve>(), r::<BrepCurve2>(), r::<BrepSurface>(), r::<Option<BrepCurve2>>(), r::<(f64,f64)>(), r::<String>(), r::<f64>(), r::<bool>(), r::<Vec<BrepLoopEdge>>(), r::<Vec<String>>(), r::<BrepLoopEdge>(), r::<BrepShellFace>(), r::<BrepVertex>(), r::<BrepEdge>(), r::<BrepFace>());
}
