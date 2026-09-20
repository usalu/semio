# Frame Publication and Latency Audit

## Scope and evidence boundary

This is a read-only audit of the current source and checkpoint7 artifact. No runtime, browser automation, build, or production source was run or changed.

The settled artifact is the sibling `paired-checkpoint-7-settled-2/steps.json`, rather than a `settled-2` child of `paired-checkpoint-7`. It has seven WGPU `settling` records: boot 4,804 ms; dismiss-tour 2,559 ms; panel-artifact 2,881 ms; panel-catalogue 3,465 ms; panel-catalogue-close 2,029 ms; window-close-last 1,721 ms; and window-reopen 3,460 ms. Each moved the published generation forward. The median is 2,881 ms and the mean is 2,988 ms.

These are explicitly composite settle-observation intervals, not action-to-registry execution times. The journey deliberately includes a one-second quiet period and awaits composite semantic outcomes after input, so the fields include that policy and observation polling as well as any renderer work. They prove that publication eventually advanced in 1.72–4.80 seconds in this run; they cannot be subtracted into a raw input latency or attributed to GPU work. The retained sibling `paired-checkpoint-7/steps.json` still has only semantic action/result fields. `📓️astra-runtime-latency.md` correctly treats a diagnostics-enabled, shared-host run as an acceptance concern pending the activation8 outcome-based probe.

The checkpoint7 WGPU console has 33,995 lines: 16,996 captured as `worker` and 17,000 mirrored as `page`. Restricting the aggregate to the worker side prevents treating the capture mirror as two renderer runs. It contains 4,837 worker `[DEBUG]`/log records, 1,042 `frame build admitted` records, 494 `os_host frame gate blocked=true` records, 495 open-gate transition records, and 473 `worker-step` overruns. The three worker-side overrun samples at or above one second are 1,992.6 ms, 2,567.5 ms, and 4,181.6 ms (the latter is the reported maximum).

## Actual publication path

1. A DOM input becomes a replaceable or lossless event in `BrowserFrameTransport`. It increments the transport generation, marks `frameRequested`, and schedules one rAF. `flush` posts exactly one bounded batch only when `inFlight` is false (`🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts:458-564`).
2. The frame Worker receives that batch, calls Rust `enqueueBatch`, then `tick`, and only afterwards posts its reply (`🎞️frame-worker/🟦️.ts:332-355`). The transport clears `inFlight` only on that reply and schedules the next rAF when the Rust result still asks for work (`🚚️browser-frame-transport/🟦️.ts:814-853`). Thus input batches, worker steps, and browser frames form a serial feedback loop; there is no parallel publication lane.
3. Rust applies the wire events, invalidates `INPUT_STATE`, and calls `OsHost::redraw_offscreen_worker` from `BrowserRendererWorker::tick` (`🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:250-326`). `OsHost::build_and_publish_snapshot` drains the bounded event page into `RuntimeApply::DispatchEvents`, pumps a bounded apply share, and then offers a frame build (`🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:205-243`).
4. The wasm `FrameBuildHandle` drives a resumable transaction only until half of the 8 ms interactive ceiling: `BROWSER_FRAME_BUILD_DRIVE_US = 4,000` µs (`🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:120-124,639-685`; `⏱️trace/🦀️.rs:99`). Its stages include input routing, tree reconciliation, render-packet construction, and snapshot publication (`🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12143-12170`). A yielded transaction therefore needs another worker round trip and rAF.
5. The Shell refresh path is real work in that transaction, not a page-side registry write. `ShellState::refresh_ui` serially awaits `render_with_document` for each selected live window and panel leaf, replaces the prior document, and retires it (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6479-6575`). The retained document exchanger progresses one eligible retained surface/document operation at a time (`🧊️renderer/🦀️.rs:7769-7838`).
6. The other 4 ms share drives `AppPresenter::present_step`. While it has a pending packet, retirement, retained fault, or acknowledgement, `admit_next_frame` refuses a new build (`🪟️winit-app/🦀️.rs:239-283`; `🧊️renderer/🦀️.rs:13599-13747`). The browser tick requests another frame while either the build or presentation remains live (`🌐️browser-worker/🦀️.rs:294-326`).

## Findings

### 1. The 4.18 s measurement localizes a worker-wall stall, not GPU time

This is proven. The Worker wraps `enqueueBatch` plus the whole synchronous Rust `tick` in `ownedStep("frame-step")` (`🎞️frame-worker/🟦️.ts:332-355`). The 4,181.6 ms sample therefore includes Rust transaction/presenter CPU work, synchronous diagnostics inside that call, and any worker descheduling while it is executing. It excludes queueing before the Worker receives the message, the next page rAF, and GPU completion/pixel presentation. It cannot identify a GPU batch, a shader, or a uniform write as the cause.

The two other multi-second samples and 470 further sub-second overrun records establish severe wall-time instability in that checkpoint. They do not identify an interior phase. Shared-host contention is a plausible contributor, but the artifact has no CPU scheduling trace to prove it.

Minimal fix: add phase-local monotonic measurements before changing scheduling. Record one generation-correlated span for Rust wire admission, `DispatchEvents` application, every `FrameTransactionStage`, Shell document refresh/retained exchange, every presenter phase, queue submit, and snapshot publication. Keep the existing watchdog and diagnostic lines.

### 2. Presentation is a deliberate head-of-line gate and was active in the checkpoint

This is proven both in source and the worker-side aggregate. `AppPresenter::admit_next_frame` returns immediately while presentation is pending. The checkpoint recorded 494 blocked-gate transitions across 1,042 frame admissions. Since the current browser driver gives both build and presentation only 4 ms per tick, a pending presentation delays any new build until repeated Worker reply → page rAF → Worker tick cycles drain it.

That order explains how a normally bounded amount of work can become seconds of input-to-publication latency under slow browser turns: every extra resumable build, document exchange, or present phase must wait for another serial loop. It is not by itself a correctness defect; the gate protects generation ordering and prevents a presentation from being overtaken.

Minimal fix: do not bypass the gate or add a competing publication path. First attribute each blocked interval to the existing `presentation_gate_shape` owner and phase. If one owner dominates after activation8, reduce its resumable work per phase or advance more of that same owner within the existing 4 ms share; preserve the one-packet generation order.

### 3. Debug output and the prepared-scene uniform hypothesis are not proven causes

Runtime diagnostics are explicitly gated (`🧊️renderer/🦀️.rs:9116-9147,15998-15999`), and checkpoint7 does emit a large volume of them. The artifact does not contain an otherwise-identical diagnostics-off run, so it cannot quantify diagnostic cost. The page/worker mirror also means raw console-line totals must not be treated as two independent logger costs. Do not silence diagnostics or claim they caused the stall; retain them while adding the bounded structured timing record above.

Likewise, the prepared GPU scene path writes its shared globals slot 0 and immediately submits the scalar command. This audit found no evidence that a later write overwrote an earlier command's uniform binding. It is not a latency finding and must not be folded into this explanation.

## Required next measurement and production law

After canonical activation8 is available, run the same fixed action journey with diagnostics both enabled and disabled. For every action, retain a bounded per-generation record with these timestamps: DOM enqueue, rAF flush, Worker receipt, Rust wire admission, dispatch applied, transaction stage transitions, each Shell refresh/document exchange, presenter phase transitions, snapshot publication, Worker reply, and first observed semantic registry/geometry result. Report p50, p95, and maximum separately for page queueing, Worker execution, build/refresh, presenter, and observation polling; do not report one combined number as GPU time.

The production-facing law should execute a finite repeated action set and require, for each generation, ordered `input accepted → dispatch applied → snapshot published → semantic registry/surface change observed`, with no skipped generation and no permanent pending build/presenter. Run it in both diagnostic modes and retain the complete timing distribution as the acceptance artifact. A deterministic unit law should separately hold rAF and Worker replies to prove that a pending build/presentation re-requests a frame without admitting a second in-flight transport batch.

Until that run exists, the defensible conclusion is: checkpoint7 proves a serious worker wall-time acceptance failure, composite settling of 1.72–4.80 seconds, and an active serial publication gate. It does not prove raw input latency, a GPU batching defect, a uniform overwrite, diagnostic overhead, or release-build latency.
