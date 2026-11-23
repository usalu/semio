# 📏 Agent Rules for Get FEM Plugin Working End to End

Ticket folder: `C:\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️22\GET-FEM-PLUGIN-WORKING-END-TO-END` (call it `$T`).

- Work ONLY in `C:\semio` (the live shared worktree). `C:\git\semio` is a stale checkout: never edit or build there.
- Read `C:\semio\AGENTS.md` and obey it. Key points: no git command that changes the repo/worktree (no commit/stash/checkout/reset/restore/clean); others edit the same files live, so re-read a file right before editing it; no compat layers, no deprecations, no legacy; concise code, no comments inside definitions; docstrings start with a unique emoji.
- Build/test only through the `repo` binary (`repo cargo --store=shared --configuration=prototype -- <cargo args>` run from the workspace directory containing the Cargo.toml; `repo run <project>:<target>`; `repo tools exec <tool> -- <args>`). If `repo` is stuck or broken for more than ~15 minutes on one step, fall back to the native tool (`cargo`, `bun`) and say so in your report. Never get stuck waiting.
- Run long commands in the background and write their output to `$T/🗑️generated/<your-agent-name>-<step>.log` (unique names; never delete other agents' logs; never `rm *.log`).
- Temporary debug prints must start with `[DEBUG] ` and be removed before you finish.
- Do NOT close, reopen or open tickets or goals. Only the coordinator does.
- Do NOT touch files outside your assigned scope unless it is the root cause of a failure in your scope (e.g. a framework API your crate calls); if you fix a framework root cause, keep the fix minimal and correct and list it.
- Windows traps: MSVC path limit and >32K command lines on big builds (use the shared store as repo selects it); Bash heredocs / `node -e` strip backslashes (write scripts as files with the Write tool); `python -` on stdin hangs (run a file).
- NEVER kill by image name or command-line pattern (`taskkill /IM repo.exe`, matching `cargo.exe` by crate name): other sessions, the fleet replica and dev servers share this host. Kill only PIDs you started yourself (record them).
- Native cargo fallback env: `CARGO_BUILD_RUSTC=rustc`, `RUSTC_WRAPPER=` (empty), `CARGO_TARGET_DIR=C:\t\<you>`, `CARGO_BUILD_BUILD_DIR=C:\t\<you>b`, `TMP`/`TEMP`/`TMPDIR=C:\t\tmp`, `CARGO_INCREMENTAL=0`, `CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0` (debug info made the 2D target 65 GB), `-j 6`.
- HOST RAM (64 GB): MSVC link.exe of FEM editor test binaries takes 8–13 GB each; four at once starved the fleet replica sync for 15 h (2026-11-22). Use `-j 3`, build only the single test target you need, never run two of your own builds at once. Owner rule: never use any drive other than C: (no G:, E:, J:, …) for builds, temp files or caches. C: is NVMe with ~120 GB free; the coordinator watches it. Silent "could not compile" with only warnings means os error 112 (disk full).
- Kill processes you start before finishing (`taskkill /T /F /PID <pid>` for trees).
- Finish with a markdown report at `$T/<your-report-name>.md`: what you ran, pass/fail counts, what you fixed (file list), what is still open and why. Your final chat message: 5 lines max + the report path.
