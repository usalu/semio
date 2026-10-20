# 🏗️ Fleet Brief: Architecture Course on Bazel

Ticket folder: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL` (below: `T`).

## Goal

`@teaching/architecture-course` and its dependency closure (proctor `@teaching/proctor`, course product, pets, `@semio-tech/assets:build`, `@semio-tech/ui-styling-tokens:generate`, generated bindings) run and build efficiently through `bun repo --tool=bazel run <project>:<target>`:

- every course target (`build`, `test`, `typecheck`, `check`, `dev`, `dev-site`, `publish`, `test-e2e`, `preflight`, deploy verbs) executes through the Bazel backend on macOS, Linux and Windows
- warm rebuilds are cache hits, and an edit invalidates only the affected actions
- progress and cancellation work

## Rules (all agents)

- Read `AGENTS.md` at the repo root first and follow it. Never edit `AGENTS.md`.
- Never use modifying git commands (commit, stash, checkout, reset, worktree). Never use `isolation: worktree`.
- Never call `nx`, `bazel`, raw `cargo` or `rustc`. Use `bun repo …` (`bun repo --tool=bazel run …`) and `bun repo cargo --store=shared … -- <cmd>`.
- Tests: only explicit files (`bun test ./<file>`), at most 2 concurrent. Never run `bun test ./` or a bare filter, because it exhausts kern.maxfiles.
- Long commands: run them in the FOREGROUND of your own Bash call with a generous timeout (max 600000 ms), or detach with `nohup … > log 2>&1 &` and poll the log. Background children of a subagent die when it ends.
- Do not start dev servers that must outlive you. The coordinator starts them.
- Put every temporary file, script and log under `T/🗑️generated/<your-name>/`, and inputs you want to keep (scripts, configs) under `T/<your-name>-tools/`.
- Write your report to `T/result-<your-name>.md` and keep it current while you work. A usage cut must leave a usable record.
- Other agents (Claude and Codex) edit the same files at the same time. Do not stop because of that. Re-read a file right before you edit it, keep edits small and compile-atomic, and never revert changes you did not make.
- The generic Bazel backend (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/**`) is actively changed by the peer fleet of ticket 26/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN (wave 1020b). Course-specific fixes are ours. Fix a generic backend defect only when it blocks the course, keep the change minimal, and record it in your result file under a "Backend edits" heading.
- Do not close, reopen or create tickets. Do not delete `T/🗑️generated` or other agents' files.
- Code style: concise, no comments inside definitions, docstrings start with a unique emoji. Schema-first, test-driven, with a language-agnostic fixture and a third-party oracle for every new feature. Support English and German for user-facing text.
- NEVER hard-link or clonefile repo sources (ln, cp -l, cp -c): the Source revision reader refuses nlink!=1 and every bun run in the repo breaks. Read files in place or byte-copy.
- Bazel 8.4.2 on darwin double-encodes non-ASCII paths in its sandbox profile. output_base and install_base must stay ASCII. Emoji are fine in target labels, not in package names.

## Peer Freeze (until the peer's integrate series finishes, round 1 ≈ 16:15, then round 2; coordinator lifts it)

Do NOT edit, only stage under `T/<your-name>-tools/staged/` with an `apply` script:
- the Bazel adapter `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/**` and the nx adapter next to it
- `📚️library/🏗️build/🏃️execution/**` (schemas `💾️cache.json`, `🌐️remote-cache.json`), the build `🔣️policy.json`
- `📚️library/🏗️build/🕸️graph/📥️inference/**`, the build `⚙️operations/` catalogue
- the root `📜️script.ts` and the CLI command registry, `📚️library/🔣️taxonomy.json`, `🕸️dependencies/🧭️direction/**`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/**`, the storage/native-image layers and the cargo tool layer
- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json` and `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json` (peer uncached-owners rewrites them: course `build` → managed-output, `check` → cached)
Never run `bun repo dependencies sync|lock` or `bun repo show projects`. Short `bun repo --tool=bazel run` on course targets are fine.
Course sources (`🎓️teaching/**` except the two manifests), course scripts, tests, Vite config, deploy and e2e code are free to edit.

## Live Status (coordinator)
- 16:28: "bun repo boots". QUIET TREE until coordination.md says show is green: only short read-only `bun repo … --dry-run` runs, NO live repo edits (stage everything under your tools folder), no builds or tests that write outside T.
