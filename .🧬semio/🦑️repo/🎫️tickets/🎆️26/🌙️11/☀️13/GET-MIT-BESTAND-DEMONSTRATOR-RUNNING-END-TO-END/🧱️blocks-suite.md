# 🧱️ Observation blocks suite: root cause and fixes

Suite: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧱️blocks/🟦️.ts`
Run: `repo tools exec bun -- test ./🟦️.ts` from the suite directory.

## Result

- Before: 0 pass, 18 fail.
- After: 16 pass, 1 fail in the parent run (17 law tests, the isolated publisher law counted once).
- Remaining failure: `observation block publisher rebuilds retired shared bytes before publishing a fresh header`. It needs the repo's captured source selection, which the direct runner does not provide (see below).

## Root cause

It is not schema or fixture drift. `$defs.blockRevalidationCase` in `🧬️schema/🔣️.json` accepts all 14 fault rows of `🧫️cases.json`. The first assertion passes for every row.

The real failures were:

1. The block child computes its input from `join(workspace,"Cargo.toml")`, but the private repository workspace was empty, so every child run failed with ENOENT before `warm` was set.
2. The `cancel` and `cancel-close` faults expect exit status 130, the runtime's contract for a SIGINT-cancelled script. The child swallowed the cancellation and exited 1. The failing expectation then closed the shared admission owner, which cascaded failures into the following tests.
3. The publisher law calls `retireBuildGraphOptionalGroupsHeadroomV1`, which spawns a closure-read child. That needs a parent script invocation and the captured source selection, so it must run as an isolated test like its sibling `♻️groups`.

## Files changed

- `🧪️tests/🧱️blocks/🟦️.ts`: imports `writeFileSync`, `registerIsolatedBuildTestV1`, `withBuildTestInvocationV1`. Seeds a minimal `Cargo.toml` into the private workspace. Converts the publisher law to `registerIsolatedBuildTestV1` with `withBuildTestInvocationV1`, matching `♻️groups`.
- `🧪️tests/🧱️blocks/👷️child/🟦️.ts`: records `interrupted` at the SIGINT emit and exits 130 for interrupted failures, otherwise 1.

No schema or fixture rows changed.

## Open item

The publisher test cannot pass under `repo tools exec bun -- test` alone. Its closure-read child depends on `import.meta.semioCapturedSourceResolutions`, which the repo build pipeline bakes in. The sibling `♻️groups` test fails the same way when run directly, so this is a runner limitation. Verify it through the repo's build-observation target.
