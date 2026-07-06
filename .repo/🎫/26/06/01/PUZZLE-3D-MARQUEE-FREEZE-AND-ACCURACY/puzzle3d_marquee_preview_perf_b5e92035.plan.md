---
name: Puzzle3d Marquee Preview Perf
overview: Add a live highlight preview while dragging the puzzle 3D rectangle/lasso marquee and eliminate the post-release freeze by projecting candidate footprints once per gesture (camera is static during left-drag) and deferring the host shell rebuild.
todos:
  - id: ticket
    content: Open/reopen repo MCP ticket associated to the best matching goal (read repo://goals).
    status: completed
  - id: capture
    content: Add marqueeCandidatesRef + marqueeBaseSelectionRef and captureMarqueeCandidates(); switch projectObjectGroupToScreenPoints to non-precise Box3.
    status: completed
  - id: preview-commit
    content: Extract shared resolve helper; add previewMarqueeSelection (local store only) and cancelMarqueePreview; refactor commitMarqueeSelection to reuse cached candidates + base; expose on RegistryInteractionValue.
    status: completed
  - id: bridge
    content: Drive capture on activation, previewMarqueeSelection per move, and cancelMarqueePreview on aborts in MarqueeBridge.
    status: completed
  - id: defer-shell
    content: "Make noteSelection in puzzle/3d/play use notifySelection({ deferShell: true })."
    status: completed
  - id: verify
    content: Extend existing vitest blocks, run puzzle 3d react+play tests, runtime-verify live highlight + fast commit on Nakagin with temporary [DEBUG] logs, then remove and close ticket.
    status: completed
isProject: false
---

# Puzzle 3D Marquee: Live Preview + Commit Performance

## Root causes (verified)
- No live highlight: selection is only computed on `pointerup` in `commitMarqueeSelection` ([puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) line 6381). `MarqueeBridge.onPointerMove` (line 6097) only updates the overlay box.
- Freeze on release #1: `commitMarqueeSelection` projects every object via `projectObjectGroupToScreenPoints` (line 6402) which runs `new Box3().setFromObject(group, true)` — precise per-vertex traversal — for all 180 objects synchronously (line 5191).
- Freeze on release #2: host `noteSelection` → `notifySelection()` runs synchronous `emit()` shell rebuild, unlike Ctrl+A's `selectAllSelection` which uses `deferShell: true` ([puzzle/3d/play/index.ts](puzzle/3d/play/index.ts) lines 1087/1098 vs 1120).

## Key insight
During a left-drag marquee the camera never moves (`OrbitControls` `LEFT: null`, line 5456/5533), so every candidate's projected screen footprint is constant for the whole gesture. Project once at gesture start, then only run cheap point-in-rect/polygon tests per move.

## Changes

### 1. Cache candidate footprints once per gesture (puzzle/3d/react/index.tsx, RegistryProvider ~6381)
- Add refs: `marqueeCandidatesRef: MarqueeCandidate[]` and `marqueeBaseSelectionRef: SelectionSnapshot | null`.
- New `captureMarqueeCandidates()`: runs the existing candidate-building loop (objects + vortices + attractions, lines 6406-6445) once and stores it in `marqueeCandidatesRef`; snapshots the current selection into `marqueeBaseSelectionRef`.
- Change object projection to non-precise bounds: `new Box3().setFromObject(group, false)` in `projectObjectGroupToScreenPoints` (line 5191) so even the one-time capture is cheap (three caches `geometry.boundingBox`). Non-precise AABB is sufficient for marquee footprints.

### 2. Shared resolve + live preview (RegistryProvider)
- Extract a pure helper that, given `args` (rect/polygon/modifiers), the cached `candidates`, and the `base` selection, returns the merged `SelectionSnapshot` (reusing `marqueeSelectionFromCandidates` + `mergeSelectionSnapshot`, lines 6446-6456).
- `previewMarqueeSelection(args)`: resolves against `marqueeBaseSelectionRef` and writes to the local store only via `selectionStore.setSnapshot(...)` (NO host `onSelect`). The scene re-highlights through the existing per-id (`useObjectSelected`) and `BulkSelectionVisualBridge` paths (line 3808) — O(N) tint, no geometry work.
- `commitMarqueeSelection(args)`: resolves against the base, calls `publishSelection(...)` (host notify) + existing suppress-click; then clears the two refs.
- `cancelMarqueePreview()`: restores `selectionStore.setSnapshot(marqueeBaseSelectionRef)` and clears refs.
- Add `captureMarqueeCandidates`, `previewMarqueeSelection`, `cancelMarqueePreview` to `RegistryInteractionValue` (line 3645) and the `interactionValue` memo (line 6984).

### 3. Drive preview from the gesture (MarqueeBridge ~6065)
- On first activation (transition to `active`, line 6107-6114): call `captureMarqueeCandidates()` before the first preview.
- On each subsequent move: call `previewMarqueeSelection({ startX, startY, endX, endY, path, modifiers })` after updating the overlay store.
- `onPointerUp` (line 6124): keep `commitMarqueeSelection(...)`.
- `cancelGesture` (line 6086) and relocate-abort paths: also call `cancelMarqueePreview()` so an aborted drag restores the original selection.

### 4. Defer host shell on marquee/manual selection (puzzle/3d/play/index.ts)
- In `noteSelection` (line 1087) call `this.notifySelection({ deferShell: true })` so the single commit's inspector/document rebuild no longer blocks the release. `notifySnapshot()` still fires synchronously to update the viewport/probe.

## Validation
- Extend existing in-file vitest blocks only: in [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) (~8285) add cases that `previewMarqueeSelection` and `commitMarqueeSelection` resolve identically from the same cached candidates, and that additive/subtractive modes merge against the captured base. In [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts) (~1977) assert `noteSelection` still updates `selection`/`selectedLabel`.
- Run `bun nx run @semio-tech/puzzle-3d-react:test` and `@semio-tech/puzzle-3d-play:test`.
- Runtime: launch the 3D play dev server on the Nakagin fixture; with a temporary `[DEBUG] performance.now()` around capture/preview/commit, confirm (a) objects highlight live while dragging, (b) release is sub-100ms vs the current multi-second freeze; then remove the debug logs.

## Process
Work inside a repo MCP ticket (read `repo://goals`, reopen `26/05/31/PUZZLE-3D-MARQUEE-SELECTION` if it covers this, else open a new one). Edit existing files only, keep code in the existing `🔖Marquee`/`🎬Viewport` regions, no backwards-compat shims, close the ticket with a summary and touched files.