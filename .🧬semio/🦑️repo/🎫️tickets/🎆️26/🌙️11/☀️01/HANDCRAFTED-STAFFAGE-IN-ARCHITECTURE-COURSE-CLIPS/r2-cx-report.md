# r2-cx report: cancellation and progress of the cast export chain

Chain: manim clip -> Python `ensure_casts` -> `_run` -> `repo tools exec --light bun -- 🎭️cast/🟦️.ts` -> `exportCast`.

## Gaps found (before)
1. TS `render()` was one synchronous loop over all frames. A signal handler is an event-loop callback, so SIGINT could never be seen between frames; it was only seen between casts, after a whole cast (up to 600 thermal frames) had finished. `main` also handled SIGINT only (no SIGTERM, SIGHUP, SIGBREAK) and exited 1 on cancel.
2. `publish` could still rename a cast into `out` after the abort fired during rendering.
3. Progress on stderr was only `[done/total]` every 50 frames (nothing for a short cast, no cast id, no pid).
4. Python `_run`: `for line in process.stderr` blocks the main thread. On Windows a blocked pipe read is not interrupted by Ctrl+C, so a KeyboardInterrupt waited for the next stderr line (up to a whole cast). Cancel used `taskkill /PID /T` on the repo shim only (fails once the shim is gone, because children are re-parented), `process.kill()` on POSIX (leaf only, not the group). Progress was only echoed to stderr, not forwarded to the caller.

## Fixes
- `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts`: `render` is async and yields (`setImmediate`) before every frame, so a signal aborts between two frames; `publish` re-checks the signal before it replaces anything; new `CastOptions.started(id, i, n)`; `main` handles SIGINT, SIGTERM, SIGHUP (POSIX) or SIGBREAK (Windows), prints `[pid n] ppid m` first, `[cast i/n] id` per cast, `[done/total]` every 50 frames and at the end, and on cancel prints `[cancelled]` and exits 130. Temp folder `.id.tmp-pid` is removed by the existing `finally`; an interrupted cast is never renamed.
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py`: new `_Tree` (Windows: job object with KILL_ON_JOB_CLOSE via ctypes, new process group, CTRL_BREAK then `TerminateJobObject`; fallback `taskkill /T`; POSIX: `start_new_session`, `killpg` SIGTERM, 3 s grace, `killpg` SIGKILL). `_run(command, progress)` reads stderr in a daemon thread and polls `process.wait(0.1)` in the caller thread, so KeyboardInterrupt is prompt on Windows; any `BaseException` (also one raised by the progress callback) stops the tree; the job is closed in `finally` (kills stragglers). `ensure_casts(ids, progress)` forwards every exporter stderr line to the callback (still echoed as `[staffage] ...`). The Python `finally` already removes the `.tmp-*` folder; the cache rename loop is per cast folder, so the cache holds only complete casts.
- Existing test fake `_run(command, progress=None)` adapted.

## Tests
- Shared language-agnostic fixture: `🧫️fixtures/🎭️cast-cancellation/🔣️.json` (one quick cast, three slow 600-frame thermal casts, cancel after 30 frames, exit code 130, progress regexes). Scenarios added to `🧪️tests/🎭️cast-export/🥒️.feature`.
- vitest `🧪️tests/🎭️cast-export/🟦️.ts`, new `🛑 cast export cancellation and progress`: abort after exactly 30 frames (listing of out unchanged, finished cast byte-equal, only the first slow cast started); abort after the last frame never publishes; real CLI subprocess progress lines + exit 0 (skipped without bun); real CLI SIGTERM -> exit 130, `[cancelled]`, empty out (POSIX only; written but NOT executed here, Windows cannot deliver SIGTERM gracefully).
- Python `Cancellation` (real subprocess through `repo`): progress reaches the caller and the cast lands in the cache; `KeyboardInterrupt` (via `_thread.interrupt_main`) 0.5 s after `[cast 1/3]` -> no tasklist/ps-visible survivor among Popen pid, bun pid and ppid, cache snapshot (paths and bytes, with a seeded sentinel cast) unchanged, no `.tmp-*`, finished well under 60 s; a callback that raises cancels the same way; a grandchild test with a Python parent that spawns a sleeping grandchild (discriminating: with `stop` replaced by `process.kill()` it hung, i.e. fails). psutil is not a dev dependency, so tasklist / `ps` is used.
- Note: the real `repo tools exec` shim already cleans its own children when killed (the mutated stop on the real chain still passed), so the grandchild test is what proves the job object / process group.

## Results
- vitest `cast-export` (ticket config `q2-vitest-alias.config.ts`): 47 passed, 1 skipped (the POSIX SIGTERM case on Windows), 1 file.
- Python `🧍️staffage.🧪️tests.🐍️` plus `🧪️tests.🐍️staffage_only`: 53 tests, 6 failures, all subtests of `staffage_only.test_rule_a_no_clip_builds_a_physical_object_from_primitives` (e.g. `_thermometer` in `⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` not in the allow list): concurrent clip migration by other agents, not this change. All staffage bridge tests, including the 4 new `Cancellation` tests, pass (the Cancellation class ran 3 then 4 tests OK).
- No bun, node, or sleeping python orphan after the runs (scan with `🧰️tc2/orphans.ps1`).

## Open
- POSIX paths (`killpg`, SIGTERM test) are unverified on Linux/macOS (no such host here).
- Graceful CTRL_BREAK on Windows only reaches a process sharing the console; otherwise the job object kills hard, and Python removes the temp output folder.
- If Python itself is hard-killed on POSIX the exporter runs on (Windows job closes with the process); a stdin heartbeat or `PR_SET_PDEATHSIG` would close that.
- `🧰️tc2/` holds `mutation.py` and `orphans.ps1`; `🗑️generated/tc2` deleted.
