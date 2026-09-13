# 🪣️ Interactive tools with visible process — status

Session start 2026-09-13 (Fable 5.1 main chat). Repo MCP failed to connect (`invalid initialize params`), so the
ticket folder is managed on disk; goal association `🎯r2603` mirrors the other September tickets.

## Goal (dev's words, condensed)

- Every tool is interactive; the user sees how the algorithm thinks (steps, percentage, status, intermediate results).
- Puzzle 3d fill: no hidden precompute-then-reveal. Count slider unbounded, arbitrarily settable, default 100. The UI
  builds the solution live: every tried object is shown (danger when colliding, highlight when collision free), locked
  objects appear as they are locked.

## Phase 1 — audits (Sonnet fleet, read-only)

| Report | Scope |
|---|---|
| `📓️audit-fill-pipeline.md` | FillBuilder stages, publications, job bridge, count apply, FILL_COUNT_MAX sites, RNG prefix property |
| `📓️audit-viewport-render-path.md` | Rust render → instances → r3f; per-instance color; brush ghost pattern; reveal cutoff |
| `📓️audit-progress-primitives.md` | WindowMeasure variants, jobs/ticks, other plugins' progress UX, schema-first gap |
| `📓️audit-tests-and-deploy.md` | fill tests, oracle convention, cargo commands, deploy chain to :6013, browser probe |
| `📓️audit-tool-inventory.md` | repo-wide tool classification A–E, top-10 conversion list |

Baseline `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` → `🗑️generated/baseline-check.txt`.

## Phase 2 — plan (this chat) → `📋️master-plan.md`

## Phase 3 — implementation waves (Opus fleet)

## Phase 2 — plan written: `📋️master-plan.md` (decisions §1, binding contract §2, waves §3, gates §4)

Audit takeaways: the planner already steps per candidate and publishes a ghost + counters, but (1) it always plans to a hidden
1000, (2) accepted pieces only become document objects at slider commit via a separate synchronous replan, (3) the ghost
never carries a collision verdict, (4) the framework has no unbounded numeric or progress measure.

## Phase 3 — implementation waves launched 2026-09-13 (Opus 5, parallel)

| Wave | Scope | Report |
|---|---|---|
| A | planner: requested-count target, verdict ring, stall reasons, preview wire + fixture/oracle | `📓️wave-A-planner.md` |
| B1 | session/job bridge: requested count to the worker, locked-chunk API, summary counters | `📓️wave-B1-session.md` |
| B2 | commands/config: setFillCount rewrite, tick commits locked pieces, default 100, ghost tail removed | `📓️wave-B2-commands.md` |
| C | viewport/UI: danger/highlight ghosts, tried ring, HUD, Number/Progress measures, reveal removal | `📓️wave-C-viewport.md` |
| D | puzzle 2d/5d parity: unbounded default 100, 5d progress/cancel | `📓️wave-D-2d-5d.md` |
| F | framework `WindowMeasure::{Number,Progress}` end to end | `📓️wave-F-framework-measures.md` |
| G | brush suggestions stream tested/free/blocked with ghost verdicts | `📓️wave-G-brush.md` |
| E | policy predicates in root `📜️script.ts` (after A/B/C) | `📓️wave-E-policy.md` |
| H | probe step, deploy, runtime evidence (after all) | `📓️wave-H-verification.md` |
