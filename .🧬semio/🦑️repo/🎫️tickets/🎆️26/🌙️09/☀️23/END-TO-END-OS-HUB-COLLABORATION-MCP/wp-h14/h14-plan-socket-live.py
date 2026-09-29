#!/usr/bin/env python3
"""🔌 H14 14c (coordinator 16:5x, C13 relay "a document socket idle ~2 min gets 4401 on its next Commands frame on p33"):
root cause = `document_plan_socket_validity` re-proved the plan's BOOTSTRAP facts — the catalog generation it was resolved in
and the active checkpoint its client seeded from — on every authorization tick (1 s), every admitted frame and every
broadcast, and answered a moved checkpoint `Unauthorized` → close `4401`. So every Check In (a member's, and since B1 the
declared checkpoint policy's) closed every other open socket of that document within one tick (7800 p33 capture: both
browsers' sockets and C13's probe socket closed right after each `server.document.check-in` ok), and a catalog
publication closed every plan socket of the hub; the Rust document link treats that close as "access revoked", the React
worker as a link loss (reconnecting, child suspended). Not time: no socket authority expires while idle.
Fix (hub only; the worker already handles every close and `RebootstrapRequired` transparently): the live authority of a
plan socket is its sealed plan, subject binding, descriptor, catalog SELECTION and a reached directory revision;
`document_plan_bootstrap_current` checks generation + checkpoint once, at grant consumption, and a plan they outdated
since its exchange is answered `409 socket-grant-stale` (grant rejected, the client re-plans; the Rust link counts 409
as a link failure, not a refusal). Laws: the consume law pins 409 + live authority unchanged for both bootstrap moves;
new quick law: a plan socket outlives a published checkpoint and ten idle minutes of ticks (paused clock) and its next
batch commits. Idempotent, region-guarded; `--dry-run` reports."""
import sys

R = "/Users/ueli/Documents/semio/🌎️hub"
SRC = f"{R}/🏗️bootstrap/🦀️.rs"
LAWS = f"{R}/🧪️tests/🔬️bin-unit/🦀️.rs"
DRY = "--dry-run" in sys.argv
files = {path: open(path, encoding="utf-8").read() for path in (SRC, LAWS)}
problems, states = [], []


def edit(path, old, new, label):
    text = files[path]
    if new in text and old not in text:
        states.append("done")
        return
    if text.count(old) != 1:
        problems.append(f"{label}: expected 1, found {text.count(old)}")
        states.append("problem")
        return
    files[path] = text.replace(old, new)
    states.append("replace")


edit(SRC, """/// 🔎️ The exact reason one document-open plan lost its socket authority. Eleven distinct refusals
/// collapse into close code `4401`; this names which. Test-only: the close frame is unchanged.
#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
enum DocumentPlanRefusalV1 {
    PlanInvalid,
    BindingMismatch,
    DescriptorMissing,
    DescriptorDiffers,
    CatalogSelectionDiffers,
    DirectoryRevisionDiffers,
    CheckpointDiffers,
}""", """/// 🔎️ The exact reason one document-open plan lost its socket authority. Six distinct refusals
/// collapse into close code `4401`; this names which. Test-only: the close frame is unchanged.
#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
enum DocumentPlanRefusalV1 {
    PlanInvalid,
    BindingMismatch,
    DescriptorMissing,
    DescriptorDiffers,
    CatalogSelectionDiffers,
    DirectoryRevisionDiffers,
}""", "refusal enum")

edit(SRC, """/// 🧭️ The pinned revision is a witness of issue order, not a freeze on the whole directory:
/// demanding equality made every outstanding plan die on the next directory append anywhere on
/// the hub — another space's invite, a rename, an unrelated membership edit — so a member's live
/// socket was closed 4401 by an event that had nothing to do with it. What a plan may never do is
/// claim a revision the directory has not reached: that pin is forged and stays refused. Whether
/// this caller still holds this scope is decided by membership and session revocation on their own
/// paths, which is what closes a removed member's socket in the very same exchange.
async fn document_plan_socket_validity(""", """/// 🧭️ The authority a document-open plan's socket holds for its whole life, re-proven before every
/// frame it admits or sends: the sealed plan, its subject binding, the document's descriptor, the
/// catalog's selection for that descriptor and surface, and a directory revision the directory has
/// reached. The pinned revision is a witness of issue order, not a freeze on the whole directory:
/// demanding equality made every outstanding plan die on the next directory append anywhere on
/// the hub — another space's invite, a rename, an unrelated membership edit — so a member's live
/// socket was closed 4401 by an event that had nothing to do with it. What a plan may never do is
/// claim a revision the directory has not reached: that pin is forged and stays refused. Whether
/// this caller still holds this scope is decided by membership and session revocation on their own
/// paths, which is what closes a removed member's socket in the very same exchange. The plan's
/// bootstrap facts are no authority: [`document_plan_bootstrap_current`] checks them once, at admission.
async fn document_plan_socket_validity(""", "live authority doc")

edit(SRC, """    let Some(catalog) = state.openable_catalog.as_ref() else { return SocketBindingValidityV1::Unavailable };
    if catalog.generation_id() != authority.catalog.generation_id
        || catalog.resolve_document_open(&descriptor, Some(&authority.surface.surface_id), authority.grant.write).is_none_or(|selection| {
            selection.package != authority.package
                || selection.artifact != authority.artifact
                || selection.parent_dialect != authority.parent_dialect
                || selection.surface != authority.surface
                || selection.browser_actor != authority.browser_actor
                || selection.grant != authority.grant
        })
    {
        #[cfg(test)]
        return record_document_plan_refusal(DocumentPlanRefusalV1::CatalogSelectionDiffers);""", """    let Some(catalog) = state.openable_catalog.as_ref() else { return SocketBindingValidityV1::Unavailable };
    if catalog.resolve_document_open(&descriptor, Some(&authority.surface.surface_id), authority.grant.write).is_none_or(|selection| {
        selection.package != authority.package
            || selection.artifact != authority.artifact
            || selection.parent_dialect != authority.parent_dialect
            || selection.surface != authority.surface
            || selection.browser_actor != authority.browser_actor
            || selection.grant != authority.grant
    }) {
        #[cfg(test)]
        return record_document_plan_refusal(DocumentPlanRefusalV1::CatalogSelectionDiffers);""", "live selection without generation")

edit(SRC, """    let checkpoint = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_active_artifact_checkpoint(&authority.scope)).await {
        Ok(Ok(checkpoint)) => checkpoint.map(document_open_checkpoint),
        Ok(Err(_)) | Err(_) => return SocketBindingValidityV1::Unavailable,
    };
    if checkpoint.as_ref() != Some(&authority.checkpoint) {
        #[cfg(test)]
        return record_document_plan_refusal(DocumentPlanRefusalV1::CheckpointDiffers);
        #[cfg(not(test))]
        return SocketBindingValidityV1::Unauthorized;
    }
    SocketBindingValidityV1::Active
}
""", """    SocketBindingValidityV1::Active
}

/// 🌱️ Whether a document-open plan's bootstrap facts are still the hub's current ones.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DocumentPlanBootstrapV1 {
    Current,
    Stale,
    Unavailable,
}

/// 🌱️ Whether a document-open plan still names the catalog generation it was resolved in and the
/// active checkpoint its client seeds from. Checked once, when its socket grant is consumed, because
/// the client bootstraps from exactly that checkpoint pair. A Check In — a member's or the declared
/// checkpoint policy's — and a catalog publication advance them without changing anyone's authority,
/// so a live socket never re-checks them (ticket 26/09/23 session 14c: every Check In closed every
/// other open socket of its document `4401` within one authorization tick), and a plan they outdated
/// since its exchange is answered stale, never as a revocation.
async fn document_plan_bootstrap_current(state: &HubState, record: &SocketGrantRecordV1) -> DocumentPlanBootstrapV1 {
    let Some(authority) = record.document_plan.as_deref() else { return DocumentPlanBootstrapV1::Current };
    let Some(catalog) = state.openable_catalog.as_ref() else { return DocumentPlanBootstrapV1::Unavailable };
    if catalog.generation_id() != authority.catalog.generation_id {
        return DocumentPlanBootstrapV1::Stale;
    }
    let checkpoint = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_active_artifact_checkpoint(&authority.scope)).await {
        Ok(Ok(checkpoint)) => checkpoint.map(document_open_checkpoint),
        Ok(Err(_)) | Err(_) => return DocumentPlanBootstrapV1::Unavailable,
    };
    if checkpoint.as_ref() == Some(&authority.checkpoint) {
        DocumentPlanBootstrapV1::Current
    } else {
        DocumentPlanBootstrapV1::Stale
    }
}
""", "bootstrap currency")

edit(SRC, """/// 🧭️ Admits a document upgrade whose credential resolved to `subject`: the oldest pending
/// document grant of that binding and audience, revalidated against its sealed plan, consumed once.
async fn consume_document_socket_grant(state: &HubState, subject: &SocketSubjectV1, audience: SocketAudienceV1, surface: Option<&str>) -> Result<SocketGrantAdmissionV1, (StatusCode, &'static str)> {
    let candidate = state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).map_err(|_| (StatusCode::UNAUTHORIZED, "socket-grant-pending"))?;
    match document_plan_socket_validity(state, &candidate, surface).await {
        SocketBindingValidityV1::Active => state.socket_grants.consume(&candidate, now_ms()).map(|record| SocketGrantAdmissionV1 { record }).map_err(|_| (StatusCode::UNAUTHORIZED, "socket-grant-consume")),
        SocketBindingValidityV1::Unauthorized => {""", """/// 🧭️ Admits a document upgrade whose credential resolved to `subject`: the oldest pending
/// document grant of that binding and audience, revalidated against its sealed plan, consumed once.
/// A plan whose bootstrap a Check In or a catalog publication outdated since its exchange is
/// answered `409` and its grant rejected: nobody's access changed, the client re-plans.
async fn consume_document_socket_grant(state: &HubState, subject: &SocketSubjectV1, audience: SocketAudienceV1, surface: Option<&str>) -> Result<SocketGrantAdmissionV1, (StatusCode, &'static str)> {
    let candidate = state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).map_err(|_| (StatusCode::UNAUTHORIZED, "socket-grant-pending"))?;
    match document_plan_socket_validity(state, &candidate, surface).await {
        SocketBindingValidityV1::Active => match document_plan_bootstrap_current(state, &candidate).await {
            DocumentPlanBootstrapV1::Current => state.socket_grants.consume(&candidate, now_ms()).map(|record| SocketGrantAdmissionV1 { record }).map_err(|_| (StatusCode::UNAUTHORIZED, "socket-grant-consume")),
            DocumentPlanBootstrapV1::Stale => {
                state.socket_grants.reject_pending(&candidate.selector);
                Err((StatusCode::CONFLICT, "socket-grant-stale"))
            }
            DocumentPlanBootstrapV1::Unavailable => Err((StatusCode::SERVICE_UNAVAILABLE, "socket-grant-503")),
        },
        SocketBindingValidityV1::Unauthorized => {""", "consume stale")

edit(LAWS, """    issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:checkpoint").await;
    publish_checkpoint_for_test(&state, STUDIO, &document_id).await;
    assert!(matches!(consume_document_socket_grant(&state, &subject, audience.clone(), Some("surface.test.editor")).await, Err((StatusCode::UNAUTHORIZED, _))));
    assert!(state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).is_err(), "revision/checkpoint change terminally rejects the pending grant");

    issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:catalog").await;
    state.openable_catalog = Some(document_open_catalog_for_descriptor_with_generation(&descriptor, "77".repeat(32)));
    assert!(matches!(consume_document_socket_grant(&state, &subject, audience.clone(), Some("surface.test.editor")).await, Err((StatusCode::UNAUTHORIZED, _))));
    assert!(state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).is_err(), "catalog change terminally rejects the pending grant");
""", """    issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:checkpoint").await;
    let checked_in = state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).expect("pending grant before the Check In");
    publish_checkpoint_for_test(&state, STUDIO, &document_id).await;
    assert_eq!(document_plan_socket_validity(&state, &checked_in, Some("surface.test.editor")).await, SocketBindingValidityV1::Active, "a Check In changes nobody's authority");
    assert_eq!(document_plan_bootstrap_current(&state, &checked_in).await, DocumentPlanBootstrapV1::Stale);
    assert!(matches!(consume_document_socket_grant(&state, &subject, audience.clone(), Some("surface.test.editor")).await, Err((StatusCode::CONFLICT, "socket-grant-stale"))));
    assert!(state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).is_err(), "a checkpoint published since the exchange rejects the pending grant as stale");

    issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:catalog").await;
    let rotated = state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).expect("pending grant before the catalog publication");
    state.openable_catalog = Some(document_open_catalog_for_descriptor_with_generation(&descriptor, "77".repeat(32)));
    assert_eq!(document_plan_socket_validity(&state, &rotated, Some("surface.test.editor")).await, SocketBindingValidityV1::Active, "a catalog generation with the same selection changes nobody's authority");
    assert_eq!(document_plan_bootstrap_current(&state, &rotated).await, DocumentPlanBootstrapV1::Stale);
    assert!(matches!(consume_document_socket_grant(&state, &subject, audience.clone(), Some("surface.test.editor")).await, Err((StatusCode::CONFLICT, "socket-grant-stale"))));
    assert!(state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).is_err(), "a catalog generation published since the exchange rejects the pending grant as stale");
""", "consume law")

edit(LAWS, """    #[tokio::test]
    async fn retained_short_admin_request_drop_duplicate_cancel_and_secret_lifecycle_is_exact() {""", """    /// 🔌️ A document socket's authority is its session, membership and sealed plan, never the checkpoint its client seeded
    /// from: a checkpoint published while the socket is open (a member's Check In, or the declared checkpoint policy's) leaves
    /// it open through ten idle minutes of authorization ticks, and its next batch commits and is acknowledged (ticket
    /// 26/09/23 session 14c: on p33 C13's probe socket and both browsers' sockets closed `4401` within one tick of every Check In).
    #[test]
    fn a_plan_socket_outlives_a_check_in_and_ten_idle_minutes_and_its_next_batch_commits() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            state.presence_clock = Some(Arc::new(TestPresenceClock::new()));
            let token = seed_author_token(&state).await;
            let document_id = artifact_document_id_for_test("plan-socket-idle");
            seed_genesis_for_document_for_test(&state, &token, STUDIO, &document_id, "plan-socket-idle").await;
            let scope = DocumentScope::new(STUDIO, &document_id);
            let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor");
            install_document_open_catalog_for_test(&mut state, &descriptor);
            let (plan, receipt) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:plan-socket-idle").await;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/scopes/{STUDIO}%2F{document_id}/document/ws?surface={}", plan.surface.surface_id);
            let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("plan socket upgrade");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            assert!(matches!(next_server_frame_at(&mut socket, "welcome").await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame_at(&mut socket, "session").await, ServerFrame::Session { actor, .. } if actor == receipt.actor_id));
            let published = publish_checkpoint_for_test(&state, STUDIO, &document_id).await;
            let active = state.directory.get_active_artifact_checkpoint(&scope).await.expect("active read").expect("active checkpoint");
            assert_eq!(active.baseline_frontier, published.baseline_frontier, "the socket's plan names a superseded checkpoint");
            tokio::time::pause();
            for _ in 0..600 {
                tokio::time::advance(std::time::Duration::from_secs(1)).await;
                tokio::task::yield_now().await;
            }
            tokio::time::resume();
            let document = db_artifact_id(&scope);
            let before = state.db.document(&document).await.expect("document handle").frontier().await.expect("frontier before").head_seq;
            let mut envelope = sample_envelope("after-ten-idle-minutes", &WireArtifactId(document_id.clone())).await;
            envelope.actor = ActorId(receipt.actor_id.clone());
            socket.send(client_binary(&ClientFrame::Commands { batch_id: 600, envelopes: vec![envelope] }, Lane::Command).await).await.expect("batch after ten idle minutes");
            match next_server_frame_at(&mut socket, "Ack after ten idle minutes").await {
                ServerFrame::Ack { batch_id: 600, stages, .. } => assert!(stages.iter().any(|stage| matches!(stage, AckStage::Applied { outcome } if matches!(outcome.as_ref(), ApplyOutcome::Accepted))), "the batch commits: {stages:?}"),
                other => panic!("the batch after ten idle minutes was answered {other:?}"),
            }
            let after = state.db.document(&document).await.expect("document handle").frontier().await.expect("frontier after").head_seq;
            assert_eq!(after, before + 1, "the batch is durable");
        });
    }

    #[tokio::test]
    async fn retained_short_admin_request_drop_duplicate_cancel_and_secret_lifecycle_is_exact() {""", "idle law")

if "CheckpointDiffers" in files[SRC]:
    problems.append("CheckpointDiffers still referenced")

print(f"states {states}")
if problems:
    print("PROBLEMS:\n  " + "\n  ".join(problems))
    sys.exit(1)
if DRY:
    print("dry-run clean")
elif "replace" in states:
    for path, text in files.items():
        open(path, "w", encoding="utf-8").write(text)
    print("applied")
else:
    print("nothing to apply")
