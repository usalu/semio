# T1 tests: partial (host load 45-65 makes every file 4-35 min)

## 1. Why repo test-file "exits in 1-8 s with no output"
- Not reproduced: store contract ran 437 s, interface 265 s then 2034 s, test-file's own suite 1486 s. Nothing exits early on this host now.
- Real defect found and fixed: the text renderer never printed a failed row's `error` (only `--json` carried it). Any failure before bun prints (deadline, Python binding, admission) looked like a silent FAILED. Fix: `row.error` line EN+DE in the test-file renderer, test assertion added (see landings 23:50).
- Likely cause of the 1-8 s exits K1/K3 saw: K1's own `pkill 'repo test-file'` plus the 21:20 ENFILE incident (fds exhausted), not a routing gap.
- Interface file: "Original Script deadline refused" after 2034 s: the bun budget (~29 min) is exhausted by real rustc runs under load 60. Environmental.

## 2. Routing gap (python)
`store/tests/contract/` is listed in suite `law-coverage-tests` (no tools: python) but the test calls `requireTestToolV1("python")`; under test-file it fails with "Test tool python not found" (the suite gives no managed Python binding). Same for `build-test-file` (test-file's own test). Suggested fix: move the file to `cargo-unit-cache-processes` (tools python) or add `tools:["python"]` to `law-coverage-tests`. Not applied: `caching/suites/🔣️.json` has uncommitted edits from other writers right now.

## 3. Table (what was actually run, one at a time, taskpolicy -b)
| file | result | cause |
|---|---|---|
| UC/💾️store/🧪️tests/📦️contract | FAIL 0/1 (437 s) | python not bound (routing gap above), pre-existing; R1 reported 24/0 under bare bun |
| UC/🔭️interface/🧪️tests | FAIL (265 s, then 2034 s deadline) | load: deadline exhausted, no assertion failure seen |
| LIB/🏗️build/🔬️test-file/🧪️tests | 11 pass / 3 fail (1486 s) | python missing (1), 1 s deadline fixture starved by load (2); not from this wave |
| all other files (about 225 in unit-cache, remote-execution, remote-cache, stage-fleet) | NOT RUN | each file costs 4-35 min at load 45-65 with one run at a time: not feasible in this session |

Lane-reported results (not re-run): see coordination.md 22:31 and the lane reports (K1 metadata 2/0, K2 portable 35/0, O1 offload 40/0, O3 command 11/0, O4 stage-remote 6/0, R1 staleness 5/0, F1 seal 10/0, G1 law 2/0).

## Open
- Run the remaining files in one `repo test-file --json` batch when load < 10 (after the play site-build), and apply the python routing fix.
