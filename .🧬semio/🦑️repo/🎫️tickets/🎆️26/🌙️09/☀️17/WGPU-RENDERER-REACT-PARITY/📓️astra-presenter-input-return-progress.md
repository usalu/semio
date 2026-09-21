# Presenter Input Return Progress

## Actual browser failure

After the pre-submit stale-candidate repair, the fresh WGPU worker booted, opened and closed Driver, committed Light appearance, and retired its popup. A physical Settings-close click then quarantined the surface at `2026-09-21T16:49:39.467Z`:

```text
worker-present-failed: presentation stalled: phase=Render engine=0 upload=2 gpu-cursor=None upload-progress=(0, 0, 0)
```

Receipts: `🗑️generated/astra-runtime/checkpoint19-iab/wgpu-render-stall.json` and `wgpu-render-stall.jpg`.

The signature proves the presenter had staged the prepared packet but had not begun GPU submission. Render could answer unchanged `Pending` for three different reasons: the runtime mutex was busy, `AppRuntime.interaction` was checked out, or the exact sealed candidate still had a blocking scene intent.

## Causal cycle

`OsHost::build_and_publish_snapshot` suppresses the general `pump_pending_applies` while Render, CloseGpu, or Acknowledge owns a sealed input candidate. That fence is necessary because normal apply work may mutate input authority. A suspended dispatch or frame-deferred operation returns `AppInteractionState` through a `ResumeDispatch` or `ResumeFrameDeferred` completion, however, and Render needs that state to check the candidate. The completion woke the host, but the fenced host refused to apply it; Render observed `interaction=None`; the tight browser presenter loop retried the unchanged cursor 4,096 times and the watchdog reported a GPU-shaped stall. `AppRuntime::return_interaction` has no second wake after a skipped completion, so merely parking Render would deadlock.

## Repair

The runtime mailbox now exposes one presenter-only restoration step. It may take only a ready completion whose typed `restores_interaction` flag is true. It returns the exact `AppInteractionState`, clears that owner from the completion, and restores any remaining dispatch/deferred cursor to the same mailbox slot. It never calls `apply_step`, `start_dispatch`, or `start_frame_deferred`, so no post-candidate event mutates the sealed revision. Native frame-maintenance refusal uses the same split: its state returns and its cursor moves back to the existing `pending_frame_deferred` owner. A checkout with no in-flight, ready, or native-refusal owner faults instead of parking forever.

The presenter distinguishes three waits:

- `RuntimeLock` schedules one later host retry because mutex release has no waker.
- `InteractionCheckout` parks the same-tick loop and relies on the existing completion publication waker.
- `SceneIntent` remains directly runnable and increments the watchdog signature only when one real bounded intent unit advanced.

The watchdog remains enabled. Its diagnostic now appends the exact input wait reason, and a scene-intent blocker that reports no progress still reaches the same fixed 4,096-step fault.

## Laws

- `presenter_restores_only_the_checked_out_interaction_and_retains_its_deferred_cursor` exercises the actual mailbox, checkout ledger, typed returned completion, and retained cursor without starting the cursor.
- The `present-stall-watch` serde fixture includes healthy moving scene-intent progress, a frozen scene-intent blocker, and a direct runtime-lock stall with exact diagnostic strings.
- `accepted_a_stale_b_abort_and_accepted_c_preserve_exact_presenter_owners` exercises actual `PreparedRenderGate` ownership: A is accepted, B is aborted without replacing A, and only exact acknowledgement publishes C.
- The four frame-job candidate-carrier laws now begin with an accepted A before staging B, close B through their distinct carrier, and prove C can seal and acknowledge afterward.

All touched Rust sources parse with `rustfmt --edition 2021 --emit stdout`. Root owns Native134 and the fresh browser activation.
## Neutral mailbox oracle

The language-neutral runtime-mailbox admission fixture now includes a presenter-only return row. It checks out the interaction owner, publishes a deferred completion, restores the state without executing that deferred cursor, and later applies the retained cursor in revision order.

The scoped Bun Nx Vitest gate passed the fixture with 18/18 tests in every Nx execution shard. Receipt: `🗑️generated/astra-runtime/runtime-mailbox-neutral/run.log`.

## Window-close checkout follow-up

The repaired worker later reproduced a typed `InteractionCheckout` presenter stall after physical focus/unfocus/refocus and closing Top then Perspective. Receipt: `🗑️generated/astra-runtime/checkpoint19-iab/window-close-interaction-checkout-fault.json` at `2026-09-21T17:42:41.417Z`.

The close hit had an exact causal hold: `handle_shell_hit` first mutated the dock and requested the UI document close, then awaited `noteShellCommand`. Pointer dispatch therefore retained the entire `AppInteractionState` across a guest round trip after close state was already observable. A presenter sealed in that interval could not regain the Shell/input owner required for pre-submit validation.

The close journal now enters the existing deferred-action owner. The close returns the exact interaction state immediately; normal mailbox pumping may run the note only after the presenter's input-publication fence releases. The focused law `the_close_cap_closes_exactly_the_clicked_window_and_refocuses_the_survivor` now requires a successful synchronous close plus one retained `shell.windowClose` note carrying the exact closed window id. Other `note_control_command` paths remain unchanged.
