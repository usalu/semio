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
