# T2b test sweep (partial: host load 30-50, one file = 1-16 min)

Full list: 193 files in scope (191 runnable; 2 helpers under testing/📥️invocation/🧪️tests/🧫️). Sequential at ~9 min/file average = ~30 h, so only a prioritised part ran. NOT RUN: the other ~175 files (sidecar, offload except literal edits, remote-cache rest, incremental, testing rest). Prioritised order list was files whose member changed in the last 25 h (120 files).

| file | result | s | cause |
|---|---|---|---|
| testing/🎚️budget/⏲️contention/🧪️tests | 14/1 fail | 565 | load: "survives a busy host" 2 s budget (pre-existing, F4 report) |
| testing/🎚️budget/🧪️tests/⏳️literal-deadlines | 1/2 fail | 1596 | see A; 2nd test = Python scan timed out (whole-framework walk under load) |
| cas/📌️pins | pass 1/0 | 310 | |
| cas/🆘️full-disk | pass 17/0 | 224 | |
| cas/🧪️tests | 48/1, rerun 49/0 | 965 / 149 | load flake (B) |
| slots/🧪️tests | 3/1, after fix 4/0 | 106 / 60 | R2 Declaration v2 (C) |
| remote-cache/retention/🍂️staleness | pass 5/0 | 66 | |
| remote-cache/retention/🔗️restorable | pass 10/0 | 190 | |
| testing levels ⏲️stretch (coordinator request) | 2/1 -> 3/0 | 24 | F2/F4 clamp (D) |
| build/🔬️test-file/🧪️tests/🔭️tool-binding | pass 3/0 | 272 | PY1 60 s probe ok |

## Failures attributed
A. literal-deadlines law: tonight's offload tests (O1/O3/O8) added literal deadlines >= 10 s: offload/🧰️command 9, 🌐️cluster 13 (ceiling 11), 🔗️native-link 1 + 🌐️cluster 2, ⏱️timeouts 1. Fix (product-side test code): 4 files now use `stretchedBudgetMillisecondsV1(n)` (import added); ⏱️timeouts keeps its literal (semantic deadlineMilliseconds:15000 input under test) with ceiling 1; cluster ceiling removed (now 0). Scoped rescan: no ADDED/SLACK left in offload. Still failing, other lanes: `build/📥️caller/🧪️tests/🌐️globals` 2 (ceiling 1), `build/🕸️graph/📥️inference/🔎️owner-discovery/🏭️farm/🧪️tests/👷️children` 2, `…/🧪️tests/📚️shared-catalog` 1 (not in my scope, unattributed; law test not rerun: it walks all of 🧰️framework, ~15 min).
B. cas guard "write that times out ... collection finishes": passed on rerun (49/0); load (the 30 ms sleep precedes the collection start under load 40).
C. slots test fixture still `version:1`; R2 made Declaration v2 (landings 05:15). Fixture updated.
D. stretch oracle: F2/F4 deliberate clamp. Python oracle now takes both schemas as argv and clamps to min of maxima (no literal); TS test passes the paths; inline formula of test 3 clamps too.

## "Explicit finite owned Bun file selection required" (🟨️bun/📁️files/🟦️.ts:37-39)
Not reproduced: every file run tonight (contention, literal-deadlines, cas, slots, ...) passed through runOwnedBunFilesV1. Ruled out by measurement: path length (longest in-scope test path 184 code units, validator also accepts 250 emoji units; nothing > 256), cwd absolute, `runtimeSupportsV1("test")` true on pinned bun 1.4.2. Remaining unmeasured clauses: capability values > 256 chars (only suite `tools-authorizers` passes `artifacts`, so not host-wide), non-integer/NaN timeout (stretchedBudgetV1 uses Math.ceil, ceiling clamped by F4). Most likely a process started before F4's clamp (O5's run predated it). Recommendation (not applied): make the refusal name the failing clause; the test at 🧪️tests:47 compares the exact message, so it would need `startsWith`.

## Python-unbound tags
Dry-run routing: only 1 of 191 files is `python:true`; suites that call `requireTestToolV1("python")` without the tag (PY1's scope) are not distinguishable by dry run.

## Open
Remaining ~175 files unrun; rerun in batches of 6 once load < 15 (`repo test-file --json <6 files>`), list in the scratchpad `order.txt` equals members changed in the last 25 h.
