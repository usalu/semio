# 🩺️ Audit H8 (Haiku) Pulse and Landing Check, Wave 1122

Read-only audit. Clock 15:27 to 15:30 CET, 2026-11-22. Scope: landings in `T/landings.md` from about 15:00 CET ("Wave 1122 E55 to E60", E57 addendum). E50, E52 and E53 (14:00 to 14:35) are outside the window and not checked. No product edits, no git, no processes touched.

## 1. Landings: after-hash check (live file found by sha256 prefix in the framework tree)

| Landing | File | Claimed after | Result |
|---|---|---|---|
| E57 | native-build birth `🟦️.ts` | acfafef0a861 | match |
| E57 | native-build birth schema | 09f3ac210c41 | match |
| E57 | native-build storage `🟦️.ts` | 7634e21fb9b7 | match |
| E57 | nx funding `🟦️.ts` | 1c562070986c | match |
| E57 | nx graph `🟦️.ts` | 62b31d0cc0f6 | match |
| E57 | funded test | 0f941657b378 | match |
| E57 | native-caller test | fdb4079e7a20 | match |
| E57 | native-caller fixture | 1fb278767068 | match |
| E57 addendum | heap census `footprint/🟨️.mjs` | 0427690d9541 | match |
| E58 | workspaces bun `cjs` / `ts` | 49fb76a3 / d9569aa9 | match |
| E58 | generation `🟦️.ts` / schema | 19a1afe8 / b78e3df6 | match |
| E58 | generation storage | 3e782003 (r1) | superseded by r2 694632ef, which is live: match, explained |
| E58 r2 | cache-gc command | 721e034b | match |
| E58 r2 | doctor rules `🔣️.json` | a9305f82 | MISMATCH: live aac77cb8c004, mtime 15:17:26, no later landing entry |
| E55 | process-tree schema / `🟦️.ts` | 320b8b656d79 / 3a0ee6386000 | match |
| E55 | ready `🟦️.ts` | 9cf5973b2f4b | MISMATCH: live b702c9f2b887, mtime 15:27:54, no landing entry |
| E55 (r2, 15:22) | exchange client / schema | 6a7d74b6a676 / 956d24ff1a14 | match |
| E56 | ready `🟦️.ts` | 34263971 | MISMATCH (same file as above; E56 "before" e9127e86 does not equal E55's after 9cf5973b2f4b, so one edit between 15:10 and 15:15 is unrecorded) |
| E56 / r3 | ready schema | eb8aab16 / 79ad4d30a8fd | MISMATCH: live f6853072b5d6, mtime 15:25:38, no landing entry |
| E56 | free-space `🟦️.ts` | ec14bf73 | match |
| E60 p1 | observation `🟨️.mjs` / publisher `🟦️.ts` | 6d3c5f01 / c0e01aa1 | match |

Entries with no hash in the log (E56 r2, E60 p2 and p3, E58 r2 cache-gc windows, E57 test files) were checked by existence only.

## 2. Landings: test re-runs (`repo test-file --json`, one at a time, load 37.9 < 40)

| File | Claimed | Live |
|---|---|---|
| ready `🪢️code-stop` | 4/0 | 4/0 |
| ready `🕰️stale-stage` | 14/0 | 14/0 |
| ready `🧮️early-admission` | 4/0 | 4/0 |
| graph-state `🚦️confirm` | 1/0 | 1/0 |
| observation `🪦️producer-publications` | 4/0 | 4/0 |
| ledger `🙈️ignored` | 2/0 | 2/0 |
| generation `🧭️scoping` | 9/0 | 9/0 |
| dependencies `🧹️tree-removal` | 7/0 | 7/0 |
| cache-gc `🥀️strays` | 5/0 | 5/0 |

All nine: rc 0, all passed. Not re-run (time): ready main 56 to 57/0, native-caller 16/1, E58 collection 4/0, E60 doctor probes.

Verdict: 9 of 9 claims reproduce. Four live files have no landing record for their current content (see risks).

## 3. Pulse (15:27 to 15:30 CET)

- Load 1 min: 40.7 (15:27), 40.1 (15:28), 37.9 (15:29); 5 min 33.6, 15 min 29.8. Host 10 cores. Memory 32 GB.
- `memory_pressure`: 135k free pages (about 2.2 GB); cumulative swapins 237.6M, swapouts 313.1M. `vm.swapusage`: 822 MB used of 2048 MB.
- `repo doctor --json`: verdict **degraded**, 6 findings:
  - `leases.hold-long` x2 (graph scope 529; graph-custody `generations`).
  - `leases.registry-stale`: 95 orphan folders and 28 dead entries of 122 folders.
  - `host.load-high`: load 38.1 on 10 cores.
  - `doctor.probe-timeout`: caches probe did not answer in 8 s.
  - `graph.implementation-newer` (info): generation 260 is 2.5 days old.
- Graph generation `current.json`: generation 260, `closure-missing`, manifest null.
- Graph read log: 113 of 114 owners done. One owner (`@leutwiler/realparts-of-powers-z-n`) has been inferring for about 2000 s with 0 rows. Heap of the graph process 3.6 GB against a 4 GB grant, collected and shrunk. Last lines show `build:closure-entry-rows` and `command-source-imports` still running.
- Workers (`S/origins/*/ready.json`, 12 present; g1-m09 absent):
  - ready: m11 (1).
  - running, stage tools: m02 (20 attempts), m03 (10), m07 (10).
  - running, stage workspace: m01 (4), m06 (2), m08 (10), m10 (6), m12 (12).
  - running, stage dependencies: m05 (20).
  - waiting, stage dependencies: m04 (11), m13 (40).
- Landing locks (`T/landing.lock.*`):
  - `caching-bootstrap`: owner E56, fresh (15:28).
  - `remote-execution`: owner E61 (slice-loop-lag watchdog), fresh (15:25).
  - `policy` and `source`: both owner "E59 tests-harness", taken 15:07, age about 21 min. Below the 40-min takeover limit. No E59 entry in `landings.md`.

## 4. Risks

1. Unrecorded live edits in `ready/` (`🟦️.ts` at 15:27:54, `schema` at 15:25:38) with no landing entry. Lane E56 holds `caching-bootstrap`; the E56 chain does not link to E55's after-hash. Ask E56 and E55 to record the current hashes before the next landing.
2. `doctor/📖️rules/🔣️.json` differs from E58 r2's after-hash (mtime 15:17). Low impact (rules text), but unrecorded.
3. E59 holds `policy` and `source` since 15:07 with no landing record. Takeover is possible at 15:47.
4. Graph read: one owner stuck about 33 min with 0 rows and heap near the grant. The generation is 2.5 days old and `closure-missing`, so a fresh read may be needed.
5. Swap churn is high (313M swapouts) and load runs about 4x the core count. Test runs and builds will be slow and may time out.
6. E60 reports selection, publication and caller-environment tests failing on "live repository store" refusal (harness drift, not verified here).
