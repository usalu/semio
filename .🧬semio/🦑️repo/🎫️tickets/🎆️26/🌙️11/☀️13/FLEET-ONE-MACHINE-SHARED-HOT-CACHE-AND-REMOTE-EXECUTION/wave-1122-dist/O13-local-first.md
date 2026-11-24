# O13: local-first placement, fleet-sized cargo jobs in every pass, round-trip attribution

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, `NB` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/`, `RE` = `L/🏗️build/🌐️remote-execution/`, `UC` = `L/⚡️caching/🦀️cargo/🗃️unit-cache/`.

## 1. Why cargo ran 4 jobs (root causes, file:line in main before the change)
- The measured pass `puzzle-wasm` is NOT a cargo pass of the component driver: it is a native-build **job** with `workerProtocol: "exclusive"` (`🏢️semio-tech/🎡️play/📋️project.json:513-526`, command `repo ./📜️script.ts wasm`). An exclusive job gets no jobserver, and the standalone wasm-pack runner `NB/🕸️wasm-build/🟦️.ts:170` set `CARGO_BUILD_JOBS = capacity.parallelism` (the local lease, at most 4). Its environment already carried `SEMIO_UNIT_CACHE_SLOTS=4` / `LIGHT_SLOTS=44` (from `component-pass/🟦️.ts:516`), so the wrapper offloaded, but cargo itself was capped at 4. This is where 4 won.
- Component cargo passes (`component-pass/🟦️.ts:908`): no `CARGO_BUILD_JOBS` of their own, the value `4` came in with `inputs.environment` of the driver process; the fleet jobserver (`tokens + fleetRemote`) was handed over but cargo's job count stayed the inherited 4.
- Jobs with `workerProtocol: "jobserver"` (`component-pass/🟦️.ts:699`) joined the LOCAL pool (`jobserver.withClient`), never `fleetServer`, and inherited `CARGO_BUILD_JOBS=4`.
- Plain cargo (`repo cargo`, `L/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts:237-244`): already `tokens = fleet.jobs`, `CARGO_BUILD_JOBS=tokens` (checked by reading; not the measured pass).
- `NB/🏗️native-build/🟦️.ts:1183` (scheduler-binary build) has no wrapper slots and stays local by design.

## 2. Why local slots idled (placement)
- `offloadGateV1` (`RE/🚀️offload/🚦️admission/🟦️.ts`) ended in `reason: headroom`: on a calm machine every unit worth a round trip was offered to the fleet whenever a worker had room, free local slots or not.
- The worth test was `cost >= 2500 ms or no record`, and the cost record was wrong: `UC/📜️script.ts` wrote `took = Date.now() - compiled` where `compiled` was taken BEFORE the gate and the wait for the slot, so a 3 s unit that queued 30 s for a slot was recorded as 33 s. Measured in the play unit cache: `semio_ui_text` cost 3125 ms, `semio_ui_wgpu_raster` 2574 ms, `semio_os_infinite_world_interaction` 2761 ms; all offloaded with 21 to 61 s round trips. Records of units that never saw a slot queue were fine, which hides the bug on small builds.

## 3. Where the 33 s go (read-only on `/Users/ueli/.sinc/play`)
`offload.jsonl` (9257 lines, 1382 `offload-run`), last 30 min (60 runs): median 32.2 s total = plan 0.35 s + upload 0.31 s + **queue and execute 31.3 s** + download 0.05 s; median 550 MB shipped, median 39 blobs actually uploaded (the `FindMissingBlobs` check in `shipOffloadActionV1` already skips present blobs; hashing is memoized by the stamp store, 0.35 s for 550 MB). Before this window the same field had median 5.0 s over 1322 runs, so the change is not the client side.
- Client side is 0.7 s of 32 s: nothing material to fix there.
- Scheduler: `build-scheduler.log` summary "queue wait p50/p90/max 0/1/19310 ms over 512 assignments": the median assignment waits 0 ms.
- Worker: `build-agent.log` on the coordinator's worker, 331 completed actions in the same 30 min: median 3.9 s, p90 14.6 s from take to finish (includes input materialization, rustc, output upload).
- So 17 to 27 s of the median are neither client nor queue nor this worker. The logs name three candidates, none of which the old event separated: scheduler retries after `Content store answered 503` / `ENOENT copyfile blobs/..` (the 550 MB inputs per action load the fleet cache), first-time blob fetch on workers other than the coordinator, and the jobserver token that a wrapper hands back during the round trip and must take back (`reacquire`, up to `tokenWaitMilliseconds` 30 s, inside the "queue and execute" mark). The data on hand cannot rank them; I do not claim a split.
- What I changed so the next run reads it off:
  1. `offload-run.detail` now ends with `worker fetch F ms, run R ms, upload U ms, away from it A ms of which token wait T ms`: F/R/U from the worker's `ExecutedActionMetadata` timestamps (new `remoteExecutionPhasesOfMetadataV1`, differences of one clock), A = round trip minus the worker's own time (queue + transport + retries + token), T measured around `reacquire`.
  2. The token round trip is gone where the pool is already fleet-sized: with `SEMIO_UNIT_CACHE_LIGHT_SLOTS > 0` the wrapper keeps its token (the pool counts the units that are away, the light slots bound them), so no unit waits behind units that started meanwhile (`RE/🚀️offload/🟦️.ts`, `tokenLeaseV1`).
  3. The overhead of a round trip is learned (below), so a fleet whose round trip is 30 s keeps every unit cheaper than that at home.

## 4. Changes
1. **Job count (every pass kind).** New `cargoJobsV1(local, env)` (`NB/🏗️native-build/🎰️jobserver/🟦️.ts`) = local grant + `SEMIO_UNIT_CACHE_LIGHT_SLOTS`. `wasm-build` uses it for `CARGO_BUILD_JOBS`; `component-pass` sets `CARGO_BUILD_JOBS` for component passes and `jobserver` jobs when `fleetRemote > 0`, and jobserver jobs now join the fleet jobserver. The wrapper's slots (`SEMIO_UNIT_CACHE_SLOTS`) still bound local rustc, the light slots bound remote, so cargo's 48 only means 4 local + 44 away.
2. **Local first.** New `tryAcquireSlotV1` (`UC/🔒️lock/🟦️.ts`): atomic, non-blocking, never overtakes the oldest waiter's ticket. Gate order (`offloadGateV1`, vectors v3): `never` / `pin` unchanged; **free local slot taken at once → compile here in that slot (`local-slot`)**; else worth test: cost below the learned overhead → stays (`cheap`); no cost record → waits up to `unmeasuredWaitMilliseconds` (15 s) for a slot here so it is measured (`unmeasured`), then goes; then door / slot-queue / host / broker as before. A unit is only offered when it would wait. With `SEMIO_UNIT_CACHE_SLOTS=0` (no slot limit) there is no slot concept and the old behaviour stays (the cluster tests of O1 run this way).
3. **Worth test.** `worthARoundTripV1` = `worth | cheap | unmeasured`: recorded local compile time (now only the compiler's run, `ranMs` in `script.ts`) against the expected overhead = running average of measured overheads (round trip minus the worker's run time, file `offload-overhead` in the unit cache root), floor `roundTripMilliseconds` raised 2500 → 8000 (the 1382-run median over the whole log is 6.2 s).
4. **Reasons.** `local-slot`, `unmeasured` added (schema enum, EN + DE texts, type); counters only, no event lines.

## 5. Tests (`repo test-file`, one at a time, no taskpolicy)
- Gate vectors v3 + independent Python model over all 3·2·2·2·3·3·2·3 = 1296 signal combinations (`🚪️gate`, new `🐍️.py`): 4/0 (vectors, worth vectors, overhead record, model).
- Cluster with stub workers wA/wB: 13/0. New test "local first": slots 1, light 4; unit c holds the only slot with an 8 s compile, unit d (cost 60 s) goes to a worker and finishes while c still runs (local and remote in flight at once), cheap unit e (cost 10 ms) waits for the slot and compiles here; asserts only d is an `offload-run`, the event detail carries the worker phases and the overhead is learned. The stale `.semio-key` file listing of the first test was corrected (it failed before this lane, another lane's stamp files).
- Driver (TI2 now allows nested `repo cargo`): new test "fleet-sized cargo jobs": a component pass AND a `jobserver` job (wasm-pack style) get `tokens + 5` as `CARGO_BUILD_JOBS` and the fleet jobserver path (`/fleet/`), without a fleet nothing changes. Passed 1/0 in an isolated copy of the file (removed again); the whole file still carries its known failures from TI2 (46/24 then), not re-run as a whole.
- `tryAcquireSlotV1` (free slot at once, no sharing, never overtakes a waiter): light-slot file 4/0. `cargoJobsV1` vector table: jobserver file 10/0 (1 skip, as before).
- Neighbours: offload 42/0, unit-cache 28/0, buildscript offload 10/0, slot-held 3/0, wasm-build 9/0, native-link cluster 1/0, unit-admission 9/0, refusal-reason 3/0, failure 2/0.
- Not run live (play release in flight, read-only rule); `repo cargo` plain launcher unchanged.

## 6. Patch set for the play snapshot: `O13-patched/` (key neutral)
`<name>.patch` = unified diff against the snapshot file as it is now (`patch <target> < <name>.patch`, run from any directory with the target path given); each applies to a copy of the snapshot file and the result is byte-identical to the shipped merge (`cmp` checked for all 11). `script` and `offload` differ from main in their import lines, so those two diffs are merged by hand against the snapshot (they do not apply from main's diff). Import closure on an APFS clone of the snapshot `🧰️framework` with all 11 applied: `bun build --target=bun` 0 errors for the wrapper entry `UC/🏁️entry.ts`, `UC/📜️script.ts`, `offload/🟦️.ts`, `component-pass`, `wasm-build`, `driver-stages` (with the snapshot's `node_modules` linked), the repo CLI script; gate vectors, `worthARoundTripV1` and `tryAcquireSlotV1` executed from the clone. Nothing under `🔏️key`, `🪪️extern`, `buildscript`, unit-cache schema: unit keys untouched.

| patch | target (under the snapshot root) | read |
|---|---|---|
| `lock.patch` | `UC/🔒️lock/🟦️.ts` | per rustc call (wrapper, sealed copy per digest) |
| `script.patch` | `UC/📜️script.ts` | per rustc call |
| `admission.patch` | `RE/🚀️offload/🚦️admission/🟦️.ts` | per rustc call |
| `offload.patch` | `RE/🚀️offload/🟦️.ts` | per rustc call |
| `client.patch` | `RE/🚀️offload/🔌️client/🟦️.ts` | per rustc call |
| `remote-execution-root.patch` | `RE/🟦️.ts` | per rustc call |
| `offload-schema.patch` | `RE/🚀️offload/🧬️schema/🔣️.json` | per rustc call (imported by the policy module) |
| `offload-policy.patch` | `RE/🚀️offload/📐️policy/🟦️.ts` | per rustc call |
| `jobserver.patch` | `NB/🏗️native-build/🎰️jobserver/🟦️.ts` | driver start (`cargoJobsV1`), and per `repo ... wasm` process start |
| `wasm-build.patch` | `NB/🕸️wasm-build/🟦️.ts` | per `repo ./📜️script.ts wasm` process, i.e. the next `puzzle-wasm`-style job that starts |
| `component-pass.patch` | `NB/🏗️native-build/🧩️component-pass/🟦️.ts` | driver start (loaded once per driver process): needs a driver restart, applies on top of SY2's file (snapshot is `831b9fbb549`) |

Apply order is free (no new files); copy to a temporary name and `mv`, ideally between units. The overhead floor 8000 and `unmeasuredWaitMilliseconds` live in the schema JSON (read per rustc call).

## 7. Open items and risks
- The 17 to 27 s unexplained part of the median round trip is unattributed (section 3); the new detail field decides it on the next build. If `token wait` is small and `away from it` stays large the next suspects are the 503s of the fleet cache (550 MB per action) and the cold blob fetch of workers; shrinking the shipped closure (rmeta only for library units) is the lever, not done here (O1 notes the can't-find-crate risk).
- Cold start: with no cost records every unit is unmeasured and waits up to 15 s for a local slot before going; on a first build the fleet starts after 15 s. A warm `c/` record set (the play unit cache already holds 2426) avoids it.
- Learned overhead includes queue and retry time of a loaded fleet, so a congested fleet keeps more units at home, by design.
- A whole-file run of the driver test file was not repeated (known 46/24 failures from TI2); only the new test was run, isolated.
- Not touched: `NB/🏗️native-build/🟦️.ts:1183` (scheduler build) and the repo cargo launcher (already fleet-sized).
