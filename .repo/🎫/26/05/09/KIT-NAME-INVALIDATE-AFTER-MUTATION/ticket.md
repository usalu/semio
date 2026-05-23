# Kit Name Refetch After Successful Mutation

**Id:** `2026/05/09/KIT-NAME-INVALIDATE-AFTER-MUTATION`

## Problem

Renaming a kit in the sketchpad details panel did not update the displayed name: `useKitName()` reads `KitStore.kitName` (`StoreField` fed by GraphQL `wip { theKit { name } }`). Invalidations were driven mainly by `KIT_EVENT_STREAM_SUBSCRIPTION`, while mutation completion for the correlator uses `KIT_COMMAND_SUCCEEDED_SUBSCRIPTION`. Successful rename could resolve without firing the event-stream path, so `kitName` never refetched.

## Fix

After each successful `KitStore.operation()` (including `renameKit`), call `invalidations.next()` so query-backed `StoreField`s refetch.

## Files

- `semio/js/index.ts`

## Verification

- `cd semio/js && npm test` → **30 passed** (2026-05-09).

## Status

Closed.
