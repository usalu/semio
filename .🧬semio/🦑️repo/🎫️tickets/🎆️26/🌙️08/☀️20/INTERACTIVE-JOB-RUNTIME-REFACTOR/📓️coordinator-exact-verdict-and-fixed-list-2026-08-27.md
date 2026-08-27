# Exact Verdict and Initialized List Review

## Native Fixed List

The coordinator read UiFixedList's private Option<Vec<T>> implementation and all3new native laws. Explicit reserve precedes reserved pushes; rejected pushes preserve their owner. Pop transfers one initialized payload. Empty backing release rejects nonempty ownership, has a separate terminal condition, and avoids traversing uninitialized capacity. N=0 and zero-sized payloads are tested; the patch-domain bound remains1153. Cold clone/serde/owned-iterator conveniences are not granted an interactive-path exemption.

The actual retained native output was reviewed: `🧪️member-ui-full-green-r5-native-2026-08-27.txt` runs102tests,102passed,0skipped,.911s. The prior initialized-length test actually failed before the correction. Typed node/document descendants remain open: a document cannot claim terminal merely because it dropped a UiNodeRecord whose UiValue descendants were only queued globally. The owning lane is mounting that semantic RED before its typed-cursor implementation.

## Exact Watchdog Authority

The next typed document law now has an actual RED: `🧪️member-ui-document-descendants-red-r1-native-2026-08-27.txt` reports0passed/1failed/102skipped,.021s. The expected exact descendant-empty condition is false after the current document terminal. The test drains stranded owners before its final assertion; this is a semantic ownership failure, not a secondary cleanup abort. The owning lane is implementing the typed in-place cursor from this result.

The coordinator read CallbackVerdict, Watchdog::finish, the fixed static telemetry stores, worker authority quarantine, exact outcome access and incremental quarantined-payload close. Optional telemetry writes use try_lock; retained callback/session verdicts do not depend on ring contents, eviction or numeric operation/generation lookup. Missing/backward clocks fail; elapsed8000microseconds is a fault. Worker authority substitutes its preadmitted Fault and retains the original candidate for explicit close.

The actual `🧪️microsecond-telemetry-verdict-green-r3-native-2026-08-27.txt` output reports5passed,15skipped,.127s. It includes4held-lock laws and exact verdict survival under saturation/invalid clocks. Root read the complete native test bodies. The100ms rendezvous proves nonblocking behavior under a deliberately held mutex, not an8ms timing certificate. Mounted typed-command sidechannel cancellation on Fault is a separate join under correction; native worker success alone does not prove it.

## Remaining RAII-Only Callbacks

Subsequent actual full native outputs were also read: trace20/20 in.169s (`🧪️microsecond-trace-full-r4-native-2026-08-27.txt`) and job22/22 in.066s (`🧪️microsecond-job-full-r2-native-2026-08-27.txt`). Root also read the targeted worker law's actual DEBUG output: four clock vectors use the same numeric operation/generation while retaining each exact session; each reports30retired bytes, comprising the original4-byte candidate plus26-byte preadmitted fault. These are small-crate native gates, not the mounted Plugin or WGPU graph.

A scoped Rust source scan of framework and s found no production Watchdog::violations/violation_count or removed watchdog_step_overrun_us caller. It did find6production guards that only Drop their watchdog:

- WGPU winit_app: enqueue_host_event, enqueue_host_metrics, redraw and redraw_offscreen_worker.
- Plugin host shard: step_job and execute_turn.

Their source bodies were read. They currently record optional telemetry without consuming an exact verdict to prevent later publication/quarantine the owner. The shard guards also span awaited calls, which needs exact execution-step review. These are open integration work, not excused by the new trace primitive. They have been assigned to the compiler/runtime lane; WGPU's current stable32-preimage catalog boundary must be coordinated before another source edit.

## Preserved Scope

Original-two-only cold close R6 remains RED/SIGABRT at8519/19965microseconds; no fresh heavy plugin/WGPU graph has run after these changes. Source/body candidates are not asserted as the sole cause. All-app native/Wasm/browser/timing gates remain open.
