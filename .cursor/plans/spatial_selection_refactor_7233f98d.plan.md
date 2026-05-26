---
name: spatial selection refactor
overview: Refactor the renderer-r3f selection so the renderer always owns a single canonical selection, every interaction has its own private selection seeded from that canonical one, and interactions only contribute back to the renderer when they explicitly archive a result.
todos:
  - id: rename-state
    content: Rename selectedSelectionTargets/interactionSelectionTargets to rendererSelection/interactionSelection across state, props, callbacks, defaults and Omit lists
    status: in_progress
  - id: drop-union
    content: Delete mergeReplSelectionLayers and make displayedSelectionTargets pick rendererSelection or interactionSelection based on interactionActive
    status: pending
  - id: simplify-finalize
    content: Simplify replFinalizeSelection to use archive.targets when present, else keep rendererSelection unchanged
    status: pending
  - id: seed-on-start
    content: Seed interactionSelection from accepted subset of rendererSelection inside startRuntime
    status: pending
  - id: single-commit
    content: Collapse the commit/dispatch helpers into one commitSelection that routes by interactionActive
    status: pending
  - id: lifecycle-resets
    content: Adjust interactionId/cancel/geometry effects so cancel preserves rendererSelection and only id-invalidating resets clear both
    status: pending
  - id: tests
    content: Extend existing test block to cover privacy of interaction selection, finalize with/without archive, start seeding, and display routing
    status: pending
  - id: ticket
    content: Open and close repo ticket 2026/05/26/REFACTOR-SPATIAL-SELECTION under runningsketchpad goal
    status: pending
isProject: false
---

## Goal

Replace the current two-layer selection in [spatial/js/renderer-r3f/index.tsx](spatial/js/renderer-r3f/index.tsx) with a clean model:

- The renderer **always** has a canonical selection: `rendererSelection: readonly SelectionTarget[]` (may be empty, never absent).
- Every interaction owns a **private** `interactionSelection: readonly SelectionTarget[]`, scoped to the running session.
- The interaction selection contributes back to `rendererSelection` **only** when the interaction archives an explicit result (`archiveContext.targets`). No implicit "if non-empty, replace" fallback.

This is opinionated, drops backwards compatibility, and removes the union-merge entirely.

## Concrete changes (single file: [spatial/js/renderer-r3f/index.tsx](spatial/js/renderer-r3f/index.tsx))

### 1. Rename / collapse state

- `selectedSelectionTargets` -> `rendererSelection`
- `interactionSelectionTargets` -> `interactionSelection`
- Same for setters, props, host-state callbacks, default-chrome keys, and the `Omit<...>` lists in `InteractionReplLayoutProps.spatialView`.

### 2. Remove the union-merge

- Delete `mergeReplSelectionLayers` (line 3170).
- Replace `displayedSelectionTargets` with:

```ts
const displayedSelectionTargets = interactionActive ? interactionSelection : rendererSelection;
```

This is the fix for the "buggy" behaviour: during an interaction the user sees only the interaction's own picks (the cutters they are choosing), never a stale union with the renderer's prior selection.

### 3. Simplify finalize

Replace `replFinalizeSelection` (line 3186) with:

```ts
function replFinalizeSelection(
  rendererSelection: readonly SelectionTarget[],
  result: InteractionSnapshot["lastResponse"],
): readonly SelectionTarget[] {
  const archived = interactionArchiveTargets(result);
  return archived.length > 0 ? archived : rendererSelection;
}
```

Drops the `interactionSelection.length > 0 -> replace` fallback. Only interactions that explicitly produce a result selection (via `archiveContext.targets`, e.g. future `SelectAll` / `InvertSelection`) contribute back; "consumer" interactions like Fillet/Trim leave `rendererSelection` untouched.

Call site (line 3563) becomes `setRendererSelection((prev) => replFinalizeSelection(prev, snapshot.lastResponse))`.

### 4. Seed interaction selection from renderer on start

In `startRuntime` (line 3591), after sending `replStartEvent(accepted)`, also seed the interaction's own layer so the user sees their carry-over picks:

```ts
const accepted = replSelectionAccepted(accept, rendererSelectionRef.current);
setInteractionSelection(accepted);
await rt.send(replStartEvent(accepted));
```

### 5. Single commit + dispatch path

Replace `commitGeneralSelectionState`, `commitInteractionSelectionState`, `commitSelectionState` with a single `commitSelection`:

```ts
const commitSelection = useCallback((next: readonly SelectionTarget[]) => {
  setSelectionMenu(null);
  setHoveredPickKey(null);
  if (interactionActive) setInteractionSelection([...next]);
  else setRendererSelection([...next]);
}, [interactionActive]);
```

Rewrite `dispatchSelectionTargets` (line 3714) and the snap-pick branch in `onSpatialInteractionEvent` (line 3801) to read `currentSelection = interactionActive ? interactionSelection : rendererSelection` from one place and commit through `commitSelection`. The runtime `selection.changed` send stays gated on `interactionActive`.

### 6. Lifecycle resets

- On `interactionId` change (line 3648): clear `interactionSelection` only. Do **not** touch `rendererSelection`.
- On `cancelActiveInteraction` (line 3547): clear `interactionSelection`. `rendererSelection` is preserved (cancel must not destroy the user's persistent selection).
- On geometry/`derivedRevision` change (line 3643): clear both (entity ids no longer valid).

### 7. Tests

Extend the existing test block in [spatial/js/renderer-r3f/index.tsx](spatial/js/renderer-r3f/index.tsx) (around the `replEscapeAction` tests near line 5660) and any sibling test covering `replFinalizeSelection` / `mergeReplSelectionLayers`:

- Renderer selection persists across a cancelled interaction.
- Interaction selection is private: picking face F during an active Fillet does not mutate `rendererSelection`.
- Finalize without `archiveContext.targets` leaves `rendererSelection` unchanged.
- Finalize with `archiveContext.targets` replaces `rendererSelection` with the archived targets.
- Start seeds `interactionSelection` with the accepted subset of `rendererSelection`.
- `displayedSelectionTargets` equals `rendererSelection` when idle and `interactionSelection` when an interaction is active (no union).

### 8. Ticket bookkeeping

Open a new ticket `2026/05/26/REFACTOR-SPATIAL-SELECTION` via the repo MCP (separate from the existing `FIX-SPATIAL-CAMERA-JUMP` ticket), associated to the `runningsketchpad` goal, and close it once the tests above pass.

## Out of scope

- Adding `SelectAll` / `InvertSelection` interactions themselves (the refactor only makes the contribution path clean; the interactions land in separate tickets).
- Touching `./elements`, `./semio`, `./coda`, or `./reuse`.
- Any change to `spatial/js/core/index.ts` selection types — the `SelectionTarget` / `SelectionSpec` API is already correct.
