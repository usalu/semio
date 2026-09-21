# Frame Turn Scheduling

## Implemented boundary

This packet implements the frame-only half of the retained scheduling design. Page ingress enqueues the input, publishes `batch-accepted`, and requests a private scheduler in the dedicated frame Worker. One scheduler callback performs at most one frame turn. A returned continuation schedules another callback rather than asking the page to submit an empty batch.

Input acknowledgement and frame publication now have independent checked sequences:

- `inputSequence` is page-owned and releases only the exact inbound lease after `batch-accepted`.
- `frameSequence` is Worker-owned, strictly increasing, and accepted independently by the page transport.
- `nextFrameSequence` refuses an invalid value or `Number.MAX_SAFE_INTEGER`; it neither saturates nor reuses a sequence.

`ActiveFrameBuild::step` now delegates to `run_frame_owner_turn`, which checks the pre-yield boundary, attempts exactly one retained `advance`, and charges one fuel unit even when that advance completes the build. Cancellation remains first. Native submission stays on the existing worker pool. Browser wasm uses the new `WorkerJobSession::try_step_on_worker` entry from its dedicated Worker callback and never uses the caller entry. The API shares the private exact-step implementation with `try_step_on_caller` while naming and constraining the scheduling authority at each call site. No uncharged loop, new capacity, or new runtime dependency was introduced.

Close is bounded through the same private scheduler: `beginClose` clears pending work and schedules at most one close unit per callback until terminal-empty. The frame Worker waits for that terminal witness alongside runtime and interactive-job closure.

## Neutral fixture and independent oracle

The strict fixture at `🧫️fixtures/🧵️frame-turn-scheduling` owns only the `frame` scheduler. It pins a two-turn retained request, the exact enqueue/ack trace, frame sequences `1, 2`, one close owner, and fail-closed sequence exhaustion. Decoder ownership is deliberately absent because the decoder candidate/commit packet remains open.

The ticket's permanent `📜️script.ts frame-scheduler-oracle` command drives the same fixture through the installed third-party React Scheduler. It schedules one callback at a time, permits ingress between callbacks, verifies the exact output order, and proves bounded close and exhaustion. The scoped Bun/Nx receipt was:

```text
frame-scheduler-oracle: input=41 generation=7 callbacks=2 close=1 exhaustion=fault
```

The exact command was:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/📜️script.ts' frame-scheduler-oracle '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json'
```

## Fail-first and browser receipts

The canonical Worker law was registered before its module existed. The first focused Nx execution failed with:

```text
Cannot find module '../../🎯️targets/🧊️wgpu/🧵️frame-turn-scheduler/🟦️.ts'
```

After implementing the retained scheduler and transport split, the focused law passed 1/1 with 108 tests outside the filter. A later source law then exposed a remaining ownership violation in the wasm frame build path: it still called `try_step_on_caller` inside a deadline-driven `loop` and could advance several job units during one Worker callback. The focused execution failed on that exact caller entry. The repair replaced the loop with one `try_step_on_worker`, one matching outcome/resume bookkeeping pass, or one close unit per callback. The same law now proves that the wasm block contains the Worker entry and contains none of `try_step_on_caller`, `BROWSER_FRAME_BUILD_DRIVE_US`, or `loop {`.

The final full browser Worker target passed all eight files and 111/111 tests in 1.72 seconds, with no failures; Nx completed in 2.5 seconds. That full run also covers the transport correction caught earlier: three stale tests had tried to release the inbound lease with a frame reply. They now publish the required `batch-accepted` acknowledgement and retain frame publication as an independent sequence.

The full command was:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/framework-renderer-wgpu:test-browser-worker'
```

The generated Worker initially failed its registered check only because `🎞️frame-worker.js` was stale. `generate-frame-worker` refreshed it through Nx, and the exact subsequent gate passed:

```text
framework-renderer-wgpu: 🎞️frame-worker.js is fresh
```

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/framework-renderer-wgpu:check-frame-worker'
```

The scheduler source is registered in the WGPU package's `frameWorkerSources` and in both schema-owned taxonomy lists. The taxonomy entries remain byte ordered.

## Native law

The canonical Rust law is `retained_frame_owner_turn_advances_once_and_charges_terminal_work` in `wgpu-frame-job-unit`. It consumes the neutral fixture, gives each of two calls one unit of fuel, proves exactly one phase per call, and proves the terminal phase is charged. `git diff --check` is clean across the packet. Native compilation and execution remain root-owned; no Cargo command was run in this lane. The root-owned exact gate is:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/framework-renderer-wgpu:test-wgpu-unit' -- retained_frame_owner_turn_advances_once_and_charges_terminal_work
```

## Remaining work

This packet does not claim complete scheduling or visual parity. The asset decoder still needs its retained worker owner, charged decoder units, capacity-one candidate, serialized frame-side commit, cancellation, fairness, and native/browser parity laws. The mounted frame-policy validator also remains intentionally RED with its eleven recorded production violations; this packet does not weaken those policies or call that baseline green. A real browser activation must still verify visible output and terminal scheduler state after the decoder/commit boundary is complete.

Native78 executed `retained_frame_owner_turn_advances_once_and_charges_terminal_work` successfully in Nextest run `8d00db48-8e7d-4ec0-8892-e8d14063c596`. The full renderer census reached 1,257 passing laws before ten unrelated scene-paint workers were terminated during the separately diagnosed clipped-hit-target helper deadlock. The retained frame owner law was not part of that hang.
