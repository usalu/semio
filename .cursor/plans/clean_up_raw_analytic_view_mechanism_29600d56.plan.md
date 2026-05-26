---
name: Clean Up Raw/Analytic View Mechanism
overview: ""
todos:
  - id: scope-display
    content: Scope replDisplayedSelectionTargets and its call site to current pickViewKind
    status: completed
  - id: toggle-preserve
    content: Remove selection-clearing and forced auto-switch when toggling view
    status: completed
  - id: derived-refresh
    content: Refresh derived view service regardless of interaction/view
    status: completed
  - id: view-masked-toggles
    content: Add view-masked toggle helpers and pass them into picking/scene/hover code paths
    status: completed
  - id: view-scoped-merge
    content: Make commit/dispatch/snap-pick merge only within current-view entities and preserve out-of-view entries
    status: completed
  - id: tests
    content: Extend tests for view-scoped display, view-scoped merge, and view-masked toggle helpers
    status: completed
  - id: ticket-close
    content: Close ticket 2026/05/26/REFACTOR-SPATIAL-SELECTION via repo MCP
    status: completed
isProject: false
---

# Clean Up Raw/Analytic View Mechanism

## Problem

In [spatial/js/renderer-r3f/index.tsx](spatial/js/renderer-r3f/index.tsx) the `pickViewKind` mechanism (`"raw" | "analytic"`) is buggy:

- View toggle is forced by `activeSelectionAccept` (line 3674 effect) so during an interaction the user cannot freely switch.
- Toggling the view clears `rendererSelection` and `interactionSelection` (lines 4571-4572), destroying the user's selection on a pure UI toggle.
- `displayedSelectionTargets`, the visibility filter (`filterKindToggles`), the selection filter (`selectionKindToggles`), and the analytic exposure/stance/overlap toggles are not scoped to the current view, so cross-view entities can bleed into the highlight set and the toggle UIs interpret state of the inactive view.
- The `derived.refresh` effect (line 3614) bails out when an interaction is active and view is not analytic — switching to analytic during an interaction shows stale derived data.

## Goal

- Raw / analytic is a pure display+filter concern. Storage of selection is unaffected by the toggle.
- The user can toggle the view at any time (idle or during an interaction).
- Selection display, selection filter UI, visibility filter UI, and analytic toggles only act on entities of the currently active view.
- Derived data is always fresh when the user switches to analytic.

## Concrete changes (single file: [spatial/js/renderer-r3f/index.tsx](spatial/js/renderer-r3f/index.tsx))

### 1. Selection display is scoped to the current view

Extend `replDisplayedSelectionTargets` to take `pickViewKind` and filter selection targets by `spatialPickViewKindSet(pickViewKind)`:

```ts
export function replDisplayedSelectionTargets(
  interactionActive: boolean,
  pickViewKind: SpatialPickViewKind,
  rendererSelection: readonly SelectionTarget[],
  interactionSelection: readonly SelectionTarget[],
): readonly SelectionTarget[] {
  const layer = interactionActive ? interactionSelection : rendererSelection;
  const allowed = spatialPickViewKindSet(pickViewKind);
  return layer.filter((t) => allowed.has(t.kind as SpatialPickTargetKind));
}
```

Update the single call site (line 3513) to pass `pickViewKind`.

### 2. View toggle preserves selection and is always allowed

Lines 4564-4573: drop the `setRendererSelection([])` / `setInteractionSelection([])` calls on toggle. Keep only `setSelectionMenu(null)` and `setHoveredPickKey(null)`. The selection store is view-agnostic; raw selections are hidden in analytic view and vice versa via step 1.

### 3. Remove the forced auto-switch effect

Delete the effect at lines 3674-3684 that forces `pickViewKind` from `activeSelectionAccept`. An interaction's accept set already filters what is selectable; it must not override the user's chosen view. The user toggles the view manually.

### 4. Derived refresh is unconditional on view

In the effect at lines 3614-3631 remove `if (interactionActive && pickViewKind !== "analytic") return;` and the `pickViewKind` dependency. Derived data must always track topology so switching to analytic at any time shows fresh surfaces/parts.

### 5. Scope visibility and selection filter UIs to the current view

The "Show kinds" and "Selection kinds" UIs (lines 4628 and 4751) already iterate `activePickViewKinds`. Lock down the side-effects so they cannot touch the other view's data:

- "Selection kinds" uncheck at line 4777: filter selections by `target.kind === kind` only (no change there). It is fine because `kind` is constrained to `activePickViewKinds`.
- "Show exposure / stance / overlap" sections (lines 4655, 4686, 4717, 4789, 4826, 4858 area) are already gated by `pickViewKind === "analytic"`.
- Add tiny helpers near `defaultSpatialPickKindToggles`:
  - `spatialPickKindTogglesForView(toggles, view)` returns a `SpatialPickKindToggles` masked to the view's kinds.
  - `spatialAnalyticTogglesForView(toggles, view)` returns either `toggles` for analytic or `{}` for raw.
- Where the renderer feeds the scene and the hover ray (lines 3759-3771, 3787, 3829, 3901, 3919, 3955, 4235-4236, 4314, plus the `SpatialPickGeometryLayer` props on line 2891), pass the view-masked toggles via these helpers. This guarantees raw-view ray picks never consult analytic toggles and analytic-view picks never consult raw toggles.

### 6. Selection menu, hover, and dispatch use view-scoped current selection

`commitSelection` (line 3686), `dispatchSelectionTargets` (line 3696), and the snap-pick branch (line 3828) already read `currentSelection = interactionActive ? interactionSelection : rendererSelection`. Wrap that read with the same view filter as step 1 so merging/inversion only sees the current view's entities. The unaffected (other-view) entries are concatenated back when committing so they survive the merge:

```ts
const allowed = spatialPickViewKindSet(pickViewKind);
const layer = interactionActive ? interactionSelection : rendererSelection;
const inView = layer.filter((t) => allowed.has(t.kind as SpatialPickTargetKind));
const outOfView = layer.filter((t) => !allowed.has(t.kind as SpatialPickTargetKind));
const merged = mergeSelectionTargets(inView, picked, mode);
commitSelection([...outOfView, ...merged]);
```

This is the central fix for "selection filter and visibility filter must be correct only for the current entities."

### 7. Lifecycle resets stay as they are

`interactionId` change keeps clearing `interactionSelection` only (line 3649). Topology/derived geometry change keeps clearing both (line 3639). View toggle no longer clears anything (step 2).

### 8. Tests (extend the existing test block, ~line 5283-5460)

In the existing `describe` block around `defaultInteractionReplChromeState` and `replDisplayedSelectionTargets`:

- `replDisplayedSelectionTargets` returns only raw kinds when `pickViewKind = "raw"` and only `surface`/`part` when `"analytic"`, for both renderer and interaction layers.
- View-masked merge: starting from a selection containing a `face` and a `surface`, picking another `face` in raw view yields `[surface, face, face]` (or whichever order the merge returns) and never drops the `surface`. Switching to analytic and clearing leaves the `face` intact.
- `spatialPickKindTogglesForView` masks unrelated kinds: in raw view, an analytic toggle in the input map cannot mark a raw kind hidden, and vice versa.
- Updated `defaultInteractionReplChromeState` assertions still pass (defaults unchanged).

### 9. Ticket bookkeeping

The ticket `2026/05/26/REFACTOR-SPATIAL-SELECTION` is already open (`.repo/🎫/26/05/26/REFACTOR-SPATIAL-SELECTION/ticket.json`). Reuse it; close via repo MCP `ticket_close` after the tests above pass.

## Out of scope

- Volumes: `analyticSummary` already aggregates `surfaces` and `parts` only; adding a third entity is a separate ticket.
- Any change to `./elements`, `./semio`, `./coda`, `./reuse`.
- Changes to `SelectionTarget` / `SelectionSpec` types in `spatial/js/core/index.ts`.
