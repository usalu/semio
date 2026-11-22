# Graph maintenance tests: format owner refusal (2026-11-22)

Follows `closure-repair-and-storage-sqlite-2026-11-22.md`. Build = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`.

## Cause
`📥️inputs/💾️storage/🧪️tests/🟦️.ts` ran its three invocation-based tests (`withBuildTestInvocationV1`) against `repository.root`, the private test repository. That helper issues the birth for the checkout, so the production check `Exact Original Graph format owner required` (unchanged, not weakened) refused. The first refusal closes receiver admission, which cascaded into the 25 `shared graph closure maintenance ...` failures.

## Fix
Same pattern as the passing `🧪️tests/📒️ledger/🟦️.ts`: a local `withPrivateStorageV1` validates the received birth with `withBuildTestInvocationV1(async()=>{})` and then enters `repository.invoke(run,{maximumElapsedMilliseconds:60000})`, an invocation issued for the private repository. The tests "actual shared graph writer ...", "optional graph maintenance defers ...", "component schema preserves every owned contract" use it.

Two stale fixture defects surfaced once the schema test ran (schema was extended by other work, fixture not):
- `🧪️tests/🔣️.json` `schemaOwnershipCase.pointer` enum lacked `#/definitions/OptionalQuota/definitions/Reclamation` (rows already used it): added.
- `schemaOwnershipHashes` for `OptionalQuota` and `OptionalLedger` were stale (schema gained `Reclamation`, `GraphScopeWriterProtocol`, a longer `OptionalPath`). Re-pinned to the received digests; the test asserts the independent python3 hashlib oracle yields the same values.

## Results (repo-graph-maintenance:test via native-exec)
Before: 21 pass / 32 fail. After: 24 pass / 28 fail. No `format owner required` failure remains; the first test (actual shared writer inside its reader) and the schema ownership test now pass.

Remaining 28 failures, separate causes, not in this scope:
- "optional graph maintenance defers actual retained readers ...": real body error `Nested persistent image requires actual parent scope`, thrown by `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts` (modified 2026-11-22 00:52, another agent's edit). The test opens two persistent images with the same options and no parent scope. The `finally` then fails with ENOENT on the retirement receipt and masks it. This test needs adapting (or the cleanup edit settling), then it stops closing admission for the 25 cascaded `closure maintenance` cases.
- Source-extraction tests `warm-lease-infrastructure`, `no-pressure`, `deferred-optional-funds-original-writer` and the import-list expectation (`../📥️inputs/💾️storage/🔒️writer/🟨️.mjs`): extracted sources changed concurrently (writer files staged today); not touched.

## Formats test (`⚡️caching/.../🧾️closures/🧪️tests/📇️formats`)
`requireClosureReadDescriptorV1` exists in `🧾️closures/👷️closure-read-worker/🟨️.mjs`; the test file was edited at 03:51 today by another agent. A direct `bun test` of the file fails 3 tests but without the harness env (`SEMIO_TEST_ARTIFACT_DIR` unset), so it is not a valid run. The owning suite is `storage-graph-processes` (`repo:test-storage-graph-processes`); full suite run via native-exec (`repo:test-storage-graph-processes`): neither `requireClosureReadDescriptorV1 is not defined` nor `Bounded renamed graph retirement path required` appears any more, so the other agent's edit landed; no fix needed here. The suite still has unrelated failures in other files (graph optional group retirement x3 with 0 pass/2 fail each, closure proof/census 3 fail, closure maintenance key-reference/busy-reader), not analysed.

## Changed files
- build/📥️inputs/💾️storage/🧪️tests/🟦️.ts
- build/📥️inputs/💾️storage/🧪️tests/🔣️.json
