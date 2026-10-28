# Audit claims2 wave 1026g (aud-claims2-g, read-only)

Auditor: aud-claims2-g. Date: 2026-10-26, audit taken at 16:06 CET. Scope: every headline performance number in the 1026g `result-*-g.md` files and the scoreboard `scoreboard-1026g.md` (refresh 7, 13:44). Read-only: no build, no graph-loading command, no source edit, no git, no kill. Only my own scripts are in `aud-claims2-g-tools/`.

## 1. Method classes used

- OFFICIAL: real `bun repo` route with the graph (`repo run`, `repo exec`, `show`), on the shared host, with queue time included in the wall unless stated.
- CLI-PARTIAL: real `bun repo cargo` or `cache status` route. Real code and real logs, but not the acceptance command (`run <product>:build`), and for `cache status` not a build at all.
- HARNESS: graph-free run on a scratch copy, overlay, fixture, synthetic tree or in-process engine with virtual edits. Real code, not the product route.
- SIMULATION: Python model, trace replay, sleep bodies on a simulated host, or static census used as a count.
- MICRO: micro-benchmark on fixtures or synthetic data.

## 2. What I checked (evidence actually opened)

- Cited generated paths: 94 unique `🗑️generated/...` references across the result files, 70 resolve on disk. 18 do not: 8 are wildcard or typo placeholders (for example `chain-g/cachestatus-*`, `integ-g/c3{`, `preadmit-g/measure-final-{agent`) that do resolve when expanded. 10 are real misses: `churn-g/` and `churn-g/env-digest.log` (the churn-g result says its scratch folder was removed itself), `store-g/` and `store-g/native-storage-4.log`, `self-g/`, `nxret-g/`, `tsfix-g`, `bench-g/window/backup`, `integ-g/STOP`, `laws-g/classified.tsv`. Numbers of those lanes cannot be re-checked from disk. Full list: `🗑️generated/aud-claims2-g/cited.txt` (deleted at the end of this audit).
- Product JSON records `gate-g/*.json` (7 files) and their logs: all official numbers in the gate table re-read from disk.
- `rust-g` raw rows `🗑️generated/rust-g/runs.jsonl` (20 rows), streamed line by line.
- `bench-g/window/*.json` (course and dashboard prime and s1, peak RSS in bytes).
- `proof2-g/p1` (`rows.txt`, `tree.json`, t1-t5 logs), `chain-g/cachestatus-{before,after1}-1.json`, `preadmit-g/measure-final-{agent,human}.json`, `ram-g/sim.txt`, `lease-g/measure1.txt`, `integ-g/c2.course-dev.log`, `xplat-g-dashboard.log`, `zt-g-course.log`.
- Not re-opened (claims taken from the result text, log exists or not checked): hold-g 110-465 s, asm-g 25-30 s, testmem-g vitest rows, ts-g jsonl rows, bzsrv-g raw, e2eperf-g spec table, xplat2-g boot bench, nxperf-g census output.

## 3. Ledger of headline numbers

Verdict: CONFIRMED = number found in the cited raw record with the same value. TABLE-ONLY = number appears in the result text, raw record not located. CLAIM = no record cited.

| # | lane | number as reported | method class | evidence on disk | verdict |
|---|---|---|---|---|---|
| 1 | zt-g (gate batch 1) | course:build 1092 s, 6.0 GB, 170 re-inferred, refresh 355 s | OFFICIAL | `gate-g/zt-g-course.json` wall 1092.5, peak_tree_mb 6007, exit 0; log "refreshed in 355 s" | CONFIRMED (wall includes 356 s capacity wait) |
| 2 | xplat-g (gate batch 2) | dashboard build 1154 s, 3.0 GB, 1 project re-inferred in 34 s | OFFICIAL | `xplat-g-dashboard.json` wall 1153.7, peak 3037 MB; log "refreshed in 34 s" | CONFIRMED. The Bazel 208 s was not re-checked. |
| 3 | noop2-g + watch-g (gate batch 3) | print-rs:test 850 s / 7.2 GB RED, then 1090 s / 7.0 GB GREEN | OFFICIAL | `noop2-g-print.json` 850.3 s, 7204 MB, exit 1; `noop2-g-print3.json` 1090.2 s, 7011 MB, exit 0 | CONFIRMED |
| 4 | phase-g (gate batch 4) | course:build 3135 s / 5.1 GB, queued 2750 s, ran about 385 s | OFFICIAL | `phase-g-course.json` 3134.9 s, 5126 MB, exit 0 | CONFIRMED (queue dominates the wall) |
| 5 | gate-g 3c, noop2-g print2 | hot no-op course:build: 2714 s, 86 MB, never admitted; print2 1614 s, 85 MB | OFFICIAL, killed | `b3-course-noop1.json` exit -15; `noop2-g-print2.json` exit -15 | CONFIRMED: no no-op was measured |
| 6 | integ-g c2 | generation 553: one refresh of 22 projects in 81 s (scoreboard: 22 refreshed) | OFFICIAL | `c2.course-dev.log`: "refreshing 22 projects (generation 553)", "generation 553 published, 23 projects refreshed in 81 s" | CONFIRMED (log says 23 published, result says 22: minor) |
| 7 | integ-g c2 | course:dev ready after 757 ms | OFFICIAL | `c2.course-dev.log` "ready after 757 ms" | CONFIRMED (time to ready only; the run wall was 1026 s incl. queue) |
| 8 | proof2-g row 1 (churn envelope) | 297 s, 2.2 GB tree, 5 official execs, 7/7 pass | OFFICIAL test runs | `proof2-g/p1/tree.json` wall 297, peakMB 2173, maxProcs 5; `rows.txt` t1-t5 walls 40/18/27/31/178 s; t1-t5 logs 1+1+1+1+3 pass, 0 fail | CONFIRMED for the numbers. Two caveats below (F9). `env.try2.log` in the same folder is an earlier failing attempt (3 pass, 4 fail). |
| 9 | bench-g baseline | course:build prime 306 s, 8.76 GiB builder tree, RED; course s1 277 s, 8.11 GiB RED; dashboard prime 463 s, 3.50 GiB RED | OFFICIAL, RED | `bench-g/window/course.bazel.noop.prime.json` peakRssBytes 9406054400 (= 8.76 GiB), s1 8708161536 (= 8.11 GiB), dashboard prime 3761192960 (= 3.50 GiB), exit 1 | CONFIRMED. Not in the scoreboard (F8). |
| 10 | noop-g | print-rs:test before 618.0 s vs after 653.0 s; course:build r8 225.2 s; capture 10.6 s to 0.1 s (memo); pin 24 s to 2 s | OFFICIAL, loads 15-35 | result table, r1-r10 logs present (`noop-g/r8.ts`, not re-opened) | TABLE-ONLY. The like-for-like pair (print-rs:test) is not faster. The 225.2 s run is not a no-op (F1). |
| 11 | rust-g | dashboard edit: before 61.6 s, 3,659 MB (load 18.4); after 44.0 s, 1,341 MB (load 36.4) | CLI-PARTIAL (`bun repo cargo`) | `rust-g/runs.jsonl` rows `b1-edit1`, `after-edit1` | CONFIRMED (the numbers) |
| 12 | rust-g | no-op: 36.9 s / 5,156 MB and 96.8 s / 4,159 MB before; 94.5 s / 1,683 MB (prof-noop, load 23.5) and 177.9 s / 1,678 MB (after-noop, load 34.8) after | CLI-PARTIAL | `runs.jsonl` rows 4, 6, 14, 15 | CONFIRMED. The scoreboard tags 94.5 s with "load 35", which is the 177.9 s row (F2). |
| 13 | rust-g | concurrent crates: sequential 113.1 s and 118.0 s; two lanes 40.2 s and 73.4 s | CLI-PARTIAL | `runs.jsonl` rows `b3-seq1`, `b3-seq2`, `b3-par1`, `b3-par2` | CONFIRMED |
| 14 | rust-g | official `test-package-body-policy` 696 s wall, 6,235 MB, GREEN | OFFICIAL | `runs.jsonl` row `official-test` rc 0 | CONFIRMED |
| 15 | chain-g | `cache status` tree 2,363 MB to 580 MB (loader 1,973 to 378 MB); 10.2 s to 16.6 s wall | CLI-PARTIAL (graph-free, not a build) | `chain-g/cachestatus-before-1.json` peak_tree_mb 2363, `cachestatus-after1-1.json` peak_tree_mb 580; one sample each | CONFIRMED, n = 1 per side (F10) |
| 16 | chain-g | loader after kernel import 1,536 MB to 255 MB; graph waiter 354 MB to 104 MB | HARNESS / process samples | table in result; `ps` RSS, which is compressed RSS on macOS (result note) | TABLE-ONLY |
| 17 | chain-g | Bazel persistent worker 1,770 MB to 98 MB ("was 1.77 GB at 07:15") | unknown | no record cited for the 07:15 baseline or the 98 MB value | CLAIM |
| 18 | ram-g | 16 GB three-agent wait 59.3 h to 25.8 h | SIMULATION (165-request trace replay, Python port) | `ram-g/sim.txt` lines 6 and 9, wait_total_h 59.3 and 25.8 | CONFIRMED as simulation output |
| 19 | lease-g | interactive mean wait 94 to 25 min ("shard-7 replay") | SIMULATION (queue replay with reconstructed service times) | `lease-g/measure1.txt` line 3-4: FIFO interactive mean 94, priority 25 | CONFIRMED as replay output (F7) |
| 20 | bzsrv-g | three concurrent runs: 133 s to 87 s; waits 75/107 s to 18/54 s | SYNTHETIC scenario on real Bazel servers, 20 s holds, 2,500 MB simulated headroom | result text, scenario.ts | TABLE-ONLY, labelled graph-free (F6) |
| 21 | fix-g | burst overshoot 8.1 GB to 0.9 GB ("claim") | SIMULATION (16 bursts, `admission_model.py`) | result text | TABLE-ONLY, not measured (F5) |
| 22 | preadmit-g | no-op wall 7.62 s to 0.408 s (agent), 3.63 s to 0.41 s (human); "MEASURED" | SIMULATION (sleep 0.4 s bodies, simulated 16 GB host, door only) | `preadmit-g/measure-final-agent.json` wallMedianS 7.622 / 0.408; bodies are sleeps (result line 38) | CONFIRMED as numbers, but NOT a build measurement (F3) |
| 23 | nxperf-g | engine closure 1,233 to 746 files | static census | result text, count | CONFIRMED as count |
| 24 | nxperf-g | scheduling edit re-keys 12,817 to at most 1,171 Nx targets | SIMULATION / static upper bound (sensitivity.mjs) | result text: "upper bound: a glob that merely contains one counts" | TABLE-ONLY, not a re-key count (F4) |
| 25 | gram-g | index patch 2.63-4.13 s, +1,466 MB to 344 ms, +176 MB | HARNESS on real data (`patch-compare.mjs`) | result text, tool present | TABLE-ONLY |
| 26 | gram2-g | builder peak RSS 4.25 to 2.87 GB; footprint 3.67 to 1.97 GB (staged) | HARNESS, sealed generation 560, staged, mixes RSS and footprint | result text | TABLE-ONLY; target not met (its own text) |
| 27 | derive-g | env move: 144-159 s to 43-52 s; part:edges 24-25 s to 2.5 s; peak 4.66-5.0 GB to 4.2-4.4 GB | HARNESS, single process, scratch copy of generation 547 | result table | TABLE-ONLY. The scoreboard's "single process peak 4.06 GB" is not in this file (F11). |
| 28 | cutoff-g | scratch run 2: 121 s, 2.08 GB (one dirty project); run 1 359 s, 3.46 GB; run 3 337 s, 4.97 GB | HARNESS, scratch copy of generation 542, load 25-50 | result text | TABLE-ONLY |
| 29 | ts-g | warm typecheck, one edit: 6.9-12.4 s course, 6-16 s ui-react; cold 23-30 s | HARNESS (tsc on overlay copies, 22 + 12 steps) | `ts-g/course-inc-rows.jsonl` present, rows not opened | TABLE-ONLY (F13) |
| 30 | ts2-g | cold bun tsc 41 CPU-s, 3.9 GB vs engine 0.4-1.4 CPU-s per leaf edit | HARNESS (in-process engine, virtual edits, live tree) | `ts2-g/both.jsonl`, `cold-bun.log` present | TABLE-ONLY; the scoreboard range "0.4-5" is not in the result (F13) |
| 31 | lane-g, lane2-g | pooled lanes about 1.8x (2.1x at lower load); pair wall 8.1 s, peak 1,427 MB | MICRO / HARNESS (two consumers, fixtures) | result text | TABLE-ONLY |
| 32 | watch-g | journal 56-63 ms vs stat walk 218-311 ms | MICRO (50,000 observations, 2,500 synthetic dirs, nothing changed) | result section 3 | TABLE-ONLY (F12) |
| 33 | churn-g | fuzz 200/200 consistent; mean wall 0.70 to 1.33 s | HARNESS (fuzz on real tree, Python peer) | churn-g folder removed by its own note | TABLE-ONLY |
| 34 | disk-g | content in use 94.0 to 69.8 GiB; reclaimed 21.56 + 3.55 + 2.75 GiB | census of the real store (no workload) | result text; `disk-g` census outputs | TABLE-ONLY, measured census; budget 40 GiB NOT MET |
| 35 | store-g | sparse images: model equals measured volume; 1 GiB image = 2,097,152 blocks fully allocated | MICRO (hdiutil scratch images) | output folder missing | TABLE-ONLY |
| 36 | asm-g | fact index view 25-30 s CPU vs 0.5 s; print-rs:test 32.5 s vs 2.3 s | HARNESS (noop-matrix.sh, live tree, 10:36) | result text | TABLE-ONLY, not re-opened |
| 37 | xplat2-g | kern.boottime child process 4.1-4.5 ms vs in-process FFI 0.061-0.064 ms | MICRO | `xplat2-g-tools/boot-clock-bench.ts` | TABLE-ONLY |
| 38 | integ-g c3d | dashboard test-unit PASSED in 494.8 s, 2 action cache hits; build 305 s | OFFICIAL | result table only | TABLE-ONLY |
| 39 | bench-g historical | "course:build warm after a scheduling-file edit: Bazel Elapsed 10.4 s"; whole repo run 1167 s | OFFICIAL, from older lane logs, marked not quiet | result table (exec-f pair2 log) | TABLE-ONLY. Not a no-op; the run wall was 1167 s (F1). |

Queue times recorded in official records: gate course 356 s (zt-g), phase-g 2750 s, no-op attempt 2714 s killed, print2 1614 s killed, integ c2 generation wait 366 s, proof2 row 5 "1650 s+ in queue" (result text only).

## 4. Flags

F1. No-op target numbers are not no-ops or not like-for-like. noop-g presents "225.2 s hot course:build (r8)" as the earlier best for target 1. Its own table says r8 ran 147 s of Bazel actions and "none of the target runs is a Bazel no-op on this tree". The like-for-like print-rs:test pair is 618.0 s before and 653.0 s after, so the batch did not make that target faster: the pin (24 s to 3.9 s) and the plan child (21 s to 18 s) improved, but the source capture (10.6 s to 30.7 s, a miss under peer edits), the Bazel publish (56 s to 88 s) and the Bazel actions (380 s+ to 473 s) got worse. The capture gain (0.1 s) is only visible on the course:build memo hit (r8).

F2. Rust no-op numbers. The scoreboard says "Cargo no-op 94.5 s / 1.68 GB (prof-noop, load 35)". The jsonl says prof-noop ran at load 23.5. The load-35 row is after-noop (177.9 s, mostly capacity wait). The scoreboard also omits that the after-noop wall is no better than before (36.9 s and 96.8 s). Only the RSS improved (4.2-5.2 GB to 1.68 GB). This is the `bun repo cargo` route, not `run <product>:build`.

F3. preadmit-g is labelled "MEASURED". Its no-op figure (0.41 s) is a 0.4 s sleep body in the real door on a simulated 16 GB host. It proves the admission decision, not a build time. It should be labelled SIMULATION, and no official no-op exists.

F4. nxperf-g "at most 1,171 of 12,817" is a static upper bound from a census, not a count of re-keys that happened. The scoreboard calls it "graph-free: scheduling edits re-key at most 1,171".

F5. fix-g "8.1 to 0.9 GB" is a Python model over 16 bursts. It is in the scoreboard's "best MEASURED" column for target 5 ("fix-g (claim)"). It should be a SIMULATION entry.

F6. bzsrv-g runs used synthetic 20 s holds and a simulated 2,500 MB headroom on real Bazel servers. The scoreboard says "graph-free, real Bazel servers, simulated 2500 MB headroom", which is accurate, but it sits in the same column as official data.

F7. lease-g 94 to 25 min is a queue replay with reconstructed service times (`measure.ts`). It is simulated. The scoreboard calls it "Shard-7 replay", which is accurate but not flagged as simulation.

F8. Builder RSS range. The scoreboard says "tree 3.0-6.0 GB official" for target 4. bench-g has official RED course runs at 8.11 and 8.76 GiB (10:30-11:19), and gram2-g's single-process cold harness reached 7.0 GB at 664 projects (its own text). The range understates the worst official builder.

F9. Churn "official GREEN 7/7, capacity waits 40-178 s". The 40-178 s values are whole run walls from `rows.txt`, not capacity waits (the t5 log does show a "bun waits for machine capacity" line). The 7/7 comes from five unit-law test runs through `repo exec`, which prove the envelope laws, not that a peer edit causes zero build re-executions. Target 3 zero-re-execution is NOT RUN (proof2 rows 2, 4, 6, 8 blocked or pending).

F10. chain-g before/after are one sample each (07:36 vs 07:47). The 1.77 GB to 98 MB Bazel worker figure (07:15) has no record cited (CLAIM). `ps` RSS on macOS is compressed RSS, as chain-g itself notes; gram2-g uses `footprint -p`; the gate's product.py samples tree RSS every 5 s. The metric is not the same across lanes.

F11. derive-g "single process peak 4.06 GB" (scoreboard target 4) is not in `result-derive-g.md`. The file says the after peak is 4.2-4.4 GB, the before peak 4.66-5.0 GB, and the double-move peak 5.8 GB. Its own text says "Peak RSS is 4.2-4.4 GB, not < 2 GB".

F12. watch-g is a synthetic micro-benchmark of the nothing-changed case (50,000 observations, 2,500 scratch dirs). The scoreboard's "56-63 ms vs 218-311 ms" does not say so.

F13. TS ranges. ts-g's "6.9-12.4 s" excludes the 27 s (ambient global) and 25 s (tsconfig) steps of the same 22-step corpus. ts2-g's "0.4-5 CPU-s" is not in the result; its table gives 0.4-1.4 for leaf edits, and the bun column for the walk row is garbled. Both are overlay or in-process harnesses, not the product route.

F14. Official runs never used the 16 GB profile. bench2-g confirms there is no memory-profile hook (`totalmem()` is read directly), and the official floor is 4,096 MB on this 32 GB host. The brief's "16 GB budget simulated via the capacity policy" was not applied to any official run. Every 16 GB number is a simulation or a harness, and the official loads were 17-65 (not quiet).

F15. Scoreboard staleness. It is refresh 7 at 13:44. It still says the batch-3 print verdict is "not in my reads" (resolved: RED 13:33 on a peer's lane2-g retention, GREEN 14:19), does not mention the 3c no-op never admitted, the phase-g GREEN (16:00), the preadmit-g apply at 16:03, or that bench2-g's table is still "not measured yet" at 16:00.

F16. Internal inconsistency in the scoreboard target 4 text: "not for 170-project or 22-project refreshes (355 s, 81 s is within 2 min)". The 81 s refresh is within the 2-minute target, so the text contradicts itself. The 81 s is for 23 published projects per the log.

F17. Missing record folders: `store-g/`, `self-g/`, `nxret-g/`, `tsfix-g`, `churn-g/` (its own note says removed). Their numbers (store-g image sizes, self-g RSS, churn env-digest log) cannot be re-checked.

F18. bench-g's table row labels "course.bazel.noop.*" are graph rebuilds that exit 1, not no-ops. Naming them "noop" invites misreading.

## 5. Best numbers per acceptance target

Brief targets were measured on the 16 GB budget, which no official run did (F14). "Official" below means the raw record was confirmed on disk.

| target | best CONFIRMED official | best graph-free / CLI-partial / simulated | verdict |
|---|---|---|---|
| 1. no-op < 10 s wall, < 1.5 GB tree | none. Hot product no-op attempts: 2714 s (killed in queue, 86 MB, gate 3c); 1614 s (print2, killed). Closest real run: course:build 225.2 s, 147 s of it Bazel actions (noop-g r8, not a no-op). Best official RSS of a cargo no-op: 1,683 MB at 94.5 s (CLI route, not official). | preadmit-g 0.41 s door, sleep body, simulated 16 GB (SIMULATION). chain-g `cache status` 580 MB, 16.6 s (CLI-partial, not a build). | NOT MET. No official no-op number exists. |
| 2. one-line edit: TS < 60 s, dashboard Rust < 2 min | none confirmed. Official dashboard build after a 1-project graph delta: 1154 s wall incl. queue, Bazel 208 s (not a one-line edit). Official integ c3d dashboard build 305 s (cargo, 2 actions). | Rust edit 44.0 s / 1,341 MB (`bun repo cargo`, load 36) vs 61.6 s / 3,659 MB baseline (load 18). TS warm edit 6.9-12.4 s course (ts-g overlay, HARNESS). | Rust met on CLI route only. TS met in harness only. No official one-line proof. |
| 3. peer edits: 0 re-executions, no failed run | official: 7/7 churn-envelope laws green over five `repo exec` runs, 297 s, 2.2 GB (proof2-g row 1). Failure side: gate batch-3 print run RED on a peer's mid-run edit (850 s), GREEN on rerun (1090 s). | churn fuzz 200/200 consistent, sync, 200 seeds (HARNESS; mean wall 0.70 to 1.33 s). Scheduling edits re-key at most 1,171 of 12,817 Nx targets (static bound, SIMULATION). | Zero re-execution for a scheduling-code peer edit: NOT RUN. |
| 4. graph generation after a peer edit: < 2 min, builder < 2 GB | time: official 34 s (1 project, xplat-g), 33 s (1 project, integ c2), 81 s (23 projects, integ-g), 355 s (170 projects, zt-g). Memory: official best 3,037 MB tree (1 project). Official worst: 8.76 GiB builder RED (bench-g). | gram2-g staged single-process 2.87 GB RSS / 1.97 GB footprint (HARNESS, sealed gen 560). gram-g index patch 0.34 s, +176 MB (HARNESS). derive-g env move 43-52 s, 4.2-4.4 GB (HARNESS). cutoff-g 121 s, 2.08 GB (HARNESS). | time MET for small deltas. Builder RSS NOT MET (official 3.0 GB tree, RED 8.8 GiB). |
| 5. three agents on 16 GB: no swap storm, fair queue, progress | none. No official 3-run measurement. Official live: capacity waits 356 s (zt-g) and queue of 2750 s (phase-g). | ram-g 59.3 h to 25.8 h wait (SIMULATION). lease-g interactive mean 94 to 25 min (SIMULATION replay). bzsrv-g 133 s to 87 s (synthetic 20 s holds, simulated 2.5 GB headroom, HARNESS). fix-g overshoot 8.1 to 0.9 GB (SIMULATION). | NOT MET officially. Model results only. |
| 6. disk: `.cache` under declared budget, lock-aware GC | measured census of the real store (no workload): content 94.0 to 69.8 GiB, declared budget 40 GiB, fixed images 97.6 GiB. | store-g sparse images (MICRO on scratch images, record folder missing). | NOT MET (69.8 GiB vs 40 GiB). |

## 6. Corrections to the scoreboard (for the coordinator)

1. Target 1: replace "225.2 s hot course:build" with "not a no-op; 147 s Bazel actions" and state that the like-for-like print-rs:test pair is 618.0 s to 653.0 s.
2. Target 1: "Cargo no-op 94.5 s / 1.68 GB, load 35" should read "load 23.5"; state that the after no-op is slower in wall than before (36.9 s, 96.8 s) and only RSS fell.
3. preadmit-g: move from "MEASURED" to "SIMULATION: sleep bodies, simulated 16 GB host".
4. nxperf-g: "at most 1,171 of 12,817" is an upper bound, not a count.
5. fix-g "8.1 to 0.9 GB" belongs in the simulated column.
6. Target 4: remove "4.06 GB" (not in the result) and use 4.2-4.4 GB; widen the official builder RSS range to include bench-g's 8.1-8.8 GiB RED runs; fix the "81 s is within 2 min" contradiction.
7. Target 3: "capacity waits 40-178 s" are run walls; the 7/7 is unit-law tests, not re-execution evidence.
8. Target 2: keep "Rust met on the cargo route" separate from the product route; "TS met" is harness only.
9. Refresh the scoreboard: batch 3b GREEN 14:19, batch 3c never admitted, batch 4 phase-g GREEN 16:00, batch 4b preadmit-g applied 16:03.
10. State the 16 GB caveat (F14) in the target table header.

## 7. Cleanup

Tools kept (inputs): `aud-claims2-g-tools/cited.py`, `aud-claims2-g-tools/rows.py`. My intermediate outputs under `🗑️generated/aud-claims2-g/` were removed at the end of this audit.
