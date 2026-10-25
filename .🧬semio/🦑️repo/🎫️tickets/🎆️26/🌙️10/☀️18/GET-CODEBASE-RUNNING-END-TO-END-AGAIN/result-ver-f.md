# result-ver-f (wave 1025f, official verifications of finished lanes, one at a time)

BLOCKED 00:05 (10/26) on the graph builder custody: holder pid 21103 (lstart Sun Oct 25 22:54:43, `--tool=bazel run`, rss 5.6 GB, 51 % cpu) holds `repository-graph-facts:<root>:shard:7` + `build-graph-state:...:selected:7` exclusive for 3028 s (50 min); 7 waiters (77599 83801 36235 36493 45565 50353 and my 71581 at position 8 of 8; machine admission -1489 MB spare, 8 granted). Queue time of a1 so far 42 min (capacity gate) + graph queue. Holder identity: graph child 21103 (ppid 20920, wrapper 20919 = crate-f's `bun repo --tool=bazel run-many --projects=@semio-tech/repo-yaml-rs,...repo-*-rs`, sh 20818, started 22:54:33), 77 min at ~100 % cpu at 00:13. I do not kill foreign processes; coordinator decides on 21103.

Tools `C/ver-f-tools/run.sh <tag> <repo args>`; logs + `verdicts.tsv` under `C/🗑️generated/ver-f/`.

## Status (newest first)
- 00:05 Coordinator added item 6 (ext-f official proofs: `ui-react:test` react-package-export, `framework-compiler-rs:test-native`, `semio-framework-pixels:test` last, report only). a1 passed the capacity gate at ~00:00 after 37 min, now at graph queue position 8 of 8.
- 23:27 start. Item 5 done statically: generation-retirement test now takes its image parent from `testScratchRootV1("generation-retirement")` (per-run sandbox) instead of the real diagnostics partition `.cache/b/o/m/rt`; ledger row `native-image-unsandboxed` removed; the concurrency law (sandbox `⚖️law`) shows neither fresh nor stale for it (it reports only peer churn: a go root-selection test move, `vite` real-graph-image x2 vs ledger 1, revalidation rename). Item 1 launched: run `a1` (`run-many framework-process test-testing,test-native-storage`, pid 44381, lstart 23:22:46) waiting in the capacity gate (6 running, 3 ahead, load 22).

## Verdict table
| # | item | run | verdict |
|---|---|---|---|
| 1 | framework-process test-testing, test-native-storage | a1 | queued |
| 2 | knot-f chain2 | - | pending |
| 3 | repo:test-runtime-bootstrap | - | pending |
| 4 | cache-f suites | - | pending |
| 6 | ext-f proofs (ui-react:test boundary, compiler-rs:test-native, pixels:test last) | - | pending |
| 5 | generation-retirement test in sandbox | static + law | law clean for the row; official run of the generation test pending |

## Changed files
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📦️generation/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧪️tests/🧫️fixtures/📒️ledger.json`
