# R193: "its child process wrote nothing for 60 s" (living report, 2026-11-18)

Scripts: `T/🧰️scripts/R193/` (T = `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`): `patch.py` (anchored edits of the base copies), `preload.ts` (bun preload serving staged file contents under the LIVE paths, also for child processes through `BUN_OPTIONS`), `runtest.sh`, `land.sh`, `land.tsv`.

## 1. What the question is, and what happened (established)

- Spawn site: `🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🟨️.mjs` (`answerIn`). Three kinds of child request share one rule: a heavy derived row (`program`, `derive`), the `preflight` (walk of the 39 declared tool closures and provider ports, once per process, first thing of every generation build: `🕰️host` `prime` -> `primeSharedClosuresOfWorkspaceV1`) and the `prime` of the shared closure store. Child: `📜️script.ts` under a 2560 MB cap. The rule: the parent stopped a child when no byte reached its stderr for the fixed `silenceMilliseconds` 60000; the child's "life line" was one `setInterval` of its own event loop every 2 s. "Wrote nothing" therefore meant "its event loop did not turn for 60 s".
- The three stopped sessions (`.cache/f/graph-server/read.1794935768501.log` 17:02-18:16, `read.1794936472671.log` 18:26-18:27, `read.1794957355419.log` 23:49-00:15): each is a graph-server generation read (whole workspace, `build:source-admission`, `generation -1`). In all three the last lines before the stop are the `dependencies:generation-verify` activity of the command, then about 60 s of nothing but the parent's own "still running after N s" ticks (the parent loop was alive), then the stop with detail `-` (the child had written no line besides the life line). The first child question of such a build is the preflight. The stop came about 60 s after the verification in the 90 s session. The child never said which question it was on or in which phase: that is the diagnostic defect (no phase side channel, the refusal named neither question nor phase).
- Cause measured today (load 18-43, this Mac, 32 GB): the real child on `{preflight:true}` takes 23-38 s wall and holds its event loop for ONE synchronous stretch of 12.1 s and 19.9 s (two runs) before the second closure is done (heartbeat lag reported by the new line: `preflight:2/39 12063`, `preflight:2/39 19934`), later stretches 0.1-2.8 s; a 500 ms interval fired once during 24 s. Plain imports 1.8 s, opening the deriver 1.5 s. At load 50-60 with swap (the sessions) a 20 s stretch is 60 s or more: the life line cannot be written, the fixed window ends. A silent but busy, healthy child was killed.
- Not established from logs: the exact load at the three stops and the client command lines (the read logs name no command, scope or selection).

## 2. Fix (staged, landing in the 07:30 window)

- Liveness is progress: the parent window is `silenceMilliseconds` x scale; the scale is `min(maximumScale 8, speed factor x max(1, load1 / cores))` (`heavyQuestionScaleV1`, speed factor of R135). A child that was quiet for `livenessMilliseconds` (5 s) is looked at: its processor time (`processCpuMillisecondsV1`, `ps`/`/proc`) rising by the R135 progress step (50 ms) re-arms the window, so a long synchronous stretch is progress. Where the platform cannot say (Windows) the window x scale stays. The wall limit `timeoutMilliseconds` is scaled the same way.
- Typed bilingual refusal `graph.heavy-question-silent` (catalog text changed in place): question (`preflight`, `prime` or `kind path`), last phase, silent seconds, longest event-loop stall, tail. EN and DE.
- Side channel: the child's life line is now `\u0001alive <phase> <lag ms>` (phase boot/read/open/prime:i/n/preflight:i/n/question; lag = lateness of that turn, so synchronous stretches are measured). The preflight (`preflightToolClosuresV1(…, {progress})`) and the prime (existing `progress`) yield to the macrotask queue after each closure/recipe, so the timer runs between them.
- A lease wait is waiting: the child's loop turns during awaits, wait lines and life lines count as sign of life, the window is not a wall limit.
- Parent-gone: the child starts the R108 watchdog thread (`🏊️project-farm/🐕️watchdog`) as its first act (dynamic imports after it), so a SIGKILLed parent ends the child within about a second whatever it computes.
- Synchronous passes on this path (list): imports 1.8 s; deriver open 1.5 s; first one or two closures of the preflight 12-20 s (NOT sliced: first-use analysis of the TypeScript machinery, measured by the lag field and covered by the processor-time rule); later closures 0.1-2.8 s (now separated by turns); the heavy `program`/`derive` rows (TypeScript program creation, not measured today); prime recipes (not measured).

## 3. Why heavy at all

The stopped sessions are builds of the whole-workspace generation by the graph server (`reason refresh` or an answer to `generation -1`), whose first act is the preflight of all tool closures. A command with a selection (`run-many --projects`, `tasks`) loads the slice of a sealed generation or the selected read (R139/R143, `loadBuildGraphUnpinnedV1` baseline) and does not need that build; which of the three clients had a selection cannot be told from the logs (no command line is logged). Open: the preflight could be limited to the tools the selection's owners name; not done.

## 4. The other four spawn sites without a parent watchdog (state)

- `🧠️memo/🧮️derive/♨️warm`: deliberately detached warm deriver with an idle timer (`idleMilliseconds`); outlives the parent by design, no parent-gone rule. Unchanged.
- `🏡️resident/🏊️delegate` (+ `🏭️executor`): children killed on `close()`/abort (SIGINT then SIGKILL); a SIGKILLed parent leaves them until their stdin closes. Unchanged.
- `🛰️sidecar/🕸️graph/🔭️read`: the read runs detached on purpose (the sidecar adopts it, its own stall watchdog sends SIGINT after 25 min without a wave). Unchanged.
- `🛰️sidecar/🚦️ready/🏃️wrapper`: stage tree, detached on purpose; bounded by the stage caps and the process-tree rules (R135/R183). Unchanged.
Fixed here: heavy-questions.

## 5. Proof

Staged (scratch, preload under live paths, transpiler cache off): heavy-questions test file 16 pass 0 fail (95 s); texts catalog tests 3/0. Red on the old files with the new test file: the silent-message law, the busy law (killed after 1.5 s), the dead/derived-window law, the scale law and the heartbeat law all fail; the lease-wait law passes on old and new (a guard). The orphan law could not be run red (the old child never prints a phase, so the test never reaches its kill step; it hung and was stopped); it is green on the new files (real parent killed, real child gone within 6 s). Laws: `law: a child that computes without printing is not stopped…`, `…truly dead is stopped within the bound derived…` (frozen via `Atomics.wait`, scale 2 -> window 3 s), `…waits on a named lease…`, `…the window is the speed factor times the load per core…` (independent model), `…turns its event loop during the preflight…` (real child), `…whose parent is killed ends within seconds…` (two real processes).

## 6. Not verified

No real stop reproduced at load 60; the scale at load 60 is a model. Windows (no processor time) not run. Running services use the old code until they restart.

## Landing

Landed 2026-11-18 07:01:40-07:03:26 UTC (entry in `landings.md`); live tests 16/0 and 3/0 under the lock.
