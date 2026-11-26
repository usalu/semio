//! 🧪️ Drives the pinned bitmap route through its original decoder, worker and retirement.

use super::*;
use crate::host::inferences::{BitmapInferenceRequest,bitmap_inference_metadata,bitmap_inference_service};
use crate::standards::v1::subsets::any::io::text::snapshot::bitmap_json_decode;
use semio_framework_job::OriginalJobFixtureCaller;
use semio_plugin::{ArtifactDocumentPayload,WireArtifactInferenceBudget,WireArtifactInferenceCacheMode,WireArtifactInferenceRequest,ARTIFACT_INFERENCE_WIRE_VERSION};
use semio_plugin::reactor::jobs::{JOB_KIND_INFER,JobStep,OriginalJobAdmission,abi,has_owned_jobs};

const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:1<<28,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:64};
const TURN:RetainedCloneGrant=RetainedCloneGrant{maximum_items:8192,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:64};
const BUDGET:WireArtifactInferenceBudget=WireArtifactInferenceBudget{allocation_bytes:1<<24,work_units:10_000_000,recursion_depth:64};

fn pinned_request()->(Vec<u8>,serde_json::Value){
    let pin:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../🧑‍💻dev/🤝️mcp-client-e2e/🧫️initial-artifact.json")).unwrap();
    let snapshot:BitmapSnapshot=bitmap_json_decode(&pin["solve"].to_string()).unwrap();
    let pack=<BitmapSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snapshot);
    let history=HistoryLog{doc_id:"pinned-bitmap".into(),schema:snapshot.schema.clone(),..Default::default()};
    let spr=semio_framework_async::poll::resolve_ready(store::os_spr::encode_history(&history,&Default::default())).unwrap();
    let document=ArtifactDocumentPayload::from_pair(&pack,&spr);
    let payload=semio_framework_pack_json::to_json_string(&BitmapInferenceRequest{snapshot:None,document:Some(document),checkpoint:None});
    (payload.into_bytes(),pin["result"].clone())
}

fn assert_pinned_result(bytes:&[u8],mut expected:serde_json::Value){
    let actual:serde_json::Value=serde_json::from_slice(bytes).unwrap();
    for entry in expected["entropy"].as_array_mut().unwrap(){
        *entry=serde_json::Value::from(entry.as_f64().unwrap());
    }
    assert_eq!(actual,expected);
    let entropy_bits=|result:&serde_json::Value|result["entropy"].as_array().unwrap().iter().map(|entry|entry.as_f64().unwrap().to_bits()).collect::<Vec<_>>();
    assert_eq!(entropy_bits(&actual),entropy_bits(&expected));
}

#[test]
fn paid_bitmap_route_solves_the_pinned_document_and_closes_its_sources() {
    let (payload,expected)=pinned_request();
    let request=ArtifactInferenceExecutionRequest{operation:87,generation:4,cancelled:false,policy:&[],budgets:&BUDGET,retained:TURN,cancellation_id:"pinned-bitmap",previous_state:None,requested_cache_mode:WireArtifactInferenceCacheMode::Cold,canonical_payload:&payload,dependencies:&[]};
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(request.operation),Generation(request.generation),STOCK,||Some(0),1_000_000,BUDGET.work_units);
    let mut decoder=caller.admitted_decoder();
    let mut route=BitmapRoute::default();
    let mut result=None;
    for _ in 0..10_000{
        match caller.turn_with_decoder(TURN,&mut decoder,|cx|Ok(route.step(&request,RevisionId(19),cx))).unwrap(){
            Ok(step) if step.terminal=>{result=Some(Ok(step.execution.unwrap()));break;}
            Ok(_)=>{},
            Err(error)=>{result=Some(Err(error));break;}
        }
    }
    for _ in 0..10_000{
        if route.terminal_is_empty(){break;}
        caller.retirement_turn_with_decoder(TURN,&mut decoder,|cx|route.close_step(cx)).unwrap();
    }
    assert!(route.terminal_is_empty());
    for _ in 0..10_000{
        if caller.retirement_turn(TURN,|cx|decoder.close_on_source(cx)).unwrap(){break;}
    }
    assert!(decoder.terminal_is_empty());
    caller.retire(STOCK);
    let bytes=result.expect("paid bitmap route reached a terminal turn").expect("paid bitmap route completed").canonical_payload.unwrap();
    assert_pinned_result(&bytes,expected);
}

#[test]
fn registered_original_bitmap_inference_publishes_the_pinned_cold_result(){
    semio_plugin::app::register_artifact_inference_service(bitmap_inference_service()).unwrap();
    let (payload,expected)=pinned_request();
    let metadata=bitmap_inference_metadata();
    let request=WireArtifactInferenceRequest{
        wire_version:ARTIFACT_INFERENCE_WIRE_VERSION,owner:metadata.owner.into(),artifact_kind:metadata.artifact_kind.into(),
        artifact_schema:metadata.artifact_schema.into(),artifact_schema_version:metadata.artifact_schema_version,
        inference_schema:metadata.inference_schema.into(),inference_schema_version:metadata.inference_schema_version,
        algorithm_version:metadata.algorithm_version,policy_version:metadata.policy_version,revision:19,generation:4,
        source_dialect:"s.wfc.bitmap@1/*".into(),policy:Vec::new(),budgets:BUDGET,retained:TURN,
        cancellation_id:"pinned-bitmap".into(),previous_state:None,requested_cache_mode:WireArtifactInferenceCacheMode::Cold,
        canonical_payload:payload,dependencies:Vec::new(),
    };
    let input=semio_framework_pack_json::to_json_string(&request).into_bytes();
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(88),Generation(request.generation),STOCK,||Some(0),1_000_000,BUDGET.work_units);
    let mut decoder=caller.admitted_decoder();
    let job=41;
    let mut admission=Some(OriginalJobAdmission{job,kind:JOB_KIND_INFER.into(),input:Some(input),checkpoint:None});
    assert!(caller.turn_with_decoder(TURN,&mut decoder,|cx|Ok(abi::start(&mut admission,cx)?)).unwrap());
    assert!(admission.is_none());
    let mut outcome=None;
    for _ in 0..10_000{
        if let Some(result)=caller.turn_with_decoder(TURN,&mut decoder,|cx|abi::step(job,cx)).unwrap(){
            if matches!(result,JobStep::Done(_)|JobStep::Failed(_)){outcome=Some(result);break;}
        }
    }
    assert!(!has_owned_jobs(88,request.generation),"registered job must settle before returning its result");
    for _ in 0..10_000{
        if caller.retirement_turn(TURN,|cx|decoder.close_on_source(cx)).unwrap(){break;}
    }
    assert!(decoder.terminal_is_empty());
    caller.retire(STOCK);
    let bytes=match outcome.expect("registered original inference reached terminal output"){
        JobStep::Done(bytes)=>bytes,
        JobStep::Failed(bytes)=>panic!("registered original inference refused: {bytes:?}"),
        _=>unreachable!(),
    };
    let result:serde_json::Value=serde_json::from_slice(&bytes).unwrap();
    assert_eq!(result["complete"],true);
    assert_eq!(result["actualCacheMode"],"cold");
    assert_eq!(result["inferenceSchema"],request.inference_schema);
    assert_eq!(result["artifactKind"],request.artifact_kind);
    assert_eq!(result["revision"],request.revision);
    assert_eq!(result["generation"],request.generation);
    let output=serde_json::from_value::<Vec<u8>>(result["canonicalPayload"].clone()).unwrap();
    assert_pinned_result(&output,expected);
}
