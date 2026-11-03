# r2-lk: cross-process export lock and Windows job-assignment race

Both open items of `r2-xp-report.md` and `r2-cv2-report.md`. Code in `V/🧍️staffage/🐍️.py`, tests in `V/🧍️staffage/🧪️tests/🐍️.py` and `🔣️.json`.

## 1. Per-cast export lock

- `_Lock` (new, before `_lock`/`_wait`/`_render`/`ensure_casts`): an operating-system lock on `<cache>/<key>/.lock-<cast>`; `msvcrt.locking(LK_NBLCK)` on Windows, `fcntl.flock(LOCK_EX|LOCK_NB)` on POSIX, standard library only. A file system without locking (errno not in `_CONTENDED`) degrades to the previous behaviour (no lock, both export, atomic publish keeps one).
- Stale-lock recovery needs no code: the system drops the lock when the owner exits or is killed, so there is no pid file to go stale and no reuse race. The lock files are tiny, bounded by the manifest, live inside the key folder and are removed with it by `_prune`.
- `ensure_casts` loop: for the missing casts (sorted, so no lock-order deadlock) take the free locks, re-check the index after the lock (the owner may just have published), export all held casts in one exporter call (`_render`, the former body), release in `finally` (also on failure or cancellation). Casts locked by others are waited for with `_wait`: poll every 0.1 s until the owner's index appears or the lock frees up (owner failed or died, then we export it ourselves).
- Progress and cancellation: `_wait` calls `progress("[wait] id,id")` immediately and then once per second; `progress` may raise, which leaves the loop (and `KeyboardInterrupt` works in the sleep). Locks are released on that path.
- `_prune` runs once after any render or wait, as before.

## 2. Windows job-assignment race

Dropped on the coordinator's instruction: agent fk already fixed it (`CREATE_SUSPENDED`, assign, `NtResumeProcess`; see `r2-fk-report.md`). I changed nothing there and removed the test I had drafted for it. The lock file name `.lock-<cast>` matches fk's `snapshot()` filter.

## Tests (class `Exclusive`, JSON case `exclusive`)

- `test_renderers_in_separate_processes_export_a_cast_once_and_get_identical_bytes`: two real processes released together through a barrier; the exporter stub (1.5 s) is invoked once, both read the same sha256 of the index, one got `[wait] <cast>` progress, no dot files remain beside the keys.
- `test_a_lock_of_a_dead_owner_is_recovered`: a real child takes the lock, a contender is refused, the child tree is killed (`taskkill /T`; the venv launcher is a separate process), the lock is acquired within 15 s and `ensure_casts` exports.
- `test_a_waiting_renderer_takes_the_result_of_the_owner_without_exporting`: the owner publishes and releases after 0.5 s; the waiter exports nothing, progress is exactly one `[wait]` line.
- `test_waiting_honours_cancellation`: progress raises on the wait line; the exception passes through, no export, the lock is free afterwards.
- `test_a_failed_export_hands_the_lock_on`.
- Pitfalls found: a `_Lock` that is not referenced is garbage collected and releases the lock (the owner script must keep it); the cache key depends on the exporter sources, which other agents edit, so the dead-owner test uses the lock path the child prints instead of recomputing the key.

## Runs

- New class `Exclusive` (5 tests), `T/🧰️lk/repeat.sh`: 20 of 20 runs OK (a first batch had 2 flakes of the dead-owner test, caused by recomputing a cache key while other agents edited exporter sources; fixed as described above).
- Full staffage suite via `T/m9_tests.sh ...🧍️staffage.🧪️tests.🐍️`: 51 tests, OK (last run). Two earlier full runs showed `Cancellation` errors from the exporter itself failing (the concurrent exporter edits fk describes in `r2-fk-report.md`); `Cancellation` alone passed 7/7 and the next full run passed.
- fk's `snapshot()` filter for `.lock-*` files in `Cancellation` already covers my lock files.

## Open

- Linux and macOS not executed (POSIX `flock` path is reasoned; the logic is shared).
- Docker bind mounts between host and devcontainer: `flock` across the VM boundary may not exclude (degrades to the old double export, still correct).
- A hung owner (alive, never finishing) blocks waiters until cancelled; the waiter has no timeout by design (exports can take minutes), cancellation is the way out.
- Two casts exported by different processes in one batch each (A+B and B+C) still work: B is exported by whoever locks it first, the others wait; no deadlock because no process waits while holding a lock others need (it exports first, then waits).
