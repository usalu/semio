# Audit: graph rebuild attribution, wave 1026g (aud-rebuild-g)

READ-ONLY audit. No source file, git state or process was touched. Scripts and raw outputs: `🗑️generated/aud-rebuild-g/` (`extract.py`, `walk-changed.py`, `attribute.py`, `summarize.py`, `gens.tsv`, `changed-files.tsv`, `per-generation.tsv`, `top-dirs.tsv`). Every script streams and was capped at 500 MB RSS (peak measured 106 MB).

## Scope and sources

- Window: graph generations published since 10-25 18:00 (local) up to the live pointer, generation 542 (08:11), plus the running builder 809b1e82251124b161882dfe (base 542, started 10-26 09:02:07, state running at audit time).
- Generation store `.cache/b/g/m`: `generations/current.json` (pointer = 542), manifests `generations/m/*.json` exist only for 539, 540, 541, 542 (older manifests are gone), build records `generations/c/*.json` (only 542 to 543 attempts survive).
- For generations 465 to 537 the publish line of the builder logs (`graph: generation N published, X projects refreshed in S s`) in `🗑️generated/**` is the only source (streamed with `rg --no-ignore`, 545 publish lines in 524 files, 0 conflicts after dedup). Publish time = earliest mtime of a log that holds the line (an upper bound; log lines carry no clock time).
- Generation 472 time and manifest from `result-graph-f.md` (manifest 20:57:31). Generation 538 has no count (chain only, between 07:30 and 07:35).
- Two publish lines (538 and 541 at 08:28 in `semio_tech_repo_dashboard_rs_test-g6.raw` and `e2e-course-e2e-1025f-zd2`) disagree with the live manifests (541 = 266 refreshed, 647 in the log) and were excluded: they come from another store with the same generation numbers.
- Changed files: a metadata walk of the working tree (no content read, no git) listed 1,557 entries (861 files and 696 directories) with mtime after 10-25 17:30. Excluded: `.git`, `.cache`, `node_modules`, `target`, `🗑️generated`, `.🧬semio`. Deleted files are not visible this way.

## Method

- Real re-inferred projects = `refreshed - 266`. Every delta build re-runs the 266 test and policy owners (`replaceTags: ["tests","policy"]`, cutoff-g result). Manifests confirm it: manifests 541 and 542 record 266 refreshed projects, i.e. 0 real re-inferences.
- Window of generation G = changed paths with mtime in `(obs(prev), obs(G)]`, where `obs(G) = publish - build wall s` (facts are observed at the start of a build).
- Cause class: `build-library` = `📚️library/**`, `🏃️process/**`, `📁️filesystem/**`, any `🏗️build/`, root build manifests; `repo-product` = other `🦑️repo/**`; `other-product` = everything else. "Outside the fleet" = path whose last two components are named in no ticket markdown (a heuristic, authorship is not provable from mtime).
- Stale-fact kinds exist only in a few logs (below). They were not available per generation.

## Per-generation table

| gen | published (local) | source | build wall s | refreshed | real re-inferred | window paths: build-lib / repo / other | cause class |
|---|---|---|---|---|---|---|---|
| 464 | 10-25 17:58 | c9.b1.0.log | 186 | 274 | 8 | 0 / 0 / 1 (of 1) | baseline only (tests and policy) or 1-19 projects |
| 465 | 10-25 18:24 | x2.b1.0.log | 188 | 267 | 1 | 0 / 0 / 11 (of 11) | baseline only (tests and policy) or 1-19 projects |
| 466 | 10-25 18:30 | x3.b1.0.log | 173 | 267 | 1 | 0 / 0 / 2 (of 2) | baseline only (tests and policy) or 1-19 projects |
| 467 | 10-25 19:24 | p1.b1.0.log | 189 | 267 | 1 | 26 / 0 / 34 (of 60) | baseline only (tests and policy) or 1-19 projects |
| 468 | 10-25 19:25 | semio_tech_presentation_test-f1.raw | 211 | 266 | 0 | 1 / 0 / 0 (of 1) | baseline only (tests and policy) or 1-19 projects |
| 469 | 10-25 19:53 | p1-chunk.aaa.log | 328 | 809 | 543 | 93 / 2 / 80 (of 175) | build-library edit in window |
| 471 | 10-25 19:52 | e2e-course-e2e-1025f-2-course-test-e2e.log | 1066 | 382 | 116 | 0 / 0 / 0 (of 0) | no file edit in window (unexplained) |
| 472 | 10-25 20:57 | result-graph-f (manifest 20:57:31) | ? | ? | ? | 128 / 15 / 119 (of 262) | not derivable (no count) |
| 473 | 10-25 23:05 | d-build.b1-run3-enoent-read-mjs.log | 907 | 8038 | 7772 | 113 / 12 / 401 (of 526) | build-library edit in window |
| 474 | 10-25 23:05 | d-build.b1-run3-enoent-read-mjs.log | 599 | 331 | 65 | 8 / 0 / 0 (of 8) | build-library edit in window |
| 475 | 10-25 23:11 | p3.b2.0.log | 241 | 295 | 29 | 23 / 0 / 1 (of 24) | build-library edit in window |
| 476 | 10-25 23:14 | bzuitc.log | 180 | 270 | 4 | 2 / 0 / 0 (of 2) | baseline only (tests and policy) or 1-19 projects |
| 477 | 10-26 01:12 | d-build.b1.230547.log | 465 | 331 | 65 | 19 / 1 / 8 (of 28) | build-library edit in window |
| 478 | 10-26 01:14 | a1.log | 102 | 293 | 27 | 0 / 0 / 0 (of 0) | no file edit in window (unexplained) |
| 479 | 10-26 01:28 | p4.b1.0.log | 271 | 781 | 515 | 0 / 0 / 0 (of 0) | no file edit in window (unexplained) |
| 480 | 10-26 02:20 | e2e-course-e2e-1025f-e1-course-test-e2e.log | 311 | 285 | 19 | 0 / 1 / 5 (of 6) | baseline only (tests and policy) or 1-19 projects |
| 481 | 10-26 01:28 | run.log | 106 | 272 | 6 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 482 | 10-26 01:30 | semio_tech_presentation_react_test-f2.raw | 115 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 483 | 10-26 01:29 | k3.a1.log | 116 | 265 | 0 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 484 | 10-26 01:46 | d-tests.b1.011235.log | 486 | 331 | 65 | 0 / 0 / 0 (of 0) | no file edit in window (unexplained) |
| 485 | 10-26 02:19 | p25.b1.log | 260 | 809 | 543 | 0 / 1 / 1 (of 2) | product edit only |
| 486 | 10-26 01:47 | run.log | 132 | 271 | 5 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 487 | 10-26 01:49 | b-_semio-tech_repo-lib_test-package-body-policy.log | 154 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 488 | 10-26 01:56 | d-heavy.b1.014643.log | 107 | 293 | 27 | 0 / 0 / 0 (of 0) | no file edit in window (unexplained) |
| 489 | 10-26 02:05 | m3.a1.log | 93 | 295 | 29 | 0 / 0 / 0 (of 0) | no file edit in window (unexplained) |
| 490 | 10-26 01:57 | b-_semio-tech_repo-test_test-discovery-boundaries.log | 244 | 759 | 493 | 0 / 0 / 0 (of 0) | no file edit in window (unexplained) |
| 491 | 10-26 02:15 | semio_tech_print_rs_test-f2.raw | 161 | 267 | 1 | 0 / 1 / 1 (of 2) | baseline only (tests and policy) or 1-19 projects |
| 492 | 10-26 02:08 | b-_semio-tech_repo-lib_test-dependency-direction.log | 104 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 493 | 10-26 02:17 | m4.a1.log | 96 | 267 | 1 | 0 / 0 / 1 (of 1) | baseline only (tests and policy) or 1-19 projects |
| 494 | 10-26 02:25 | teaching_architecture_course_typecheck-f2.raw | 125 | 270 | 4 | 1 / 0 / 25 (of 26) | baseline only (tests and policy) or 1-19 projects |
| 495 | 10-26 03:07 | p25.c1.log | 106 | 270 | 4 | 0 / 0 / 19 (of 19) | baseline only (tests and policy) or 1-19 projects |
| 496 | 10-26 02:37 | dev-site.log | 103 | 276 | 10 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 497 | 10-26 02:48 | semio_tech_repo_dashboard_rs_build-f2.raw | 102 | 331 | 65 | 0 / 0 / 8 (of 8) | product edit only |
| 498 | 10-26 02:47 | m1c.a1.log | 86 | 292 | 26 | 0 / 0 / 0 (of 0) | no file edit in window (unexplained) |
| 499 | 10-26 02:53 | dev-site2.log | 93 | 266 | 0 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 500 | 10-26 03:25 | semio_tech_print_test-f2.raw | 95 | 267 | 1 | 0 / 0 / 2 (of 2) | baseline only (tests and policy) or 1-19 projects |
| 501 | 10-26 02:52 | c-runtimeboot.log | 90 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 502 | 10-26 03:09 | c-cache-red-chunk.aaa.log | 111 | 267 | 1 | 0 / 0 / 2 (of 2) | baseline only (tests and policy) or 1-19 projects |
| 503 | 10-26 03:03 | e2e-course-1025f-f1-1-test-e2e.log | 128 | 266 | 0 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 504 | 10-26 03:15 | c-cache-red-chunk.aab.log | 124 | 267 | 1 | 0 / 0 / 2 (of 2) | baseline only (tests and policy) or 1-19 projects |
| 505 | 10-26 03:25 | c-cache-red-chunk.aac.log | 130 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 506 | 10-26 03:31 | c-cache-red-chunk.aad.log | 105 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 507 | 10-26 03:42 | semio_tech_mit_bestand_bericht_build-f2.raw | 110 | 266 | 0 | 0 / 0 / 4 (of 4) | baseline only (tests and policy) or 1-19 projects |
| 508 | 10-26 03:53 | e2e-course-1025f-f3-1-test-e2e.log | 128 | 266 | 0 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 509 | 10-26 03:38 | c-cache-rest-chunk.aaa.log | 135 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 510 | 10-26 03:45 | c-cache-rest-chunk.aab.log | 114 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 511 | 10-26 03:54 | semio_tech_print_build-f2.raw | 119 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 512 | 10-26 03:50 | c-cache-rest-chunk.aac.log | 119 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 513 | 10-26 03:55 | c-cache-rest-chunk.aad.log | 100 | 268 | 2 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 514 | 10-26 04:00 | c-cache-rest-chunk.aae.log | 101 | 267 | 1 | 0 / 1 / 0 (of 1) | baseline only (tests and policy) or 1-19 projects |
| 515 | 10-26 04:06 | e2e-course-1025f-f4-1-test-e2e.log | 146 | 266 | 0 | 0 / 0 / 4 (of 4) | baseline only (tests and policy) or 1-19 projects |
| 516 | 10-26 04:09 | c-cache-rest-chunk.aaf.log | 129 | 267 | 1 | 19 / 0 / 1 (of 20) | baseline only (tests and policy) or 1-19 projects |
| 517 | 10-26 06:14 | semio_tech_print_test-g2.raw | 4800 | 808 | 542 | 16 / 0 / 1 (of 17) | build-library edit in window |
| 518 | 10-26 05:47 | e2e-course-1025f-g1-1-test-e2e.log | 843 | 381 | 115 | 0 / 0 / 0 (of 0) | no file edit in window (unexplained) |
| 519 | 10-26 05:48 | e-runtimeboot.log | 261 | 805 | 539 | 0 / 0 / 0 (of 0) | no file edit in window (unexplained) |
| 520 | 10-26 05:55 | e2e-course-1025f-g2-1-test-e2e.log | 115 | 284 | 18 | 0 / 0 / 2 (of 2) | baseline only (tests and policy) or 1-19 projects |
| 521 | 10-26 06:01 | e-cache-rest-chunk.aag.log | 114 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 522 | 10-26 06:06 | e-cache-rest-chunk.aah.log | 109 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 523 | 10-26 06:11 | e-cache-rest-chunk.aai.log | 115 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 524 | 10-26 06:15 | e-cache-rest-chunk.aaj.log | 124 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 525 | 10-26 06:53 | semio_tech_print_test-g3.raw | 106 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 526 | 10-26 06:21 | e-cache-rest-chunk.aak.log | 121 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 527 | 10-26 06:25 | e-cache-rest-chunk.aal.log | 106 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 528 | 10-26 06:40 | e-cache-red-chunk.aaa.log | 111 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 529 | 10-26 06:45 | e-cache-red-chunk.aab.log | 110 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 530 | 10-26 06:56 | e-cache-red-chunk.aac.log | 109 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 531 | 10-26 07:04 | semio_tech_print_test-g4.raw | 104 | 267 | 1 | 0 / 0 / 1 (of 1) | baseline only (tests and policy) or 1-19 projects |
| 532 | 10-26 07:04 | e-cache-red-chunk.aad.log | 130 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 533 | 10-26 07:20 | semio_tech_print_test-g5.raw | 125 | 267 | 1 | 3 / 0 / 1 (of 4) | baseline only (tests and policy) or 1-19 projects |
| 534 | 10-26 07:15 | e2e-course-1025f-q2-1-test-e2e.log | 133 | 266 | 0 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 535 | 10-26 07:16 | g-runtimeboot.log | 110 | 267 | 1 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 536 | 10-26 07:24 | e2e-course-1025f-q3-1-test-e2e.log | 138 | 266 | 0 | 2 / 0 / 2 (of 4) | baseline only (tests and policy) or 1-19 projects |
| 537 | 10-26 07:30 | i-runtimeboot.log | 465 | 268 | 2 | 0 / 0 / 0 (of 0) | baseline only (tests and policy) or 1-19 projects |
| 538 | 10-26 07:33 | chain only (base of 539) | ? | ? | ? | 2 / 0 / 4 (of 6) | not derivable (no count) |
| 539 | 10-26 07:35 | manifest 539 | ? | 268 | 2 | 5 / 0 / 0 (of 5) | baseline only (tests and policy) or 1-19 projects |
| 540 | 10-26 08:02 | manifest 540 | ? | 381 | 115 | 152 / 6 / 1 (of 159) | build-library edit in window |
| 541 | 10-26 08:08 | manifest 541 | ? | 266 | 0 | 20 / 0 / 0 (of 20) | baseline only (tests and policy) or 1-19 projects |
| 542 | 10-26 08:11 | manifest 542 | ? | 266 | 0 | 6 / 0 / 0 (of 6) | baseline only (tests and policy) or 1-19 projects |

Legend: "refreshed" = projects the generation refreshed (test and policy baseline included). "real re-inferred" = refreshed minus 266. "window paths" = changed paths in the observation window, split build-library / repo product / other product.

## Stale facts recorded in the logs (only these builds print the kinds)

- crate-f `b-r1.log` (graph-f, 00:03-00:27 10-26, graph-f result: generation 477 not yet published at 00:27): 722 stale facts (analysis 446, file 151, exists 66, readdir 47, resolve 8, lstat 2, stat 2) leave 567 projects stale.
- nx4 `in1.log` (10-22, 3,146 projects) and `pt1.log` (544), dtest `o8.log` (575), e2e course logs (37 to 119): older than the window, listed for reference only.

## Totals (window)

- Generations listed: 78 (76 with a refresh count, 472 and 538 without).
- Refreshed projects summed: 32,024. Of which baseline (test and policy, 266 per build): about 20,200 (63 %). Real re-inferred: 11,809.
- Real re-inferred by class (generations with at least 20 real): build-library edit in window 9,131 projects (7 generations); product edit only 608 (2); no file edit in window 1,952 (10, unexplained, see below).
- Builds at 10 or more real re-inferred: 10 generations (469, 471, 473, 479, 485, 490, 517, 518, 519, 540). 473 refreshed 8,038 projects, more than the 4,019 owners of the store (double counting or a whole-store invalidation; not resolved).
- Generation 517: 4,800 s wall (80 min) for 542 projects. Generation 471: 1,066 s for 116.
- Wall time of the builds with a logged duration: 18,539 s = 5.15 h.
- Builder CPU-hours (estimate, no CPU is logged): lower bound 32,024 projects x 0.4 cpu-s (infer-f measured per project in a fresh worker) = 3.6 CPU-h; upper bound = all wall time on one core = 5.2 CPU-h. The parent's serial fallback ran at about 4.5 s per project (infer-f), so the true value sits between these bounds, nearer the upper one for serial builds.
- Changed paths since 10-25 17:30: 1,557, of which 1,198 are named in a lane note and 359 (239 files, 120 directories) are named nowhere (the largest unnamed groups are runtime captures and leases under `🧰️framework/🤖️generated/process/{captures,leases}`, 53 entries, written by builds and runs themselves).

## Top 20 directories by attributed re-inference (heuristic)

Attribution: a build's real re-inference is split equally over the build-library directories changed in its window; if none changed, over all changed directories in the window. Scores are shares of a window, not proofs.

1. 278 | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack`
2. 277 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🌀️control-plane`
3. 132 | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🧪️tests`
4. 106 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests`
5. 106 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript`
6. 100 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧷️pin/🧬️schema`
7. 99 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/♻️scoped`
8. 99 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/♻️scoped/🔄️lifetime`
9. 99 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript`
10. 98 | `♻️mit-bestand`
11. 98 | `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript`
12. 98 | `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript`
13. 98 | `✏️s`
14. 98 | `🌎️hub`
15. 98 | `🎓️teaching`
16. 98 | `🏢️semio-tech`
17. 98 | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage`
18. 98 | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform`
19. 98 | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed`
20. 98 | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧷️live-children`

## Attribution split

- Build-library edits (`📚️library`, `🏃️process`, `📁️filesystem`, `🏗️build`): dominant in 7 generations, about 9,100 real re-inferred projects (about 77 % of the real total). Mostly `🕰️generations/**`, `🏗️build/🕸️graph/**`, `🏃️process/📦️artifacts/🏗️native-build/💾️storage/**`, `🏗️build/📥️inputs/**`. Each such edit stales every project whose command closure holds the walked file (infer-f: 3,739 of 4,020 owners).
- Product edits only: 608 real (2 generations: 485 with 543 real, whose window holds the `jack` artifact lockfile and the dashboard `control-plane` tests; 497 with 65 real, 8 other-product files).
- No file edit in the window: 1,952 real in 10 generations (471, 478, 479, 484, 488, 489, 490, 498, 518, 519). These cannot be explained by mtimes. Candidates: a whole-implementation digest change (`implementation` differs between manifests 539/540/542), deleted entries (a removed file is invisible to a file walk), or a log from another store. Not verified.
- Other agents outside this fleet: at most 359 of 1,557 changed paths are named in no lane note; their share of re-inference cannot be measured from mtimes.

## Recommendations (one line per top cause, owner lane)

1. Test and policy baseline (266 projects in all 76 builds, about 20,200 projects, about 63 % of refresh work): keep the tests and policy discovery behind the fact index instead of replacing the tags each build. Owner: cutoff-g (graph owner per its result).
2. Build-library rewrites of shared walks (`🕰️generations/**`, `🏗️build/🕸️graph/**`, `🏃️process/…/💾️storage/**`): derived-fact cutoff is implemented in cutoff-g; batch facade and import rewrites per wave so one commit does not stale about 3,700 owners. Owner: cutoff-g.
3. Runtime state inside the source tree (`🧰️framework/🤖️generated/process/{captures,leases}`, 53 entries in the window): if a graph observation walks these directories, every build writes its own invalidation. Exclude generated runtime directories from observation walks (or verify they are not observed). Owner: keys-g.
4. 1,952 re-inferred projects with no file edit in the window (471, 479, 490, 518, 519 carry 1,800 of them): find which implementation or policy digest moved for these generations (compare the `implementation` field of the manifests). Owner: self-g.
5. Product edits (dashboard tests, `jack` artifacts): 608 projects, small. Keep per-project keys; no action beyond keys-g hygiene. Owner: exec.
6. Long builds (generation 517 at 80 min, 542 projects; builder 809b running since 09:02): confirm the infer-f farm (worker processes inside the grant) is applied in the tree, then measure seconds per project. Owner: exec.

## Caveats

- Generations 465 to 537 are not verifiable against the live store (only 539 to 542 manifests survive). Numbers come from log lines; collisions with other stores were found and excluded only where they conflicted with the live chain.
- Mtime is the working-tree state, not commits. Deleted and renamed entries are missing except as directory mtimes.
- CPU-hours are estimates, not measurements.
