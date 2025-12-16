---
slug: TOOLBAR-TOOLS-E2E
summary: Implement toolbar in apps and extend Type app E2E test for port tool
---
# Previously

User requested:
1. Get tools in apps working
2. Implement a toolbar in the bottom middle between left and right panels
3. Extend the Type app E2E test to test the port tool (select the tool, move cursor to geometry, click to create port)

The toolbar container existed but the tools inside were not rendering.

# Plan

1. Investigate why toolbar sections are not visible
2. Fix the rendering issue 
3. Add E2E test for port tool functionality
4. Clean up test code

# Changes

## Root Cause Analysis

The toolbar was rendering but showing "Loading..." because:
1. `ToolsToggleGroup` was registered as a panel section by `TypeApp`
2. When rendered in `LayoutWrapper`, it used `useParams()` to get route params
3. `useParams()` returned empty because the toolbar is rendered outside `<AppRouter>` routes
4. The null check `if (!kit || !type || !canSetActiveTool) return null` caused empty render

## Fix 1: ToolbarScopeWrapper (Sketchpad.tsx)

Created `ToolbarScopeWrapper` component that:
- Parses URL path to extract kit/type/design/quality GUIDs
- Wraps toolbar content with appropriate scope providers (KitScopeProvider, TypeScopeProvider, etc.)
- Provides the context that `useKitScope()` and `useTypeScope()` hooks need

```tsx
const ToolbarScopeWrapper: FC<{ children: ReactNode }> = ({ children }) => {
  const location = useLocation();
  const scopeGuids = useMemo(() => {
    const pathMatch = location.pathname.match(/^\/kits\/([^/?]+)(?:\/(designs|types|qualities)\/([^/?]+))?/);
    return { kit: pathMatch?.[1], itemType: pathMatch?.[2], item: pathMatch?.[3] };
  }, [location.pathname]);
  // ... wrap with appropriate scope providers
};
```

Applied wrapper in toolbar render section.

## Fix 2: ToolsToggleGroup (Type.tsx)

Changed from `useParams()` to scope hooks:
```tsx
// Before
const { kit, type } = useParams();

// After
const kitScope = useKitScope();
const typeScope = useTypeScope();
const kit = kitScope?.guid;
const type = typeScope?.guid;
```

## Fix 3: Added toolbar container ID (Sketchpad.tsx)

Added `id="semio.sketchpad.toolbar"` to the toolbar container div for testability.

## Fix 4: E2E Test (sketchpad.test.ts)

Extended Type test to:
1. Wait for toolbar to be visible
2. Locate port and selection tool toggles
3. Click port tool button to activate it
4. Verify tool is activated (data-state="on")
5. Move cursor to canvas center
6. Click to simulate port creation
7. Switch back to selection tool

Note: The test found duplicate IDs (container and button share same ID) - worked around with `locator('button[role="radio"]').first()`.
