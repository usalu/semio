# 🚀 Fleet Brief 2026-11-08: Norm Plugin and Artifact Editors End to End

Coordinator: main chat (Opus 5.5). Fleet: 20 parallel agents. Ticket `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS` is already open (do NOT open/close/reopen tickets or goals). Ticket folder `TICKET` = `/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`.

## Objective

Every norm artifact under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/` (EN 1990–1999, DIN 4108/16798/18599, ISO 16757, VDI 3805) works end to end:
1. Its Rust crates (`@semio-tech/norm-<x>-rs`) `:build` and `:test` green through `repo`; its TS package tests green.
2. Its editor and viewer are visual (3D viewport, diagrams, properties, checks, outliner), live (edit → preview → commit → incremental re-evaluation), localized EN+DE, accessible, with no JSON visible in any editor body.
3. Load norm plugin in the hosted `s` app, open the artifact, edit a numeric field, see updated check results, undo/redo, reload and see the edit persisted, in both English and German.
4. Norm-complete per `📐️specs/en*-target.md`, with language-agnostic tests and a third-party oracle cross-check for every feature.

## Must-read (in this order, skim, do not read everything)

- `/home/deck/semio/AGENTS.md` and the nearest `AGENTS.md` files of the paths you edit.
- `TICKET/📌️important/build-loop.md` (build etiquette, pitfalls, source-boundary budget per crate: 262144 bytes / 32 files / 4000 lines).
- `TICKET/📌️important/artifact-rollout-checklist.md` (the EN 1993 reference vertical; what each artifact needs).
- `TICKET/📐️specs/visual-editor-architecture.md` (design), and your artifact's `📐️specs/*-target.md`.
- `TICKET/e2e-current-progress.md`, `TICKET/e2e-coordination.md` (latest end-to-end state, host URL, session-address notes).
- `TICKET/STATUS.md` top section only (the native infrastructure status; very long, do not read all).

## Hard rules

- Never run modifying git commands (commit, stash, checkout, reset, restore, worktree). Others work in the same files: never revert unrelated changes; re-read a file right before editing it; keep edits atomic.
- Never use `repo dashboard`, IDEs, or raw cargo/nx/bazel/bun. Build and test only through `repo`.
- Builds: Bash with `run_in_background: true`, at most ONE build/test of your own at a time, never duplicate a run another agent already has going (check `ps aux | grep 'repo run'`):
  `cd /home/deck/semio && SEMIO_LANGUAGE=en SEMIO_NX_GRAPH_CAPACITY_WAIT_MS=5400000 repo run '<project>:<target>' --output=static > "$LOG" 2>&1; echo "exit $?" >> "$LOG"`
  with `LOG=TICKET/🗑️generated/<your-lane>/<n>.txt`. Capacity/memory waits are normal; do not cancel. Start a build only when `free -m` shows > 4 GiB available. Read errors with `grep -n 'error\[' "$LOG"` plus context.
- Temporary files, scripts and logs only inside `TICKET` (scripts in `TICKET/🧰️scripts/`, logs in `TICKET/🗑️generated/<your-lane>/`). Delete your own tool-output logs when done; keep scripts and markdown reports.
- Temporary debug logs must start with `[DEBUG] ` and be removed before you finish.
- Code: concise, no comments inside definitions, docstrings start with a unique fitting emoji, schema-first, no legacy/compat layers/deprecations/migration scripts, EN+DE strings (no default language), file paths ≤ 240 UTF-8 bytes from repo root.
- Test-driven: add/extend a language-agnostic test (JSON fixtures / `.feature`) for each feature and cross-check with a third-party library oracle (Python oracles via `/home/deck/semio/.venv/bin/python -I`).
- Do not stop to ask questions; decide and act. Do not stop halfway.

## Ownership (one writer per directory)

- `shared` lane alone edits shared norm crates: `📕️norm/📇️registry`, `📕️norm/⚖️compliance`, `📕️norm/🎬️structural-scene`, `📕️norm/🧮️mechanics`, `📕️norm/🧬️twin-dialect`, `📕️norm/🪟️*`, `📕️norm/🏭️bridge`, plugin root `Cargo.toml`. Others append exact compile errors/requests to `TICKET/📌️important/requests.md` under a heading `## fleet-1108 → shared` and SendMessage to "main" a one-line note starting with `[for shared]` (the coordinator relays it); meanwhile continue on their own files. The `shared` lane polls that heading after every build cycle. Trivial type-directed fixes (missing import) in shared crates are allowed if `shared` does not answer within your next build cycle; log them in `TICKET/📌️important/requests.md` under "shared fixes".
- Artifact lanes own only `📕️norm/🗿️artifacts/<their artifact>/**`.
- Framework (`🧰️framework/**`) is owned by other teams; change it only for a proven blocker with a minimal edit, and document it.

## Reporting

- Write your findings/progress to `TICKET/🔍️audits/fleet-1108-<your-lane>.md` (create/update; this is your durable report).
- Your final reply to the coordinator: at most 15 lines: status (green/red per gate with log path and exit code), files changed, remaining blockers, and the report path. Never claim a pass you did not observe.

## Persistence (added by coordinator)

- A queued or running build is NOT a reason to hand back. Queue the build, then keep implementing the next gaps in your lane while it waits; wait for the build's `exit` line with the Monitor tool (until-loop on the log), then fix its errors and re-queue. Hand back only when every gate in your lane is green or truly blocked by something outside your ownership (state exactly what).
- Builds wait in a capacity queue of ~10+ jobs on a 16 GB host; expect 30–90 min per build. Batch many edits per build.

## Build gate (MANDATORY from now, added by coordinator)

The host was saturated (~15 parallel builds, swap full, the shared build died in graph-plan). From now on every `repo run` build/test of a lane goes through the 2-slot gate, never directly:
Heavy builds go through the host-wide scheduler (dedupe by content, fair order between agents, admission by learned peak memory, hard memory limit): `repo run <project:target> --output=static` is scheduled by default, and `repo scheduler run --key=<project:target> --priority=agent --peak-mb=<n> -- <command...>` queues any other heavy command (Bash, `run_in_background: true`; redirect the output into `TICKET/🗑️generated/<lane>/<n>.txt`). Only the `shared` lane may use the `interactive` priority. Light commands (`repo tools exec --light bun -- test <file>`, Python oracles, static scanners) are not builds and need no scheduling.
Do not queue a new build if a run of yours is still waiting/running. Expect long waits: implement meanwhile.

## Requests inbox (added by coordinator)

Whenever you resume and after every build cycle, read your section `## fleet-1108 → <your lane/artifact>` in `TICKET/📌️important/requests.md` and handle every new item there (shared-surface, shared-scene, ndp, oracle and precompile-review lanes file items for you). Mark handled items `[done]` in place.
