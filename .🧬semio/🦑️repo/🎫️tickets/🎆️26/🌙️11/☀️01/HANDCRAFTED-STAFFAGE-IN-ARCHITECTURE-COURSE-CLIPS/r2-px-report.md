# r2-px report: POSIX cancellation of the cast export chain

## Environment found
- WSL2 `Ubuntu-24.04` exists (was stopped; I started it and terminated it again afterwards). It has python3.12 but no bun, node, uv or pytest. The Windows bun is a Windows exe and says nothing about POSIX signals. Installing bun would be installing system software, so I did not.
- Docker Desktop is installed but its daemon is stopped (`dockerDesktopLinuxEngine` pipe missing); no images; podman absent. The devcontainer was therefore not run. Starting Docker and pulling the image is a system change and a download, left to the owner.
- Consequence: the Python half was verified on real Linux; the Bun half (SIGTERM vitest case, Bun signal handlers) could not be run on Linux and is reviewed only.

## Linux verification of the Python half
- `T/🧰️px/posix_tree.py` extracts `_Tree` and `_run` from `🧍️staffage/🐍️.py` by `ast` (no manim needed) and drives them with `sh` stand-ins. Run in WSL with `T/🧰️px/run_wsl.sh` (needs `MSYS_NO_PATHCONV=1` from Git Bash). **7 of 7 pass on Linux 6.18**:
  - stop signals the whole session group (leader plus two `sleep` grandchildren; pgid = pid, new session; done in under 2.5 s; return code -SIGTERM).
  - SIGTERM ignored by leader and grandchild: escalates to SIGKILL after the grace, all gone.
  - leader already exited, grandchildren alive: `kill()` still takes the group (killpg on a dead leader's pgid works because the group lives on).
  - `_run` with a progress callback raising `KeyboardInterrupt`: tree gone in under 6 s.
  - `_run` failure carries the stderr tail.
  - real SIGINT delivered to the main thread during `_run`: `KeyboardInterrupt`, tree gone (the stderr reader thread does not block it).
  - hard-killed Python (SIGKILL) with a `start_new_session` child: both group members SURVIVE. This documents the orphan gap that the watchdog closes. No `_Tree` change was needed; no POSIX bug found in `_Tree`.

## Orphan fix on every OS: parent watchdog
- `🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts`: new `--parent <pid>` argument (`parseArguments(...).parent`, validated: positive integer, else error naming `--parent`), exported `processAlive(pid)` (`process.kill(pid, 0)`, `EPERM` counts as alive) and `watchParent(pid, gone, interval = 500, alive)` (unref'd timer, fires once, returns a stopper). `main` starts it when `--parent` is given: on loss it aborts the controller (cancel between frames, temp folder removed, `[cancelled]`, exit 130) and arms an unref'd 5 s `process.exit(130)` fallback for a stuck event loop. The watched pid is Python's, passed explicitly, because bun's own parent is the `repo tools exec` shim, which a hard kill of Python does not take along on POSIX (and Windows `process.ppid` is fixed at creation).
- `🧍️staffage/🐍️.py` `ensure_casts`: passes `--parent str(os.getpid())`. Works on Windows (job object plus watchdog), Linux and macOS (macOS has no `prctl`; the pid watchdog covers it, so no ctypes `PR_SET_PDEATHSIG` was added). PID reuse within 500 ms is the only theoretical gap.

## Tests added
- vitest `🧪️tests/🎭️cast-export/🟦️.ts` (all run on Windows): `parses --parent and refuses a bad process id`; `watches a parent` (callback never while alive, once after, silent after stop); `exits 130 within seconds when the parent given by --parent is hard-killed, on every OS` (real CLI, fake parent bun process SIGKILLed after `[cast 1/3]`, exit 130, `[cancelled]`, empty out, within 30 s).
- Python `Cancellation.test_the_exporter_is_told_which_process_to_watch`.

## Results
- vitest `cast-export` (ticket `q2-vitest-alias.config.ts`): 50 passed, 1 skipped (the SIGTERM case, POSIX only), 1 file.
- Python `🧍️staffage.🧪️tests.🐍️`: 36 tests OK (includes the 4 `Cancellation` real-chain tests plus the new one).
- No bun/python orphans left; WSL distro terminated; nothing generated to delete.

## Review of what could not be run (Bun on POSIX)
- `process.on("SIGTERM"|"SIGINT"|"SIGHUP")` handlers are the supported Bun/Node way; the handler only calls `controller.abort()`, the loop yields via `setImmediate` per frame, so the signal is seen between frames. `child.kill("SIGTERM")` in the existing vitest case therefore should give exit 130; still unexecuted.
- `repo tools exec` shim: a SIGTERM to the group reaches shim and bun together; if the shim exits first the bun still finishes its cancel cleanly within the 3 s grace, otherwise SIGKILL.

## Open
- Run the `SIGTERM` vitest case and the chain once on Linux/macOS/devcontainer (needs bun there).
- `posix_tree.py` is a ticket-local harness; promote to a repo test if wanted (needs a Linux CI lane).
