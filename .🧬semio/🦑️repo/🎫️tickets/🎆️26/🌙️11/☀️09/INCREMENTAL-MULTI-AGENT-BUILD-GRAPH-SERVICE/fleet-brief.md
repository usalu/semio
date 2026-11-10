# ⚡ Fleet Brief: Incremental Multi-Agent Build Graph Service

Ticket `26/11/09/INCREMENTAL-MULTI-AGENT-BUILD-GRAPH-SERVICE` (goal `aioptimizedrepo`), already open — do not open/close/reopen tickets or goals. `TICKET` = `/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/INCREMENTAL-MULTI-AGENT-BUILD-GRAPH-SERVICE`. `BUILD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`.

## Goal

Developing must be incredibly fast for many agents and workspaces editing the monorepo at the same time, on a 16 GB RAM / 200 GB disk host, with bounded resources (no RAM/swap/cache exhaustion, no crashes over long sessions). Target design: `/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🔍️audits/fleet-1108-build-graph-design-review.md` (read it first). Background on today's mitigations: `.../NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🔍️audits/fleet-1108-infra.md`.

Success metrics (measured by the bench harness, see the `bench` lane): no-op re-plan after a one-file edit in seconds, not minutes; cold plan of one artifact project bounded by its own closure; planner/daemon peak RSS ≤ 3 GB total on the host regardless of agent count; zero OOM kills; disk store capped and GC'd.

## Must-read

- `/home/deck/semio/AGENTS.md` and nearest `AGENTS.md` of every path you edit.
- `TICKET/🔍️audits/architecture-map.md` (written by the `architect` lane; wait for it if you are an implementation lane and it is missing).
- Your lane section in `TICKET/🔍️audits/implementation-plan.md` (also from `architect`).

## Hard rules

- No modifying git commands; others work in the same files; re-read before editing; atomic edits.
- No `repo dashboard`, no IDEs, no raw cargo/nx/bazel. Prefer light verification: `repo tools exec --light bun -- test ./<path>` (the `./` prefix is required), Python via `/home/deck/semio/.venv/bin/python -I`.
- Heavy `repo run` builds go through the host-wide scheduler, which is the default of `repo run`: `repo scheduler run --key=<project:target> --peak-mb=<n> -- repo run <project:target> --output=static > "TICKET/🗑️generated/<lane>/<n>.txt" 2>&1` (dedupe, fairness, admission, hard limits), one at a time per lane, only when the lane's light tests are green. Never kill/pause other processes.
- Schema-first, language-agnostic tests (JSON fixtures / `.feature`) plus an independent oracle where applicable; concise code, no comments inside definitions, docstrings start with a unique emoji; EN+DE user-facing strings; no legacy, no compat layers, no migration scripts; replace old mechanisms completely.
- Everything async, cancellable, with progress for expensive operations. Bounded memory everywhere (no unbounded maps in long-lived processes).
- Temporary files only under `TICKET/` (`🧰️scripts/`, `🗑️generated/<lane>/`); delete your tool-output logs when done; keep scripts and markdown reports.
- Do not stop to ask; do not stop halfway; hand back only when your lane is done or truly blocked by something outside your ownership.

## Ownership

Each lane owns the directories named in its implementation-plan section; cross-lane needs go to `TICKET/📌️requests.md` under `## → <lane>` plus a one-line SendMessage to "main".

## Reporting

`TICKET/🔍️audits/<lane>.md` (durable). Final reply ≤ 15 lines: what is verified (exact command and pass counts), files changed, remaining blockers.
