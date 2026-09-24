"""📌️ One-off: mount the hub Check In command (routes, job runner, fenced publisher) in the bootstrap."""
p = "/Users/ueli/Documents/semio/🌎️hub/🏗️bootstrap/🦀️.rs"
s = open(p, encoding="utf-8").read()
def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:120], s.count(old))
    s = s.replace(old, new)

rep('''use semio_hub::artifact_authority::{
    ArtifactPair, AuthorityError, AuthorityLimits,''', '''use semio_hub::artifact_authority::check_in::{
    check_in_refusal_of_authority_error, CheckInMaterialization, DocumentCheckInAdmission, DocumentCheckInJob, DocumentCheckInJobs, DocumentCheckInKey, ReplayingArtifactAuthority, DOCUMENT_CHECK_IN_STALL_BOUND_MS,
};
use semio_hub::artifact_authority::{
    ArtifactPair, AuthorityError, AuthorityLimits,''')
rep('''    artifact_maintenance: Arc<ArtifactCasMaintenanceSupervisor>,
''', '''    artifact_maintenance: Arc<ArtifactCasMaintenanceSupervisor>,
    /// @emoji 📌️ Every Check In this process accepted, keyed by author, document and request id.
    check_ins: Arc<DocumentCheckInJobs>,
''')
rep('''        artifact_maintenance: artifact_maintenance.clone(),
''', '''        artifact_maintenance: artifact_maintenance.clone(),
        check_ins: Arc::new(DocumentCheckInJobs::default()),
''')

region = r'''//#region 📌️CheckIn
/// 📍️ The db ledger point one directory frontier names; genesis has no edited tip.
fn check_in_ledger_point(frontier: &ArtifactFrontier) -> db::document::ArtifactLedgerPoint {
    if frontier.head_edit_ordinal == 0 && frontier.last_commit_seq == 0 && frontier.head_edit_id.is_empty() {
        return db::document::ArtifactLedgerPoint::genesis();
    }
    db::document::ArtifactLedgerPoint { head_seq: frontier.head_edit_ordinal, commit_seq: frontier.last_commit_seq, chain_hash: frontier.chain_hash.0, head_edit_id: Some(protocol::MutationId(frontier.head_edit_id.clone())) }
}

/// ⛔️ The refusal a ledger read maps to: a point that is not on this document's ledger is an unknown
/// head, an approval decision inside the range cannot be replayed, a cancelled walk is cancellation.
fn check_in_refusal_of_ledger_error(error: &db::DbError) -> Option<DocumentCheckInRefusalV1> {
    match error {
        db::DbError::NotFound(_) | db::DbError::InvalidArgument(_) => Some(DocumentCheckInRefusalV1::UnknownHead),
        db::DbError::Conflict(_) => Some(DocumentCheckInRefusalV1::LedgerNotReplayable),
        _ => Some(DocumentCheckInRefusalV1::Unavailable),
    }
}

/// 🌊️ Rebootstrap transfer control over the job, so the active pair read shares its cancellation.
struct CheckInPairControlV1<'a>(&'a DocumentCheckInJob);

impl RebootstrapTransferControl for CheckInPairControlV1<'_> {
    fn now_ms(&self) -> u64 {
        self.0.now_ms()
    }

    fn is_cancelled(&self) -> bool {
        AuthorityOperationControl::is_cancelled(self.0)
    }

    fn report(&self, _progress: RebootstrapProgress) {}
}

/// 🧾️ The durable claim of one Check In request; released unless completed with its checkpoint.
struct CheckInClaimGuardV1 {
    service: Arc<DirectoryService>,
    actor_user_id: String,
    correlation_id: String,
    command_sha256: String,
    complete: bool,
}

impl CheckInClaimGuardV1 {
    async fn release(&mut self) {
        if !self.complete && self.service.release_checkpoint_publication(&self.actor_user_id, &self.correlation_id, &self.command_sha256).await.is_ok() {
            self.complete = true;
        }
    }
}

impl Drop for CheckInClaimGuardV1 {
    fn drop(&mut self) {
        if self.complete {
            return;
        }
        let service = self.service.clone();
        let (actor_user_id, correlation_id, command_sha256) = (self.actor_user_id.clone(), self.correlation_id.clone(), self.command_sha256.clone());
        tokio::spawn(async move {
            let _ = service.release_checkpoint_publication(&actor_user_id, &correlation_id, &command_sha256).await;
        });
    }
}

/// 🔒️ Publishes a Check In candidate only while the author still writes, the descriptor is the one it
/// was materialized against, and the active checkpoint is still its parent; completes the durable
/// claim in the same directory transaction as the checkpoint event.
struct FencedCheckInPublisherV1 {
    state: HubState,
    subject: SocketSubjectV1,
    audience: SocketAudienceV1,
    scope: DocumentScope,
    descriptor: DocumentDescriptor,
    parent: PublishedArtifactCheckpoint,
    completion: CheckpointPublicationCompletionV1,
}

impl FencedCheckInPublisherV1 {
    fn changed() -> AuthorityError {
        AuthorityError::Publication("check-in authority changed".into())
    }

    async fn authority_is_current(&self) -> Result<bool, AuthorityError> {
        if self.subject.revalidate(self.state.directory.as_ref(), &self.audience, now_ms()).await != SocketBindingValidityV1::Active {
            return Ok(false);
        }
        let descriptor = self.state.directory.get_document_descriptor(&self.scope).await.map_err(|_| Self::changed())?;
        let current = self.state.directory.get_active_artifact_checkpoint(&self.scope).await.map_err(|_| Self::changed())?;
        Ok(descriptor.as_ref() == Some(&self.descriptor) && current.as_ref().map(|checkpoint| checkpoint.checkpoint_id) == Some(self.parent.checkpoint_id))
    }
}

impl VerifiedCheckpointPublisher for FencedCheckInPublisherV1 {
    async fn reserve(&self, plan: &semio_hub::artifact_authority::chunk_cas::ArtifactCasOwnershipPlanV1, context: &OperationContext<'_>) -> Result<semio_hub::artifact_authority::chunk_cas::ArtifactCasReservation, AuthorityError> {
        context.checkpoint()?;
        if !self.authority_is_current().await? {
            return Err(Self::changed());
        }
        HubVerifiedCheckpointPublisher::new(self.state.directory_service.clone(), self.state.artifact_cas.clone(), "system:check-in").reserve(plan, context).await
    }

    async fn publish_reserved(&self, checkpoint: &os_directory::ArtifactCheckpoint, reservation: &semio_hub::artifact_authority::chunk_cas::ArtifactCasReservation, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        let authorization = tokio::time::timeout(std::time::Duration::from_secs(2), self.state.socket_binding_gates.acquire_record(&self.subject, &self.audience)).await.map_err(|_| Self::changed())?;
        let document_write = tokio::time::timeout(std::time::Duration::from_secs(2), self.state.socket_binding_gates.gate(SocketBindingKeyV1::DocumentWrite(self.scope.clone())).lock_owned()).await.map_err(|_| Self::changed())?;
        context.checkpoint()?;
        if !self.authority_is_current().await? || checkpoint.scope != self.scope || checkpoint.parent_checkpoint_id != Some(self.parent.checkpoint_id) {
            return Err(Self::changed());
        }
        let mut completion = self.completion.clone();
        completion.checkpoint_id = checkpoint.checkpoint_id;
        completion.completed_at = i64::try_from(context.now_ms()).map_err(|_| Self::changed())?;
        let result = self
            .state
            .directory_service
            .publish_reserved_artifact_checkpoint_and_complete_checkpoint_publication(DirectoryActor { kind: DirectoryActorKind::System, id: "system:check-in".into() }, checkpoint.clone(), reservation.clone(), completion, context.now_ms())
            .await
            .map(|_| ())
            .map_err(|_| AuthorityError::Publication("check-in completion failed".into()));
        drop(document_write);
        drop(authorization);
        result
    }
}

/// 📌️ Runs one admitted Check In to its terminal status: read the active checkpoint and its pair,
/// read the committed ledger from its baseline to the named head, fold it through the package codec,
/// and publish the result behind the author/descriptor/active-checkpoint fence. A monitor revalidates
/// the author every 50 ms and revokes the job the moment write access is lost.
async fn run_document_check_in(state: HubState, subject: SocketSubjectV1, scope: DocumentScope, request: DocumentCheckInV1, job: Arc<DocumentCheckInJob>, mut claim: CheckInClaimGuardV1) {
    let audience = SocketAudienceV1::Document(scope.clone());
    let monitor = tokio::spawn({
        let state = state.clone();
        let subject = subject.clone();
        let audience = audience.clone();
        let job = job.clone();
        async move {
            while !job.is_terminal() {
                if subject.revalidate(state.directory.as_ref(), &audience, now_ms()).await != SocketBindingValidityV1::Active {
                    job.revoke();
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
        }
    });
    let span = state.tracer.span("server.document.check-in");
    let outcome = materialize_and_publish_check_in(&state, &subject, &audience, &scope, &request, &job, &claim).await;
    match outcome {
        Ok((checkpoint_id, parent_checkpoint_id, baseline)) => {
            claim.complete = checkpoint_id != parent_checkpoint_id;
            if !claim.complete {
                claim.release().await;
            }
            job.finish_ready(checkpoint_id, parent_checkpoint_id, baseline);
        }
        Err(Some(refusal)) => {
            claim.release().await;
            job.finish_refused(refusal);
        }
        Err(None) => {
            claim.release().await;
            job.finish_cancelled();
        }
    }
    monitor.abort();
    let status = job.status();
    let mut record = span.finish(match status.phase {
        DocumentCheckInPhaseV1::Ready => TraceOutcome::Ok,
        DocumentCheckInPhaseV1::Cancelled => TraceOutcome::Cancelled,
        _ => TraceOutcome::Refused,
    });
    record.principal = Some(subject.trace_principal());
    record.space = Some(scope.space_id.clone());
    record.artifact = Some(scope.document_id.clone());
    record.detail = status.refusal.map(|refusal| format!("{refusal:?}"));
    state.tracer.emit(record);
}

async fn materialize_and_publish_check_in(
    state: &HubState,
    subject: &SocketSubjectV1,
    audience: &SocketAudienceV1,
    scope: &DocumentScope,
    request: &DocumentCheckInV1,
    job: &Arc<DocumentCheckInJob>,
    claim: &CheckInClaimGuardV1,
) -> Result<(ArtifactHash, ArtifactHash, EditedArtifactFrontierV1), Option<DocumentCheckInRefusalV1>> {
    let refuse = |error: AuthorityError| check_in_refusal_of_authority_error(&error);
    let context = OperationContext::stall_bounded(DOCUMENT_CHECK_IN_STALL_BOUND_MS, AuthorityLimits::maximum(), job.as_ref()).map_err(refuse)?;
    job.advance(DocumentCheckInPhaseV1::Materializing, 0);
    let Some(authority) = state.artifact_authority.as_ref() else { return Err(Some(DocumentCheckInRefusalV1::Unavailable)) };
    let head = request.head.artifact_frontier().ok_or(Some(DocumentCheckInRefusalV1::UnknownHead))?;
    let descriptor = match state.directory.get_document_descriptor(scope).await {
        Ok(Some(descriptor)) => descriptor,
        Ok(None) => return Err(Some(DocumentCheckInRefusalV1::AuthorityChanged)),
        Err(_) => return Err(Some(DocumentCheckInRefusalV1::Unavailable)),
    };
    let current = match state.directory.get_active_artifact_checkpoint(scope).await {
        Ok(Some(current)) => current,
        Ok(None) | Err(_) => return Err(Some(DocumentCheckInRefusalV1::Unavailable)),
    };
    let baseline = &current.baseline_frontier;
    if head == *baseline {
        let parent = current.parent_checkpoint_id.ok_or(Some(DocumentCheckInRefusalV1::Unavailable))?;
        return Ok((current.checkpoint_id, parent, request.head.clone()));
    }
    if head.head_edit_ordinal < baseline.head_edit_ordinal || head.last_commit_seq < baseline.last_commit_seq {
        return Err(Some(DocumentCheckInRefusalV1::StaleHead));
    }
    if head.head_edit_ordinal == baseline.head_edit_ordinal || head.last_commit_seq == baseline.last_commit_seq {
        return Err(Some(DocumentCheckInRefusalV1::UnknownHead));
    }
    context.checkpoint().map_err(refuse)?;
    let pair_control = CheckInPairControlV1(job.as_ref());
    let pair_context = RebootstrapContext::new(context.now_ms().saturating_add(REBOOTSTRAP_DEADLINE_MS), &pair_control);
    let active = match state.rebootstrap.active_pair(scope, &pair_context).await {
        Ok(active) => active,
        Err(RebootstrapError::Cancelled) => return Err(None),
        Err(_) => return Err(Some(DocumentCheckInRefusalV1::Unavailable)),
    };
    if active.selection.active_checkpoint_id != current.checkpoint_id {
        return Err(Some(DocumentCheckInRefusalV1::ActiveCheckpointChanged));
    }
    job.advance(DocumentCheckInPhaseV1::Materializing, 1);
    let wal = state.db.storage().await.wal().await;
    let commits = match db::document::artifact_ledger_tail(&wal, &db_core_document_id(&db_artifact_id(scope)), &check_in_ledger_point(baseline), &check_in_ledger_point(&head), job.cancellation()).await {
        Ok(commits) => commits,
        Err(_) if AuthorityOperationControl::is_cancelled(job.as_ref()) => return Err(None),
        Err(error) => return Err(check_in_refusal_of_ledger_error(&error)),
    };
    drop(wal);
    let envelopes: Vec<protocol::MutationEnvelope> = commits.into_iter().flat_map(|commit| commit.envelopes).collect();
    job.advance(DocumentCheckInPhaseV1::Materializing, 2);
    let candidate = authority
        .materialize_check_in(
            CheckInMaterialization { descriptor: descriptor.clone(), scope: scope.clone(), parent_checkpoint_id: current.checkpoint_id, base_pair: active.pair().clone(), head: head.clone(), envelopes: directory::os_spr::encode_envelopes(&envelopes) },
            &context,
        )
        .await
        .map_err(refuse)?;
    #[cfg(test)]
    if let Some(gate) = state.live_gate.as_ref().filter(|gate| gate.check_in_pause_enabled.load(std::sync::atomic::Ordering::Acquire)) {
        gate.check_in_admitted.add_permits(1);
        gate.check_in_release.acquire().await.expect("check-in test release").forget();
    }
    context.checkpoint().map_err(refuse)?;
    let publisher = FencedCheckInPublisherV1 {
        state: state.clone(),
        subject: subject.clone(),
        audience: audience.clone(),
        scope: scope.clone(),
        descriptor,
        parent: current.clone(),
        completion: CheckpointPublicationCompletionV1 { actor_user_id: claim.actor_user_id.clone(), correlation_id: claim.correlation_id.clone(), command_sha256: claim.command_sha256.clone(), checkpoint_id: ArtifactHash([0; 32]), completed_at: 0 },
    };
    let published = CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(state.artifact_cas.clone()), publisher).publish_candidate(candidate, &context).await.map_err(refuse)?;
    Ok((published.checkpoint.checkpoint_id, current.checkpoint_id, request.head.clone()))
}

fn check_in_status_response(status: DocumentCheckInStatusV1) -> Response {
    let code = if status.phase.is_terminal() { StatusCode::OK } else { StatusCode::ACCEPTED };
    let Some(body) = status.canonical_json() else { return StatusCode::INTERNAL_SERVER_ERROR.into_response() };
    let mut response = (code, [(axum::http::header::CONTENT_TYPE, "application/json")], body).into_response();
    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("private, no-store"));
    response
}

/// 🔐️ The author of one document write: a current space author's session, never a share and never a
/// spectator. `401` for no credential, `403` for a member who may not write.
async fn check_in_author(state: &HubState, scope: &DocumentScope, headers: &HeaderMap) -> Result<(SocketSubjectV1, String), StatusCode> {
    let (subject, _) = authenticate_document_socket_subject(state, scope, headers).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
    match &subject {
        SocketSubjectV1::Session { role: Some(SpaceRole::Author), user_id, .. } => {
            let user_id = user_id.clone();
            Ok((subject, user_id))
        }
        SocketSubjectV1::Session { .. } => Err(StatusCode::FORBIDDEN),
        SocketSubjectV1::Share { .. } => Err(StatusCode::UNAUTHORIZED),
    }
}

/// @emoji 📌️ `POST /spaces/{space_id}/documents/{document_id}/check-ins` — Check In. The author names
/// one committed head of this document's ledger; the hub materializes the checkpoint from its own
/// log and makes it active. Answers the job's status (`202` while it runs, `200` once terminal); a
/// retry of the same request answers the same job, or after a restart the same durable checkpoint.
async fn post_document_check_in(Path((space_id, document_id)): Path<(String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    if uri.query().is_some() || headers.get_all(axum::http::header::CONTENT_TYPE).iter().count() != 1 || headers.get(axum::http::header::CONTENT_TYPE).and_then(|value| value.to_str().ok()) != Some("application/json") {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let Some(source) = std::str::from_utf8(&body).ok() else { return StatusCode::BAD_REQUEST.into_response() };
    let Some(request) = DocumentCheckInV1::parse_canonical_json(source) else { return StatusCode::BAD_REQUEST.into_response() };
    if request.head.document_id != document_id {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let scope = DocumentScope::new(space_id, document_id);
    let (subject, user_id) = match check_in_author(&state, &scope, &headers).await {
        Ok(author) => author,
        Err(status) => return status.into_response(),
    };
    if state.artifact_authority.is_none() {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    let command_sha256 = os_directory::hex_lower(&Sha256::digest(source.as_bytes()));
    let key = DocumentCheckInKey { user_id: user_id.clone(), space_id: scope.space_id.clone(), document_id: scope.document_id.clone(), request_id: request.request_id.clone() };
    let job = match state.check_ins.admit(key.clone(), &command_sha256) {
        DocumentCheckInAdmission::Owner(job) => job,
        DocumentCheckInAdmission::Existing(job) => return check_in_status_response(job.status()),
        DocumentCheckInAdmission::Conflict => return StatusCode::CONFLICT.into_response(),
        DocumentCheckInAdmission::Unavailable => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    let claim = NewCheckpointPublicationClaimV1 { actor_user_id: user_id.clone(), correlation_id: request.request_id.clone(), command_sha256: command_sha256.clone(), claimed_at: now_ms() };
    match state.directory_service.claim_or_read_checkpoint_publication(&claim).await {
        Ok(CheckpointPublicationClaimV1::Claimed(_)) => {}
        Ok(CheckpointPublicationClaimV1::Existing(record)) if record.disposition == CheckpointPublicationDispositionV1::Completed => {
            let completed = match record.checkpoint_id {
                Some(checkpoint_id) => state.directory.get_artifact_checkpoint(&scope, checkpoint_id).await.ok().flatten(),
                None => None,
            };
            match completed.and_then(|checkpoint| Some((checkpoint.checkpoint_id, checkpoint.parent_checkpoint_id?, EditedArtifactFrontierV1::of_artifact_frontier(&checkpoint.baseline_frontier)?))) {
                Some((checkpoint_id, parent, baseline)) if baseline == request.head => job.finish_ready(checkpoint_id, parent, baseline),
                _ => {
                    state.check_ins.forget(&key);
                    return StatusCode::CONFLICT.into_response();
                }
            }
            return check_in_status_response(job.status());
        }
        Ok(_) => {
            state.check_ins.forget(&key);
            return StatusCode::CONFLICT.into_response();
        }
        Err(_) => {
            state.check_ins.forget(&key);
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    }
    let guard = CheckInClaimGuardV1 { service: state.directory_service.clone(), actor_user_id: user_id, correlation_id: request.request_id.clone(), command_sha256, complete: false };
    let accepted = job.status();
    tokio::spawn(run_document_check_in(state.clone(), subject, scope, request, job, guard));
    check_in_status_response(accepted)
}

/// @emoji 📣️ `GET /spaces/{space_id}/documents/{document_id}/check-ins/{request_id}` — one of the
/// caller's own Check Ins; another author's request id is indistinguishable from an unknown one.
async fn get_document_check_in(Path((space_id, document_id, request_id)): Path<(String, String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Response {
    if uri.query().is_some() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let scope = DocumentScope::new(space_id, document_id);
    let (_, user_id) = match check_in_author(&state, &scope, &headers).await {
        Ok(author) => author,
        Err(status) => return status.into_response(),
    };
    match state.check_ins.get(&DocumentCheckInKey { user_id, space_id: scope.space_id, document_id: scope.document_id, request_id }) {
        Some(job) => check_in_status_response(job.status()),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// @emoji 🛑️ `POST /spaces/{space_id}/documents/{document_id}/check-ins/{request_id}/cancel` — stops
/// the caller's own Check In at its next checkpoint; a published checkpoint is never withdrawn.
async fn post_document_check_in_cancel(Path((space_id, document_id, request_id)): Path<(String, String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    if uri.query().is_some() || !body.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let scope = DocumentScope::new(space_id, document_id);
    let (_, user_id) = match check_in_author(&state, &scope, &headers).await {
        Ok(author) => author,
        Err(status) => return status.into_response(),
    };
    match state.check_ins.get(&DocumentCheckInKey { user_id, space_id: scope.space_id, document_id: scope.document_id, request_id }) {
        Some(job) => {
            job.cancel();
            check_in_status_response(job.status())
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
//#endregion 📌️CheckIn
'''
rep("//#endregion 📣️CheckpointPublication\n", "//#endregion 📣️CheckpointPublication\n\n" + region)
rep('''        .route("/spaces/{space_id}/documents/{document_id}/active-checkpoint/pair", get(get_active_checkpoint_pair))
''', '''        .route("/spaces/{space_id}/documents/{document_id}/active-checkpoint/pair", get(get_active_checkpoint_pair))
        .route("/spaces/{space_id}/documents/{document_id}/check-ins", post(post_document_check_in).layer(DefaultBodyLimit::max(DOCUMENT_CHECK_IN_MAX_BYTES)))
        .route("/spaces/{space_id}/documents/{document_id}/check-ins/{request_id}", get(get_document_check_in))
        .route("/spaces/{space_id}/documents/{document_id}/check-ins/{request_id}/cancel", post(post_document_check_in_cancel).layer(DefaultBodyLimit::max(0)))
''')
rep('''AdminRecordedConnectionV1, ArtifactFrontier, ArtifactHash, ConnectionView,''', '''AdminRecordedConnectionV1, ArtifactFrontier, ArtifactHash, ConnectionView, DocumentCheckInPhaseV1, DocumentCheckInRefusalV1, DocumentCheckInStatusV1, DocumentCheckInV1, EditedArtifactFrontierV1, DOCUMENT_CHECK_IN_MAX_BYTES,''')
open(p, "w", encoding="utf-8").write(s)
print("ok")
