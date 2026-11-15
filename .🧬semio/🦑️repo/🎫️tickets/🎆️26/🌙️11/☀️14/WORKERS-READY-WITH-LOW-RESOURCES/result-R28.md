# Package R28: restart-to-first-inference of the graph read (PREPARED, NOT LANDED)

Scripts and payload: `T/🧰️scripts/R28/` (T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`). Nothing is landed. No read was started; the live read was only observed (`ps`, one 5 s `sample`, `footprint` header, the R20 diagnostics, its store files, one CAS blob read).

## Verdict in four lines
1. **Root cause found and measured: the closure settle manifest has been unreadable on every start.** The live record (`.cache/b/g/m/derived-nodes/n/7ab2cb43….json.br`, its value blob in the CAS) is **70 937 502 bytes of JSON** (8 129 closures, 25 095 parts, width 785 words, 204 124 path rows, on average **1 678 parts per closure**), the node store reads a value chunk with `maxOutputLength = maximumChunkBytes = 67 108 864` (`🧮️derived-nodes/💾️store/🟨️.mjs:23`, schema `🧬️schema/🔣️.json:49`), the read throws, `get` catches it and returns `undefined`, `manifestOf` (`closure-kinds/🟨️.mjs:87`) takes an empty manifest. So the incremental path (`affectedPartsV1` + `answerFromManifest`, `:176`) never ran on the live tree: all 8 129 suspects went through the slow path (`graph.get` + part metas + invalidate + a second `ask`), and each start rewrote the same 71 MB manifest at its very end (`:88`, only when the whole settle finished). The live record is version 1, `marks` is empty (0 entries): nobody ever read it back.
2. **Fixed by principle, not by a bigger limit:** the manifest is stored in 256 hash-chained shards (head + 128 entry shards + 128 index shards), saved while the settle runs, and closures are **vouched** by the stamps of the dirty files they read, so the same edits are verified once, not once per start.
3. **Measured** (below): production-shaped manifest 67.8 MB single record = unreadable, sharded = 257 records, largest 311 KB, save 0.54 s, load 0.31 s, idle replan 0.06 s, one changed closure = 1 shard written; synthetic 8 000 closures, 200 edited files + 2 shared files: second start over the same edits checks **0 rows (was 202) and derives 0**, 90 ms (was 190-230 ms) with a cheap `hold`; production `hold` is a re-parse, so the row count is the number that scales.
4. **NOT proven live** (no read may be started): the 60 s target. The prelude outside the closure settle (boot + admission, membership, fact classify/verify: 25-30 s measured in U/M) and the epoch case (no journal cursor = no dirty set = everything slow) are unchanged. First start after landing is cold once (the old record is a miss by design) and writes the sharded manifest while it runs.

## 1. Profile (what the prelude spends, with seconds, MB and file:line)

Sources: the live observations of today (log lines, the sample, the store files) and the numbers measured by packages S, U and M (`result-S/U/M.md`, same code) which I re-read in code. Where a number is old it says so.

| Phase | Where | Time | Memory | Notes |
|---|---|---|---|---|
| boot, sealed snapshot, admission | `🏛️sealed/📥️execution/🟦️.ts:22` `profileMarkV1("source.served", …)` | 10 s (U) | small | **`source.served` is not a computation.** It is the profile mark written immediately before the sealed child process is spawned with the stored snapshot (`:23-28`); the wrapper's `[repo] still working after 440 s; last phase started: source.served` only says the parent has seen no further mark because the child's phases (`inference`, `verification`, …) are printed as `build:graph-phase` lines, not as marks. The 7 minutes at 100 % of one core are the child's graph command. There is nothing to cache at that mark (the snapshot is a stored text). |
| membership | `🕰️generations/🟨️.mjs` `graph.build.membership` | 2-5 s (U, M) | small | journal `since(cursor)` when a cursor exists. |
| fact index load + classify + verify facts | `:285-304` `graph.plan.index/doubt/facts` | 10-18 s (U, M); 700 000 facts classified, 125 000 verified | 1.0-1.7 GB footprint | stat-vouched (`🔎️doubt`), already incremental. |
| **input closure verification** | `closure-kinds/🟨️.mjs:102` `runOnce`, `:130` `settleChunk`, progress `:131/:166/:179` ("Verifying input closures: n of 8129") | **40 s per pass with a cursor, 150 s without (U); 56 s (M); live today ~10 closures/s: 1 233 of 8 129 after ~2 min** | footprint 1.0 G (peak 1.7 G) | single thread (the sample below: 2 791 of 2 791 main-thread samples busy, every other thread idle in `__ulock_wait`/`kevent`); M measured derive concurrency 8→32 as slower (61 → 99 s): CPU bound, not I/O bound. 2-3 passes per command (shared read, restore/prefetch, builder: U item 6). |
| rederivation of touched closures | `:140-142` invalidation + "rederivation" stages | proportional to the hot-file edits (83 changed tool files = 3 300 derived closures, U) | rows of the touched parts | each non-fresh closure is asked a **second** time (`redo` = all of `again`). |
| manifest save | `:88` `saveManifest`, once at the end of `runOnce` | not measured; the value is 70.9 MB JSON, 3.08 MB brotli | the whole manifest string | lost whenever the start is killed (code changes restart the graph server every 90-600 s today: 06:42, 06:44, 06:48 in the log). |

Live sample of today (pid 4415, 5 min into a start at 8.3 → 9.3 GB RSS, 105 % CPU): `Physical footprint: 1.0G, peak 1.7G` against RSS 8.58 GB. So **the 11.5 GB is freed-but-resident malloc pages, not live memory** (S measured the same: 355 MB footprint, 11 GB reclaimable). The footprint peak is the rows of the loaded parts (`table`, `closure-kinds/🟨️.mjs`), not a materialised map worth streaming. `sample` cannot name JS frames (bun, no symbols): the attribution above is by code reading plus the measured store record, not by a flame graph.

What is re-verified on every start, against what: `graph.get` → `verifyObserved` → `holdParts` → `partHolds` → `scope.holds(rows)` → `classify` → the generation's `hold(rows)` per row, in the order of the suspect list; with a dirty summary an untouched part is skipped by `meta.touched` (`:115-118`), but **only the manifest path (`:176`) skips the closure itself**, and that path needed a readable manifest. It is not content-hashing files the cursor vouches for: it is re-asking every closure.

## 2. Design (what the patch does)

All edits are anchored (`T/🧰️scripts/R28/edits.py`), one module is replaced as a whole (`replace/`, guarded by the sha256 of the module it was built from), two test files are new.

1. **Sharded, hash-chained manifest** (`📒️manifest/🟨️.mjs`, new persisted form version 3, no migration: an old record is a miss).
   - Head (`closure-manifest\0all\0`): part ids, marks, volatile/config/bare sets, outside rows, `at`, and the list `[shard key, subject, digest]` of 128 entry shards and 128 index shards, sealed by `check = sha256(head)`.
   - Entry shard: names, digests, facts, lengths, **vouches**, part sets as words of one common width. Index shard: the inverted index (paths, dirs, bare) of the keys that hash into it (FNV-1a mod 128). Sorted, so equal content is equal text.
   - Subjects name their digest (`e17-<24 hex>`), so a reader of the previous head still finds the previous shard; shards are written first, the head last.
   - `planSaveV1` serialises and writes **only shards whose revision moved** (`revs`/`saved` bookkeeping in `recordEntryV1`, `forgetEntryV1`, `indexPartV1`) and only if the digest differs; every shard is written again once a day so none outlives the 7 day retention of the store (`🧬️schema:41-52`) unseen.
   - `readManifestV1`/`loadManifestV1`: wrong version, bad shape, missing shard, any digest or head check mismatch, part numbers out of range → **empty manifest (a miss), never trusted**. Parts loaded from the store are also checked against their id (`loadPart`: `partIdOf(rows) === id`, else a miss), and an entry whose parts are not all there goes to the slow path.
2. **Saved while it runs** (`closure-kinds/🟨️.mjs`, `settleChunk`): after each chunk's flush the manifest is saved when `manifestSaveMilliseconds` (15 000, schema `Policy`) and `manifestSaveSpendRatio` × (duration of the last save) (10) have passed. A start that is killed keeps every finished chunk; the next start answers those closures from the manifest and derives only the rest.
3. **Vouches** (`createVouchV1`, `stampOfV1` in `🧭️row-paths`): the journal names what changed since the *generation*, not since the last verification, so every start over the same edits asked the same closures again. A closure that the dirty set touches is now compared by a **vouch**: sha256 over the stamps (file = sha256 of the bytes, remembered by size/mtime/ctime/inode; directory = sorted names and kinds; link = target; absent = `-`) of exactly the dirty files and directories its parts read. Equal vouch = the same bytes were read = the recorded answer holds, nothing is checked or walked. Never vouched: parts a configuration, a volatile row, an install tree, an unwatched (`excluded`) path or a failing outside row can move; any file whose stamp differs between the start of the run and the end of the chunk that verified it (changed while it ran); stamps that cannot be read. The generation store passes `vouchStamp` only when it knows the workspace root (`🕰️generations/🟨️.mjs`, the `settler.settle({…})` call).
4. **Not done, with the reason**: parallel derivation in a worker pool (the derive callback is a closure over the process; M measured more concurrency as slower; a worker split needs the closure walkers to be serialisable first), streaming the final assembly (footprint is 1.0-1.7 GB here, the 11.5 GB is RSS), the cursor-less epoch (no dirty set exists; the vouch needs one), "plan once" across the three passes of a command (U item 6; with a readable manifest and vouches passes 2 and 3 should drop to the manifest path, not measured live).

## 3. Proof

Tests first: `T/🧰️scripts/R28/new/…/✂️cutoff/🧪️tests/🧾️vouch/🟦️.ts` (8 tests, 79 expectations) was written before the code and is red before (the file does not load: `stampOfV1` does not exist). Laws:
- a second start over an unchanged tree with the same journal derives 0 closures and **checks 0 rows**; with k = 3 edited files exactly the 3 closures that name them are derived; the same file edited again, reverted to an older content, or a shared file read by all 40 closures is never taken from a stale vouch;
- control: without the stamp the same journal makes every start check the rows again (the old behaviour is asserted too);
- a file edited while the run derives is not vouched for;
- a damaged shard in the store is a miss: the start is correct, the edited closure is derived, nothing is vouched;
- 13 ways to damage a persisted manifest (head check/part/version/time/shard list, entry digest/vouch/name/bits, index path, missing shard, swapped shard, no shards) and 7 junk inputs read as an empty manifest; a sound one round-trips with its vouches; the head check and a shard digest equal an independent `shasum -a 256`;
- only changed shards are written, an idle save writes nothing, a loaded manifest writes nothing until it changes, a day later everything is written again;
- the stamp of a file equals `shasum -a 256`, follows the bytes not the times, a directory stamp follows names only;
- a settle killed (SIGKILL) at the 35th derivation of 60 leaves a durable manifest with the 30 finished closures, and the resumed start derives at most the 30 unfinished.

Existing tests of the touched modules on the overlay (`BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`, `suite.sh`), before = live tree, after = overlay:

| test file | before | after |
|---|---|---|
| closure-kinds/📒️manifest | 1 pass | 1 pass |
| generations/✂️cutoff | 159 pass (flaky heap/GC tests red under load 20+, green alone: seen red before and after) | 159 pass (one of the heap tests red once under load, green in 4 of 4 isolated reruns before and after) |
| generations/✂️cutoff/🧾️vouch | (file does not load) | 8 pass |
| derived-nodes | 7 pass, **7 fail** | 7 pass, **7 fail** (the same 7 tests, red on the live tree too, not touched) |
| generations | 30 pass | 30 pass |
| generations/🔌️load | 99 pass | 99 pass |

Two existing assertions had to change with the format and are edited in `edits.py`: the manifest round-trip test compares sorted keys (shards do not keep insertion order), the cutoff test that counted node files ("… and the settle manifest" = 1) counts a head and its shards.

Micro-benchmarks (`bench.ts`, `bench-manifest.ts`; real files, real node store, in-memory CAS; bun 1.4.2, load 15-22):

| scenario | before | after |
|---|---|---|
| manifest of the live shape (8 129 entries, 25 095 parts, 200 760 paths, 1 678 parts per entry): single record | 67.8 MB, **load fails** (`Cannot create a Buffer larger than 67108864 bytes`) | not applicable |
| same, sharded | – | 257 records, **largest 311 KB**, head 1.18 MB, total 67.9 MB JSON / 6.3 MB brotli, full save 0.54 s, **load 0.31 s**, idle replan 0.06 s (0 writes), one changed closure 0.06 s (1 write) |
| 8 000 closures, cold derive | 3.3-4.0 s | 3.5-4.7 s |
| first start after 200 own + 2 shared (of 50) edits | derives 520, checks 202 rows, 0.47-0.63 s | derives 520, checks 202 rows, 0.52-0.56 s (records the vouches) |
| **second start, same tree, same journal** | derives 0, **checks 202 rows**, 190-225 ms | derives 0, **checks 0 rows, 520 vouched**, 78-93 ms |
| third start | 160-230 ms, 202 rows | 72-88 ms, 0 rows |
| cold run killed at the 4 000th closure, restart | derives 4 001, 2.5-2.9 s | derives 4 001, 2.25 s (the 3 840 finished closures come from the manifest saved during the run) |

The synthetic `hold` is a hash of a small file; the production `hold` of an `analysis` row is a re-parse, so the row count (202 → 0) is the figure that carries over, the milliseconds are not.

Parse proof: `bun build --no-bundle` of all 9 changed or new `.mjs`/`.ts` files and `json.load` of the changed schema: all OK. Dry run of `land.py`: 7 edits (6 anchored + 1 replace), 2 new files, no REFUSED. Rehearsal (`rehearse.py <scratch root>`): lands into a scratch root, second dry run clean (idempotent), `--rollback` restores all 7 files byte for byte and removes the new test files.

## 4. What the other packages and the coordinator must know
- **One cold start after landing.** The old record is a single version 1 record; the new loader treats it as a miss. That start costs what a start costs today (first project at 160-200 s with a cursor, 280 s without: U) and leaves a readable manifest, saved while it runs.
- Landing restarts the server's graph read like any edit of the sidecar closure (`closure-kinds` is in the graph server's code); land it in the same window as the other prepared packages. Edits touch no file of R5c (`🏗️build/🕸️graph/📥️inference/📦️project-nodes`), R26 (workspaces/format stamps/restart triggers) or R24; they touch `🕰️generations/🟨️.mjs` at one call site and one import line.
- `closure-kinds` now re-exports `stampOfV1`; `settle()` accepts `vouchStamp(name) → Promise<string|undefined>` (absent = no vouching, the old behaviour), returns `vouched`.
- Policy keys added to `✂️cutoff/🧬️schema/🔣️.json`: `manifestSaveMilliseconds` 15000, `manifestSaveSpendRatio` 10, `vouchConcurrency` 64.
- The stale 71 MB record and its CAS blob stay until the store's own prune (7 days); nothing deletes them.

## 5. NOT verified
- Any live read (not allowed): restart-to-first-inference on the real tree, the 60 s target, the share of the 8 129 closures that are vouched in practice, the real size of the sharded manifest on disk, the save time of a full 257-record write on the real store with the real CAS (measured only with an in-memory CAS).
- The cursor-less epoch and the first start after landing are unchanged by design.
- A flame graph (bun gives no JS symbols to `sample`); the attribution of the 7 minutes is by code reading and the measured record, with the earlier per-phase seconds from S/U/M.
- Windows and Linux (the stamp uses `lstat` with `bigint`, `readdir`, `readlink`; the shard hash and file names have no platform dependence, not run there).
- `🧭️invocation/🕸️graph/🧠️memo/🧮️derive` and `🎯️task-state` also use `sharedClosureSettlerV1`; their test files were not run (the settle contract they use is unchanged, `vouchStamp` is opt-in).

## 6. The exact landing command

```
python3 "/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R28/land.py" --land
```
(`--dry-run` first; it takes `T/landing.lock`, writes backups to `T/🗑️generated/R28-landing-<time>/`, runs the six test files, appends to `landings.md`; roll back with `land.py --rollback <that backup dir>`. If `REFUSED … not the one replace/ was built from` appears, a peer edited the manifest module: run `make_manifest.py` again to re-merge.)
