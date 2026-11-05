# Read-Only Capacity and Lease Audit

Observed through the existing correctly rooted repository MCP bridge at 2026-11-05T00:42:57.525000+00:00; no direct process tool, foreign cancellation, cap adjustment or publication mutation occurred. Full original outputs are generated/1104-current-capacity_status.log and generated/1104-current-leases_status.log. A five-second sample uses the first-party watchdog liveChildPidsV1/treeCpuMillisecondsV1 APIs through registered Bun; input1105-native-liveness.mjs, full output generated/1105-native-liveness.json.

The host reports 32768MB total, 3931MB available, warning memory pressure, 1623MB swap used and zero swap-out MB/s. Available memory is already165MB below the unchanged4096MBfloor. The original lower request requires at least2552MB (fixed1094+onejob1458), prefers two jobs and has no current grant. Its actualPID8119 is a capacity waiter, holds no repository lease, has no descendants before admission, and used40msCPU over5199ms, consistent with active capacity polling. Its single persistent handle remains2305; there is no observed stale own reservation or pending own source publication blocking admission.

Foreign admitted operations are:

| Operation/PID | Age | Observed MB | Reserved MB | Tree CPU Increase /5199ms |
| --- | ---: | ---: | ---: | ---: |
| Renderer WGPU83655 |2565s|6116|4511|4160ms|
| Workspace components92113 |1591s|5165|3584|4190ms|
| Renderer React92203 |936s|4183|4279|4210ms|

The process trees and active lease-owner descendants are live. Their lease standings report dependencies:generation-verify, counter98304, with recent heartbeats. CPU activity is evidence of active verification work, not proof that Rust compilation or the verification counter advanced. Observed footprints exceed aggregate reservation by3090MB; actual host availability remains the decisive memory gate.

Lease status contains15holders, zero lease waiters, zero wait-for edges and zero cycles. Ten stored order-violation records exist, but they are historical/non-enforced records rather than an observed current wait cycle. No evidence justifies stopping another owner's process, reclaiming an image or altering limits. The current refusal is genuine host-memory pressure while foreign verification work remains active.

## Descendant Names and Counter Recheck

At00:54:18Z, existing first-party systemAncestryV1 plus watchdog descendants were sampled once. All three foreign grant trees contained repository Bun frontends and Bun retained-loader eval descendants; no rustc, Cargo, Bazel or other standalone compiler process was observed. Eval program bodies were omitted rather than copied into the ticket. React's earlier extra descendant24995 had exited. Bounded names/start observations are generated/1105-native-commands.json; input1105-native-commands.mjs.

At00:54:42Z, fresh leases still reported generation-verify/done98304 for the same three owner processes, unchanged across705seconds from the earlier snapshot. Heartbeats remained recent (0–8seconds), with15holders and zero waiters/waitFor/cycles. Thus the CPU sample does not establish forward completion, Rust compilation, or an infinite loop. In-process TypeScript work cannot be excluded from executable names alone. The evidence supports a production-source-stable observation window; it does not authorize stopping or altering another owner.

## Persistent lower request at 2480 seconds

Actual unified session 2305 remains live, PID 8119, capacity-only. No Source admission or compiler/test execution. Latest output alternates insufficient memory (779 MB spare above the unchanged 4096 MB floor, request needs 2552 MB) and earlier requests waiting ahead (UI Rust crate-boundaries, Forms config tests, Puzzle2D). Running workload labels remain WGPU, components and React typecheck. No request restart, foreign cancellation or limit change.

The defining generation progress producer was handed to the separate read-only audit: dependency generation storage controlOf.tick publishes work before yielding; verifyGenerationReceiptV1 increments its local work counter and ticks modulo protocol.progressWork. Static progress labels alone do not establish an infinite loop.

## Stationary counters at 1793841282511

Fresh rightful leases_status reports all three foreign standings still dependencies:generation-verify done=98304, another 1200 seconds after sample1793840082546. Heartbeat ages8/9/4seconds; implementation ages4467/3495/2840seconds. Current lease waiters0 and cycles empty. Same lower native request remains capacity-only at3283seconds. These observations establish a stationary reported counter with live heartbeats, not the cause or an infinite-loop proof. Production source remains untouched by this lane.

## Same request beyond one hour

Actual session2305/PID8119 remains capacity-only at3731seconds. Latest sample has three running/three ahead; request needs2552MB above floor, reported spare−276MB. No Source/compiler/test admission and no terminal receipt. Source remains stable in the lower captured dependency closure; no duplicate request or foreign process action. Flow reported a separate WidgetDescriptor RetireOwned derive outside the lower closure, queued for the next broad capture.

## Queue reached zero ahead

The unchanged lower PID8119/session2305 request reached zero earlier requests at5813seconds. It remains live at5915seconds, refused by learned reservations still growing to peak plus memory. Running labels now include Forms E2E and Puzzle2D, rather than the original three verification owners. No Source/compiler/test admission has occurred. Queue advancement is observed; no foreign process was signalled and no admission limits were changed.

## Genuine lower request terminal

The sole persistent PID8119/session2305 request completed naturally with exit1 at2026-11-05T02:04:00.966Z, no signal. It advanced through Source admission and refused before compilation at the Ordered file limit: lookup/borrowed/tests/cases.json1358 bytes, remainingFiles0, remaining byte allowance41566. No Rust compiler or tests ran. The active native window is released. No request was canceled or duplicated during its wait.

The next repair must consolidate cohesive neutral fixtures/test ownership while preserving every expectation and unchanged Source limits. Procedural owns the borrowed/rank fixture; exact cfg(test) census is pending. Durable primary evidence: generated/1103-lower-owned-equality-r1.log and JSON.
