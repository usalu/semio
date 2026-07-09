---
name: Brush Kind Suggestion Weights
overview: Add a per-kind "suggestion percentage" with one window slider per node/object kind and per handle/vortex kind (two groups each summing to 1, proportional auto-adjust), and use those weights to weighted-randomly re-order the brush suggestion list in both puzzle 2D and 3D.
todos:
 - id: ticket
   content: Read repo://goals and open a new ticket for brush kind suggestion percentages
   status: completed
 - id: helper
   content: Add normalizeKindWeightGroup/uniformKindWeights/weightedOrder helpers to framework playground core
   status: completed
 - id: wasm-2d
   content: Add node/handle kind weight state + set_brush_kind_weights to puzzle 2D WASM; weighted candidate order + weighted target-handle pick
   status: completed
 - id: renderer-2d
   content: Add Puzzle2dRenderer.setBrushKindWeights and per-frame push to WASM
   status: completed
 - id: play-2d
   content: Add 2D weights state, per-kind sliders, weight commands with proportional renorm, setKindCatalogs
   status: completed
 - id: host-2d
   content: "Wire playground host: feed 2D catalogs to controller + forward setBrushKindWeights to renderer"
   status: completed
 - id: brush-3d
   content: Add 3D kind-weights ref + weightedOrderBrushCompatibleCandidates and use it in BrushSession
   status: completed
 - id: play-3d
   content: Add 3D weights state from fixture catalogs, per-kind sliders, weight commands with renorm
   status: completed
 - id: tests
   content: Extend existing 2D/3D/framework test files for weighting, ordering, and normalization
   status: completed
 - id: build
   content: Rebuild 2D WASM and run puzzle 2D/3D + framework-core test targets; close ticket
   status: completed
isProject: false
---

# Brush Kind Suggestion Weights (Puzzle 2D + 3D)

## Decisions (confirmed)

- Two independent slider groups per surface: node/object-kind weights sum to 1, and handle/vortex-kind weights sum to 1 (separately).
- Moving one slider renormalizes the rest of its group proportionally so the group total stays 1.
- When the brush is active, the suggestion candidate list is weighted-random re-ordered (top = most likely); Tab still cycles through all candidates.

## Concept mapping

- 2D suggestion candidate = a node kind, plus a chosen target handle on the placed node.
  - Node-kind weights → re-order the node-kind candidate list.
  - Handle-kind weights → bias which target handle (by `handleKind`) is selected for the preview.
- 3D suggestion candidate = `{ objectKindId, sourceVortexIndex }` (a vortex carries a `vortexKind`), so a single weighted ordering using `objectWeight[objectKindId] * vortexWeight[vortexKind]` covers both groups.

## Ticket

- The existing open ticket [.repo/.../BRUSH-ENGAGEMENT-NO-SUGGESTIONS/ticket.json](.repo/🎫/26/06/02/BRUSH-ENGAGEMENT-NO-SUGGESTIONS/ticket.json) is about hit-testing, a different concern. At execution start, read `repo://goals`, then open a NEW ticket (e.g. `Brush Kind Suggestion Percentages`) via `ticket_open`, associating it with the most appropriate goal (likely `🎯r2602🎯runningsketchpad`). Put any temp files under that ticket folder.

## Shared helper

Add a pure normalizer used by both play controllers in `@semio-tech/framework-playground-core` (extend the existing exports in [framework/core/index.ts](framework/core/index.ts), region `WindowMeasure` or a new `KindWeights` region):

- `normalizeKindWeightGroup(weights, changedId, newValue)`: clamp `newValue` to [0,1]; set `weights[changedId]=newValue`; scale remaining entries so they sum to `1-newValue` (if all remaining are 0, distribute `1-newValue` equally). Returns a fresh record.
- `uniformKindWeights(ids)`: equal weights summing to 1.
- `weightedOrder(ids, weightOf, rng)`: weighted sampling without replacement (full permutation; top = highest expected). Generic so both surfaces reuse the ordering logic.

## Puzzle 2D

1. WASM ([puzzle/2d/rs/lib.rs](puzzle/2d/rs/lib.rs)):
   - Add `brush_node_kind_weights: HashMap<String,f64>` and `brush_handle_kind_weights: HashMap<String,f64>` to the board host (near `brush_*` state ~2842) plus exported `set_brush_kind_weights(&mut self, json: &str)` (parse `{ nodeWeights, handleWeights }`, store, then `brush_rebuild_preview` if brush active) next to `set_brush_flush_distance` (~4262).
   - In `brush_enter_slot` (~4215) replace `brush_shuffle_candidates(...)` with a weighted ordering keyed by node-kind weight (default weight when missing = uniform), still seeded per `source_handle_id` so it stays stable while hovering a slot.
   - In `brush_pick_target_handle_index` (~3924) keep the compatibility filter but pick among compatible templates by weighted-random on `handle_kind` weight instead of strictly nearest.
2. Renderer ([puzzle/2d/react/index.tsx](puzzle/2d/react/index.tsx)): add `Puzzle2dRenderer.setBrushKindWeights(nodeWeights, handleWeights)` (mirror `setBrushFlushDistance` at ~3795 and the per-frame push at ~4673) calling `session.set_brush_kind_weights(JSON.stringify(...))`.
3. Play controller ([puzzle/2d/play/index.ts](puzzle/2d/play/index.ts)):
   - Add `nodeKindWeights`/`handleKindWeights` state and `setKindCatalogs(nodeKindIds, handleKindIds)` (init uniform for newly seen ids; drop unknown) mirroring `setBrushEngagementPossibles` (~573).
   - Extend `windowMeasuresForPane` (~687)/`brushMeasures` (~557) to emit one `slider` per node kind (group node) and one per handle kind (group handle), value = weight, min 0 max 1 step 0.01, label includes the kind label + percentage.
   - Add `run` cases `setNodeKindWeight`/`setHandleKindWeight` (~744 switch): apply `normalizeKindWeightGroup`, `rebuildShellMode`, then `hostBridge.runHostCommand("setBrushKindWeights", { nodeWeights, handleWeights })`.
4. Playground host ([framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx)):
   - Feed catalogs to the 2D controller: from `snap.fixture` merged catalogs, push node/handle kind ids to `puzzle2dShellController?.setKindCatalogs(...)` (near `notifyBrushCandidates` ~3363).
   - Add host-bridge case `setBrushKindWeights` (~4290 switch) forwarding to the renderer's `setBrushKindWeights`.

## Puzzle 3D

1. Brush ordering ([puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx)):
   - Add module-level `puzzle3dBrushKindWeightsRef = { current: { objectWeights, vortexWeights } }` (near `puzzle3dBrushEngagementSourceRef` ~5951).
   - Add `weightedOrderBrushCompatibleCandidates(candidates, weights, kindCatalogs)` beside `shuffleBrushCompatibleCandidates` (~3324); weight = `objectWeights[objectKindId] * vortexWeights[vortexKind]` (vortexKind from the candidate's template). Replace the `shuffleBrushCompatibleCandidates(compatible)` call in `BrushSession` enter (~6793) with the weighted order, reading the ref.
2. Play controller ([puzzle/3d/play/index.ts](puzzle/3d/play/index.ts)):
   - Derive object/vortex kind ids from `parseKindCatalogs(this.fixture.meta)`; add `objectKindWeights`/`vortexKindWeights` state initialized uniform, refreshed when fixture changes (`setFixture` ~786).
   - Extend `windowMeasures` (~926)/`brushMeasures` (~910) with one slider per object kind and per vortex kind.
   - Add `run` cases `setObjectKindWeight`/`setVortexKindWeight`: normalize group, update `puzzle3dBrushKindWeightsRef`, emit.

## Tests (extend existing files only)

- 2D Rust unit tests in [puzzle/2d/rs/lib.rs](puzzle/2d/rs/lib.rs) test module: weighted ordering favors high-weight node kinds; handle-weight target selection; setter parses JSON.
- 2D TS tests in [puzzle/2d/react/index.tsx](puzzle/2d/react/index.tsx) test region: `setBrushKindWeights` plumbing.
- 3D TS tests in [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) test region (next to the existing `shuffleBrushCompatibleCandidates` test ~10238): `weightedOrderBrushCompatibleCandidates` with injected RNG and combined object\*vortex weighting.
- Helper tests for `normalizeKindWeightGroup` (group always sums to 1) in the framework core test file.

## Build/verify

- Rebuild the 2D WASM (run the existing puzzle-2d wasm build nx target) so `set_brush_kind_weights` is available to JS.
- Run the puzzle 2D/3D and framework-core nx test targets. No new `launch.json` entries (sliders are UI, not executables; no new permanent scripts).
