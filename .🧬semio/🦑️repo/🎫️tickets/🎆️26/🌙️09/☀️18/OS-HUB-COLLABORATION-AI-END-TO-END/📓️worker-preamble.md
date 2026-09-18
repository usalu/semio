# Worker preamble — OS Hub Collaboration AI End To End (ticket 26/09/18)

Ticket folder: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END`
Read `📓️status.md` (wave table) and the audit memo(s) your slice names before editing.

Rules every worker follows (from AGENTS.md + hard-won incidents):
1. Foreground only. No sub-agents. No background commands (they die when your turn ends). No worktrees, no `isolation`.
2. Never run `git commit`, `git stash`, `git checkout`, `git reset`, `git worktree`. Auto-commit runs elsewhere.
3. Other agents (Claude and Codex peers) edit the same files right now. Re-read a file immediately before editing it, keep edits scoped to your slice, and never stop or "wait for the peer" — keep working on your task. A vanished stub may be a peer's deletion: diff before assuming.
4. Do NOT close, reopen or edit `🎫️ticket.json`. Do NOT delete `🗑️generated` or anything in it you did not create. Do NOT touch `📌️important.md`.
5. Command captures go to `🗑️generated/<slice>-*.txt` (`.txt`, never `.log`). Scratch scripts/probes go into the ticket folder named `🐍️<slice>-*.mjs|ts|py` or `📜️<slice>-*.sh`. Nothing ad-hoc outside the ticket folder.
6. Your final deliverable is a markdown report `📓️<slice>-<topic>.md` in the ticket folder: what was measured, what was fixed (file + line), what is verified at runtime vs. by tests only, honest gaps, files changed. In chat return ≤10 lines pointing at the report. Never claim a test passes or a feature works unless you ran it and captured the output.
7. Cargo etiquette: always `-p <crate>` (never workspace-wide), at most ONE cargo process from you at a time, builds run in foreground, macOS has no `timeout`. A shared build-dir lock is normal: a 0 % CPU cargo with a live `rustc` child is working, do not kill peers' cargo. For wasm builds export `CARGO_PROFILE_WASM_DEV_DEBUG=false`. Plugin crates need `--features component-app-assembly`; cfg(wasm32) code only compiles with `--target wasm32-wasip2` / `wasm32-unknown-unknown`.
8. Servers: activate once, serve detached with `nohup … > 🗑️generated/<slice>-serve.txt 2>&1 & disown`, poll with `curl`, probe headless (playwright via existing probe patterns in this folder). Kill only the processes you started (by pid).
9. Toolchain: `bun` + `nx`; permanent scripts live in the nearest `📜️script.ts` and are registered in `📋️project.json`; every runnable command gets a `.vscode/launch.json` row following the existing order/grouping/naming.
10. No legacy shims, no compat layers, no deprecations, no migration scripts. Fix the root. No `[DEBUG]` logs left behind. Docstrings start with a unique emoji; no comments inside definitions.
11. Repo MCP is unavailable; ticket bookkeeping is on disk. Do not use the broken search tool; use grep.
12. A previous worker on your slice died mid-way when its parent session ended. First run `git status --short` / `git diff --stat` on your slice's paths and read any `🗑️generated/<slice>-*` captures to inherit partial progress instead of redoing it.
