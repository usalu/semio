# Plan: Separate Filter and Selection Tools in Kit App

## User Request
- Separate filter tools and selection tools into two distinct toolbar sections.
- Render them in separate horizontally-adjacent toolbar containers.
- Filter section shows all artifact kind toggles (currently all active by default—make them inactive by default, activate on click).
- Selection section should show tool kind buttons with icons.
- Both sections should use the same Toolbar component but be registered as separate sections with different specificity values to maintain left-to-right ordering.

## Analysis
- **File**: `js/semio/sketchpad/Kit.tsx`
- **Current State**: 
    - `panelVisibility` includes `toolbar`.
    - `addSection` is likely used to register toolbar content.
    - `artifactKinds` constant defines the filters.
    - `selection` state exists.
    - `useKitAppActiveTool` hook exists.
- **Changes Needed**:
    1.  **State Initialization**: Update `createDefaultState` in `Kit.tsx` to initialize `panelVisibility` (or filter state? No, filters are likely derived or stored in state) such that artifact filters are inactive by default. Wait, `Kit.tsx` has `KitAppState` which has `selection` but where are the filters stored? 
        -   Looking at `Kit.tsx` summary: `filterSearch: "", expandedRows: new Set<string>()`.
        -   Wait, there is no explicit "filter" state for artifact kinds in the provided summary of `KitAppState`.
        -   I need to check how artifact filters are currently implemented. They might be implemented as part of the `panelVisibility` or a separate state that is missing from the summary or I missed it.
        -   Actually, the prompt says "Filter section shows all artifact kind toggles". I need to find where these toggles are currently rendered and where their state is kept.
    2.  **Toolbar Split**: 
        -   Currently there is likely one `addSection("toolbar", ...)` call.
        -   I need to split this into two `addSection("toolbar", ...)` calls with different ids (e.g. `kit-filters`, `kit-tools`) and specificity/order.
    3.  **Selection Tools**:
        -   Implement the selection tools section using icons (Selection, Additive, Subtractive, etc. if available, or just the tool modes).
        -   The prompt mentions "Selection section should show tool kind buttons with icons". I need to check `ToolKind` enum and available icons.

## Steps
1.  **Analyze `Kit.tsx`**: Read the full content of `js/semio/sketchpad/Kit.tsx` to understand current toolbar implementation and filter state.
2.  **Refactor Filter State**: If filters are currently implied or default to "all active", introduce explicit state or change the default logic.
3.  **Implement Split Toolbar**:
    -   Create `FilterToolbar` component (or inline).
    -   Create `SelectionToolbar` component (or inline).
    -   Register them via `addSection` in `Kit` component.
4.  **Verify**: Ensure correct rendering and behavior.

## Todo
- [ ] Analyze `js/semio/sketchpad/Kit.tsx` for toolbar and filter logic.
- [ ] Create/Update state for filters if needed (inactive by default).
- [ ] Split toolbar registration into `filters` and `tools` sections.
- [ ] Implement `tools` section with icons.
- [ ] Verify UI and behavior.
