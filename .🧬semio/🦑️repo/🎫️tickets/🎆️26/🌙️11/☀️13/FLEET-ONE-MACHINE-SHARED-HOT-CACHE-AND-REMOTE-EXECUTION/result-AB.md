# Package AB: a graph read that costs what changed (2026-11-14 09:50 to 10:50, live tree g1-server, 32 GB, load 8 to 17)

## Verdicts (against the targets: unchanged under 10 s, after a small peer edit under 30 s, parent footprint under 3 GB)
| read | before my landings | after my landings | target |
|---|---|---|---|
| (1) nothing changed | **36.5 s** (read4), 26.9 s (read5), 28.0 s (read6) wall; 3 reads queued behind a peer's rebuild for 198 s first (below) | **20.0 s** wall (19.6 real, 17.7 user, 5.2 sys); 14 further unchanged reads: 19.5 to 25.3 s, median 22.5 s | < 10 s: **NOT MET** |
| (2) after one source file of the build was edited (my own landing; 3 projects stale) | 172.1 s (read7) | 166.9 s (m1) | < 30 s: **NOT MET** |
| (3) after another machine's edit arrived | not obtainable before | **no peer edit arrived**: 13 further reads over 16 minutes (10:30 to 10:46) all found the generation 86 unchanged; the fleet was quiet | < 30 s: **NOT MET** if it refreshes any project |
| parent physical footprint | 1.3 to 3.5 GB unchanged (noisy), 4.3 GB stale (read7) | **1.85 GB** unchanged (14 reads: 1.4 to 1.9 GB), **2.37 GB** stale (children 1.0, 0.5, 0.5 GB) | < 3 GB: **met** for the parent, not for the tree of a stale read |
The original 314.7 s (`graph-warm-1.log`) is not reproducible any more: it was the first read after a generation had been published by a cold build without a recorded cursor, and it refreshed 7 projects. The same run structure today (read2, 290 s) is dissected below. What is gone: the 150 to 230 s verify epoch after a cold publish (landing 1, tested, not re-observed live), 5 s of the 7.4 s fact verification of every pass (landing 1), two of the three passes when the cursor does not move (package X) and 0.6 to 1 s of classification (landings 2, 3). What is NOT gone: 6 s of classification of 678 000 rows per pass, 3 s of manifest verification per call, 18 to 21 s of closure verification when any fact is stale, 31 to 45 s between the last inferred project and the published generation, 8 s of upload.

## 1. Where the 290 to 315 s went (read2 = a read with a recorded cursor, before package X's plan-once; same structure as the 315 s run, whose only extra was the epoch of 110 s)
Times from line counts of stderr sampled every second (`fp.py` in the scratchpad), phase milliseconds from `SEMIO_GRAPH_PHASES=1`.
| s | what |
|---|---|
| 0 to 10 | boot, admission, source capture, dependency verification 1.4 s, manifest cursor 3 s |
| 10 to 70 | pass 1: membership 0.7 s, classification of 678 370 facts (doubt) 9.3 s, verification of 119 339 facts 7.4 s, 5 input closures 40 s |
| 70 to 90 | prefetch / builder lease / capacity upgrade (a gap with no output) |
| 90 to 110 and 120 to 140 | pass 2 and pass 3, each membership + doubt + facts (about 20 s) and closures; the same 119 339 facts again |
| 140 to 220 | 4 projects inferred (the largest took 55 s) |
| 220 to 270 | generator producers, patch and pack of the fact index, publish, doubt stamps, retire |
| 270 to 290 | upload 8 s |
The three verification passes (U finding 6) are real; package X removed passes 2 and 3 when `seams.unchanged` holds. Finding: `unchanged` compares the exact snapshot instance, so ANY change of any of the 119 000 shared files during the 40 s of pass 1 (the fleet edits all day) fails it and a full second pass (16 s: membership 0.9, doubt 7.0, facts 0.9, closures 3.5) runs; read7 and m1 both show it. An incremental re-plan by the rows of `since(old cursor)` is the next cut (design in section 6).

## 2. What was changed (landings 1 to 4 in `landings.md`) and how each is proven
1. **Environment rows are vouched by the cursor** (`field`, `digest`: 27 500 rows, each read and parsed a manifest per pass; policy `fileRowKinds`, `rowFilePathV1`). Proof: `🧪️tests/🗂️file-rows` 9 tests: mapping equals an independent Python model on 300 random cases (path forms, dirty sets, subtrees), level table (cursor names the file, outside the workspace, unseen region, `stamp` kind), a reported change is never passed for every file kind of row, and the rows of a changed file are doubtful in every dirty shape. Live: facts phase 7.4 s -> 2.2 s (pass 1, 119 495 -> 91 809 rows), 0.8 s in pass 2.
2. **A cold build records its cursor when it publishes** (`published`), not only when it is sealed after the final assembly. The reader that starts in between needed a full epoch (`No verified change journal cursor is recorded for generation 79`, 110 s of warm-1). Proof: test `a generation published by a cold build has its cursor before the build is sealed`, scoped builds record nothing, an edit made after the recorded cursor is doubtful. Soundness: the cursor is taken at the start of the build, every file the build read later and changed in between is reported by the next `since`. NOT observed live (it needs a cold publish).
3. **Memo per pass and verdicts by id** (landings 2, 3). Proof: ids equal rows for 11 row kinds x 21 paths x 4 dirty shapes with zero rows decoded; doubt module test; fact-codec test (87 pass). Live gain only 0.5 s (6.9 -> 6.0 s): row decoding was not the cost (measured below).
4. **Structural row equality in the stream patch** (`sameFactV1`, sound shortcut for key equality). Proof: 3 tests including a Python model, stream-patch 4, row-log 7, stream 6, patch-drift 2, waves 6, generations 30, load 92 pass. Live effect NOT proven: the tail after the last inferred project was 45 s before and 41 s after.
No compatibility layer, nothing removed that is not replaced; `[DEBUG]` lines were added twice and removed again (grep shows none).

## 3. Measured facts that decide the next steps
- **Classification (doubt phase, 6 s per pass)**: a loop over all 685 000 rows costs 5.6 s; materialising the rows (with their facts) is 1.3 s, the rest is the verdict of 367 000 distinct workspace paths: `excluded()` of the manifest port (Set lookups + roster class + gitignore matcher `sharedV1`, 14 microseconds per unknown path) 3.4 s. An ancestor shortcut was tried offline (identical verdicts on all 367 000 paths) and gains nothing. The way to O(changed): a persisted table of the static verdict per path, updated by the rows of `since`, and a merge-join of the sorted string table of the fact index (front coded, UTF-8 order) with the sorted snapshot, so no string is decoded.
- **Level 2 rows**: 78 000 rows (readdir / exists of directories the manifest does not contain) are checked by directory stamps (156 000 directories, 2.4 s stat) and `epoch.settled` writes them again (1.9 s every pass).
- **Closure verification** (18 to 21 s when any fact is stale, 40 s for 5 closures): re-derivation of the answers whose supporting file changed; the first one loads the whole inference library.
- **Between the last inferred project and the published generation: 31 to 45 s**. A temporary timer showed `fragments stored` -> `index patched` = **31.5 s**, pack 0.9 s, publish 0.3 s, `doubt.published` 2.6 s. My offline bench of the same patch for the biggest owner (108 835 rows, all equal) takes 5.6 s, so the in-process 31 s is not the row matching alone (it contains four forced full garbage collections `runtimeCollectGarbageV1(true)` on a 2.3 GB heap, and the stream upsert of the derived pseudo owners). Not resolved; see section 6.
- **Upload 8 to 11 s**: 5 104 documents are read and neutralized again every time (`transformGenerationV1`); the fact index transcode alone is 6 s. Per-fragment caching saves 2 s, so it was not built.
- **Capacity**: a read waited **198 s** (read3) behind a peer's rebuild: `behind projects (running, 1 job(s), 28672 MB)`. The learned `phase:rebuild` ratio is 32 295 basis points (3.2 times the default) from a cold run, although the stale refreshes I measured peak at 4.3 GB footprint. A rebuild of 3 projects should not hold 28 GB (package V's area).
- **Memory**: `maximum resident set` 12 to 14 GB is freed malloc pages (the footprint of the parent is 0.2 to 2.4 GB). The 12.3 GB of warm-1 was RSS.

## 4. Why the group server reports the `verify` tier (answer, nothing changed)
`manifestTrustV1` reads `.cache/f/health.json`: `trusted` only when `failures` is empty, else `verify`. The failures are cumulative counters per kind since the daemon started (never cleared; `createFailuresV1.summary`). On this machine: `blocked` 126 360 (all `.git/objects/pack`: `pack-incomplete` about once a second for 5 hours and `EACCES` on one pack file, i.e. the git replica is stuck, not the worktree), `refused` 9 (old ticket paths of 09/23 refused by the boundary), `attest` 4. None concerns a graph-visible path, but the health file has only the kind, so the tier is `verify` for the whole life of the daemon after one such event. Cost of `verify`: `verifiedReplicaFilesV1` = lstat of 119 000 files, 3.0 s idle, 6.4 s at load 17, and `cursor()` and `since()` each call it (two calls per epoch pass); `trusted` is a 1 s manifest read plus the settle wait. Fix (not built, it is in `🤝️shared-worktree` and needs its landing protocol): the daemon counts git object blocks as their own kind (`objects-blocked`) and failures expire after a window; then `trusted` is the normal tier.

## 5. Correctness (what the tests prove, what they do not)
- An edit the cursor reports is never missed: file-rows tests above, the cursor property test of package S (8 seeds x 25 random edits, stale set through the cursor equals a full pass), graph-hooks 49.
- An edit the cursor cannot see (gitignored, unshared, roster-ignored regions, files outside the workspace): the rows there get level 1 or 2 and are checked by stamps or by the precise check (test: unseen region and outside rows reach the precise check). A file created in an ignored directory the walk enters is found by the membership scan (kept, 0.7 to 1.3 s).
- Cursor missing, daemon unusable: the epoch (full verification) runs unchanged. A periodic or background full audit is NOT built.

## 6. Design of the next cuts (measured, not built)
1. Incremental re-plan when the cursor moved during the pass: classify only the rows named by `since(old)` through an inverted path table (the fact index has typed `rowPath` string ids and a sorted string table; counting sort over `rowPath` is milliseconds), instead of a second full pass (16 s) in every stale read.
2. Persisted static path verdicts (section 3) for a 6 s -> about 1 s doubt phase and the unchanged read at about 12 s.
3. Find the 31 s of the patch (timers around the four garbage collections and the derived upserts) and move the upload off the critical path (deferred to process end).
4. Wait for the capacity grant of a read that has nothing to rebuild: do not queue it behind a peer's rebuild (package V).

## 7. Tests run (members touched and dependants)
graph-hooks 49 and file-rows 9, cursor 20, fact-codec 87, doubt 6, same-fact 3, stream-patch 4, row-keys 2, row-log 7, stream 6, patch-drift 2, waves 6, generations 30, load 92, carried-closures 3, portable 3: all pass. `pinned-publication` needs the repo root as working directory (envelope), not runnable under plain bun, not touched by my change. Real command through the changed path: `repo show projects` (m1 stale 166.9 s, m2 unchanged 20.0 s).

## 8. NOT verified
A cold publish followed by a reader (landing 1b live); the effect of landing 4 on the patch tail; Windows and Linux; the `trusted` tier live; any read under 10 s; the read after a peer edit if none arrived (see the table); a 16 GB machine; the audit job.

## 9. What other packages must know
- Every edit of a graph-hooks or generations file restales about 3 projects once (3 refreshed after each of my four landings, 100 to 170 s per read until the next generation is published).
- `T/🧰️scripts/AB/` has `fp.py` (footprint and line-count sampler), `classify-bench.ts`, `ids-bench.ts`, `patch-bench.ts`, `published-time.ts`, `tier-time.ts`, `doubt-census.ts` (run with the bun of `runtimes`).
