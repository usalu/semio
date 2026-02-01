# Kit Selection Surface Area & Gap Analysis

**Location:** `js/semio/sketchpad/Kit.tsx`

A comprehensive analysis of Kit app's current selection infrastructure and gaps compared to Design app's selection contract.

---

## 1. Selection State Shape

### `KitAppSelection`

```typescript
interface KitAppSelection {
  types?: Guid[];
  designs?: Guid[];
  qualities?: string[];
  ports?: Guid[];
  tags?: Guid[];
  concepts?: Guid[];
  files?: string[];
  folders?: Guid[];
  authors?: string[];
}
```

**9 Selection Dimensions:**

| Dimension | Type | Purpose |
|-----------|------|---------|
| `types` | `Guid[]` | Selected type artifacts |
| `designs` | `Guid[]` | Selected design artifacts |
| `qualities` | `string[]` | Selected quality names |
| `ports` | `Guid[]` | Selected port definitions |
| `tags` | `Guid[]` | Selected tag identifiers |
| `concepts` | `Guid[]` | Selected concept identifiers |
| `files` | `string[]` | Selected file paths |
| `folders` | `Guid[]` | Selected folder identifiers |
| `authors` | `string[]` | Selected author names |

**Key Differences from Design:**

- **Multi-dimensional:** Kit has 9 independent selection dimensions vs Design's 3 (pieces, connections, connectors)
- **Non-exclusive:** Multiple dimensions can be selected simultaneously (e.g., types AND designs AND ports)
- **Simpler structure:** No nested connector objects like Design's `{piece, connector, designPiece}`

---

## 2. Existing Selection Events

Events are registered via `registerEventHandler()` with XState dispatch.

### Implemented Events

| Event Type | Parameters | Implementation Status | Effect |
|------------|-----------|----------------------|--------|
| `KIT.SELECT_TYPE` | `kitGuid`, `typeGuid` | ✅ **Implemented** | Adds typeGuid to selection.types (preserves others) |
| `KIT.DESELECT_TYPE` | `kitGuid`, `typeGuid` | ✅ **Implemented** | Removes typeGuid from selection.types |
| `KIT.SELECT_DESIGN` | `kitGuid`, `designGuid` | ✅ **Implemented** | Adds designGuid to selection.designs (preserves others) |
| `KIT.DESELECT_DESIGN` | `kitGuid`, `designGuid` | ✅ **Implemented** | Removes designGuid from selection.designs |
| `KIT.SET_SELECTION` | `kitGuid`, `selection: KitAppSelection` | ✅ **Stub Hook** | Direct replacement via hook (not event handler) |
| `KIT.CLEAR_SELECTION` | `kitGuid` | ✅ **Stub Hook** | Clears all selections via hook |
| `KIT.SET_HOVER` | `kitGuid`, `hover: KitAppHover` | ✅ **Implemented** | Sets hover state |
| `KIT.CLEAR_HOVER` | `kitGuid` | ✅ **Implemented** | Clears hover state |

### Event Handler Implementation Details

**KIT.SELECT_TYPE:**
```typescript
registerEventHandler("KIT.SELECT_TYPE", {
  action: (context: any, event: any) => {
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    const types = [...(app.selection?.types || [])];
    if (!types.includes(event.typeGuid)) types.push(event.typeGuid);
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, selection: { ...app.selection, types } } } };
  },
});
```

- **Merge-style:** Adds to existing types without clearing other dimensions
- **Duplicate check:** Does not add if already selected
- **Preserves other dimensions:** Keeps designs, qualities, ports, etc.

**KIT.DESELECT_TYPE:**
```typescript
registerEventHandler("KIT.DESELECT_TYPE", {
  action: (context: any, event: any) => {
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    const types = (app.selection?.types || []).filter((t: Guid) => t !== event.typeGuid);
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, selection: { ...app.selection, types } } } };
  },
});
```

- **Merge-style:** Removes from types without affecting other dimensions
- **No-op if not selected:** Filter handles non-existent IDs gracefully

**KIT.SELECT_DESIGN / KIT.DESELECT_DESIGN:**
- Same pattern as types (add/remove without clearing other dimensions)

### Missing Events (Compared to Design)

Kit lacks explicit events for:

- `KIT.SELECT_TYPES` (plural - batch select)
- `KIT.SELECT_QUALITY` / `KIT.DESELECT_QUALITY`
- `KIT.SELECT_PORT` / `KIT.DESELECT_PORT`
- `KIT.SELECT_TAG` / `KIT.DESELECT_TAG`
- `KIT.SELECT_CONCEPT` / `KIT.DESELECT_CONCEPT`
- `KIT.SELECT_FILE` / `KIT.DESELECT_FILE`
- `KIT.SELECT_FOLDER` / `KIT.DESELECT_FOLDER`
- `KIT.SELECT_AUTHOR` / `KIT.DESELECT_AUTHOR`
- `KIT.SELECT_ALL` (select all artifacts)
- `KIT.DESELECT_ALL` (clear all dimensions)

**Why these are missing:**
- Kit currently only implements type/design selection via events
- Other dimensions rely on `KIT.SET_SELECTION` hook for direct replacement
- No batch/plural selection events

---

## 3. Existing Selection Hooks

Kit provides **12 selection-related hooks**.

### State Access Hooks

#### `useKitAppSelection()`

Read/write the full selection object.

```typescript
HookResult<KitAppSelection>

// Usage
const [selection, setSelection, canSetSelection] = useKitAppSelection();
// selection = { types: [...], designs: [...], qualities: [...], ... }
```

**Implementation:**
- Reads via XState selector from Sketchpad actor
- Wraps `createKitSelectionSelector(kitGuid)`
- Returns triadic pattern: `[value, setter, canSet]`

**Status:** ✅ Fully implemented

---

### Action Hooks (Dimension-Specific)

#### Types

##### `useKitAppSelectType()`

Adds a single type to selection (merge-style).

```typescript
ActionHookResult<[typeGuid: Guid]>

// Usage
const [selectType, canSelectType] = useKitAppSelectType();
selectType?.(typeGuid);
```

**Behavior:**
- Dispatches `KIT.SELECT_TYPE` event
- Preserves other selected types
- Preserves other selection dimensions (designs, qualities, etc.)
- No-op if type already selected

**Status:** ✅ Implemented

---

##### `useKitAppDeselectType()`

Removes a single type from selection.

```typescript
ActionHookResult<[typeGuid: Guid]>

// Usage
const [deselectType, canDeselectType] = useKitAppDeselectType();
deselectType?.(typeGuid);
```

**Behavior:**
- Dispatches `KIT.DESELECT_TYPE` event
- Preserves other selected types
- Preserves other selection dimensions
- No-op if type not selected

**Status:** ✅ Implemented

---

#### Designs

##### `useKitAppSelectDesign()`

Adds a single design to selection (merge-style).

```typescript
ActionHookResult<[designGuid: Guid]>

// Usage
const [selectDesign, canSelectDesign] = useKitAppSelectDesign();
selectDesign?.(designGuid);
```

**Status:** ✅ Implemented (same pattern as types)

---

##### `useKitAppDeselectDesign()`

Removes a single design from selection.

```typescript
ActionHookResult<[designGuid: Guid]>

// Usage
const [deselectDesign, canDeselectDesign] = useKitAppDeselectDesign();
deselectDesign?.(designGuid);
```

**Status:** ✅ Implemented (same pattern as types)

---

#### Bulk Operations

##### `useKitAppSetSelection()`

Directly replaces the entire selection object.

```typescript
ActionHookResult<[selection: KitAppSelection]>

// Usage
const [setSelection, canSetSelection] = useKitAppSetSelection();
setSelection?.({ types: [guid1, guid2], designs: [guid3] });
```

**Behavior:**
- Dispatches `KIT.SET_SELECTION` event
- Replaces entire selection (not merge)
- Low-level hook (advanced usage)

**Status:** ✅ Implemented

---

##### `useKitAppClearSelection()`

Clears all selection dimensions.

```typescript
ActionHookResult<[]>

// Usage
const [clearSelection, canClearSelection] = useKitAppClearSelection();
clearSelection?.();
```

**Behavior:**
- Dispatches `KIT.CLEAR_SELECTION` event
- Results in `selection = {}`

**Status:** ✅ Implemented

---

### Other Hooks

##### `useKitAppSetFilter()` / `useKitAppToggleRow()` / `useKitAppSetSort()` / `useKitAppToggleSort()`

Not selection-related, but affect filtered/visible artifacts.

##### `useKitAppSetHover()` / `useKitAppClearHover()`

Hover state management (separate from selection).

---

## 4. Gap Analysis: Kit vs Design

### What Kit Has (Design Doesn't)

✅ **Multi-dimensional selection:** 9 independent dimensions vs Design's 3  
✅ **Merge-style by default:** `SELECT_TYPE` adds without clearing (Design's `selectPiece` replaces)  
✅ **Dimension preservation:** Selecting a type doesn't clear designs/qualities/etc.

### What Kit Is Missing (Design Has)

#### Selection Helpers Per Dimension

Design has **4 helpers per dimension** (piece/connection):
1. **Select single (replace):** `selectPiece(id)` - replaces entire dimension
2. **Select multiple (replace):** `selectPieces(ids)` - batch replace
3. **Add to selection:** `addPieceToSelection(id)` - merge
4. **Remove from selection:** `removePieceFromSelection(id)` - merge

Kit has **2 helpers per dimension** (types/designs only):
1. **Add to selection:** `selectType(id)` - merge only
2. **Remove from selection:** `deselectType(id)` - merge only

**Missing for Kit:**
- ❌ **Replace-style helpers:** No `useKitAppSelectSingleType()` (clears other types)
- ❌ **Batch helpers:** No `useKitAppSelectTypes([id1, id2])` (replace with multiple)
- ❌ **Toggle helpers:** No `useKitAppToggleTypeInSelection()` (add if missing, remove if present)
- ❌ **Clear dimension:** No `useKitAppClearTypes()` (clear just types, keep others)

#### Coverage Gaps

Kit only has add/remove hooks for **2 out of 9 dimensions**:

| Dimension | Select (Add) | Deselect (Remove) | Replace Single | Replace Batch | Toggle | Clear Dimension |
|-----------|-------------|-------------------|---------------|---------------|--------|-----------------|
| **types** | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ |
| **designs** | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ |
| qualities | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| ports | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| tags | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| concepts | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| files | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| folders | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| authors | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |

**Total Gaps:**
- ❌ **7 dimensions** have no dedicated hooks (qualities, ports, tags, concepts, files, folders, authors)
- ❌ **4 helper types** missing for all dimensions (replace single, replace batch, toggle, clear dimension)

#### Select All / Deselect All

Design has:
- ✅ `useDesignAppSelectAll()` - selects all pieces AND connections
- ✅ `useDesignAppDeselectAll()` - clears all selections

Kit has:
- ✅ `useKitAppClearSelection()` - clears all selections
- ❌ **No `useKitAppSelectAll()`** - would need to select all types, designs, qualities, ports, tags, concepts, files, folders, authors

**Missing:** Intelligent select-all that respects current filters/visibility.

#### Modifier Key Strategy

Design documents:
- **Click:** Replace selection (uses `selectPiece`)
- **Ctrl/Cmd+Click:** Toggle (uses `addPieceToSelection` / `removePieceFromSelection`)
- **Shift+Click:** Range select (not implemented)

Kit currently:
- **Click:** Uses `selectType` which **adds** (merge-style)
- **Ctrl/Cmd+Click:** Uses `deselectType` (remove)
- **No replace-on-click** without explicitly calling `setSelection({})`

**Missing:** Clear semantics for "replace vs add" based on modifier keys.

---

## 5. Selection Inverse Diff

### Function: `inverseKitAppSelectionDiff()`

```typescript
export const inverseKitAppSelectionDiff = (
  selection: KitAppSelection,
  diff: KitAppSelectionDiff
): KitAppSelectionDiff => {
  const inverseDiff: KitAppSelectionDiff = {};

  if (diff.types) {
    inverseDiff.types = {};
    if (diff.types.added) inverseDiff.types.removed = diff.types.added;
    if (diff.types.removed) inverseDiff.types.added = diff.types.removed;
  }

  if (diff.designs) {
    inverseDiff.designs = {};
    if (diff.designs.added) inverseDiff.designs.removed = diff.designs.added;
    if (diff.designs.removed) inverseDiff.designs.added = diff.designs.removed;
  }

  // ... same for qualities, files, folders, authors
  // NOTE: ports, tags, concepts are NOT implemented in inverse diff function!

  return inverseDiff;
};
```

**Status:** ⚠️ **Partial Implementation**

**Implemented dimensions:**
- ✅ types
- ✅ designs
- ✅ qualities
- ✅ files
- ✅ folders
- ✅ authors

**Missing dimensions:**
- ❌ **ports** - not in inverse diff
- ❌ **tags** - not in inverse diff
- ❌ **concepts** - not in inverse diff

**Logic:**
- Same swap pattern as Design (added ↔ removed)
- Simpler than Design (no connector state restoration)

---

## 6. Summary Table: Selection Hook Coverage

| Hook Category | Design Coverage | Kit Coverage | Gap |
|--------------|----------------|--------------|-----|
| **State Access** | ✅ `useDesignAppSelection()` | ✅ `useKitAppSelection()` | None |
| **Replace Single** | ✅ `selectPiece(id)` | ❌ None (only merge-style `selectType`) | 9 dimensions × 1 helper = **9 missing** |
| **Replace Batch** | ✅ `selectPieces(ids)` | ❌ None | 9 dimensions × 1 helper = **9 missing** |
| **Add to Selection** | ✅ `addPieceToSelection(id)` | ⚠️ Only types/designs | 7 dimensions × 1 helper = **7 missing** |
| **Remove from Selection** | ✅ `removePieceFromSelection(id)` | ⚠️ Only types/designs | 7 dimensions × 1 helper = **7 missing** |
| **Toggle** | ❌ Design doesn't have | ❌ Kit doesn't have | 9 dimensions × 1 helper = **9 missing** (both) |
| **Clear Dimension** | ❌ Design doesn't have | ❌ Kit doesn't have | 9 dimensions × 1 helper = **9 missing** (both) |
| **Clear All** | ✅ `deselectAll()` | ✅ `clearSelection()` | None |
| **Select All** | ✅ `selectAll()` | ❌ None | **1 missing** |
| **Focus** | ✅ `focusPiece()` / `clearFocus()` | ❌ None | **2 missing** (not applicable for Kit) |

**Total Gaps for Kit:**
- **9** Replace Single helpers (1 per dimension)
- **9** Replace Batch helpers (1 per dimension)
- **7** Add helpers (qualities, ports, tags, concepts, files, folders, authors)
- **7** Remove helpers (qualities, ports, tags, concepts, files, folders, authors)
- **9** Toggle helpers (all dimensions - would be new)
- **9** Clear Dimension helpers (all dimensions - would be new)
- **1** Select All helper
- **3** Inverse diff dimensions (ports, tags, concepts)

**Grand Total: 54 missing helpers** (excluding focus which is Design-specific)

---

## 7. Selection Diff Types

### `KitAppSelectionDiff`

```typescript
interface KitAppSelectionDiff {
  types?: KitAppSelectionTypesDiff;
  designs?: KitAppSelectionDesignsDiff;
  qualities?: KitAppSelectionQualitiesDiff;
  ports?: KitAppSelectionPortsDiff;
  tags?: KitAppSelectionTagsDiff;
  concepts?: KitAppSelectionConceptsDiff;
  files?: KitAppSelectionFilesDiff;
  folders?: KitAppSelectionFoldersDiff;
  authors?: KitAppSelectionAuthorsDiff;
}

interface KitAppSelectionTypesDiff {
  added?: Guid[];
  removed?: Guid[];
}

// ... same pattern for all other dimensions
```

**Status:** ✅ Fully defined (all 9 dimensions)

**Note:** Despite all diff types being defined, only some are used by event handlers (types, designs).

---

## 8. Proposed Helper Strategy

To match Design's selection contract, Kit needs:

### Naming Convention

```typescript
// Replace-style (clears other items in same dimension)
useKitAppSelectSingleType(typeGuid)           // Replaces types, keeps other dimensions
useKitAppSelectTypes([typeGuid1, typeGuid2])  // Batch replace

// Merge-style (preserves all selections)
useKitAppAddTypeToSelection(typeGuid)         // Add (already exists as useKitAppSelectType)
useKitAppRemoveTypeFromSelection(typeGuid)    // Remove (already exists as useKitAppDeselectType)

// Toggle-style (new)
useKitAppToggleTypeInSelection(typeGuid)      // Add if missing, remove if present

// Clear-style (new)
useKitAppClearTypes()                         // Clears just types, keeps other dimensions
```

### Generic Utilities (Recommended)

```typescript
// In new file: js/semio/sketchpad/kitSelectionHelpers.ts

function addToSelection<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  value: KitAppSelection[K] extends (infer T)[] ? T : never
): KitAppSelection {
  const current = selection[key] || [];
  if (current.includes(value)) return selection;
  return { ...selection, [key]: [...current, value] };
}

function removeFromSelection<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  value: KitAppSelection[K] extends (infer T)[] ? T : never
): KitAppSelection {
  const current = selection[key] || [];
  return { ...selection, [key]: current.filter(v => v !== value) };
}

function toggleInSelection<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  value: KitAppSelection[K] extends (infer T)[] ? T : never
): KitAppSelection {
  const current = selection[key] || [];
  if (current.includes(value)) {
    return removeFromSelection(selection, key, value);
  } else {
    return addToSelection(selection, key, value);
  }
}

function clearSelectionDimension<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K
): KitAppSelection {
  const { [key]: _, ...rest } = selection;
  return rest;
}

function replaceSelectionDimension<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  values: KitAppSelection[K]
): KitAppSelection {
  return { ...selection, [key]: values };
}
```

---

## 9. Modifier Key Strategy (Recommended)

Based on Design's approach:

| Modifier | Behavior | Hook to Use |
|----------|----------|-------------|
| **None (Click)** | Replace dimension | `selectSingleType(id)` - clears other types, keeps other dimensions |
| **Ctrl/Cmd** | Toggle | `toggleTypeInSelection(id)` - add if missing, remove if present |
| **Shift** | Add to selection | `addTypeToSelection(id)` - add without removing |
| **Alt** | Remove from selection | `removeTypeFromSelection(id)` - explicit remove |

**Current Kit behavior (needs fixing):**
- Click uses `selectType` which **adds** (should replace)
- Ctrl+Click uses `deselectType` (should toggle)

---

## 10. Empty Selection Convention

### Current Behavior

Kit uses **undefined** for empty dimensions:

```typescript
// Empty
selection = {}

// With types selected
selection = { types: [guid1, guid2] }

// Types cleared
selection = { types: undefined } // or key deleted
```

**Recommendation:** Follow Design's pattern:
- Undefined fields = no selection for that dimension
- Delete keys entirely when cleared (don't keep empty arrays)

---

## 11. Commands vs Hooks

### Current Mix

Kit has a mixed approach:
- **Event handlers:** For `SELECT_TYPE`, `DESELECT_TYPE`, `SELECT_DESIGN`, `DESELECT_DESIGN`
- **Direct hooks:** For `SET_SELECTION`, `CLEAR_SELECTION`
- **No commands:** Unlike Design's command-based approach

Design uses:
- **Commands:** All selection operations go through command registry
  - `semio.designApp.selectPiece`
  - `semio.designApp.addPieceToSelection`
  - etc.

**Gap:** Kit doesn't have a command layer for selection operations (only event handlers).

---

## 12. Recommendations

### Phase 1: Immediate (Core Helpers)

1. **Rename existing hooks** for clarity:
   - `useKitAppSelectType` → `useKitAppAddTypeToSelection`
   - `useKitAppDeselectType` → `useKitAppRemoveTypeFromSelection`
   - (Keep old names as deprecated aliases)

2. **Add replace-style hooks** for types/designs:
   - `useKitAppSelectSingleType()` - replaces types dimension
   - `useKitAppSelectTypes()` - batch replace types
   - Same for designs

3. **Add toggle hooks** for types/designs:
   - `useKitAppToggleTypeInSelection()`
   - Same for designs

### Phase 2: Extended (All Dimensions)

4. **Create generic helpers** in `kitSelectionHelpers.ts`:
   - `addToSelection()`, `removeFromSelection()`, `toggleInSelection()`, etc.

5. **Generate hooks for all 7 missing dimensions**:
   - qualities, ports, tags, concepts, files, folders, authors
   - Use generic helpers internally

6. **Add `useKitAppSelectAll()`**:
   - Intelligent selection based on current filters
   - Select all visible artifacts

### Phase 3: Parity (Match Design)

7. **Fix inverse diff** to include ports, tags, concepts

8. **Document modifier key strategy** in UI components

9. **Write tests** for all selection helpers

---

## File References

- **Selection types:** Lines 218-277 in Kit.tsx
- **Inverse diff:** Lines 353-416 in Kit.tsx
- **Event handlers:** Lines 937-1004 in Kit.tsx
- **Selection hooks:** Lines 1342-1525 in Kit.tsx
- **Store implementation:** Lines 421-892 in Kit.tsx
