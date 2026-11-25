# Generator preview stall: previewEval tick receiving is one byte per two turns

## Evidence
`pw-gen`/`pw-gde` gate trace: every previewEval run receives its ticks with `step:preview-receiving` x1818..x2286 per tick, and each completed run retires (`drive:retiring`) and re-initializes for the next run. The run does finish ticks, but at about 2000 host turns per tick the served path (about 10-17 ms per wasm turn) spends tens of seconds per tick and the boot test samples meshes=0 before any scene lands. Native passes because turn count is free there.

## Root cause
`OriginalToolRunNativeState::receive_with_domain` phase 1 (`tool-run/tick/native`) launched ONE `ToolRunTickNativeCursor::advance` (one wire byte, token grant or record admission) with `with_native_receiving`, returned `Ok(false)`, and settled the receiving scope only on the NEXT call. So one wire byte cost two driver turns; a tick of about 900 bytes cost about 1800-2300 turns. Same class as the display writer fix (one token per turn) in `🔬️generator-empty-preview.md`.

## Fix
Phase 1 now loops inside the turn (bounded by the retained grant's `maximum_items`): settle the previous outcome, check completion, launch the next advance; it leaves the turn when the step reports zero progress, the turn must yield (fuel or deadline), or no outcome is pending. Refusal and settlement semantics are unchanged; an unsettled outcome still settles first on the next call.

The temporary `[DEBUG] tool-run gate` probe (`debug_tool_run_gate` and its 12 call sites in the tool-run instance) is removed.

## Files
- tool-run/tick/native: `🎞️tick/🛬️native/🦀️.rs` (phase 1)
- tool-run/instance: `🦀️.rs`, `🎟️job-admission-and-finalize-turn/🦀️.rs`, `👀️generation-watch-and-turn-driver/🦀️.rs` (probe removal)

## Not verified
No browser/served run. Remaining non-tool-run `[actor] [DEBUG]` lines (turn phase, ingress gates) in the console belong to other emitters and still fail the boot test if present in the rebuilt module.

## Follow-up (after the staged 21:46 rebuild): preview stays "Cancelled", reactor idles

Evidence: the page snapshot in `pw-gen/*/error-context.md` shows the preview status pill `Cancelled`; the console is 1396 fully idle turns (every `more-work inputs` flag false), so nothing owes or runs a hop.

Root cause: a `previewEval` run job that closes before its run settled (`link.settled == None`, e.g. a document/settings restart or a faulted run) calls `FlowEvalSession::cancel_preview_evaluation`, which sets `preview_cancelled` and a drain cursor (`preview_cancellation_cursor = Some(0)`). `preview_cancellation_step` is the only thing that drains that cursor and NOTHING in production calls it (only tests do). Consequences while the cursor stays set: `window_tick_owed` is false (`preview_cancelled`), `begin_window_tick_key` refuses ("original preview cancellation must finish before another tick"), so every later run settles immediately with no hop, `restart_owed` is false, no effect is owed and the reactor idles on a permanently cancelled, empty preview. Native passes because no run is ever closed unsettled there.

Fix:
- `🌊️flow/🖥️host/⏱️eval-session/🔭️preview-tessellation-cancellation-and-status/🦀️.rs`: new `resume_preview_evaluation()` lifts a finished cancellation (`false` while it still drains).
- `✏️s/🔌️plugins/🌀️procedural/.../🧵️preview-eval/⏯️tool-run/🦀️.rs`: new `PreviewEvalRunJob::drain_cancellation` drives `preview_cancellation_step` under the job's step context. It runs in `close_step` once the release is recorded (the closing job now finishes the drain it started) and at the head of every live `step_value` (draining a stale cancellation left by a previous run). `PreviewEvalRunJob::new` and the hop law call `resume_preview_evaluation()` so a live run owes and dispatches hops again once drained.

The `[actor] [DEBUG] turn phase / reconcile loop / more-work inputs` strings do not exist in any current source (grep over `🧰️framework` and `✏️s`, all extensions, finds only `[TRACE]` variants of the same lines in `⚛️reactor/🔄️turn/🔦️more-work-sources-and-tracing`, gated by runtime diagnostics): the staged module was built from a tree that still had them, so they vanish on the next rebuild. Not touched.

generation2d has the same pattern (`generation2d/.../🧵️preview-eval/⏯️tool-run/🦀️.rs` ~706); not changed here.

## Follow-up 2 (runtime-confirmed): tick during an unfinished drain was a hard error

After `drain_cancellation` the served build failed with `interactive-job.worker-continuation: WorkLimit: original preview cancellation must finish before another tick` (raised by `begin_window_tick_key`) because a window tick was dispatched while the drain was still stepping, and a `WorkLimit` surfaces as a non-retryable worker fault.

Fix: `MountedPreviewTick::step` (`generation3d/.../🧵️preview-eval/⏱️tick-and-resolve/🦀️.rs`) now, before it starts a tick (no host yet), advances the session's cancellation drain one step on the tick's own effect and answers `(None, progress)` (pending) while `preview_cancellation_terminal_is_empty()` is false. The tick path therefore both drives the drain every turn it is dispatched and never reaches `begin_window_tick`'s refusal. The run job's `drain_cancellation` (close and live step) remains the driver when no tick is in flight; `resume_preview_evaluation` lifts the finished cancellation.

Unchecked: the composition workspace crate does not resolve under `repo cargo check` here (cargo resolver panic `did not find features for ... preview-eval`); procedural rebuild is the check. The non-mounted `evaluate_tick_with_seeds` and generation2d keep the old refusal (no mounted caller reaches them while cancelled).

## Follow-up 3: drained cancellation wipes the new run's owed latches

After the 01:42 rebuild the pane still reads `Cancelled` / tool run `Running · Evaluating nodes (1/2)`, meshes=0. The `[DEBUG] ladder zero ... Stage(24)/Stage(14)` lines come from a store-publication probe that is not in source any more; they are not part of the preview-eval path.

Reading `preview_cancellation_step`: phase 2 resets EVERY window tick latch to `FlowEvalWindowTickLatch::default()` (unfinished=false). A run that starts after a restart owes its windows in `PreviewEvalRunJob::new` (`note_window_tick_outcome(.., true)`), but the cancellation drain that follows (stepped by `drain_cancellation`) wipes exactly those latches, so once the cancellation is lifted `window_tick_owed` is false, the hop law answers `Settled` and the run completes with no hop and no mesh.

Fix (`generation3d/.../🧵️preview-eval/⏯️tool-run/🦀️.rs`, run step): when `resume_preview_evaluation()` lifts a cancellation (it was cancelled and the drain is finished), the run owes every attached window one evaluation again before it consults `next_preview_eval_hop`; while the drain is unfinished the hop is `Wait`.

Unchecked: the composition workspace crate cannot be `cargo check`ed here (resolver panic); procedural rebuild is the check. If the pane still reads `Cancelled` after this, the drain itself is not completing (suspects: `SessionRetirement::admit` returning no progress, or the geometry port `cancel_step` in phase 4); the next probe belongs in `preview_cancellation_step`.

## Follow-up 4: `[DEBUG] gen-preview` instrumentation (remove after the cause is known)

`semio_os_flow::debug_gen_preview(String)` (defined in the session crate, `🔭️preview-tessellation-cancellation-and-status`) prints `[DEBUG] gen-preview <line>` through eprintln only when the line differs from the previous one on that thread. Call sites:
- session: `cancel_preview_evaluation`; every `preview_cancellation_step` (phase, cursor, demand, grant items) and its `unfunded` early return; `resume_preview_evaluation` (refused while draining with phase/cursor, or lifted); `window_tick_owed` (per window, true/false, or false because cancelled).
- run job (`generation3d/.../🧵️preview-eval/⏯️tool-run`): per step `run N hop=<Dispatch|Wait|Settled> nodes meshes tessellated ext_fault`; `drain_cancellation` outcome (still draining / turn failure / owner fault).
- `⏱️tick-and-resolve`: mounted tick yielding to the drain; `finish_tick` (`more`, extension invocation count).
- edit preview window `render`: `eval_text` bytes, `meshes`/`instances` JSON bytes, `cancelled`, session `eval_json` bytes.

How to read one run: `cancel_preview_evaluation` then `cancellation_step phase=0..5` lines show whether the drain completes (a last line stuck on one phase or `unfunded` names the stall); `resume lifted` then `run N hop=Dispatch` then `finish_tick` then `render eval_text=Some(..)` shows the chain; if `render` prints `eval_text=None`/0 B while `session_eval` is non-zero the window renders from the retained transient text and the tick's publication never lands in it (the KNOWN GAP question: the edit `render` takes `preview_eval_text` from the transient, not from the session). Remove every `debug_gen_preview` call and the helper when done.

## Follow-up 5: the cancellation drain never starts because the session queue has no reserved slot

Trace: `generator-preview-trace.cjs` (full console, 90 s) -> `🗑️generated/preview-trace/console.txt`. The first run dispatches (`hop=Dispatch(0)`, `Dispatch(1)`) but finishes with `nodes=0 meshes=0` and `render eval_text=None`. At 25.7 s a restart calls `cancel_preview_evaluation`; from then on every line is `resume refused: draining phase=0 cursor=Some(0)` and `drain_cancellation ... -> owner fault app.message`, and NO `cancellation_step phase=0` line ever appears. So `next_preview_cancellation_demands` fails before the step runs, and the drain never leaves phase 0 (the pane stays `Cancelled`, `window_tick_owed ... (cancelled)`).

Cause: phase 0 quotes `SessionRetirement::admission_demands`, which returns `OwnershipLimit "session retirement requires an admitted original queue slot"` when a retained root is present (an earlier handoff, e.g. the preceding eval-json reset) but the owner queue has no reserved slot. Nothing in production ever reserves that slot (`reserve_retirement_step` is only called by tests), so the error repeats forever and surfaces as an owner fault whose message (`app.message`) the run job swallows.

Fix: `SessionRetirement` gains `slot_owed` / `slot_demands` / `reserve_slot` (`🧾️eval-session-state`); cancellation phase 0 quotes the slot reservation when owed and performs `reserve_step` as its step (phase stays 0), then admits on the next step. Test: `original_preview_cancel_reserves_its_own_queue_slot_when_a_retained_root_displaces_it` (`🧹️retirement/🧪️tests`), not compiled (crate not resolvable via `repo cargo -p`).

Probes removed: `debug_gen_preview` helper, re-exports (`🎞️session`, `eval-session`, `📦️packages/🦀️rust`) and all call sites (session, run job, tick-and-resolve, edit preview window); no `follow unit` remains in source.
