---
goal: R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS
---

# Ticket

## Summary

Bulk close

## Changes

### Undo Redo Migration Analysis 2026-03-04

- Compared legacy undo/redo integration points in:
  - `compose/js/sketchpad/Desing.tsx.old`
  - `compose/js/sketchpad/Console.tsx.old`
  - `compose/js/sketchpad/Design.Details.tsx.old`
  - `compose/js/sketchpad/Design.Diagram.tsx.old`
  - `compose/js/sketchpad/Design.Model.tsx.old`
- Compared current migration state in:
  - `compose/js/sketchpad/Design.tsx`
  - `compose/js/sketchpad/Sketchpad.tsx`
  - `compose/js/sketchpad/elements.tsx`
- No product code changed in this pass.

### Legacy Behavior Inventory

- `Desing.tsx.old` exposed undo/redo through keyboard shortcuts:
  - `ctrl+z` => `undo()`
  - `ctrl+y` and `ctrl+shift+z` => `redo()`
- `Console.tsx.old` exposed undo/redo as explicit commands:
  - registered `undo` and `redo` commands
  - routed them through `commands.undo()` / `commands.redo()`
  - wrapped non-editor commands in `startTransaction` / `finalizeTransaction` / `abortTransaction`
- `Design.Details.tsx.old` treated edit sessions as transaction scopes:
  - text inputs started on focus and finalized on blur
  - steppers started on pointer down, finalized on pointer up, aborted on pointer cancel
  - add/remove/reorder mutations were explicitly wrapped in one transaction each
- `Design.Diagram.tsx.old` grouped pointer drag into a single undo item:
  - drag start selected the target and called `startTransaction()`
  - `Escape` during drag called `abortTransaction()`
  - drag stop called `finalizeTransaction()`
- `Design.Model.tsx.old` carried the same transaction pattern conceptually for transform controls, but the actual transform-control flow was commented out, so it was not a complete runtime feature.

### Current Migration State

- The current Design app already has the core undo/redo engine:
  - `Sketchpad.tsx` `PlainKitDiffAppStore` records `kitDiff` + `selectionDiff`, maintains `pastTransactionsStack`, `redoStack`, supports `undo`, `redo`, `abortTransaction`, and merges active transaction edits on finalize.
  - `Design.tsx` `DesignStore` routes `compose.designApp.startTransaction`, `compose.designApp.finalizeTransaction`, `compose.designApp.abortTransaction`, `compose.designApp.undo`, and `compose.designApp.redo` into that store.
- The current Design app already has the primary user-facing triggers:
  - `useDesignAppUndo()` and `useDesignAppRedo()` are implemented.
  - `ctrl+z`, `ctrl+y`, and `ctrl+shift+z` are already wired in `Design.tsx`.
- The current UI transaction model is cleaner than the legacy one:
  - `DesignAppTransactionProvider` wraps the app once.
  - `elements.tsx` `TransactionProvider` / `useTransaction()` let controls self-manage start/finalize/abort.
  - `Input` supports lazy commit with `Enter` finalize and `Escape` abort.
  - `Combobox` and `ActionDropdown` start/finalize around open/close and selection.
- The current diagram drag flow already preserves the old grouped-history intent:
  - drag start defers `transaction.start()`
  - `Escape` aborts and resets transient drag refs
  - drag stop schedules `updatePieces(...)` and then `transaction.finalize()`

### Migration Gaps

- The old Console command entry point is not migrated:
  - there is no current `compose/js/sketchpad/Console.tsx`
  - the legacy `undo` / `redo` command registration has no current equivalent command surface
- Undo/redo regression coverage is missing in the current test suite:
  - grep found no undo/redo assertions in `compose/js/sketchpad.test.ts`
  - current migration risk is behavioral regressions in transaction grouping, not missing store primitives
- The old Model transform undo story remains effectively unported:
  - legacy code only sketched transform-control transaction handling
  - if 3D transform interactions are still planned, they need a new transaction wrapper in the current Scene flow rather than a direct port

### Implementation-Ready Migration Plan

1. Keep the current store/history architecture as the source of truth. Do not reintroduce legacy `useDesignEditorCommands` patterns.
2. If a command surface is still required, reintroduce undo/redo through the current command system by calling `store.execute(\"compose.designApp.undo\", origin)` and `store.execute(\"compose.designApp.redo\", origin)` or the existing `useDesignAppUndo()` / `useDesignAppRedo()` hooks.
3. Extend the existing test file only (`compose/js/sketchpad.test.ts`) with explicit coverage for:
   - one text-field edit => one undo step
   - one drag gesture => one undo step
   - `Escape` during drag => aborts without persisting the move
   - redo restores the reverted move/edit
4. Audit any remaining direct mutation paths in `Design.tsx` and ensure they either:
   - run inside `DesignAppTransactionProvider`-aware controls, or
   - explicitly call the transaction hook for pointer-driven gestures
5. Treat Scene/3D transform history as a separate follow-up migration, because the old implementation was incomplete and should be rebuilt on current transaction hooks instead of copied.

## Log

- `./repo/cli/cli tree "undo redo"` was executed first for repo context; the CLI accepted the query only as a single argument.
- Opened a new ticket under `R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS`.
- Reviewed the legacy files and current store/hooks to separate already-migrated behavior from remaining gaps.

## Todos

- [x] Inspect legacy undo/redo touchpoints in the old Design editor files.
- [x] Inspect current Design app store and UI transaction architecture.
- [x] Identify which legacy behaviors are already migrated.
- [x] Identify what is still missing or risky.
- [x] Write an implementation-ready migration plan in the ticket.

## Plan

- Map legacy undo/redo entry points and transaction boundaries.
- Compare them with the current Design app history stack and UI transaction hooks.
- Record the gaps and the safest migration path without reintroducing legacy architecture.
