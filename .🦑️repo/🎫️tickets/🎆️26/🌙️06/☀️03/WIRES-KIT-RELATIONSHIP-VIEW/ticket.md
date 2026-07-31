# Wires Kit Relationship View

Goal: Kit app Wires window — identities from visible VFS nodes, relationships from Rust store, synced with VFS expand/collapse.

## Summary

- Renamed kit Diagram window to **Wires** (`wires` window kind, `SKETCHPAD_SURFACE_KIT_WIRES`).
- Added `visibleVirtualFileSystemNodes` on `VirtualFileSystemController`.
- Built `sketchpadKitWiresFixtureFromVisible` + `sketchpadFetchKitWiresReferences` (Rust GraphQL via `@semio-tech/compose-js`, DTO fallback).
- Sync on VFS expand, children load, route change, and topology refresh.

## Fix empty edges (reopened)

- `sketchpadKitWiresVisibleNodes` prepends the kit root so `kit -> child` containment edges render.
- `kitWiresVfsPreparePromises` memoizes full two-level VFS expansion; concurrent syncs share one prepare.

## Files

- `framework/product/platform/core/index.ts`
- `compose/client/lib/sketchpad/js/index.ts`
- `compose/client/lib/sketchpad/js/package.json`
- `compose/client/lib/sketchpad/js/vite.config.ts`
- `compose/client/lib/sketchpad/js/vitest.config.ts`
- `.repo/🎫️/26/06/03/WIRES-KIT-RELATIONSHIP-VIEW/fix-empty-edges.md`

## Verification

- `bun ./📜️script.ts test --run -t "sketchpadKitWires|kit wires"` — 10 passed.
