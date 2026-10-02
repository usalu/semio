//! 🌍 Owner-authored GIS HTTP routes, schemas and closed proposal decoding.

use semio_framework_os_kernel::{DslValue, FromValue, ToValue};
use semio_framework_os_kernel::os_directory::schema::DocumentScope;
use crate::inference_schema::{GisMapApprovalUndoReceiptV1, GisMapApprovalUndoRequestV1, GisMapReconciledJobV1, GisMapInferenceApprovalReceiptV1, GisMapInferenceApprovalRequestV1, GisMapInferenceEventPageV1, GisMapInferenceJobReceiptV1, GisMapInferenceJobRequestV1};
use semio_framework_job::reconcile::{JobReconcileRequestV1, JobReconcileResultV1, parse_job_reconcile_result_v1};
use semio_framework_os_kernel::os_directory::client::{DirectoryClient, DirectoryTransport, DocumentHttpOperationV1, DocumentHttpPortCodeV1, DocumentHttpPortDeclarationV1, DOCUMENT_HTTP_PORT_TOPIC};
use semio_framework_os_kernel::os_directory::client::document_http::CompiledDocumentHttpPortV1;
use semio_framework_async::OperationContext;

/// 📜 Publishes this owner's finite service transport through an existing manifest topic.
pub fn declaration() -> DocumentHttpPortDeclarationV1 {
    let operation = |action: &str, method: &str, route: &[&str], send_body: bool, cursor_field: Option<&str>, input_schema: &str, output_schema: &str| DocumentHttpOperationV1 {
        action: action.into(), method: method.into(), route: route.iter().map(|part| part.to_string()).collect(), send_body, cursor_field: cursor_field.map(str::to_string), request_max_bytes: 1024, response_max_bytes: 16 * 1024, input_schema: input_schema.into(), output_schema: output_schema.into(),
    };
    DocumentHttpPortDeclarationV1 { schema: DOCUMENT_HTTP_PORT_TOPIC.into(), owner: "gis".into(), service_id: "s.gis.gismap.inference".into(), operations: vec![
        operation("submit", "POST", &["inference", "gis-map", "jobs"], true, None, include_str!("🧬️schema/submit-input.json"), include_str!("🧬️schema/submit-output.json")),
        operation("events", "GET", &["inference", "gis-map", "jobs", "{jobId}", "events"], false, Some("after"), include_str!("🧬️schema/events-input.json"), include_str!("🧬️schema/page-output.json")),
        operation("cancel", "POST", &["inference", "gis-map", "jobs", "{jobId}", "cancel"], false, None, include_str!("🧬️schema/cancel-input.json"), include_str!("🧬️schema/page-output.json")),
        operation("approve", "POST", &["inference", "gis-map", "jobs", "{jobId}", "approval"], true, None, include_str!("🧬️schema/approve-input.json"), include_str!("🧬️schema/approve-output.json")),
        operation("reconcile", "POST", &["inference", "gis-map", "jobs", "reconcile"], true, None, include_str!("🧬️schema/reconcile-input.json"), include_str!("🧬️schema/reconcile-output.json")),
        operation("undo", "POST", &["inference", "gis-map", "approval-undos"], true, None, include_str!("🧬️schema/undo-input.json"), include_str!("🧬️schema/undo-output.json")),
    ] }
}

/// 🗺 Typed GIS client composed over neutral document transport and the owner declaration.
pub struct GisMapInferenceClientV1<'a, T: DirectoryTransport> {
    client: &'a DirectoryClient<T>,
    port: CompiledDocumentHttpPortV1,
}

#[derive(FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct GisMapReconcileEnvelopeV1 {
    schema: String,
    version: u32,
    request_id: String,
    found: bool,
    #[value(required)]
    job: Option<GisMapReconciledJobV1>,
}

pub(crate) fn decode_reconcile_v1(value: DslValue, scope: &DocumentScope, request_id: &str) -> Result<JobReconcileResultV1<GisMapReconciledJobV1>, DocumentHttpPortCodeV1> {
    let envelope = GisMapReconcileEnvelopeV1::from_value(value).map_err(|_| DocumentHttpPortCodeV1::Invalid)?;
    if envelope.request_id != request_id || envelope.job.as_ref().is_some_and(|job| !job.validate(&scope.document_id)) { return Err(DocumentHttpPortCodeV1::Invalid); }
    parse_job_reconcile_result_v1(&envelope.schema, envelope.version, &envelope.request_id, envelope.found, envelope.job).map_err(|_| DocumentHttpPortCodeV1::Invalid)
}

impl<'a, T: DirectoryTransport> GisMapInferenceClientV1<'a, T> {
    pub fn new(client: &'a DirectoryClient<T>) -> Result<Self, DocumentHttpPortCodeV1> {
        Ok(Self { client, port: CompiledDocumentHttpPortV1::compile("gis", declaration())? })
    }

    /// 📮 Sends one admitted request without retrying an indeterminate submission.
    pub async fn submit(&self, ctx: &OperationContext, scope: &DocumentScope, request: &GisMapInferenceJobRequestV1) -> Result<GisMapInferenceJobReceiptV1, DocumentHttpPortCodeV1> {
        let value = self.port.call(self.client, ctx, scope, "submit", &request.to_value()).await?;
        let receipt = GisMapInferenceJobReceiptV1::from_value(value).map_err(|_| DocumentHttpPortCodeV1::Invalid)?;
        receipt.validate().then_some(receipt).ok_or(DocumentHttpPortCodeV1::Invalid)
    }

    /// 📈 Reads one exact owner-bound progress page.
    pub async fn events(&self, ctx: &OperationContext, scope: &DocumentScope, job_id: &str, after: u64) -> Result<GisMapInferenceEventPageV1, DocumentHttpPortCodeV1> {
        self.page(ctx, scope, "events", job_id, Some(after)).await
    }

    /// 🛑 Cancels through the server's own lifecycle response.
    pub async fn cancel(&self, ctx: &OperationContext, scope: &DocumentScope, job_id: &str) -> Result<GisMapInferenceEventPageV1, DocumentHttpPortCodeV1> {
        self.page(ctx, scope, "cancel", job_id, None).await
    }

    async fn page(&self, ctx: &OperationContext, scope: &DocumentScope, action: &str, job_id: &str, after: Option<u64>) -> Result<GisMapInferenceEventPageV1, DocumentHttpPortCodeV1> {
        let mut fields = vec![("jobId".into(), DslValue::String(job_id.into()))];
        if let Some(after) = after { fields.push(("after".into(), after.to_value())); }
        let value = self.port.call(self.client, ctx, scope, action, &DslValue::Object(fields)).await?;
        let page = GisMapInferenceEventPageV1::from_value(value).map_err(|_| DocumentHttpPortCodeV1::Invalid)?;
        page.validate(job_id).then_some(page).ok_or(DocumentHttpPortCodeV1::Invalid)
    }

    /// ✅ Approves only the exact job and proposal advertised by the owner.
    pub async fn approve(&self, ctx: &OperationContext, scope: &DocumentScope, request: &GisMapInferenceApprovalRequestV1) -> Result<GisMapInferenceApprovalReceiptV1, DocumentHttpPortCodeV1> {
        let value = self.port.call(self.client, ctx, scope, "approve", &request.to_value()).await?;
        let receipt = GisMapInferenceApprovalReceiptV1::from_value(value).map_err(|_| DocumentHttpPortCodeV1::Invalid)?;
        receipt.validate(&request.job_id, &request.proposal_hash).then_some(receipt).ok_or(DocumentHttpPortCodeV1::Invalid)
    }

    /// 🔎 Recovers one exact retry identity without submitting a replacement operation.
    pub async fn reconcile(&self, ctx: &OperationContext, scope: &DocumentScope, request: &JobReconcileRequestV1) -> Result<JobReconcileResultV1<GisMapReconciledJobV1>, DocumentHttpPortCodeV1> {
        let payload = DslValue::Object(vec![("schema".into(), request.schema.to_value()), ("version".into(), request.version.to_value()), ("requestId".into(), request.request_id.to_value())]);
        let value = self.port.call(self.client, ctx, scope, "reconcile", &payload).await?;
        decode_reconcile_v1(value, scope, &request.request_id)
    }

    /// ↩️ Applies the retained inverse only against the exact durable document tail.
    pub async fn undo(&self, ctx: &OperationContext, scope: &DocumentScope, request: &GisMapApprovalUndoRequestV1) -> Result<GisMapApprovalUndoReceiptV1, DocumentHttpPortCodeV1> {
        if !request.validate() || request.expected_current.document_id != scope.document_id { return Err(DocumentHttpPortCodeV1::Invalid); }
        let value = self.port.call(self.client, ctx, scope, "undo", &request.to_value()).await?;
        let receipt = GisMapApprovalUndoReceiptV1::from_value(value).map_err(|_| DocumentHttpPortCodeV1::Invalid)?;
        (receipt.applied && receipt.target_id == request.target_id && receipt.frontier.validate() && receipt.frontier.document_id == scope.document_id && request.expected_current.head_edit_ordinal.checked_add(1) == Some(receipt.frontier.head_edit_ordinal)).then_some(receipt).ok_or(DocumentHttpPortCodeV1::Invalid)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
