# Round 2 Fleet Brief: Dashboard As The Only Control Plane

Ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT` reopened 2026-10-08. Owner request:

> dashboard must be the only control plane for devs. The current implementation is incomplete and buggy.
> Dissolve all launch.json from vscode, claude code, etc. Canonicalize configs, mechanisms, etc for
> everything to be declarative, clean. Everything end to end and battle tested for the complete monorepo.

The binding target design is `fleet-plan.md` (same folder) §1–§6. Round 1 executed parts of it; round 2
finds what is missing or broken and finishes it.

## Paths

- Ticket folder `T` = `C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️09\☀️23\DASHBOARD-LAUNCH-COCKPIT`
- Dashboard `D` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`
- TUI framework = `🧰️framework/🔨️modules/🖱️ui/⌨️tui`
- Repo CLI = `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli`
- Launch files to dissolve: `.vscode/launch.json` (~70k lines), `.vscode/🧩️launch.seed.jsonc`, `.claude/launch.json`

## Rules For Every Agent

1. Windows host. Use the Bash tool with POSIX syntax. Always `cd /c/git/semio` first and use
   `git -c core.quotepath=false …`. Paths contain emoji: quote them.
2. Do not use the Grep/Glob tools (broken in this repo); search with `git grep -n` or `rg` through Bash.
3. Never run git-modifying commands (commit, stash, checkout, reset, restore, worktree). Never edit `AGENTS.md`.
4. Other agents and developers edit the same files concurrently. Ignore unrelated changes; never revert them.
5. Write files only inside `T` (reports `r2-*.md`, helper scripts) unless your brief says you own source files.
   Generated output (logs, json dumps) goes to `T/🗑️generated/`.
6. No daemon start/stop on the real workspace and no dashboard install unless your brief says so.
   Cargo builds/tests use `CARGO_TARGET_DIR=/c/git/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-<your slice>`.
7. Report only verified facts; mark anything not run as "UNVERIFIED". Cite `file:line`.
8. Final answer to the coordinator: at most 25 lines, pointing to your report file.
