# Procedural 3D Previews: Prior Art, Known Issues & Architecture Contracts

**Date**: 2026-08-29  
**Source**: Ticket & plan archive audit  
**Scope**: Procedural 3D preview pipeline, flow evaluation, hover/selection mechanisms

---

## 1. Core Architecture: Preview Pipeline

### 1.1 End-to-End Data Flow

The procedural 3D editor (`procedural3d` app) generates 3D preview geometry through a tessellation pipeline:

```
Flow Evaluation (FlowEvalSession::tick)
  ↓ Collect geometry handles from evaluated channels
  ↓ pending_preview_tessellate_handles (identify handles needing tessellation)
  ↓ Issue brep.tessellate effects (async to WASM extension)
  ↓ Cache tessellated mesh in session
  ↓ preview_payload_from_eval_with_session (build JSON mesh+instance arrays)
  ↓ Render World3d surface with meshes & instances
```

**Key Insight**: Preview geometry is extracted per-handle from flow channel eval outputs. Each handle maps to one mesh+instance pair. Multiple handles per widget are allowed (different geometry on different channels or array outputs).

### 1.2 Geometry Extraction Pipeline

**Function**: `preview_payload_from_eval_with_session` (procedural3d artifact core ~line 942-995)

1. **Widget Filter**: Only `Widget::Neuron { preview: true }` and `Widget::OutputPreview` widgets emit geometry (other widget types are skipped)
2. **Channel Scan**: For each preview widget, extract all geometry handles from its eval output channels via `geometry_handles_for_widget(eval, widget_id)` 
3. **Per-Handle Tessellation**: Call `mesh_data_for_preview_handle(handle, tolerance, session)` for each handle
4. **Instance Placement**: All instances placed at origin `[0.0, 0.0, 0.0]` — spatial arrangement encoded in geometry coordinates (via transform nodes)

**Channel Enumeration (line 804)**: Prefers `widget_eval["out"]`, falls back to `widget_eval["in"]` — distinguishes output vs input geometry channels per widget.

### 1.3 Tessellation Caching Strategy

**Current State (Post-June-09 fix)**:
- Tessellation happens in a **shared WASM kernel** (`@semio-tech/flow-module-brep`) 
- Geometry handles (e.g., `solid-*`) are **created in flow_core's linked brep kernel**, passed to tessellator
- Both the flow evaluation and tessellation use the same kernel instance, so handles are valid
- Session caches tessellated meshes via `session.preview_mesh_json(handle)` and `session.resolve_preview_tessellate()`
- On repeat evaluation (same fixture signature), evaluation uses CAS Merkle cache → unchanged branches re-use cached handles

**Root Cause (Fixed June 09)**: Previous bug was standalone `@semio-tech/flow-module-brep` used for tessellation while eval ran in a separate kernel instance. Handles created in one instance are invalid in the other.

---

## 2. Known Open Issues & Blockers

### 2.1 Preview Hang on Complex CSG (Fixed July 09)

**Problem**: Sphere∩Torus boolean mesh preview would hang on every interaction (scroll, pan, hover, select).

**Root Cause**: 
- `procedural/plugin/app_3d.rs`'s `render()` called `evaluated_preview_payload()`, which constructed a fresh `FlowHost::from_fixture(fixture.clone())` and re-evaluated from scratch on every call
- Fresh `FlowHost` never had cached eval signature → no CAS Merkle cache hit
- Sphere∩Torus uses expensive mesh-based boolean (`boolean_mesh_sync`) — full tessellation every time
- `nodeGraphViewport` (pan/zoom) dispatched unthrottled on wheel events, mutating `fixture.camera` → triggered re-render → full CSG recompute

**Fix (July 09 — "Procedural 3D Slider and Preview Hang Fix")**:
- Added fixture-signature-gated preview cache to `Procedural3dRuntime`
- Cache key: hash of `widgets` + `synapses` only (excluding camera)
- Refresh cache only at `set_document_op` write path
- Viewport-only commands skip recompute (camera changes don't invalidate cache)
- Generated preview mode gets same caching
- Verified with kernel robustness test at slider extreme (radius = 10.0)

### 2.2 Inspector Slider Control Regression (Fixed July 09)

**Problem**: Moving a slider in the Inspector panel did nothing — Value field control div was empty.

**Root Cause**: 
- Commit `263740dd69` ("Default All Panels to Hidden or Folded") set `defaultOpen: false` for property rows with controls
- Leaf property rows (single field, no children) have no expand chevron, so control remains unreachable
- Global regression affecting every single-field Inspector control app-wide

**Fix (July 09)**:
- Changed `Tree` component's `defaultOpen` logic: leaf property rows with controls must always default open (no other way to reveal them)
- Rows with nested children still respect "default all folded" intent
- Extended Tree tests to assert leaf controls render without explicit open interaction

### 2.3 In-Canvas Slider Drag Missing (Fixed July 09)

**Problem**: No way to drag sliders directly on the flow graph canvas.

**Root Cause**: 
- `node-graph-host.tsx` wired `GraphParamOverlays` and `GraphStepperOverlays` but had no `GraphSliderOverlays` counterpart
- Rust/WASM side half-ported: `FlowHost::set_slider_value` existed + wasm export, but nothing called it
- Missing accessor: `dag`/`flow_core` had `stepper_overlay_state_json()` but no `slider_overlay_state_json()` (even though underlying `slider_track_bounds`, `slider_track_center` helpers existed)

**Fix (July 09)**:
- Added `slider_overlay_state_json()` to `dag` + `flow_core` (mirrors stepper pattern)
- Added `sliderOverlayStateJson()` to `FlowWasmSession` TS interface
- Implemented `GraphSliderOverlays` in `node-graph-host.tsx`, wired to `session.setSliderValue(widgetId, value)` + commit
- Extended framework/renderer/react tests for slider drag behavior

### 2.4 Flow Graph Hover Resetting 3D Camera (Fixed July 23)

**Problem**: Hovering 3D preview mesh would snap flow graph camera back to default, losing pan/zoom state.

**Root Cause**: 
- Hovering updated ephemeral `hovered_node_id` and triggered flow re-render
- `FlowGraphCanvasHost::syncFlowSessionFromScene()` with `applyCamera: false` still called `loadFixtureJson`
- `loadFixtureJson` replaced the entire fixture **including camera**, snapping live pan/zoom back to document default
- `applyCamera: false` only skipped explicit `session.setCamera()` path, not the fixture load

**Fix (July 23)**:
- `FlowHost::apply_fixture` / `replace_fixture` now preserve live camera (same as undo/redo)
- Added `FlowSession.cameraJson()` for viewport reporting
- Procedural 2d/3d: graph camera lives in plugin runtime; `nodeGraphViewport` is pure view action (no VCS op)
- Flow host `emitInteractionState` now reports viewport after gestures

### 2.5 Document Read/Write Round-Trip (Deferred, Affects Demonstrator)

**Problem**: Only the binary pack-native load path (`loadAppDocumentPack`) works. JSON-text read/load methods are declared but wired to `undefined`:
- `readAppDocument` (load for persistence)
- `loadAppDocument` (history replay)

**Impact**: Demonstrator can only suspend **pristine** panes (unedited). Suspending an edited pane would lose work.

**Status**: Deferred pending demonstrator boot confirmation. Planned fix: expose pack-native document read on channel client, move `ShellHost` consumers to pack path, delete vestigial JSON-text pair.

---

## 3. Hover & Selection Architecture

### 3.1 Framework Ownership (Post-Aug-14 Migration)

**Status** (from FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM ticket, Aug 14):

- **Selection/hover are framework-owned**, not app-managed
- Procedural plugin declares an interaction domain `"graph"` with:
  - Three granularities: `node`, `edge`, `handle`
  - Transitive hover enabled (`HoverSpec { transitive: true }`)
  - Multiple+Single selection modes with Pick+Rectangle methods
  - Replace/Additive/Subtractive/Invertive/Range merge modes
  - Broadcast enabled

**Window Bindings**:
- 3D: Main flow window + Preview window + Generate preview window all bound to `"graph"` domain
- 2D: Main flow window + Preview window bound to `"graph"` domain

**Topology Implementation**: 
- Procedural 3D/2D both implement `interaction_topology()` walking widget/neuron hierarchy, synapses as edges
- Proper parent-linkage for transitive hover on cluster groups

**Retained Verbs Reading InteractionView**:
- `delete-selection`: reads `interaction.selection("graph").ids`
- `translate-selection`, `rotate-selection`, `scale-selection`: filter mesh selection by `interaction.selection("graph").ids`
- `node-graph-edit` (2D): same pattern

### 3.2 Preview Geometry Hover Limitation (Known Framework Gap)

**Current State**: Preview window geometry instances never reflect selected/hovered state:

```rust
// Line 960 in preview_payload_from_eval_with_session:
let selected = false;
let hovered = false;  // Hard-coded to false
```

**Root Cause**: `render()` entrypoint carries no `InteractionView` argument. Only `handle`/`copy_fragment`/`cut_operations` verbs received interaction view in framework migration (Aug 14 ticket notes).

**Impact**: 3D preview meshes render but don't highlight on hover/selection. Users can't visually confirm selection before operating on preview geometry.

**Planned**: Thread `InteractionView` into `render()` path in a future framework wave. Until then, selection highlights only appear in flow graph, not preview.

### 3.3 Hover Context: Graph vs Preview

**Graph (NodeGraph surface)**:
- Hover state managed by framework via interaction domain
- Transitive hover on cluster groups (child neurons highlighted when cluster hovered)
- Visual feedback in node-graph-host.tsx (not yet threaded through rendering)

**Preview (World3d surface)**:
- No hover highlighting yet (instances hardcoded `hovered: false`)
- Framework owns hover domain but instances don't consume it
- Future: thread interaction state into mesh rendering pipeline

---

## 4. Dirty-Aware Flow Recompute (Optimization Design)

### 4.1 Problem & Solution

**Problem**: Every UI gesture (drag, pan, select, reconnect) triggers full flow recompute + animation, even when tree structure unchanged.

**Solution** (from "dirty-aware-flow-recompute" plan):
1. **Predictive dirty-set**: Compute changed node + downstream in JS before async eval
2. **Selective animation**: Light up only changed nodes, not all
3. **Skip eval gate**: Skip evaluation entirely for presentational changes (move, select)
4. **Authoritative gate**: Rust tree-signature in `FlowHost::evaluate_internal` prevents stray re-evaluations

### 4.2 Implementation Details

**JS-Side** (`flow/react/index.tsx`):
- Export `flowTreeDirtyNeuronIds(prevFixture, currFixture)` — returns `{ ids: string[], structural: boolean }`
- Canonicalize widget signatures (ignore layout, camera, slider min/max/step)
- Diff incoming synapse edges
- BFS downstream reachable nodes
- If empty + not structural → no eval, just render + update `lastEvalFixtureRef`

**Rust-Side** (`flow/core/lib.rs`):
- Add `last_tree_signature: Option<u64>` to `FlowHost`
- `tree_signature(tree, seeds)` hashes sorted neurons/synapses + seed entries
- In `evaluate_internal`: compute signature first; if cached and outputs exist, return early

### 4.3 Verification

- JS tests: layout-only diff → empty, not structural; reconnect → target + downstream only
- Rust tests: `pointer_up_screen` on unchanged tree → no recompute; reconnect → recomputes

---

## 5. Flow Canvas Context Menu (Design Completed)

### 5.1 Architecture (from "flow-procedural-rich-context-menu" plan)

Right-clicking flow canvas opens context-aware menu (implemented, completed July):

```
right-click → FlowCanvas builds context
  ↓ Calls contextMenu(ctx) in play controller
  ↓ Controller builds ContextMenuItems + dispatch
  ↓ ContextMenuController displays + captures selection
  ↓ onSelect bumps commandRequest epoch
  ↓ FlowCanvas effect runs session operation
```

**Context**: `{ hoveredNodeId, selectedNodeIds, isImageWidget, isBackground, previewOffNodeIds, screen, world }`

**Menu Items** (shared):
- Add node (spotlight)
- Delete (count-aware, destructive)
- Toggle preview (checked)
- Replace image (only on image widgets)
- Select all
- Clear selection
- Reorganize

**Procedural-specific items**:
- Isolate in preview (set show_mode to selected node only)

### 5.2 Wiring

- `FlowCanvas`: render ContextMenuController, accept `contextMenu(ctx)` builder + `commandRequest` epoch channel
- `flow/play` + `procedural/play`: own menu definition + dispatch, export builders
- `ProceduralFlowEditor`: forward contextMenu/commandRequest/onPreviewOffChange to FlowCanvas
- `Playground renderer`: wire builders + dispatch in both flow + procedural surface hosts

---

## 6. Contract Rules for Preview & Hover Implementation

### 6.1 Binding Constraints

1. **Hover must be declared via `interaction_domain()`** on the panel builder — framework owns transitive resolution
   - Procedural: `interaction_domain("graph")` on artifact panel
   
2. **Window bindings are mandatory** — every window must declare `.window_kind_interactions(windowId, vec![InteractionRef::new("graph")])`
   - 3D: 3 windows (main + preview + generate)
   - 2D: 2 windows (main + preview)

3. **Topology walk is framework contract** — `interaction_topology()` must return correct parent-linkage for transitive hover
   - Walk widget/neuron hierarchy recursively
   - Synapses as edge granularity

### 6.2 Verb Implementation Rules

1. **Verbs must read `interaction.selection("graph").ids`** not app config fields
   - Delete-selection, translate/rotate/scale use this
   - Inspect verb does NOT receive InteractionView (current framework gap)

2. **Selection/hover **NOT** stored in Config or Presence** — both were purged in Aug 14 migration
   - Config retains only genuine display preferences (lod_mode, show_mode, camera)
   - Presence retains only shareable view state (camera, active_utility_id)

3. **Preview window cannot emit hover highlighting yet** — framework gap
   - `render()` carries no InteractionView
   - Instance hover field must be `false` until framework threads interaction through
   - Planned for future wave

### 6.3 Eval & Cache Rules

1. **Preview cache key must hash fixture widgets + synapses only**
   - Exclude camera (viewport changes are presentational, not computational)
   - Exclude other ephemeral state

2. **Session must cache tessellated meshes per geometry handle**
   - Retain live handles in `session.retain_preview_meshes(&live)`
   - Dispose unused handles automatically

3. **Dirty-set optimizations must preserve cache correctness**
   - Presentational changes skip eval but must not corrupt cached signatures
   - `lastEvalFixtureRef` reset on external fixture load

4. **Flow evaluation preserves live camera**
   - `apply_fixture` / `replace_fixture` keep existing camera
   - `loadFixtureJson` never replaces viewport

---

## 7. S Product Development Blockers (Aug 28-29 tickets)

### 7.1 Current Status: S End-to-End (Aug 29)

**Goal**: `bun ./📜️script.ts dev s` (launch `s-react` on port 6070) boots semio **s** host OS with all ~58 plugins interactive.

**Wave 0 Status** (Rust fleet compiles for wasm32-wasip2): ✅ FIXED (Aug 29)
- Root cause: stale `.await` in `ArtifactStore::detach_backbone` call after peer de-async
- Location: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:900`
- Fix: drop `.await`, discard Result like peer call sites

**Wave 1 Status** (Dev server boots :6070): In progress

### 7.2 Demonstrator End-to-End (Aug 28)

**Goal**: `♻️mit-bestand/🧺️demonstrator` (port 6029) works end-to-end with all 6 app panes.

**Wave 0 Result** (Ground truth): Demonstrator could not boot at all

**Root Cause**: Peer's uncommitted semantic-mutations refactor broke `semio-s-plugin-stdio` (3093 dirty files) and transitively broke demonstrator build.

**Current State**:
- Framework plugin wasm compilation fixed (Aug 28)
- Stdio + transitive plugins remain broken mid-refactor (not in scope for resolver)
- Dev server can come up on last-good plugin artifacts (Aug 26-28)
- Wave 0 verification proceeds against last-good builds

**Known Gap**: Document round-trip (read/load) blocks "version controlled" verdict and pane suspension for edited documents. Only pristine panes can suspend. Planned fix deferred pending boot confirmation.

---

## 8. Related Cursor Plans (Completed or In-Flight)

| Plan | Status | Purpose |
|------|--------|---------|
| flow_procedural_rich_context_menu | ✅ Completed | Right-click menu dispatch (July) |
| dirty-aware_flow_recompute | ✅ Completed | Selective recompute + animation (July) |
| procedural_preview_window | Planned | Preview window integration details |
| brep_suite_procedural | TBD | BRep kernel integration |
| procedural-3d-hardening | TBD | Robustness improvements |
| procedural_3d_eval_session | TBD | Session lifecycle management |
| procedural_feature_complete | TBD | Feature parity goals |

---

## 9. Summary: Known Open Issues

| Issue | Severity | Status | Fix ETA |
|-------|----------|--------|---------|
| Preview hang (CSG) | 🔴 Critical | ✅ Fixed (July 09) | Deployed |
| Inspector slider missing | 🟠 High | ✅ Fixed (July 09) | Deployed |
| No in-canvas slider drag | 🟠 High | ✅ Fixed (July 09) | Deployed |
| Flow camera reset on hover | 🟠 High | ✅ Fixed (July 23) | Deployed |
| Preview instance hover not reflected | 🟡 Medium | 🚧 Framework gap | Wave TBD |
| Document read/write round-trip | 🟡 Medium | ⏸️ Deferred | After demonstrator boots |
| Inspection verb lacks InteractionView | 🟡 Medium | ⏸️ Framework gap | Wave TBD |
| Stdio mutations refactor blocking S | 🔴 Critical | ⏸️ Peer-owned | Peer in-flight |

---

## 10. Files to Know

**Preview Pipeline Core**:
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/.../✏️editor/🦀️component.rs` (lines 779-995)
  - Geometry extraction, tessellation, payload building

**Flow Evaluation & Cache**:
- `flow/core/rs/lib.rs` (FlowHost, tree-signature, eval gate)
- `flow/react/index.tsx` (dirty-set predictor, evaluate() orchestration)

**Hover & Selection**:
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/{3d,2d}/🦀️component.rs` (InteractionDefinition, topology)
- `🧰️framework/🛍️products/💻️os/AGENTS.md` (plugin contract)

**Context Menu**:
- `flow/react/index.tsx` (FlowCanvas, contextMenu builder prop)
- `flow/play/index.ts`, `procedural/play/index.ts` (command dispatch)

---

**End of Report**
