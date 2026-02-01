# Kit Selection Quick Reference

## For Developers: Using the Kit Selection System

---

## TL;DR

```typescript
// Use hooks to select items
const [addType, canAdd] = useKitAppAddTypeToSelection();
const [removeType, canRemove] = useKitAppRemoveTypeFromSelection();
const [toggleType, canToggle] = useKitAppToggleTypeInSelection();
const [selectType, canSelect] = useKitAppSelectSingleType();

// Wire to click handlers
<TableRow
  onClick={(e) => {
    if (e.altKey) removeType?.(typeGuid);
    else if (e.shiftKey) addType?.(typeGuid);
    else if (e.ctrlKey || e.metaKey) toggleType?.(typeGuid);
    else selectType?.(typeGuid);
  }}
/>
```

---

## Available Hooks

### Per-Dimension Selection Hooks

Each dimension (types, designs, qualities, ports, tags, concepts, files, folders, authors) has 6 hooks:

```typescript
// Types dimension (Guid-based)
useKitAppAddTypeToSelection(): ActionHookResult<[typeGuid: Guid]>
useKitAppRemoveTypeFromSelection(): ActionHookResult<[typeGuid: Guid]>
useKitAppToggleTypeInSelection(): ActionHookResult<[typeGuid: Guid]>
useKitAppSelectSingleType(): ActionHookResult<[typeGuid: Guid]>
useKitAppSelectTypes(): ActionHookResult<[typeGuids: Guid[]]>
useKitAppClearTypeSelection(): ActionHookResult<[]>

// Files dimension (string-based)
useKitAppAddFileToSelection(): ActionHookResult<[fileName: string]>
useKitAppRemoveFileFromSelection(): ActionHookResult<[fileName: string]>
useKitAppToggleFileInSelection(): ActionHookResult<[fileName: string]>
useKitAppSelectSingleFile(): ActionHookResult<[fileName: string]>
useKitAppSelectFiles(): ActionHookResult<[fileNames: string[]]>
useKitAppClearFileSelection(): ActionHookResult<[]>

// ... same pattern for all 9 dimensions
```

### Global Hooks

```typescript
// Get current selection
useKitAppSelection(): HookResult<KitAppSelection>

// Select all items across all dimensions
useKitAppSelectAll(): ActionHookResult<[]>

// Clear entire selection
useKitAppClearSelection(): ActionHookResult<[]>
```

---

## Hook Pattern

All hooks return `ActionHookResult<TArgs>`:

```typescript
type ActionHookResult<TArgs extends any[]> = readonly [
  action: ((...args: TArgs) => void) | undefined,
  canAct: boolean
];

// Usage
const [addType, canAdd] = useKitAppAddTypeToSelection();

if (canAdd) {
  addType(typeGuid); // Safe to call
}

// Or conditionally render
{canAdd && <Button onClick={() => addType?.(typeGuid)}>Add</Button>}
```

---

## Modifier Key Semantics

Standard desktop app behavior:

| Modifier | Action | Hook to Use |
|----------|--------|-------------|
| None | Replace selection (select only this) | `useKitAppSelectSingle*()` |
| Ctrl/Cmd | Toggle in selection | `useKitAppToggle*InSelection()` |
| Shift | Add to selection | `useKitAppAdd*ToSelection()` |
| Alt | Remove from selection | `useKitAppRemove*FromSelection()` |

### Example Click Handler

```typescript
function handleTypeClick(typeGuid: Guid, event: React.MouseEvent) {
  const [addType] = useKitAppAddTypeToSelection();
  const [removeType] = useKitAppRemoveTypeFromSelection();
  const [toggleType] = useKitAppToggleTypeInSelection();
  const [selectType] = useKitAppSelectSingleType();

  if (event.altKey) {
    removeType?.(typeGuid);
  } else if (event.shiftKey) {
    addType?.(typeGuid);
  } else if (event.ctrlKey || event.metaKey) {
    toggleType?.(typeGuid);
  } else {
    selectType?.(typeGuid);
  }
}
```

---

## Selection State Shape

```typescript
interface KitAppSelection {
  types?: Guid[];        // Type GUIDs
  designs?: Guid[];      // Design GUIDs
  qualities?: string[];  // Quality names
  ports?: Guid[];        // Port GUIDs
  tags?: Guid[];         // Tag GUIDs
  concepts?: Guid[];     // Concept GUIDs
  files?: string[];      // File names
  folders?: Guid[];      // Folder GUIDs
  authors?: string[];    // Author emails
}
```

**Important:**
- All fields are optional
- Empty dimensions are **deleted** (not stored as `[]`)
- Dimensions are independent (selecting types doesn't affect designs)

---

## Common Patterns

### 1. Table Row Click

```typescript
<TableRow
  onClick={(e) => handleRowClick(item.guid, e)}
  className={isSelected(selection, "types", item.guid) ? "selected" : ""}
>
  {item.name}
</TableRow>
```

### 2. Diagram Node Click

```typescript
<Node
  onClick={(e) => {
    if (e.altKey) removeType?.(node.id);
    else if (e.shiftKey) addType?.(node.id);
    else if (e.ctrlKey) toggleType?.(node.id);
    else selectType?.(node.id);
  }}
  isSelected={isSelected(selection, "types", node.id)}
/>
```

### 3. Background Click (Clear)

```typescript
<Canvas
  onClick={(e) => {
    if (e.target === e.currentTarget) {
      clearSelection?.();
    }
  }}
>
  {children}
</Canvas>
```

### 4. Keyboard Shortcuts

```typescript
useHotkeys("escape", () => clearSelection?.());
useHotkeys("mod+a", (e) => {
  e.preventDefault();
  selectAll?.();
});
```

### 5. Check if Selected

```typescript
import { isSelected } from "./kitSelectionHelpers";

const selected = isSelected(selection, "types", typeGuid);

// Use in rendering
<Icon className={selected ? "text-primary" : "text-muted"} />
```

---

## Helper Functions

Direct usage (not common, prefer hooks):

```typescript
import {
  addToSelection,
  removeFromSelection,
  toggleInSelection,
  clearSelection,
  clearSelectionDimension,
  replaceSelectionDimension,
  isSelected,
} from "./kitSelectionHelpers";

// Add item to dimension
const newSelection = addToSelection(selection, "types", typeGuid);

// Remove item from dimension
const newSelection = removeFromSelection(selection, "types", typeGuid);

// Toggle item in dimension
const newSelection = toggleInSelection(selection, "types", typeGuid);

// Replace entire dimension
const newSelection = replaceSelectionDimension(selection, "types", [guid1, guid2]);

// Clear specific dimension
const newSelection = clearSelectionDimension(selection, "types");

// Clear all selections
const empty = clearSelection();

// Check if selected
const selected = isSelected(selection, "types", typeGuid);
```

---

## Dimension Reference

| Dimension | Type | Example Value |
|-----------|------|---------------|
| `types` | `Guid[]` | `["550e8400-e29b-41d4-a716-446655440000"]` |
| `designs` | `Guid[]` | `["660e8400-e29b-41d4-a716-446655440001"]` |
| `qualities` | `string[]` | `["energy-efficiency", "cost"]` |
| `ports` | `Guid[]` | `["770e8400-e29b-41d4-a716-446655440002"]` |
| `tags` | `Guid[]` | `["880e8400-e29b-41d4-a716-446655440003"]` |
| `concepts` | `Guid[]` | `["990e8400-e29b-41d4-a716-446655440004"]` |
| `files` | `string[]` | `["model.glb", "texture.png"]` |
| `folders` | `Guid[]` | `["aa0e8400-e29b-41d4-a716-446655440005"]` |
| `authors` | `string[]` | `["user@example.com"]` |

---

## Testing Your Integration

### 1. Manual Verification

- [ ] Click item → only that item selected
- [ ] Ctrl+click → toggle selection
- [ ] Shift+click → add to selection
- [ ] Alt+click → remove from selection
- [ ] Click background → clear selection
- [ ] Escape key → clear selection
- [ ] Selecting types doesn't clear designs

### 2. Unit Test Example

```typescript
import { addToSelection, isSelected } from "./kitSelectionHelpers";

it("should select a type", () => {
  const selection = {};
  const newSelection = addToSelection(selection, "types", "type-1");
  
  expect(isSelected(newSelection, "types", "type-1")).toBe(true);
  expect(newSelection.types).toEqual(["type-1"]);
});
```

---

## Performance Tips

1. **Use useMemo for callbacks:**
   ```typescript
   const handleClick = useMemo(() => {
     if (!canSelect) return undefined;
     return (guid: Guid) => selectType(guid);
   }, [canSelect, selectType]);
   ```

2. **Avoid unnecessary re-renders:**
   ```typescript
   // Good: Only re-render when types selection changes
   const typesSelected = selection?.types || [];
   
   // Bad: Re-render on any selection change
   const anySelected = Object.keys(selection || {}).length > 0;
   ```

3. **Batch operations when possible:**
   ```typescript
   // Good: Select multiple at once
   selectTypes([guid1, guid2, guid3]);
   
   // Bad: Multiple individual selections
   addType(guid1);
   addType(guid2);
   addType(guid3);
   ```

---

## Troubleshooting

### Hook returns undefined action

**Cause:** `canAct` is `false`, usually because:
- `setSelection` is unavailable (no KitScope)
- State machine doesn't permit selection changes

**Fix:**
```typescript
const [action, canAct] = useKitAppAddTypeToSelection();

if (!canAct) {
  console.log("Selection not available");
  return <DisabledButton />;
}
```

### Selection not syncing across tabs

**Cause:** Y.js sync issue

**Fix:**
- Verify KitStore is using Y.Map for selection
- Check network tab for WebSocket connection
- Ensure `yProvider` is configured correctly

### Empty arrays in selection object

**Cause:** Not using helper functions correctly

**Fix:**
```typescript
// Wrong: Manual assignment
selection.types = [];

// Right: Use removeFromSelection (deletes key when empty)
const newSelection = removeFromSelection(selection, "types", lastTypeGuid);
```

### TypeScript errors with hooks

**Cause:** Type parameter mismatch

**Fix:**
```typescript
// Types dimension uses Guid
const [addType] = useKitAppAddTypeToSelection();
addType("type-guid" as Guid); // Cast to Guid

// Files dimension uses string
const [addFile] = useKitAppAddFileToSelection();
addFile("model.glb"); // Plain string
```

---

## See Also

- **Implementation Details:** `KIT_SELECTION_COMPLETION_SUMMARY.md`
- **Test Plan:** `KIT_SELECTION_TEST_PLAN.md`
- **Testing Summary:** `KIT_SELECTION_TESTING_SUMMARY.md`
- **Design Document:** `KIT_SELECTION_HELPERS_DESIGN.md`
- **Usage Examples:** `js/semio/sketchpad/KitSelectionExample.tsx`

---

## Questions?

If you encounter issues:
1. Check test suite for examples: `js/semio/sketchpad/kitSelection.test.ts`
2. Review helper functions: `js/semio/sketchpad/kitSelectionHelpers.ts`
3. Look at hook implementations: `js/semio/sketchpad/Kit.tsx` (lines 1517-2363)
4. File an issue in the repository
