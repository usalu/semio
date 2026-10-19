# Brief: General Executor (Get Codebase Running End To End Again)

Repo `/Users/ueli/Documents/semio` (macOS arm64, zsh). Coordinator ticket `C` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`. Read `C/coordination.md` first (current state of the build CLI) and the audit reports `C/audit-*.md` relevant to you.

## Hard rules
- Follow `AGENTS.md` (read it). Never run a modifying git command (commit/stash/checkout/reset/restore/add). No worktrees. No sub-agents. Run every command in the FOREGROUND (background children die when you stop). Prefix temporary logs with `[DEBUG] ` and remove them before finishing.
- `cd /Users/ueli/Documents/semio` in every Bash call. The shell `grep` is aliased to ugrep and misses matches on emoji paths: use `command grep` or `/usr/bin/grep`. Emoji path segments carry U+FE0F; discover names with `ls`.
- Other agents (Claude and Codex fleets) edit the repo concurrently. Re-read files right before editing; use small anchored edits in shared files. Never revert others' changes. Never delete `🗑️generated` folders or others' files.
- Builds: the only entry is `bun repo …` (`bun repo run <project>:<target>`, `bun repo cargo --store=shared --configuration=prototype -- <cargo args>`). Raw `cargo`/`rustc` are refused by the compiler ingress gate (`.cargo/config.toml`). Unless your task says you own a build step, do NOT start builds: the coordinator serializes them (disk has ~55 GB free, RAM is limited, concurrent cargo builds have caused kernel panics).
- Disk: never create private cargo target dirs. Keep generated output under your ticket area `C/🗑️generated/<prefix>-*`.
- No legacy/compat layers, no deprecations, no re-exports for compatibility (greenfield). Concise code, no comments inside definitions, docstrings start with a unique emoji. Schema-first. Every feature needs a language-agnostic test.
- Path length < 256 chars.

## Report
Write `C/result-<prefix>.md`: what you changed (file list), how you verified (exact commands + outcome), what is still open. Final message: max 8 lines.
- LAUNCH: `.vscode/🧩️launch.seed.jsonc` is the only authored launch file; `.vscode/launch.json` is generated (`workspace:generate-launch`). Edit the seed, never launch.json. Modules moved/renamed: also re-point static imports in the root `📜️script.ts` (verify with `bun build --target=bun 📜️script.ts`).
