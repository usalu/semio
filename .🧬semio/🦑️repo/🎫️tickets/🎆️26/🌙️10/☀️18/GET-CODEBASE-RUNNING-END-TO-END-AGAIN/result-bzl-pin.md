# Result bzl-pin: Bazel test tree, contract re-pin, cache class wiring

Abbreviations: `L/` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, `B/` = `L/🏗️build/`, `BZ/` = `B/🔧️tools/🏗️bazel/`, `BST/` = `L/⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/`, `C/` = this ticket. No `bun repo`, no cargo, no git-modifying command, no sub-agent. Max one test process at a time (fleet rule against `bun test ./`: only explicit files were used).

## Verdict

PARTIAL. 19 of 20 test files are green under the envelope wrapper (`C/bzl-act-tools/wrap.ts`) with the real Bazel binary. The aggregate `BZ/🧪️tests/🟦️.ts` and the file `BZ/🧪️tests/🔏️implementation/🟦️.ts` still fail on ONE real defect that I could not fix inside my scope (section 3): the closure law "no external runtime dependency" is violated by a concurrent import sweep that made `L/🏗️build/…/inference → print artifacts` import the coarse barrel `@semio-tech/repo-lib/storage`. All 21 aggregate failures are that one failure plus its cascade (the failing test closes the shared Original envelope; 19 later tests fail with "Original process admission closed").

## 1. Contract re-pin

The pinned hash that failed was NOT in the Bazel schema (the bzl-ops guess was wrong). `BZ/🧪️tests/📋️contract/🔣️.json` pins 12 definitions; `BZ` `StorageLayout` had already been re-pinned by a peer; the failing one was `MaterializedPaths` of `L/../🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧬️schema/🔣️.json`. Diff read (`git diff HEAD`): one enum value `filename: "fixed.img"` became `"fixed.dir"` (14:36). This is the intended coordinator decision "Linux becomes a unprivileged directory-backed partition backend" (xplat-linux, still finishing: its `🧪️tests/📥️paths/🔣️.json` fixture still says `fixed.img`, not my change). Re-pinned `sha256` of `MaterializedPaths` from `e22a05b7…8a217` to `15bb943a7b9cbd377d375c4aec86d0d3ad68192afcfdfbea733d54c617c99d15` by recomputing the contract test's exact normalization (`C/bzl-pin-tools/digest.ts`: all 12 now `ok`). Contract not weakened. If xplat-linux changes that definition again the pin must be recomputed with `bun C/bzl-pin-tools/digest.ts`.

## 2. Cache decision from the plugin class (coordinator addendum)

Adapter `BZ/📜️script.ts`:
- `bazelTargetCacheClassV1(task)`: reads `definition.metadata.semio.cache`, validates it against the closed schema `B/🏃️execution/🧬️schema/💾️cache.json` (invalid is refused: `Closed repository cache decision required`).
- `bazelCacheDecisionV1` derives from it (fail closed). Order: explicit `cache:true` with a non-deterministic class (and not an ordering `noop`) throws `Conflicting cache declaration`; then continuous flag, skipped, noop (always cacheable), non-deterministic class (uncached, reason = class name), manifest `cache:false` (`declared`, authoritative also against a deterministic class), no manifest flag and not deterministic (`undeclared`), deterministic or explicit true without declared outputs (`unbounded`), else cacheable. So a `deterministic` target without a manifest flag is now derived cacheable (outputs still mandatory, `[]` is fine).
- `BazelUncachedReasonV1` and `BZ/🧬️schema/🔣️.json` `CacheDecision.reason` gained `driver-owned, managed-output, side-effecting, live, aggregate, diagnostic` (the mandatory reason is thus the plugin class). The BUILD rule (`uncached_reason`, `no-cache` requirement) needed no change.
- Fixture test (language-agnostic, 20 rows in `classDecisions` of `BZ/🧪️tests/📥️actions/🔣️.json`, closed by the new section in `🧬️schema.json`): every row's metadata is validated with Ajv strict against `💾️cache.json`; each expectation equals a SQLite `CASE` oracle for the non-error rows; error rows (conflicts, mismatched reason/class, unknown class, extra key) assert the thrown message; the covered classes must equal the schema's eight classes. Test in `BZ/🧪️tests/📥️actions/🟦️.ts`.
- Not run end to end: a real graph that carries `metadata.semio.cache` through `readBuildGraphV1` into a Bazel run (the adapter already reads `metadata.semio.execution` from the same place, so the path exists, but no product run was done per instruction).

## 3. Open blocker: closure law broken by the import sweep

`BZ/🧪️tests/🔏️implementation/🟦️.ts` ("closes every local import and module-owned schema") fails with `External implementation runtime dependency refused: typescript`. Reproduce quickly: `SEMIO_TEST_ARTIFACT_DIR=… bun C/bzl-act-tools/wrap.ts ./BZ/🧪️tests/🔏️implementation/🟦️.ts`; locate with `bun C/bzl-pin-tools/importer.ts <entry> <target>` (mirrors the capture: Transpiler imports plus literal `./x.ts` strings, BFS parents). Shortest chain now:

`BZ/📜️script.ts` → `B/🕸️graph/🟦️.ts` (literal `./📥️inference/🟨️.mjs`) → `inference/🟨️.mjs` → `inference/🧩️component-targets/🟨️.mjs` → `L/../📓️print/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts` → `print/📦️artifacts/🟦️.ts` (changed 15:13: `import … from "@semio-tech/repo-lib/storage"`) → `L/📦️packages/🟦️typescript/💾️storage/🟦️.ts` (barrel re-exporting `@semio-tech/nx-bootstrap-tools/wasm/script`) → `⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts` → `📦️packages/🟦️typescript/📇️inventory/🟦️.ts` → `🧹️normalization/🧪️support/🏗️source-services/🟦️.ts` → `import "typescript"`.

Two more edges of the same sweep (also 15:13) reached the same barrel from the Bazel bootstrap and I reverted them to the previous direct relative import (same library, still exports `repositoryCachePartitionRootV1`): `BST/📜️script.ts` line 11 and `BST/🩺️readiness/🟦️.ts` line 9 (`../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts`, resp. one more `..`). If the layering sweep (lay13?) rewrites them again the Bazel closure breaks again.

Recommended fix (owner: the layering sweep, needs `L/📦️packages/🟦️typescript/package.json` exports, held back by the manifest freeze): a narrow entry such as `@semio-tech/repo-lib/storage/paths` → `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts` (only `repositoryCachePartitionRootDataV1`) for `print/📦️artifacts`. Any consumer on the build-graph path of the Bazel adapter must not import `@semio-tech/repo-lib/storage` (it drags bun/wasm/rustup tooling and `typescript` into the Original closure).

## 4. Neutral registry no longer pulls the Nx tool into the Bazel closure

First root cause found on the way (this one is fixed): `BZ/📜️script.ts` imported `buildExecEnvironmentV1` from `B/⚙️operations/🟦️.ts`, which dynamically imported the Nx cache tool (whole coordinator/nx closure) and the closure capture follows dynamic imports and literal paths of `import type` too.
- New leaf `B/⚙️operations/🌱️exec-environment/🟦️.ts` (`buildExecEnvironmentV1`, imports nothing); removed from `B/⚙️operations/🟦️.ts`; importers re-pointed (`BZ/📜️script.ts`, `BZ/⚙️operations/🧪️tests/🟦️.ts`).
- `buildCacheToolV1` (the registry with the two dynamic imports) deleted from `B/⚙️operations/🟦️.ts`; `runBuildCacheOperationV1` now takes the resolver as a required 4th argument (dependency inversion: neutral module imports no adapter). The composition root `B/📜️script.ts` passes `resolveCacheTool` (nx and bazel dynamic imports live there). Test `B/⚙️operations/🧪️tests/🟦️.ts` passes `async()=>bazel` for the refusal row.

## 5. Per-file results (real Bazel binary: `SEMIO_TEST_BAZEL_BINARY=$PWD/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel`, `SEMIO_WORKSPACE_ROOT=$PWD`, `SEMIO_TEST_ARTIFACT_DIR=C/🗑️generated/bzl-pin-art`)

Wrapped = `bun C/bzl-act-tools/wrap.ts ./<file> --timeout …` (Original envelope). Plain = `bun test ./<file>`: every suite that needs the Original envelope fails plain with `Original parent Script process invocation required` by design (5 arguments, 6 dependencies, 7 contract, 9 environment, 12 control, 13 snapshot, 14 implementation, 15 framing, 17 actions with 2); the rest pass plain with the same counts. Wrapped is the authoritative column.

| # | File (under L/) | Wrapped | Plain |
|---|---|---|---|
| 1 | `B…/🏗️bazel/💾️storage/🧪️tests/🟦️.ts` | 9 pass | 9 pass |
| 2 | `…/💾️storage/🧪️tests/📥️runtime/🟦️.ts` | 0 tests (helper module) | same |
| 3 | `…/💾️storage/🗑️reset/🧪️tests/🟦️.ts` | 1 pass | 1 pass |
| 4 | `…/⚙️operations/🧪️tests/🟦️.ts` | 12 pass | 12 pass |
| 5 | `…/🧪️tests/📤️arguments/🟦️.ts` | 3 pass | needs envelope |
| 6 | `…/🧪️tests/🕸️dependencies/🟦️.ts` | 12 pass (13 fail in the first loop only because `SEMIO_WORKSPACE_ROOT` was unset; rerun green) | needs envelope |
| 7 | `…/🧪️tests/📋️contract/🟦️.ts` | 2 pass (after the re-pin) | needs envelope |
| 8 | `…/🧪️tests/📥️delegation/🟦️.ts` | 1 pass | 1 pass |
| 9 | `…/🧪️tests/📥️environment/🟦️.ts` | 1 pass | needs envelope |
| 10 | `…/🧪️tests/📥️purpose/🧪️tests/🟦️.ts` | 9 pass | 9 pass |
| 11 | `…/🧪️tests/📥️purpose/🟦️.ts` | 0 tests (module) | same |
| 12 | `…/🧪️tests/📥️control/🟦️.ts` | 1 pass | needs envelope |
| 13 | `…/🧪️tests/📦️snapshot/🟦️.ts` | 4 pass | needs envelope |
| 14 | `…/🧪️tests/🔏️implementation/🟦️.ts` | 2 pass, 3 FAIL (section 3) | needs envelope |
| 15 | `…/🧪️tests/📥️framing/🟦️.ts` | 1 pass | needs envelope |
| 16 | `…/🧪️tests/📥️inventory/🟦️.ts` | 2 pass | 2 pass |
| 17 | `…/🧪️tests/📥️actions/🟦️.ts` | 12 pass (11 + the new class test) | needs envelope |
| 18 | `…/🧪️tests/📥️fingerprint/🟦️.ts` | 2 pass | 2 pass |
| 19 | `BST/🧪️tests/🟦️.ts` | 1 pass | 1 pass |
| 20 | `BST/🩺️readiness/🧪️tests/🟦️.ts` | 7 pass | 7 pass |
| A | aggregate `BZ/🧪️tests/🟦️.ts` (87/88 tests) | 57 pass, 10 skip, 21 fail = file 14's 3 + cascade 18 (closure, section 3) | n/a |

Also green, plain: `B/🧭️invocation/🧪️tests/🟦️.ts` 3 pass (under the wrapper its CLI-help test fails by harness artifact: the wrapper's declared envelope has no Repo selection), `B/🧭️selection` 6 pass (wrapped), `BZ/⚙️operations/🧪️tests/🟦️.ts` 10 pass plain. The `SEMIO_TEST_BAZEL_BINARY` real-binary tests ran inside files 1 and 4 (and the aggregate): their `skipIf` rows executed (aggregate: 10 skips are the `SEMIO_TEST_ARTIFACT_DIR`-gated or per-platform rows). Build checks: `bun build --target=bun 📜️script.ts`, `B/📜️script.ts`, `BZ/📜️script.ts` all bundle.

## 6. Files

Changed: `BZ/🧪️tests/📋️contract/🔣️.json` (one sha256), `BZ/📜️script.ts` (class derivation, exec-environment import), `BZ/🧬️schema/🔣️.json` (CacheDecision reasons), `BZ/🧪️tests/📥️actions/{🟦️.ts,🔣️.json,🧬️schema.json}` (class test + fixture + schema), `BZ/⚙️operations/🧪️tests/🟦️.ts` (import), `B/⚙️operations/🟦️.ts` (registry removed, exec-environment moved, docstring), `B/⚙️operations/🧪️tests/🟦️.ts`, `B/📜️script.ts` (resolver), `BST/📜️script.ts` and `BST/🩺️readiness/🟦️.ts` (barrel import reverted, see 3). Created: `B/⚙️operations/🌱️exec-environment/🟦️.ts`, `C/bzl-pin-tools/{digest.ts,importer.ts,run-all.sh,run-plain.sh}`. All tool-generated output under `C/🗑️generated/bzl-pin-*` deleted. No `[DEBUG]` added.

## 7. Open items

1. Section 3 blocker (layering sweep + narrow storage entry); after it, rerun the aggregate (expect 87 tests green).
2. The pin of `MaterializedPaths` must be recomputed whenever xplat-linux touches that definition again (`bun C/bzl-pin-tools/digest.ts`).
3. `B/⚙️operations/🔣️catalog.json` still lists cache implementations by tool; its note does not yet mention that the resolver moved to the composition root (behavior unchanged).
4. The aggregate's cascade failure mode (one failing test closes the shared Original envelope and fails 18 unrelated tests) is pre-existing harness design; per-file runs avoid it.
