use semio_framework_os_mcp::GatewayErrorCode;

use super::*;
use semio_framework_os_kernel::os_directory::DocumentScope;
use semio_framework_async::OperationContext;
use semio_framework_async::{CancelToken, TraceId};

fn sample_job_id() -> String {
    "ab".repeat(16)
}

/// 🆔️ A well-formed 32-lower-hex job id that is never the fixture's own `sampleJobId` — a hostile
/// "foreign job" candidate that happens to equal the sample asserts nothing at all.
fn foreign_job_id() -> String {
    let candidate = "2".repeat(32);
    if sample_job_id() == candidate {
        "3".repeat(32)
    } else {
        candidate
    }
}

fn sample_proposal_hash() -> String {
    "cd".repeat(32)
}

fn context(cancel: &CancelToken) -> OperationContext {
    OperationContext { actor: 1, generation: 0, trace: TraceId(1), lane: 1, deadline_ms: Some(u64::MAX), cancel: cancel.child_now(), capability: None }
}

fn scope() -> DocumentScope {
    DocumentScope::new("space:alpha".to_string(), "doc:tokyo".to_string())
}

/// 🎭️ A scripted transport: it records every request it is handed and replays one canned reply,
/// so the client's own decoding, bounds and error mapping are exercised with no hub at all.
struct ScriptedTransport {
    replies: std::sync::Mutex<Vec<Result<InferenceHubResponseV1, InferenceHubTransportErrorV1>>>,
    seen: std::sync::Mutex<Vec<InferenceHubRequestV1>>,
}

impl ScriptedTransport {
    fn new(replies: Vec<Result<InferenceHubResponseV1, InferenceHubTransportErrorV1>>) -> Self {
        Self { replies: std::sync::Mutex::new(replies), seen: std::sync::Mutex::new(Vec::new()) }
    }

    fn ok(status: u16, body: serde_json::Value) -> Self {
        Self::new(vec![Ok(InferenceHubResponseV1 { status, body: serde_json::to_vec(&body).expect("scripted body") })])
    }

    fn requests(&self) -> Vec<InferenceHubRequestV1> {
        self.seen.lock().expect("scripted transport lock").clone()
    }
}

impl InferenceHubTransport for ScriptedTransport {
    async fn request(&self, context: &OperationContext, request: &InferenceHubRequestV1) -> Result<InferenceHubResponseV1, InferenceHubTransportErrorV1> {
        if context.cancel.is_cancelled_now() {
            return Err(InferenceHubTransportErrorV1::Cancelled);
        }
        self.seen.lock().expect("scripted transport lock").push(request.clone());
        let mut replies = self.replies.lock().expect("scripted transport lock");
        if replies.is_empty() {
            return Err(InferenceHubTransportErrorV1::Unavailable);
        }
        replies.remove(0)
    }
}

fn block_on<F: std::future::Future>(future: F) -> F::Output {
    static LOCK:std::sync::Mutex<()>=std::sync::Mutex::new(());
    let _guard=LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    install_remote_inference_protocols_v1(vec![gis_map_mcp_protocol_v1()]).unwrap();
    tokio::runtime::Builder::new_current_thread().enable_all().build().expect("current-thread runtime").block_on(future)
}

const HUB_INFERENCE_RECEIPT_SCHEMA:&str="semio.hub.inference-job-receipt/v1";
const HUB_INFERENCE_EVENTS_SCHEMA:&str="semio.hub.inference-job-events/v1";
const HUB_INFERENCE_APPROVAL_RECEIPT_SCHEMA:&str="semio.hub.inference-approval-receipt/v1";
const INFERENCE_REQUEST_MAX_BYTES:usize=1024;
const INFERENCE_RESPONSE_MAX_BYTES:usize=16384;
fn hub_inference_jobs_path(scope:&DocumentScope,_route:&str)->String {
    format!("/spaces/{}/documents/{}/inference/gis-map/jobs",scope.space_id.replace(":","%3A"),scope.document_id.replace(":","%3A"))
}

#[test]
fn owner_codec_declares_only_bounded_closed_request_fields() {
    let port=semio_framework_os_kernel::os_directory::client::document_http::CompiledDocumentHttpPortV1::compile("gis",crate::inference_client::declaration()).unwrap();
    let request=HubInferenceSubmitRequestV1::new(GIS_MAP_INFERENCE_SERVICE_ID,sample_job_id(),1000);
    let payload=dsl(&serde_json::to_value(request).unwrap()).unwrap();
    let encoded=encode("submit",&payload).unwrap();
    let prepared=port.prepare("submit",&encoded).unwrap();
    assert!(prepared.body.unwrap().len()<=INFERENCE_REQUEST_MAX_BYTES);
    let mut fields=json(&encoded).unwrap();
    for (key,value) in [("schema",serde_json::json!("other/v1")),("version",serde_json::json!(2)),("requestId",serde_json::json!("Z".repeat(32))),("serviceId",serde_json::json!("other.service")),("policyVersion",serde_json::json!(2)),("lifetimeMs",serde_json::json!(0))] {
        let mut candidate=fields.clone();candidate[key]=value;
        assert!(port.prepare("submit",&dsl(&candidate).unwrap()).is_err(),"{key}");
    }
    fields["private"]=serde_json::json!(true);
    assert!(port.prepare("submit",&dsl(&fields).unwrap()).is_err());
    assert_eq!(error(503,br#"{"schema":"semio.hub.inference-error/v1","code":"approval.commit-unavailable"}"#),InferenceRouteErrorV1::CommitUnavailable);
    assert_eq!(error(503,br#"{"schema":"foreign/v1","code":"inference.denied"}"#),InferenceRouteErrorV1::Unavailable);
}

//#region 🧪️Calls
#[test]
fn a_submit_call_posts_the_bounded_closed_intent_to_the_exact_job_route() {
    let transport =
        ScriptedTransport::ok(200, serde_json::json!({ "schema": HUB_INFERENCE_RECEIPT_SCHEMA, "jobId": sample_job_id(), "state": "accepted", "proposalState": "none", "proposalHash": serde_json::Value::Null, "cursor": 0, "expiresAtMs": 9 }));
    let cancel = CancelToken::root_now();
    let request = HubInferenceSubmitRequestV1::new(GIS_MAP_INFERENCE_SERVICE_ID, sample_job_id(), 1_000);
    let receipt = block_on(submit_hub_inference_job(&transport, &context(&cancel), "https://hub.invalid", &scope(), "inference/gis-map", &request)).expect("scripted receipt");
    assert_eq!(receipt.job_id, sample_job_id());
    assert_eq!(receipt.proposal_hash, None);
    let seen = transport.requests();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].method, InferenceHubMethodV1::Post);
    assert_eq!(seen[0].path, hub_inference_jobs_path(&scope(), "inference/gis-map"));
    assert!(seen[0].body.len() <= INFERENCE_REQUEST_MAX_BYTES);
    assert_eq!(seen[0].maximum_response_bytes, INFERENCE_RESPONSE_MAX_BYTES);
}

#[test]
fn an_events_call_refuses_a_foreign_job_id_or_an_out_of_range_cursor_before_any_request() {
    let transport = ScriptedTransport::new(Vec::new());
    let cancel = CancelToken::root_now();
    assert_eq!(block_on(read_hub_inference_job_events(&transport, &context(&cancel), "https://hub.invalid", &scope(), "inference/gis-map", "not-a-job", 0)).unwrap_err(), InferenceRouteErrorV1::Invalid);
    assert_eq!(block_on(read_hub_inference_job_events(&transport, &context(&cancel), "https://hub.invalid", &scope(), "inference/gis-map", &sample_job_id(), INFERENCE_PROGRESS_MAX_CURSOR + 1)).unwrap_err(), InferenceRouteErrorV1::Invalid);
    assert!(transport.requests().is_empty(), "a malformed read never reaches the network");
}

#[test]
fn an_already_cancelled_operation_context_never_reaches_the_hub_and_maps_to_cancelled() {
    let transport = ScriptedTransport::ok(200, serde_json::json!({}));
    let cancel = CancelToken::root_now();
    cancel.cancel_now();
    let error = block_on(cancel_hub_inference_job(&transport, &context(&cancel), "https://hub.invalid", &scope(), "inference/gis-map", &sample_job_id())).unwrap_err();
    assert_eq!(error, InferenceRouteErrorV1::Cancelled);
    assert_eq!(error.to_gateway_error("inference_cancel").code, GatewayErrorCode::Cancelled);
    assert!(transport.requests().is_empty(), "a cancelled call is never sent");
}

#[test]
fn a_transport_failure_maps_onto_the_closed_route_vocabulary_and_never_a_fabricated_success() {
    for (transport_error, expected) in [
        (InferenceHubTransportErrorV1::Unauthorized, InferenceRouteErrorV1::Denied),
        (InferenceHubTransportErrorV1::DeadlineExceeded, InferenceRouteErrorV1::Unavailable),
        (InferenceHubTransportErrorV1::Unavailable, InferenceRouteErrorV1::Unavailable),
        (InferenceHubTransportErrorV1::ResourceLimit, InferenceRouteErrorV1::Bounds),
        (InferenceHubTransportErrorV1::InvalidRequest("authority mismatch"), InferenceRouteErrorV1::Invalid),
        (InferenceHubTransportErrorV1::Cancelled, InferenceRouteErrorV1::Cancelled),
    ] {
        let transport = ScriptedTransport::new(vec![Err(transport_error.clone())]);
        let cancel = CancelToken::root_now();
        let approval = HubInferenceApprovalRequestV1::new(sample_job_id(), sample_proposal_hash());
        assert_eq!(block_on(approve_hub_inference_job(&transport, &context(&cancel), "https://hub.invalid", &scope(), "inference/gis-map", &approval)).unwrap_err(), expected, "{transport_error:?}");
    }
}

#[test]
fn an_approval_receipt_must_bind_the_exact_job_proposal_and_durable_undo_scope() {
    let scope = scope();
    let request = HubInferenceApprovalRequestV1::new(sample_job_id(), sample_proposal_hash());
    let exact = serde_json::json!({
        "schema": HUB_INFERENCE_APPROVAL_RECEIPT_SCHEMA,
        "jobId": request.job_id.clone(),
        "mutationId": "aa".repeat(16),
        "commandHash": "bb".repeat(32),
        "proposalHash": request.proposal_hash.clone(),
        "applied": true,
        "undo": {
            "targetId": "cc".repeat(16),
            "expectedCurrent": {
                "documentId": scope.document_id.clone(),
                "headEditOrdinal": 2,
                "headEditId": "aa".repeat(16),
                "lastCommitSeq": 2,
                "chainSha256": "dd".repeat(32),
            },
        },
    });
    let transport = ScriptedTransport::ok(200, exact.clone());
    let cancel = CancelToken::root_now();
    let receipt = block_on(approve_hub_inference_job(&transport, &context(&cancel), "https://hub.invalid", &scope, "inference/gis-map", &request)).expect("exact approval receipt");
    assert_eq!(json(&receipt.undo).unwrap()["targetId"].as_str().unwrap(), "cc".repeat(16));

    for (name, candidate) in [
        ("foreign-job", {
            let mut value = exact.clone();
            value["jobId"] = serde_json::json!(foreign_job_id());
            value
        }),
        ("foreign-proposal", {
            let mut value = exact.clone();
            value["proposalHash"] = serde_json::json!("11".repeat(32));
            value
        }),
        ("long-mutation", {
            let mut value = exact.clone();
            value["mutationId"] = serde_json::json!("11".repeat(32));
            value
        }),
        ("malformed-command", {
            let mut value = exact.clone();
            value["commandHash"] = serde_json::json!("g".repeat(64));
            value
        }),
        ("guessed-target", {
            let mut value = exact.clone();
            value["undo"]["targetId"] = serde_json::json!("short");
            value
        }),
        ("foreign-document", {
            let mut value = exact.clone();
            value["undo"]["expectedCurrent"]["documentId"] = serde_json::json!("other-document");
            value
        }),
    ] {
        let transport = ScriptedTransport::ok(200, candidate);
        assert_eq!(block_on(approve_hub_inference_job(&transport, &context(&CancelToken::root_now()), "https://hub.invalid", &scope, "inference/gis-map", &request)), Err(if name=="long-mutation" || name=="malformed-command" || name=="guessed-target" {InferenceRouteErrorV1::Invalid} else {InferenceRouteErrorV1::Conflict}), "{name}",);
    }
}



#[test]
fn neutral_job_pages_refuse_foreign_jobs_and_private_payloads() {
    let source = include_str!("🧫️fixtures/🔣️.json");
    let vectors: serde_json::Value = serde_json::from_str(source).unwrap();
    let owned: DslValue = semio_framework_os_kernel::os_pack::json::from_json_str(source).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_os_kernel::os_pack::json::to_json_string(&owned)).unwrap(), vectors);
    for case in vectors["operationLabels"].as_array().unwrap() {
        assert_eq!(inference_operation_label(case["spaceId"].as_str().unwrap(), case["documentId"].as_str().unwrap(), case["jobId"].as_str()), case["expected"].as_str().unwrap());
    }
    for case in vectors["cases"].as_array().unwrap() {
        let cancel = CancelToken::root_now();
        let transport = ScriptedTransport::ok(200, case["response"].clone());
        let result = block_on(read_hub_inference_job_events(&transport, &context(&cancel), "https://hub.invalid", &scope(), "inference/gis-map", case["requestedJobId"].as_str().unwrap(), 0));
        match case["expectedError"].as_str() {
            Some(code) => assert_eq!(result.unwrap_err().code(), code, "{}", case["name"]),
            None => assert_eq!(serde_json::to_value(result.unwrap()).unwrap()["state"], case["response"]["state"]),
        }
    }
}

#[test]
fn installed_gis_descriptor_discovery_uses_the_registered_tool_without_granting_execution() {
    use semio_framework_os_mcp::{catalog::{CatalogSource,compile},workspace::HeadlessWorkspace,protocol::{ToolRegistry,InMemoryToolRegistry}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🗺️discovery/🔣️.json")).unwrap();
    let repo=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(7).unwrap();
    let descriptor=semio_framework_os_mcp::workspace::load_package_descriptor(&repo.join("🌎️hub/🧩️compositions/🌍️gis")).unwrap();
    let catalog=std::sync::Arc::new(compile(&CatalogSource {descriptors:vec![descriptor],..Default::default()},semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native).unwrap());
    let output=std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).expect("select generated output for owner discovery");
    let folder=output.join("owner-discovery");std::fs::create_dir_all(&folder).unwrap();
    let workspace=std::sync::Arc::new(HeadlessWorkspace::open_folder(folder,"agent:test".into(),Vec::new(),catalog).unwrap());
    let mut registry=InMemoryToolRegistry::new();register_inference_tools(&mut registry,Some(workspace));
    let result=registry.call(fixture["tool"].as_str().unwrap(),fixture["arguments"].clone()).unwrap();
    println!("[DEBUG] owner descriptor discovery error={} structured={:?}",result.is_error,result.structured_content);
    assert!(!result.is_error);assert_eq!(result.structured_content.unwrap(),fixture["expected"]);
    let denied=registry.call("inference_get",serde_json::json!({"artifactId":fixture["unboundArtifact"],"inferenceSchema":GIS_MAP_INFERENCE_SERVICE_ID})).unwrap();
    assert!(denied.is_error);assert_eq!(denied.structured_content.unwrap()["code"],fixture["executionError"]);
}
