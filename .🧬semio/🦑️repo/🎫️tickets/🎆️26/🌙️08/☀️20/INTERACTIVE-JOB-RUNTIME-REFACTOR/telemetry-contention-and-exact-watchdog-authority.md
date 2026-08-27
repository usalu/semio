# Telemetry Contention And Exact Watchdog Authority

## Reproduced Boundary

The existing callback paths synchronously lock the site BTreeMap, violation ring, and event ring. Job driving wraps `job.step` in a Watchdog, then pushes an event; plugin quarantine looks up operation/generation in the process-global violation snapshot. That lookup also locks and allocates a full snapshot. The global bounded ring is not an exact per-job authority: eviction and unrelated operations can interfere, and retaining only operation/generation does not prove an exact session owner.

Four neutral cases and mounted native tests now hold each real mutex across callback completion observation. Native first invocation ran only the event case because nextest failed fast:0PASS/1FAIL,3notrun. Three exact follow-up invocations each failed0PASS/1FAIL,18skipped: timer/site(.135s summary), watchdog/site(.153s summary), watchdog/violation(.141s summary). All four callbacks returned only after their held mutex was released. The two watchdog cases additionally retained the exact operation/generation violation after release; replacing blocking locks with a lossy try-lock cannot satisfy those fault-preservation assertions. The100ms test rendezvous is not a latency certificate or a changed8000us production threshold.

## Proposed Authority Split

An exact callback-owned verdict must be distinct from optional telemetry. A private verdict owner records identity, checked elapsed microseconds, and missing/backward-clock failure without locking, allocation, or a global lookup. The callback guard writes that exact owner before optional sampling; an explicit finish result can transfer the verdict to a mounted worker's already-owned session/ledger. Quarantine must consult that exact retained authority, not the event/violation ring. Cancellation, terminal outcome retirement, and rebase must carry the same session identity.

Optional site/event snapshots may report contention/saturation and omit a sample; they may not erase a verdict or substitute a healthy state for unknown authority. A fixed preallocated telemetry store or cold site registration is required to remove lazy heap allocation from the callback. No spinlock, blocking retry, larger time grant, global ID-only authority, or whole snapshot lookup is an acceptable repair.

The exact callback verdict requires saturation and mutex-contention laws independent of the global ring; a real job must still quarantine when its telemetry cannot be recorded. Existing plugin close already has its own outer verdict state and must preserve it. Renderer branches that currently use global violation count/snapshot for quarantine need the same exact owner migration, while diagnostics-only readers remain explicitly cold.

## Source Checkpoint

Production was unchanged through all four actual RED cases. The subsequent coherent patch adds a private-field `CallbackVerdict`, explicit `Watchdog::finish`, missing-clock preflight, fixed static telemetry arrays, and nonblocking optional writes. `TraceEvent` is optional when no actual clock exists; no synthetic timestamp or panic after a missing-clock verdict is used. The root fixture oracle now also checks five exact clock-verdict vectors.

Mounted worker authority retains the verdict and quarantined original outcome. An invalid callback produces its preadmitted Fault while the original candidate remains owned for incremental retirement. The checked-out session exposes a borrowed verdict; the numeric operation/generation global lookup was removed. Plugin reserved-route checks and WGPU frame quarantine now use the exact session/guard verdict. The media path receives the mounted Fault without dropping its original candidate. The raw driver now requires an explicit verdict destination; remaining direct calls are test harnesses, while mounted production workers enforce quarantine internally.

`🧪️microsecond-telemetry-verdict-green-r3-native-2026-08-27.txt`:5PASS/0FAIL,15skipped,.127s. Four held-lock cases now return before release. Watchdog fault preservation is asserted on the exact returned callback owner, not the optional ring. The fifth law covers strict equality, missing/backward readings, ring saturation, and contention without losing the retained verdict.

`🧪️microsecond-worker-quarantine-r1-native-2026-08-27.txt`:1PASS/0FAIL,21skipped,.011s summary. The actual WorkerJobSession covers four clock vectors, preserves the original four-byte output under quarantine, rejects zero-item retirement credit, and incrementally releases the exact30bytes including its preadmitted Fault. Same numeric operation/generation on distinct sessions does not share fault authority. Full trace20PASS/0FAIL (.169s) and job22PASS/0FAIL (.066s) are retained in `🧪️microsecond-trace-full-r4-native-2026-08-27.txt` and `🧪️microsecond-job-full-r2-native-2026-08-27.txt`.

The mounted typed-command driver now cancels its publication lease on Fault as well as Cancelled, preventing an already-populated typed completion sidechannel from publishing after worker quarantine. That plugin join is source-ready but not compiled or exercised after this change. Plugin/WGPU native consumers, the actual registered-Fault-to-unchanged-Store law, and cold-close timing remain open. Six other RAII-only watchdog consumers still need explicit verdict adoption; no full callback or all-app completion is claimed.

Canonical source suite before the production patch passed985 (979previous+6newchecks). Native logs use `🧪️microsecond-telemetry-...` names in this master ticket. No cleanup or shared generated output write occurred.
