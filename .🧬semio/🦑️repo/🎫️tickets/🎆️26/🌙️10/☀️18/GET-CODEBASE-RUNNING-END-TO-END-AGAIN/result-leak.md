# Result leak: leaked test disk images detached, test teardown fixed, wasm builder release failure registered

Executor `leak` (Sonnet 5.5). Foreground only, no sub-agents, no cargo, no modifying git command, no `package.json` / `bun.lock` edit, nothing under the frozen storage / revision paths. `P/` = `🧰️framework/🔨️modules/🏃️process/`, `PR/` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/`, `C` = this ticket folder.

## 1. Leaked images detached

`hdiutil info -plist` showed 47 images. 34 were test images with no open file (`lsof +f -- <mount>` empty for each, no process mentioned their paths): 14 `ex-*`/`case-candidates-*` mounts under the shared scratchpad `ta/` (`ex-*/v`, `case-candidates-*/.cache/b/{l,g}`) and 14 `C/🗑️generated/nx/artifact-source-*/.cache/b/l`. All 34 were detached with plain `hdiutil detach` (no `-force` needed), then their now dead directories (28 dirs, about 67 MiB each, the fixed `fixed.dmg` files) were removed from the scratchpad and `C/🗑️generated/nx`.

Left attached on purpose (live): `.cache/b/{l,o,d,t,g,p,n}`, `🧰️framework/.cache/b/{l,t}`, `.🧬semio/🦑️repo/⚡️cache/rust/fixed`, the Xcode simulator runtimes and the Cursor installer. Final state: 13 images, all live or system.

During the work one more image appeared (`ta/ex-8w0T76`, created 22:32, before the fix below landed, from a peer's run of the old `🍎️image.ts`); same checks, detached and removed. Peers running old test code can still leak until they restart.

## 2. Why tests leak mounts, and the fix

Root causes (evidence: the leaked directory names map 1:1 to test code):

* `PR/🧪️test/🧪️tests/🚷️discovery-boundaries/🟦️.ts` (`case-candidates-*`): the plugin under test (`createNodesV2` with `workspaceRoot: root`) births the lease/graph partitions as images below `root/.cache/b/*`. The `finally` did `rmSync(root)`, which cannot remove a mount (`EBUSY`) and never detached.
* `PR/📚️library/⚡️caching/🧪️tests/🗿️artifact-source/🟦️.ts` (`artifact-source-*`): real Nx child inside a temp workspace mounts `.cache/b/l`; the test had no teardown at all (not even on success). When the lease path exceeded 256 characters (as it does from this long ticket path) it failed after the mount and left it.
* `P🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🍎️image.ts` (`ex-*`): `rmSync(parent)` ran only at the end of the inner `finally`; every earlier failure (open, retire, timeout) left `parent/v` mounted.
* A test killed by its timeout runs no `finally` at all.

Fix (shared helper, framework tier so products may import it, no storage path touched):

* `P🧪️testing/🎛️execution/🟦️.ts` (next to `runBudgetedTestCommand`): new `createTestScratchRootV1(parent, prefix)`, `disposeTestScratchRootV1(root)`, `detachDiskImagesUnderV1(root)`, `attachedDiskImagesV1()`.
  * Lists images through `/usr/bin/hdiutil info -plist` + `/usr/bin/plutil -convert json` (system tools only, macOS only; no-op elsewhere, plain `rm` there).
  * Selects images whose backing file (realpath, because `hdiutil` reports `/tmp/...` unresolved) lies under the root, detaches deepest first (nested images), plain detach then `-force` on the owned tree, then removes the tree.
  * Every root is registered once in the process; a single `process.on("exit")` guard disposes whatever the test did not dispose (crash, timeout, uncaught error). A different process or `SIGKILL` cannot be covered by any in-process hook.
* Re-exported explicitly in `P📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts`.
* Adopted at all 7 sites: 5 `mkdtempSync` roots in `discovery-boundaries`, the root in `artifact-source` (body wrapped in `try/finally`), the `ex-` root in `🍎️image.ts` (outer `try/finally`; the inner `rmSync(parent)` is gone). The library copy `PR/📚️library/🏃️process/🦀️cargo/🎯️exact/🧪️tests/🍎️image.ts` mounts nothing itself (its `ex-` dir is inside the compiler store) and is unchanged.
* Tests added to `P🧪️testing/🎛️execution/🧪️tests/🟦️.ts` (suite `test-command`, no new Nx target): (a) an 48 MiB outer image with a nested inner image attached under a scratch root; the listing order is inner first; after `disposeTestScratchRootV1` the system `hdiutil info` oracle shows neither image and the tree is gone; disposing an unknown root throws. (b) a child process creates a root, attaches an image, prints the path and throws; after it exits the images are detached and the tree is removed (the exit guard).

Not done on purpose: sweeping `SEMIO_TEST_ARTIFACT_DIR` after every test child in `runBudgetedTestCommand` / `withRepositoryTestArtifactsV1` (`PR/🧪️test/💾️storage`, frozen area). That directory is shared by concurrent runs in the law harness (`ta/`), so a blanket sweep would detach a peer's live mount. If wanted later: in `retainRepositoryTestArtifactsV1`, call `detachDiskImagesUnderV1(row.path)` right before the `rmSync(row.path, {recursive:true})` (runs are exclusive there). Prepared, not applied.

### Verification (exact commands, outcome)

* `SEMIO_TEST_ARTIFACT_DIR=/tmp/leakart bun test ./P🧪️testing/🎛️execution/🧪️tests/🟦️.ts` : 3 pass, 0 fail (the 2 new tests plus the existing esbuild-bundle test, which also checks every bundled input stays below `P`). First run of the new tests was red twice and found real defects (image-path is reported unresolved; outer image too small), both fixed. `hdiutil info` afterwards: no image under the test dir.
* `LAW3_ART_DIR=/private/tmp/lk zsh C/law3-tools/runt.sh <discovery-boundaries> "rejects loop|portable no-follow"` : 2 pass. `... "candidate inventory"` : fails at the second `invokeCurrentImplementation` row with `Original process birth already received` (pre-existing: one birth per process, unrelated), but the image count before and after is identical (14 and 14) and `/tmp/lk` is empty, i.e. a failing run no longer leaks (before: leaked `case-candidates-*`).
* `... <artifact-source> "artifact verification"` : fails with `Materialized absolute path exceeds 256 characters` from this ticket's long path (pre-existing, same failure that used to leak the mount); after it the image count is unchanged and `C/🗑️generated/nx` is empty.
* Type check: `bun node_modules/typescript/bin/tsc --noEmit --strict ...` on the helper and its test: zero errors. `bun build --no-bundle` on all 5 touched TS files: ok.
* `🍎️image.ts` was not executed (it runs a real cargo build; no cargo allowed).

## 3. wasm builder registers its release failure on the Original

Cause: `buildWasmWebV1` acquired the compiler capacity lease and the Cargo build lease inside the `withCargoCompilerInputsV1` body but released them in its outer `finally`, outside the issued operation. The tests/lint runners release inside that body, so a release failure rejects the operation, which `withIssuedResourceOperationV1` records on the Original. Wasm released outside, so the failure only reached its direct caller and the Original settled.

Fix `P📦️artifacts/🕸️wasm-build/🟦️.ts`: `capacity`, `lease` and `bodyFailed` are now declared inside the body callback, the body is wrapped in `try/catch/finally`, and the `finally` releases lease then capacity (first failure kept, thrown only when the body itself did not fail, same rule as the test and lint runners). The outer `finally` only removes the two output dirs. Release order is now lease, capacity, remove, remove.

Corpus `P📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/📥️consumers/🔣️.json`: `wasm-cleanup` `settlement` `settles` -> `rejects`; traces of `wasm-success`, `wasm-undefined`, `wasm-frozen`, `wasm-threaded`, `wasm-cleanup` re-pinned to the new release order (13 lines). The law (`🟦️.ts` unchanged) asserts for `wasm-cleanup` that the Original settles rejected with the identical release failure object.

Verification: `LAW3_ART=ticket LAW3_MAX_MS=3600000 LAW3_CAPS=<native grant> python3 C/law3-tools/run1.py P📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/📥️consumers/🟦️.ts` : 2 pass, 0 fail (the first run with only the settlement pin changed failed on the trace diff of `wasm-success`, which proved the new order is real). `tsc --noEmit --strict` on `🕸️wasm-build/🟦️.ts`: zero errors. Callers (`PR/📚️library/🏃️process/🕸️wasm/🔣️policy/🟦️.ts`, `📥️consumption` law, cache-contracts source scan) do not depend on the old `finally` shape.

## Files changed

* `P🧪️testing/🎛️execution/🟦️.ts`, `P🧪️testing/🎛️execution/🧪️tests/🟦️.ts`, `P📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts`
* `P🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🍎️image.ts`
* `PR/🧪️test/🧪️tests/🚷️discovery-boundaries/🟦️.ts`, `PR/📚️library/⚡️caching/🧪️tests/🗿️artifact-source/🟦️.ts`
* `P📦️artifacts/🕸️wasm-build/🟦️.ts`, `P📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/📥️consumers/🔣️.json`

Temporary output (`/tmp/leakart`, `/private/tmp/lk`, law3 logs for my runs) deleted; no `[DEBUG]` lines added.

## Still open

1. Other tests in the repo that call plugins/CLI with a private `workspaceRoot` (`mkdtemp(... SEMIO_TEST_ARTIFACT_DIR ...)`, about 350 files use that env) can leak the same way; only the 7 observed sites were converted. A grep for `mkdtempSync(join(output` / `mkdtemp(` next to `createNodesV2`, `nx run`, `openNative*` finds candidates; the helper is the one-line replacement.
2. `P📦️artifacts/💾️cache/🧪️tests/🟦️.ts` still unchecked for the child-exit race noted in result-law5.
3. Optional central sweep in `retainRepositoryTestArtifactsV1` (see above), frozen area.
4. `discovery-boundaries` rows 2, 4 and 5 and `artifact-source` fail for reasons independent of this ticket (single process birth per process; plugin copied outside the repo cannot resolve `../../../../🔨️modules/...`; 256 character path from this ticket).
