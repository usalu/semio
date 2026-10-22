# Wave 1022a brief (supersedes the stage-only rules of 1020d/1020e/1021a)

User (10-22 01:30): get the codebase completely running again. The architecture course (`@teaching/architecture-course`), semio-tech play and the mit-bestand demonstrator must run end to end. Other agents (Claude and Codex) work on the same files at the same time: do not stop for them, keep going on your own task.

Start state: `HANDOVER-1021.md` and the tail of `coordination.md`. Results of earlier executors: `result-*.md`.

## Rules (read AGENTS.md too)
- LIVE EDITS ARE ALLOWED (no more staging + GO). Make small anchored edits, re-read a file right before you edit it, never revert other people's changes. Greenfield: no compat layers, no deprecations, no re-exports for compatibility; concise code, no comments inside definitions, docstrings start with a unique emoji; schema-first; EN/DE texts; cross-platform (macOS, Linux, Windows, devcontainer); progress + cancellation for expensive operations; every feature gets a language-agnostic test.
- Never run a modifying git command (commit/stash/checkout/reset/restore/add/worktree). No sub-agents.
- `cd /Users/ueli/Documents/semio` in every Bash call. The shell `grep` is ugrep and misses matches on emoji paths: use `/usr/bin/grep` or `rg`. Emoji path segments carry U+FE0F: discover names with `ls`. Paths < 256 chars. macOS has no `timeout` command.
- All builds/tests through the repo binary: `bun repo run <project>:<target> [--configuration=…]`, `bun repo --tool=bazel run …`, `bun repo cargo --store=shared --configuration=prototype -- <cargo args>`. Raw cargo/rustc are refused by the ingress gate; never set RUSTC or CARGO_TARGET_DIR (no private target dirs). `bun test` only on explicit files (`bun test ./<file>`), never `bun test ./` or a bare filter (ENFILE). At most one heavy process per agent; prefix with `nice -n 10`.
- Long commands (> 9 min): launch detached with `python3 C/chain-tools/detach.py <log> <cmd…>` (prints the pid; survives your turn), record pid + log in your result file, then wait in the FOREGROUND with bounded loops (`for i in $(seq 1 55); do kill -0 <pid> 2>/dev/null || break; sleep 10; done`). Never `run_in_background`, never Monitor. Before you report, kill every process you started that is still waiting (child first, then wrapper) unless it is a deliberate detached run you hand over (say so in the result).
- If a run waits on a lease ("behind the request of process N"), read N's command line; do not kill processes you did not start. Codex pid 37466 (`test-producer-source`, running since 10-21) is a peer: leave it, but report if it blocks you.
- Generated output under `C/🗑️generated/<prefix>-*` only; scripts under `C/<prefix>-tools/`. Never delete `🗑️generated` folders or other people's files.
- Keep `C/result-<prefix>.md` current while you work (a 5-hour usage cut can stop you at any moment): changed files, exact commands + outcome, what is open. "WRITTEN BUT UNVERIFIED" is an acceptable outcome; say it plainly. Final message ≤ 8 lines. Do not close or reopen the ticket.

`C` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`.
E2E driver: `sh C/e2e-orch-tools/run.sh <course|play|demo|all> [--phase …] --go --run-id <id>` (preflight/report are light; cold/warm/e2e need --go). Launch it detached with detach.py, report in `C/report-e2e-orch-<id>.md`.

## Addendum 12:40
- NEVER use `pkill`, `killall` or `pgrep -n`-style kills: kill only pids you recorded when you started them (two agents killed other agents' / the coordinator's processes this way today).
- Graph-needing commands (`show projects|targets`, `run …`): check `bun repo leases status` first; if a coordinator-owned graph rebuild holds `repository-graph-facts` exclusively, wait for it instead of queueing behind it.

## Wave 1023a addendum (00:10, weekly quota 91 % → final focused wave; wind-down at ~97 %)
- Resume point: `C/HANDOVER-1022.md`. Each executor owns ONE critical-path item; keep `C/result-<prefix>.md` current every ~20 min — a quota stop can come any time.
- Cargo: one cargo process per agent; capacity waits of 10-30 min are normal — wait, don't spawn more.
