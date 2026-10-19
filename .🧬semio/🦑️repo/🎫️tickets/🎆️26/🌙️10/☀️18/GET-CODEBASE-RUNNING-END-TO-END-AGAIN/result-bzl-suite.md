# Result bzl-suite: Bazel contract suite (`workspace:test-build-tool-bazel`)

Abbreviations: `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `BZ` = `B/🔧️tools/🏗️bazel`, `C` = this ticket.

## Runs (pass / fail)

The canonical runner (`bun repo run workspace:test-build-tool-bazel`) queued for 590 s without starting (run 1 waited on `native-fixed-image:.cache/b/d`, run 5 on `dependencies:store-ownership:*`, both held exclusively by the `dependencies sync --all` loop of deps4). For the source-parity phase the same test file was run under the genuinely issued envelope wrapper `C/bzl-act-tools/wrap.ts` (pattern `^(?!Bazel actual owned runtime ).*`, i.e. exactly the first command of `bazelTestCommandsV1`).

| Run | Runner | pass | fail | Note |
|---|---|---|---|---|
| 1 | canonical | - | - | queued 590 s behind sync, SIGALRM |
| 2 | wrapper, source parity | 40 | 35 | first failure = contract pin; 19 cascade ("Original process admission closed") |
| 3 | wrapper, after re-pin | 68 | 7 | |
| 4 | wrapper, after test-owner fixes | 72 | 3 | remaining: Nx parity + its cascade + afterAll |
| 6 | wrapper, after loader policy fix | 72 | 3 | same three |

## Root causes and fixes

1. Pinned contract (first failure). `Command` and `MaterializedPaths` of `…/native-build/💾️storage/🧱️fixed/🧬️schema/🔣️.json` moved legitimately: the xplat-linux/xplat-win2 decision (directory-backed partition) removed the `format`/`information` command actions and the `fixed.vhd` filename, added `swap-pending*` faults. Re-pinned both in `BZ/🧪️tests/📋️contract/🔣️.json` by recomputing the test's exact normalization (`C/bzl-pin-tools/digest.ts`: 12 of 12 ok). `Command` `1ca805ba…` -> `2bb5b0ac…8f7dd`, `MaterializedPaths` `15bb943a…` -> `aa754355…8d231`. Contract not weakened. If xplat-win2 changes these definitions again, recompute with `bun C/bzl-pin-tools/digest.ts`.
2. Test owners moved with the code (bzl-ops split `publishCapturedBazelGraphV1` into planning + `generateBazelWorkspaceV1`; bzl-sandbox added `issueBazelTaskEnvironmentV1`/`bazelTaskCargoDirectoriesV1`/`cargoDirectoryKeys`, bzl-ops added `buildExecEnvironmentV1` to the base environment). Updated the source-reading tests to the new owners, no code change:
   - `BZ/🧪️tests/📥️delegation/🟦️.ts` (orchestration initializer now read from `generateBazelWorkspaceV1`),
   - `BZ/🧪️tests/🔏️implementation/🟦️.ts` (planner asserts on `publishCapturedBazelGraphV1`, `taskFingerprint(` and the BUILD write asserted on `generateBazelWorkspaceV1`),
   - `BZ/🧪️tests/📥️fingerprint/🟦️.ts` (the transpiled fingerprint closure now includes `cargoDirectoryKeys`, `bazelTaskCargoDirectoriesV1`, `issueBazelTaskEnvironmentV1` via one `fingerprintCode()` helper; bindings gain `buildExecEnvironmentV1`, `resolve`).
3. Real code defect (runtime `cache` phase): the captured implementation closure is now 518 files, but the immutable loader template in `B/📥️source/🔏️closure/🟦️.ts` hard-coded `files.length>512` while the schema `Policy` (`maximumFiles` 2048, schema `maxItems` 2048) is the authority, so the captured planner died with `Closed immutable implementation snapshot required`. The loader template now interpolates all five limits from `policy` (`maximumFiles`, `maximumFileBytes`, `maximumSourceBytes`, `maximumSnapshotBytes`, `readChunkBytes`); no literal copies remain.
4. Shared-envelope cascade: NOT changed. It is the intended envelope law (`track` in `🧭️routing/📥️invocation/🏃️process/🟦️.ts` closes admission of the scope chain on the first rejected operation; documented on `registerIsolatedBuildTestV1`, which exists for tests that reject on purpose). Loosening it would weaken the process-envelope fail-fast guarantee, so I treat it as a design property, not a harness defect. After the fixes the cascade is one test (run 4/6) instead of 19.

## Still failing (environment, not code)

- `Bazel and installed Nx preserve converted literal defaults and original caller argument boundaries` (+ cascade `Bazel control drains owner command budgets…` + unnamed afterAll): `ENOENT lstat .nx/installation`. The symlink points into `.cache/b/t/m/nx/3abdf882…` whose directory now holds only a stale `.stage-copy-tuSqkS` (15:03); the Nx install tree is gone. `tools setup nx` is blocked on deps4 ("Closed pinned dependency recipe required", see coordination). Needs `bun repo tools setup nx` after the dependency sync finishes.
- Runtime phases (`cache`, `nested`, `uncached`, `birth`, `services`, `refusals`, `cancellation`, `configuration`, `offline`): not verified. Wrapper runs of `cache` and `nested` blocked on shared `native-fixed-image:.cache/b/d` behind the sync (cache timed out at 120 s while waiting). The first `cache` run (before fix 3) reached the real Bazel binary and showed the loader defect.

## Not run

Canonical runner to completion; runtime phases `uncached`…`offline`.

## Freeze note (19:52)

All four edits above (contract pin, three test owner fixes, closure loader policy) were applied before the coordinator's hard freeze; no file under `📚️library/` was edited after it. At 19:52 the `dependencies sync --all` loop (3 processes) still held the leases, so the canonical runner and the runtime phases remain unrun. Next steps after the freeze lifts and the sync ends: `bun repo tools setup nx` (restores `.nx/installation`), then `perl -e 'alarm 590; exec @ARGV' -- bun repo run workspace:test-build-tool-bazel` (or one `phase` at a time) and fix whatever the runtime phases show. Logs: `C/🗑️generated/bzl-suite/` (delete when done).
