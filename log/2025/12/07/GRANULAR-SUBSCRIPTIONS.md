---
date: '2025-12-07T20:50:21.810Z'
slug: GRANULAR-SUBSCRIPTIONS
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Migrate all exported hooks to granular subscriptions with [state, setState, canSetState] pattern
model: claude-opus-4.5
---
# Previously

The Sketchpad hooks were using `useKit()`, `useDesign()`, and `useType()` with selector patterns that caused overfetching. Every component that needed just one field (like kit name or pieces array) would subscribe to the entire store, causing unnecessary re-renders when unrelated fields changed.

Current hook patterns have several issues:
1. Hooks take optional `id` parameters instead of using scopes consistently
2. Hooks return only the state value, not setters or capability flags
3. No consistent way to determine if a state can be modified (for disabling UI elements)
4. Setter functions are scattered across `useXxxCommands()` hooks

# Plan

## New Hook Architecture: [STATE, SETSTATE, CANSETSTATE] = useSELECTOR()

All hooks will follow this unified pattern:
```typescript
const [value, setValue, canSetValue] = useValue()
```

### Key Principles:

1. **No Parameters**: Hooks always derive context from scopes (useKitScope, useDesignScope, useTypeScope, usePieceScope, etc.)
2. **Tuple Return**: All hooks return `[state, setState, canSetState]`
3. **Can Flag**: The third value determines if the setter is available (used to disable UI elements)
4. **Scope-Based**: Use provider contexts instead of passing IDs

### Return Type:
```typescript
type GranularHookResult<T> = readonly [T, ((value: T) => void) | undefined, boolean]
```

- `T`: The current state value
- `setState`: Function to update the state (undefined if not settable)
- `canSet`: Boolean indicating if the state can be set (use for disabled prop)

### Hook Categories:

#### 1. Read-Only Hooks (can always be false)
```typescript
const [pieces, , canSet] = usePieces() // canSet always false, no setter
const [depth, , ] = usePieceDepth() // Read-only derived value
```

#### 2. Read-Write Hooks (can depends on context)
```typescript
const [name, setName, canSetName] = useKitName()
const [camera, setCamera, canSetCamera] = useDesignAppCamera()
const [isSelected, setIsSelected, canSetIsSelected] = useIsPieceSelected()
```

#### 3. Nested Field Hooks (deep granularity)
```typescript
const [xAxisY, setXAxisY, canSetXAxisY] = useFlatPiecePlaneXAxisY()
const [originX, setOriginX, canSetOriginX] = usePiecePlaneOriginX()
```

### Migration Strategy:

1. Add `GranularHookResult<T>` type to shared.ts
2. Create `createGranularHook` utility for consistent hook creation
3. Migrate each app's hooks systematically:
   - Sketchpad.tsx: Global and Kit hooks
   - Design.tsx: Design app hooks  
   - Type.tsx: Type app hooks
   - Kit.tsx: Kit app hooks
   - Quality.tsx: Quality app hooks
   - Home.tsx: Home app hooks
   - Docs.tsx: Docs app hooks

4. Update all UI components to use the new `canSet` flag for disabled states

# Changes

## Phase 1: Type Definitions (shared.ts)
- Added `GranularHookResult<T> = [T, (value: T) => void, boolean]` for read-write hooks
- Added `GranularHookNoSetResult<T> = [T, undefined, boolean]` for read-only hooks

## Phase 2: App Hooks Migration

### Design.tsx
- `useDesignAppSelection()` → `[selection, setSelection, canSet]`
- `useDesignAppFullscreen()` → `[fullscreen, setFullscreen, canSet]`
- `useDesignAppCamera()` → `[camera, setCamera, canSet]`
- `useDesignAppActiveTool()` → `[tool, setTool, canSet]`
- `useDesignAppPanelVisibility()` → `[visibility, setVisibility, canSet]`
- `useDesignAppDiagramCenter()` → `[center, setCenter, canSet]`
- `useDesignAppDiagramScale()` → `[scale, setScale, canSet]`
- `useDesignAppHover()` → `[hover, setHover, canSet]`
- `useDesignAppOthers()` → `[others, undefined, canRead]` (read-only)
- `useDesignAppFocusedPieceGuid()` → `[guid, setGuid, canSet]`
- `useDesignAppSelectedModelTags()` → `[tags, setTags, canSet]`
- `useDesignAppIsPieceSelected()` → `[isSelected, undefined, canRead]` (read-only, uses PieceScope)
- `useDesignAppIsPieceHovered()` → `[isHovered, undefined, canRead]` (read-only, uses PieceScope)
- `useDesignAppIsConnectionSelected()` → `[isSelected, undefined, canRead]` (read-only, uses ConnectionScope)
- `useDesignAppIsConnectionHovered()` → `[isHovered, undefined, canRead]` (read-only, uses ConnectionScope)

### Type.tsx
- `useTypeAppSelection()` → `[selection, setSelection, canSet]`
- `useTypeAppFullscreen()` → `[fullscreen, setFullscreen, canSet]`
- `useTypeAppCamera()` → `[camera, setCamera, canSet]`
- `useTypeAppActiveTool()` → `[tool, setTool, canSet]`
- `useTypeAppPanelVisibility()` → `[visibility, setVisibility, canSet]`
- `useTypeAppHover()` → `[hover, setHover, canSet]`
- `useTypeAppOthers()` → `[others, undefined, canRead]` (read-only)
- `useTypeAppIsPortSelected()` → `[isSelected, undefined, canRead]` (read-only, uses PortScope)
- `useTypeAppIsPortHovered()` → `[isHovered, undefined, canRead]` (read-only, uses PortScope)
- `useTypeAppSelectedModelGuid()` → `[guid, setGuid, canSet]`
- `useTypeAppSelectedModelTags()` → `[tags, setTags, canSet]`

### Kit.tsx
- `useKitAppSelection()` → `[selection, setSelection, canSet]`
- `useKitAppFullscreen()` → `[fullscreen, setFullscreen, canSet]`
- `useKitAppOthers()` → `[others, undefined, canRead]` (read-only)
- `useKitAppIsTypeHovered()` → `[isHovered, undefined, canRead]` (uses TypeScope)
- `useKitAppTypeStatus()` → `[status, undefined, canRead]` (uses TypeScope)
- `useKitAppTypeColor()` → `[color, undefined, canRead]` (uses TypeScope)
- `useKitAppIsDesignHovered()` → `[isHovered, undefined, canRead]` (uses DesignScope)
- `useKitAppDesignStatus()` → `[status, undefined, canRead]` (uses DesignScope)
- `useKitAppDesignColor()` → `[color, undefined, canRead]` (uses DesignScope)

### Quality.tsx
- `useQualityAppFullscreen()` → `[fullscreen, setFullscreen, canSet]`
- `useQualityAppSelection()` → `[selection, setSelection, canSet]`
- `useQualityAppHover()` → `[hover, setHover, canSet]`
- `useQualityAppActiveTool()` → `[tool, setTool, canSet]`
- `useQualityAppFormulaNodes()` → `[nodes, undefined, canRead]` (read-only)
- `useQualityAppPanelVisibility()` → `[visibility, setVisibility, canSet]`
- `useQualityAppWindowLayout()` → `[layout, setLayout, canSet]`

### Home (xstate-hooks.ts)
- `useHomePanelVisibility()` → `[visibility, setVisibility, canSet]`
- `useHomeSelection()` → `[selection, setSelection, canSet]`
- `useHomeHover()` → `[hover, setHover, canSet]`
- `useHomeSortColumn()` → `[column, setColumn, canSet]`
- `useHomeSortDirection()` → `[direction, setDirection, canSet]`
- `useHomeLoadingKits()` → `[kits, undefined, canRead]` (read-only)

### Sketchpad.tsx (Field-Level Hooks)

#### Connection Field Hooks (use ConnectionScope)
- `useConnectionGap()` → `[gap, setGap, canSet]`
- `useConnectionShift()` → `[shift, setShift, canSet]`
- `useConnectionRise()` → `[rise, setRise, canSet]`
- `useConnectionRotation()` → `[rotation, setRotation, canSet]`
- `useConnectionTurn()` → `[turn, setTurn, canSet]`
- `useConnectionTilt()` → `[tilt, setTilt, canSet]`
- `useConnectionU()` → `[u, setU, canSet]`
- `useConnectionV()` → `[v, setV, canSet]`

#### Piece Field Hooks (use PieceScope)
- `usePieceCenterU()` → `[centerU, setCenterU, canSet]`
- `usePieceCenterV()` → `[centerV, setCenterV, canSet]`
- `usePieceScale()` → `[scale, setScale, canSet]`
- `usePieceIsHidden()` → `[isHidden, setIsHidden, canSet]`
- `usePieceIsLocked()` → `[isLocked, setIsLocked, canSet]`
- `usePieceColor()` → `[color, setColor, canSet]`
- `usePieceDescription()` → `[description, setDescription, canSet]`
- `usePieceName()` → `[name, setName, canSet]`

## Phase 3: Component Refactoring

### ConnectionsSectionForm Refactoring (Design.tsx)
Refactored `ConnectionsSectionForm` to use granular hooks instead of manual diff creation:

**Before:**
```tsx
const ConnectionsSectionForm: FC<{connections: Connection[]}> = ({ connections }) => {
  const { updateConnection } = useDesignAppCommands();
  // Manual diff creation
  const handleChange = (updatedConnection: Connection) => {
    const diff: ConnectionDiff = {};
    if (updatedConnection.gap !== connection.gap) diff.gap = updatedConnection.gap;
    // ... more manual diff building
    updateConnection(origin, updatedConnection.guid, diff);
  };
  // ... 200+ lines of form with isSingle checks everywhere
};
```

**After:**
```tsx
const SingleConnectionInfo: FC = () => {
  const connection = useConnection() as Connection;
  // Read-only info display using useConnection() hook
};

const SingleConnectionFields: FC = () => {
  const [gap, setGap] = useConnectionGap();
  const [shift, setShift] = useConnectionShift();
  // ... other granular hooks
  // No manual diff creation - hooks handle it internally
};

const ConnectionsSectionForm: FC<{connections: Connection[]}> = ({ connections }) => {
  const isSingle = connections.length === 1;
  if (isSingle) {
    return (
      <ConnectionScopeProvider guid={connections[0].guid}>
        <SingleConnectionInfo />
        <SingleConnectionFields />
      </ConnectionScopeProvider>
    );
  }
  return <MultipleConnectionsMessage />;
};
```

**Key benefits:**
1. No manual diff creation in components
2. Granular subscriptions - only re-render when specific field changes
3. Consistent origin tracking via hooks (origin is in hook implementation)
4. Cleaner separation of concerns (info vs fields)
5. Multi-selection case explicitly separated

## Phase 4: Documentation
- Updated AGENTS.md with new "Granular Hook Architecture" section
- Documented all scope providers and usage patterns
- Added examples for different hook types