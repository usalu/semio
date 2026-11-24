# H4: worker headroom for component finals, and final fetch

RE = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/`. Evidence read-only: `/Users/ueli/.sinc/play/.cache/b/r/m/u/x/offload.jsonl`, the play ledger `/Users/ueli/.sinc/play/.cache/b/r/m/u/c/*.peak`, the fleet action cache `.cache/b/p/m/fleet/cache-server/v1/main/ac`, `/Users/Shared/semio-store/v1/origins/g1-*/memory.json`, the scheduler's `/v1/fleet/load` and `/v1/fleet/idle?peak=` (signed read, `H4-data/probe-fleet.ts`). Nothing restarted, nothing written under `.sinc/play`, no scratch build, no sub-agents. Lock `lock.worker-headroom` released. Landing: `T/landings.md` "H4".

## 1. Headroom, per worker, with numbers

### 1.1 What a final really needs (measured)
Ledger entries written today for hub finals (MB, `c/*_component.peak` of the play clone): note 1916, fem 2724, procedural 3655, playbook 5300, norm_energy 4106, norm_eurocode 5053, norm_eurocode_materials 3905, norm_eurocode_structural 2790, puzzle 2147, draw 3238, layout 3787, dag 4011, energy 7063 (local); flow_bim 2181, flow_list 2231 (plugins). Median of the 13 hub finals: **3787 MB**. The 3655 MB of the plan line ("3 worker slots fit one 3655 MB final") is the median of learned-else-family peaks the pinned driver computed at its start (O11 rule), i.e. already measured, not an unmeasured quantile.
Thread counts (aux metadata in the fleet AC, the only two final results with `codegen-threads` today): layout **3787 MB at 4 threads** (m08), dag **4011 MB at 1 thread** (m02, pressure warn). The old ledger mixed counts in one number.

### 1.2 What the rule asks
The pinned requester declares `learned x 1.25` (draw 4048, layout 4734, dag 5014, energy 8829). The worker admits when that fits `available - floor(2048 on 16 GB) - stage hold (25 % of the stage tree)`; the scheduler places when `freeMegabytes - committed >= declared`; and, until this landing, a worker whose Ready stage runs lent **at most 1 unit at all** (W4 `stageConcurrency`), so a worker busy with any small unit refused the final (`slots`).

### 1.3 Per worker (memory.json 18:35; scheduler table 17:55)
| worker | available | compressor | swap | pressure | repo subjects | room above warden guard (avail - 1638) | rule room (avail - 2048 - stage hold) | scheduler free 17:55 |
|---|---|---|---|---|---|---|---|---|
| m01 | 5537 | 2755 | 1573 | normal | 1369 (slice 1.4 GB) | 3899 | 3147 | 2533 |
| m02 | 3823 | 6429 | 3790 | warn | 1212 | 2185 | 1751 | 2726 |
| m03 | 3233 | 7650 | 1233 | warn | 286 | 1595 | 1113 | 933 |
| m06 | 5719 | 2490 | 2984 | normal | 4199 | 4081 | 3503 | 4197 (busy 1: refused by the 1-unit cap) |
| m07 | 6159 | 1975 | 1640 | normal | 2175 | 4521 | 3649 | 2909 |
| m08 | 3846 | 6011 | 2340 | warn | 2742 | 2208 | 1769 | 1647 |
| m10 | 5447 | 3248 | 2522 | normal | 1481 | 3809 | 3227 | 4789 |
| m11 | 7248 | 1267 | 750 | normal | 2335 | 5610 | 4616 | 3449 (refusing: `slots`, 1 unit running) |
| m12 | 7334 | 2498 | 914 | normal | 2132 | 5696 | 4768 | 5960 |
| m13 | 4996 | 4169 | 1064 | normal | 283 | 3358 | 2877 | 3281 |
`/v1/fleet/idle` at 17:55: slots 3 at 3072 MB, 2 at 4096, 1 at 5120, 0 from 6144 (18:33: 3, 1, 0).

What holds the memory: on every 16 GB Mac 9 to 13 GB are not available, of which the classified repo subjects (Ready stage, slice worker, pool child, local units, cargo) are only 0.3 to 4.2 GB; the rest is wired memory, the compressor (1.3 to 7.7 GB; m02/m03/m08 are at `warn` from it) and unclassified processes (sidecar roles, replica). The pool child "grant 4915" (`pool.json grantMegabytes`) is the 30 % stage entitlement default the puller reports; it is not booked in the remote admission (`withReadyStageV1` books the stage at its measured size plus 25 %). Warden reservations hold nothing in the admission (the warden acts on processes, it books no grant).

Answer: with measured peaks, no margin, above the warden's suspend line (10 %), the 10 macOS workers could carry **6 hub finals of median size at once** at 18:35 (m01, m06, m07, m10, m11, m12 one each); the rule (x1.25 above the 2048 floor and the stage hold) admits **1** (m12), and the old 1-unit stage cap cut that further whenever the worker ran any other unit. Run time data agree: at 17:03-17:05 three finals ran remotely at once (draw m03, layout m08, dag m02), after 17:25 none.

### 1.4 Double counting: checked, and where it was
- Worker admission vs. host decision (`RE🌡️load/🟦️.ts`, `decideHostCapacityV1` lines 216-221 of `🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts`): the stage grant (observed) and the hold (25 %) are counted once; with `budgetBasisPoints` 10000 the remote grants cancel in the budget (`managed` = `committed`), so the old code charged nothing twice. Child grants inside the stage tree reserve their growth for at most the 60 s ramp on top of the stage hold: a short, real overlap, not the binding term.
- The real defect was the opposite: running remote units were modelled as grants admitted one hour ago with no observation, so the worker's own admission reserved **none** of a running final's remaining growth: two finals admitted a second apart both counted on the memory the first was about to take; only the scheduler's charge protected it. Lending by memory (below) needs that fixed first.
- Report vs scheduler: the report must not reserve that growth, because the scheduler charges `declared - measured` per task (O12 `placementChargeMegabytesV1`). Now kept apart explicitly (single charge).

## 2. Changes (landed)
1. **Lend by measured memory** (`RE🌡️load/🟦️.ts` `evaluateRemoteExecutionAdmissionV1`, schema `RE🌡️load/🧬️schema/🔣️.json`): `stageConcurrency` removed; while the Ready stage runs, slots are limited by `concurrency` and the memory decision only (the stage keeps its own reserve through its claim). Trade-off: more units beside a stage; the warden ranks `remote-unit` first if the stage then grows (note to the memory session, 2).
2. **Each running unit charged once in the worker** (`RemoteExecutionHeldUnitV1 {megabytes, usedMegabytes}`): remote grants are ramping grants with `reserved = declared`, `observed = measured` (the O12 task meter, `heldUnits` in `RE👷️worker/🟦️.ts`), so the unrealized `declared - used` is kept free and what the unit holds is counted once. The load report (`buildRemoteExecutionLoadReportV1`) passes the units at their used size only (scheduler charges the rest). Without measurements (Windows) a unit is taken as already in the sample (the old behaviour).
3. **Peak per (crate, threads)** (`RE🚀️offload/🚦️admission/🟦️.ts`): `<crate>.peak` is a JSON object thread count -> MB; `writeUnitPeakV1(root, crate, mb, threads, force)` learns at that count (remote: the worker's `codegen-threads`, else 1; local compile: 1; kill: the fair share); `readUnitPeakV1(root, crate, threads)` = the measurement at that count, else the nearest below + `worker.codegen.extraMegabytesPerThread` (512) per thread, never above a measurement with more threads, else the nearest above (`offloadPeakAtThreadsMegabytesV1`, Python oracle vectors). Finals are declared at the fair share (`offloadDeclaredThreadsV1`: 4), other units at 1; the family estimate reads at the fair share. Callers: wrapper `RE🚀️offload/🟦️.ts`, unit-cache `📜️script.ts`, `🧩️driver-stages`. Takes effect for main requesters; the pinned play clone keeps its own ledger code (old plain-number files read as "not measured" by the new code: they are relearned on the next measurement).
4. **Not done (scheduler closure, would restart the scheduler during the play build)**: the 12.5 % floor and the x1.25 margin stay; both are defensible separately, stacked they cost ~2.7 GB per final on a 16 GB worker. Recommendation for after the play build: admit a lent unit whose peak was measured at its thread count against the warden's guard line instead of the floor plus margin.
5. **Pool child yield**: needs the memory session's interface; written as `note-H4-worker-headroom-for-finals.md` in its ticket (classify sidecar roles, a yield ask for admitted lent units, rank of a final by elapsed work). Nothing written into its files.

## 3. Final fetch 42-76 s
- Placement is not the cost: note on m08 at 13:12 placed 3.7 GiB in **4.6 s** (cached), so clone + chmod + stat of ~1,100 files is ~4 s. Cached blobs are **not** re-verified per action (`RemoteExecutionBlobCacheV1.path`: stat + utimes only); directories re-hash only their small proto bytes. Placement is already one 32-wide gate (F5).
- The minute is the network: fetch time / shipped bytes for today's finals = 66-100 MB/s (layout 42.2 s for 4.96 GiB, dag 46.2 s / 4.68 GiB, draw 48.2 s / 4.06 GiB, puzzle 75.9 s / 6.07 GiB, flow_bim 15.2 s / 1.77 GiB), i.e. 1 GbE line rate with almost nothing cached, even where the same worker ran the final's own library 9 minutes before (m08 `semio_hub_layout` 1.1 GB at 17:04, its final at 17:13: 42 s).
- Why nothing is cached: the blob cache (12 GiB bound, LRU, 10-min protection) lives in the worker's `remote-execution-worker` ledger area inside the dependencies image (budget 5.5 GiB, image ~27 GiB); m08 alone received 19.7 GiB of input closures from the play requester between 15:30 and 17:15 plus the fleet's own offloads, so the wasm32 rlibs of one final are evicted before the next one comes.
- Fixes landed (`RE👷️worker/🟦️.ts`): (a) an input the machine's own content store (`casRootV1`, the host CAS the pool puller and the worker's builds fill) already holds is copied from local disk with the same sha256 verification instead of fetched; a corrupt or vanished local copy falls back to the network; (b) each action result carries `type.semio.dev/input-fetch` {cachedFiles, cachedBytes, localFiles, localBytes, fetchedFiles, fetchedBytes}, so the split is measured per final from now on; (c) the cache bound = min(`maximumBlobBytes`, cache + work-volume free - room of every slot (agent disk rule), cache + host free - CAS7 worker reserve (4 % / 8 GiB)): the image is never filled to a `disk-full` refusal by the cache. Raising the 12 GiB ceiling needs `🔣️policy.json` (scheduler closure): left for after the play build.
- Expected fetch time: `uncached bytes / ~95 MB/s + ~4 s placement`. A final whose inputs are all cached or held locally: ~4-6 s (note: 4.6 s). With the local-store source, the uncached part is what neither the blob cache nor the host CAS holds; read the `input-fetch` entry of the next finals to know it (section 5).

## 4. Restart safety
The agents restart themselves (closure watch, 300 s quiet, max deferral 30 min) and drain: `createSidecarRestartV1` (`RE🛰️sidecar/🟦️.ts:63-72`) calls `drain()`, the agent's `drainWhenIdleV1` (`RE🛰️agent/🟦️.ts:23-31`) sets `draining` (every new admission refuses) and aborts only when `state.running === 0`; served remote actions are never aborted, the bound is `drainMaximumMilliseconds` 7,200,000 (2 h). A running final is therefore finished before its worker restarts. The scheduler closure was not touched.

## 5. After landing (play log)
(see the appended observation lines)

## Tests (`repo test-file`, one at a time)
unit-admission 12/0 (oracle: 600 admissions with held units at used/peak, 42 thread vectors), single-charge 8/0, stage-claim 8/0, why 11/0, refusal-reason 3/0, worker 17/0 (2 new: local-store source + fetch metadata, blob bound vs Python oracle 48 cases), worker retirement 4/0, codegen 3/0, remote-execution main 17/0, sidecar drain 6/0, offload main 41/0, disk 8/0 (2 of 4 runs 7/1 on the collect-before-refuse race), unit-cache main 27/1 (slot FIFO timing, untouched). tsc on the touched files: no new errors.

## Open
- Floor + x1.25 stacking and the 12 GiB ceiling: scheduler-closure edits, after the play build.
- Blob cache inside the 27 GiB dependencies image: moving it to the host volume (sized by the CAS7 reserve alone) is a ledger-area change (storage owners).
- Fetch affinity: the scheduler could prefer the worker that ran the final's library (its inputs are cached there); scheduler closure.
- The unaccounted 9-13 GB per worker (memory session note).
