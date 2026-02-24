---
goal: SKETCHPAD
---

# Ticket

## Summary

Research completed: Full analysis of Design detail panel PiecesSection rendering pipeline, section registration, selection flow, and potential issues documented.
## Findings

### 1. PiecesSection Component (Design.tsx:4624-5634)

`PiecesSection` is a thin wrapper around `PiecesSectionForm`:

```tsx
export const PiecesSection: FC = () => {
  return <PiecesSectionForm />;
};
```

`PiecesSectionForm` (line 4628) does the heavy lifting:
- Reads `useDesignAppSelection()` to get `selection.pieces`
- Computes `knownSelectablePieceIds` from design pieces + included designs
- Resolves `selectedPieceIds` via `resolveSelectionEntryGuidByKnownIds`
- Falls back to `fallbackKnownSelectedPieceIds` if no valid selection resolved
- Uses `usePiecesFromIds()` to get actual piece objects
- Enriches pieces by looking up from `directPiecesMap` and `includedDesignMap`
- Renders form fields for single piece (name, description, type selector, position/plane, attributes, connections)
- For multi-select shows common values with batch update capabilities

### 2. Detail Panel Section Registration (Design.tsx:9922-10130)

A `useEffect` in the `App` component (line 9622) manages detail sections:

```
useEffect(() => {
  if (appType !== "design") return;
  // ... compute selection state
  removeSection("details", ...); // clear all previous sections
  
  if (!hasSelection) {
    addSection("details", { id: "...design.properties", content: <DesignSection /> });
  } else if (hasPortSelected) {
    addSection("details", { id: "...connector.properties", content: <ConnectorSection /> });
    addSection("details", { id: "...design.properties", content: <DesignSection /> });
  } else {
    if (hasPieces) {
      addSection("details", { id: piecesSectionId, content: <PiecesSection /> });
    }
    if (hasConnections) {
      addSection("details", { id: connectionsSectionId, content: <ConnectionsSection /> });
    }
    addSection("details", { id: "...design.properties", content: <DesignSection /> });
  }
  addSection("details", { id: "...kit.properties", content: <KitSectionLazy /> });
}, [selection, addSection, removeSection, appType, t, design, kitGuid]);
```

Key behavior:
- Sections are added/removed via `useAddPanelSection`/`useRemovePanelSection` from `PanelSectionProvider` context
- The effect runs when `selection` changes (it's a dependency)
- Each section is wrapped in `KitScopeProvider` + `DesignScopeProvider`
- Specificity ordering: connector (30) > pieces (30) > design (20) > kit (10)

### 3. PanelTabContent Rendering (Sketchpad.tsx:17297-17309)

```tsx
const PanelTabContent: FC<{ sections: PanelSection[] }> = ({ sections }) => {
  const sortedSections = [...sections].sort((a, b) => (a.order || 0) - (b.order || 0));
  return (
    <TreeStateProvider>
      <Tree>
        {sortedSections.map((section, index) => (
          <PanelTabSectionItem key={section.id} section={section} defaultOpen={section.defaultOpen ?? index === 0} />
        ))}
      </Tree>
    </TreeStateProvider>
  );
};
```

`PanelTabSectionItem` renders each section as a `TreeSection` with label from i18n.

### 4. How the Detail Panel Becomes Visible (Sketchpad.tsx:17345-17410)

In `LayoutWrapper`:
- `detailsSections = usePanelSections("details")` reads all registered detail sections
- `sectionsByKind[PanelKind.DETAILS] = detailsSections`
- A `useEffect` maps `panelConfigs[appType]` panels to side panel tabs
- `PanelKind.DETAILS` has `position: PanelPosition.RIGHT` so it becomes a right side panel tab
- The right side panel is visible when `panelVisibility.rightSidePanel` is true AND chat/settings are not active
- Default state: `panelVisibility: { toolbar: true, details: true }` (line 1413) — details IS visible by default

### 5. Is PiecesSection Conditionally Rendered?

YES. `PiecesSection` is only added to the details panel when:
- `appType === "design"` (the effect guards with `if (appType !== "design") return;`)
- `hasPieces === true` (at least one piece GUID resolves to a known piece in the design)
- `hasPortSelected === false` (if a port/connector is selected, ConnectorSection takes priority)

When NO selection exists, only `DesignSection` + `KitSection` are shown.
When pieces ARE selected, `PiecesSection` is added with `specificity: 30, order: 0, defaultOpen: true`.

### 6. Selection Flow

1. User clicks a piece node in diagram → `DESIGN.SELECT_PIECE` event dispatched (line 1510)
2. Event handler updates `app.selection.pieces` array with the piece GUID
3. `useDesignAppSelection()` hook returns updated selection
4. The `App` component's useEffect re-runs (selection is in dependency array)
5. Effect computes `selectedKnownPieceGuids` and adds `PiecesSection` to details panel
6. `PanelSectionProvider` state updates → `usePanelSections("details")` returns new sections
7. `LayoutWrapper` re-renders right side panel with new sections
8. `PanelTabContent` renders `PiecesSection` inside a `TreeSection`

### 7. Potential Issues

**A. No issues with visibility/rendering logic** — The wiring is correct. If a piece is selected:
- The selection state updates
- The useEffect adds a PiecesSection
- The detail panel displays it

**B. Possible data resolution issue** — `PiecesSectionForm` has complex fallback logic (lines 4643-4697) that could fail to resolve pieces if:
- `usePiecesFromIds()` returns objects with `type.name === "unknown"`
- The fallback to `directPiecesMap.get(pieceGuid)` fails because the piece isn't in `design.pieces`
- This would cause `pieces` to be empty → form shows nothing or no data

**C. Scope context dependency** — `PiecesSection` is wrapped in `KitScopeProvider` + `DesignScopeProvider` (line 10083-10086), so `useDesign()`, `useKit()`, and `useDesignAppSelection()` should all resolve correctly.

**D. Section ID toggling** — The section ID changes between `pieceSingleId` vs `pieceMultipleId` based on count. This means when selection changes from 1 to 2 pieces, the old section gets removed and a new one with different ID is added. This could cause a brief flash but shouldn't break functionality.

**E. The `useEffect` dependency on `selection` reference** — If the selection object reference changes on every render even when the content is the same, it could cause unnecessary re-registrations. However, the `useDesignAppSelection` hook uses `createGranularSelector` which likely provides stable references.

**F. `PanelSectionProvider` uses `useState` not `useRef`** — Section updates trigger re-renders of the entire tree since sections are stored in React state.

## Changes

No code changes — research only.

## Log

- Read Design.tsx PiecesSection (4624-5634)
- Read Design.tsx detail panel section registration effect (9922-10130)
- Read Design.tsx App component and plugin registration (1397-1547)
- Read Sketchpad.tsx PanelTabContent + LayoutWrapper (17250-18163)
- Read Sketchpad.tsx PanelSectionProvider (14510-14600)
- Read shared.ts PanelKind/PanelKindConfig (800-870)

## Todos

- [x] Read PiecesSection component
- [x] Read detail panel section registration
- [x] Read PanelTabContent rendering
- [x] Read LayoutWrapper panel wiring
- [x] Analyze selection flow
- [x] Identify potential issues

## Plan

Research-only ticket. No code changes.
