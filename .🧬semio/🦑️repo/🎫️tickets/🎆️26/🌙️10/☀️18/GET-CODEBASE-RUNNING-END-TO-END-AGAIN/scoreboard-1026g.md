# Scoreboard wave 1026g (build system for many agents on 16 GB)

Read-only auditor status-g. Refresh every ~40 min for ~4 h, newest refresh at the top.

## Refresh 7 (final): 2026-10-26 13:44 CET (refresh 6 at 13:04; this closes the ~4 h window from 09:45)

Clock note: coordinator stamps run about 1.5 h ahead of the wall clock. Lane "last update" is the file mtime; self-stamps are claims.

Method rule: "measured" only if a result file (or a coordination entry quoting one) states the number and its method. "Graph-free" = offline harness, scratch copy or fixture. "Official" = real `bun repo` route with the graph. "Simulated" = model or replay. "Claim" = stated in a lane entry, method not checked by me.

## 0. Current state in one paragraph

Products are GREEN on official runs: dashboard, course and print (repair-g, 12:21); course build after batch 1 (gate-g, 12:45, 1092 s incl. a 356 s capacity wait, 6.0 GB tree); dashboard build after batch 2 (gate-g, 13:12, 1154 s, 3.0 GB tree, one project re-inferred in 34 s); course:dev and dashboard run (integ-g, 13:25, the dev-route regression is gone). The churn envelope proof is GREEN officially (proof2-g row 1, 13:35, 7/7, 2.2 GB). Batch 3 (noop2-g + watch-g) ran print-rs:test, which ended 13:33; its verdict is not in my reads. No acceptance target has an official measurement that meets its number yet: no-op, one-line edit rebuild, peer-edit zero re-exec, builder RSS and disk are still open.

## 1. Acceptance targets

| # | target | best MEASURED value so far (source, method) | owning lanes | status | what is left |
|---|---|---|---|---|---|
| 1 | no-op `bun repo run <product>:build`: < 10 s wall, < 1.5 GB tree RSS | No official no-op measured. Earlier best: 225.2 s hot `course:build` total (noop-g r8; 147 s Bazel actions re-executed). Cargo no-op 94.5 s / 1.68 GB (rust-g `prof-noop`, load 35). Official: batch 3 (noop2-g planner skip, receipt replay once, capture ledger) applied via gate-g 13:12; its no-op effect is not measured. Graph-free: `cache status` tree 580 MB, queue waiter 104 MB (chain-g). preadmit-g (13:45, MEASURED, staged): pre-admission for no-op, measured in its overlay. | noop-g, noop2-g, chain-g, self-g, bzsrv-g, rust-g, keys-g, phase-g, gate-g, bench2-g, preadmit-g | NOT MET | bench2-g baseline (harness ready 13:05) and preadmit-g apply (batch 4 or later). |
| 2 | one-line edit rebuild < 60 s TS, < 2 min dashboard Rust | TS (graph-free): warm `typecheck` one edit 6.9-12.4 s course, 6-16 s ui-react; cold 23-30 s (ts-g). ts2-g (graph-free): warm worker 0.4-5 CPU-s vs cold 41 CPU-s / 3.9 GB. Rust (rust-g, real `bun repo cargo`): BEFORE 61.6 s / 3.66 GB, AFTER 44.0 s / 1.34 GB (load 36). lane-g and lane2-g (graph-free fixtures): concurrent artifact consumers, pooled lanes ~1.8x faster. Official builds listed in section 3 are not one-line edits. | ts-g, ts2-g, rust-g, lane-g, lane2-g, keys-g, keys2-g, chain-g | TS: partly. Rust: met on the cargo route, partly on the `run` route | Official one-line edit proofs (proof-g rows 3, 5, 9) not run. |
| 3 | peer edits (incl. build-library scheduling code) during/between runs: 0 re-executions, no failed run | Official, GREEN: churn envelope (proof2-g row 1, 13:35): 7/7 laws, 5 official execs in 297 s, capacity waits 40-178 s, 2.2 GB tree, generation 553 served. Graph-free: scheduling edits re-key at most 1,171 of 12,817 Nx targets (nxperf-g); churn fuzz 200/200 consistent; watch-g journal 56-63 ms vs stat walk 218-311 ms (applied in batch 3). Official, RED earlier (before the fix): proof-g row 1, 7 attempts x 6 restarts, exit 75. | churn-g, nxperf-g, keys-g, keys2-g, hold-g, self-g, fix-g, watch-g, repair-g, proof2-g | partly | The zero-re-execution proof for a scheduling-code peer edit (proof-g rows 2, 4, 6, 8) is not run. |
| 4 | graph generation after a typical peer edit: < 2 min, builder < 2 GB RSS | OFFICIAL, 13:12 (gate-g batch 2, xplat-g, `product.py`): generation 552, ONE project re-inferred in 34 s, dashboard build 208 s in Bazel, 1154 s wall incl. queue; tree peak 3.0 GB, 7 processes (builder not isolated). OFFICIAL, 12:45 (batch 1, zt-g): generation 549, 170 projects re-inferred, 355 s refresh, tree 6.0 GB. OFFICIAL, 13:25 (integ-g, dev route): generation 553, 22 projects refreshed in 81 s. OFFICIAL, 12:21 (repair-g): generation 546 sealed 169 projects in 85 s; dashboard 174 s (66 owners healed). GRAPH-FREE: derive-g env-move, 12:40, single process peak 4.06 GB, part:edges 24-25 s; gram-g memory fix (per-chunk abort listeners; fact-index re-partition 2.7 s / +1.47 GB -> 0.34 s / +176 MB, 12:45). | cutoff-g, asm-g, hold-g, gram-g, gram2-g, derive-g, gate-g, integ-g, repair-g | partly: time met for a one-project delta (34 s), not for 170-project or 22-project refreshes (355 s, 81 s is within 2 min); builder RSS NOT MET (tree 3.0-6.0 GB official; 4.06 GB single process in derive-g) | Per-process builder RSS in an official run. derive-g env-move bound (5.5-7 GiB) and gram2-g cold harness pending. |
| 5 | three agents on 16 GB: no swap storm, no starvation, fair queue, progress visible (EN+DE) | Graph-free, real Bazel servers, simulated 2500 MB headroom (bzsrv-g): all done 133 s before, 87 s after; waits 75/107 s -> 18/54 s. Shard-7 replay: interactive mean wait 94 -> 25 min (lease-g). 16 GB three-agent queue model 59.3 h -> 25.8 h (ram-g, simulated). fix-g (claim): burst overshoot 8.1 -> 0.9 GB. Official, live: capacity waits 40-356 s on the wave's official runs (gate-g, proof2-g, integ-g). lane2-g (13:38, graph-free, two concurrent consumers): measured cargo lease vs lane pool. nxret-g (13:42, LIVE + GREEN): bound wired into `repo` (details in its result file). xplat2-g (13:32, LIVE): Windows LockFileEx lease backend. My `ps` at 13:44: `repo mcp` (PID 10247) at 11.3 GB RSS after 18.6 h, not from a wave lane, not killed. | ram-g, fix-g, lease-g, bzsrv-g, lane-g, lane2-g, testmem-g, preadmit-g, nxret-g, phase-g, xplat2-g | partly | No official 3-run measurement with swap and progress visible. The `repo mcp` process has no owner. phase-g not applied (gate-g batch 4). |
| 6 | disk: steady-state `.cache` under a declared budget, lock-aware GC | disk-g (census, graph-free): content in use 94.0 -> 69.8 GiB; loose caches 26.3 -> 4.8 GiB; reclaimed 21.56 + 3.55 + 2.75 GiB; declared default budget 40 GiB; fixed images 97.6 GiB. store-g (11:33, partly staged): sparse macOS images for new images; existing images still preallocated. No new size measured. hyg2-g (13:24): a live lease test artifact directory under the ticket folder exceeds the limit (its own finding). | disk-g, store-g, bzsrv-g, lane-g, hyg2-g | NOT MET (69.8 GiB vs 40 GiB; no post-change size) | store-g recreate run for existing images, then `.cache` size. gate-g batch 7 (store-g) not yet applied. Remove the ticket test-artifact dir (hyg2-g). |

## 2. Lane table (every `result-*-g.md`, as of 13:44)

State from the last status line plus mtime. "finished" = a DONE/FINAL line and no open owner work.

| lane | state | last update (mtime / self-stamp) | landed changes | open items |
|---|---|---|---|---|
| asm-g | finished (claims) | 10:41 / DONE (coordination) | owner-keyed test discovery, lazy fact-index (claims) | official delta run |
| bench-g | finished without baseline | 11:24 / 11:25 DONE (coordination) | suite applied 09:58 | replaced by bench2-g |
| bench2-g | running (waiting) | 13:03 / 13:05 STATUS | harness ready, graph-free | baseline after gate-g batch 3 verdict |
| bzsrv-g | finished | 08:32 / 08:40 FINAL | slots, stable package path, growth 1 per 4096 MB, idle reaper | official run not done |
| chain-g | finished | 09:02 / 09:00 FINAL | loader 1536 -> 255 MB, Bazel worker 1.77 GB -> 98 MB | hot no-op RSS not measured |
| churn-g | finished | 09:35 / 09:40 DONE | race classifier, stable snapshot reads, restart supervisor, fuzz harness | proof2-g row 1 GREEN (13:35) |
| cutoff-g | finished with FAILURE | 09:52 / 09:55 | derived walk facts, TS program kind, retention of 266 test+policy owners | official run 72310 exit 1 (version skew per repair-g); proof-g row 0 RED |
| derive-g | running | 12:45 / 12:40 | graph-free env-move reproduction and baseline | env-move re-edge bound (5.5-7 GiB); test-domain walk as derived fact; closure-facts law red |
| diag1-g | finished (diagnosis) | 12:17 / 12:18 DONE | none (scratch) | root cause fixed by repair-g |
| diag2-g | finished (diagnosis) | 11:45 / 11:45 DONE | none (scratch) | root cause fixed by repair-g |
| disk-g | finished except images | 08:42 / 09:00 DONE | 40 GiB budget, lock-aware GC, COW dedupe; 21.6 GiB reclaimed | images (store-g recreate) |
| docs-g | finished | 09:25 / DONE (coordination) | `build-guide.md` EN+DE | taxonomy verify not run |
| fix-g | finished | 10:07 / DONE (coordination) | rev-g HIGH/MED and rev2-g items fixed with laws | official confirmation |
| gate-g | running | 13:19 / 13:19 | batch 1 zt-g GREEN (course:build 1092 s, 6.0 GB); batch 2 xplat-g GREEN (dashboard build 1154 s, 3.0 GB); batch 3 noop2-g + watch-g: print-rs:test ended 13:33, verdict not in my read | batches 4-8 (phase-g, keys2-g, route2-g, store-g, self-g flip); revert on red |
| gram-g | finished | 12:37 / 12:45 FINAL | abort-listener leak fix, fact-index re-partition rule | official memory measure (gram2-g) |
| gram2-g | running | 13:36 / 12:40 STARTED | none landed in my read | cold full harness with native-memory sampler |
| go-g | finished | 10:32 / 10:39 DONE (coordination) | registry outage root fix, launcher busy retry | proof2-g row 10 |
| hold-g | finished except official | 08:52 / 08:55 | queued claims in chunks <= 128 | proof2-g row 2 |
| hyg2-g | finished | 13:24 / 13:24 DONE (coordination) | 50/72 test files got emoji docstrings; 4/5 duplicate pairs fixed | test artifact dir under ticket exceeds limit |
| integ-g | running, resumed | 13:21 / 13:25 | dev routes GREEN official (course:dev ready 757 ms, no `stale build.graph.released-read`); dashboard run EN/DE boot | full product set (c2) not yet re-run |
| keys-g | measured, oracle done | 09:23 / 08:31 MEASURED | `ownerViews`, Nx scheduling exclusions | proof2-g rows 3 |
| keys2-g | finished (landed part) | 11:29 / 11:29 DONE (coordination) | print:check program read + rule | batch 5 |
| laws-g | finished (13:40 DONE) | 13:40 (coordination) | 19 unrouted law files routed to 11 new suites; 8 double-routed fixed; platform skips | official route |
| lane-g | finished (measured, code landed) | 09:51 / 09:50 MEASURED | compile-atomic shared-cargo lanes, bounded pool | proof2 row 7 |
| lane2-g | finished (graph-free measure) | 13:37 / 13:38 MEASURED | two concurrent artifact consumers on real lease + lane pool | official |
| lease-g | finished | 08:10 / 08:45 follow-ups | progress/ETA, yield exit 75, priority classes | proof2 row 4 |
| noop-g | finished, target not met | 09:19 / 08:50 LIVE | source memo, pin ledger, plan child skip | no-op not demonstrated |
| noop2-g | finished (landed part) | 11:28 / 11:29 DONE | planner skip landed 10:26; receipt replay once; capture ledger; batch 3 applied via gate-g | official no-op via bench2-g |
| nxperf-g | finished | 10:10 / 10:10 FINAL | Nx closure 1,233 -> 746; re-key <= 1,171 of 12,817 | proof2 row 8 |
| nxret-g | finished (LIVE + GREEN) | 13:43 / 13:42 | bound wired into `repo` (compile-atomic patches, tsc strict clean in its files) | none stated |
| phase-g | finished, staged | 11:28 / 11:28 STAGED | host-capacity estimates by phase x class, 54 steps | gate-g batch 4 |
| preadmit-g | finished (staged) | 13:39 / 13:45 MEASURED | pre-admission for no-op, measured in overlay | apply via gate-g after batch 4 |
| proof-g | stopped | 11:35 / 11:35 STOPPED | rows 0 and 1 RED before fixes | superseded by proof2-g |
| proof2-g | running | 13:33 / 13:35 | row 1 GREEN 7/7 official (churn envelope); proof 2 (Rust target inputs) started | rows 2-10 |
| ram-g | finished (own file stale since 07:54) | 07:54 / 07:54 DONE | measured-demand admission, 16 GB floor 2048 MB | fix-g fixed rev-g HIGH |
| repair-g | finished (products GREEN) | 12:21 / 12:21 GREEN | graph code root causes fixed; dashboard, course, print GREEN official | none |
| rev-g | finished | 09:04 / 09:55 DONE | review-1026g.md | handed to fix-g |
| rev2-g | finished | 09:41 / 10:50 DONE | review2-1026g.md | handed to fix-g |
| route-g | finished | 10:16 / 10:16 DONE | closed route table 51 suites | none |
| route2-g | waiting, staged | 11:32 / 11:32 WAITING | 9 test files landed; 26 staged | gate-g batch 6 |
| rust-g | finished | 08:54 / 09:20 FINAL | per-command closure, dashboard edit 61.6 -> 44 s | proof2 row 3 |
| self-g | finished (level 2 behind switch) | 10:11 / 10:20 STATE | sealed snapshot + trampoline; default still `live` | gate-g batch 8 flips to sealed |
| store-g | finished (partly staged) | 11:32 / 11:33 DONE | sparse macOS images for new images; Linux libc; pinned tool paths | gate-g batch 7; recreate run |
| testmem-g | finished | 10:09 / 10:20 LANDED | worker policy from admitted grant | official dashboard `test-unit` run |
| ts-g | finished | 08:58 / 09:25 DONE | vitest workers capped at 4 | proof2 row 5 |
| ts2-g | finished | 10:10 / 10:12 FINAL | persistent Bazel worker for `course:typecheck` | proof2 row 9 |
| undeclared-g, nodeffi-g, e2eperf-g, nxret-g, xplat2-g, aud-ram2-g | launched 12:55 (coordination); result files as listed | 13:24 (undeclared-g), 13:32 (xplat2-g DONE LIVE) | xplat2-g: Windows LockFileEx lease backend, live | the rest: no status line read |
| watch-g | finished (applied in batch 3) | 10:28 / 10:29 DONE | change journal; 56-63 ms vs 218-311 ms | none |
| xplat-g | finished (applied in batch 2) | 10:50 / 10:50 DONE | 12 of 14 audit findings fixed | none |
| zt-g | finished (applied in batch 1) | 11:28 / 12:35 (gate-g) | zero-touch findings closed | none |

Twelve lanes launched 12:55 (coordination): proof2-g, undeclared-g, nodeffi-g, lane2-g, xplat2-g, e2eperf-g, preadmit-g, nxret-g, bench2-g; aud-laws-g, aud-ram2-g, aud-leftover-g. aud-laws-g and aud-leftover-g are DONE; laws-g and hyg2-g followed from them.

## 3. Product guard (official results only)

- 12:21 repair-g: dashboard `build` GREEN (generation 545, 191 s cargo); course `build` GREEN (generation 546); print `test` GREEN (generation 547, 307.7 s).
- 12:45 gate-g batch 1 (zt-g): course `build` GREEN, 1092 s, 6.0 GB tree.
- 13:12 gate-g batch 2 (xplat-g): dashboard `build` GREEN, 1154 s, 3.0 GB tree, generation 552 (1 project, 34 s).
- 13:25 integ-g: `course:dev` GREEN (ready 757 ms, no `stale build.graph.released-read`); dashboard `run` GREEN, EN/DE boot screens.
- 13:33 gate-g batch 3 (noop2-g + watch-g): print-rs:test ended; verdict not in my reads.
- integ-g's full cycle set (dashboard test-unit/journeys/authority, course test/e2e, print check/build/test) has not been re-run as a set since the repair.
- `bun repo leases status | grep -c WAITING` returned 0 in every probe; still unusable as a gate.

## 4. Open risks for the owners

- Batch 3's print-rs:test verdict is not in my reads; batches 4-8 depend on it.
- Churn proof row 1 is GREEN now, but the zero-re-execution proof for scheduling-code peer edits (rows 2, 4, 6, 8) is not run.
- Builder RSS per process is not measured officially. Target 4 is not met on the RSS side.
- Unexplained `repo mcp` process at 11.3 GB RSS (18.6 h, PID 10247).
- No official no-op or one-line edit numbers yet; bench2-g and preadmit-g are the next measurements.
- The ticket folder holds a live lease test artifact directory over its size limit (hyg2-g finding); remove before ticket close.
