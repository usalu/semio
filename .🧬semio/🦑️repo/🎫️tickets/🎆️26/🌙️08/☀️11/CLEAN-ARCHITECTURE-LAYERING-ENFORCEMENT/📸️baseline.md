# Baseline Snapshot — Pre Layering Refactor

Captured 2026-08-11, before the Clean Architecture layering enforcement work begins.

## 1. `bun ./📜️script.ts verify gate`

**Result: FAILED** (exit code 1)

- `[verify] dependency-cruiser boundaries…` → **PASSED** (`✔ no dependency violations found (194 modules, 257 dependencies cruised)`)
- `[verify] generated catalog freshness…` (`nx run @semio-tech/plugin-registry:check`) → **FAILED**
  - Root cause: large number of pre-existing "plugin taxonomy tree violations (area `✏️s/🔌️plugins` is `clean`)" across many apps/artifacts (`✒️writer`, `🪐️space`, `🪵️sourcing`, `🗂️curate`, etc.). Two recurring violation shapes:
    1. `<file> is not declared by any #[path] in 📦️glue.rs`
    2. `📦️glue.rs declares #[path = "..."] but the file does not exist on disk`
  - Also several artifacts reported as missing `🧬️schema/`, `⚙️engine/🦀️component.rs`, `⚙️engine/🟦️component.ts`, `🚪️io/`, `⚙️engine/`.
  - This matches the "known architecture violations" flagged as expected in the task instructions.
- Gate aborted after the plugin-registry check failure (`bun nx run @semio-tech/plugin-registry:check exited with status 1`), so no later gate stages ran.

Full log: `📸️baseline-verify-gate.txt` (5388 lines).

## 2. `cargo check --workspace`

**Result: FAILED**

- Compilation of `semio-compose-rs` fails with 22 errors (plus 823 warnings), tail shows:
  - `error[E0433]: cannot find module or crate 'dsl'` in `compose/client/lib/rs/lib.rs:723`
  - `error[E0433]: cannot find module or crate 'vcs'` in `compose/client/lib/rs/lib.rs:7919`
  - Overall: `error: could not compile 'semio-compose-rs' (lib) due to 22 previous errors; 823 warnings emitted`

Full (tail -200) log: `📸️baseline-cargo-check.txt`.

## 3. Workspace crate count

**94 crates** (via `cargo metadata --no-deps --format-version 1`, counting `packages`).

## 4. `git status --short | head -50`

Captured read-only for reference (not acted upon): `📸️baseline-git-status.txt`. Shows staged additions/modifications from other concurrent sessions' work in `.🦑️repo/🎫️tickets/.../ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/` and several `component.rs` files under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/...`, plus this new ticket folder as untracked.

## Summary

| Check | Status |
|---|---|
| `verify gate` — dependency-cruiser boundaries | PASS |
| `verify gate` — generated catalog freshness (plugin-registry check) | FAIL (pre-existing taxonomy/glue.rs violations) |
| `verify gate` overall | FAIL |
| `cargo check --workspace` | FAIL (22 errors in `semio-compose-rs`: unresolved `dsl`/`vcs` crates) |
| Workspace crates | 94 |

This baseline captures the pre-existing failure state (expected, per task) before the layering refactor begins.
