# result-mem2 (wave 1027b, lane mem2, mirror prefix m2): what the graph child holds before its first project, and cuts toward builder < 2 GB

FOR coordinator: batch ready `C/mem2-tools/staged/factview/` (2 files: `🕸️graph/🕰️generations/🟨️.mjs` +1 line + one docstring sentence, new law `🕰️generations/🧪️tests/🍂️view-release/🟦️.ts`; base = live 19:40 (after G46 and the multiwriter revert); `stage.py check` on live at 20:55: 2/2 base). A generation build drops the verification view of the fact index (`indexCache` key `view:<digest>`) right before `compute`, so the graph child no longer carries the rows its plan materialized through the whole inference. Apply `python3 -I C/mem2-tools/staged/factview/stage.py apply`; laws `sh C/mem2-tools/staged/factview/verify.sh <root>` (mirror m2C = live 19:40: new law RED on base 3/3 (`opened: 0`: compute still finds the cached view) -> GREEN 3/3; verify.sh = new law + generations 🟦️.ts, plan-once, index-chunks, fact-index: 44 pass / 0 fail twice; load-timing flakes seen once each on base and on batch: `the chunked builder equals the expanding builder…` (10 s budget), `3,000 owners … in bounded time`, `the exclusive hold…`); revert `python3 -I C/mem2-tools/staged/factview/stage.py revert`; rebase after a peer edit `python3 -I C/mem2-tools/mkbatch.py C/mem2-tools/staged/factview --root <tree>` (2 anchors, each exactly once; rebased twice today: multiwriter in, then out). Parse scan (`gate-g-tools/scan.ts`) 2/2 ok, `tsc --strict` on the new law 0 errors, stage.py from the mode-keeping template (`test_stage_mode.py` OK).
FOR coordinator: product proof = the next refreshing print after a library edit, `python3 -I C/buildmem-tools/product-fp.py g<NN>-mv1 @semio-tech/print-rs:test`: the graph child should peak ~0.3-0.5 GB lower (the view of the 295k rows a print refresh verifies is free before its first project); output unchanged (the view is only read by the plan; publish loads the whole index to patch it, as before).
FOR farm/policy owners (decision by measurement, not staged): keep `minimumProjects` 24. Farming earlier does not shrink the parent: in-process b1 parent max RSS 3146 / footprint peak 2657 / live end 1902 MB; farm at 4 misses e4/e4c (31-32/36 adopted) 2919/2654/1851 and 2839/2675/1850, tree +1.3 GB, wall +15 % (1051 s vs 908 s, load 37-54); farm at the FIRST miss x1 (35/36 adopted, the parent inferred only framework) still 2622 / 2692 / 1747. The project that opens the farm and every cold project started while a worker boots are inferred by the parent anyway (`prepare` answers false), and the parent retains ~1.0 GB over its 36 records that no release frees (section 3).
FOR next lane (builder < 2 GB): the remaining parent memory is not the closure caches in farm mode (only 128 MB) but ~1.0 GB retained per 36 projects that `release()` does not free (x1: 1465 MB live after closure entries, closure scripts, observation memos, input memos, interner and the result were all released; a 1-project run keeps 451 MB). Next: a V8 heap snapshot (`Bun.generateHeapSnapshot("v8")` works in the pinned bun 1.4.2) of a 4-6 project farm run, retainers by owner, under a streaming analyzer.

## 1. What the parent holds before its first project (graph-free, measured)
Harness `C/mem2-tools/` = farmid's captured-snapshot harness (production parent loop `emojiProjectJsonNodes` inside a captured snapshot, stream loader, snapshot as descriptor 3) + phase marks (RSS, `ru_maxrss`, JSC heap, macOS physical footprint by `/usr/bin/footprint`), snapshot rerooted to the live workspace, live workspace data read-only, scratch fact store, JSC RAM 2 GiB as the graph child; 36 projects of the g33 print refresh. Runs b0stop, b1, e4, e4c, x1, fw1 (host load 37-54):
| mark | t s | RSS MB | footprint MB | JSC heap MB |
|---|---|---|---|---|
| loader start | 0 | 15 | 6 | 0 |
| snapshot read + digest (19.8 MB regular file) | 0.1 | 42 | 28 | 0 |
| JSON parse + admission (payloads lazy) | 0.1-0.5 | 60 | 42 | 24 |
| entry module graph evaluated | 0.6-3 | 119-131 | 65-90 | 28 |
| whole library imported | 1-6 | 186-189 | 102-118 | 10-44 |
| harness inputs (git ls-files of 465k paths; harness only) | 11-19 | 280-344 | 157-202 | 80-142 |
| discovery (`discoverNativeBuildOwnersV1`, every Cargo.toml) | 44-77 | 660-784 | 306-566 | 225-239 |
| contracts, command inputs, 4,020 configs, os-dev playground | +3 | 669-728 | 297-527 | 227-249 |
| cargo targets, farm + governor open | +3 | 736-837 | 333-641 | 233-250 |
| FIRST PROJECT | 17-84 | 771-895 | 282-533 | 243-245 |
- Snapshot + base64 + module graph = ~190 MB RSS / ~110 MB footprint; the loader keeps 1,405-1,417 decoded module images = 12.5 MB. The cuts "stream/mmap the snapshot" and "drop loader source strings" are worth at most ~20 MB (the 19.8 MB buffer, which the farm needs again for its worker image) + 12.5 MB: NOT staged (immutability-critical loader, < 3 % of the target).
- The harness parent reaches its first project at 0.77-0.90 GB RSS. farmid's "1.7-1.9 GB before the first project" came from a series without a project mark (its plateau lies after the first heavy projects: framework done at 1.46-2.2 GB).
- What the harness lacks and production has: the generation. Read-only probe `C/mem2-tools/genprobe.ts` on the live generation (669/674, 548,987 fact rows, 8.8 MB packed / 45.5 MB expanded): pointer + manifest 39 MB RSS; opening the fact index view 336 MB RSS / 234 MB live; every row read (a whole-workspace verification) 834-846 MB RSS / 523-525 MB live / footprint 526-577; released: live 523 -> 33 MB, footprint 577 -> 427 at once (the rest is allocator pages the next allocations reuse). A print refresh verifies 295k rows (`build:graph-phase:facts:…:295146`, g45 log) and kept that view in `indexCache` until `engine.dropIndexes()` after publish. So the production graph child = ~0.19 GB library + ~0.5 GB generation view + ~0.6 GB setup before its first project = the 1.7-1.9 GB baseline; `factview` frees the generation part before inference.

## 2. During inference
| run | inferred by workers | wall s | parent max RSS | parent footprint peak | parent live end | tree peak |
|---|---|---|---|---|---|---|
| b1 in-process | 0/36 | 908 | 3146 | 2657 | 1902 | 3132 |
| e4 farm at 4 misses (measurement patch) | 31/36 | 1051 | 2919 | 2654 | 1851 | 4427 |
| e4c same, live-equal image | 32/36 | 1792 (load 50) | 2839 | 2675 | 1850 | 4470 |
| x1 farm at the first miss | 35/36 | 946 | 2622 | 2692 | 1747 | 4245 |
| fw1 only framework, in-process | - | 52 | 1503 | 841 | 514 | 1503 |
36 owners / ~100k rows in every run. The data root is the live checkout and the images differ (b1/e4 = pre-G46 mirror m2A, e4c/x1 = m2C), so per-owner digests are not comparable across them: e4c vs x1 (same image, 25 min apart) equal row counts in 36/36 owners, digests differ in 10 (live peer edits between the runs); b1 vs e4 counts differ in 2 (print, repo-lib). Farm output identity is farmid's replay law, not re-proven here; factview does not touch inference (its law compares the generation with the Python model). In b1 the parent steps: first project 838 -> framework (TypeScript program, 1,240 files) 1460-1810 -> framework-os-kernel 2560 -> print (15.5k rows) 3140 MB.

## 3. What the parent keeps (attribution after the run, live MB after a full collection)
| step (cumulative) | e4c (32 adopted) | x1 (35 adopted) | fw1 (1 project) |
|---|---|---|---|
| live at end | 1850 | 1747 | 514 |
| closure entries reset (`resetClosureEntriesV1`) | 1722 | 1619 | 514 |
| closure scripts unbound | 1722 | 1619 | 514 |
| observation memos (`releaseInferenceMemoryV1`) | - | 1511 | 456 |
| input memos (`releaseInputMemoryV1`) | - | 1472 | 456 |
| string interner | - | 1467 | 452 |
| result dropped | - | 1465 | 451 |
In farm mode the closure caches are 128 MB (buildmem's 749 + 545 MB were the in-process case). ~1.0 GB (1465 - 451) grows with the 36 projects and is freed by none of the production releases.

## 4. Laws (factview)
| law | base | batch |
|---|---|---|
| a closure / whole-workspace delta refresh in a fresh store (= a graph child) computes without the view its plan opened (`readIndexView` inside compute opens a new one); the generation equals the Python model | 2/2 fail (`opened: 0`) | 2/2 pass |
| [adversarial] a custody that waited beyond the hint bound plans again, computes without a cached view and builds the edit made during the wait (Python model) | fail | pass |
| neighbours: generations 🟦️.ts, plan-once, index-chunks, fact-index | 40-41 pass + 1 timing flake | 41 pass x2 (flakes once) |

## 5. Not done / next
1. Heap snapshot of a 4-6 project farm run to name the ~1.0 GB the parent keeps (FOR next lane above); then the farm can take the parent below 2 GB.
2. Farm-first: when the farm is open, a cold project should wait (bounded) for a booting worker instead of running in the parent (`prepare` answers false today), else the heaviest first projects stay in the parent.
3. Publish: `readIndex` unpacks the whole previous index eagerly after inference to patch it; a streaming patch would cut the post-inference peak.
4. Product proof owed (FOR line above).

## Files
Tools `C/mem2-tools/`: `harness/{entry.ts,run.ts,mkpatches.py,base-patches.json,eager4.json,eager1.json}`, `prep.sh`, `run.sh` (M2_ROOTS, M2_MIRROR, M2_ATTR, M2_STOP), `loadermarks.py`, `marks.py`, `genprobe.ts`, `mkbatch.py` (anchored-edit batch builder, mode-keeping stage.py). Staged `mem2-tools/staged/factview/{edits.py,new/,files/,base/,manifest.json,stage.py,verify.sh}`. Outputs `C/🗑️generated/mem2/` (images, logs, json; deletable).

## Status
20:55 DONE: factview staged + verified on m2C (= live 19:40), measurements b0stop/b1/e4/e4c/x1/fw1 + genprobe complete; mirrors m2A, m2B dropped, m2C dropped at the end; no process of mine runs; nothing live edited.
