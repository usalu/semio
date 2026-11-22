# 🕸️ Census: why every worker computes its own graph slice

Read-only audit. No source, git state, store file, process or ticket state was changed. No generated folder was touched.

Abbreviations: `BUILD` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `GRAPH` = `BUILD/🕸️graph`, `OBS` = `BUILD/👁️observation`, `STORE` = `/Users/Shared/semio-store/v1/origins`. Line numbers are as read on 2026-11-22.

## Short answer

The fleet already has a remote tier for per-project graph records (owner records in the fleet cache, with a pool that publishes and a take that fetches). The whole-workspace generation tier is the one that cannot be used today, because no complete head exists (the coordinator's 14,336 MiB whole rebuild is queued and its generation is quarantined, `audit-graph-producer-2031.md`). A worker therefore has nothing complete to adopt and falls back to a slice. Within the slice, the "taken X of 600" counter already counts owners fetched from the store or fleet; the cost is the owners nobody has published, which every worker infers on its own, because the only single-flight claim I found is inside the coordinator's owner pool. Nothing in the worker path claims an owner before inferring it locally (not fully verified, see section 5).

## 1. What a worker computes, and the cache key of each result

Per-project inference, not Nx `createNodes`. The unit is an owner (one project root) inferred by `inferProjectBodyV1` inside a farm child.

- Owner identity: `ownerRecordKeyV1(root)` = first 24 hex of `sha256(root)` (`GRAPH/📥️inference/📦️project-nodes/📋️present/🟨️.mjs:8`). The record file is `<workspaceData>/np/<key>.json` (`project-nodes/🟨️.mjs:145`, `:148`). The owner `root` is workspace relative: owner discovery joins it to `workspaceRoot` only for reads (`GRAPH/📥️inference/🔎️owner-discovery/🟨️.mjs:286`). So the file name is machine independent. The file itself is machine local.
- Record key text: `projectRecordKeyV1(head, root, json, selectedTargets, selectedInputs)` = `[...head, root, json, selectedTargets, selectedInputs]` (`project-nodes/🟨️.mjs:215`). `head` = `[projectAuthority, registryDigest, commandRevision, assemblyAuthority, "closure-references/4"]` (`:569`). `projectAuthority` = sha256 of the taxonomy contracts (`:495`). `commandRevision` = sha256 of the command inputs (`:496`).
- Fleet key: `ownerRecordKeyTextV1(key)` = `JSON.stringify([stamp, null, key])` (`:218`). The record header digest is `sha256` of the same text (`OBS/🟨️.mjs:601`).
- Validity of a stored record (`OBS/🟨️.mjs:596-609`, `projectRowsHoldV1` `:563`): every observation row must still hold on the reader's tree. File rows hold on a stat match (size, mtime, ctime, inode) or, after a stat miss, on the content hash (`fileFactHolds`, `OBS/🟨️.mjs:1028-1035`; async variant `:1448-1458`). So a file with identical bytes on another machine still validates.

Machine independence, by part:
- Yes: the owner root, the record key digest, the contract and command digests, the content hash fallback.
- Absolute path inside the rows: file rows are recorded under `resolve(root, name)` (`OBS/🟨️.mjs:386`). Fleet transport neutralizes them (`GRAPH/🪪️graph-state/📦️project-records/🟦️.ts:85-88`, `:149`), so the fleet path is root-neutral. The raw local path (`exportProjectRecordV1`, `OBS/🟨️.mjs:1133`) is not.
- Relocation table: `noteFormerRootV1` (`OBS/🏚️relocation/🟨️.mjs:13`) is only called from tests (checked across the whole `🧰️framework`). No production code teaches a former root, so a raw record from another checkout cannot be rebased. This is a reason to keep records neutral, not the cause of the slice (the fleet path is neutral).
- Not verified: whether `json` (targets from `componentBuildTargets`, playground targets, `:288-298`) carries any machine path, and whether the implementation digest (`GRAPH/🕰️generations/🔌️load/🟨️.mjs:170-171`, built from all rows incl. environment stamps at `:579`) equals across the ten macOS workers. If it differs, whole-head adoption is refused (section 2).

## 2. Remote or fleet tier today, and why a worker still computes

Two tiers exist.

Per-project (owner) tier:
- Pool child publish and claim: `GRAPH/📥️inference/📦️project-nodes/🌐️owner-pool/🔬️inference/🟨️.mjs:22` (fleet port), `:35` (port opened on the fleet client), `:41-51` (claim keyed by `ownerRecordKeyTextV1(projectRecordKeyV1(...))`, granted/stored/refused, failure note `:48`). A stored record is never recomputed by a granted pool child.
- Take (fetch) in the worker: `GRAPH/📥️inference/📦️project-nodes/📥️take/🟨️.mjs:57` lanes (hot = pool-published, local = present, remote = fleet), `:220` `takePassV1`, `:245` `fleetPublishedNamesV1` (reads `pool-published-<wave>.txt` per origin in the store, max age 7200000 ms), `:260` `takePollV1`.
- Policy (`📥️take/🧬️schema/🔣️.json:60`): concurrency 12, poll 2000 ms growing 150% to 20000 ms, batch 48, pass bound 900000 ms, no-progress bound 300000 ms, strong machine = 24576 MB and 8 cores.
- The slice counter: `GRAPH/🧠️memory/📋️slice-report/🟨️.mjs:115-140`. The line `taken` counts owners "taken from the store or the fleet, or inferred" (`:115`). So "taken 285 of 600" on m01 (`STORE/g1-m01/ready.json`) is the fleet tier at work, not computation.
- Coordinator pool now: `STORE/g1-server/pool.json` reads `state idle`, `units 4900`, `published 0`, `stored 0`, `slots 5`, `reason starting`. Heavy owners therefore have no producer right now (see section 3).

Whole-generation tier:
- Head key: `generationExchangeKeyV1(identity, platform, origin, scope)` (`GRAPH/🕰️generations/🌐️remote-exchange/🟦️.ts:64`). Head score and ranking: `:80`, `:86-88`.
- Adoption test: `completeAheadHeadV1` (`:154-156`) requires `note.stale === 0`, `note.implementation === location.implementation`, matching identity, platform and scope, and `!keepsLocalV1` (`:145-146`: keeps local when the signatures match or `note.stateMs <= local.modifiedMs + 60000`, or when the head is other code).
- Whole-read gate (`GRAPH/🕰️generations/🤝️whole-gate/🟦️.ts`): `fetchable` = `state === "complete" && ahead` (`:34`). A complete head ends the gate at once (`:49`). When this machine already holds owners (`held > 0`) the gate answers `skipped` and the machine reads the difference itself (`:50`). A machine below 24000 MB total is `capable=false` and yields 300000 ms before it looks again (`:54-55`; threshold `remote-exchange/🧬️schema/🔣️.json` `servingTotalMegabytes = 24000`, used at `remote-exchange/🟦️.ts:579` and `whole-gate/🟦️.ts:117`).
- Loader: `GRAPH/🕰️generations/🔌️load/🟨️.mjs:576` `loadGenerationGraphV1`. `wholeCurrent` (whole adoption) only when `policy === "current"` and no roots (`:580`). The slice read from `GRAPH/📥️load/🟦️.ts:311-315` passes roots and `policy: "fresh"`, so it always goes through `refreshClosure` (`:645`), and only a builder restores a head (`:613`).

Why workers still compute a slice, in order of how much each one matters today:
1. No complete head exists. The 13 destination adoption receipts are all missing (`audit-graph-producer-2031.md`), and the coordinator's whole generation is quarantined (`audit-m11-scoped-plan-capacity.md`). `fetchable` is false, so nothing is adopted. This is the blocker the census calls "whole-graph generation never completes".
2. Even with a complete head, a machine that already holds any owner is `skipped` (`whole-gate:50`) and reads itself. A fresh machine adopts, but a head of other code is never adopted (`keepsLocalV1`, `:146`), and every owner would be stale.
3. The slice read never asks the whole gate (`load:580`). It refreshes the closure with `fresh`, so each owner is re-verified per row. The test `wave-delta` and `🔭️scope` (`remote-exchange/🧪️tests/🔭️scope/🟦️.ts:33`) assert `ownersReinferred === 0` after a restore, so the design intends that adoption suffices. The live code path for a slice does not reach that restore unless the builder restores a complete head (`load:613`), which needs point 1 to be true.
4. The coordinator does not publish owners. `pool.json` is idle with `published 0`, so owners that need more than a worker can hold are not produced (see section 3).

## 3. Why the slice is slow and memory heavy

Measured in the store and tickets:
- m01 `ready.json`: `peakMegabytes 9392` on a 16 GB machine, "taken (pass 2) 285 of 600" at 16887 s (`STORE/g1-m01/ready.json`). Other workers 5.1 to 10.7 GB (`wave-1121/census-workers-ready-blockers.md`, blocker 2).
- m11 slice sample: 210495 distinct source observations, 356 owner checks, 796 MiB tree footprint (`audit-m11-scoped-plan-capacity.md`). The `slice-memory.json` estimate: settled-read collection about 359 MB, scope map about 65 MB, row store about 41 MB.

What dominates, from the code:
- Observation rows kept per scope: every observed file row is retained in the parent scope (`OBS/🟨️.mjs:239-248`, `scopeRowsV1` `:254`). 210k rows of stat plus digest per read is the main resident structure.
- Look closures: "a look walks the script closure of its owner, about as much memory as an inference, so 24 uncut looks held 3 GB of live heap" (`📥️take/🟨️.mjs:36`). Concurrency is clamped only by the cut (`:38-40`).
- Shared-cache caps: `fileShaMemoV1` up to 200000 entries (`OBS/🟨️.mjs:404`), `settledReads` bounded by `settledReadBytes` (`:860-863`).
- Heavy owners: owners that need more than the machine can give are set aside (`project-nodes/🟨️.mjs:66-70`, `HEAVY_WAIT_V1`, `OVERRUN_ASIDE_V1`), so a 16 GB worker waits for a 32 GB producer that is idle or blocked. That is the slow tail (m01 at 285 of 600 for 4.7 h).
- Step-down on 16 GB (round 30): 4 projects at a time (`WORKERS-READY-WITH-LOW-RESOURCES/ROUND.md`), which finishes but slowly.
- The slice-report estimate overcounts shared structures (`slice-report/🟨️.mjs:147`: 263 GB for a 100 MB sidecar), so the report numbers are upper bounds, not the footprint.

## 4. Design: content-addressed per-project leaves in the fleet cache

Target behavior: a worker infers only owners nobody has. Every inferred owner is written back under its content key. Adoption of a complete generation is enough for Ready.

Keys (keep): owner record key text `ownerRecordKeyTextV1` (`project-nodes/🟨️.mjs:218`), built from the record key (`:215`) and the head (`:569`). Make the head machine independent by construction: contracts, command, registry digests and the json already are, and neutralize every path in the rows before hashing (`project-records/🟦️.ts:85-88`). Put the environment stamps of the implementation digest under the platform key, not the implementation digest (`load/🟨️.mjs:170-171`, `:579`).

Protocol per owner, read-through and write-through with single-flight:
1. Look up the fleet record (`take`, `take/🟨️.mjs:220`). Hit: adopt, verify rows (`OBS/🟨️.mjs:596`), done.
2. Miss: claim the key in the fleet (the pool's claim at `owner-pool/🔬️inference/🟨️.mjs:41-46`, lifted out of the pool so every worker uses it, same TTL and lease renewal as `whole-gate` `holdV1`). Granted: infer locally, publish the neutral record, release. Held: wait on the key with the poll of `takePollV1` (`take/🟨️.mjs:260`), then take. Stored: take. Refused or failed: infer only if no failure note exists (`owner-pool/🔬️inference:48`).
3. Heavy owners: claim them for the strong machines first (`take/🟨️.mjs:73`), instead of each 16 GB worker retrying locally (`project-nodes/🟨️.mjs:66-70`).

Ready from adoption alone:
- A complete generation whose owners all have fleet records must be adoptable with no slice. Change `completeAheadHeadV1` (`remote-exchange/🟦️.ts:154-156`) to accept a head whose `implementation` matches under the platform-keyed digest, and `keepsLocalV1` (`:145-146`) so a worker with an older head takes the newer one even when it holds owners (today `held > 0` skips, `whole-gate:50`).
- Make `wholeReadGateV1` (`whole-gate/🟦️.ts:49-50`, `:81`) return `fetch` for a complete head even when `held > 0`, and let the slice read (`load/🟦️.ts:311-315`) consult it first, so a restore plus per-owner row check replaces owner-by-owner inference.
- Re-verify rows on take, never re-infer (`OBS/🟨️.mjs:596-609`), which the existing `wave-delta` contract already requires.

Files to change (paths relative to `GRAPH` unless absolute):
- `📥️inference/📦️project-nodes/🟨️.mjs` (lines 215-230, 557-600): read-through, claim before local infer, write-through publish.
- `📥️inference/📦️project-nodes/📥️take/🟨️.mjs` (lines 57, 220, 260): one lane for "claim-or-take" shared with the pool.
- `📥️inference/📦️project-nodes/🌐️owner-pool/🔬️inference/🟨️.mjs` (lines 22-51): move the claim out of the pool child into the shared port.
- `🪪️graph-state/📦️project-records/🟦️.ts` (lines 85-88, 149): neutral record for every publisher, not only pool children.
- `🕰️generations/🌐️remote-exchange/🟦️.ts` (lines 145-156, 574-579): adoption rules and the serving threshold.
- `🕰️generations/🤝️whole-gate/🟦️.ts` (lines 34, 49-50, 54-55, 81): fetch with `held > 0`, and the small machine rule.
- `🕰️generations/🔌️load/🟨️.mjs` (lines 580-613): slice read tries restore first.
- `📥️load/🟦️.ts` (lines 311-315): slice path calls the restore before refresh.
- `🕸️graph/🕰️host/🟦️.ts:174`: implementation rows keyed by platform.
- `🧠️memory/📋️slice-report/🟨️.mjs`: report the true footprint, not the shared-bytes overcount (`:147`).

Tests to add (language-agnostic oracle, per CLAUDE.md): one scenario in the `wave-delta` style (`remote-exchange/🧪️tests/📡️wave-delta/🟦️.ts`): two workers, the second restores the complete head and infers zero owners (`ownersReinferred === 0`, `computed(...) === []`), plus a Python oracle in `🕰️generations/🧪️tests/🐍️.py` for the key text and the neutral digest.

## 5. Not verified

- Whether the implementation digest equals across the ten macOS workers (environment stamps). First check: compare `implementation` in the head notes of `STORE/g1-server` with a worker's.
- Whether `json` (targets) carries machine paths.
- Whether any worker-side claim exists outside the pool (I found none in `project-nodes` or `take`).
- No live run was made, and no receipt was re-read beyond the single files cited.
