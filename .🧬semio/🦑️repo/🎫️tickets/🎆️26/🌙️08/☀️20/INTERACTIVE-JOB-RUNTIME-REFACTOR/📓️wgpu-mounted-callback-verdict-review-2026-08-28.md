# Mounted WGPU Callback Verdict Review

Read-only source review while the Plugin fixture hold was coordinated. No WGPU/UI-host production edits, no Cargo, no taxonomy preimage repin, and no runtime/timing claim.

## Actual Boundaries

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️winit_app.rs:54` event and :65 metrics create RAII-only Watchdogs. Both mutate frame generation and scheduler before enqueue, without consuming `finish()`. The void metrics helper cannot stop `handle_metrics` (:106) from enqueueing surface resize and `RuntimeApply::Resize` afterward.
- The same file :117 redraw and :130 offscreen call `redraw_core` under RAII-only guards. `build_and_publish_snapshot` (:190 onward) advances presentation, then commits fullscreen/cursor wake and `snapshot_sink.publish` before the outer callback verdict. `present_snapshot` also mutates scheduler deadlines.
- `ui/🖥️host/📦️packages/🦀️rust/🦀️enqueue.rs:234` EventQueue already owns fixed coalesced slots and a preallocated bounded discrete queue. However `enqueue` advances generation before capacity refusal; overflow consumes the incoming owned event. It has no reserved candidate/commit or exact returned-event API.
- `ui/🖼️render/📦️packages/🦀️rust/🦀️schedule.rs:83` scheduler invalidation is a small scalar mutation, but deadline requests and `should_render` allocate; no staged invalidation receipt exists.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:14514` `admit_next_frame` already preflights `has_pending_presentation` and retains `AppPresentCursor` in `self.pending`. This is the smallest existing candidate owner to extend. `present_step` retains its frame through BeginGpu/Engine/Uploads/Submit/CloseGpu/Acknowledge/Fullscreen/Directives, but real GPU submission occurs in Submit; Fullscreen and Directives perform real platform effects before Complete.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️render_snapshot.rs:82` the sink is not a bounded publication receiver: `publish` uses blocking Mutex::lock and allocates Arc inside the lock; `acquire` blocks too. The current comments do not prove contention cannot happen.

## Smallest Cohesive Follow-Up

1. Reuse EventQueue's actual fixed storage: an in-place candidate/reservation holding the exact event and checked next generation, with capacity refusal returning that event. Keep coalesced/discrete generations and scheduler invalidation uncommitted until the real callback-owned verdict accepts the staged result. Metrics must return a typed verdict and retain one scalar candidate covering queue, resize lane, and runtime mailbox admission; a void-helper early return cannot gate its caller.
2. Extend the existing retained AppPresentCursor rather than creating a second watchdog or frame owner. Reserve a final metadata receiver before child advancement; retain Complete directives and old/new snapshot owners in place through refusal/cancel/unwind. Replace sink blocking/Arc creation at publication with an explicitly admitted candidate and nonblocking exact receiver/retirement seam.
3. Preserve measurement of actual submit/fullscreen/cursor/commit work. A later snapshot refusal cannot roll back an already submitted GPU frame. Moving those calls outside the watchdog would not solve the contract. Existing APIs cannot honestly prove “no external effect after an overrun discovered at callback return”; the external commit phase itself needs an explicit measured authority/result contract and fault handling. Keep that obligation open rather than treating withheld snapshot/cursor metadata as GPU rollback.

## Real-Source TDD Selectors

Existing (not rerun here): `callback_latency_tests::mounted_pointer_storm_callback_p99_stays_below_two_milliseconds`, `mounted_resize_storm_callback_p99_stays_below_two_milliseconds`, `mounted_frame_generation_exhaustion_is_permanent_and_non_wrapping` in winit_app. They currently use percentile telemetry, not exact per-callback fault publication authority.

Add schema-first laws in that same module and the actual EventQueue/sink modules: (a) held receiver/zero credit/cancel/7999–8000–8001us preserves incoming event and old generations; (b) metrics fault yields zero resize/runtime queue commits; (c) candidate child completion cannot publish before the actual callback verdict; (d) final-sink contention/poison preserves the exact retained candidate without blocking; (e) injected post-submit overrun records the already-issued effect and quarantine explicitly—must never assert zero GPU effects or fake rollback. Use the existing trace clock/verdict fixtures, not an independent fake watchdog. Native platform and offscreen/Wasm entry gates remain separate required executions.

