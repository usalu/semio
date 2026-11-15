# Graph slice: memory-pressure cancel and slice cost (1112)

## PART 1 DONE

### Cause of the false "only 83/96 MB of free memory" cancels
- Without a cgroup (macOS, Windows) the pressure watch sampled `os.freemem()` (free pages only; on macOS inactive/file-cache pages are excluded, so a host that runs for days reads tens of MB while GBs are reclaimable).
  - `🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🌡️pressure/🟨️.mjs` (import `freemem`, default `free=freemem` of `cgroupMeasuresV1`).
  - Same bug in `…/slice-pool/🧱️limit/🟨️.mjs` `hostAvailableBytesV1` (non-Linux branch fed the load memory lending policy with `freemem()`).
- The capacity admission already used the right notion: `🏗️build/🧠️memory/🟨️.mjs` `createAvailableMemoryV1` / `availableMemoryBytesV1` (Linux `MemAvailable`, macOS `vm_stat` free + inactive pages, cached 1 s). Its log line "8014 MB available" comes from there.

### Fix
- Both slice-pool modules now use `availableMemoryBytesV1` from `🧠️memory/🟨️.mjs`; no `freemem` is left in the resident closure. Floor (256 MB) and window (30 s) untouched.
- Test added in `🏡️resident/🧪️tests/🛑️cancel/🟦️.ts`: a darwin `vm_stat` text with ~83 MB free pages and ~6 GB inactive pages never trips over 120 s through the real `cgroupMeasuresV1` -> `createPressureWatchV1` chain, and the two source files contain no `freemem`. The fixture "without a cgroup a short dip of free memory never trips" keeps passing.
- `bun test ./🟦️.ts` in `🛑️cancel`: 34 pass, 0 fail. All resident suites run: pass except `🏗️provision` (6 fail, "Caller-owned provisioning artifacts required") and `🔭️watch` (4 fail); neither touches the files changed (not caused by this change; not investigated further).

### Why a single owner "reserves" 8112 MB
- `🧠️memory/🟨️.mjs` `createMemoryGovernorV1`: an owner's cost (`owner.taken`) is the drop of the *host-wide* available memory while it runs (`sample`, delta of `headroom()` split between running owners), so any other process growing meanwhile (other agents' cargo/rustc, file cache) is billed to the owner. The maximum is learned per kind (`learned.set(kind, max)`) and persisted via `createGraphOwnerPeaksFileV1` (`…/np/owner-peaks.json` under the graph writer directory). One polluted `language:cargo` sample (8112 MB) then becomes the estimate of every cargo owner: each is admitted only when 8112 MB are free, so they run one at a time ("waiting for memory headroom (8112 of N MB)").
- Smallest correct follow-up (not done here): measure the owner's own process tree RSS instead of the host headroom delta (or discard samples where the host available memory fell by more than the owner's RSS).

### Disturbance of a running `repo run`
- Edited files are `.mjs` modules loaded by the graph service at start. The already running service keeps the old code and measurement until it restarts; the edit does not by itself change the running process. I did not verify whether the graph key (hash of the tool sources) includes these modules; if it does, the next run starts a new graph load instead of joining the old service. No run was active when the edit was made.

### Measured fact behind the 8112 MB
`.cache/b/g/m/inference/selections/8d8f21d7…/np/owner-peaks.json` currently holds `{"language:cargo": 8506310656, "project": 3490037760, "language:rust": 594264064, "language:typescript": 37191680}` bytes: a cargo owner is "known" to cost 8.5 GB (8112 MiB shown after the 1.0x basis), a project owner 3.3 GB. Real owners do not take that much; the figure is the host-wide drop of available memory seen while the owner ran (other agents' builds included). Because it is the persisted maximum it never decays, so every later run serialises cargo owners ("one at a time", `graphOwnerAdmissionV1` needs headroom - remaining >= 8.5 GB) and sits in "waiting for memory headroom". Deleting that file would only reset it until the next noisy sample.

## PART 2 (first investigation notes, superseded by "PART 2 DONE" below)

Findings (file:line are in `🏗️build`):
1. Per-owner persistence and reuse already exist. Each owner has a record `np/<ownerKey>.json` (`🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:84,257`, written by `👁️observation/🟨️.mjs` `observeInferenceComputationV1` ~l.902-960), content-addressed by `sha256([OBSERVATION_REVISION, null, key])` with `key = [projectAuthority, registryDigest, commandRevision, assemblyAuthority, root, json, selectedTargets, selectedInputs]` and validated against the real observed input facts (`current(header.observations)`). A record is written only when its owner finishes, so a cancel at 568/683 keeps 568 records; nothing is kept for the owner that was running.
2. Single-target slicing also exists: `options.ownerRoots` (`project-nodes/🟨️.mjs:166,246`) restricts the jobs; `🕸️graph/🧭️discovery/🟦️.ts:81` passes it only when `part.cold` is false. A run whose graph key has no previous parts (`Loading the project graph of 440cffa5563d`) is cold and reads all 683 owners. That is the by-design meaning of "slice" today; the cold path is where the 60 minutes go.
3. The np directory holds 3826 owner records but none was rewritten today (newest record 2026-10-30; only `owner-peaks.json` is new). So either every owner was a hit (then the time is spent validating huge observation sets) or optional publications were declined: `📥️inputs/💾️storage/🟨️.mjs:116-120` (`withBuildGraphOptionalPublicationsV1`) silently returns false when the image has less than `minimumAvailableBytes` free after the record (`.cache/b/g/m` has 1.9 GiB free of 5.8) or the optional quota/ledger refuses; the racy-input rule (`observation/🟨️.mjs` `racy`: any observed file with mtime/ctime within `RACY_MS` of the owner's start) also skips the write. The `workspace` (language:cargo) and `framework-os-*` owners observe thousands of source files that other agents edit constantly, so those records are skipped or invalidated on nearly every run, which matches the 370-400 s owners. I could not tell hit-validation from recompute without instrumenting the run, which I did not do while a run was live.
4. What invalidates the key: `head` (contracts digest, `registryDigest` of catalog providers, command inputs, assembly authority) plus the module-load authority (`inferenceAuthorityDigestV1`, every tool module the process loaded outside the stamped fragments) -> `inferenceRecordIdentityV1`. Editing a tool module (for example this fix) changes the identity of fresh contexts (`inference/contexts/<id>/np`, 24 of them exist); source edits by peers invalidate only the owners that observed them.

### Smallest correct change (not applied)
- Measure the owner cost from the owner's own process tree (RSS of the slice worker) instead of the host-wide available-memory delta in `createMemoryGovernorV1.sample` (`🧠️memory/🟨️.mjs` ~l.143-148), and drop the polluted persisted peaks (ignore `owner-peaks.json` entries not written by the new format). That removes the serialisation and the 8112 MB reservation.
- Make a declined/racy optional publication visible (`[graph] record for <owner> not kept: <reason>`) so item 3 can be told apart; then persist the heavy workspace-level owners per input group instead of one record over all files.

## Verification
- `cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🛑️cancel && bun test ./🟦️.ts` -> 34 pass.
- Live: restart the graph service, then a run on this macOS host must not print "The plan was cancelled: the host has had only N MB of free memory" while `vm_stat` shows GBs of free+inactive pages.

## Files changed
- `🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🌡️pressure/🟨️.mjs`
- `🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs`
- `🏗️build/🏡️resident/🧪️tests/🛑️cancel/🟦️.ts`
(all under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`)

## PART 2 DONE: owner cost measurement and record reuse

All paths below are under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`.

### A. Owner memory peaks
- Cause: `🧠️memory/🟨️.mjs` `createMemoryGovernorV1.sample` billed the drop of the host-wide headroom (`last-free`) to the running owners, so peer builds were charged to them; `learned.set(kind,max)` kept the maximum forever (8.5 GB `language:cargo`, 3.5 GB `project`).
- Change: the governor now takes a `used` port (own consumption) and bills its growth; `createGraphParentMemoryGovernorV1` always supplies `createProcessTreeMemoryV1()` = resident bytes of this process plus all descendants (slice worker, farm workers, `cargo metadata`; `ps -A -o pid=,ppid=,rss=` on POSIX, own RSS on Windows, 1 s cache). Admission still uses the cgroup/host headroom; the policy numbers are untouched. New exports `psProcessRowsV1`, `processTreeBytesV1`, `createProcessTreeMemoryV1` (+ `.d.mts`). Default `used` for bare ports (tests) is the headroom drop, the old law.
- Poisoned values: deleted by hand `.cache/b/g/m/inference/selections/8d8f21d7…/np/owner-peaks.json` (no migration code). The running service (old code) learns again at the first owner of each kind and may rewrite a polluted file; after the restart delete it once more if it reappears.
- Tests: `🧠️memory/🧪️tests/🟦️.ts` +2 (host noise of 6000 MB is not billed, learned 100 MB; process tree sum/ports/Windows/failure fallback): 20 pass, 0 fail.

### B. Why no owner record is reused
- Cause (measured): the optional graph quota is full and never reclaimed. `.cache/b/g/m/optional.sqlite` (mtime 2026-10-30 07:49) holds `used_bytes=1073741770` of `maximum_bytes=1073741824`; inventory of the optional leaves is the same 1073741770 bytes; no `np/ip/rr/mr` file was written since 2026-10-30 (only `ce/` closure entries). Every publication is declined at `📥️inputs/💾️storage/🟨️.mjs` `withBuildGraphOptionalPublicationsV1` (`scope.optionalQuota.reserve(plan)` fails, line ~120 declines; `ledger.publish` intent `usedBytes+peakBytes>maximumBytes` in `📒️ledger/🟨️.mjs`). Consequences: no record survives a cancelled or finished run, and the farm treats the declined adoption as fatal (`🏊️project-farm/🟨️.mjs` `adoption.declined` -> `stop("degraded")`, all pools closed), so the 683 owners run in one process one after another.
- Why nothing evicted: `withGraphFactWriterV1` ran the least-recently-written eviction (`evictOptionalGraphLruV1`, 10 min protection, lowWater 512 MB) only `if(optional&&!current&&…)`; the host's graph loads run inside `withGenerationCustodyV1` / prefetch / `withBuildGraphStateV1` custody, which pass `optionalCaches:false`, and the node inference nests in that writer (`current` set), so no writer ever reclaimed.
- Change: `📥️inputs/💾️storage/🟨️.mjs` line ~188: eviction now runs for every outermost writer (`!current&&usedBytes>lowWater`), `optional` still gates only the headroom maintenance. Dry run on the real store: 6915 of 7343 np entries (535 MB) would be retired to reach the low-water mark, no group protected.
- Test: `📥️inputs/💾️storage/🧪️tests/🚪️admission/🟦️.ts` (was broken by `quota.policy`, now `quota.definitions.DefaultPolicy.const`; extended with a second run per row using `{optionalCaches:false,shared:true}` that must evict exactly once when above low water and never run maintenance): 1 pass.
- Per-owner persistence already exists (record written when the owner finishes, keyed by real input facts: `observeInferenceComputationV1`), and `fileFactHolds` falls back to the content hash, so replica mtime churn does not invalidate. Records are invalidated by their authority rows: every tool module loaded by the process is an observation of each record, so an edit of any tool module (including this change) makes all records stale once; after that they hit again.
- Cold single selection: not changed. `🕸️graph/🧭️discovery/🟦️.ts:81` passes `ownerRoots` only when `!part.cold` by design (the doc says "all of them when cold"): `ownerRoots` is the dirty set of a warm incremental refresh; a cold key has no edges to know which owners a selection needs, so it reads all owners. With records written again the cold read turns into hits.

### Test evidence and pre-existing failures
- `🧠️memory` 20 pass / 0 fail; `📥️inputs/💾️storage/🧪️tests/🚪️admission` 1 pass.
- Same suites with my one-line storage change reverted: `💾️storage/🧪️tests/🟦️.ts` 9 pass 12 fail, `🤝️shared-writer` 0/4, `🎟️custody` 0/1 identical with and without the change (pre-existing). `📦️project-nodes/🧪️tests/🚦️governed-dispatch` 1 fail (the creation call contains `report:line=>process.stderr…`, not touched here, pre-existing). I did not rerun `🏗️provision` (6) and `🔭️watch` (4); they touch none of the files changed.

### Restart
- Both changes are `.mjs` modules loaded at service start: the running graph service and act-155 keep the old code (old cost law, full quota). `repo service host stop` and a new run pick them up; the first writer open after that evicts about 535 MB of the oldest np records, then records are written per owner and the farm is no longer disabled by declined adoptions.
- Live timing before/after: not measured, act-155 still running (no rc file). Before: owners 0 of 683 after 477 s, `workspace` owner 301 s and 222k rows, one at a time.

### Files changed
- `🧠️memory/🟨️.mjs`, `🧠️memory/🟨️.d.mts`, `🧠️memory/🧪️tests/🟦️.ts`
- `📥️inputs/💾️storage/🟨️.mjs`, `📥️inputs/💾️storage/🧪️tests/🚪️admission/🟦️.ts`
- deleted data: `.cache/b/g/m/inference/selections/8d8f21d7…/np/owner-peaks.json`
