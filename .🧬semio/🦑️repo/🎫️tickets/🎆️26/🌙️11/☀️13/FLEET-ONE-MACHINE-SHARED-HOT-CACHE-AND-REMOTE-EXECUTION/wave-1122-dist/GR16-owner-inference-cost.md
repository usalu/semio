# GR16: what one owner inference costs, why, and what changed (2026-11-24, 08:39 to 10:00 Mac clock)

`I` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/`, `SI` = `I🧭️script-inputs/`, `PN` = `I📦️project-nodes/`, `T` = the ticket folder. Lock `lock.graph-closure` (released). Probes and scripts: `T/wave-1122-dist/GR16/` (`preload.mjs` counts fs calls, `one-owner-v4.mjs` runs ONE owner through the real service in a fresh process with and without a shared closure store, `run-ab2.sh`, `summarize-ab2.py`); their outputs lived in the session scratchpad. Nothing restarted, no read killed, no source log added (so no `[DEBUG]` line to remove). Landing: `T/landings.md` "Wave 1122-dist GR16".

## Verdict in six lines
1. One owner costs 6 to 35 CPU seconds and 0.4 to 3 GB in a fresh pool child, not under a second: 20.9 s CPU median (279 s for 6 owners), 180.8 s for the heavy `cad-js`; wall time 2 to 5 times that under the host load of 40 to 70 on 10 cores. The recorded owner-costs table of the coordinator says the same: 2,860 owners, median 113 s, mean 450 s wall, 357 h in all.
2. The inference itself is not the cost. In a CPU profile of three owners (962 s of samples) the import walk (`walkNodeV1`) is 4.8 %. The rest is the shared closure entries every fresh child produces again: an owner needs 13 to 20 entries (the shared tool and provider groups, about 2,000 modules each); each is derived (13 per owner), saved as thousands of layer files into a private staging store that is deleted after the owner (5,100 to 6,600 `writeFile`, 39,000 to 76,000 `stat` per owner), verified row by row (`observationLayerHoldsAsyncV1` 10.7 %), exported and packed to be offered to the fleet (`packBundleV1` 19 %), plus forced garbage collections (7 %). A pool child is recycled after almost every heavy owner (RSS above 1.2 to 3 GB, `ownerPoolRecycleV1`), so every owner is a cold start.
3. Landed: (a) pool children share ONE bounded closure store per machine, so the next child loads what the last one derived; (b) a target that declares module inputs, globs or runtime entries is cut at the declared tools like every other target (an edit inside the build library no longer stales its entry), the toolchain script every component target lists is a declared tool; (c) a kill/resume law over the stored owner records.
4. Measured after, six distinct owners each in a fresh process, old and new interleaved at load 42 to 55: CPU sum 279 s to 167 s (-40 %), median 20.9 s to 12.6 s, last owners 4.4 s CPU and 6.6 s wall (was 19.1 s and 19.5 s); `writeFile` per owner 5,000 to 6,600 to 10 to 330 once the store is warm; peak RSS median 1.2 GB to 0.8 GB. It is a 1.7 to 4 times gain, not the 100 times that "well under a second" needs: see the open items.
5. Settled owners do survive a kill at the record layer (new law: SIGKILL after two owners settled, the next process reloads the two and infers the other two); what looked like starting over on the workers is the file inventory, the take pass and stale closure entries, not lost records (section 5).
6. Pick-up: no restart needed, a worker's next pool child starts from the files on disk (section 6). One caveat: `provider-closures/🔣️.json` is a data file of the inference authority, so the record identity moves once; mixed old/new code refuses each other's records until both have restarted.

## 1. Measurement (task 1)
Coordinator, one owner through `PN🏊️project-farm/👷️service/🟨️.mjs` `open`+`run` (the code a pool child runs), real wave session (`.cache/f/owner-pool-waves/…/session.bin`), the fleet lane on (as in production), host load 36 to 72 on 10 cores, `sysctl kern.num_files` 9.6 to 10.1 k.

| run | owner | wall | CPU | peak RSS | closure entries | fs calls |
|---|---|---|---|---|---|---|
| cold, load 38 | dag-dag (TS package) | 283 s | 108.8 s | 2.6 GB | | 2,824 reads, 38,194 stat, 5,668 writes |
| cold, load 70 | dag-dag | 272 s | 71.5 s | 2.7 GB | | 2,824 reads |
| same process, second owner | draw-js | 207 s | 50.0 s | 3.95 GB | | 2,911 reads |
| cold, load 40 (`A`) | dag-dag | 48.9 s | 40.6 s | 2.4 GB | derived 13, loaded 7 (fleet fetched 6, published 5) | 38,949 stat, 5,742 writes, 4,924 statSync |
| shared store, cold | dag-dag | 193 s | 63.4 s | 1.9 GB | derived 7, fetched 13 | 36,672 writes (14 entries imported as files) |
| shared store, 55 s later | dag-dag | 52.6 s | 38.0 s | 1.7 GB | derived 5, loaded 14, stale 6 | 68,521 stat |
| shared store, other owner | draw-js | 11.0 s | 6.3 s | 0.7 GB | derived 0, loaded 10, fetched 7 | 2,047 stat, 87 writes |

- Record of the owner: 222 rows, 8 of them `closure`, 172 `global` (generator contracts), 6 leaf files, no process spawned (`GR16/preload.mjs`). The answer is tiny; the cost is everything the answer rests on.
- Dominant term (cpuprofile, `Profiler` over three owners; async stacks are partial, so the shares are lower bounds): `packBundleV1` `🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts:314` called from `🚚️entry-lane/🟦️.ts:73` (offering a derived entry to the fleet, awaited up to 15 s per entry at `SI🟨️.mjs:1789`) 19 %; layer verification 10.7 %; `store.save` (`SI💾️entries/🟨️.mjs:68`, compress and write a layer per module) 9.5 %; forced `gc` via `releaseIfGrownV1` (`🧠️memory/🦶️footprint/🟨️.mjs:288`) 7 %; the real import walk 4.8 %; native JSON stringify 8.9 %, readFileSync 6.4 %, hash 5.2 %.
- Process start plus module load of the inference closure is 1.0 s CPU / 1.1 to 2.3 s wall per fresh child (`bootstrap` phase); hashing is 6.6 % of the profile (`update` 5.2 %, `Hash` 1.4 %), so re-hashing the closure per owner is not the term; no `cargo metadata` and no process spawn for the TypeScript and project owners measured (the shared-store cold run spawned 2); the Rust owners (`cad-cad-rs` 140 s CPU before it failed on a probe-only admission gap) need their own measurement with a real admission.
- After an unrelated tool-file touch: the "shared store, 55 s later" row. Peers kept editing tool files, 6 of 20 entries were stale, and the owner cost 38 s CPU, as much as cold. So today any edit of the shared 1,901-file core (GR10) sends every fresh child back to the cold figure.

## 2. What changed (task 2)
a. Validity closure. `SI🟨️.mjs:948` `toolCoverDeclaredV1` (called at `:1008`): a module a target declares (literal input, glob, runtime entry) that is the entry of a declared tool is named by one reference and not walked; `keepsWholeWalkV1` (which switched the tool cover and the walk stops off for every such target, so its entry carried a full walk of 2,381 files, GR10 item 4) is gone, so the cover of the command script and the stops apply to every target. New tool `cargo-execution` (`SI🔌️provider-closures/🔣️.json:137`, view native, EN and DE contract, ceiling 2,275 in `🧪️tests/📏️closure-ceilings/🔣️.json`, corpus tool in `🧪️tests/🔣️.json`); `native-toolchain` stays resource-only, so the keys of every other native target are unchanged. The keyed files never shrink: the group is keyed through the reference (law checks the expansion). Task hashes of the component targets may change once. Not done: a per-plugin declared subset of the inference authority (section 7, item 4).
b. Batching and bounded memory. The pool child was already one long-lived process per many owners with an RSS recycle and cancellation. What made it one owner per process was the private store. `PN🌐️owner-pool/🔬️inference/🟨️.mjs:23` `ownerPoolClosureStoreV1` and `:37` bind `.cache/f/owner-pool-closures` writable with limits from the pool policy (`🧬️schema/🔣️.json:106`: 8,192 entries, 65,536 layers, 8,192 values, collected by the store's own grace GC); `SI🟨️.mjs:1255` `bindClosureStoreV1(root, dir, writable, limits)` and `PN🟨️.mjs:279` (`closureStoreWritable`, `closureStoreLimits`) carry it. The published record is byte-identical (222 rows, 6 leaves, same in all runs).
c. Settled owners survive: law below; no code change was needed at the record layer.

## 3. Tests (one `repo test-file` at a time)
- New `SI🔌️provider-closures/🧪️tests/🧱️whole-walk-boundary` 4/0 (no tool row in the entry of a literal/glob/runtime target; edit inside a tool plus a touch of a tool file nobody imports plus the toolchain script: 0 of 3 entries stale; an own edit stales that owner alone; the literal is covered by `toolClosure:cargo-execution:native`, keyed files are never fewer). Differential: with the old gating restored temporarily, 2 of the 4 fail (`error: literal`, stale count above 0).
- New `PN🌐️owner-pool/🧪️tests/🗃️shared-closures` 2/0 (policy-bound store path and limits; a fresh child loads what the first derived, derives only the moved entry).
- Extended `I🧪️tests/♻️owner-reuse` 2/0: steps "every owner's data changes and the process is killed after two owners settled" (`SIGKILL`, per-owner lines), "the next fresh process reloads two, infers two", "the build after reloads everything".
- Regression: tool-cover 6, tool-boundary 6, blast-radius 3, closure-ceilings 4, preflight 2, provider-closures main 5 (corpus updated), shared-groups 2, walk-views 5, answer-validity 10, script-inputs main 4, owner-pool main 32, staged-leaves 2, stale-take 1, take-no-derive 1, refs-file 4, closure-facts 22: all 0 fail. `🔒️lint` still fails with the same 6 module-level names as before (GR13), none mine.
- The third-party oracle of the closure laws is esbuild (existing `tool-boundary`, `tool-cover`); the store law has no third-party analogue.

## 4. Measured again, and the estimate (task 3)
No new whole read was started (the 08:35 read of pid 28540 runs with the old code; the preamble forbids touching it). Bounded measurement instead: `summarize-ab2.py` over six distinct owners, each in a fresh process, old (private store) and new (shared store) interleaved, load 42 to 55:

| owner | old wall / CPU / peak | new wall / CPU / peak |
|---|---|---|
| dag-dag (store cold, fills it) | 37.1 s / 21.4 s / 1,066 MB | 100.9 s / 34.6 s / 1,297 MB |
| draw-js | 30.6 / 16.3 / 1,287 | 28.4 / 12.0 / 640 |
| cad-js-module-aec-building-structure | 33.9 / 20.8 / 1,856 | 24.7 / 13.2 / 1,017 |
| cad-js | 216.4 / 180.8 / 2,963 | 129.7 / 92.9 / 2,258 |
| compiler-syntax-ecma-ts | 31.4 / 21.0 / 2,082 | 14.3 / 9.5 / 947 |
| canvas-react-renderer | 19.5 / 19.1 / 1,173 | 6.6 / 4.4 / 401 |
| total / mean CPU | 279 s / 46.6 s | 167 s / 27.8 s |

Owners per minute per fresh child: old median 32 s wall = 1.9, new median 26 s = 2.3 and the last owners 9; per machine it is the memory grant that decides (pool.json: grant 4,915 MB of 16 GB, child 1,041 MB, 0 to 1 children today). Estimate for the 5,231 owners: CPU 244 k s to 145 k s by the means (68 to 40 CPU hours), 109 k s to 66 k s by the medians; with the 11 workers at 4 children each (44 parallel, 1.0 GB per warm child) that is about 1 to 2 h, against the 12 to 16 h of the read estimator (1 child per machine, 150 to 520 s per owner). Not measured on a worker and not on a whole read: treat it as an estimate.

## 5. Settled owners and "attempts that look like starting over" (task 2c)
- Where records settle: a pool child publishes each owner's record to the fleet cache when it finishes it (`PN🌐️owner-pool/🔬️inference/🟨️.mjs` `claim`/`publish`), the reader writes an adopted record durably (`publishFileDurablySyncV1`); nothing waits for the end of a read.
- Verified by the law above (SIGKILL after two of four owners, fresh process: two reloaded, two inferred) and by the existing owner-reuse steps (fresh process per build, no edit: 0 inferred and 0 closure entries derived).
- Why W7 saw attempts 9 to 17 start over: a restart pays the file inventory again (about 220 s), the take pass is counted from 0 (`taken 0 of 608`), and every record whose closure entry went stale under a peer's edit re-asks or re-infers (GR12). That is time, not loss. The unexplained part of W7 item 2 (m06, m11, m12 show `taken 0` where m08 shows 52 at once) needs a worker-side trace that I did not do.

## 6. What a worker needs, and time to Ready (task 4)
- Pickup: the pool puller spawns every child from the files on disk (`PN🌐️owner-pool/🏃️runner/🟨️.mjs:82`, `[process.execPath, childEntry, file]`), a child lives at most 30 min and is recycled after almost every heavy owner, so a worker's next child runs the new driver, session fields and policy as soon as the replica has delivered the files (about 1 s). No restart by hand. The long-lived puller keeps its old modules but only reads the keys it already knew.
- Caveat to decide (I did not act): `provider-closures/🔣️.json` is in the inference authority (103 data files, 7 of them changed in 12 h), so every record made by new code has another identity than the records of old code. A process that still runs the old code (the coordinator read of 08:35) refuses records of new children until it restarts; the fleet re-infers once. It also means the first owners of the new code are all cold (the store is empty).
- Time to Ready on a 16 GB worker: not measured on a worker. By the estimate above the `workspace` selection (605 owners, 12 % present) needs about 530 inferences; at 4 children and 26 s median that is about 1 h wall before the 23 min build, so expect roughly 1.5 to 2 h to Ready after the pick-up, if the memory warden admits 4 children (today it admits 0 to 1 while the stuck Ready stages hold the memory, W7).

## 7. Open items (ordered by expected gain)
1. Per-process verification: a warm child still verifies 10 to 17 loaded entries (6 s CPU, 2,000 to 244,000 stats per owner); hand the parent's verdict of the shared groups to the children (a per-machine verdict directory with a time window), so a group is verified once per window, not once per child.
2. The shared 1,901-file core (GR10): any edit of it stales about 18 tool groups; cost now once per machine per edit burst instead of once per child, but the walk boundary at view-pruned prefixes (GR10 option 1) is still open.
3. Layer files: an entry is thousands of files (36,672 writes to import 14 fleet entries, 244 k stats for `cad-js`); one pack file per entry would remove most of the I/O (store format change).
4. Authority: 103 data files (policy and schema JSON of the dependency managers, storage budgets, caches) are part of every record's identity whatever the owner reads; a declared subset per inference plugin is the per-plugin validity closure the task asked for, and needs a semantic map of which owner kind reads which policy.
5. The offer of derived entries to the fleet is awaited up to 15 s on the owner's critical path (`SI🟨️.mjs:1789`) and packs 19 % of the CPU; make it a background offer by one process per machine.
6. Forced garbage collections (7 %) after every growth step of a pool child.
7. Rust and Cargo owners (704 + 1,869 of 5,231, median 64 to 147 s in the owner-costs table) were not profiled: the probe has no admission for native dependencies.
