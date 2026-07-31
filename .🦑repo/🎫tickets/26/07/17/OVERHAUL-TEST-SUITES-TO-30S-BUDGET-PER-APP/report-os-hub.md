# os-hub test-suite overhaul report

## Status: SKIPPED — pre-existing compile failure, unrelated to os-hub

`cargo build --tests -p os-hub` fails at the workspace-resolution stage, before any
os-hub code compiles:

```
error: multiple workspace roots found in the same workspace:
  /Users/ueli/Documents/semio/compose/client/bin/store/rs
  /Users/ueli/Documents/semio
```

Root cause: `compose/client/bin/store/rs/Cargo.toml` currently has an uncommitted,
in-progress `[workspace]` table added at its top (confirmed via `git status`/`git diff`,
not authored by this session):

```diff
+[workspace]
+
 [package]
 name = "compose-store"
```

The root `Cargo.toml` lists `compose/client/bin/store/rs` as a workspace member
(line ~124), so once that crate declares its own `[workspace]` table, Cargo sees two
competing workspace roots and refuses to resolve *any* package in the tree — including
`os-hub`, which has no dependency on `compose` at all.

This matches the documented pattern of another concurrent session mid-refactor
(likely the `GENERALIZE-APPS-ONTO-FRAMEWORK-PRIMITIVES` / plugin-SDK extraction work
visible in git status for this same session). Per the unit instructions, I did not
attempt to fix foreign/unrelated code and did not touch `compose/client/bin/store/rs`
or the root `Cargo.toml`.

## What was inspected before stopping

- `framework/product/os/hub/project.json` — `test` target calls `bun ./script.ts test`.
- `framework/product/os/hub/script.ts` — `TestScript` calls
  `runCargo(["test", "--manifest-path", "rs/Cargo.toml", ...segments], this.root)`
  directly (NOT yet routed through `runCargoTestBudgeted`). This migration is still
  needed once the workspace is unblocked.
- `framework/product/os/hub/rs/bin.rs` — single in-source `#[cfg(test)] mod tests`
  (lines 784-1087) with 9 `#[tokio::test]` cases: WS duplex fan-out, sqlite-file
  persistence round-trip, op-id dedupe, snapshot CAS conflict, concurrent-append
  version-conflict fix, REST append/version, REST `ops?since=` filter, VFS node
  create/list, and share-token auth gating. All 9 exercise real conditional/branching
  logic (WS protocol framing, CAS semantics, auth gating) — none are trivial
  export-exists/getter-identity/string-list checks, so no deletions were made.

## Next steps (for whoever picks this back up)

1. Wait for / coordinate on the `compose/client/bin/store/rs` workspace-extraction
   work to land so the repo-wide `cargo build --tests -p os-hub` succeeds again.
2. Re-baseline: `cargo build --tests -p os-hub` (warm) then
   `time cargo test -p os-hub` from repo root.
3. Migrate `framework/product/os/hub/script.ts`'s `TestScript` from raw
   `runCargo(["test", ...])` to `runCargoTestBudgeted(["os-hub"], this.root, segments)`
   (import from `../../../repo/lib/js/index.ts`, matching existing import depth in
   that file) — this is a pure mechanical swap, independent of the workspace issue,
   and can be done any time.
4. Re-measure against the 30s budget; only then decide whether any of the 9 tests
   above are cheap to trim (unlikely given they're all substantive).

## Files touched by this session

None inside `framework/product/os/hub` (blocked before any edit). Only this report
file was created:
- `.repo/🎫/26/07/17/OVERHAUL-TEST-SUITES-TO-30S-BUDGET-PER-APP/report-os-hub.md`
