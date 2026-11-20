# CPU picker deadlock (2026-11-19)

## Observation
A (`repo cargo ... check -j4 --target wasm32-wasip2`) held the exclusive `semio.compiler.cpu-picker/v1` for 85 min; the watchdog called it "progressing" with phase `semio.compiler.host-capacity/v1`. B (release native build under `repo capacity run --jobs=6`) held cores and queued a `descriptor:build` job on the picker at position 2.

## Cause
`acquireCompilerCapacityLeaseV1` (`🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🎫️lease/🟦️.ts`) took the gate (`host-capacity/v1`, shared), then the exclusive picker, then polled every 40 ms for its cores while still holding the picker. A picker holder blocked on cores another claimant holds, while that claimant queues for the picker, is a cycle (hold-and-wait) that no ordering rule saw: gate, picker and cores are one lease class (`compiler-capacity`, rank 3).
The "progressing" report is a side effect of the same loop: the wait published `{resource: host-capacity}` every second, and any published line renews liveness, so the blocked picker holder looked alive and named the gate as its phase.

## Fix
The picker is held for one non-blocking attempt only (`pick`): take every core with `timeoutMs:0`; success keeps the cores, failure releases them, and in both cases the picker is released before anything waits. The wait (progress publication and the 40 ms poll) happens holding only the shared gate and no core, then the claim re-queues for the picker. No picker holder ever blocks, so the cycle cannot form, and a picker holder cannot be reported as progressing while waiting.

Trade-off, now the law: the picker no longer orders blocked claims. An unsatisfiable wide claim does not hold narrow claims back (no head-of-line blocking); whole-host writers keep their priority through the exclusive gate. Fixture `multiCore.order` is now `narrow, wide`.

## Tests (bun, no cargo)
`🧪️tests/🟦️.ts` of the capacity module:
- new: "A claim waiting for cores never holds the core picker ..." holds all but one core, starts a two-core claim that must wait, then five other claimants take and release the picker within 2 s while it still waits (the old code starves them); after release the waiter is granted two cores.
- changed: multi-core test expects the narrow claim to be granted while the wide one waits, then the wide one after release.
- `🪆️scope` census fixture: literalCalls 14 -> 16 for that test file, total 99 -> 101. That suite still fails on three rows that other workers' in-flight edits changed (component-pass, wgpu wasm script, caching wasm-outputs test); not touched.
