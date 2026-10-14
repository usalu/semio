# Actor Library Current Job Close Audit

Fresh read-only Actor library source scan on 2026-10-13. No builds/tests run; no conclusion about Kernel2458 execution.

No current production Actor occurrence of `WorkerJobCloseStep`, `pump_worker_job_retirements`, `InteractiveJobCloseStep::Refused`, or a direct InteractiveJob close call was found. Consequently no concrete Actor library stale grant/Result/removed-variant compiler blocker was established. Package Rust glue mounts the actual Actor root and reexports it; libtest findings must not be attributed to this library compilation.

The root grant-taking `JobTurnBridge::close_step` at1818 and payload-authority call at1833 are explicitly `#[cfg(test)]` on both struct and impl. Root `JobReplayStore::close_step` at1529 and progress-store close at2612 already accept their actual `&mut job::StepContext`; their result enums are Actor-owned `JobReplayCloseStep` / `JobProgressCloseStep`, not canonical WorkerJobCloseStep. Record byte-page close calls at1485/1538/1572 are distinct Actor-owned replay record APIs and are not stale Job signatures.

Actual provider boundary `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/🤝️provider/🦀️.rs:14–15` accepts the genuine context for cancel/step and returns complete `OriginalTurnFailure`. No internal context fabrication is needed.

One separate semantic custody concern exists in `🎟️retained-turn/💼️job/📥️source/🦀️.rs:85–88`: OriginalJobSourceCursor close already receives its genuine original context, but the child retirement error handler debits fuel and calls `let _ = cx.consume_retained(progress)`, discarding any secondary original receipt refusal while returning only primary ValueError. This is not a proven compilation blocker and should not prompt a speculative whole-API migration. If the canonical complete failure contract is required at this source owner boundary, preserve that secondary error in the actual failure owner there, without calling canonical debit again after this existing debit. The successful child branch already consumes its receipt once.

Actor unit libtest contains stale bare-grant Job close calls and removed Refused constructions (unit123/249/254), but those are outside this requested library-only floor. Current Kernel graph qualification must use its actual compiler diagnostics before assigning any of these as blockers.
