# Checkpoint 19 Pre-Submit Input Authority Audit

Read-only audit on 2026-09-21. No build or test was run.

## Evidence and boundary

`🗑️generated/astra-runtime/checkpoint19-iab/wgpu-drivers-fault.json` records the browser quarantine at `2026-09-21T16:34:08.439Z` after the Driver heading click:

```text
worker-present-failed: prepared frame input authority was stale before submit
```

The receipt identifies the old fault string but does not distinguish Shell-witness loss from an Interpreter interaction-epoch rebase.

The current source separates this case correctly from GPU work:

- `renderer/🦀️.rs:15283-15310` is the preflight. Both `PresentedInputCandidateProgress::Stale` and the final exact Shell/Interpreter witness mismatch now take the staged packet with `gate.abort_pending()`, clear `cursor.witness`, enter `Aborted`, and return `AppPresentStep::Pending`.
- `renderer/🦀️.rs:15097-15133` then closes any active engine/GPU cursor, discards the exact Shell/Interpreter candidate, releases raster ownership, and moves the aborted frame into bounded retirement.
- `Shell/🦀️.rs:13046-13055` discards without publishing hits, geometry, accessibility, or widget maps. `Interpreter/🦀️.rs:2104-2118` reports an epoch rebase as stale, but `ui/engine/🦀️.rs:1209-1218` still clears the matching sealed witness. Thus ordinary interaction-epoch staleness reaches terminal cleanup rather than an abort-time “lost witness” fault.
- The browser worker turns `host.presenter.has_pending_presentation()` into `continue_frame` at `browser-worker/🦀️.rs:368-388`. The resulting `Pending` therefore schedules cleanup and successor admission instead of producing the `present-failed` quarantine.

`begin_prepared_present` at `ui/gpu/🦀️.rs:607-610` only reserves the raster presenter cursor. The first `queue.submit` is in `prepared_present_step`'s `ClearScene` phase at `ui/gpu/🦀️.rs:640-645`; subsequent offscreen submits and the swapchain present follow later. The observed stale boundary is before either call.

## Publication and last-frame result

No pre-submit stale route calls `Shell::acknowledge_presented_input` (`renderer/🦀️.rs:15418`) or `PreparedRenderGate::acknowledge_presented` (`renderer/🦀️.rs:15423`). `PreparedRenderGate::abort_pending` removes only `pending` (`ui/prepared/🦀️.rs:3487-3489`); its `last_valid` packet is replaced only by acknowledgement (`3477-3484`). Therefore the last completed presentation remains the active authority and a stale candidate cannot publish input.

This is high-confidence source evidence. It does not prove the fresh browser activation yet.

## Error and close classification

Keep the narrow change narrow.

- A normal pre-submit input stale condition is a cancellation/reschedule. The two current branches are the correct place to return `Pending`.
- All queued runtime applies are held while Render, CloseGpu, or Acknowledge is active (`renderer/🦀️.rs:14875-14877`; `winit-app/🦀️.rs:239-242`). This prevents ordinary input from creating a new post-preflight stale candidate.
- Explicit host close uses `close_frame_owners_step` (`renderer/🦀️.rs:14909-14959`) and also discards the candidate before retiring its packet. It is already a bounded cancellation path.
- Do **not** turn GPU execution errors into the pre-submit retry. After `prepared_present_step` begins at `renderer/🦀️.rs:15342`, it can have submitted ClearScene, composite, glass, or the swapchain work (`ui/gpu/🦀️.rs:640-730`). Its error path must close the GPU cursor and must not acknowledge the input candidate. A structural stale/missing authority in Acknowledge (`renderer/🦀️.rs:15378-15428`) likewise must not promote candidate input after GPU work. The current paths enter `Aborted`; their `Err` classification remains appropriate for an invariant breach, subject to their retirement being driven before a fault is surfaced.

## Required fail-first regression

The new `a_pre_submit_input_epoch_change_retires_and_reschedules_without_faulting_the_surface` in `🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:45-64` is a source contract only. Add one actual `AppPresenter`/worker-host test using the existing `runtime_with_presented_input_candidate` fixture from `🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs:115-183`:

1. acknowledge a baseline packet A and record its `PreparedRenderGate::last_valid_identity` plus a route through A's published hit map;
2. stage packet B and its exact candidate; advance B to Render but before `prepared_present_step`;
3. advance B's presented interaction epoch, leaving B sealed, so `progress_presented_input_candidate` returns `Stale`;
4. assert the presenter answers `Pending`, drives `Aborted` to terminal retirement, and can admit a fresh candidate C;
5. assert B was never acknowledged, A's hit route and last-valid identity remain until C succeeds, and C alone publishes the replacement authority;
6. in the browser host variant, assert the tick reports `continue_frame: true` and no `present-failed` fault.

That proves the Driver-class sequence rather than merely checking strings. A separate injected error after the first `queue.submit` should assert that no candidate input is acknowledged and cleanup closes the GPU cursor; it must not assert resubmission of that same frame.

## Conclusion

The current pre-submit repair matches the required ownership law and preserves both unacknowledged input isolation and the last completed frame. The checkpoint receipt predates it. Fresh Native/browser execution and the behavioral regression above remain required before calling the Driver route repaired.
