# r2-xp: cross-platform review of the cast bridge

Scope: TS exporter `S/🔨️modules/📤️export/🎭️cast/🟦️.ts` and the Python side `V/🧍️staffage/🐍️.py` (`_Tree`, `_run`, cache). Linux and macOS cannot run here; those parts are reasoned, not executed.

## Fixed (each with a test)

1. **Silent no-op when the exporter path goes through a link (exporter, former line 315).** The entry guard compared `fileURLToPath(import.meta.url)` with `resolve(argv[1])`. A symlinked checkout (`/workspaces/...`, `/var` -> `/private/var` on macOS, a junction on Windows) or a different drive-letter case made them differ, so `main()` never ran and bun exited 0 without writing anything (Python then failed later with a confusing missing-folder error). New exported `isEntry(moduleUrl, entry)` compares `realpathSync.native` of both. Tests: vitest "runs as the program when it is started through a linked folder, and as nothing when it is imported" (junction, runs bun for real), fixture `🧫️fixtures/🎭️cast-cancellation/🔣️.json` key `linkedEntry`, feature scenario in `🥒️.feature`.
2. **Cache key differed between Windows and Linux/macOS/devcontainer (Python `_source_digest`, `_catalog_bytes`).** (a) CRLF from `core.autocrlf` changed the hash of every source file; (b) `sorted()` of `Path` objects orders case-insensitively on Windows and case-sensitively on POSIX. The devcontainer bind-mounts the same checkout, so host and container shared one `.🧬semio/.../staffage` cache with different keys, and `_prune` of each side treated the other's keys as dead and deleted them after 24 h (cache ping-pong plus double renders). Now files are sorted by `as_posix()` and read through `_lf` (CRLF -> LF). Test: `Cache.test_the_key_ignores_line_endings_and_the_case_rule_of_the_path_order` with the vectors `key.lineEndings` and `key.order` in `V/🧍️staffage/🧪️tests/🔣️.json`.
3. **Parent killed by SIGTERM/SIGHUP/SIGBREAK left the exporter to the watchdog (Python `_run`).** The exporter runs in its own session (`start_new_session`) or process group, so a terminal hang-up or `kill` of the renderer reached only Python, which died without `tree.stop()`. New `_terminable()` turns those signals into `SystemExit(128+n)` (main thread only, previous handlers restored), so the existing `except BaseException: tree.stop()` runs. `_run` is now `_terminable` + `_supervise`. Tests: `Cancellation.test_a_terminate_signal_stops_the_tree_and_the_handlers_come_back` and `..._during_the_export_leaves_no_process_behind` (real SIGTERM on POSIX, interrupt on Windows).
4. **POSIX `_Tree.stop` gave the grandchildren no grace.** It waited only for the direct child (the `repo` shim); once the shim died on SIGTERM, `kill()` sent SIGKILL at once to a bun that was still removing its temporary folder. New `_drain` polls `killpg(pgid, 0)` up to the grace period before the SIGKILL. Covered by the SIGTERM process-tree test above (not distinguishing on Windows; reasoned for POSIX).

## Reviewed, no defect

- `--parent` watchdog: `process.kill(pid, 0)` is `ESRCH` for an exited Windows process (libuv checks the exit code), `EPERM` counts as alive, the timer is unref'd, the 5 s forced `exit(130)` is a backstop. Existing test kills the parent for real on every OS.
- Signals: SIGINT/SIGTERM/SIGHUP on POSIX, SIGINT/SIGTERM/SIGBREAK on Windows (SIGHUP does not exist there). Exit code 130 is set through `exitCode`, so stderr is flushed. A signal during a long synchronous solve is seen at the next frame yield; Python's SIGKILL after the grace covers the rest.
- Atomic writes and cross-device rename: the exporter's temporary folder is inside `--out` and Python's inside the cache root, so every rename is within one file system (no `EXDEV`); `patiently` retries `EPERM/EBUSY/ENOTEMPTY`; `_publish` keeps the winner of a race.
- Paths: ids are slugs (`^[a-z0-9]+(?:-[a-z0-9]+)*$`), so no traversal, reserved Windows names or case collisions; `resolve()` everywhere; `SEMIO_TEACHING_CACHE_ROOT` may be absolute. Line endings: the exporter writes LF (`writeFile` does not translate), Python reads universal newlines.
- `_repo()` honours `SEMIO_REPO_BINARY`, `which("repo")` (PATHEXT on Windows) and the native-repo marker; bun discovery is delegated to `repo tools exec --light bun`; the child inherits the environment; stdout is DEVNULL, stderr is drained by a thread even after a failure, so no pipe deadlock.
- Zombies: `process.wait` reaps the direct child on every path, `kill()` waits up to 10 s; Windows job object has KILL_ON_JOB_CLOSE.
- Unbounded cache: `_prune` removes keys not in the manifest and abandoned `.tmp-*` folders older than a day after every render; with finding 2 fixed, platforms no longer evict each other.

## Open (documented, not changed)

- Two renders of the same missing cast both render it (wasted CPU, correct result: one publish wins). A cross-platform lock would add a lock file with stale-lock handling; not worth it for correctness.
- Windows: a child spawned by `repo` in the milliseconds between `Popen` and `AssignProcessToJobObject` is outside the job (no `CREATE_SUSPENDED` in `subprocess`). Only matters for a hard kill of Python; the `--parent` watchdog covers the exporter itself.
- POSIX watchdog: a parent that died but is not yet reaped still answers `kill(pid, 0)` (zombie); the delay is bounded by the reaper. PID reuse inside the 500 ms poll is theoretical.
- The standalone CLI with a shared `--out` and two parallel runs of the same id can lose the `rename` race with `ENOTEMPTY` after 8 retries (an error, never a corrupt cast); Python always uses a private `out`.
- Prune only runs after a render that had missing casts.

## Runs (Windows)

- Python staffage suite via `T/m9_tests.sh` (module `...visuals.🧍️staffage.🧪️tests.🐍️`): 45 tests, OK.
- Vitest `🎭️cast-export` through `T/q2-vitest-alias.config.ts`: 62 passed, 1 skipped.
- Not run: Linux and macOS.

## Files

`S/🔨️modules/📤️export/🎭️cast/🟦️.ts`, `S/🧪️tests/🎭️cast-export/🟦️.ts` and `🥒️.feature`, `S/🧫️fixtures/🎭️cast-cancellation/🔣️.json`, `V/🧍️staffage/🐍️.py`, `V/🧍️staffage/🧪️tests/🐍️.py` and `🔣️.json`.
