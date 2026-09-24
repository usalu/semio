"""🔁️ G9: rewrites the MCP inference quartet onto the generic service model (hub-published or guest-executed)."""
import pathlib
P = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs")
s = P.read_text()

def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:100], s.count(old))
    s = s.replace(old, new)

def cut(start_marker, end_marker):
    start = s.index(start_marker)
    end = s.index(end_marker, start)
    return s[start:end]

rep('''pub const GIS_MAP_INFERENCE_SERVICE_ID: &str = "s.gis.gismap.inference";
pub const GIS_MAP_INFERENCE_ARTIFACT_SCHEMA: &str = "gis.map";
pub const GIS_MAP_INFERENCE_ARTIFACT_KIND: &str = "s.gis.gismap";
''', '')
rep('''            Self::Unavailable => "this hub publishes no trusted GIS Map inference binding, so all four inference routes fail closed — bind a hub whose readiness reports `features.inference: true`",''',
    '''            Self::Unavailable => "this hub executes no such inference service — bind a hub whose readiness lists it in `features.inferenceServices`",''')
rep('''    /// 🧭️ `service_id` comes from [`resolve_hub_inference_route`] — the artifact's own descriptor
    /// kind decides which hub-backed service this intent names, never the call site.''',
    '''    /// 🧭️ `service_id` comes from [`resolve_document_inference_service`] — the artifact's own
    /// descriptor kind and the hub's published roster decide which service this intent names.''')
rep('''            || !HUB_INFERENCE_ROUTES.iter().any(|route| route.service_id == self.service_id)''',
    '''            || !is_service_id(&self.service_id)''')
rep('''fn is_lower_hex(value: &str, length: usize) -> bool {''', '''/// 🪪️ A declared inference service id: the `InferenceIdV1` shape the discovery schema pins.
fn is_service_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= 256 && value.bytes().next().is_some_and(|byte| byte.is_ascii_alphanumeric()) && value.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-'))
}

fn is_lower_hex(value: &str, length: usize) -> bool {''')

old_paths = cut("/// 🛣️ The four exact hub paths", "/// 🔓️ Decodes one hub reply")
rep(old_paths, '''/// 🛣️ The exact hub paths of one hub-executed service, built from the route family the hub
/// publishes for it and percent-encoded per segment — never string-concatenated by a caller.
pub fn hub_inference_jobs_path(scope: &DocumentScope, route: &str) -> String {
    format!("/spaces/{}/documents/{}/{route}/jobs", percent_encode(&scope.space_id), percent_encode(&scope.document_id))
}

pub fn hub_inference_job_events_path(scope: &DocumentScope, route: &str, job_id: &str, after: u64) -> String {
    format!("{}/{}/events?after={after}", hub_inference_jobs_path(scope, route), percent_encode(job_id))
}

pub fn hub_inference_job_cancel_path(scope: &DocumentScope, route: &str, job_id: &str) -> String {
    format!("{}/{}/cancel", hub_inference_jobs_path(scope, route), percent_encode(job_id))
}

pub fn hub_inference_job_approval_path(scope: &DocumentScope, route: &str, job_id: &str) -> String {
    format!("{}/{}/approval", hub_inference_jobs_path(scope, route), percent_encode(job_id))
}

pub fn hub_inference_approval_undo_path(scope: &DocumentScope, route: &str) -> String {
    format!("/spaces/{}/documents/{}/{route}/approval-undos", percent_encode(&scope.space_id), percent_encode(&scope.document_id))
}

/// 🛂️ A route family the hub published: relative, lower-case segments only, so a hostile readiness
/// body can never steer a protected request off its document scope.
pub fn is_hub_inference_route(route: &str) -> bool {
    !route.is_empty() && route.len() <= 64 && route.split('/').all(|segment| !segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'))
}

''')
rep('''/// 📥️ `POST /spaces/{space}/documents/{document}/inference/gis-map/jobs`.
pub async fn submit_hub_inference_job<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, request: &HubInferenceSubmitRequestV1) -> Result<HubInferenceJobReceiptV1, InferenceRouteErrorV1> {
    let body = request.encode()?;
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Post, path: hub_inference_jobs_path(scope), body, maximum_response_bytes: INFERENCE_RESPONSE_MAX_BYTES };''',
    '''/// 📥️ `POST /spaces/{space}/documents/{document}/{route}/jobs`.
pub async fn submit_hub_inference_job<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, route: &str, request: &HubInferenceSubmitRequestV1) -> Result<HubInferenceJobReceiptV1, InferenceRouteErrorV1> {
    if !is_hub_inference_route(route) {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let body = request.encode()?;
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Post, path: hub_inference_jobs_path(scope, route), body, maximum_response_bytes: INFERENCE_RESPONSE_MAX_BYTES };''')
rep('''pub async fn read_hub_inference_job_events<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, job_id: &str, after: u64) -> Result<HubInferenceEventPageV1, InferenceRouteErrorV1> {
    if !is_lower_hex(job_id, INFERENCE_REQUEST_ID_HEX_LENGTH) || after > INFERENCE_PROGRESS_MAX_CURSOR {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Get, path: hub_inference_job_events_path(scope, job_id, after),''',
    '''pub async fn read_hub_inference_job_events<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, route: &str, job_id: &str, after: u64) -> Result<HubInferenceEventPageV1, InferenceRouteErrorV1> {
    if !is_hub_inference_route(route) || !is_lower_hex(job_id, INFERENCE_REQUEST_ID_HEX_LENGTH) || after > INFERENCE_PROGRESS_MAX_CURSOR {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Get, path: hub_inference_job_events_path(scope, route, job_id, after),''')
rep('''pub async fn cancel_hub_inference_job<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, job_id: &str) -> Result<HubInferenceEventPageV1, InferenceRouteErrorV1> {
    if !is_lower_hex(job_id, INFERENCE_REQUEST_ID_HEX_LENGTH) {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Post, path: hub_inference_job_cancel_path(scope, job_id),''',
    '''pub async fn cancel_hub_inference_job<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, route: &str, job_id: &str) -> Result<HubInferenceEventPageV1, InferenceRouteErrorV1> {
    if !is_hub_inference_route(route) || !is_lower_hex(job_id, INFERENCE_REQUEST_ID_HEX_LENGTH) {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Post, path: hub_inference_job_cancel_path(scope, route, job_id),''')
rep('''pub async fn approve_hub_inference_job<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, request: &HubInferenceApprovalRequestV1) -> Result<HubInferenceApprovalReceiptV1, InferenceRouteErrorV1> {
    let body = request.encode()?;
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Post, path: hub_inference_job_approval_path(scope, &request.job_id),''',
    '''pub async fn approve_hub_inference_job<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, route: &str, request: &HubInferenceApprovalRequestV1) -> Result<HubInferenceApprovalReceiptV1, InferenceRouteErrorV1> {
    if !is_hub_inference_route(route) {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let body = request.encode()?;
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Post, path: hub_inference_job_approval_path(scope, route, &request.job_id),''')
rep('''    hub_origin: &str,
    scope: &DocumentScope,
    request: &semio_framework_os_kernel::os_directory::GisMapApprovalUndoRequestV1,
) -> Result<semio_framework_os_kernel::os_directory::GisMapApprovalUndoReceiptV1, InferenceRouteErrorV1> {
    if !request.validate() || request.expected_current.document_id != scope.document_id {''',
    '''    hub_origin: &str,
    scope: &DocumentScope,
    route: &str,
    request: &semio_framework_os_kernel::os_directory::GisMapApprovalUndoRequestV1,
) -> Result<semio_framework_os_kernel::os_directory::GisMapApprovalUndoReceiptV1, InferenceRouteErrorV1> {
    if !is_hub_inference_route(route) || !request.validate() || request.expected_current.document_id != scope.document_id {''')
rep('''        path: hub_inference_approval_undo_path(scope),''', '''        path: hub_inference_approval_undo_path(scope, route),''')

rep('''/// 🎫️ The payload one `job_` handle carries. The handle is owned by the connection's own
/// `SessionHandle`, so a job id minted by one MCP connection is unreadable by another; the hub then
/// applies the authoritative owner-private check on top of it.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceJobHandlePayloadV1 {
    pub space_id: String,
    pub document_id: String,
    pub job_id: String,
    pub subject_user_id: String,
    pub authority_generation: u64,
    pub request_id: String,
    pub base: Option<HubInferenceBaseBindingV1>,
}''', '''/// 🎫️ The payload one `job_` handle carries, for either execution site. The handle is owned by the
/// connection's own `SessionHandle`, so a job minted by one MCP connection is unreadable by another;
/// a hub job additionally carries the subject that minted it, and the hub applies its own
/// owner-private check on top.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceJobHandlePayloadV1 {
    pub site: InferenceExecutionSiteV1,
    pub service_id: String,
    pub artifact_kind: String,
    pub plugin_id: String,
    pub document_id: String,
    pub job_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hub: Option<HubInferenceJobBindingV1>,
}

/// 🌎️ What a hub-executed job's handle binds beyond the job itself: the published route family, the
/// minting subject and authority generation, the client idempotency key, and the frozen base.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceJobBindingV1 {
    pub route: String,
    pub space_id: String,
    pub subject_user_id: String,
    pub authority_generation: u64,
    pub request_id: String,
    pub base: Option<HubInferenceBaseBindingV1>,
}''')

rep('''        use_when: vec!["run the hub's GIS Map inference over a bound document".to_string(), "watch, cancel or approve a hub inference job".to_string()],''',
    '''        use_when: vec!["run a declared inference over an open document and commit its result".to_string(), "watch, cancel or approve an inference job".to_string()],''')
rep('''        "Submit Hub Inference Job",
        "Submits one bounded, deterministic inference job to the hub-backed service this artifact's own descriptor kind declares and returns its owner-private receipt and a session-owned job handle. Nothing is applied to the document. — Reicht einen begrenzten, deterministischen Inferenzauftrag beim hub-gestützten Dienst ein, den die Deskriptor-Art dieses Artefakts deklariert, und liefert dessen nur dem Eigentümer sichtbare Quittung sowie ein sitzungsgebundenes Auftrags-Handle. Es wird nichts am Dokument angewendet.",''',
    '''        "Submit Inference Job",
        "Starts one job for an inference service the document's own kind declares — in the plugin's guest, or on the bound hub when the hub executes that service — and returns a session-owned job handle and its first page. Nothing is applied to the document until an approval commits the job's proposal. — Startet einen Auftrag für einen Inferenzdienst, den die Art des Dokuments deklariert — im Gast des Plugins oder auf dem gebundenen Hub, wenn dieser den Dienst ausführt — und liefert ein sitzungsgebundenes Auftrags-Handle mit seiner ersten Seite. Am Dokument wird nichts angewendet, bis eine Genehmigung den Vorschlag des Auftrags festschreibt.",''')
rep('''        "Poll Hub Inference Job Events",
        "Reads the next owner-private bounded page of lifecycle events and progress rows for one job handle. This is the hub job's own event cursor; MCP `notifications/progress` covers local job progress instead. — Liest die nächste, nur dem Eigentümer sichtbare begrenzte Seite mit Lebenszyklus-Ereignissen und Fortschrittszeilen zu einem Auftrags-Handle. Dies ist der Ereigniscursor des Hub-Auftrags; `notifications/progress` deckt den lokalen Auftragsfortschritt ab.",''',
    '''        "Read Inference Job Events",
        "Reads the job's lifecycle events and progress rows after a cursor, with its state, proposal and result. The same page shape for every service and execution site. — Liest die Lebenszyklus-Ereignisse und Fortschrittszeilen des Auftrags nach einem Cursor, mit Zustand, Vorschlag und Ergebnis. Dieselbe Seitenform für jeden Dienst und jeden Ausführungsort.",''')
rep('''        "Cancel Hub Inference Job",
        "Records the owner's durable cancel request on the hub and interrupts this process's local wait. Cancellation is idempotent and never applies anything. — Vermerkt die dauerhafte Abbruchanforderung des Eigentümers beim Hub und unterbricht das lokale Warten dieses Prozesses. Der Abbruch ist idempotent und wendet niemals etwas an.",''',
    '''        "Cancel Inference Job",
        "Cancels a running job, or withdraws the proposal of one awaiting approval. Cancellation never applies anything. — Bricht einen laufenden Auftrag ab oder zieht den Vorschlag eines auf Genehmigung wartenden Auftrags zurück. Ein Abbruch wendet niemals etwas an.",''')
rep('''        "Approve Hub Inference Proposal",
        "Explicitly approves one offered proposal by its exact hash. The hub rebuilds the typed effect and its inverse server-side; `applied` is true only after a real committed-WAL witness. — Genehmigt ausdrücklich einen angebotenen Vorschlag anhand seines exakten Hashes. Der Hub baut die typisierte Wirkung und ihre Umkehrung serverseitig neu auf; `applied` ist nur nach einem echten festgeschriebenen WAL-Zeugen wahr.",''',
    '''        "Approve Inference Proposal",
        "Commits one offered proposal, named by its exact hash, as an edit of the document through the normal edit path, so every collaborator sees it and it can be undone. — Schreibt einen angebotenen Vorschlag, benannt durch seinen exakten Hash, als Bearbeitung des Dokuments über den normalen Bearbeitungsweg fest, sodass alle Mitwirkenden ihn sehen und er rückgängig gemacht werden kann.",''')
rep('''/// 💡️ The four hub-backed inference job capabilities, folded into `CatalogSource.gateway`. The
/// general plugin-declared execution route (`inference_run`) is NOT one of them: it crosses no
/// network, needs no hub binding, and belongs with the plugin-declared family
/// ([`inference_capabilities`]).''', '''/// 💡️ The four inference job capabilities, folded into `CatalogSource.gateway`. `inference_run` is
/// their synchronous sibling in the plugin-declared family ([`inference_capabilities`]): the same
/// service resolution and engine, answered in one call with no proposal.''')

helpers = cut("/// 📦️ The canonical request body one `inference_run` call carries into the guest.", "/// 💡️ Runs one declared inference for real.")
tail_helpers = cut("/// 🧩️ Folds the terminal fields onto the invariant ones", "/// 💡️ Registers the four hub-backed inference job tools")
deterministic = cut("/// 🆔️ The deterministic client idempotency key one hub-backed inference READ uses", "/// 💡️ The hub-backed replacement for `channel.not-wired`")

new_region = (pathlib.Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-g9/g9-quartet-handlers.rs.txt").read_text()
    .replace("@@HELPERS@@", helpers).replace("@@TAIL_HELPERS@@", tail_helpers).replace("@@DETERMINISTIC@@", deterministic))
start = s.index("//#region 🔖️HubInferenceRouting")
end = s.index("//#endregion 💡️InferenceHubRead") + len("//#endregion 💡️InferenceHubRead")
s = s[:start] + new_region + s[end:]

rep('''//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;''', '''//#region 💼️Jobs
#[path = "💼️jobs/🦀️.rs"]
pub mod jobs;
pub use jobs::*;
//#endregion 💼️Jobs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;''')
P.write_text(s)
print("ok")
