# Graph slice slowness for `mit-bestand-demonstrator:activate-prototype`

Status: not solved yet. The cold plan does not complete in minutes; several real defects are fixed, one dominant cost is still open (see "Open").

## Defects found and fixed (each with evidence)
1. Farm capacity priced like a compile run (3584 MB for one worker) and never granted on a 16 GB host. Now priced per worker (1024 MB, `workerMegabytes`), so 2-4 workers are granted. (`host-capacity/🟦️.ts`, farm policy)
2. The farm only engaged after 24 misses, which never came at 5 s per owner. Now it engages at once when the refresh has a known stale set (`engage: ...||wantedRoots!==undefined`).
3. Farm refused on every start: "worker processes were not used because they differ ... (identity differs: records)". Cause: the parent digest of its module-load authority was memoised before later loads, and a worker that adopts the rows holds 13 directory-`lstat` rows fewer. Fix: the digest is computed from the shipped rows themselves in the parent (`inferenceAuthorityRowsV1`) and in the worker (`adoptInferenceAuthorityV1`) through one function. Verified: workers now answer `ready` with the parent's identity and are accepted.
4. Host memory watch cancelled the plan with "the host has had only 73 MB of free memory (floor 256 MB)" while the host had 5-9 GB available: the slice pool watch and `hostAvailableBytesV1` used `os.freemem()` (free pages only, a few MB on macOS). They now use `availableMemoryBytesV1` (free + inactive pages, `vm_stat`).
5. `heapStats()` was taken every 2 s in the slice worker; it walks the whole heap and cost 21 s of 173 s (12%), and 54 s of the 160 s owner phase at 5 GB heap (a third). The next sample now waits 20 times the cost of the last one.
6. `releaseOwnerStateV1` dropped the command-closure walks after every owner (1.3-1.8 s CPU per rust owner versus 0.25 s kept, measured in `profile-owner.mjs`); it now drops them every 32 owners, and the closure-entry shrink under memory pressure runs on the same interval instead of after every owner.
7. `readBuildGeneratedCompileOutputV1` validated every ignored output root of every generator contract (path normalisation and regex) on every import of every command closure: 30% of owner inference in a CPU profile. The roots are now validated and listed once per contracts object (`generatedOutputIndexV1`).
8. The slice worker inherited the resident service's 1 GiB JSC target; it now runs with the graph child target (`graphChildHeapBytesV1`, 512 MB). The standalone harness halves resident size for 40% more CPU at 256-512 MB (1629 MB resident at 150 owners with 1 GiB, 696-767 MB with 512/256 MB).
9. The captured implementation snapshot of the repo (about 25.4 MB) exceeded the 24 MiB limit of the auxiliary stdin channel while the snapshot limit itself is 32 MiB, so every `repo` command failed with "Bounded canonical UTF-8 auxiliary DATA required" after the repo grew. The auxiliary limit is now 32 MiB, as the snapshot limit.
10. The lease ETA ("158 of 1885 - about 1 min left") restarted its phase clock whenever another progress line interleaved. The start of a counter series is now kept per (phase, total) and reset only when the counter goes backwards.

## Measurements
- Standalone single-owner inference (harness `profile-owner.mjs`, no service): 0.2-0.5 s per rust owner when the shared walks are kept, 1.3-1.8 s when released per owner, 4-9 s once for the first owner.
- Service, cold refresh of 1885 owners, before the fixes: 1 owner per 5-15 s, slice worker 4.6-9 GB resident, plan cancelled by the false memory watch.
- Service after the fixes: farm granted and accepted (3-4 workers, 1024 MB each); owner phase starts at about 75 s; 1.8-5 s per owner in the parent (about 0.3-0.5 owner/s), slice worker 5.8-7 GB resident.
- The slice worker keeps 5-9 GB resident for a live set of about 0.5 GB after a collection (JS heap 1-2.5 GB plus 0.7-1.9 GB external; `vmmap` shows the rest in the allocator "App-Specific Tag 1" region). Its growth happens in the fact and verification phases (3.4 GB at the start of the owner phase) and keeps climbing during owners.

## Open (what still makes a cold plan take hours)
- The owners are reused, not re-inferred: no project record is written during the owner phase (np directory unchanged), so the farm never receives a job (`prepare` is not called). The 2-5 s per owner is spent validating and resolving the stored records and closure entries in the parent: the owner-phase profile is `JSON.parse` of stored closure entries and layers (`script-inputs/💾️entries` `load`, `layerBatchesSync` scanning bytes in JavaScript, zlib), 2.8 GB of layers in 40609 files and 1.1 GB of entries in the closure store. A larger in-process entry budget (512 MiB) did not change the rate, so it is not simple cache thrash.
- Next step with the best expected gain: profile only the owner phase (the profile of this session mixes load and owners) and make the per-owner resolution reference the shared layers by digest without re-reading them; and bound the slice worker memory (fact index and rows) so that the gate can grant more workers.
- Every edit to graph or build implementation code invalidates all owner records and generations again; batch edits.

## Changed files (all under `🧰️framework/`)
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts`
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🏊️farm/🟦️.ts`
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs`
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs`
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🧬️schema/🔣️.json`
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs`
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🎯️target-fanout/🟦️.ts`
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs`
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts`
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🌡️pressure/🟨️.mjs`
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs`
- `🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts`
- `🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🟦️.ts`
- `🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧬️schema/🔣️.json`
- `🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧪️tests/🔣️.json`

## Verification
`bun test` directly: farm capacity (5 pass), standing leases (26 pass), target-fanout (2 pass), cancel (36 pass). The project-farm and auxiliary suites need the repo test harness and could not run directly. Scripts of this session: `profile-owner.mjs`, `identity-check.mjs`, `identity-check2.mjs`, `farm-check.mjs`, `farm-fork.mjs` (farm handshake and identity checks).

---

# Profile of the owner phase and fixes (second session)

## Method
`sample` only shows unsymbolised bun frames. The JavaScript profile came from `bun:jsc` `startSamplingProfiler()` / `samplingProfilerStackTraces()` started temporarily inside the slice worker (and a farm worker) and dumped every 90 s (removed again). Counters (`[DEBUG]`, removed) around `observeInferenceComputationV1` lookup, `closure.resolve`, `store.load` and the layer verdict.

## What the 2-5 s per "reused" owner really was
1. The owner records are not the problem (4 KB, lookup `read` 17 ms of 1275 lookups). The time was in `settleClosureRowsV1` (76 % of the observe time): bringing the closure entries a record names up to date. Per owner about 18 own entries (37 000 entries for 1893 owners, nothing shared), each loaded by `store.load` (45 % of all parent samples, of it `JSON.parse` 28 %).
2. Cause of the parse cost: every stored entry carries a `sidecar` with, per shared layer, the list of all walk questions the layer belongs to, as full JSON strings of about 700 characters. That relation is layers x walks: 3.5 MB per entry on average, 136 MB for the largest (entries above the 256 MiB unpack limit could never be loaded and were re-derived by every plan). 37 000 entries x 3.7 MB = 130 GB of JSON, 1 GB brotli.
3. Layers were read by a byte-by-byte JavaScript scanner (`layerBatchesSync`, 15 % self) to cut batches; the native `JSON.parse` of the whole layer is several times cheaper.
4. `load` probed every layer of the entry with a `stat` (about 800 per entry, 7 % of the main thread in `sample`); a lost layer is already found when the layer is read by the verification that follows.
5. `heapStats` 5 %, synchronous `vm_stat` once per second 8 % of the parent (memory governor).
6. Memory: the parent keeps the resolved nodes with literal file lists of every command closure (late closures) and the row log; the 6-9 GB are that, not a leak. Not changed.

## Why a completed plan did not make the next one fast, and why every edit restarts everything
The `analysis` rows (provider `build:command-references:v1`) in every closure entry carry the provider revision, which is the digest of all source files of the inference component (`inference/🟨️.mjs` `sourceFiles`, every roster component). Editing ANY inference component file (also an unrelated comment or a debug counter) changes the revision, every stored entry goes stale (36 608 stale entries seen by the probe `undefined.stale` of the parallel session) and the next plan derives all 37 000 entries again. All my instrumented restarts therefore paid the derivation again; the cold run of 3862 s (owners 1893 of 1893 reached) was such a run plus farm inference of the owner records. Peer machines editing these files do the same.
The farm is used only for the owner records (np); once a record exists (the stamp, not the file bytes, keys it) the owner is reused and the farm idles.
No generation had been published by any plan so far because none ran to the end without being cancelled or restarted; whether the second plan skips the owner phase is measured below.

## Farm churn (found while the records were missing)
`recycleReasonV1` compared the raw resident set of a worker (garbage included, JSC does not return pages: a forced `Bun.gc(true)` leaves rss unchanged, measured) with 60 % of its budget and 125 % of the baseline taken after its first job: workers were replaced after two jobs (log: `worker 1 restarted after 837 MB`, one restart every 8 s), each replacement paid the boot and re-walked the shared closures. Policy now: `recycleMemoryRatio` 0.75, `recycleGrowthRatio` 1 (fixture rows moved accordingly).

## Changes of this session (on top of the list above)
- `📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs`: sidecar stored as a walk table plus indices (record version 3, loaded back to the same strings), whole-layer native parse instead of the JS scanner, no `stat` probes in `load`; the earlier stored entries were converted once by `convert-closure-store.mjs` (this folder; version 1 and unloadable entries deleted).
- `📥️inference/🧪️tests/🌊️layer-stream/🟦️.ts`: test for the compact sidecar (record below 1 MiB for 200 layers x 200 walks, round trip equal, version 2 record is a miss).
- `🏡️resident/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts`: heap statistics sample share 20 -> 200.
- `🧠️memory/🟨️.mjs`: available memory answer kept 5 s instead of 1 s.
- `📦️project-nodes/🏊️project-farm/🏊️pool/🧬️schema/🔣️.json` and `🧪️tests/🔣️.json`: recycle policy and fixture rows.

## Measurements with the final code (no edit between the two plans)
- Plan 1 (`🗑️generated/dry-fixed-8.log`; the closure entries were stale because of the last edit, 213 of 1893 owner records missing): graph slice 2112 s, of which about 1560 s for the single owner `@semio-tech/framework-os-dev` (project with the playground preparation targets of all variants, 3 650 rows, computed in the parent, not by the farm); the other 212 owners took about 550 s. The plan then stopped in the unrelated nx stage with `Internal error: Original SDK binary link group required` (after "Validating current owner custody"), not in the graph.
- Plan 2 (`🗑️generated/dry-fixed-9.log`, same code): the slice was served from the published generation without an owner phase: doubt 8 s, facts 10 s, assembly 12-24 s, then `tasks: 2 stale, 0 up to date`; hot slice about 25 s. The run then waited for host capacity ("need 4880 MB; 3.7 GB spare above the 2 GB floor"), the next stage, not the graph.
- A full cold slice (no generation, no records, no entries) was not re-measured; the 3862 s run before the changes (owners 1893 of 1893 reached, all closure entries stale) is the reference.

## Still open
1. `@semio-tech/framework-os-dev` takes 26 min in one process (single owner, thousands of targets; its rows grow for 25 min). Needs its own profile (`playgroundPreparationTargets` and `projectWithDefaults` for the 9 `prepare-<variant>-react-prototype` targets); split per variant or let the farm take it.
2. Any edit of a file of the inference component set restarts the derivation of all 37 000 closure entries (analysis rows carry the digest of the whole component). Peer machines and sessions edit these files all day. The provider revision should digest only the analyzer source (`🧬️source-analyses`) and its parser dependencies, not the roster of inference components.
3. Parent memory 5-9 GB (resolved late closures with literal file lists plus the row log); farm workers still end at 800-900 MB (policy 0.75 of 1024 MB); next lever is a retained-heap measure after a collection instead of the resident set.
4. The `bun:jsc` sampling profiler recipe is worth a permanent `repo` diagnostic.
5. Not investigated: the nx-stage failure "Original SDK binary link group required" and the transient `Relative filesystem fstatat failed` of the stale-project preparation under concurrent peer writes.
6. The `🗑️generated` folder of this ticket (logs, samples, profiles) still exists; `rm` of it was blocked by the safety check, to be removed by the dev.

---

# Third session: the two open items re-examined (read-only, nothing under `🧰️framework` edited)

Harness: `prof-osdev.mjs` (this folder; bun, temporary closure store and staging, no `.cache` writes), `entry-rows.mjs`. Scratch scans in the session scratchpad.

## A. "Entry digest covers the whole inference component": not what the code does
- Analysis rows carry `provider.id` and `provider.revision` from `🧬️source-analyses/🟨️.mjs:176-191`: the revision is `sha256(stamp + typescript bytes)` (`compilerRevisionV1`), the id `sha256([module url, stamp])` with the stamp from `🔖️stamps` (`semio.graph-inferrer.source-analyses/3`). Reproduced from the store: ids `...c4e967f0`, revision `ec6919ef` and `script-provider-ports` revision `01b08715` (sha of `scriptProviderPortsV1.toString()`) all equal the values stored in 22 909 entries / 49 689 layers today. Editing a component file does not change any of them. The revision does not include `sourceFiles` (that digest, `♻️revision/🟨️.mjs:41`, feeds generations and the invocation, not entries).
- What does stale entries: the analysis row of a file in the walk, when its parse result (`{runtime:{imports,requires}, urls}`) changed. Of 4 731 distinct analysed files in a layer sample, 4 715 reproduce their stored `resultSha256` today; the 16 that do not are files whose import lists were edited (script-inputs, entries, target-policy, generations, host-capacity ...). The peer probe `undefined.stale` (49 782 stale events) lists only analysis rows of such files (project-farm worker, project-nodes, cargo script, runtime main). Because the build tree is in the closure of every `repo ...` command, one changed import list stales every entry that reached the file.
- So a comment or body edit does not stale entries; adding/removing an import specifier (builtins included, because `command-references` hashes the full specifier list) does. A narrowing (drop `node:`/`bun:` builtins from the `references` result, then bump the stamp) would avoid the builtin-only case; it changes the analyzer and invalidates the store once, so it was not written.

## B. `@semio-tech/framework-os-dev`: not slow in isolation
Same stages as `emojiProjectJsonNodes` for this owner (catalog, `playgroundPreparationTargets`, `inferProjectBodyV1` in late-closure mode, `resolveClosureRefsV1`), cold, one process: playgroundPreparationTargets 1.8 s (4 113 targets, nativePreparation 0.8 s of it), inferProjectBody 9-19 s, resolve 10-17 s, total 21 s warm / 37 s cold with a writable store, row log and closure pass; 8 401 entries (5 816 derived, 5 314 layers, 25 of them 11 MB walk layers). Walks are shared (one layer per walk question), the nine `prepare-*-react-prototype` lanes do not re-walk. Without a row log `resolve` re-read the 25 big layers per entry (`supportOf` at `🧭️script-inputs/🟨️.mjs:1321`): 201 s; the service passes a log, so that is not the 26 min.
The 1 560 s therefore comes from the service context, not from the owner: not reproduced here (observation record capture `observeProjectInferenceV1` needs the physical-ancestry setup and failed in the harness; farm, concurrency with other owners, 5-9 GB heap GC pressure and progress publishing were not exercised). Next step: sampling-profile the parent while that owner runs (`startSamplingProfiler` recipe above).

## Deliverable
`🩹patches/` holds one small diff and a README; nothing else is justified by the measurements.
