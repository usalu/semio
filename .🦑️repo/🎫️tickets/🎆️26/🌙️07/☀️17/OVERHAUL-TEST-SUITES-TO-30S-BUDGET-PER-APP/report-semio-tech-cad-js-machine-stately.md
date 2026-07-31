# `@semio-tech/cad-js-machine-stately` — test-suite overhaul report

## Status: SKIPPED (blocked on pre-existing, out-of-scope failure)

## What was found and fixed in-scope

Before any test could even run, two pre-existing bugs in this unit's own
infrastructure files were blocking the `test` target entirely:

1. `cad/machine/stately/script.ts` — all 5 imports from `repo/lib/js/index.ts`
   used a relative path one level too deep (`../../../../repo/lib/js/index.ts`
   from `cad/machine/stately/`, which resolves above the repo root). Fixed to
   `../../../repo/lib/js/index.ts` (3 levels: stately -> machine -> cad ->
   repo root), matching every sibling project's script.ts (e.g.
   `cad/kernel/brepjs/script.ts`, `cad/module/aec-building-energy/script.ts`).

2. `cad/machine/stately/js/vitest.config.ts` — every sibling alias was missing
   the `/js/` path segment and one directory level (e.g.
   `"@semio-tech/cad-js-core": resolve(root, "../../core/index.ts")` pointed
   at the nonexistent `cad/machine/core/index.ts` instead of
   `cad/core/js/index.ts`). Rewrote using the same `jsRoot = resolve(root,
   "../../..")` + `resolve(jsRoot, "<pkg>/js/index.ts")` pattern used by every
   other module's vitest.config.ts (verified against
   `cad/module/aec-building-energy/js/vitest.config.ts`,
   `cad/kernel/brepjs/js/vitest.config.ts`, etc).

Both fixes are confined to this unit's own two files and get the `test`
target to actually start and resolve all its imports/aliases correctly.

Confirmed the runner is already budget-enforced: `TestScript.run` calls
`runVitest(this.root, segments, "js/vitest.config.ts")` from
`repo/lib/js/index.ts`, which routes through `runTestBudgeted` (hard
wall-clock kill). No runner migration needed. `js/vitest.config.ts` already
had no `maxConcurrency`/`fileParallelism`/`testTimeout` overrides (verified).

## Test classification (in-source `import.meta.vitest` block, `js/index.ts`)

All 4 existing tests are KEEP-category — genuine parity checks between the
pure-TS state engine and the XState-backed `StatelyStateEngine` across real
interaction workflows (box creation, undo, distance/area measurement), i.e.
reducers/state-machine behavior with real branching logic. None are trivial
(no export-exists checks, no CSS-substring assertions, no plain
serde/JSON-shape padding). No deletions were made.

## Why this is reported as skipped, not pass/fail

After the two infra fixes above, the suite still fails — but for a reason
entirely outside this unit: `bootstrapCadModules()` (in
`cad/runtime/js/index.ts`, a separate Nx project/unit) relies on Vite's
`import.meta.glob` to eagerly load all shipped `modelDefinition` JSON assets
(typologies, interactions, manifests, etc). Under this repo's current
`bun x vitest` invocation, `typeof import.meta.glob` evaluates to
`"undefined"` at runtime, so `shippedModelDefinitionAssetsCache` falls back to
`emptyModelDefinitionAssets()` — zero interactions/manifests get registered
for every model definition. That makes
`loadSpatialInteraction("primitive.box")` return `null` and
`listSpatialInteractionsForModelDefinition(...)` return `[]`, so all 4 tests
in this unit fail downstream of code this unit does not own.

This was verified to be a repo-wide, pre-existing failure and NOT something
introduced by editing this unit: reproduced identically, unmodified, by
running the *native* test targets of two other, unrelated projects that also
call `bootstrapCadModules()`:

- `cd cad/runtime && bun ./📜️script.ts test` — same
  `[DEBUG] typeof import.meta.glob: undefined` line, both of that project's
  own tests fail (`expected false to be true`).
- `cd cad/module/aec-building-energy && bun ./📜️script.ts test` — same debug
  line, its own test fails (`Cannot read properties of null (reading
  'sources')`).

Per the ticket's stop condition ("errors UNRELATED to tests... a sign of
another concurrent session's in-progress refactor... do not attempt to fix
unrelated production code"), this was left alone. The actual root cause lives
in `cad/runtime/js/index.ts` (`shippedModelDefinitionAssets()`), a different
Nx project from this unit, and fixing Vite/vitest's `import.meta.glob`
transform pipeline repo-wide is out of scope for
`@semio-tech/cad-js-machine-stately`.

## Before / after

- Before: `test` target failed immediately at import resolution (could not
  even start vitest).
- After (with the two in-scope fixes): vitest starts, resolves all aliases,
  runs 4 tests in ~18ms — all 4 fail due to the out-of-scope
  `cad/runtime` / `import.meta.glob` issue above. Wall-clock for the whole
  `bun ./📜️script.ts test` invocation is well under 1s (blocked before it ever
  gets slow), so the 30s budget question is moot until the upstream issue is
  fixed.

## Files touched

- `cad/machine/stately/script.ts` (fixed relative import depth, 5 lines)
- `cad/machine/stately/js/vitest.config.ts` (fixed alias resolution paths)
- No test files were created or deleted; no coverage was removed.
