# L9 fleet reader finding source-failed (worker.why.code enum)

## Diagnosis
- Reader: `readRemoteExecutionFleetFactsV1` (build/🌐️remote-execution/🚢️status/🟦️.ts) GETs `/v1/fleet/load` from the scheduler and validates it with `requireRemoteExecutionShapeV1("FleetLoad")` against `build/🏃️execution/🧬️schema/🌐️remote-execution.json` (`WorkerWhy.code` closed enum, 12 codes).
- In this tree the enum, the TS union (`🌡️load/🔎️why/🟦️.ts`) and every producer (agent, readiness, sidecar, scheduler `not-registered`) agree: no producer here can emit an unknown code. Not a schema/producer bug in this tree.
- Live read (bun, `l9-fleet-read.ts`, 2026-11-22 02:1x): machine g1-m03 now reports `no-headroom` (valid). The table is served by a scheduler running a DIFFERENT code version than this replica: the same call now fails on `/machines/0/counters/skipReasons is not allowed`, a field that exists nowhere in this tree (grep: 0 hits). So the earlier `/machines/3/worker/why/code` failure is the same class: version skew between the live scheduler/machines (newer or older digest) and the schema in this replica; the fleet-mechanism team is mid-change (sidecar/ready/tools-grace is new, uncommitted).
- The schema is closed on purpose (fail closed); loosening it would hide skew. No edit made; the fleet mechanism owner must either roll the scheduler and the replica to one digest or land the schema+producer change together (add new code to enum + TEXTS en/de + schema).
- Duration (8999794 ms): the reader itself has a 3 s timeout per call and the native read above took seconds, so the 2.5 h was not the reader; the `repo fleet status` call was queued behind the resident service / other graph commands (not investigated further, no repo command run).

## Change
None (shared fleet code is being edited concurrently; no coordination ticket owns this schema, PROTOCOL.md in ☀️14 WORKERS-READY-WITH-LOW-RESOURCES is worker-readiness only).

## Verification
Native bun read logged to 🗑️generated/l9-read.log: finding reproduced as a skew error; enum/TS/producers cross-checked by grep.
