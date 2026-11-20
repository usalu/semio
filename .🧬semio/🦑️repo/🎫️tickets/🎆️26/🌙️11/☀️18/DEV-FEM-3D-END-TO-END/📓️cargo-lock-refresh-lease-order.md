# Cargo registry-fetch vs native-storage-mapping lease order

## Symptom
`[lease-order] native-storage-mapping (image-mapping, rank 7) is requested while cargo:registry-fetch (registry-fetch, rank 8) is held exclusive` in the component driver run (attempt `semio-hub-fem-component_component-1795049120492.txt`, line 471, after "Cargo.lock ... is current again"). It is observe-mode and deduped per process, so it also appeared in a run with no lock refresh (`...1795045737765.txt`); the lock refresh is not the cause, it only made the run visible.

## Root cause
The rank table (`🔒️leases/🧭️order/🔣️policy.json`) is right: `registry-fetch` (8) is the innermost lease, held only while the fetch runs (documented on `withRepositoryCargoInputsOwnedV1`). But code running under that exclusive lease called `storage.recheck()`. A reservation recheck goes through `custody.read`, then the image port `withJournal`, which takes `withNativeStorageMappingReadV1` (shared `native-storage-mapping`, rank 7). So mapping was requested while rank 8 was held.

Offending calls under the fetch lease:
- `withRepositoryCargoRegistryReadyOnceV1` writer: leading `storage.recheck()` (`🟢️ready/💾️storage/🟦️.ts`)
- `publishReady`: two rechecks
- `captureReady`: recheck in the failed-metadata catch
- the fetch closure in `withRepositoryCargoInputsOwnedV1` (`📥️dependencies/🟦️.ts`): recheck at start, per request, and at end. The stale-lock refresh (`ensureRepositoryCargoLockV1`, force) runs inside this closure.

## Fix
Release-before design, no rank change. Removed every `storage.recheck()` from the held region, using `checkResourceLeaseControl` (no lease) for cancellation. Storage is still rechecked by the enclosing compiler-writer custody (`withCompilerWorkspaceSharedWriteV1` wrapper does `recheck(); inner(); recheck()`), i.e. before the fetch lease is acquired and after it is released, and by the SH-lease consumption rechecks that follow.

## Test
New `📥️dependencies/🧪️tests/🔒️fetch-order/🟦️.ts` (imported from the dependencies test index): AST check with the third-party `typescript` parser that the custody writer closure, `publishReady`, `captureReady` and the fetch closure contain zero `recheck` calls. Passes (the dependencies index: 9 pass, 1 unrelated fail: `shareSchedulerBuildV1 is not defined` from the peer's in-progress `🧰️native-helpers/🛰️fleet`). Not run: end-to-end hub component run.

## Files
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/💾️storage/🟦️.ts`
- `.../🦀️cargo/📥️dependencies/🟦️.ts`
- `.../🦀️cargo/📥️dependencies/🧪️tests/🔒️fetch-order/🟦️.ts` (new), `.../🧪️tests/🟦️.ts` (import)
