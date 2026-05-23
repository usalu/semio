# Kit Rename Duplicate Tx And Sketchpad Materialization

**Id:** `2026/05/09/KIT-RENAME-DUPLICATE-AND-MATERIALIZATION`

## Problems

1. Two `renameKit` transactions per edit — lazy `Input` fired `onLazyChange` on Enter and again on the blur that Enter triggers.
2. Details panel showed stale title (`Untitled`) — `useKitName()` reads an unscoped GraphQL `wip { theKit { name } }` field; host kit from `fetchFullKit()` / `applyKitClientSnapshotToLocalStore` reflects footer read scope.

## Changes

- `elements/ui/index.tsx` — `skipLazyBlurCommitRef` so Enter-key commit is not duplicated on blur.
- `semio/sketchpad/index.tsx` — kit name field uses `kit?.name` from `useKitSnapshotTriad()` (materialized host store); removed `useKitName` from `KitSectionForm`.

## Verification

- `elements/ui`: `npm test` (no test files; exit 0).

## Status

Closed.
