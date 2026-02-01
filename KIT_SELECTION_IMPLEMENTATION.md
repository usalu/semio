# Kit Selection Implementation Summary

**Implementation Date:** February 1, 2026  
**Prompt:** Prompt D - Implement Helper Hooks and Wire into UI

---

## 📦 Files Created

### 1. `/workspaces/semio/js/semio/sketchpad/kitSelectionHelpers.ts`

Generic utility functions for Kit selection operations.

**Exports:**
- `SelectionValue<K>` - Type helper for extracting dimension value types
- `addToSelection()` - Add item to dimension (no-op if duplicate)
- `removeFromSelection()` - Remove item from dimension (deletes key if empty)
- `toggleInSelection()` - Toggle item (add if missing, remove if present)
- `replaceSelectionDimension()` - Replace entire dimension
- `clearSelectionDimension()` - Clear single dimension
- `clearSelection()` - Clear all dimensions
- `selectAllInDimension()` - Select all items in dimension
- `isSelected()` - Check if item is selected

**Key Features:**
- ✅ Type-safe generics using `SelectionValue<K>` helper
- ✅ Duplicate detection (returns same reference if no change)
- ✅ Empty key deletion (no empty arrays stored)
- ✅ Dimension independence (operations never affect other dimensions)

---

### 2. `/workspaces/semio/js/semio/sketchpad/Kit.tsx` (Modified)

Added 54 new selection hooks plus 1 global hook.

**New Factory Function:**
- `createDimensionSelectionHooks<K>()` - Generic factory that creates all 6 operations for a dimension

**Hook Pattern (per dimension):**
```typescript
useKitAppAdd{Dimension}ToSelection()          // Merge: add without clearing others
useKitAppRemove{Dimension}FromSelection()     // Merge: remove from selection
useKitAppToggle{Dimension}InSelection()       // Merge: toggle (add/remove)
useKitAppSelectSingle{Dimension}()            // Replace: clear dimension, select one
useKitAppSelect{Dimension}()                  // Replace: select multiple
useKitAppClear{Dimension}()                   // Clear: remove dimension
```

**All 9 Dimensions:**
1. **Types** (Guid) - 6 hooks
2. **Designs** (Guid) - 6 hooks
3. **Qualities** (string) - 6 hooks
4. **Ports** (Guid) - 6 hooks
5. **Tags** (Guid) - 6 hooks
6. **Concepts** (Guid) - 6 hooks
7. **Files** (string) - 6 hooks
8. **Folders** (Guid) - 6 hooks
9. **Authors** (string) - 6 hooks

**Total:** 54 hooks (9 dimensions × 6 operations)

**Global Hook:**
- `useKitAppSelectAll()` - Select all artifacts in all dimensions

**Import Added:**
```typescript
import {
  addToSelection,
  removeFromSelection,
  toggleInSelection,
  replaceSelectionDimension,
  clearSelectionDimension,
  type SelectionValue,
} from "./kitSelectionHelpers";
```

---

### 3. `/workspaces/semio/js/semio/sketchpad/KitSelectionExample.tsx`

Comprehensive example component demonstrating all usage patterns.

**Examples Included:**

1. **`KitTypeTableExample`** - Basic table with modifier key detection
   - Click: Replace selection
   - Ctrl/Cmd + Click: Toggle
   - Shift + Click: Add
   - Alt + Click: Remove
   - Background click: Clear all

2. **`KitTableWithKeyboardShortcuts`** - Keyboard integration
   - Escape: Clear selection
   - Ctrl/Cmd + A: Select all

3. **`KitMultiDimensionExample`** - Independent dimension selection
   - Shows types and designs selected independently

4. **`KitDiagramExample`** - Diagram node selection
   - Node click with modifier key support

5. **Usage Summary** - Complete documentation in comments

---

## 🎯 Implementation Details

### Hook Architecture

All hooks follow the `ActionHookResult<TArgs>` pattern:

```typescript
export type ActionHookResult<TArgs extends any[]> = 
  readonly [action: ((...args: TArgs) => void) | undefined, canAct: boolean];
```

**Pattern:**
```typescript
const [action, canAct] = useKitAppAddTypeToSelection();

// Check if action is available
if (canAct && action) {
  action(typeGuid);
}

// Or use optional chaining
action?.(typeGuid);
```

### Factory Function Benefits

The `createDimensionSelectionHooks()` factory eliminates code duplication:

```typescript
// Instead of writing 54 individual hooks, we have:
function createDimensionSelectionHooks<K extends keyof KitAppSelection>(dimensionKey: K) {
  // Returns { useAdd, useRemove, useToggle, useSelectSingle, useSelect, useClear }
}

// Each dimension hook is a simple call:
export function useKitAppAddTypeToSelection(): ActionHookResult<[typeGuid: Guid]> {
  return createDimensionSelectionHooks("types").useAdd();
}
```

### Underlying Primitive

All hooks use the existing `useKitAppSelection()` hook:

```typescript
const [selection, setSelection] = useKitAppSelection();
// selection: Current KitAppSelection object
// setSelection: Function to update (undefined if canSet is false)
```

### Permission Gating

All operations respect the `canSet` gate from `useKitAppSelection()`:

```typescript
const [selection, setSelection] = useKitAppSelection();
const canAct = setSelection !== undefined;  // Only true if actor.can(KIT.SET_SELECTION)
```

---

## 🔧 Modifier Key Pattern

Standard UX pattern for selection interactions:

| Modifier       | Action                  | Hook                                      |
|----------------|-------------------------|-------------------------------------------|
| None           | Replace selection       | `useKitAppSelectSingle{Dimension}()`      |
| Ctrl/Cmd       | Toggle                  | `useKitAppToggle{Dimension}InSelection()` |
| Shift          | Add                     | `useKitAppAdd{Dimension}ToSelection()`    |
| Alt            | Remove                  | `useKitAppRemove{Dimension}FromSelection()`|
| Background     | Clear all               | `useKitAppClearSelection()`               |
| Escape         | Clear all               | `useKitAppClearSelection()`               |
| Ctrl/Cmd + A   | Select all              | `useKitAppSelectAll()`                    |

**Implementation:**

```typescript
const handleRowClick = (id: Guid, event: React.MouseEvent) => {
  const isCtrlOrCmd = event.ctrlKey || event.metaKey;
  const isShift = event.shiftKey;
  const isAlt = event.altKey;

  if (isCtrlOrCmd) {
    toggleTypeInSelection?.(id);
  } else if (isShift) {
    addTypeToSelection?.(id);
  } else if (isAlt) {
    removeTypeFromSelection?.(id);
  } else {
    selectSingleType?.(id);
  }
};
```

---

## 📊 Empty Selection Convention

**Rule:** Empty dimension arrays are **deleted** from the selection object.

```typescript
// ✅ CORRECT
selection = { types: ["guid1"], designs: ["guid2"] }

// After clearing types
selection = { designs: ["guid2"] }  // "types" key deleted

// ❌ WRONG
selection = { types: [], designs: ["guid2"] }  // Don't keep empty arrays
```

**Rationale:**
1. Cleaner serialization: `{}` vs `{types: [], designs: [], ...}` (9 empty arrays)
2. Simpler conditionals: `if (selection.types)` vs `if (selection.types?.length)`
3. Smaller payload for Y.js sync and localStorage
4. Consistent with "no selection" semantic

**Implementation:**

```typescript
export function removeFromSelection<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  value: SelectionValue<K>
): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  const newArray = currentArray.filter((v) => v !== value);
  
  // Delete key if empty
  if (newArray.length === 0) {
    const { [key]: _, ...rest } = selection;
    return rest;
  }
  
  return { ...selection, [key]: newArray };
}
```

---

## ✅ Implementation Checklist

- [x] Create `kitSelectionHelpers.ts` with generic utilities
- [x] Add `createDimensionSelectionHooks` factory to Kit.tsx
- [x] Generate 54 hooks (9 dimensions × 6 operations)
- [x] Add `useKitAppSelectAll()` hook
- [x] Add helper imports to Kit.tsx
- [x] Create comprehensive example component
- [x] Document modifier key patterns
- [x] Document empty selection convention
- [x] Show keyboard shortcut integration
- [x] Show multi-dimension independence

---

## 🚀 Next Steps (Prompt E)

1. **Unit Tests** - Test generic helper functions
   - `addToSelection` duplicate detection
   - `removeFromSelection` empty key deletion
   - `toggleInSelection` add/remove logic
   - Multi-dimension independence

2. **Integration Tests** - Test hooks with XState
   - Hook permission gating
   - Modifier key interactions
   - Keyboard shortcuts

3. **Parity Check** - Compare with Design.tsx
   - Verify all selection patterns supported
   - Document behavioral differences

4. **UI Wiring** - Update existing components
   - Replace old `useKitAppSelectType()` / `useKitAppDeselectType()` calls
   - Add modifier key handlers to tables/diagrams
   - Test with real UI interactions

---

## 📝 Notes

### Backward Compatibility

Old hooks (`useKitAppSelectType`, `useKitAppDeselectType`) still exist and work. They implement merge-style "add" behavior, not replace. To avoid confusion, prefer the new explicit hooks:

```typescript
// OLD (confusing name - actually adds, doesn't replace)
const [selectType] = useKitAppSelectType();

// NEW (explicit)
const [addType] = useKitAppAddTypeToSelection();
const [selectSingleType] = useKitAppSelectSingleType();
```

### Type Safety

All helpers are fully type-safe using TypeScript generics:

```typescript
type SelectionValue<K extends keyof KitAppSelection> = 
  KitAppSelection[K] extends (infer T)[] ? T : never;

// Examples:
// SelectionValue<"types"> => Guid
// SelectionValue<"qualities"> => string
```

TypeScript prevents:
- ❌ Adding a `string` to `types` (expects `Guid`)
- ❌ Adding a `Guid` to `qualities` (expects `string`)
- ❌ Using invalid dimension keys

### Dimension Independence

Selecting in one dimension **never** affects other dimensions:

```typescript
// Start empty
selection = {}

// Select type
selectSingleType("type-1")
selection = { types: ["type-1"] }

// Select design (types kept!)
selectSingleDesign("design-1")
selection = { types: ["type-1"], designs: ["design-1"] }

// Clear types (designs kept!)
clearTypes()
selection = { designs: ["design-1"] }
```

This is the **key difference** from Design.tsx, where pieces/connections/connectors are mutually exclusive.

---

## 🎓 Usage Examples

### Simple Click Handler

```typescript
const [selectType] = useKitAppSelectSingleType();

<TableRow onClick={() => selectType?.(typeGuid)}>
  ...
</TableRow>
```

### With Modifier Keys

```typescript
const [selectSingleType] = useKitAppSelectSingleType();
const [toggleType] = useKitAppToggleTypeInSelection();
const [addType] = useKitAppAddTypeToSelection();
const [removeType] = useKitAppRemoveTypeFromSelection();

<TableRow onClick={(e) => {
  if (e.ctrlKey || e.metaKey) toggleType?.(typeGuid);
  else if (e.shiftKey) addType?.(typeGuid);
  else if (e.altKey) removeType?.(typeGuid);
  else selectSingleType?.(typeGuid);
}}>
  ...
</TableRow>
```

### Batch Selection

```typescript
const [selectTypes] = useKitAppSelectTypes();

const handleSelectAll = () => {
  const allGuids = types.map(t => t.guid);
  selectTypes?.(allGuids);
};
```

### Clear on Background Click

```typescript
const [clearSelection] = useKitAppClearSelection();

<div onClick={(e) => {
  if (e.currentTarget === e.target) {
    clearSelection?.();
  }
}}>
  {/* Table content */}
</div>
```

---

## 🔗 Related Files

- `DESIGN_SELECTION_CONTRACT.md` - Design app selection specification
- `KIT_SELECTION_GAP_ANALYSIS.md` - Gap analysis vs Design
- `KIT_SELECTION_HELPERS_DESIGN.md` - Design document (Prompt C output)
- `PROMPTS_KIT_SELECTION_MIGRATION.md` - 5-prompt migration strategy

---

## 🏁 Completion Status

**Prompt D: COMPLETE** ✅

All tasks from Prompt D implemented:
1. ✅ Created `kitSelectionHelpers.ts` with generic utilities
2. ✅ Added 54 hooks to Kit.tsx (9 dimensions × 6 operations)
3. ✅ Demonstrated UI click handler wiring in example file
4. ✅ Ensured hooks use `useKitScope()` correctly
5. ✅ Provided ready-to-use TypeScript code

**Next:** Proceed with Prompt E (tests and parity checks)
