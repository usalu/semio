# r2-fk: cast-export cancellation orphan (agent fb's flaky PID)

## Verdict
A genuine race in the code (Windows), not a test artefact. `_Tree.__init__` started the exporter shell with `Popen` and only then put it into the job object (`_assign`). A job object catches the processes started after the assignment; whatever the process spawned in between stays outside. Under load the gap between `Popen` returning and `AssignProcessToJobObject` can be seconds (preempted thread), and `repo tools exec` starts `bun` within milliseconds. That `bun` (and `repo`) is then outside the job, `TerminateJobObject` does not touch it, and the test sees an orphaned PID until the exporter's `--parent` watchdog kills it.

cv2 (`T/r2-cv2-report.md`) disproved this at 150 runs with `cmd /c ping`, but only without load and without widening the window.

## Evidence
- `T/🧰️fk/membership.py` runs the real cancellation scenario with `IsProcessInJob` on the exporter PID and its `ppid`, with an injectable delay before `_assign` (`FK_DELAY`, simulates preemption).
  - Before the fix, `FK_DELAY=3`: both PIDs `OUTSIDE-JOB` (2 of 2 runs). Without delay: `in-job`, which is why it is rare.
  - After the fix, `FK_DELAY=3`: both `in-job` (3 of 3 runs).
- PID reuse (`T/🧰️fk/pid_reuse.py`) is not the cause: 80 spawns in 20 s, 0 reuses within 10 s.
- The `ppid`/PID in the test belong to the job (membership shows `in-job`), so the test's orphan check is sound.

## Fix
`V/🧍️staffage/🐍️.py` `_Tree.__init__` (about line 166): on Windows with a job, the process is created `CREATE_SUSPENDED` (0x4), assigned, and then resumed in a `finally` (new `_Tree._resume`, `NtResumeProcess`). No child can exist before it is in the job, so the result no longer depends on timing. A failed assignment still resumes (falls back to `taskkill /T` as before). POSIX is unchanged (`start_new_session`, no race: the session exists from `exec`).

## Test fixes (all in `V/🧍️staffage/🧪️tests/🐍️.py`, `Cancellation`)
1. `test_an_interrupt_mid_export_...`: the `< 60 s` bound measured from the start of the render, so a slow `repo` boot under load (55 to 160 s observed with 8 to 24 busy loops on top of the other agents) failed it with no orphan involved. It is now measured from the interrupt (the cancel latency, which is what it claims to check).
2. Same test: the 0.5 s `Timer` is kept and cancelled in a `finally`. Before, an export that failed early left a live timer that fired `interrupt_main` into the next test (seen as `KeyboardInterrupt` inside `test_progress_reaches...` and as `IndexError` on `interrupted[0]`).
3. `snapshot()`: another agent's cross-process cast locks (`.lock-<cast>`, never removed by design) made "cache untouched" fail on every run (`+ .lock-slow-a` etc.). `snapshot` now ignores `.lock-*` files and key folders that hold only locks.

## Results
- `loop.py` (CPU burners via `FK_LOAD`, tests run in-process): `test_a_grandchild_of_the_command_dies_with_it` 100 runs under 16 busy loops: 0 failures (`T/🗑️generated/fk` is deleted; the command is `FK_LOAD=16 T/🧰️fk/run.sh 100 test_a_grandchild_of_the_command_dies_with_it`).
- `test_an_interrupt_mid_export_...` (needs the real exporter, 80 to 120 s per run under this fleet's load, so 100 runs were not feasible): 3 of 3 ok before the exporter broke; the failures since then are all the exporter itself (below), not cancellation.
- Full staffage suite `T/m9_tests.sh ...🧍️staffage.🧪️tests.🐍️`: 51 tests, 3 failures at the last run. 2 are the exporter error below (`quick` and `slow-a` casts); the third (`test_a_failing_progress_callback...` snapshot `{} != {staffage: None}`) was my first `snapshot` filter dropping the empty `staffage` folder, fixed afterwards (second form keeps it) and not re-run to the end.

## Open
- The exporter currently fails for every cast with `Spread syntax requires ...iterable not be null or undefined` (stderr only, no stack) after `[cast 1/n]`. Not cancellation: concurrent edits in the thermal, transport, representation, cast schema and catalog modules by other agents (many files modified in the last hour). Whoever owns those should print `error.stack` in `🎬️cli`/`🎭️cast/🟦️.ts` `main` (`process.stderr.write(\`${error.message}\`)` hides it). Once it is fixed, re-run `FK_LOAD=8 T/🧰️fk/run.sh 20 test_an_interrupt_mid_export_kills_the_process_tree_and_leaves_the_cache_untouched` and the suite.
- `test_a_signal_...` (line ~977, another agent's new test) also starts an uncancelled `Timer`; same fix applies if it ever leaks.

## Files
- Edited: `V/🧍️staffage/🐍️.py` (`_Tree.__init__`, `_Tree._resume`), `V/🧍️staffage/🧪️tests/🐍️.py` (`snapshot`, one interrupt test).
- Scripts: `T/🧰️fk/{loop.py,run.sh,membership.py,pid_reuse.py,stop.ps1}`.
