# 🧮️ Census: the 14336 MB graph memory gate

Read-only audit. No source file, git state, generated folder or ticket state was changed.

Build library root (abbreviated `BUILD`): `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`
Host capacity module (abbreviated `HC`): `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity`

## 1. Where 14336 MB comes from

14336 MB is not a literal in production code. It is a computed cap that equals exactly half of the host's capacity after its floor, on this 32 GB machine.

- Floor: `HC/📏️decision/🟦️.ts:hostFloorMegabytesV1` = `clamp(total * floorBasisPoints/10000, floorMinimumMegabytes, floorMaximumMegabytes)` with `1250`, `1536`, `4096` from `BUILD/🔣️policy.json` path `hostCapacity.memory`. For 32768 MB: `clamp(4096, 1536, 4096)` = 4096.
- Capacity: `BUILD/🌡️host-capacity/🟦️.ts:172` (`capacity=Math.max(0,total-hostFloorMegabytesV1(...))`) = 32768 - 4096 = 28672.
- Rebuild phase reservation: `HC/🪜️phases/🟦️.ts:89` `wanted=Math.min(Math.floor(capacityMegabytes*policy.estimates.rebuildShareBasisPoints/SCALE), build.megabytes)`. With `rebuildShareBasisPoints` = 5000 (`BUILD/🔣️policy.json` `hostCapacity.estimates`) this is `floor(28672*0.5)` = **14336**. It is the share cap, not a measured value.
- The learned `build.megabytes` is at least as large as the cap, so the min picks the cap. The learned value is `estimateHostPhaseV1` (`HC/🪜️phases/🟦️.ts:66`): `quantile(ratioBasisPoints, 9000) * (1+2500/10000) * phaseReferenceMegabytes(1024)`. One outlier ratio of 280000 bp (about 28x the reference, about 28 GB) is enough to saturate the cap.
- The grant is resized into that block by `HC/🪜️phases/🟦️.ts:88-90` (`phase==="rebuild"` branch), then the claim is queued as a plain fixed block at `HC/🎫️gate/🟦️.ts:446` (`fixedMegabytes: Math.max(state.reservedMegabytes, resized.reservedMegabytes)`). It carries no `minimumMegabytes`, so it can never shrink.

Evidence for the saturated estimate, read from `~/Library/Caches/semio/host-capacity/estimates.json` (read only, 134 KB, modified 20:51): `classes["phase:rebuild"].samples` contains `ratioBasisPoints` 280000 (active 2001580 ms, peak at 1645087 ms, a cold run), 116318 (active 540993 ms), and several in the 19000 to 50000 range. The ticket notes agree: `wave-1121/census-workers-ready-blockers.md` line 20 gives the 14,336 MiB request on a 32 GiB host; `result-U.md` line 25 says the learned `phase:rebuild` was 22380 MB from a single sample of 25x the prior after a cold 53-minute run; `result-AB.md` line 38 says the learned ratio is 3.2x the default while stale refreshes measured a 4.3 GB footprint.

Conclusion: the 14336 request is the 50% share cap of this host, saturated by a few cold-run tree readings. The 4.3 GB measured peak is not what is being asked for.

Premise check: on a 16 GB worker the same formula gives floor = `clamp(2048, 1536, 4096)` = 2048, capacity = 14336, and rebuild cap = **7168**, not 14336. So the 14 GB request is not generated on the 16 GB workers by this path. The worker can still wait forever for 7168 MB if its free memory is short, because it hits the same missing deadline described in section 2.

## 2. Why the request waits forever

Decision function: `HC/📏️decision/🟦️.ts:decideHostCapacityV1` (lines 190 to 254). Relevant rules:

- `HC/📏️decision/🟦️.ts:191`: `cap=Math.max(0,host.totalMegabytes-floor)` (budget basis is 10000 bp here, so `budget` is effectively `cap` minus committed reservations).
- `:196` `observed=host.availableMegabytes-floor-unrealized`; `budget=share*budgetBP/10000 - committed`; `headroom=min(observed,budget)`. The incident line "spare 12607 MB" is this headroom.
- `:210` `need(jobs)=min(fixed+jobs*perJob, cap)`; `:214` `fits` requires `need-held <= freeObserved` and `need <= freeBudget`.
- `:218` **the only "never" rule for rebuild requests**: `if(waiter.rebuild!==undefined && request.fixedMegabytes > floor(cap*budgetBP/10000)) refuse("never")`. With `budgetBP=10000` this is `fixed > cap`, which 14336 does not exceed on 32 GB or 16 GB. It never fires for this request.
- `:233-235` "never" for `minimumMegabytes > capacityLimit`. The rebuild claim has no `minimumMegabytes`, so it does not apply.
- `:235` and `:240-241` "starved" needs a shrinkable request (has a minimum) or `active.length===0`. Our own holder is running (2747 MB admitted), so `active.length` is not zero and the starved path is closed.
- `:243-249`: the request is refused with reason `memory` and only sets `blocked` (strict barrier) when `drains` is true, i.e. the request fits after our own grants drain. The drain check `:249` uses `drainedBudget` from the share, not the live ledger.
- `:257-260` `unsatisfiableHostRequestV1`: the only escape. It requires `drainedMegabytes < needMegabytes` (drained = observed room above the floor if all our grants ended), `waited >= bypassMilliseconds` (30 s), `admitted===0`, and `roomMegabytes >= estimates.minimumMegabytes` (512). If it fires, the request is admitted with `min(asked, headroom)` at reason `reduced`.
- `BUILD/🌡️host-capacity/🎫️gate/🟦️.ts:307`: caller applies it as `lone` (only when every decision waits and this request is the first). It hard-codes `replaced:false`.
- `BUILD/🌡️host-capacity/🎫️gate/🟦️.ts:308-310`: throws `memoryNeverFits` or `memoryStarved` only when `mine.reason` is `never` or `starved`. A request that is `memory` with an active holder throws nothing, so no error ever reaches the caller.

Why it hangs in this incident: the holder of 2747 MB is alive, so `drained` (observed room if our grants ended) is still at least the 14336 need on this machine, `unsatisfiableHostRequestV1` returns false, and `never`/`starved` never fire. The request waits for the holder to end. Nothing else ends the wait. There is no admission deadline and no clamp for a non-shrinkable block, and the only "never" rule compares against the budget cap, which is 28672 on a 32 GB box.

Missing detections:
- No check that `fixedMegabytes` is achievable from the machine's free memory plus our own holders' memory, with a deadline.
- No error with an actionable message (the user-visible output stays `[capacity] waiting ... not enough free memory`).
- `BUILD/🔣️policy.json` `hostCapacity.classCeilings` has `{"name":"read-parent","pattern":"^(?:show|graph)(?::|$)","megabytes":4096}`. It is applied only to the floor (`HC/🪜️phases/🟦️.ts:77`: `floor=Math.min(declared??Infinity, ...)`), not to the rebuild phase at `:89`. So the 4096 MB class ceiling for `show`/`graph` does not limit the 14336 rebuild block.

## 3. Does the graph computation need that much?

Measured data available:
- Learned rebuild samples in `estimates.json` range from about 2 GB to a saturating 28 GB (section 1). The high samples come from cold runs with tree readings that count shared and compressed pages more than once. `HC/🪜️phases/🟦️.ts:84` already says this ("a tree reading of a thrashing builder counts shared and compressed pages many times").
- Peak learning is clamped only at `capacity` (`BUILD/🌡️host-capacity/🟦️.ts:185` `ceilingMegabytes: capacity`), not at the rebuild share, so a thrash reading can reach 28 GB and be learned.
- Measured stale-refresh peaks: 4.3 GB footprint (result-AB line 38).

Knobs that scale the graph work down:
- `BUILD/🔣️policy.json` `hostCapacity.estimates.rebuildShareBasisPoints` (5000) and `maximumRebuildSlots` (4) (`HC/🪜️phases/🟦️.ts:98-101` `hostRebuildSlotsV1`: `floor(share/size)`, which is 1 slot at 14336 MB per rebuild on 32 GB).
- `BUILD/🌡️host-capacity/🟦️.ts` env `SEMIO_BUILD_PARALLEL` (`:61`) and `cargo --parallelism`.
- Graph plan memory: `BUILD/🧠️memory/🪜️plan-stage/🧬️schema/🔣️.json` `Policy.const`: `hostShare` 0.3, `treeShare` 0.4, `minimumMegabytes` 2048, `maximumMegabytes` 16384, `serveHostMegabytes` 24000, env `SEMIO_GRAPH_PLAN_LIMIT_MB` (`off` disables). `BUILD/🧠️memory/🪜️plan-stage/🟨️.mjs:43-50`.
- Farm width: `BUILD/🧠️memory/🪜️plan-stage/🟨️.mjs:79` `planFarmWorkersV1` and `:73` `farmWidthV1`, bounded by `treeShare`, with `SEMIO_GRAPH_CUT_CONCURRENCY`.
- Lean slice: `LEAN_SLICE_POLICY_V1` (`BUILD/🧠️memory/🪜️plan-stage/🟨️.mjs:49`).
- Resident slice ceiling: `HOST_POLICY_V1.sliceHostElasticity` in `BUILD/🏡️resident/🧬️schema/🔣️.json` (`maximumMegabytes` 8192, `hostCapBasisPoints` 4000).

The request is sized by a learned share, not a measured need. The graph computation probably needs roughly 4 to 5 GB for a refresh; the 14 GB comes from the cap plus a polluted quantile.

## 4. Concrete fix proposal

A. Clamp the request to what the machine can actually give, and refuse with an error instead of waiting forever.
- `HC/🪜️phases/🟦️.ts:89`: also clamp `wanted` to `total - floor - unmanaged` and to `hostProfileV1` physical memory minus `estimates.minimumMegabytes` (already implied, but make it explicit so a 16 GB machine never requests more than its own achievable ceiling).
- `BUILD/🌡️host-capacity/🎫️gate/🟦️.ts:446` (`quote`): give the rebuild claim a `minimumMegabytes` equal to the execution estimate (`estimate.phases.execution.megabytes` at minimum jobs). Then `HC/📏️decision/🟦️.ts:233-240` makes it shrinkable and it is admitted with the free memory (`shrunk`), and `BUILD/🔼️upgrade` grows it later.
- `HC/📏️decision/🟦️.ts:218`: extend the "never" rule for rebuild requests to `fixedMegabytes > total - floor - foreignMegabytes` (the achievable ceiling), not only the budget. Keep it pure.
- `BUILD/🌡️host-capacity/🎫️gate/🟦️.ts:307-310`: add a deadline. After `wait.overrideAfterMilliseconds` (120 s) with `memory` and a non-zero holder set, admit with `min(asked, headroom)` at reason `reduced` and `jobs=minimumJobs`, or throw a typed error `memoryUnachievable` naming need, spare and holder. Do not wait indefinitely.
- `unsatisfiableHostRequestV1` (`HC/📏️decision/🟦️.ts:259-261`): compare `needMegabytes` to `drained` computed from the live ledger (already) and also to the rebuild cap, so it fires with a holder alive. Keep `replaced` and `quiet` honest instead of hard-coded `false` at `gate/🟦️.ts:307`.

B. Degrade parallelism instead of memory.
- `HC/🪜️phases/🟦️.ts:98-101` `hostRebuildSlotsV1`: already bounded by share. Make the claim pick the largest slot count that fits the free memory, and admit at reduced parallelism rather than waiting for the full block.

C. Stop learning from saturated readings.
- `BUILD/🌡️host-capacity/🟦️.ts:185`: clamp recorded rebuild peaks at `rebuildShareBasisPoints * capacity` (not `capacity`) and drop samples whose ratio exceeds that cap, so a thrash reading cannot become the next request. Keep the quantile and margin.
- Consider raising `minimumSamples` for `phase:rebuild` above 3 (`BUILD/🔣️policy.json` `estimates.minimumSamples`), so one cold run cannot dominate.

D. Tests to extend (all under `HC/🧪️tests/`):
- `🛟️unsatisfiable/🟦️.ts`: add a case where a holder is alive, drained < need, and the request is a rebuild claim; expect a bounded admission or the typed error, not a wait.
- `📉️shrink/🟦️.ts` and `🪜️phases/🟦️.ts`: a rebuild block with `minimumMegabytes` on a 16 GB profile (`total=16384`, floor 2048, cap 14336) must be clamped to 7168 and admitted when free memory covers the minimum.
- `🧮️graph-plan/🟦️.ts`: `reserveGraphPlanCapacityV1` with `megabytes` above achievable must return the granted megabytes, not block.
- `🧮️farm-width/🟦️.ts` and `BUILD/🧠️memory/🪜️plan-stage/🧪️tests/📐️farm-width/🟦️.ts`: worker count scales to the grant.
- `🌡️host-capacity/🧪️tests/🪜️phases/🟦️.ts`: a sample of 280000 bp must not produce a learned rebuild above the clamped share.
- Language-agnostic oracle: add the same case to `🧪️tests/📐️estimates/🐍️oracle/🐍️.py` so the Python oracle and TypeScript agree on the clamp.

Not checked (out of scope or not reached in this read): the 11.6 h wait timing against the lease ledger, the exact composition of the 12607 MB spare on the live host (needs a live `repo fleet status` read), and whether the fleet store copies `estimates.json` between machines.
