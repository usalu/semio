# Kit Wires Typology Parity

## Summary

Fixed kit app wires diagram identities falling behind the virtual file system when a typology is unfolded.

## Root cause

`prepareKitWiresVfsForTopology` only awaited kit-root children and was memoized per kit. `syncKitWiresTopology` ran synchronously after expand, before typology children were in `childrenStore`.

## Fix

- Await kit root plus every expanded branch in `prepareKitWiresVfsForTopology` via `ensureChildrenLoadedAsync`.
- Removed per-kit prepare memoization (`kitWiresVfsPreparePromises`, `clearKitWiresVfsPrepare`).
- Resync wires from `invalidateKitVirtualFileSystem` when kit mutations clear VFS cache.
- Added regression test: expand typology → wires node ids match visible VFS ids.

## Files

- `semio/client/lib/sketchpad/js/index.ts`
