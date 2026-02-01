# Kit Selection Helper Layer Design

**Based on:** Design Selection Contract + Kit Selection Gap Analysis

A comprehensive design for Kit's merge-style selection helper layer that preserves dimension independence.

---

## Design Principles

1. **Dimension Independence:** Selecting a type does NOT clear designs, ports, tags, etc.
2. **Merge-Style by Default:** Operations preserve existing selections in unrelated dimensions
3. **Explicit Clear:** Clearing requires explicit action (no implicit clearing)
4. **Hook-Based:** All helpers are React hooks following the `ActionHookResult<TArgs>` pattern
5. **Underlying Primitive:** All helpers use `useKitAppSelection()` and `setSelection()` under the hood

---

## 1. Generic Utility Functions

### File: `js/semio/sketchpad/kitSelectionHelpers.ts`

```typescript
// js/semio/sketchpad/kitSelectionHelpers.ts

import { Guid } from "../semio";
import type { KitAppSelection } from "./Kit";

/**
 * Helper type to extract array element type from a selection dimension
 */
type SelectionValue<K extends keyof KitAppSelection> = 
  KitAppSelection[K] extends (infer T)[] ? T : never;

/**
 * Adds a value to a selection dimension without clearing other dimensions.
 * 
 * @param selection - Current selection object
 * @param key - Dimension key (e.g., "types", "designs")
 * @param value - Value to add (e.g., guid)
 * @returns New selection object with value added
 * 
 * @example
 * const newSelection = addToSelection(
 *   { types: ["guid1"] },
 *   "types",
 *   "guid2"
 * );
 * // Result: { types: ["guid1", "guid2"] }
 */
export function addToSelection<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  value: SelectionValue<K>
): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  
  // No-op if already selected (duplicate check)
  if (currentArray.includes(value)) {
    return selection;
  }
  
  return {
    ...selection,
    [key]: [...currentArray, value],
  };
}

/**
 * Removes a value from a selection dimension without affecting other dimensions.
 * 
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param value - Value to remove
 * @returns New selection object with value removed
 * 
 * @example
 * const newSelection = removeFromSelection(
 *   { types: ["guid1", "guid2"], designs: ["guid3"] },
 *   "types",
 *   "guid2"
 * );
 * // Result: { types: ["guid1"], designs: ["guid3"] }
 */
export function removeFromSelection<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  value: SelectionValue<K>
): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  const newArray = currentArray.filter((v) => v !== value);
  
  // Delete key if array becomes empty (convention: no empty arrays)
  if (newArray.length === 0) {
    const { [key]: _, ...rest } = selection;
    return rest;
  }
  
  return {
    ...selection,
    [key]: newArray,
  };
}

/**
 * Toggles a value in a selection dimension (add if missing, remove if present).
 * 
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param value - Value to toggle
 * @returns New selection object with value toggled
 * 
 * @example
 * toggleInSelection({ types: ["guid1"] }, "types", "guid2") 
 * // => { types: ["guid1", "guid2"] }
 * 
 * toggleInSelection({ types: ["guid1", "guid2"] }, "types", "guid2")
 * // => { types: ["guid1"] }
 */
export function toggleInSelection<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  value: SelectionValue<K>
): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  
  if (currentArray.includes(value)) {
    return removeFromSelection(selection, key, value);
  } else {
    return addToSelection(selection, key, value);
  }
}

/**
 * Replaces an entire selection dimension without affecting other dimensions.
 * 
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param values - New values for the dimension (undefined to clear)
 * @returns New selection object with dimension replaced
 * 
 * @example
 * replaceSelectionDimension(
 *   { types: ["guid1"], designs: ["guid2"] },
 *   "types",
 *   ["guid3", "guid4"]
 * );
 * // Result: { types: ["guid3", "guid4"], designs: ["guid2"] }
 */
export function replaceSelectionDimension<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  values: KitAppSelection[K] | undefined
): KitAppSelection {
  // If values is undefined or empty array, delete the key
  if (!values || (Array.isArray(values) && values.length === 0)) {
    const { [key]: _, ...rest } = selection;
    return rest;
  }
  
  return {
    ...selection,
    [key]: values,
  };
}

/**
 * Clears a single selection dimension without affecting others.
 * 
 * @param selection - Current selection object
 * @param key - Dimension key to clear
 * @returns New selection object with dimension cleared
 * 
 * @example
 * clearSelectionDimension({ types: ["guid1"], designs: ["guid2"] }, "types")
 * // Result: { designs: ["guid2"] }
 */
export function clearSelectionDimension<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K
): KitAppSelection {
  const { [key]: _, ...rest } = selection;
  return rest;
}

/**
 * Clears all selection dimensions.
 * 
 * @returns Empty selection object
 * 
 * @example
 * clearSelection()
 * // Result: {}
 */
export function clearSelection(): KitAppSelection {
  return {};
}

/**
 * Selects all items in a dimension (replaces existing selection for that dimension).
 * 
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param allValues - All available values for the dimension
 * @returns New selection object with all values selected
 * 
 * @example
 * selectAllInDimension({ types: ["guid1"] }, "types", ["guid1", "guid2", "guid3"])
 * // Result: { types: ["guid1", "guid2", "guid3"] }
 */
export function selectAllInDimension<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  allValues: SelectionValue<K>[]
): KitAppSelection {
  return replaceSelectionDimension(selection, key, allValues as KitAppSelection[K]);
}

/**
 * Checks if a value is selected in a dimension.
 * 
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param value - Value to check
 * @returns True if value is selected
 */
export function isSelected<K extends keyof KitAppSelection>(
  selection: KitAppSelection,
  key: K,
  value: SelectionValue<K>
): boolean {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  return currentArray.includes(value);
}
```

---

## 2. Hook Wrappers (All 9 Dimensions)

### Pattern for Each Dimension

Each dimension gets **4 standard hooks**:

```typescript
// Add to selection (merge)
useKitApp{Add}{Dimension}ToSelection(): ActionHookResult<[id: string | Guid]>

// Remove from selection (merge)
useKitApp{Remove}{Dimension}FromSelection(): ActionHookResult<[id: string | Guid]>

// Toggle in selection (merge)
useKitApp{Toggle}{Dimension}InSelection(): ActionHookResult<[id: string | Guid]>

// Replace dimension (clear others in same dimension, keep other dimensions)
useKitApp{SelectSingle}{Dimension}(): ActionHookResult<[id: string | Guid]>

// Replace dimension with multiple (batch)
useKitApp{Select}{Dimension}(): ActionHookResult<[ids: (string | Guid)[]]>

// Clear dimension
useKitApp{Clear}{Dimension}(): ActionHookResult<[]>
```

### File: `js/semio/sketchpad/Kit.tsx` (in Hooks region)

```typescript
// #region Selection Helper Hooks

import {
  addToSelection,
  removeFromSelection,
  toggleInSelection,
  replaceSelectionDimension,
  clearSelectionDimension,
} from "./kitSelectionHelpers";

/**
 * Hook factory for dimension-specific selection operations.
 * Creates add/remove/toggle/replace hooks for a specific selection dimension.
 */
function createDimensionSelectionHooks<K extends keyof KitAppSelection>(
  dimensionKey: K
) {
  /**
   * Adds a single item to the dimension without clearing others.
   */
  function useAdd(): ActionHookResult<[value: SelectionValue<K>]> {
    const [selection, setSelection, canSet] = useKitAppSelection();
    const action = useMemo(() => {
      if (!canSet || !setSelection) return undefined;
      return (value: SelectionValue<K>) => {
        const newSelection = addToSelection(selection || {}, dimensionKey, value);
        setSelection(newSelection);
      };
    }, [selection, setSelection, canSet]);
    return [action, canSet];
  }

  /**
   * Removes a single item from the dimension.
   */
  function useRemove(): ActionHookResult<[value: SelectionValue<K>]> {
    const [selection, setSelection, canSet] = useKitAppSelection();
    const action = useMemo(() => {
      if (!canSet || !setSelection) return undefined;
      return (value: SelectionValue<K>) => {
        const newSelection = removeFromSelection(selection || {}, dimensionKey, value);
        setSelection(newSelection);
      };
    }, [selection, setSelection, canSet]);
    return [action, canSet];
  }

  /**
   * Toggles a single item in the dimension (add if missing, remove if present).
   */
  function useToggle(): ActionHookResult<[value: SelectionValue<K>]> {
    const [selection, setSelection, canSet] = useKitAppSelection();
    const action = useMemo(() => {
      if (!canSet || !setSelection) return undefined;
      return (value: SelectionValue<K>) => {
        const newSelection = toggleInSelection(selection || {}, dimensionKey, value);
        setSelection(newSelection);
      };
    }, [selection, setSelection, canSet]);
    return [action, canSet];
  }

  /**
   * Replaces the dimension with a single item (clears others in dimension).
   */
  function useSelectSingle(): ActionHookResult<[value: SelectionValue<K>]> {
    const [selection, setSelection, canSet] = useKitAppSelection();
    const action = useMemo(() => {
      if (!canSet || !setSelection) return undefined;
      return (value: SelectionValue<K>) => {
        const newSelection = replaceSelectionDimension(
          selection || {},
          dimensionKey,
          [value] as KitAppSelection[K]
        );
        setSelection(newSelection);
      };
    }, [selection, setSelection, canSet]);
    return [action, canSet];
  }

  /**
   * Replaces the dimension with multiple items (batch replace).
   */
  function useSelect(): ActionHookResult<[values: SelectionValue<K>[]]> {
    const [selection, setSelection, canSet] = useKitAppSelection();
    const action = useMemo(() => {
      if (!canSet || !setSelection) return undefined;
      return (values: SelectionValue<K>[]) => {
        const newSelection = replaceSelectionDimension(
          selection || {},
          dimensionKey,
          values as KitAppSelection[K]
        );
        setSelection(newSelection);
      };
    }, [selection, setSelection, canSet]);
    return [action, canSet];
  }

  /**
   * Clears the dimension (keeps other dimensions).
   */
  function useClear(): ActionHookResult<[]> {
    const [selection, setSelection, canSet] = useKitAppSelection();
    const action = useMemo(() => {
      if (!canSet || !setSelection) return undefined;
      return () => {
        const newSelection = clearSelectionDimension(selection || {}, dimensionKey);
        setSelection(newSelection);
      };
    }, [selection, setSelection, canSet]);
    return [action, canSet];
  }

  return { useAdd, useRemove, useToggle, useSelectSingle, useSelect, useClear };
}

// #region Types Selection Hooks

export function useKitAppAddTypeToSelection(): ActionHookResult<[typeGuid: Guid]> {
  return createDimensionSelectionHooks("types").useAdd();
}

export function useKitAppRemoveTypeFromSelection(): ActionHookResult<[typeGuid: Guid]> {
  return createDimensionSelectionHooks("types").useRemove();
}

export function useKitAppToggleTypeInSelection(): ActionHookResult<[typeGuid: Guid]> {
  return createDimensionSelectionHooks("types").useToggle();
}

export function useKitAppSelectSingleType(): ActionHookResult<[typeGuid: Guid]> {
  return createDimensionSelectionHooks("types").useSelectSingle();
}

export function useKitAppSelectTypes(): ActionHookResult<[typeGuids: Guid[]]> {
  return createDimensionSelectionHooks("types").useSelect();
}

export function useKitAppClearTypes(): ActionHookResult<[]> {
  return createDimensionSelectionHooks("types").useClear();
}

// #endregion Types Selection Hooks

// #region Designs Selection Hooks

export function useKitAppAddDesignToSelection(): ActionHookResult<[designGuid: Guid]> {
  return createDimensionSelectionHooks("designs").useAdd();
}

export function useKitAppRemoveDesignFromSelection(): ActionHookResult<[designGuid: Guid]> {
  return createDimensionSelectionHooks("designs").useRemove();
}

export function useKitAppToggleDesignInSelection(): ActionHookResult<[designGuid: Guid]> {
  return createDimensionSelectionHooks("designs").useToggle();
}

export function useKitAppSelectSingleDesign(): ActionHookResult<[designGuid: Guid]> {
  return createDimensionSelectionHooks("designs").useSelectSingle();
}

export function useKitAppSelectDesigns(): ActionHookResult<[designGuids: Guid[]]> {
  return createDimensionSelectionHooks("designs").useSelect();
}

export function useKitAppClearDesigns(): ActionHookResult<[]> {
  return createDimensionSelectionHooks("designs").useClear();
}

// #endregion Designs Selection Hooks

// #region Qualities Selection Hooks

export function useKitAppAddQualityToSelection(): ActionHookResult<[qualityName: string]> {
  return createDimensionSelectionHooks("qualities").useAdd();
}

export function useKitAppRemoveQualityFromSelection(): ActionHookResult<[qualityName: string]> {
  return createDimensionSelectionHooks("qualities").useRemove();
}

export function useKitAppToggleQualityInSelection(): ActionHookResult<[qualityName: string]> {
  return createDimensionSelectionHooks("qualities").useToggle();
}

export function useKitAppSelectSingleQuality(): ActionHookResult<[qualityName: string]> {
  return createDimensionSelectionHooks("qualities").useSelectSingle();
}

export function useKitAppSelectQualities(): ActionHookResult<[qualityNames: string[]]> {
  return createDimensionSelectionHooks("qualities").useSelect();
}

export function useKitAppClearQualities(): ActionHookResult<[]> {
  return createDimensionSelectionHooks("qualities").useClear();
}

// #endregion Qualities Selection Hooks

// #region Ports Selection Hooks

export function useKitAppAddPortToSelection(): ActionHookResult<[portGuid: Guid]> {
  return createDimensionSelectionHooks("ports").useAdd();
}

export function useKitAppRemovePortFromSelection(): ActionHookResult<[portGuid: Guid]> {
  return createDimensionSelectionHooks("ports").useRemove();
}

export function useKitAppTogglePortInSelection(): ActionHookResult<[portGuid: Guid]> {
  return createDimensionSelectionHooks("ports").useToggle();
}

export function useKitAppSelectSinglePort(): ActionHookResult<[portGuid: Guid]> {
  return createDimensionSelectionHooks("ports").useSelectSingle();
}

export function useKitAppSelectPorts(): ActionHookResult<[portGuids: Guid[]]> {
  return createDimensionSelectionHooks("ports").useSelect();
}

export function useKitAppClearPorts(): ActionHookResult<[]> {
  return createDimensionSelectionHooks("ports").useClear();
}

// #endregion Ports Selection Hooks

// #region Tags Selection Hooks

export function useKitAppAddTagToSelection(): ActionHookResult<[tagGuid: Guid]> {
  return createDimensionSelectionHooks("tags").useAdd();
}

export function useKitAppRemoveTagFromSelection(): ActionHookResult<[tagGuid: Guid]> {
  return createDimensionSelectionHooks("tags").useRemove();
}

export function useKitAppToggleTagInSelection(): ActionHookResult<[tagGuid: Guid]> {
  return createDimensionSelectionHooks("tags").useToggle();
}

export function useKitAppSelectSingleTag(): ActionHookResult<[tagGuid: Guid]> {
  return createDimensionSelectionHooks("tags").useSelectSingle();
}

export function useKitAppSelectTags(): ActionHookResult<[tagGuids: Guid[]]> {
  return createDimensionSelectionHooks("tags").useSelect();
}

export function useKitAppClearTags(): ActionHookResult<[]> {
  return createDimensionSelectionHooks("tags").useClear();
}

// #endregion Tags Selection Hooks

// #region Concepts Selection Hooks

export function useKitAppAddConceptToSelection(): ActionHookResult<[conceptGuid: Guid]> {
  return createDimensionSelectionHooks("concepts").useAdd();
}

export function useKitAppRemoveConceptFromSelection(): ActionHookResult<[conceptGuid: Guid]> {
  return createDimensionSelectionHooks("concepts").useRemove();
}

export function useKitAppToggleConceptInSelection(): ActionHookResult<[conceptGuid: Guid]> {
  return createDimensionSelectionHooks("concepts").useToggle();
}

export function useKitAppSelectSingleConcept(): ActionHookResult<[conceptGuid: Guid]> {
  return createDimensionSelectionHooks("concepts").useSelectSingle();
}

export function useKitAppSelectConcepts(): ActionHookResult<[conceptGuids: Guid[]]> {
  return createDimensionSelectionHooks("concepts").useSelect();
}

export function useKitAppClearConcepts(): ActionHookResult<[]> {
  return createDimensionSelectionHooks("concepts").useClear();
}

// #endregion Concepts Selection Hooks

// #region Files Selection Hooks

export function useKitAppAddFileToSelection(): ActionHookResult<[filePath: string]> {
  return createDimensionSelectionHooks("files").useAdd();
}

export function useKitAppRemoveFileFromSelection(): ActionHookResult<[filePath: string]> {
  return createDimensionSelectionHooks("files").useRemove();
}

export function useKitAppToggleFileInSelection(): ActionHookResult<[filePath: string]> {
  return createDimensionSelectionHooks("files").useToggle();
}

export function useKitAppSelectSingleFile(): ActionHookResult<[filePath: string]> {
  return createDimensionSelectionHooks("files").useSelectSingle();
}

export function useKitAppSelectFiles(): ActionHookResult<[filePaths: string[]]> {
  return createDimensionSelectionHooks("files").useSelect();
}

export function useKitAppClearFiles(): ActionHookResult<[]> {
  return createDimensionSelectionHooks("files").useClear();
}

// #endregion Files Selection Hooks

// #region Folders Selection Hooks

export function useKitAppAddFolderToSelection(): ActionHookResult<[folderGuid: Guid]> {
  return createDimensionSelectionHooks("folders").useAdd();
}

export function useKitAppRemoveFolderFromSelection(): ActionHookResult<[folderGuid: Guid]> {
  return createDimensionSelectionHooks("folders").useRemove();
}

export function useKitAppToggleFolderInSelection(): ActionHookResult<[folderGuid: Guid]> {
  return createDimensionSelectionHooks("folders").useToggle();
}

export function useKitAppSelectSingleFolder(): ActionHookResult<[folderGuid: Guid]> {
  return createDimensionSelectionHooks("folders").useSelectSingle();
}

export function useKitAppSelectFolders(): ActionHookResult<[folderGuids: Guid[]]> {
  return createDimensionSelectionHooks("folders").useSelect();
}

export function useKitAppClearFolders(): ActionHookResult<[]> {
  return createDimensionSelectionHooks("folders").useClear();
}

// #endregion Folders Selection Hooks

// #region Authors Selection Hooks

export function useKitAppAddAuthorToSelection(): ActionHookResult<[authorName: string]> {
  return createDimensionSelectionHooks("authors").useAdd();
}

export function useKitAppRemoveAuthorFromSelection(): ActionHookResult<[authorName: string]> {
  return createDimensionSelectionHooks("authors").useRemove();
}

export function useKitAppToggleAuthorInSelection(): ActionHookResult<[authorName: string]> {
  return createDimensionSelectionHooks("authors").useToggle();
}

export function useKitAppSelectSingleAuthor(): ActionHookResult<[authorName: string]> {
  return createDimensionSelectionHooks("authors").useSelectSingle();
}

export function useKitAppSelectAuthors(): ActionHookResult<[authorNames: string[]]> {
  return createDimensionSelectionHooks("authors").useSelect();
}

export function useKitAppClearAuthors(): ActionHookResult<[]> {
  return createDimensionSelectionHooks("authors").useClear();
}

// #endregion Authors Selection Hooks

// #region Global Selection Hooks

/**
 * Selects all artifacts in all dimensions (respects current filter).
 */
export function useKitAppSelectAll(): ActionHookResult<[]> {
  const kit = useKit();
  const [, setSelection, canSet] = useKitAppSelection();
  const action = useMemo(() => {
    if (!canSet || !setSelection || !kit) return undefined;
    return () => {
      const allSelection: KitAppSelection = {
        types: kit.types?.map(t => t.guid) || [],
        designs: kit.designs?.map(d => d.guid) || [],
        qualities: kit.qualities?.map(q => q.name) || [],
        ports: kit.ports?.map(p => p.guid) || [],
        tags: kit.tags?.map(t => t.guid) || [],
        concepts: kit.concepts?.map(c => c.guid) || [],
        files: kit.files?.map(f => f.path) || [],
        folders: kit.folders?.map(f => f.guid) || [],
        authors: kit.authors?.map(a => a.name) || [],
      };
      setSelection(allSelection);
    };
  }, [kit, setSelection, canSet]);
  return [action, canSet];
}

// #endregion Global Selection Hooks

// #endregion Selection Helper Hooks
```

---

## 3. Modifier Key Strategy

### Standard Pattern

```typescript
interface ClickHandlerParams {
  id: string | Guid;
  event: React.MouseEvent;
}

function handleArtifactClick({ id, event }: ClickHandlerParams) {
  const isCtrlOrCmd = event.ctrlKey || event.metaKey;
  const isShift = event.shiftKey;
  const isAlt = event.altKey;

  // Priority: Ctrl/Cmd > Shift > Alt > None
  
  if (isCtrlOrCmd) {
    // TOGGLE: Add if missing, remove if present
    toggleTypeInSelection?.(id);
  } else if (isShift) {
    // ADD: Add to selection without removing others
    addTypeToSelection?.(id);
  } else if (isAlt) {
    // REMOVE: Remove from selection
    removeTypeFromSelection?.(id);
  } else {
    // REPLACE: Clear others in dimension, select this one
    selectSingleType?.(id);
  }
}
```

### Hook Usage in Components

```typescript
// In Kit app table component
export function KitArtifactTable() {
  // Get all 4 operations for types dimension
  const [selectSingleType] = useKitAppSelectSingleType();
  const [addTypeToSelection] = useKitAppAddTypeToSelection();
  const [removeTypeFromSelection] = useKitAppRemoveTypeFromSelection();
  const [toggleTypeInSelection] = useKitAppToggleTypeInSelection();
  const [clearSelection] = useKitAppClearSelection();

  const handleTypeRowClick = (typeGuid: Guid, event: React.MouseEvent) => {
    const isCtrlOrCmd = event.ctrlKey || event.metaKey;
    const isShift = event.shiftKey;
    const isAlt = event.altKey;

    if (isCtrlOrCmd) {
      toggleTypeInSelection?.(typeGuid);
    } else if (isShift) {
      addTypeToSelection?.(typeGuid);
    } else if (isAlt) {
      removeTypeFromSelection?.(typeGuid);
    } else {
      selectSingleType?.(typeGuid);
    }
  };

  const handleBackgroundClick = (event: React.MouseEvent) => {
    // Only clear if clicking background (not propagated from row)
    if (event.currentTarget === event.target) {
      clearSelection?.();
    }
  };

  return (
    <div onClick={handleBackgroundClick}>
      {types.map(type => (
        <TableRow
          key={type.guid}
          onClick={(e) => handleTypeRowClick(type.guid, e)}
        >
          {/* Row content */}
        </TableRow>
      ))}
    </div>
  );
}
```

### Keyboard Shortcuts

```typescript
// Escape key: Clear all selections
useHotkeys('escape', () => {
  clearSelection?.();
});

// Ctrl/Cmd+A: Select all
useHotkeys('mod+a', (e) => {
  e.preventDefault();
  selectAll?.();
});

// Delete/Backspace: Delete selected items
useHotkeys('delete,backspace', () => {
  deleteSelected?.();
});
```

---

## 4. Empty Selection Convention

### Proposed Convention: **Delete Empty Keys**

```typescript
// ✅ CORRECT: Delete keys with empty arrays
selection = { types: ["guid1"], designs: ["guid2"] }

// After clearing types
selection = { designs: ["guid2"] }  // "types" key deleted

// NOT this:
selection = { types: [], designs: ["guid2"] }  // ❌ Don't keep empty arrays
```

### Rationale

**Why delete empty keys:**

1. **Cleaner serialization:** `{}` vs `{types: [], designs: [], ...}` (9 empty arrays)
2. **Simpler conditionals:** `if (selection.types)` vs `if (selection.types?.length)`
3. **Smaller payload:** No redundant data in Y.js sync or localStorage
4. **Consistent with "no selection" semantic:** Missing key = dimension not selected
5. **Matches Design convention:** Design uses `undefined` for empty (not empty arrays)

**Implementation:**

```typescript
// In removeFromSelection helper:
const newArray = currentArray.filter((v) => v !== value);

if (newArray.length === 0) {
  // Delete the key instead of keeping empty array
  const { [key]: _, ...rest } = selection;
  return rest;
}

return { ...selection, [key]: newArray };
```

**Exception: Initial State**

The default state should be `{}`, not `{types: [], designs: [], ...}`:

```typescript
const EMPTY_SELECTION: KitAppSelection = {};

// NOT:
const EMPTY_SELECTION: KitAppSelection = {
  types: [],
  designs: [],
  // ... all empty
};
```

---

## 5. Hook Summary Table

### All 54 New Hooks

| Dimension | Add | Remove | Toggle | Select Single | Select Batch | Clear |
|-----------|-----|--------|--------|---------------|--------------|-------|
| **types** | `useKitAppAddTypeToSelection` | `useKitAppRemoveTypeFromSelection` | `useKitAppToggleTypeInSelection` | `useKitAppSelectSingleType` | `useKitAppSelectTypes` | `useKitAppClearTypes` |
| **designs** | `useKitAppAddDesignToSelection` | `useKitAppRemoveDesignFromSelection` | `useKitAppToggleDesignInSelection` | `useKitAppSelectSingleDesign` | `useKitAppSelectDesigns` | `useKitAppClearDesigns` |
| **qualities** | `useKitAppAddQualityToSelection` | `useKitAppRemoveQualityFromSelection` | `useKitAppToggleQualityInSelection` | `useKitAppSelectSingleQuality` | `useKitAppSelectQualities` | `useKitAppClearQualities` |
| **ports** | `useKitAppAddPortToSelection` | `useKitAppRemovePortFromSelection` | `useKitAppTogglePortInSelection` | `useKitAppSelectSinglePort` | `useKitAppSelectPorts` | `useKitAppClearPorts` |
| **tags** | `useKitAppAddTagToSelection` | `useKitAppRemoveTagFromSelection` | `useKitAppToggleTagInSelection` | `useKitAppSelectSingleTag` | `useKitAppSelectTags` | `useKitAppClearTags` |
| **concepts** | `useKitAppAddConceptToSelection` | `useKitAppRemoveConceptFromSelection` | `useKitAppToggleConceptInSelection` | `useKitAppSelectSingleConcept` | `useKitAppSelectConcepts` | `useKitAppClearConcepts` |
| **files** | `useKitAppAddFileToSelection` | `useKitAppRemoveFileFromSelection` | `useKitAppToggleFileInSelection` | `useKitAppSelectSingleFile` | `useKitAppSelectFiles` | `useKitAppClearFiles` |
| **folders** | `useKitAppAddFolderToSelection` | `useKitAppRemoveFolderFromSelection` | `useKitAppToggleFolderInSelection` | `useKitAppSelectSingleFolder` | `useKitAppSelectFolders` | `useKitAppClearFolders` |
| **authors** | `useKitAppAddAuthorToSelection` | `useKitAppRemoveAuthorFromSelection` | `useKitAppToggleAuthorInSelection` | `useKitAppSelectSingleAuthor` | `useKitAppSelectAuthors` | `useKitAppClearAuthors` |

**Total:** 54 hooks (9 dimensions × 6 operations)

**Plus Global:**
- `useKitAppSelectAll()` - Select all dimensions
- `useKitAppClearSelection()` - Clear all dimensions (already exists)

---

## 6. Migration Path from Existing Hooks

### Deprecation Strategy

```typescript
// OLD (current names - merge-style add)
export function useKitAppSelectType() { ... }
export function useKitAppDeselectType() { ... }

// NEW (explicit names)
export function useKitAppAddTypeToSelection() { ... }
export function useKitAppRemoveTypeFromSelection() { ... }

// DEPRECATED (keep for backward compatibility)
/**
 * @deprecated Use useKitAppAddTypeToSelection() instead
 */
export function useKitAppSelectType() {
  return useKitAppAddTypeToSelection();
}

/**
 * @deprecated Use useKitAppRemoveTypeFromSelection() instead
 */
export function useKitAppDeselectType() {
  return useKitAppRemoveTypeFromSelection();
}
```

### Event Handler Migration

Current event handlers (`KIT.SELECT_TYPE`, `KIT.DESELECT_TYPE`) can remain unchanged:

- They already implement merge-style behavior
- Hooks call `setSelection()` which triggers event handlers internally
- No breaking changes to event system

---

## 7. Type Safety

### Generic Helpers are Fully Typed

```typescript
type SelectionValue<K extends keyof KitAppSelection> = 
  KitAppSelection[K] extends (infer T)[] ? T : never;

// Examples:
// SelectionValue<"types"> => Guid
// SelectionValue<"qualities"> => string
// SelectionValue<"files"> => string
```

TypeScript ensures:
- ✅ Can't add a `string` to `types` (expects `Guid`)
- ✅ Can't add a `Guid` to `qualities` (expects `string`)
- ✅ Can't use invalid dimension keys

---

## 8. Testing Strategy

### Unit Tests (kitSelectionHelpers.test.ts)

```typescript
import { describe, it, expect } from 'vitest';
import {
  addToSelection,
  removeFromSelection,
  toggleInSelection,
  clearSelectionDimension,
  clearSelection,
} from './kitSelectionHelpers';

describe('addToSelection', () => {
  it('adds value to dimension', () => {
    const result = addToSelection({}, 'types', 'guid1');
    expect(result).toEqual({ types: ['guid1'] });
  });

  it('preserves other dimensions', () => {
    const result = addToSelection(
      { designs: ['guid2'] },
      'types',
      'guid1'
    );
    expect(result).toEqual({ types: ['guid1'], designs: ['guid2'] });
  });

  it('no-op if duplicate', () => {
    const input = { types: ['guid1'] };
    const result = addToSelection(input, 'types', 'guid1');
    expect(result).toBe(input); // Same reference
  });
});

describe('removeFromSelection', () => {
  it('removes value from dimension', () => {
    const result = removeFromSelection(
      { types: ['guid1', 'guid2'] },
      'types',
      'guid1'
    );
    expect(result).toEqual({ types: ['guid2'] });
  });

  it('deletes key if empty', () => {
    const result = removeFromSelection(
      { types: ['guid1'] },
      'types',
      'guid1'
    );
    expect(result).toEqual({});
    expect('types' in result).toBe(false);
  });

  it('no-op if not selected', () => {
    const result = removeFromSelection(
      { types: ['guid1'] },
      'types',
      'guid2'
    );
    expect(result).toEqual({ types: ['guid1'] });
  });
});

describe('toggleInSelection', () => {
  it('adds if missing', () => {
    const result = toggleInSelection({}, 'types', 'guid1');
    expect(result).toEqual({ types: ['guid1'] });
  });

  it('removes if present', () => {
    const result = toggleInSelection(
      { types: ['guid1'] },
      'types',
      'guid1'
    );
    expect(result).toEqual({});
  });
});

describe('clearSelection', () => {
  it('returns empty object', () => {
    const result = clearSelection();
    expect(result).toEqual({});
  });
});
```

---

## 9. Implementation Checklist

- [ ] Create `js/semio/sketchpad/kitSelectionHelpers.ts` with generic utilities
- [ ] Add `createDimensionSelectionHooks` factory to Kit.tsx
- [ ] Generate 54 hooks (9 dimensions × 6 operations)
- [ ] Add `useKitAppSelectAll()` hook
- [ ] Deprecate old `useKitAppSelectType` / `useKitAppDeselectType` (keep as aliases)
- [ ] Update inverse diff to include ports, tags, concepts
- [ ] Wire modifier key handlers in table components
- [ ] Add keyboard shortcuts (Escape, Ctrl+A, Delete)
- [ ] Write unit tests for generic helpers
- [ ] Write integration tests for hooks
- [ ] Update documentation

---

## 10. Benefits of This Design

✅ **Type-safe:** Generic helpers enforce correct types per dimension  
✅ **DRY:** Single factory generates all hooks (no copy-paste)  
✅ **Dimension independence:** Selecting types never affects designs/ports/tags  
✅ **Explicit semantics:** Clear naming (add/remove/toggle/replace)  
✅ **Modifier key UX:** Standard pattern matching Design's approach  
✅ **Backward compatible:** Old hooks kept as deprecated aliases  
✅ **Testable:** Pure utility functions, easy to unit test  
✅ **Scalable:** Adding a new dimension requires only 1 line (call factory)  
✅ **Consistent:** All dimensions follow same 6-operation pattern  
✅ **Empty convention:** Clean serialization, no redundant data
