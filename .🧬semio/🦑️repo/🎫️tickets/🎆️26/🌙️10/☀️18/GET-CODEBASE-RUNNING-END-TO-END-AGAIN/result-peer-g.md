# result-peer-g (wave 1026g): official evidence for "peer edits cause 0 re-executions and no failed runs"

## VERDICT: NO EVIDENCE PRODUCED (all three scenarios NOT RUN). Checkpoint 21:58, usage cut-off.
Why: course:build never got past admission. Attempts (host 32 GB / 10 cores, load 24-45, 3-6 peers queued, every run asks 7168 MB):
| attempt | start | outcome |
|---|---|---|
| a (`🗑️generated/peer-g/a`) | 20:10 | exit 1 after 312 s in the capacity queue: host `ENFILE: file table overflow, socketpair` in `readHostCountersV1` (`spawn vm_stat`). Host fault, not a build result. |
| b (`.../b`, control:course) | 20:17 | 69 min: capacity wait 1700 s (spare 4.6 GB < 7168 MB), then `graph: a peer builds the graph` wait, then a second capacity wait (`too many disk images are attached`); I cancelled it by pid at 21:27 for gate-g priority (coordinator 20:56). |
| c (`.../c`, chain control, p1, control, p2, control, p3, control) | 21:27 | waited 30 min for gate-g's product builds to end (gate rule in my driver), control had just started at 21:57; killed by me at 21:58 (checkpoint). |
No file under test was ever edited (`git status` of the four files clean); no `EDIT ON` line was written. Nothing runs, nothing is staged for gate-g, no library edit, ticket not closed.

## Findings from the attempts (measured, not yet product evidence)
1. A hot course:build is admitted as `need 7168 MB` (sized at 200 % from 15 earlier runs): on this host it queued 20-40 min at load 30-45 while the work needs ~0.7 GiB. The reserve, not the build, prevents any peer-edit measurement today (same as bench2-g).
2. Host ENFILE (system file table) kills a queued run inside the capacity sampler (`runHostCommandV1` spawn vm_stat, no retry). Candidate fix for gate-g/capacity owner: classify ENFILE/EMFILE from `spawn` in `readHostCountersV1` as transient `busy` (churn-g `🌊️races` already has the class) and retry with backoff instead of exit 1. NOT staged.
3. A peer-built graph makes a queued run wait as its "front door alone" (yield, churn-g 09:15); it worked (waited, did not fail).

## Tools (ready, tested only up to launch)
`C/peer-g-tools/scenario.py` (driver: reuses bench3-g `measure.run`; stage = `control:<product>` | `peer:<id>:<product>`; gates on WAITING < 4 AND no `gate-g-tools/product.py` process; retries host ENFILE up to 4x; waits for the trigger row in the live log, appends ONE comment line to each file of the scenario, holds it 25 s, restores byte-exact (sha256 checked, forced restore in `finally`), announces `peer-g EDIT ON/OFF <file>` in coordination.md; records exit, wall, phases, Bazel `executed/cached` line, graph rows, tree RSS, peer files modified during the run), `scenarios.json` (p1: `CITATION.cff` + `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts`; p2: `🎓️teaching/🏛️architecture/📚️course/📚️course/🟦️.ts`; p3: `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🪆️scope/🤲️consumption/🟦️.ts`), `stopafter.py <pid> <json>`.
NOT yet exercised: the trigger/edit/restore path (never reached). Test it first graph-free: run `peer:p1` against a fake log, or accept the first official run as its test.

## Exact next command (when gate-g is idle and WAITING < 4; one official run at a time)
```
cd /Users/ueli/Documents/semio; C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; O="$C/🗑️generated/peer-g/d"; mkdir -p "$O"
env -u SEMIO_SCRIPT_STALL_MS python3 -I "$C/chain-tools/detach.py" "$O/chain.log" python3 -I "$C/peer-g-tools/scenario.py" "$O" control:course peer:p1:course control:course peer:p2:course control:course peer:p3:course control:course
```
Read `$O/chain.log` (one JSON line per stage: `executed`, `line`, `peerState.restored`, `peers`) and `$O/*.json`. Verdict rules: P1 PASS = exit 0 and `executed == 0` in the peer run and in the next control; P2 expected: only the actions whose inputs include the course TS source (record which, from the `INFO: N processes` line and the Bazel log); P3 live baseline = whatever happens (restart/exit 75/graph re-inference/failure), compare with seal-g's `C/seal-g-tools/isolation.py` (graph-free live vs sealed). Count a run as noisy when `peerFilesDuringRun` lists other lanes' edits. print-rs: use stage product `printbuild` (target `@semio-tech/print-rs:build`) or `print` (`:test`); not tried; do not edit under `📓️print/**` while rustedit-g/agents3-g run.
Cleanup when done: delete `C/🗑️generated/peer-g/` (raw logs, markers, backups of unedited files).

DONE marker not written as a success: see coordination line "peer-g STOPPED".
