# Result E63: warm single-target `repo run` latency (measure only, nothing landed)

## Verdict
No code landed. In this wave a warm run cannot be measured or fixed cleanly on this tree: every run I made was restarted or stretched by peer landings (slice worker exit 75 "code of the build tool itself changed", "a new pass over the owners starts: N path(s) changed under the read", Nx tool admission re-verifying after closure edits). I did find and measure where the time goes in the one run whose slice was not restarted, and I list the levers with evidence. There is NO before/after for the same command (3 "before" runs below, 0 "after").

## Measured (command `SEMIO_REPO_PROFILE=1 repo run repo:test-cache-reset --dry-run --exclude-dependencies`, script `T/🧰️scripts/E63/run.sh <name>`, load gate respected at start: 40.8 / 36.8 / 30.4)
| run | start | outcome | total |
|---|---|---|---|
| before1 | 15:29 | FAILED at 695 s: `BUILD_OWNER_FAILED` owner `repo`, `test.aggregate-refused route-targets:patch-loader-processes:0` (a peer added suite row `patch-loader-processes` to `⚡️caching/🗂️suites/🔣️.json` 15:41:56 before its target existed in `⚡️caching/📋️project.json`; cured minutes later) ; 142 slice restart ticks (exit 75) | 695 s |
| before2 | 15:45 | completed, one dependency receipt re-key (E58 walk stamp), slice not restarted | 765 s |
| before3 | 15:58 | slice restarted 18+ times by "code of the build tool itself changed (🕸️graph/📥️load/🟦️.ts)" (E61's landing) and repeated owner passes; cancelled by me (SIGINT, my own pid) at 463 s | 463 s, no result |

### before2 phase table (the only clean run; seconds from start, load 31-42)
| t (s) | phase |
|---|---|
| 0-36 | graph service started again (code changed since the last run) |
| 36-198 | graph slice read = 160 s: 2.9..~92 first line only after about 54 s with no phase (worker `start`), inventory (memo check 46k observations 1.8 s, then scoped discovery: owner `repo` stage `command-sources` 30 s alone) to 138 s, owners round 1 138-172 s (3600 owners, taken 0 of 1 wanted), facts-hold 178-198 s |
| 198 | capacity admitted 2,303 MB immediately |
| 217-262 | source admission: stale dependency receipt (E58 walkStamp /4 re-key, once per machine), source listing 12 s, metadata 22 s, providers 4 s |
| 340-625 | Nx tool admission = 285 s: storage planning 2 s, index read 10 s, verify 10 s, prepare 12.5 s, "Verifying build tool integrity" 72 s, verification 47 s, prepare 42 s, verification 34 s, prepare 4 s (three prepare/verify rounds = bundle/generation work caused by a peer landing in the authorizer closure) |
| 625-765 | Nx run, publish |
Comparison: E35 round 2 quiet run 276 s, loaded 667 s; this host with 15 lanes landing ran 765 s.

## Where the slice time goes (evidence)
1. Slice worker is plain `bun .../👷️worker.ts` forked per read (`🧵️process`). Importing its modules costs 0.74 s in isolation (`T/🧰️scripts/E63`-style probe: 9 imports, BUN_RUNTIME_TRANSPILER_CACHE_PATH=0), so the 40-54 s before the first phase line is NOT import/transpile. Candidates left unmeasured: pin of the implementation (`useProcessWideImplementationPinV1`), generation/registry reads, lease/queue waits (before3 shows "waiting for the lock repository-graph-optional-ledger, place 2 of 2 behind process ..." at 275 s). A `sample(1)` of a worker in that window only showed unsymbolicated JIT frames plus lstat/openat, so it could not name the function; a JS-level phase timer inside `worker.ts` before the first `inventory` step is the next measurement.
2. Owner `repo` is rediscovered after every worker restart (command-sources 7-30 s each, in the sampled `slice-memory.json`): its recorded rows include the repo CLI sources that peers edit all day, so its checkpoint never holds; `hold(rows)` fails and `observeOwnerFactsMemoizedV1` falls through to full discovery.
3. A worker restart (exit 75) loses all in-memory state; stored per-owner records exist (`📥️load/🧠️admission-memo`, `💾️owner-checkpoint`, fleet exchange) and were used (memo rows 23-42k verified in 2-4 s), so the restarts themselves cost about the 40-50 s boot plus the `repo` owner, not the 3600 owners.
4. Nx admission rebuilds (prepare/verify rounds) whenever a peer edits one of the ~1,800 authorizer-closure files; E28's split keeps the generation but the bundle `A` still changes. Unchanged key => no rebuild (confirmed: no generation clone appeared), but the keys changed in every run here.

## What I did not do and why
- No edit: every candidate lever touches files that peers (E61 slice event-loop lag in `📥️load`, E62 owner registry scope, E58 receipts, tools lanes) change in the same minutes, and I could not produce a before/after at a stable tree (the stable-tree window is shorter than one run). Landing blind into the churn would have restarted the 15 supervisors for a change I cannot prove.
- The whole graph read (pid 71335) and pid 72392 were not touched; my only signalled process was my own `repo run` (18992).

## Recommendations (ordered by evidence)
1. Add a JS phase timer around worker boot (before `inventory`) in `👷️worker.ts` / `report.registerSliceReportSourceV1`, to name the 40-54 s (lease wait vs implementation pin vs registry read). Owner of the slice report: E61.
2. Make the `repo` owner record stable: its `files` hold the CLI source closure; key the checkpoint on a closure content digest (like the receipt `profileSha256` of E35/E58) instead of per-file rows, or exclude implementation files of the running tool from its rows (the implementation pin already guards them).
3. On exit 75, hand the next worker the previous worker's finished owner records through the checkpoint before it exits (the exit hook), so a restart does not rediscover `repo` and its neighbours.
4. A measurement mode that freezes the implementation (a pinned copy of the tree for the run) so latency numbers do not depend on peer landings; today an A/B needs a quiet replica.
5. Make `requireCanonicalTestAggregateV1` tolerate a suite row whose target is not yet declared only through one atomic landing: a half-landed suite row failed every `repo run` for the whole machine for minutes (before1).

## NOT verified
- Any speed-up (nothing changed). The 40-54 s boot cause. Whether a quiet tree gives the 276 s of E35 today. The cost split between boot and `repo` discovery at load below 20.

## Other lanes must know
- Peer landings in `🕸️graph/📥️load/🟦️.ts` restart every slice worker (exit 75, message "code of the build tool itself changed", 18 times in one 8-minute run): E61 and E62 should land in one step each and not as many small edits.
- The `slice-memory.json` of `.cache/f` is shared by every slice worker of the checkout (my sampled phases alternated with the whole read's).
- Scripts kept: `T/🧰️scripts/E63/{run.sh,sample.py,stack.sh}`.

# Round 2 (16:10-16:45): landed (1) finish-on-loaded-code and (2) the boot gap; (3) not done

## (1) A running slice is not restarted by a code edit (landed)
Cause of the 18 restarts: a pinned worker observing the CURRENT version of an implementation file it loaded at another digest threw `requirePinnedImplementationFactsV1` (stale source facts, exit 75). Change: `finishOnLoadedImplementationV1()` (called by `👷️worker.ts` after the pin) makes `record()` store the LOADED digest for a `file` fact of a loaded implementation file that moved after load (`loadedImplementationRecordedFactV1`); the existing law "a pinned process holds a fact when it names the loaded digest" then keeps every round green, nothing throws, and the next process (new code, new loaded digest) finds the old digest stale and observes again. Test: `🔒️loaded-implementation` 7/0 (+1, two child processes, independent sha256: before the flag the file edited after load records the current digest, with the flag the loaded one, unrelated path and error facts untouched, the second process records the new digest). Graph stale-facts 2/0, seal-under-churn 3/0.
Live: after2 had 0 exit-75 restarts and 0 "itself changed" lines (before3: 18, before1: 142 ticks), while peers landed code during the run. Limit: results computed by old code of an edited implementation file are recorded at the old digest only for `file` rows; `analysis` rows read the current text.

## (2) The 40-54 s pre-phase gap: found and fixed
Phase timer (temporary `[DEBUG] slice-boot` marks in the worker, removed): imports 3.0 s, invocation 0 s, backend 0 s, `withRepositoryCompilerRuntimeEntriesV1` 0.7 s, then **`backend.graphExternalInputs()` 21.3 s** (3.78 s -> 25.05 s at load 50). It calls `nxBuildEngineExternalInputsV1`, which closed the private Nx engine import graph (2,098 files read and scanned) once per family and again for the digests, nothing memoised. Isolated measure at load 59: closure 9.2 s, again 5.8 s, external inputs 16.5 s. Fix: `nxBuildEngineClosureV1` answers from an in-process memo while every file's `dev:ino:size:mtimeNs` stamp holds (2,098 lstats); after: 9.4 s, 0.04 s, external inputs 0.24 s. Test: engine-digest +1 (memo equals a fresh process, callers cannot mutate it), nx-engine 19/0, generations external 6/0; engine-digest main 11/1: the failing spawn-only test (`cut` closure still contains `🏗️build/📜️script.ts`) is a content question of the closure, not of the memo (same scan), unverified against the pre-change tree.
Remaining for the next round: the first closure still costs 9 s per worker process (every restart): a persisted per-file references memo keyed by stamp (like E35's digest memo) would make it a few hundred ms.

## Numbers (same command; load was 48-59 throughout, gate 40 never met, so these are NOT 3+3 comparable runs)
| run | load at start | slice (s) | restarts | total |
|---|---|---|---|---|
| before2 (clean) | 37 | 160 | 0 | 765 s |
| before3 | 30 | restarted 18x | 18 | cancelled 463 s |
| after1 | 49 | n/a | 0 | failed 160 s: `Resource lease timed out: resource-directory-admission:v1` (load 50, peers hold it; unrelated) |
| after2 | 56 | about 250 | 0 | 1,597 s at load 48-56 (Nx admission and dependency adoption under `resource-directory-admission:v1` waits dominate) |
So the restart count is down to 0 and the 21 s gap is gone, but I could NOT show a total improvement: the load was above 50 and the Nx stage waited on leases. No 3 runs each were possible.

## (3) `repo` owner records surviving unrelated CLI edits: NOT done
Not started: it needs the per-file row validation law of E62/E75 in the owner checkpoint (`📥️load/💾️owner-checkpoint`, `observeOwnerFactsMemoizedV1.hold`), and I ran out of room before reading their landings. With (1) landed the `repo` owner is no longer rediscovered per restart (there are none), only per request when its rows moved.

## Unverified
Total latency improvement; (1) under a peer edit of an `analysis`-read file; whether the engine-digest spawn-only failure pre-exists; Windows/Linux.
