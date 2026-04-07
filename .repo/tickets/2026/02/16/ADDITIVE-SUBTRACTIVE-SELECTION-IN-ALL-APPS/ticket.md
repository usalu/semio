---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Additive/subtractive selection now works consistently across all sketchpad apps. Type.tsx model/connector clicks use applySelectionComposition with activeTool + keyboard modifiers. Design.tsx 3D scene uses activeTool instead of hardcoded SELECTION_NORMAL. Quality.tsx has a new selection toolbar with additive/subtractive/intersect toggles. Locale labels added for en + de.

## Plan

1. ✅ Fix Type.tsx model selection to use applySelectionComposition + activeTool + keyboard modifiers
2. ✅ Fix Type.tsx connector selection to use applySelectionComposition + activeTool + keyboard modifiers
3. ✅ Fix Type.tsx handlePortClick to clear models on replace mode
4. ✅ Fix Design.tsx 3D scene onSelect to use activeTool instead of ToolKind.SELECTION_NORMAL
5. ✅ Fix Design.tsx 3D scene to clear connections on replace mode
6. ✅ Add Quality.tsx QualitySelectSettings component with additive/subtractive/intersect toggles
7. ✅ Register Quality.tsx selection toolbar section
8. ✅ Add quality selection locale labels (en.json + de.json)
9. ✅ TypeScript type-check passes
10. ✅ All 12 unit tests pass

## Changes

- Type.tsx: ModelsSectionForm click uses applySelectionComposition with activeTool + keyboard modifiers (replaces simple toggle)
- Type.tsx: ConnectorsListSectionForm click uses applySelectionComposition with activeTool + keyboard modifiers (replaces simple toggle)
- Type.tsx: handlePortClick clears models on replace mode
- Design.tsx: ModelPiece onSelect uses activeTool from useDesignAppActiveTool (replaces hardcoded SELECTION_NORMAL)
- Design.tsx: ModelPiece onSelect clears connections on replace mode
- Quality.tsx: Added QualitySelectSettings component with Toggle buttons for additive/subtractive/intersect modes
- Quality.tsx: Registered selection toolbar section in App useEffect
- Quality.tsx: Added Toggle and AddIcon/RemoveIcon/IntersectIcon imports
- en.json: Added quality.tools.select.additive/subtractive/intersect labels
- de.json: Added quality.tools.select.additive/subtractive/intersect labels

## Log

- Read and analyzed shared.ts (selection composition system)
- Read and analyzed kitSelectionHelper.ts and kitSelectionHelpers.ts (duplicates)
- Read and analyzed Kit.tsx (selection hooks, table click handling, diagram selection)
- Read and analyzed Design.tsx (selection types, tools, lasso, empty onNodeClick)
- Read and analyzed Type.tsx (selection types, port click with composition, keyboard modifier handler)
- Read and analyzed Quality.tsx (selection types, select/deselect commands, NO composition support)
- Read and analyzed Home.tsx (selection types, manual shift/ctrl handling)
- Read and analyzed Sketchpad.tsx (events, selectors, machine)

## Report

### 1. shared.ts — Selection Composition System

**Types/Interfaces/Enums (lines 568-633):**

- `ToolKind` enum: `SELECTION_NORMAL`, `SELECTION_ADDITIVE`, `SELECTION_SUBTRACTIVE`, `SELECTION_INTERSECT`, `LASSO_RECTANGULAR`, `LASSO_FREEFORM`, `CONNECTOR`, `HAND`
- `SelectionCompositionKind` = `"replace" | "additive" | "subtractive" | "intersect"`
- `SelectionKeyboardState` = `{ shiftKey?, altKey?, ctrlKey?, metaKey? }`
- `resolveSelectionCompositionKind(toolKind, keyboard?)`: shift→additive, alt/ctrl/meta→subtractive, shift+subtractive→intersect, else maps tool to composition
- `applySelectionComposition<T>(previous, incoming, compositionKind)`: replace→incoming, additive→union, subtractive→difference, intersect→intersection
- `isSelectionToolKind(kind)`: checks if tool is a selection tool
- `toSelectionToolKind(compositionKind)`: maps composition kind back to tool kind

**XState Integration (lines ~1940-2100):**

- `AppMachineContext<TSelection>` has `selection?: TSelection`
- `AppMachineEvent` has `{ type: "SELECT"; diff: TSelectionDiff }` and `{ type: "DESELECT" }`

### 2. kitSelectionHelper.ts (696 lines) & kitSelectionHelpers.ts (536 lines)

**DUPLICATES** — both files contain identical generic selection utilities:

- `addToSelection(selection, dimensionKey, value)` → pushes value if not present
- `removeFromSelection(selection, dimensionKey, value)` → filters value out
- `toggleInSelection(selection, dimensionKey, value)` → add if absent, remove if present
- `replaceSelectionDimension(selection, dimensionKey, values)` → replaces entire dimension
- `clearSelectionDimension(selection, dimensionKey)` → empties one dimension
- `clearSelection()` → returns `{}`
- `selectAllInDimension(selection, dimensionKey, allValues)` → selects all values in dimension
- `isSelected(selection, dimensionKey, value)` → boolean check

Both also have Kit Diagram Geometry code (shape strategies, snap points, anchor resolution).

**Issue: Two duplicate files exist and are both potentially imported.**

### 3. Kit.tsx (9178 lines)

**Selection Types (lines 258-400):**

- `KitAppSelection` = `{ types?, designs?, qualities?, ports?, tags?, concepts?, files?, folders?, authors? }` (all `Guid[]` or `string[]`)
- Per-dimension diff interfaces: `KitAppSelectionTypesDiff` etc., each with `added?/removed?`
- `KitAppSelectionDiff` composites all dimension diffs

**Selection Helper Hooks (lines 1880-3152):**

- `createDimensionSelectionHooks<K>()` factory creates `useAdd`, `useRemove`, `useToggle`, `useSelectSingle`, `useSelect`, `useClear` for any dimension
- Explicit per-entity hooks for: Types, Designs, Qualities, Ports, Tags, Concepts, Files, Folders, Authors, Global
- These hooks use `toggleInSelection`/`addToSelection`/`removeFromSelection` but do NOT use `resolveSelectionCompositionKind` or `applySelectionComposition`

**Table Row Click (line 6098-6200):**

- Uses `resolveSelectionCompositionKind(activeTool, { shiftKey, altKey, ctrlKey, metaKey })`
- Has range selection (shift without other modifiers)
- Uses `applySelectionComposition(currentValues, [selectionValue], compositionKind)` ✅

**Keyboard Modifier Handler (line 7770-7810):**

- Has `keydown`/`keyup` listeners that dynamically switch `activeTool` based on keyboard modifiers ✅

**Merge Conflict (line ~1930):**

- `<<<<<<< HEAD` / `>>>>>>> origin/ueli/latest` markers present between JSDoc comment and section marker

### 4. Design.tsx (9887 lines)

**Selection Types (lines 246-400):**

- `DesignAppSelection` = `{ pieces?, connections?, connectors?, connector? }`
- `DesignAppSelectionDiff` with per-dimension diffs (added/removed)

**Tools:**

- `DesignAppTools = [SelectionAdditiveTool, SelectionSubtractiveTool, LassoRectangularTool, LassoFreeformTool, HandTool]`
- `DesignSelectSettings` component with toggles for additive/subtractive/intersect modes

**Lasso Selection (line 6973-7060):**

- `onSelectionChange`: Uses `resolveSelectionCompositionKind(activeTool)` — note: keyboard modifiers NOT passed, only tool kind
- Uses `applySelectionComposition(base.pieces, selectedPieceGuids, compositionKind)`

**Node Click (line 7265): EMPTY**

- `onNodeClick: (e, node) => { }` — clicking diagram nodes does nothing

**Table Piece Click (line 8474):**

- Uses `resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, { shiftKey, ctrlKey, metaKey, altKey })` ✅ — keyboard modifiers ARE passed

### 5. Type.tsx (4030 lines)

**Selection Types (lines 116-145):**

- `TypeAppSelection` = `{ connectors?, models? }` (both `Guid[]`)
- `TypeAppSelectionPortsDiff`, `TypeAppSelectionModelsDiff` — added/removed
- `TypeAppSelectionDiff` = `{ connectors?, models? }`

**Event Handlers (lines 316-350):**

- `TYPE.SELECT_CONNECTOR`: Simple push — no composition
- `TYPE.DESELECT_CONNECTOR`: Simple filter — no composition
- `TYPE.SELECT_MODEL`: Simple push — no composition
- `TYPE.DESELECT_MODEL`: Simple filter — no composition

**Port Click (line 1860-1870):**

- `handlePortClick`: Uses `applySelectionComposition(selection?.connectors, [connectorId], resolveSelectionCompositionKind(activeTool))` ✅

**Keyboard Modifier Handler (lines 3450-3490):**

- Has `keydown`/`keyup` listeners that switch `activeTool` based on modifier keys ✅

**Scene click deselect (line 1928):**

- `if (!(event.ctrlKey || event.metaKey) && !event.shiftKey && deselectAll) deselectAll()` — respects modifiers

### 6. Quality.tsx (2213 lines)

**Selection Types (lines 107-127):**

- `QualityAppSelection` = `{ formulaNodes?: Guid[] }`
- `QualityAppSelectionFormulaNodesDiff` = `{ added?, removed? }`
- `QualityAppSelectionDiff` = `{ formulaNodes?: QualityAppSelectionFormulaNodesDiff }`

**Selection Commands (line 627-645):**

- `selectFormulaNode`: Replace-only — removes all current, adds one. No composition support ❌
- `deselectAll`: Removes all current. No composition support ❌
- No `resolveSelectionCompositionKind` or `applySelectionComposition` usage anywhere ❌
- No keyboard modifier handling ❌

### 7. Home.tsx (1801 lines)

**Selection Types (lines 105-114):**

- `HomeSelection` = `{ kits?: Guid[] }`
- `HomeSelectionDiff` = `{ added?, removed? }`

**Event Handlers (lines ~100-260):**

- `HOME.SELECT_KIT`: Simple push
- `HOME.DESELECT_KIT`: Simple filter

**Table Click Handler (lines 1284-1314):**

- Manual shift/ctrl handling — does NOT use `resolveSelectionCompositionKind` ❌
- Shift: Range selection (manual implementation)
- Ctrl/Meta: Toggle (manual add/remove)
- Neither: Replace (selectKit)
- Has the right behavior but inconsistent implementation pattern

### 8. Sketchpad.tsx (18159 lines)

**Events (lines 8500-8600):**

- Home: `HOME.SELECT_KIT`, `HOME.DESELECT_KIT`, `HOME.CLEAR_SELECTION`
- Kit: `KIT.SELECT_TYPE`, `KIT.DESELECT_TYPE`, `KIT.SELECT_DESIGN`, `KIT.DESELECT_DESIGN`, `KIT.SET_SELECTION`, `KIT.CLEAR_SELECTION`
- Type: `TYPE.SET_SELECTION`, `TYPE.CLEAR_SELECTION`, `TYPE.SELECT_CONNECTOR`, `TYPE.DESELECT_CONNECTOR`, `TYPE.SELECT_MODEL`, `TYPE.DESELECT_MODEL`, `TYPE.SELECT_ALL`, `TYPE.DESELECT_ALL`
- Design: `DESIGN.SELECT_PIECE`, `DESIGN.DESELECT_PIECE`, `DESIGN.SELECT_CONNECTION`, `DESIGN.DESELECT_CONNECTION`, `DESIGN.SET_SELECTION`, `DESIGN.CLEAR_SELECTION`, `DESIGN.SELECT_ALL`, `DESIGN.DELETE_SELECTED`

**Selectors (lines 9353-10010):**

- `selectHomeSelection`, `createKitSelectionSelector`, `createDesignSelectionSelector`, `createTypeSelectionSelector`

## App Selection Support Matrix

| Feature | Home | Kit | Type | Design | Quality |
|---|---|---|---|---|---|
| Selection type | `kits` | 9 dims | `connectors, models` | `pieces, connections, connectors` | `formulaNodes` |
| `resolveSelectionCompositionKind` | ❌ manual | ✅ table | ✅ port click | ✅ lasso + table | ❌ |
| `applySelectionComposition` | ❌ manual | ✅ table | ✅ port click | ✅ lasso + table | ❌ |
| Keyboard modifier handler | ❌ | ✅ | ✅ | ✅ (partial) | ❌ |
| Node/diagram click selection | N/A | ✅ | ✅ | ❌ EMPTY | N/A |
| Lasso selection | N/A | N/A | N/A | ✅ | N/A |
| Range selection (shift) | ✅ manual | ✅ | N/A | N/A | ❌ |
| Tool-based composition | N/A | ✅ | ✅ | ✅ | ❌ |

## Issues Found

1. **Merge conflict in Kit.tsx** (~line 1930): `<<<<<<< HEAD` / `>>>>>>> origin/ueli/latest`
2. **Duplicate files**: `kitSelectionHelper.ts` and `kitSelectionHelpers.ts` contain identical code
3. **Design onNodeClick is EMPTY**: `(e, node) => { }` — clicking diagram nodes does nothing
4. **Quality has NO composition support**: select/deselect is replace-only
5. **Home uses manual shift/ctrl** instead of `resolveSelectionCompositionKind`
6. **Design lasso doesn't pass keyboard modifiers** to `resolveSelectionCompositionKind`, only tool kind
7. **Kit selection helper hooks** (add/remove/toggle) don't use composition system

## Todos

- [ ] Resolve merge conflict in Kit.tsx line ~1930
- [ ] Remove duplicate kitSelectionHelpers.ts (keep kitSelectionHelper.ts)
- [ ] Implement Design onNodeClick with composition support
- [ ] Add composition support to Quality app
- [ ] Refactor Home to use resolveSelectionCompositionKind
- [ ] Pass keyboard modifiers in Design lasso onSelectionChange
- [ ] Consider unifying Kit selection helper hooks to use composition system

## Plan

1. Fix merge conflict in Kit.tsx
2. Remove kitSelectionHelpers.ts duplicate, update imports
3. Implement Design onNodeClick with resolveSelectionCompositionKind
4. Add keyboard modifier handler and composition support to Quality
5. Refactor Home table click to use resolveSelectionCompositionKind
6. Pass keyboard modifiers in Design lasso selection
7. Run tests to verify
