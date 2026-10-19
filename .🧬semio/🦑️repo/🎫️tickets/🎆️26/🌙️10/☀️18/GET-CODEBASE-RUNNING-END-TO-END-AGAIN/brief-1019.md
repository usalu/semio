# Brief 2026-10-19: Executors (Get Codebase Running End To End Again)

Repo `/Users/ueli/Documents/semio` (macOS arm64, zsh, 10 cores). C = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`.

## Goal of the session
Tool-neutral build system (schema-first) with two implementations: nx + bun (default) and Bazel. Bazel must run EVERY task and use Bazel's own action cache, disk/remote cache, sandboxing and lazy minimal builds (only what the requested target needs). Products end to end: course `@teaching/architecture-course` (renamed from architecture-quiz at 12:21), `@teaching/proctor`, `@semio-tech/semio-tech-play` (:6033), `@semio-tech/mit-bestand-demonstrator`.

## Read first
`AGENTS.md`; `C/brief-executor.md` (hard rules); the audits `C/audit-bazel-state-1019.md`, `C/audit-neutral-build-1019.md`, `C/audit-products-1019.md`; tail of `C/coordination.md`.

## Hard rules (summary; brief-executor.md wins)
- No modifying git commands, no worktrees, no sub-agents, foreground only. `cd /Users/ueli/Documents/semio` in every Bash call. Use `command grep` (shell grep is ugrep and misses emoji paths). Discover emoji names with `ls`.
- Other fleets (Claude and codex) edit concurrently. Re-read right before editing; small anchored edits; never revert others' work. Do not edit `package.json`/`bun.lock` unless your task says so (deps owner only). Do not touch `.vscode/` (retired by the dashboard control-plane work; user decision pending).
- Builds and long runs: do NOT start `bun repo dependencies sync`, `bun repo show projects`, cargo builds or product activations unless your task says so. The coordinator serializes them. Short checks (`bun test <file>`, `bun build --target=bun <file>`, `bun repo --help`, targeted tsc) are fine. macOS has no `timeout`: use `perl -e 'alarm N; exec @ARGV' -- <cmd>`; a single Bash call must finish in under 10 minutes.
- Greenfield: no compat layers, no deprecations, no re-exports for compatibility. Concise code, no comments inside definitions, docstrings start with a unique emoji. Schema-first. Every feature gets a language-agnostic test (fixture/schema-driven) plus a third-party cross-check where feasible.
- Paths < 256 chars. Temporary output only under `C/🗑️generated/<prefix>-*`. `[DEBUG] ` prefix on temporary logs; remove them before you finish.
- Moving/renaming modules: re-point static imports in the root `📜️script.ts` too (verify `bun build --target=bun 📜️script.ts`).

## Report
Write `C/result-<prefix>.md`: changes (file list), verification (exact commands + outcome; say "not run" when not run), open items. Final message ≤ 8 lines.
- FILE TABLE: never run `bun test ./`, `bun test <dir>`, or any test runner without explicit file paths; at most 2 concurrent test processes. A whole-tree `bun test ./` opened 122,880 files and exhausted the system file table (ENFILE), killing the dependency sync.
- CLI CLOSURE: imports inside the repo CLI / build-tool runtime closure (root `📜️script.ts`, `🏗️build/**`, `⚡️caching/**`, `🔧️tools/**`, bootstrap) stay RELATIVE (zero-touch: `bun repo --help` without node_modules; scanner `C/scan-bare-imports.ts`). Never route runtime code through barrels (e.g. `@semio-tech/repo-lib/storage`) that pull build-time deps like `typescript`.
- HARD FREEZE (19:45 until the coordinator lifts it): no edits to ANY `package.json`/`bun.lock`, to anything under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/` (build tool, caching, storage, dependencies, native-build), to the root `📜️script.ts`, or to `🔣️taxonomy.json` files. The dependency sync loads these and restarts its 25-minute walk on every change. Queue such edits and apply them after the lift.
- BUN TEST PATHS: always prefix the path with `./` (`bun test ./🧰️framework/...`). Without `./` bun treats the argument as a FILTER and scans the whole repository (121k fds observed 02:00 → ENFILE kills the sync).
