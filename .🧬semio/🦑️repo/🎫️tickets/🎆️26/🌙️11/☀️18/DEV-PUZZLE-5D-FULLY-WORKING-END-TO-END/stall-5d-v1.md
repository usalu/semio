# Stall diagnosis, 5D v1 (interim)

Logs: `🗑️generated/stall5d-1.log` .. `stall5d-15.log` (run 1 = baseline, 7/8 = after fixes 1-3, 13/14/15 = probes).

## Fixed (verified by logs)
1. Deflate cursor drop panicked during unwinding (SIGABRT of whole test binary): `🧰️framework/🔨️modules/🗜️deflate/🛫️encode/⏳️cursor/🦀️.rs` `Drop for RetainedDeflateEncoder` now returns early when `std::thread::panicking()`. Verified in stall5d-1.log (failures are plain FAILED, no abort).
2. Fixture item stock exhausted: `🧰️framework/.../🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json` `grant.maximumItems` 65536 -> 16777216. The 5D world window config genesis (pack measuring pass) costs ~10 items/unit and ~6000 units, i.e. more than the 65536 lifetime items of one fixture (probe stall5d-5/6). After the change genesis finishes (stall5d-7).
3. 5D window config owner lacked `store_disposer_birth()` ("Window owner must declare its original disposer birth", stall5d-7.log): added to `✏️s/.../🖐️5d/.../✏️editor/🪟️window/🦀️.rs` owners! macro (2D's owners! macro lacks it as well; not touched).
4. `RefusedWorkerSource::admission_demands` quoted only depth 1 although worker birth reserves a 39712-byte authority allocation: now quotes capacity (`🔌️plugin/👷️job/🚪️session/🦀️.rs`, `birth_capacity_bytes` added in the worker crate). Quote-correctness fix, not sufficient alone.
5. 5D testkit: `drain_settled` now drains ui-progress and local-interaction replies like the 2D testkit; `maintain` funds the host's maintenance policy grant instead of the 1-item quote (the quote grant has depth 1 / release 0).
6. Laws settle loop sleeps 1 ms on idle turns instead of spinning (`registered-app-construction-and-typed-operation-settlement`).

## Update after stall5d-16..19
7. Root cause of the target_volume worker-admission stall: the 5D testkit `settle` never renewed the per-turn payload page grant (`cx.payload_page_granted` stays true on the long-lived fixture cx, so `reserve_page` in the worker fault-payload builder returns no progress for ever; stall5d-16 showed `pages=Some(0) staged=None`). Fix: `semio_plugin::__job::testkit::renew_payload_page_grant(app.1.cx())` each settle iteration (as 3D action-producer does). After it (stall5d-17/19) there is no stall any more: the worker job now runs and the tests fail within 2 s with a panic instead.
8. All `[DEBUG] stall5d` probes removed (grep clean).

## New failure (stall5d-19.log, RUST_BACKTRACE=1)
`original work scope retains an issued action or committed proof` (semio-framework-value-control-contract work-action/🦀️.rs:24, Drop of OriginalWorkScope) raised from `Emit::prepare_child_original` (`🔌️plugin/📜️declaration/📨️emission/🦀️.rs:548`, `with_original_effect` -> `prepare_child_effect` -> `effect.nested(child)`), called from `ArtifactRetainedCommandJob` step (retained-command-job 613/648/652) on the worker pump. i.e. the 5D addTargetVolume job emits child emits and the child preparation effect leaves an issued action open when the closure returns. agent_lane hits the same panic (stall5d-17.log). Narrowest next piece: why `prepare_child_effect` returns with a live nested action (early `?`/Err path after `effect.nested(child)`, or a Pending return while an action is issued).

## Superseded: earlier "Still failing"
- target_volume dispatch tests (stall5d-15.log, outliner test alone): op `addTargetVolume` stays in Worker stage, session not admitted (`s=false rej=true/true`). The admission is stuck in `OriginalWorkerPageReservation::admit_step` (`🔨️modules/🧵️job/📬️outcome/👷️worker/🦀️.rs` line ~25-40): `body_ready=true`, fault payload builder `init=Some(true)`, grant is generous (64 items, 32 KiB copy, 16 MiB capacity), process payload bytes 0, yet it never reaches `ready()` over 1M turns. So the fault-payload page staging (`begin_staged_page_funded` / `write_staged_source_funded` / `commit_staged_page_funded` / `seal`) makes no progress under the fixture caller. Narrowest next probe: print `builder.staged_page_len()`, `page_count()`, `published()` in that branch.
- agent_lane (stall5d-14.log): admission succeeds under policy grants, then `setActiveExample` worker session (s=true) makes no progress for 4096 idle turns (~5 s). Unknown whether the worker job is slow or blocked; stall5d-13/14 diag line shows op in Worker stage with no pending outcome.

## Hypothesis
Both are worker-job plumbing under the fixture caller (not the 5D document code): page reservation for the worker fault page stalls (target_volume), and the admitted worker job never returns an outcome (agent_lane). 2D passes with the same framework, so look for what the 5D tool job type adds (size 39712-byte authority state, larger job).

## Probes
All removed (no `stall5d` string left in `🧰️framework` or the puzzle plugin).
