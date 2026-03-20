# Ticket

## Todos
# Previously

Components in Design.tsx were directly using `useDesignAppCommands()` hook and destructuring transaction methods (`startTransaction`, `finalizeTransaction`, `abortTransaction`) and kit mutation methods (`addPiece`, `updatePiece`, etc.) for use in components. This violated the intended architecture where:

- Components should ONLY use triadic hooks for read and write
- Hooks should abstract implementation (XState for writes, store for reads)
- Only the sketchpad machine should use commands

# Plan

1. Create new triadic/dyadic hooks for transactions and kit mutations
2. Update all components to use these new hooks instead of `useDesignAppCommands()`
3. Keep `sharedCommandsRef` pattern for ReactFlow memoized node components (can't use hooks)
4. Verify no TypeScript errors after refactoring

# Changes

## Created New Hooks (around line 2119)

### Transaction Hook

```typescript
export function useDesignAppTransaction(): [TransactionActions | undefined, boolean];
```

Returns `TransactionActions` object with `start()`, `finalize()`, `abort()` methods.

### Kit Mutation Hooks

```typescript
export function useDesignAppAddPiece(): [(origin: string, piece: Piece) => void, boolean];
export function useDesignAppUpdatePiece(): [(origin: string, pieceId: string, diff: any) => void, boolean];
export function useDesignAppUpdatePieces(): [(origin: string, updates: Array<{ id: string; diff: any }>) => void, boolean];
export function useDesignAppAddConnection(): [(origin: string, connection: Connection) => void, boolean];
export function useDesignAppAddConnections(): [(origin: string, connections: Connection[]) => void, boolean];
export function useDesignAppUpdateConnections(): [(origin: string, updates: Array<{ id: string; diff: any }>) => void, boolean];
export function useDesignAppDeleteSelected(): [() => void, boolean];
export function useDesignAppUndo(): [() => void, boolean];
export function useDesignAppRedo(): [() => void, boolean];
```

## Updated Components

### DesignSectionForm (line 3211)

- Changed from `useDesignAppCommands()` destructuring to `[transaction] = useDesignAppTransaction()`
- Updated all transaction patterns: `transaction?.start()`, `transaction?.finalize()`, `transaction?.abort()`

### PiecesSectionForm (line 3706)

- Uses `[transaction]`, `[updatePiece]`, `[updatePieces]` hooks

### SingleConnectionFields (line 4323)

- Uses `[transaction]` hook

### DesignNodeComponent (line 5109)

- Uses `[addConnectionAction] = useDesignAppAddConnection()`

### DesignDiagram (line 5789)

- Uses all new hooks for transaction, addPiece, updatePieces, addConnection, addConnections, updateConnections

### ModelDesign (line 7503)

- Uses `[transaction]`, `[updatePiece]` hooks

### DesignAppScene (line 7564)

- Uses `[transaction]`, `[addPiece]` hooks

### App (line 7721)

- Uses `[transaction]`, `[deleteSelected]`, `[undo]`, `[redo]`, `[addPiece]` hooks

## Pattern Transformation

Old pattern:

```typescript
const { startTransaction, finalizeTransaction, abortTransaction } = useDesignAppCommands();
startTransaction?.("origin");
finalizeTransaction?.("origin");
abortTransaction?.("origin");
```

New pattern:

```typescript
const [transaction] = useDesignAppTransaction();
transaction?.start("origin");
transaction?.finalize("origin");
transaction?.abort("origin");
```

## Preserved Exception

The `sharedCommandsRef = commands` pattern in DesignDiagram is preserved. This is needed because ReactFlow's memoized node components (`PieceNodeComponent`) cannot use hooks directly, so they access commands via this shared ref.

## Verification

- No TypeScript errors after refactoring
- Only 1 remaining `useDesignAppCommands()` usage - the legitimate `sharedCommandsRef` pattern
- No remaining old transaction patterns (`startTransaction?.(`, `finalizeTransaction?.(`, `abortTransaction?.(`)

## Changes

## Log

## Summary
# Summary

>-
