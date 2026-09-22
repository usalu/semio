# G18 — Tree compile health (native crates + os product TypeScript/vitest)

Auditor G18, 2026-09-22. Scope: measure compile health of the whole tree the four outcomes
depend on (421 uncommitted paths, several peers editing concurrently). No source edits, no
servers, no sub-agents, one cargo process at a time, `CARGO_INCREMENTAL=0` +
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-g18`, captures in `🗑️generated/g18-*.txt`.

## Method
- Each native crate: `cargo check -p <crate>` (plugin crates: `--features component-app-assembly`),
  tail ≤300 lines kept, `grep -c '^error'` recorded. A run with 0 warnings and no `Checking <crate>`
  line proves nothing (cache hit) — flagged explicitly where it happens.
- TypeScript: os product typecheck verb from `📋️project.json`.
- Vitest: projects registered for the os product; smallest set covering `🖥️host` run.
- Churn: `git status --short` bucketed by top-3 path segments; `git log -1 --format=%cd` for
  auto-commit recency.

## Crate results

(filling)

## TypeScript

(filling)

## Vitest

(filling)

## Uncommitted churn buckets

(filling)

## Auto-commit recency

(filling)

## Ranked breaks blocking a running slice

(filling)
