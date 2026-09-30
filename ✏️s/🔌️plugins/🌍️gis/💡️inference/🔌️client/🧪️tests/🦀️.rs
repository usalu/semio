use super::*;
use semio_framework_os_kernel::os_directory::client::{DirectoryWsConnection, DirectoryWsPoll, HttpMethod, HttpResponse, TransportError};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct TestTransport { replies: Arc<Mutex<std::collections::VecDeque<Vec<u8>>>>, calls: Arc<Mutex<Vec<String>>> }
struct TestWs;
impl DirectoryWsConnection for TestWs {
    fn send_text(&mut self, _: String) -> Result<(), TransportError> { unreachable!() }
    fn send_binary(&mut self, _: Vec<u8>) -> Result<(), TransportError> { unreachable!() }
    fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> { unreachable!() }
    fn close(&mut self) {}
}
impl DirectoryTransport for TestTransport {
    type Ws = TestWs;
    async fn http(&self, _: &OperationContext, _: HttpMethod, url: &str, _: Option<&str>, _: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
        self.calls.lock().unwrap().push(url.into());
        Ok(HttpResponse { status: 200, body: self.replies.lock().unwrap().pop_front().unwrap() })
    }
    async fn get_accepting(&self, _: &OperationContext, _: &str, _: Option<&str>, _: &str) -> Result<HttpResponse, TransportError> { unreachable!() }
    fn issue_socket_grant(&self, _: &OperationContext, _: &str, _: &str, _: &[u8], _: u64) -> Result<HttpResponse, TransportError> { unreachable!() }
    fn open_ws(&self, _: &OperationContext, _: &str, _: &[String], _: u64) -> Result<Self::Ws, TransportError> { unreachable!() }
}
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn decode<R: FromValue>(value: &serde_json::Value) -> R { semio_framework_os_kernel::os_pack::json::from_json_str(&value.to_string()).unwrap() }
fn context() -> OperationContext { OperationContext { actor: 0, generation: 0, trace: semio_framework_async::TraceId(0), lane: 0, deadline_ms: None, cancel: semio_framework_async::CancelToken::root_now(), capability: None } }

#[test]
fn production_manifest_installs_owner_transport_and_schema_vectors() {
    let plugin = crate::plugin().unwrap();
    let topic = plugin.manifest.topic_contributions.iter().find(|topic| topic.topic == DOCUMENT_HTTP_PORT_TOPIC).unwrap();
    let declared = DocumentHttpPortDeclarationV1::from_value(topic.payload.clone()).unwrap();
    assert_eq!(declared, declaration());
    assert!(CompiledDocumentHttpPortV1::compile(&plugin.manifest.plugin_id, declared.clone()).is_ok());
    assert!(CompiledDocumentHttpPortV1::compile("foreign", declared.clone()).is_err());
    let fixture = fixture();
    for vector in fixture["schemaVectors"].as_array().unwrap() {
        let operation = declared.operations.iter().find(|operation| vector["schema"].as_str().unwrap().starts_with(&operation.action) || (vector["schema"] == "page-output.json" && operation.action == "events")).unwrap();
        let schema = if vector["schema"].as_str().unwrap().ends_with("-input.json") { &operation.input_schema } else { &operation.output_schema };
        let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(schema).unwrap();
        assert_eq!(validator.is_valid_json(&vector["value"].to_string()), vector["valid"].as_bool().unwrap(), "{}", vector["name"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn typed_owner_transport_preserves_proposal_binding_and_closed_geometry() {
    let fixture = fixture();
    let mut forged = fixture["page"].clone(); forged["jobId"] = serde_json::json!("2".repeat(32));
    let mut open = fixture["page"].clone(); open["preview"]["ring"][4][0] = serde_json::json!(8);
    let mut forged_digest = fixture["page"].clone(); forged_digest["preview"]["proposalHash"] = serde_json::json!("a".repeat(64));
    let mut unapplied = fixture["approval"].clone(); unapplied["applied"] = serde_json::json!(false);
    let replies = [fixture["receipt"].clone(), fixture["page"].clone(), fixture["approval"].clone(), forged, forged_digest, open, unapplied];
    let transport = TestTransport { replies: Arc::new(Mutex::new(replies.iter().map(|reply| reply.to_string().into_bytes()).collect())), calls: Arc::new(Mutex::new(Vec::new())) };
    let client = DirectoryClient::new(transport.clone(), "http://gis.test");
    let owner = GisMapInferenceClientV1::new(&client).unwrap();
    let scope = DocumentScope::new("space/one", "document?two");
    let request = decode(&fixture["request"]);
    let receipt = owner.submit(&context(), &scope, &request).await.unwrap();
    let page = owner.events(&context(), &scope, &receipt.job_id, 0).await.unwrap();
    let approval = GisMapInferenceApprovalRequestV1 { schema: "semio.hub.inference-approval/v1".into(), version: 1, job_id: receipt.job_id.clone(), proposal_hash: page.proposal_hash.clone().unwrap() };
    assert!(owner.approve(&context(), &scope, &approval).await.unwrap().applied);
    assert_eq!(owner.events(&context(), &scope, &receipt.job_id, 0).await, Err(DocumentHttpPortCodeV1::Invalid));
    assert_eq!(owner.events(&context(), &scope, &receipt.job_id, 0).await, Err(DocumentHttpPortCodeV1::Invalid));
    assert_eq!(owner.events(&context(), &scope, &receipt.job_id, 0).await, Err(DocumentHttpPortCodeV1::Invalid));
    assert_eq!(owner.approve(&context(), &scope, &approval).await, Err(DocumentHttpPortCodeV1::Invalid));
    assert_eq!(owner.events(&context(), &scope, &receipt.job_id, 17).await, Err(DocumentHttpPortCodeV1::Invalid));
    assert_eq!(transport.calls.lock().unwrap().len(), 7);
    assert!(transport.calls.lock().unwrap()[0].ends_with("/spaces/space%2Fone/documents/document%3Ftwo/inference/gis-map/jobs"));
    let preview = page.preview.unwrap();
    let ring = geo::LineString::from(preview.ring.iter().map(|point| (point[0], point[1])).collect::<Vec<_>>());
    assert!(ring.is_closed());
    assert!(preview.validate());
    let mut open_preview = preview.clone(); open_preview.ring[4][0] = 8.0;
    let oracle_open = geo::LineString::from(open_preview.ring.iter().map(|point| (point[0], point[1])).collect::<Vec<_>>());
    assert_eq!(open_preview.validate(), oracle_open.is_closed());
    let mut nonfinite = preview.clone(); nonfinite.ring[0][0] = f64::INFINITY;
    assert!(!nonfinite.validate());
    eprintln!("[DEBUG] gis-owner transport submit/events/approval validated; foreign job and open ring refused; calls={}", transport.calls.lock().unwrap().len());
}
