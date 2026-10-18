//! 🌀️ Poll kernel turn of the reactor turn: the one reactor turn that polls the kernel, routes events, drives retirement, reconciliation and publication and fills the turn patch. See [`super`].

use super::*;

/// ↩️ One `Effect::Respond` per inbound `Event::Request` this turn served — kept apart from the
/// document-backbone effects so the answer's ordering against them is explicit rather than
/// incidental (answers trail, so a request that also wrote the document publishes the write first).
///
/// 🚪️ A close-cleanup verdict names ONE already-retired lifetime, so it may not kill the turn that
/// every OTHER instance in this actor shares.
///
/// 🐛️ It did: `?` here turned instance 5's `plugin.internal.zero-progress` into a shard worker fault
/// for the whole turn, the host trapped the actor, and instance 6 — a sibling pane mid-boot in the
/// same shard — went to Plugin Recovery with it (six-pane demonstrator boot, ticket 26/08/28). The
/// instance whose cleanup faulted is already gone and has already left quarantine; what is left is a
/// diagnostic, not a live window's crash.
///
/// 🌡️ One fair live-maintenance step per turn — and a burst while the stepped instance reports
/// displaced-owner pressure, bounded by the turn's retirement deadline. A results-window
/// playback (a coalesced window-config amend per frame, three displaced owners each) outran the
/// single step's one owner per 26-stage rotation and saturated its 1 024-slot queue after
/// ~340 frames (ticket 26/09/16/FEM-3D-INTERACTIVE-FEATURE-COMPLETE).
///
/// 🎯️ M1 (ticket 26/08/17 `design-unified.md`): decodes the pack-encoded
/// `ui_contract::UiIntent`, drops it if it targets a tree the user can no longer see (the
/// revision guard, at the reconciler that owns the revision — `PATCHES.revision`,
/// `ui_runtime::is_stale_intent` imported rather than reimplemented), and otherwise
/// batches it per instance for the dispatch pass below (mirrors `app_commands`'
/// batch-then-dispatch shape). Real dispatch replaces the prior packet's "decode-and-
/// mark-dirty" interim — see `📓️terra-sdk-wire-report.md`'s M1 section for the full route.
///
/// 🔁️ Two delivery shapes share one id space. A `RequestRegistry::request` id parks a
/// future and is woken by `resolve`; a `request_continuation` id parks NOTHING and is
/// answered by redispatching its `response_action` into the owning app instance with the
/// outcome merged onto the original request object. Before this branch existed the second
/// shape did not: a plugin that wanted an extension result had to hand-mint a `RequestId`,
/// which owns no registry slot, so `resolve` silently dropped every extension outcome.
///
/// 📄️ A host answer larger than one `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` page arrives
/// as PROLOGUE pages here and its terminal page rides the `Event::Completed` above, so
/// `Event::Completed` stays THE one completion door for a `req` and this arm never needs
/// a second dispatch site in the turn generator. A continuation claims its own pages;
/// everything else falls through to the parked-future accumulator.
///
/// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): used to discard every
/// non-final chunk outright (`if done { resolve(req, Ok(bytes)) }` — every earlier
/// `bytes` was simply dropped on the floor, silent data loss for any multi-chunk
/// response). `append_chunk` accumulates instead; `cap` is the owning instance's
/// `QuotaSchema.message_bytes` (default 64 MiB when unset/unknown — matches
/// `instance_task_quota`'s own `unwrap_or` fallback idiom above).
///
/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1, design-abi.md §4): a job spawned
/// through `host::jobs::spawn` (`🌐host/🦀️.rs`) allocates its `job` id from
/// THE SAME `RequestRegistry` counter as every other awaitable `host::*` call — the
/// `Effect::SpawnJob{job, ..}` this actor emitted carried `job == req.0` — so
/// `Event::JobCompleted{job, result}` resolves the identical parked `RequestFuture`
/// an `Event::Completed{req, result}` would, closing the "no `req`-per-job
/// correlation table yet" gap `📓️terra-M5-report.md` §4 named (no separate table
/// needed: the request id already IS the job id).
///
/// 📥️ The inbound half of the `request`/`respond` seam (`📜️.wit`'s `request-event`,
/// "every 'someone else calls INTO this actor' seam is now one inbound `request`,
/// answered with the `respond` effect within a bounded number of turns"). This actor's
/// installed `ExtensionBundle` is the ONE capability table that answers it, so the
/// request is served inline and answered on the SAME turn — an extension handler is a
/// pure `Fn(&[u8]) -> Result<Vec<u8>, Fault>` with nothing to await. An actor with no
/// bundle answers the bundle's own typed refusal (`extension.inactive`/
/// `extension.missing`), never silence: dropping the event stranded every caller's
/// parked request forever (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 📥️ A page that names a different command on THIS instance is the host saying it has
/// moved on: it drives one command per instance at a time, so the owner retained here can
/// no longer have a driver. It is superseded into the retiring lane (see
/// `COMMAND_INGRESS_RETIRING_SLOT`) and the host is asked for this page once more, which
/// the now-free live lane admits on the next turn. A page for ANOTHER instance is genuine
/// backpressure — that owner's host driver is still live — and waits as it always did.
///
/// 📥️ Taken only when it IS the assembly this page belongs to. An `if let` on a bare
/// `retained.take()` dropped every other owner shape on the floor — the page went with it,
/// the turn still answered `Idle`, and the host's drain then spun its whole continuation
/// ceiling asking a reactor that no longer owned anything.
///
/// 📥️ The terminal arm this chain never had: a page that matches no admission shape is a
/// page nobody owns, and it says so by name instead of leaving the turn `Idle` with the
/// page silently gone.
///
/// 🎯️ M1: surviving intents dispatch through the SAME `route_app_frame`/effects/events plumbing
/// as `app_commands` above, immediately after it (so a mutation an app command made this turn is
/// already visible to the intent's own dispatch) — via the NEW `plugin_dispatch_intents`, which
/// routes through the app's EXISTING typed command path (`PluginApp::handle_intent_frame` →
/// `ArtifactApp::command_from_intent` → `dispatch_typed_command_inner`), never a parallel path.
/// Each handled batch's surfaces feed the retained render set so the reply patch — the next `UiPatch`
/// revision bump — is produced in this SAME turn (design decision: no new reply channel).
///
/// 🚫️async: E5 executor bridge — `plugin_dispatch_intents` stays genuinely `async fn`; safe to
/// resolve synchronously here for the same reason as `plugin_exchange` above.
///
/// 🌳️ Deduped per instance — several intents on the same surface this turn must not queue a
/// redundant re-render (the second `diff()` would return `None` anyway, but there is no reason
/// to pay for it).
///
/// 🧹️ The SECOND publication retirement of this turn, and the one that matters for the crossing
/// count: the first ran before the events, so every `Event::PatchAck` this turn carried left its
/// slot `Acknowledged` with nothing to retire it until the NEXT turn — one host round trip per
/// published surface whose only work was freeing a slot (the `<n>:ea---` half of the two-state
/// ping-pong, `📓️reactor-reconcile-spin-2026-09-14.md` §1). Bounded by the same units and the same
/// wall deadline as the first run, so a turn cut short still answers `MoreWork` through
/// `PendingPatchAuthority::has_retiring` and finishes on the next one.
///
/// 👥️ M2 (ticket 26/08/17 `design-unified.md`): `now_ms` is read ONCE for both `record_peer`'s
/// expiry stamping below and `PRESENCE.expire` at the end of this turn — a single wall-clock
/// reading per poll, not one per presence update.
///
/// 🕹️ A refused commit THROWS AWAY the tree this turn just rendered and leaves the
/// surface at its previous revision, which the host then reads back as `unchanged` —
/// indistinguishable from a healthy refresh. It is the one exit on this path that can
/// lose a render the host explicitly asked for, so it is recorded as a shell fault
/// naming the surface rather than a `let _ =` (ticket 26/09/02/PUZZLE-3D-END-TO-END
/// wave B46 §8.1, wave B48 §3). Not a turn-fatal `?`: a commit refused because the
/// instance is closing is an ordinary teardown race, and faulting the whole turn for
/// it would break the close ladder.
///
/// 🕹️ A surface whose previous reconcile is still in flight cannot be re-rendered now, so the
/// dirty render is DEFERRED — and a deferred ring with no room left would drop it, which is a
/// render the host asked for that no later turn ever performs. That is exactly the shape of
/// "the guest never re-publishes its selection lane after a pick" (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B46 §8.1: `interactionSelect` settles, no verdict is
/// recorded, and `data-guest-selection-json` stays `selectedIds:[]` for 150 s), so it is a
/// typed fault naming the surface rather than a `let _ =`.
///
/// 🎟️ A refusal by one of the PROCESS-WIDE reconcile tables is not the ordinary "previous
/// reconcile still in flight" race the defer ring exists for: it says this surface cannot be
/// admitted at all until some other surface hands credit back, and while the reservation was
/// priced at the per-surface CEILING that was structural — three surfaces of a thirteen-surface
/// session, refused/deferred/re-dirtied for ever with `effects=0` (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B56 §1.1, wave B58). Priced by its own body it should
/// never happen, so it is reported by NAME, once per refusal, instead of deferring in silence.
///
/// 🚫️async: E5 executor bridge (× 2) — `LocalExecutor::{run_until_idle,has_ready}` stay
/// genuinely `async fn` (its own doc: "run_until_idle handles Pending without ever yielding
/// its own future" — matches `⚛️reactor/💼️jobs`'s identical use of this exact bridge).
///
/// 🧵️ The COLD executor's own slice of the same hold: every `AsyncTask` `spawn_task` admitted
/// lives on `TASK_EXECUTOR`, and until the task lane became a product lane nothing but a native
/// fixture ever polled it — a spawned task would have parked forever inside a real guest. Its
/// answer is "some task is still alive", ready or parked, so it is folded into `more_work` the
/// same way `REACTOR_EXECUTOR`'s is.
///
/// 🔁️ The process-pool pump stays gated on the REACTOR executor alone. `TASK_EXECUTOR`'s answer
/// is "some task is still alive", ready OR PARKED, so folding it in here stopped pumping the
/// pool for as long as any task was parked on a host round trip — which starved every
/// interactive job step behind it (measured 2026-09-22: a command page set never reached its
/// terminal ingress status in 512 turns).
///
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): resumed `AsyncTask` follow-
/// ups (and any replayed `task_restarts` from a `restore` before this turn) — AFTER
/// `run_until_idle` so a task that resolved just now is redispatched the SAME turn, not the
/// next one. A resume can itself spawn more tasks (`dispatch_emit` runs for real), so the
/// executor may have fresh ready work by the time this returns — folded into `more_work` below
/// rather than requiring a second `run_until_idle` pass this turn (the next `poll` picks it up).
///
/// 🔒️ `typed_operation_scan.contended` is deliberately NOT folded in. A busy instance lock is not
/// an answer of "there is runnable work", it is "another owner is mid-step and I could not look" —
/// and the guest is single-threaded, so on wasm it can only ever be a reader inside this very turn.
/// Answering `MoreWork` for it turned every concurrent read into a host turn round trip that
/// produced nothing: 12 of 512 idle generation3d turns, measured 2026-09-10. Every owner that can
/// hold that lock is itself either a reactor executor task (`executor_pending`), a task resume
/// (`resumes`), an ingress command (`command_ingress`) or a lifecycle step (`lifecycle`), each of
/// which already arms this turn on its own account. See `📓️idle-turns-2026-09-10.md`.
///
/// 👥️ M2: once per poll — expire ages-out peer marks, then flush drains every key touched since
/// the last flush into one coalesced `PresenceUpdate` each (free burst coalescing: a hover storm
/// between polls still costs exactly one update per `(surface, node_key)`).
#[expect(clippy::result_large_err, reason = "Patch reservation and publication callbacks return the original fixed surface or patch owner on refusal; these transfers must not allocate an error wrapper.")]
pub(super) async fn poll_kernel_turn<PA: semio_framework_plugin_reactor_app::PluginApp, T, Prepared>(
    runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,
    events: &mut Option<Event>,
    command_page: Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
    cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>,
    budget: semio_framework::kernel::Budget,
    prepare: impl FnOnce(&semio_framework::kernel::TurnResult) -> Result<Prepared, semio_framework::Fault>,
    publish: impl FnOnce(semio_framework::kernel::TurnResult, Prepared) -> T,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    cx:&mut semio_framework_job::StepContext<'_>,
) -> Result<T, semio_framework_job::OriginalTurnFailure> {
    admit_turn_entry(budget, cx)?;
    let original_progress=cx.retained_progress();let original_work=cx.original_work_units_spent();
    let retryable_lifecycle = command_page.is_none() && cold_pair_page.is_none() && events.iter().all(|event| matches!(event, Event::InstanceOpen { .. } | Event::InstanceClose(_) | Event::InstanceLifecycleAck(_)));
    let mut dirty = DirtyPollOwners::new();
    let focus = admit_lifecycle_focus(runtime, events)?;
    let close_instances: Vec<u32> = events.iter().filter_map(|event| if let Event::InstanceClose(request) = event { Some(request.lifetime.instance_id) } else { None }).collect();
    let mut effects: Vec<Effect> = Vec::new();
    trace_turn_phase_retention("lifecycle");
    let close_cleanup_work = drive_turn_retirement(runtime, cx)?;
    semio_framework_plugin_runtime_instances::inst_opening::plugin_resume_actor_openings(runtime,identity,cx).await?;
    PATCHES.with(|patches| redirty_acknowledged_deferred_surfaces(patches, &mut dirty,cx))?;
    redirty_arena_starved_surfaces(&mut dirty)?;
    let selected_event = select_turn_event(runtime, events, cx)?;
    dispatch_turn_event(runtime, selected_event, &mut dirty, &mut effects, identity, cx).await?;
    let extension_retirement_work = drive_extension_turn(runtime, &mut effects, cx).await?;
    let close_cleanup_work = close_cleanup_work || extension_retirement_work;
    let cold_pair_ingress = drive_cold_pair_ingress(runtime, cold_pair_page, identity, cx).await;
    let cold_pair_loading = matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_));
    let document_load_work = step_restored_document_loads(runtime, &mut effects, identity, cx).await || COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow().is_some());
    let command_ingress = drive_command_ingress(runtime, command_page, close_instances, &mut effects, identity, cx).await?;
    advance_original_receiving(runtime, &mut dirty, &mut effects, identity, cx).await?;
    step_pending_patch_close(cx)?;

    trace_turn_phase_retention("ingress");
    let (continuation, typed_operation_scan) = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_continue_typed_operations(runtime, semio_framework_plugin_runtime_ops::ops_typed_operations::TypedOperationGrant::turn(budget,cx), identity,cx).await?;
    let typed_operation_contended = typed_operation_scan.contended;
    if let Some((instance, output)) = continuation {
        route_exchange_output(instance, output, &mut effects,cx)?;
    }

    trace_turn_phase_retention("continuation");
    let now_ms = u64::try_from(semio_framework_plugin_actor_host::now_ms().await).unwrap_or(0);

    trace_turn_phase_retention("presence-clock");
    render_dirty_surfaces(runtime, dirty, now_ms, &mut effects, cx).await?;
    let reconcile_work = drive_turn_reconcile(&mut effects, cx)?;
    let executors = drive_turn_executors(&mut effects);
    let resumes_remain = drain_task_resumes(runtime, &mut effects, 64, identity, cx);
    effects.extend(semio_framework_plugin_runtime_ops::ops_document::plugin_drain_document_backbones(runtime)?);
    let more_work = settle_turn_work(runtime, TurnWorkInputs { events_pending: events.is_some(), executors, close_cleanup_work, typed_operation_runnable: typed_operation_scan.runnable, typed_operation_contended, reconcile_work, resumes_remain, document_load_work }, &effects)?;

    let lifecycle_receipt = focus.map(|instance| runtime.guest_lifetimes.borrow_mut().prepare_turn(instance, &ReactorClosePeers)).transpose()?.flatten();
    let (ui_patches, ui_patch_receipt) = if cold_pair_loading { (semio_framework::kernel::UiTurnPatches::default(), None) } else { fill_turn_patch_page(runtime, turn_patch_budget_bytes(budget))? };
    let presence = PRESENCE.with(|hub| {
        let mut hub = hub.borrow_mut();
        hub.expire(now_ms);
        hub.flush()
    });
    let status = if more_work { TurnStatus::MoreWork } else { TurnStatus::Idle };

    let retained_receipt=budget.retained.return_original(original_turn_progress(original_progress,cx.retained_progress())?,u64::try_from(cx.original_work_units_spent()-original_work).map_err(|_|reactor_close_fault("original turn work exceeds fixed receipt extent"))?)?;
    let result = semio_framework::kernel::TurnResult { retained_receipt, ui_patches, effects, presence, next_wake: ARMED_TIMERS.with(|timers| timers.borrow().first()), status, fuel_used: 0, command_ingress, cold_pair_ingress, lifecycle_receipt, ui_patch_receipt };
    publish_turn_result(runtime, result, focus, retryable_lifecycle, prepare, publish)
}
