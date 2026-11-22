# Result E73 (worker-memory), 16:15 CET

Landing: `T/landings.md` "Wave 1122 E73". Lock remote-execution taken and released. Scratch: `.../scratchpad/E73/` (status dump, lock helper), nothing in `🗑️generated`.

## 1. Evidence (read-only, `S/origins/<o>/`, 15:50)
All 13 workers are 16 GB Macs (`pool.json totalMegabytes 16384`), so "m01/m06 are on time" is load timing, not size.
| role | measurement (15:50) |
|---|---|
| slice process (workspace stage) | `slice-memory.json` rss 1.0-1.8 GB, tree 1.2-1.3 GB (m08 1274, m10 1031, m12 1056, m01 998, m06 1781, m11 3312); CPU 4-12 % of uptime; m08 owner lookups average 5.2 s, maximum 378 s |
| stage tree peak (`ready.json peakMegabytes`) | m08 10208, m01 9392, m10 6739, m12 6163, m06 7539: this is a sum of RSS over the tree, so it also counts shared pages (E18 found the same inflation under thrash); the slice itself holds about 1.2 GB |
| owner puller (`pool.json`) | slots/target m10 5-6, m12 3-4, m06 3-4, m08 2, children 1-3, childMegabytes 735-3120, ownersPerMinute 0-1 |
| remote-execution agent (`repo remote-execution status`) | **m10 `running 3`, `headroom true`, offloadedIn 174 while its workspace stage runs**; m03 running 7 (headroom false), m06/m07 running 1; m08/m01/m11/m12 headroom false |
Per-role swap and page-outs are not in `S`: only the coordinator keeps `host-capacity/sample.json`. Page-outs of the workers cannot be read from here.

## 2. Cause
Pool sizing (`ownerPoolSizingV1`) and agent admission (`admitRemoteExecutionActionV1`) both read the memory the stage holds NOW (available memory, observed grants). A reservation the stage has not grown into is not counted by the gate (grants count `observed ?? reserved`, a pool counts unrealized memory only up to 15 % of the machine). So while the stage is still small, the puller starts 3-6 children of 0.8-1.4 GB and the agent takes offloaded compiles (m10: 3 running), and the stage then grows into memory that is already taken: swap, a 30 s timer firing after 1114 s (E61).

## 3. Change (schema first, one place)
- New member `B/🌐️remote-execution/🌡️load/🪑️stage-claim/` (`🧬️schema/🔣️.json` Policy and Claim, `🟦️.ts`): the readiness loop writes `ready-stage-claim.json` beside the host-capacity files `{stage,pid,entitlementMegabytes,observedMegabytes,atMs}`; stale after 120 s (a dead loop never holds memory back).
- Entitlement (`readyStageEntitlementV1`): 0.9 quantile + 25 % of the stage's completed (`ok`) earlier runs from `ready-timings.json` (failed, stalled, interrupted runs teach nothing, as for every learned size, E18), 30 % of the machine when none completed (4915 MB on 16 GB, 2457 on 8 GB), never below 1024 MB nor above 50 % of the machine.
- `withReadyStageV1`: the reader `readRemoteExecutionHostViewV1` (single source of the pool, the agent and the load report) adds the stage as a grant (1 core, what it holds) and takes `entitlement - observed` from the host's available memory. Pool children, offload admission and the reported `freeMegabytes` therefore leave the stage its room with no rule of their own; nothing changes for a machine without a stage.
- Agent: while a stage runs, any pressure above normal refuses an offload (`pressure-warn` / `pressure-critical`, existing codes).
- Publisher in `🚦️ready/🟦️.ts` (`runReadyStageV1`): claim at stage start (observed 0), at every sampler tick (current tree RSS, throttled to a 64 MB change or 5 s), cleared at completion. Only with `SEMIO_FLEET_STORE_ROOT` (a fleet worker), so tests and the coordinator are untouched.
- 16 GB oracle numbers (Python, `🐍️.py`): available 8000, stage at 1200, child 800 MB: children 2 (was 7); available 10000: 5 (was 9); once the stage has grown to its entitlement both are equal.

## 4. Proof
Tests (`repo test-file`): new `🧪️tests/🪑️stage-claim` 8/0 (entitlement 4 host sizes x 7 histories, hold and staleness, file round trip/foreign/stale/malformed, publisher throttling and single learning, reader on a simulated 16 GB view, pool children vs the Python oracle incl. 16 GB fixtures, agent verdicts: admit without stage, refuse `memory` while the stage still has to grow, admit when grown, refuse under warn/critical, oracle `--check`), `🏃️run` 1/0 (a real `runReadyStageV1` of a fleet worker publishes and removes the claim, a machine outside a fleet publishes nothing). Dependants: why 11/0, remote-execution main 17/0, fleet 9/0, pool 7/0, ready main 57/0, watchdog 9/0, stale-stage 14/0, code-stop 4/0, stage-completion 8/0.

## NOT verified
- Live effect: nothing proven on m08/m10/m12. The change reaches the workers through the replica; their old stage processes keep old code until the next stage attempt, so the claim appears only after that. To check after rollout: `S/origins/g1-m10/pool.json` (`slots`/`target` lower while `ready.json` says running, children 0-2), scheduler `repo remote-execution status` (m10 `running` 0 while its stage runs, headroom false), slice timer gaps in `slice-memory.json` (30 s steady), and ownersPerMinute / owners taken per minute in the stage log. Pageouts and loop lag cannot be read from `S`.
- The 30 % default entitlement is a model value; the real slice holds about 1.2 GB. If workers still thrash the lever is `shareBasisPoints` (schema Policy), no code.
- Whether the lower pool throughput on m08/m10 slows the stage (children publish owners the slice fetches); the other machines' pullers keep serving.
- Windows sampler branch (`sample.megabytes`) edited, not run on Windows.

## What others must know
- Do not read `peakMegabytes` of a stage as a real footprint on a thrashing host (RSS sum).
- The host view now has an optional `stage` and one extra grant `ready-stage` while a stage runs: the load report's `running` counts it.
