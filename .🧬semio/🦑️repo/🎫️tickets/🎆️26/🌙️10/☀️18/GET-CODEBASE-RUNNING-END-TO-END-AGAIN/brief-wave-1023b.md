# Wave 1023b brief (10-23 02:10, weekly + 5-h usage reset to 0 %)

User (10-23 02:00): get the codebase completely running again. The dashboard, the architecture course (`@teaching/architecture-course`), semio-tech play (`@semio-tech/semio-tech-play`), the mit-bestand demonstrator (`@semio-tech/mit-bestand-demonstrator`) etc. must run end to end. Other agents (Claude, Codex, Cursor) work on the same files at the same time: do not stop for them, keep going on your own task.

`C` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`.
Start state: `C/HANDOVER-1022.md`, then `C/coordination.md` from "## 00:02 (10/23)" and "## 02:10 (10/23", then the `C/result-*.md` files named in your own brief. All rules of `C/brief-wave-1022a.md` (incl. the 12:40 + 1023a addenda) still hold; the most important ones are repeated here.

## Rules (read AGENTS.md too)
- LIVE EDITS. Small anchored edits, re-read a file right before you edit it, never revert other people's changes. Greenfield: no compat layers, deprecations or compatibility re-exports; concise code, no comments inside definitions, docstrings start with a unique emoji; schema-first; EN/DE texts; cross-platform; progress + cancellation for expensive operations; every feature/fix gets a language-agnostic test (+ an independent oracle where it fits).
- Never run a modifying git command (commit/stash/checkout/reset/restore/add/worktree). No sub-agents. Do not close or reopen the ticket.
- `cd /Users/ueli/Documents/semio` in every Bash call. The shell is zsh: quote separators (`echo '---'`, never a bare `===`). The shell `grep` is ugrep and misses matches on emoji paths: use `/usr/bin/grep` or `rg`. Emoji path segments carry U+FE0F: discover names with `ls`. Paths < 256 chars. macOS has no `timeout`.
- Builds/tests only through the repo binary: `bun repo run <project>:<target> [--configuration=…]`, `bun repo --tool=bazel run …`, `bun repo --tool=nx run …`, `bun repo cargo --store=shared --configuration=prototype -- <cargo args>`. Raw cargo/rustc are refused; never set RUSTC or CARGO_TARGET_DIR. `bun test ./<file>` only on explicit files (never `bun test ./` or a bare filter). One heavy process per agent, prefixed `nice -n 10`. Capacity waits of 10-30 min are normal: wait, never spawn more.
- Long commands (> 9 min): launch with `python3 C/chain-tools/detach.py <log> <cmd…>` (Terminal.app launch, survives app restarts, prints the pid), record pid + log in your result file, then wait in the FOREGROUND with bounded loops (`for i in $(seq 1 55); do kill -0 <pid> 2>/dev/null || break; sleep 10; done`). Never `run_in_background`, never Monitor. Before you report, kill every process you started that is still waiting (child first, then wrapper) unless it is a deliberate handover (say so).
- NEVER `pkill`, `killall`, `pgrep -n`-style kills: kill only pids you recorded. Graph-needing commands (`show projects|targets`, `run …`): check `bun repo leases status` first. If a run waits "behind the request of process N", read N's command line and wait; do not kill other people's processes (tell the coordinator in your result file if something blocks you > 30 min).
- At 02:05 an orphan `show projects` (pid 77841, from the previous session) is rebuilding the graph generation and holds `repository-graph-facts` shard 7 exclusively; queue behind it, do not kill it.
- Peers moved `🧰️framework/🔨️modules/🏗️build` → `🧰️framework/🔨️modules/🏃️process/🏗️build` around 01:26; if a module path is dangling, look for the moved file and fix the importer at the source (no re-export shims).
- Output under `C/🗑️generated/<prefix>/` only; scripts under `C/<prefix>-tools/`. Never delete `🗑️generated` folders or other people's files. Temporary logs carry the `[DEBUG] ` prefix and are removed before you finish.
- Keep `C/result-<prefix>.md` current (newest-first status line at the top, every ~20 min; a usage cut can stop you any time): changed files, exact commands + outcome, what is open. "WRITTEN BUT UNVERIFIED" is an acceptable outcome; say it plainly. Final message ≤ 8 lines, pointing to your result file.
- E2E driver: `sh C/e2e-orch-tools/run.sh <course|play|demo|all> [--phase preflight|cold|warm|e2e|probe|report|all] --go --run-id <id>` (launch detached; report in `C/report-e2e-orch-<id>.md`). Ports: course 6061/6063/8791/8793, play 6033, demo 6029.
- Dashboard: AGENTS.md forbids agents from USING `repo dashboard` as a dev tool. Only the dashboard executor builds, tests and smoke-launches it (headless, then stops it) to prove it runs.

## Addendum 04:55
- NEVER walk the tree with Python `glob('**', recursive=True)`, `os.walk(followlinks=True)` or `find -L`: symlink cycles in node_modules / Bazel / cache dirs made two agent scans grow to 40 GB + 2.9 GB for 2 h (swap 16 GB, capacity gate starved). Use `rg --files`, `git ls-files` or `fd` (they skip ignored dirs and do not follow links). A tool call that times out leaves its process running: kill it yourself (record pid) before the next call.

## Addendum 07:50 (USER: no Terminal windows, everything headless)
- `C/chain-tools/detach.py` is now HEADLESS (own session, no window, scrubbed env). Never start anything through Terminal.app, `open -a`, `osascript` or any GUI app; servers and browsers run headless only (Playwright headless, no `open http://…`). A desktop-app update restart may kill detached runs: just rerun the same run-id.
- Runs already started in a Terminal window keep running there; do not close windows. New launches must use the headless detach.py.

## Addendum 10:20 (pid reuse)
- macOS reuses pids. Before you kill a recorded pid, check that it is still YOUR process: `ps -o lstart=,command= -p <pid>` must match the start time and command you recorded. If it doesn't match, do not kill. (An outside SIGTERM killed play's `cc` linker at 09:33 after 47 min of compile.)
