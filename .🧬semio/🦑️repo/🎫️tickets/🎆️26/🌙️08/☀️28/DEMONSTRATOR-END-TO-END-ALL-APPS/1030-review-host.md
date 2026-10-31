# 1030 review: host / OS side (waves 1028-1030)

STATUS: DONE. Read-only, no builds, no sub-agents. Scope: uncommitted Rust diff under `🧰️framework/🛍️products/💻️os/**`, `🌎️hub/**`, kernel history, plus the committed task-resume and shard-pump code it depends on.
Most of the OS diff is docstring-emoji churn and relocation; the logic changes are the ones below. Only items I re-read in source are listed. M = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`.

## H1 (medium, 55%) deterministic guest trap on an event: no fault frame reaches the shell, retry slot wedged
`M/🖥️host/🧵️shard/🎬️shard-loop-frame-turns-and-job-control/🦀️.rs:~126` (`Err(fault)=>{original.retain_trap(fault);return Err(...)}`) and `M/🖥️host/🧵️shard/🌀️shard-loop-registry-and-pump/🦀️.rs:~180-183`.
Mechanism: on a non-`ActorCall` trap the executor retains the trap and returns `Err` without `send_outcome(ShardOutcome::Fault)` (the `Ok` path and the `ActorCall` path do send). The pump then does `retain_lifecycle_retry(owner)?` and propagates the error. The retry re-enters `execute_turn_for`, whose `checkout_guest` is refused ("owner and checkout remain held"). The shell never receives a fault frame for that actor; the retry slot is consumed until close.
Fix direction: after `retain_trap`, send `ShardOutcome::Fault` for the actor and unregister (as the cancel-job failure arm already does), or make the retained trap terminal for the lifecycle retry.
Owner: runtime.

## H2 (medium, 55%) `ColdReplayProjection::advance` clones the whole base snapshot but charges only `size_of::<S>()`
`🏪️store/📬️paged-one-item/❄️cold-apply/🦀️.rs` (`ColdReplayProjection::advance`, `demands`, `replay_header::<S>`).
`*self.clone = Some(source.get().clone())` is a deep clone in one turn, priced as one shallow header (`copy_bytes:size_of::<S>()`). The bounded factory path in the same file meters `snapshot_bytes`; the replay path does not. A large document replays as an unmetered, uncancellable clone, so the per-turn budget and cancellation law is silently broken (the "short connection shortage must not freeze" requirement).
Fix direction: take a `ColdPreparationBounds`-style `snapshot_bytes` into the replay factory and charge/refuse it as the bounded preparation path does.
Owner: runtime (resid for the pricing).

## H3 (medium, 50%) `ColdReplayEdit::advance` leaves the post root half-applied on error, then is retry-able
Same file, `ColdReplayEdit::advance`. `apply_diff_cold_into(...)?` mutates `post` in place; on `Err` `self.applied` stays `false`. A caller that re-polls `advance` after a `Progress`/refusal re-applies the diff on an already partially mutated post (double application). The computed `inverse` (an owned `M`-typed value) is also dropped on that path without retirement.
Fix direction: set a failed/poisoned flag before applying and refuse every further `advance`; retire `inverse` on the error path.
Owner: runtime.

## H4 (medium, 45%) `BackboneDocument.transitions` is now an `ArtifactHistoryLedger` with a panicking `Drop`, but the document has no retire path
`🖥️host/🔌️plugin-host/🗃️backbone-document/🦀️.rs:29,79,91,135,163,185,290` with `🌿️vcs/🗃️history-ledger/🦀️.rs:755-759` (`Drop` asserts `terminal_is_empty()` unless panicking).
`BackboneDocument` derives `Clone, Debug, PartialEq` and has no `Drop` or retire. Its `edit_messages` are drained by hand (`pop` loops in `decode_backbone_payload` and `space-catalog:146`) so that no ledger is dropped full, but `transitions` is moved in non-empty. Dropping any decoded/cloned document that has history transitions (`workflow-store` `document()` clone, `encode_backbone_payload` consumers, error returns after decode) hits the assertion, which aborts a wasm guest. `vcs.edits` is the same pattern, so check whether a wrapper retires it; I found none in this file.
Fix direction: add `BackboneDocument::retire`, or keep transitions a plain `Vec` and build the ledger only inside `backbone_envelope_of`.
Owner: runtime.

## H5 (low-medium, 45%) reserved producer steps/borrows through a raw pointer after releasing its lock
`🔌️plugin/📜️declaration/👷️job/🧱️ownership/📦️producer/🦀️.rs` (`step`, `borrow_outcome`; moved verbatim from `reserved-tool-job`).
`state.inner` is turned into `*mut dyn`, the `try_lock` guard is dropped, then the job is stepped or borrowed. `ArtifactReservedToolJob` is `Clone` (shared `Arc`), so another clone's `close_step` (`state.inner.take()` drop) can free the box while a `JobOutcomeView<'a>` from `borrow_outcome` is still live. Today both run on one actor thread sequentially, which limits this to an interleaving across turns of the same view lifetime. Two clones are reachable in the ActionBus path. Also each clone's `Drop` asserts `state.is_none()`, so a clone dropped without its own `close_step` panics.
Owner: runtime (the new ownership crate).

## H6 (low, 50%) task-resume wire output silently discarded on decode failure
`⚛️reactor/🔄️turn/📬️task-resumes-and-wire-effects/🦀️.rs` (`drain_task_resumes`): `if let Ok(effect)=decode_wire_effect(one)` and `if let Ok(event)=decode_wire_app_event(one)` drop undecodable effects and events with no fault frame. A `Command` resume that yields only an undecodable effect looks like success (`delivering=true`), and the user's action disappears. Compare the `Fault` resume arm and `push_admitted_effect`, which do surface a shell fault.
Fix direction: on `Err`, push `shell_fault_effect(instance, plugin.resume-output-undecodable)`.
Owner: runtime.

## H7 (low, 45%) resume for a non-live instance is re-queued forever
Same function: `if native_close_key(...).is_err(){ push_admitted(resume); continue }` then returns `!TASK_RESUMES.is_empty()` as more-work. For an instance with "no acknowledged live lifetime" (never live, or already retired without `cancel_instance_step` having run) the resume stays queued and `more-work` stays true on every poll, spinning the host loop. The queue itself is bounded (`REACTOR_RESUME_SLOTS`), so this is a livelock, not growth. Resolves only if instance close always runs `begin_cancel_instance`.
Owner: runtime.

## H8 (low, 50%) GIS inference runs under `StepBudget::new(u64::MAX, u64::MAX, grant)` with a never-cancelled token
`🌎️hub/🧩️compositions/🌍️gis/💡️inference/🦀️.rs` (`run_controlled`, `execution_request`): fuel and deadline are unbounded, `cancelled:false`, `root_cancel_token()` fresh. Cancellation works only if the executable calls the passed `checkpoint` closure; `deterministic_map_inference` passes `|_| Ok(())`. Only `retained` bytes are bounded. The hub job budget (`work_units`, wall deadline) is not enforced by this call.
Owner: fwfix / bundle (gis).

## H9 (low, 40%) `member_stores` now fallible: partial stores dropped unretired
`🌎️hub/🧩️compositions/🌍️gis/💡️inference/🦀️.rs` (`member_stores`, committer `:~759`): if `drawing_store(...)?` or `value_store(...)?` fails after `parent_store` was built, the parent `ArtifactStore` and snapshots are dropped without the owner retirement the rest of the code enforces. A fresh genesis store has empty ledgers, so it is likely benign. Also `funded_bounded_artifact_store_owners` is admitted three times per commit, under the `documents` lock.
Owner: bundle.

## Checked, no finding
- One event per poll: guest `poll_wit`/bridge refuses a second event; wasmtime `execute_turn` stages then polls once; shard sends `from_ref(event)` only while `phase()==Ready` and `&[]` on retries, so MoreWork retries neither duplicate nor drop the event (see H1 for the trap case).
- Removal of reserve/write/close-poll-input is clean in Rust (`OriginalActorGuestEntry`, `terminal_is_empty`, WIT, macro exports all updated together).
- `DocumentStorePublicationToken` checks operation, parent generation and base child-content generation. The ladder passes the correct `operation_id` and live values.
- Job output page loop (`job-crossing`): bounded by `PENDING_PAGES=1024`, offsets checked, expected byte count checked, ack required. Total output length is guest-announced and not host-capped here (media limit is enforced elsewhere).
- Policy: no new `.unwrap()/.expect()/#[allow]/[DEBUG]` in non-test Rust in this scope (only test `unwrap`s on fixtures).
