# Wave B42 — the `cancelJob` guest turn that never yields

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Battery evidence: `🗑️generated/probe-2026-09-12T08-08-54.md`
(wasm #56), lines 1071–1073:

```
[DEBUG] shard 0 terminated by the host watchdog: the worker was silent for 18603 ms;
outstanding: cancelJob puzzle#1 started 18603 ms ago. A guest turn that never yields blocks the
worker's event loop and its progress ticker with it …
[DEBUG] action failed engagementAbort undefined Error: shard 0 terminated by the host watchdog …
```

`engagement-abort` itself verdicts PASS at 720.9 s; the shard death surfaces at 762.1 s
(`step outliner-rows … newFaults=…`), then `PluginRuntime: shard 0 lost, restoring actors: puzzle#1`
and 10 guest-death faults.

## 1. Wire path of the cancel (read, before measuring)

* `engagement_abort` — `✏️editor/🎮️commands/🛑️engagement-abort/🦀️.rs:26` — cancels the live plan
  (`precompute.cancel_fill_job_for`) and pushes `Effect::CancelJob { job }` + `SetActiveTool ""`.
* React host — `📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:2263-2269` — `cancel-job`
  never becomes a `requestedEffects` entry; it is host work:
  `serializeCommandIngressForActor(actorId, () => shardClient.cancelJob(actorId, job))`.
* `ShardClient.cancelJob` — `🎭️actor/📮️shard-client/🟦️.ts:2082` — sends `{kind:"cancelJob", requestId}`,
  i.e. the **request-id** branch of the worker, not the fire-and-forget one.
* Shard worker — `🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:531` — `await actor.api.cancelJob(msg.job)`
  inside `beginRequest()`; that is the outstanding request the watchdog names.
* Guest — `🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs:403` `cancel_job` → `owner.cancel()` + `jobs.remove(&job)`.
* Puzzle owner — `✏️editor/⏳️precompute/🦀️.rs:3277` `BoundedJob::cancel` for `FILL_JOB_KIND`.

## 2. Synchronous drains found by reading (candidates for the runaway)

| file:line | shape |
| --- | --- |
| `⏳️precompute/🦀️.rs:3179-3201` | `Drop for Puzzle3dPrecomputeSession` loops `FILL_ENVELOPE_SESSION_CLOSE_TURNS = FILL_ENVELOPE_MAX_OPERATIONS * (FILL_ENVELOPE_MAX_ITEMS + 9) = 4 * 65 545 = 262 180` `pump_fill_terminal_step()` calls with **no deadline and no yield** |
| `⏳️precompute/🦀️.rs:2453-2461` | `Drop for Puzzle3dFillSession` re-installs itself into a throwaway `Puzzle3dPrecomputeSession`, whose `Drop` then runs the loop above |
| `✏️editor/🦀️.rs:3036-3053` | `puzzle3d_session_check_in` **drops** the whole `Puzzle3dSessionState` (fill session included) on a stale lease / byte-budget refusal / contended registry |
| `✏️editor/🦀️.rs:3036 (retire)` | `Puzzle3dSessionRegistry::retire` drops a slot's state **while holding the registry mutex** |
| `⏳️precompute/🦀️.rs:1559` | `CollisionMutationStep::Rejected(mut rejected) => while !rejected.retire_one() {}` — unbounded |

Each `pump_fill_terminal_step` at `close_cursor == 2` retires exactly ONE `FillBuilderRetirementCursor`
owner, so the drain is O(retained owners) — Nakagin-scale plans hold thousands.

## 3. Measurement (in progress)

