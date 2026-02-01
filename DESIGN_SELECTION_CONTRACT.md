# Design Selection Contract

**Location:** `js/semio/sketchpad/Design.tsx`

A comprehensive specification of Design app's selection system, entry points, and helper hooks.

---

## 1. Selection State Shape

### `DesignAppSelection`

```typescript
interface DesignAppSelection {
  pieces?: Guid[];
  connections?: Guid[];
  connectors?: Array<{ piece: Guid; connector: Guid }>;
  connector?: { piece: Guid; designPiece?: Guid; connector: Guid };
}
```

**Components:**

| Component | Type | Purpose | Notes |
|-----------|------|---------|-------|
| `pieces` | `Guid[]` | Array of selected piece GUIDs | Undefined when empty |
| `connections` | `Guid[]` | Array of selected connection GUIDs | Undefined when empty |
| `connectors` | `Array<{piece, connector}>` | Multi-select connectors | Unused in current implementation |
| `connector` | `{piece, designPiece?, connector}` | **Single** selected port/connector | Used for connection mode. `designPiece` is optional for hierarchical pieces |

**Invariants:**

- Only **one** of `pieces` or `connections` should have items at a time (UI enforces exclusivity)
- `connector` is independent and can coexist with pieces/connections
- Empty selections are represented by `undefined` fields, not empty arrays

---

## 2. Selection Events

Selection mutations are triggered via **XState events** dispatched to the Sketchpad actor.

### Event Dispatch Pattern

All selection-related events follow this pattern:

```typescript
actor.send({
  type: "DESIGN.SET_SELECTION",
  kitGuid: Guid,
  designGuid: Guid,
  selection: DesignAppSelection
});
```

### Selection-Specific Events

| Event Type | Parameters | Effect | Notes |
|------------|-----------|--------|-------|
| `DESIGN.SET_SELECTION` | `kitGuid`, `designGuid`, `selection: DesignAppSelection` | Replace entire selection | Used by state management hooks |
| `DESIGN.SYNC` | `kitGuid`, `designGuid`, `state: Partial<DesignAppState>` | Partial state sync | Alternative dispatch for non-selection mutations |

### Related Events (Non-Selection)

These mutations don't directly change selection but interact with the selection system:

| Event Type | Parameters | Effect |
|------------|-----------|--------|
| `DESIGN.SET_HOVER` | `kitGuid`, `designGuid`, `hover: DesignAppHover` | Set hover state |
| `DESIGN.CLEAR_HOVER` | `kitGuid`, `designGuid` | Clear all hover highlights |
| `DESIGN.FOCUS_PIECE` | `kitGuid`, `designGuid`, `pieceGuid: Guid` | Focus a single piece |

---

## 3. Selection Helper Hooks

All hooks follow the **triadic pattern**: `[value, setter, canSet]`

### Command-Based Hooks (Entry Points)

These are the primary selection manipulation hooks. All call internal commands through the app actor.

#### **Piece Selection**

##### `useDesignAppSelectPiece()`

Replaces selection with a single piece.

```typescript
ActionHookResult<[pieceGuid: string]>

// Usage
const [selectPiece, canSelectPiece] = useDesignAppSelectPiece();
selectPiece?.(pieceGuid);
```

**Behavior:**
- Clears previous piece selection
- Clears connection selection
- Clears connector selection

**Command:** `semio.designApp.selectPiece`

---

##### `useDesignAppSelectPieces()`

Replaces selection with multiple pieces.

```typescript
ActionHookResult<[pieceGuids: string[]]>

// Usage
const [selectPieces, canSelectPieces] = useDesignAppSelectPieces();
selectPieces?.([guid1, guid2, guid3]);
```

**Behavior:**
- Clears previous piece selection
- Clears connection selection
- Clears connector selection

**Command:** `semio.designApp.selectPieces`

---

##### `useDesignAppAddPieceToSelection()`

Adds a piece to the current selection without clearing others.

```typescript
ActionHookResult<[pieceGuid: string]>

// Usage
const [addPiece, canAddPiece] = useDesignAppAddPieceToSelection();
addPiece?.(pieceGuid);
```

**Behavior:**
- Adds to `selection.pieces` array
- No-op if piece already selected
- Preserves other selected pieces

**Command:** `semio.designApp.addPieceToSelection`

---

##### `useDesignAppRemovePieceFromSelection()`

Removes a piece from the current selection.

```typescript
ActionHookResult<[pieceGuid: string]>

// Usage
const [removePiece, canRemovePiece] = useDesignAppRemovePieceFromSelection();
removePiece?.(pieceGuid);
```

**Behavior:**
- Removes from `selection.pieces` array
- No-op if piece not selected
- Preserves other selected pieces

**Command:** `semio.designApp.removePieceFromSelection`

---

#### **Connection Selection**

##### `useDesignAppSelectConnection()`

Replaces selection with a single connection.

```typescript
ActionHookResult<[connectionGuid: string]>

// Usage
const [selectConnection, canSelectConnection] = useDesignAppSelectConnection();
selectConnection?.(connectionGuid);
```

**Behavior:**
- Clears previous connection selection
- Clears piece selection
- Clears connector selection

**Command:** `semio.designApp.selectConnection`

---

##### `useDesignAppAddConnectionToSelection()`

Adds a connection to the current selection.

```typescript
ActionHookResult<[connectionGuid: string]>

// Usage
const [addConnection, canAddConnection] = useDesignAppAddConnectionToSelection();
addConnection?.(connectionGuid);
```

**Behavior:**
- Adds to `selection.connections` array
- No-op if connection already selected
- Preserves other selected connections

**Command:** `semio.designApp.addConnectionToSelection`

---

##### `useDesignAppRemoveConnectionFromSelection()`

Removes a connection from the current selection.

```typescript
ActionHookResult<[connectionGuid: string]>

// Usage
const [removeConnection, canRemoveConnection] = useDesignAppRemoveConnectionFromSelection();
removeConnection?.(connectionGuid);
```

**Behavior:**
- Removes from `selection.connections` array
- No-op if connection not selected
- Preserves other selected connections

**Command:** `semio.designApp.removeConnectionFromSelection`

---

#### **Connector/Port Selection**

##### `useDesignAppSelectPiecePort()`

Selects a single connector/port on a piece (used for connection mode).

```typescript
ActionHookResult<[pieceGuid: string, connectorGuid: string, designPieceGuid?: string]>

// Usage
const [selectPort, canSelectPort] = useDesignAppSelectPiecePort();
selectPort?.(pieceGuid, connectorGuid, designPieceGuid);
```

**Behavior:**
- Sets `selection.connector` to `{piece, connector, designPiece?}`
- Clears piece and connection selections
- `designPieceGuid` is optional (for hierarchical piece references)

**Command:** `semio.designApp.selectPiecePort`

---

##### `useDesignAppDeselectPiecePort()`

Deselects the current port/connector.

```typescript
ActionHookResult<[]>

// Usage
const [deselectPort, canDeselectPort] = useDesignAppDeselectPiecePort();
deselectPort?.();
```

**Behavior:**
- Clears `selection.connector`
- Preserves piece and connection selections

**Command:** `semio.designApp.deselectPiecePort`

---

#### **Bulk Operations**

##### `useDesignAppSelectAll()`

Selects all pieces and connections in the design.

```typescript
ActionHookResult<[]>

// Usage
const [selectAll, canSelectAll] = useDesignAppSelectAll();
selectAll?.();
```

**Behavior:**
- Collects all piece GUIDs from `design.pieces`
- Collects all connection GUIDs from `design.connections`
- Replaces entire selection
- Clears connector selection

**Command:** `semio.designApp.selectAll`

---

##### `useDesignAppDeselectAll()`

Clears all selections (pieces, connections, connectors).

```typescript
ActionHookResult<[]>

// Usage
const [deselectAll, canDeselectAll] = useDesignAppDeselectAll();
deselectAll?.();
```

**Behavior:**
- Clears `selection.pieces`
- Clears `selection.connections`
- Clears `selection.connector`
- Results in `selection = {}`

**Command:** `semio.designApp.deselectAll`

---

### State Access Hooks

These read selection state without mutations.

#### `useDesignAppSelection()`

Reads the full selection object.

```typescript
HookResult<DesignAppSelection>

// Usage
const [selection, setSelection, canSetSelection] = useDesignAppSelection();
// selection = { pieces: [...], connections: [...], connector: {...} }
```

**Returns:**
- `value`: Current `DesignAppSelection` object
- `setter`: Direct replacement function (advanced usage)
- `canSet`: Boolean indicating if mutations are allowed

**Sources:** XState selector from Sketchpad actor

---

#### `useDesignAppSelectionField()`

Reads selection as a `Field<T>` object pattern.

```typescript
Field<DesignAppSelection>

// Usage
const field = useDesignAppSelectionField();
// field.value = { pieces: [...], connections: [...] }
// field.set(newSelection) - alternative to hook setter
// field.canSet = true/false
```

**Returns:**
- `field.value`: Current selection
- `field.set`: Setter function (no-op if `canSet` is false)
- `field.canSet`: Mutation availability

**Note:** This is the underlying hook used by `useDesignAppSelection()`

---

### Focus Hooks

##### `useDesignAppFocusPiece()`

Sets focus on a single piece (independent of selection).

```typescript
ActionHookResult<[pieceGuid: string]>

// Usage
const [focusPiece, canFocus] = useDesignAppFocusPiece();
focusPiece?.(pieceGuid);
```

**Behavior:**
- Updates `focusedPieceGuid` in app state
- Separate from selection system
- Used for inspection/details panels

**Command:** `semio.designApp.focusPiece`

---

##### `useDesignAppClearFocus()`

Clears focus on the current piece.

```typescript
ActionHookResult<[]>

// Usage
const [clearFocus, canClearFocus] = useDesignAppClearFocus();
clearFocus?.();
```

**Command:** `semio.designApp.clearFocus`

---

---

## 4. Event Handlers & UI Integration

### Click Handlers (Example Patterns)

**Single Select (Replace):**
```tsx
onClick={() => selectPiece?.(pieceGuid)}
```

**Ctrl/Cmd + Click (Add to Selection):**
```tsx
onClick={(e) => {
  if (e.ctrlKey || e.metaKey) {
    addPiece?.(pieceGuid);
  } else {
    selectPiece?.(pieceGuid);
  }
}}
```

**Shift + Click (Range Selection - Not Directly Supported):**
Currently not implemented in selection hooks. Would require:
1. Tracking previous selection
2. Computing piece indices in design
3. Adding all pieces between previous and current

**Right-Click (Context Menu):**
```tsx
onContextMenu={(e) => {
  e.preventDefault();
  // Open context menu
  // May select piece first if not already selected
}}
```

### Diagram Click Handlers

In `Diagram` components, clicks dispatch through React Flow callbacks:

```tsx
// On node click
onNodeClick={(event, node) => {
  const pieceGuid = node.id;
  
  if (event.ctrlKey || event.metaKey) {
    addPiece?.(pieceGuid);
  } else if (event.shiftKey) {
    // Range select (not implemented)
  } else {
    selectPiece?.(pieceGuid);
  }
}}
```

### Modifier Key Semantics

| Key(s) | Behavior | Hook |
|--------|----------|------|
| **None** | Replace selection | `selectPiece` or `selectConnection` |
| **Ctrl** / **Cmd** | Toggle add/remove | `addPieceToSelection` / `removePieceFromSelection` |
| **Shift** | Range select | Not implemented |
| **Alt** | Hover/preview | Separate hover system |

---

## 5. Inverse Diff Mechanism

### Function: `inverseDesignAppSelectionDiff()`

Calculates the inverse transformation to undo selection changes.

```typescript
export const inverseDesignAppSelectionDiff = (
  selection: DesignAppSelection,
  diff: DesignAppSelectionDiff
): DesignAppSelectionDiff => {
  const inverseDiff: DesignAppSelectionDiff = {};

  if (diff.pieces) {
    inverseDiff.pieces = {};
    if (diff.pieces.added) {
      inverseDiff.pieces.removed = diff.pieces.added;
    }
    if (diff.pieces.removed) {
      inverseDiff.pieces.added = diff.pieces.removed;
    }
  }

  if (diff.connections) {
    inverseDiff.connections = {};
    if (diff.connections.added) {
      inverseDiff.connections.removed = diff.connections.added;
    }
    if (diff.connections.removed) {
      inverseDiff.connections.added = diff.connections.removed;
    }
  }

  if (diff.connector) {
    inverseDiff.connector = {
      piece: selection.connector?.piece,
      designPiece: selection.connector?.designPiece,
      connector: selection.connector?.connector,
    };
  }

  return inverseDiff;
};
```

### Logic

**Pieces/Connections:**
- **Forward:** `added: [id]` → **Inverse:** `removed: [id]`
- **Forward:** `removed: [id]` → **Inverse:** `added: [id]`
- Swaps add/remove operations

**Connector:**
- **Forward:** `{piece, connector}` → **Inverse:** Restore previous `selection.connector`
- Requires original selection snapshot to restore the old value
- If new connector is set, inverse restores the old one

### Undo/Redo Flow

1. **User action:** `selectPiece(id1)` replaces selection from `{pieces: [id0]}` to `{pieces: [id1]}`
   - Forward diff: `{pieces: {removed: [id0], added: [id1]}}`
   - Inverse diff: `{pieces: {removed: [id1], added: [id0]}}`

2. **On undo:** Apply inverse diff to restore `{pieces: [id0]}`

3. **On redo:** Apply forward diff to restore `{pieces: [id1]}`

### Integration with Transaction System

The inverse diff is calculated in `DesignStore.executeCommand()`:

```typescript
const edit = {
  do: { selectionDiff: result.diff?.selection },
  undo: { selectionDiff: inverseDesignAppSelectionDiff(currentSelection, result.diff?.selection) },
};
this.recordEdit(edit);
```

---

## 6. Selection Diff Types

### `DesignAppSelectionDiff`

Represents changes to selection.

```typescript
interface DesignAppSelectionDiff {
  pieces?: DesignAppSelectionPiecesDiff;
  connections?: DesignAppSelectionConnectionsDiff;
  connector?: DesignAppSelectionPortDiff;
}

interface DesignAppSelectionPiecesDiff {
  added?: Guid[];
  removed?: Guid[];
}

interface DesignAppSelectionConnectionsDiff {
  added?: Guid[];
  removed?: Guid[];
}

interface DesignAppSelectionPortDiff {
  piece?: Guid;
  designPiece?: Guid;
  connector?: Guid;
}
```

**Semantics:**

- Only changed fields appear in diff (others are `undefined`)
- Empty arrays are omitted
- `undefined` means no change
- When both `added` and `removed` are present: first removes, then adds

---

## 7. Commands Registry

All selection mutations are registered as commands in `DesignStore.commandRegistry`.

### Command Context

```typescript
interface DesignAppCommandContext extends KitCommandContext {
  designApp: DesignAppState;
  Guid: Guid;
  design: Design;
}
```

Provides current state snapshot for command logic to decide which pieces to remove/add.

### Command Results

```typescript
interface DesignAppCommandResult {
  diff?: DesignAppDiff;
  kitDiff?: KitDiff;
}
```

Commands return:
- **Selection commands:** `{diff: {selection: DesignAppSelectionDiff}}`
- **Piece/connection modifications:** `{kitDiff: {designs: {...}}}`
- **Combined operations:** Both `diff` and `kitDiff` (e.g., `deleteSelected`)

---

## 8. Transaction Lifecycle

Selection changes participate in the transaction system.

### Flow

1. **Command execution:**
   ```typescript
   await designStore.executeCommand("semio.designApp.selectPiece", pieceGuid);
   ```

2. **Inside command handler:**
   - Read current selection from `context.designApp.selection`
   - Calculate diff (added/removed)
   - Return `{diff: {selection: ...}}`

3. **Edit recording (if in transaction):**
   ```typescript
   const edit = {
     do: { selectionDiff: diff.selection },
     undo: { selectionDiff: inverseDesignAppSelectionDiff(oldSelection, diff.selection) },
   };
   this.recordEdit(edit);
   ```

4. **Apply to state:**
   - `applySelectionDiff(selectionDiff)` updates Y.js backing store
   - Subscribers notified via `onChanged`

5. **Transaction finalize/abort:**
   - All edits in `currentTransactionStack` merged into one edit
   - Moved to `pastTransactionStack`
   - Redo stack cleared

---

## 9. Summary Table

### Selection Hooks Quick Reference

| Hook | Input(s) | Effect | Use Case |
|------|----------|--------|----------|
| `useDesignAppSelectPiece` | `pieceGuid` | Replace with 1 piece | Click to select |
| `useDesignAppSelectPieces` | `pieceGuids[]` | Replace with N pieces | Batch select |
| `useDesignAppAddPieceToSelection` | `pieceGuid` | Add to current | Ctrl+click |
| `useDesignAppRemovePieceFromSelection` | `pieceGuid` | Remove from current | Ctrl+click to deselect |
| `useDesignAppSelectConnection` | `connectionGuid` | Replace with 1 connection | Click connection |
| `useDesignAppAddConnectionToSelection` | `connectionGuid` | Add connection | Ctrl+click connection |
| `useDesignAppRemoveConnectionFromSelection` | `connectionGuid` | Remove connection | Ctrl+click to deselect |
| `useDesignAppSelectPiecePort` | `pieceGuid, connectorGuid, designPieceGuid?` | Select connector | Enter connection mode |
| `useDesignAppDeselectPiecePort` | (none) | Clear connector | Exit connection mode |
| `useDesignAppSelectAll` | (none) | Select all pieces & connections | Cmd+A |
| `useDesignAppDeselectAll` | (none) | Clear all selections | Escape or menu |
| `useDesignAppFocusPiece` | `pieceGuid` | Focus piece (independent) | Show details panel |
| `useDesignAppClearFocus` | (none) | Clear focus | Close details panel |

---

## 10. Special Cases & Notes

### Piece vs Connection Mutual Exclusivity

When selecting a piece, the command handler explicitly clears connections:

```typescript
return {
  diff: {
    selection: {
      pieces: { removed: currentPieces, added: [guid] },
      connections: { removed: currentSelection.connections || [] },
      connector: {}, // Also clear connector
    },
  },
};
```

This is **enforced at command level**, not UI level.

### Connector as Independent State

The `connector` field in `DesignAppSelection` is:
- **Independent** of pieces/connections
- **Set together** with clearing pieces/connections
- **Represents** the active port for connection creation mode

### DesignPiece Hierarchy

The `designPiece` field in `connector` allows:
- Reference to a piece that is an **instance** of another design (nested designs)
- Distinguish between the piece itself and its instantiated design context

### Empty Selection Representation

```typescript
// ❌ Wrong
selection = { pieces: [], connections: [] }

// ✅ Correct
selection = {}
// or
selection = { pieces: undefined, connections: undefined }
```

Undefined fields indicate "no selection" for that type.

---

## File References

- **Selection types:** Lines 218-240 in Design.tsx
- **Commands:** Lines 310-898 in Design.tsx
- **Inverse diff:** Lines 904-935 in Design.tsx
- **Selection hooks:** Lines 1411-1850+ in Design.tsx
- **Store implementation:** Lines 940-1350+ in Design.tsx
