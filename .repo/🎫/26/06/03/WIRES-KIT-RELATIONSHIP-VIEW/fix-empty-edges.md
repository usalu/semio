# Fix Empty Kit Wires Edges

Reopened to fix nodes-without-edges in dev sketchpad.

## Root cause

1. `visibleVirtualFileSystemNodesFromTree` omits the kit root; containment edges need both endpoints visible.
2. `kitWiresVfsPreparedKitId` short-circuit let concurrent syncs finish before two-level VFS expansion completed.

## Fix

- `sketchpadKitWiresVisibleNodes` prepends kit root for wires sync only.
- `kitWiresVfsPreparePromises` memoizes full prepare per kit; cleared on invalidate/route reset.
