# O18-patched (lane SY3): the requester side of O18 for the play snapshot

Snapshot `/Users/ueli/.sinc/play` (O17 applied). `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `R` = `L/🌐️remote-execution`, `OFF` = `R/🚀️offload`. Nothing was written to the snapshot. Apply: `T/wave-1122-dist/O18-patched/apply.sh <snapshot-root>` (reads `list.tsv`; stage 1 verifies every target against its pre-image sha256 and refuses on any mismatch before changing anything; a file that already has the post-image is skipped, so it is idempotent; stage 2 copies to `<name>.o18tmp` and renames, schemas and policy first, `offload/🟦️.ts` last). Order after O17; no driver or worker restart: every file is read per rustc call (the wrapper reseals per digest), nothing is in the native-build driver.

## Behaviour change
1. A linked unit (cdylib, dylib, staticlib: the wasm component finals and native links) asks for `semio-linker=<id>` in its action's platform properties (id = 16 hex of the SHA-256 over `rust-lld`, `wasm-component-ld` and `gcc-ld/*` of its own compiler's sysroot). The scheduler places the action only on a worker advertising the same id. Until a worker runs O18 code and restarts, none advertises it, so the final compiles locally. A linked unit whose own linker identity is unknown stays local (`not-portable`). The property is not part of the unit key.
2. For a linked unit the requester's execute budget, the unit deadline and the download time limit grow by `worker.outage.seconds` (600), and the output reads go through the outage-tolerant store (retry while the store answers code 14, pause 1 s doubling to 15 s). So it waits for the worker's retried result hand-over instead of compiling again. Cheap units keep their short bounds.
3. Policy: `worker.outage` {600 s, 1000 ms, 15000 ms} in `L/🔣️policy.json`, with its schema entry (the schema validator is closed, so the entry is required).

## Files (pre-image = snapshot now, post-image = shipped)
| file in this folder | kind | pre sha256 (16) | post sha256 (16) |
|---|---|---|---|
| `remote-execution-schema.json` | patched | `4464efff01c919d6` | `a057860fc2a6631e` |
| `policy.json` | patched | `9fb5cb438a006efe` | `8f588265d88165fc` |
| `offload-schema.json` | patched | `6ff244d092953fe6` | `ff16be1831728b7a` |
| `content.ts` | patched | `f6dfb6d2bb445d22` | `bd7d7768838324d4` |
| `toolchain.ts` | patched | `1966e615908b62c7` | `d61c59b61940dddf` |
| `action.ts` | patched | `46a01cbee70f5356` | `67e10680d08bcef1` |
| `offload.ts` | patched | `c7acf2b32a9c409f` | `cff4016a6d13d929` |
| `linker-test.ts` | new | `NEW` | `415c6d779b6f0ede` |
| `linker-oracle.py` | new | `NEW` | `2330d65290235bfa` |
| `content-retry-test.ts` | new | `NEW` | `723cc2fd2bad8c14` |

Targets: `remote-execution-schema.json` -> `L/🏃️execution/🧬️schema/🌐️remote-execution.json`; `policy.json` -> `L/🔣️policy.json`; `offload-schema.json` -> `OFF/🧬️schema/🔣️.json` (`property.linker`); `content.ts` -> `R/🗄️content/🟦️.ts` (`retryRemoteExecutionContentOutageV1`, `outageTolerantRemoteExecutionContentV1`; purely additive); `toolchain.ts` -> `OFF/🧰️toolchain/🟦️.ts` (`offloadLinkerIdentityV1`, `offloadSysrootV1`, worker offer); `action.ts` -> `OFF/📜️action/🟦️.ts` (`placement.linker`); `offload.ts` -> `OFF/🟦️.ts`; new tests `linker-test.ts` + `linker-oracle.py` -> `OFF/🧪️tests/🔗️linker/{🟦️.ts,🐍️.py}` and `content-retry-test.ts` -> `R/🧪️tests/🔁️content-retry/🟦️.ts`. Full sha256 of pre and post are in `list.tsv`. Longest target path 164 bytes.

## Where the snapshot differs from main for other reasons (NOT ported)
- `remote-execution.json` schema: main also has `chargeRampSeconds`, `codegen`, task `peakMegabytes`, heartbeat `usage`, `freeMegabytes` texts, `warden-cancelled` (O12/O16 scheduler and worker); only `worker.outage` was added. `policy.json`: main also has `blobSharePercent`, `graph`, `chargeRampSeconds`, `codegen`; only `worker.outage` was added. Neither is read by a requester.
- `offload.ts`: main prints `, N codegen threads` from the worker's metadata (needs `remoteExecutionThreadsOfMetadataV1` in the root module and `threads` in `OffloadOutputsV1`, i.e. `🔌️client/🟦️.ts` and `R/🟦️.ts`, which carry O16 worker-side changes). Cosmetic, left out; the 4-thread grant is worker-side anyway.
- `toolchain.ts`: main's docstring of `offloadSdkIdentityV1` differs (text only); the snapshot's was kept.
- `action.ts`: the snapshot's `content.missing/write(..., "input")` intent arguments (newer than main's file) were kept; only the platform change was ported.
- `🔌️client/🟦️.ts` and the root `R/🟦️.ts` are unchanged (not in the list). The snapshot's older tests of gate/offload/wasm-cdylib already fail before the patch (see below).

## Verify after applying (from the clone root)
No `repo test-file` exists in the snapshot CLI ("unknown command"), so the member tests ran with the snapshot bun on exact paths: `bun test ./<path>` (the leading `./` makes it a path; a bare path is a filter that scans the tree, which I ran once by mistake on a scratch clone: 77,979 files, no harm). Scratch: APFS clones (`cp -c`) of the snapshot `🧰️framework` in the session scratchpad with the snapshot's `node_modules` symlinked, apply.sh run on it (10 written; second run 10 skipped; wrong root refused), deleted afterwards.

| test file (under `R/` or `OFF/`) | result with the patch |
|---|---|
| `OFF/🧪️tests/🔗️linker/🟦️.ts` | 3 pass, 0 fail |
| `R/🧪️tests/🔁️content-retry/🟦️.ts` | 7 pass, 0 fail |
| `OFF/🧪️tests/🔗️native-link/🟦️.ts` | 10 pass, 0 fail |
| `OFF/🧪️tests/🩺️failure/🟦️.ts` | 2 pass, 0 fail |
| `OFF/🧪️tests/🧰️toolchain/🟦️.ts` | 4 pass, 0 fail |
| `OFF/🧪️tests/🟦️.ts`, `🚪️gate/🟦️.ts`, `🕸️wasm-cdylib/🟦️.ts` | same failures as on the pristine snapshot clone (4 fail / 0 pass / 0 pass): stale snapshot test files (`input.expensive is not a function`, a Source revision request) and not caused by the patch |

Import closure: `bun build --target=bun` of `OFF/🟦️.ts`, `OFF/🔌️client/🟦️.ts` and the wrapper entry `unit-cache/🏁️entry.ts` succeed. After applying to the real clone, `cmp` each target with its file here.

## Evidence-based answers (snapshot code, patch sets up to O17, `SEMIO_OFFLOAD` unset = mode `distribute`, `OFFLOAD_POLICY_V1.mode.default`)
### (a) Ordinary rlib units with a local grant "1 of 6 jobs"
- Cargo does get a fleet-sized job count, not 1 and not 6: `component-pass/🟦️.ts:519` sets `CARGO_BUILD_JOBS = cargoJobsV1(tokens, env)` = tokens + `SEMIO_UNIT_CACHE_LIGHT_SLOTS` = 1 + `fleetRemote` whenever `fleetRemote > 0` (`jobserver/🟦️.ts:269`). `fleetRemote` is the idle fleet slot count (cap `fleetJobsMaximum` 48). The 6 is `componentJobsMaximum` (`job-slots` schema), the number of concurrent component passes, which does not bound cargo's job count inside a pass. A fleet jobserver of tokens + fleetRemote is created (`:534`). With `fleet` unreachable (`fleetIdleSlotsV1` null) or mode `never`: `fleetRemote` 0 and cargo keeps the local grant.
- Placement is local first with overflow, per unit in `unit-cache/📜️script.ts:173` + `offloadGateV1` (`admission/🟦️.ts:190`): the wrapper's own slot count is `SEMIO_UNIT_CACHE_SLOTS` = 1, so one unit compiles here (`localSlot(0)` takes the free slot: reason `local-slot`). Every further rlib unit finds the slot taken and then: recorded local cost < the learned round trip overhead (8000 ms floor) = `cheap` -> stays and waits for the local slot (queued behind it, not offloaded); unmeasured -> waits up to 15 s for the slot, then goes on; cost >= overhead (or unmeasured after 15 s) -> offloaded when `door`, slot queue >= 1.5 s, host or broker would make it wait (`queue:true`), else `headroom` (`queue:false` = `if-idle`: the scheduler declines at once when no other machine has headroom, the unit then compiles here). Units sent away take a light slot (`acquireLightSlotV1`, bounded by `lightSlots` = fleetRemote). So yes, overflow reaches workers, but only for units that cost at least ~8 s; a build of many cheap rlibs is still serialised behind 1 local slot while cargo spawns up to 1 + fleetRemote rustc wrappers.
### (b) Component finals while no worker advertises `semio-linker`
- Before this patch set (O17 snapshot): a final is `expensive` (cdylib wasm; history >= 120 s / 4096 MB or the 900 s seed) -> gate asks only `awaitFleetFitV1` (the scheduler's `/v1/fleet/idle?peak=`, which sees no linker) -> if any worker has room at the placed peak: `final`, `queue:true`, the final goes to ANY worker with the same toolchain digest and target, including one whose linker cannot read the bitcode (g1-m06). If no worker has room within `finals.waitMilliseconds` (30 s): `final-wait`, compiles here.
- After this patch set: the gate is unchanged, so the idle probe still says yes; the requester uploads the inputs (blobs already in the store are skipped) and submits an action with `semio-linker`. The coordinator's live scheduler finds no compatible worker and answers `FAILED_BACK` ("No other machine can take the action", `scheduler/🟦️.ts:116`, `placementAnyCandidateV1`) at once; the client maps it to `no-idle-machine` -> `local(...)` -> the final compiles here immediately. Cost: one input upload attempt per final, no wait. As soon as one worker restarts with O18 code and advertises the same id, finals go there. (Scheduler behaviour is read from main; the scheduler is not part of the snapshot.)
### (c) A remote unit fails or its result is lost
- Compiler exit != 0 on the worker that is a real compiler answer (`classifyOffloadExitV1`): "compile-result", handed to the caller byte for byte and NEVER compiled again locally, so the build FAILS with that status (this is how g1-m06's `rust-lld: unknown file type`, exit 1, broke a final). Only exit 126/127/>128/negative (killed, infrastructure) and structured missing-input errors (E0460, E0463, E0464, E0583, E0786: `incomplete-inputs`) fall back to a local compile, at once (`offload/🟦️.ts:151-158`); a killed one also raises the learned peak.
- Result lost / worker or store gone: any exception while reading or writing the outputs (`offload/🟦️.ts:177`, the catch around the whole unit) returns `local("infrastructure")` and the unit compiles here at once, after the remote work already ran (no extra delay, but all that work is repeated). With this patch a linked unit instead retries the output reads while the store answers 14, up to 600 s. A transport loss during Execute (scheduler restart) is re-submitted for 20 s (`rideThroughMilliseconds`); the scheduler deduplicates by action digest.
- Silent or wedged action: the client waits until its deadline, then falls back local (`declined infrastructure "deadline"`): ordinary unit max(60 s, 4 x recorded local cost) capped 15 min, 240 s if unrecorded; linked unit the same factor capped 60 min, 40 min if unrecorded; the overall wait is action seconds (3600 for wasm cdylib) + 30 s margin + the queue wait. The server's keep-alive (5 s) lets a silent stream (20 s) re-attach every 10 s; an attach that stays silent ends as `infrastructure "silent after attach"`. With the patch a linked unit's deadline and budget are 600 s longer.
