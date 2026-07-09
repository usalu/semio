---
name: Procedural 3D Slider and Hang Fix
overview: "Restore functional parity for the procedural 3D react renderer by fixing two concretely diagnosed root causes: sliders cannot be edited at all (Inspector control never renders, no in-canvas slider drag), and any BREP-boolean preview (e.g. Sphere Cut With Torus) re-runs full CSG evaluation synchronously on every UI event, causing the reported \"infinite hang\"."
todos:
  - id: fix-tree-defaultopen
    content: Fix ui/js/react/index.tsx Tree defaultOpen regression so leaf property controls always render
    status: completed
  - id: tree-regression-test
    content: Extend Tree tests to cover leaf property-control visibility through TreeDataItemView
    status: completed
  - id: dag-slider-overlay-state
    content: Add slider_overlay_state_json to dag crate + FlowHost + wasm export
    status: completed
  - id: flowsession-ts-iface
    content: Add sliderOverlayStateJson to FlowWasmSession TS interface in os-shell.tsx
    status: completed
  - id: graph-slider-overlays
    content: Add parseDagSliderOverlays + GraphSliderOverlays to node-graph-host.tsx wired to setSliderValue
    status: completed
  - id: slider-overlay-test
    content: Extend framework/renderer/react/index.test.ts for slider drag behavior
    status: completed
  - id: preview-cache
    content: Add fixture-signature-gated preview cache to procedural/plugin/rs/app_3d.rs (main + generate preview)
    status: completed
  - id: preview-cache-test
    content: Extend procedural-plugin tests to assert cache hit/miss behavior
    status: completed
  - id: kernel-extreme-range-test
    content: Add kernel_3d_brepkit test for radius=10 sphere cut torus at slider max
    status: completed
  - id: e2e-verify
    content: Re-verify in browser + run all affected test suites; open/close repo ticket
    status: in_progress
isProject: false
---

# Procedural 3D: Fix Slider Editing and BREP-Boolean Preview Hang

## Context

The current `procedural3d` app is a Rust WASM plugin (`procedural/plugin/rs/app_3d.rs`) rendered through the generic `framework/renderer/react` hosts (`node-graph-host.tsx`, `world-3d-host.tsx`, `ui-interpreter.tsx`) — the old per-app `procedural/3d/react` package was deleted during the post-premigration refactor in favor of this shared architecture. "Parity with premigration" is interpreted as **restoring the functional behavior** (sliders drive the 3D preview live, no hangs) through this current architecture, not resurrecting the old per-app React files, consistent with the repo's ongoing move away from bespoke per-app hosts.

I reproduced both bugs directly (booted `dev:procedural:3d` on :6018, used a live browser session) and traced each to a specific, verified root cause in the code — not a guess:

### Bug A — moving a slider does nothing (verified via live DOM inspection)

- The Inspector panel's "Value" field for an `inputSlider` widget renders a tree row (`procedural-play-inspector.value`) whose control content div (`data-slot="tree-property-content"`) is **completely empty** — confirmed via `innerHTML === ""` in the live app.
- Root cause, confirmed by `git blame`: [ui/js/react/index.tsx](ui/js/react/index.tsx) line ~11992 has:

```1990:1994:ui/js/react/index.tsx
  const hasControl = Boolean(item.control);
  const propertyLayout = hasControl;
  const hasNestedTreeItems = childItems.length > 0 || hasDynamicChildren || Boolean(item.emptyState) || branchCount > 0;
  const defaultOpen = hasControl ? false : getTreeItemDefaultOpen(item);
  const propertyExpandable = hasControl ? hasNestedTreeItems : isExpandable;
```

  Commit `263740dd69` ("Default All Panels to Hidden or Folded") flipped `defaultOpen` from `true` to `false` for control-bearing property rows. For a leaf property row with a control but **no nested tree items** (e.g. any single Inspector field: slider value, number, text, toggle…), `propertyExpandable` is `false`, so there is **no chevron and no click affordance to ever open it** (see [ui/js/react/index.tsx](ui/js/react/index.tsx) lines 11290-11381: the control only mounts inside `{open ? <TreeBranchContent>{children}</TreeBranchContent> : <div data-slot="tree-property-content" />}`). The control is permanently unreachable. This is a **global regression**, not procedural3d-specific — it silently breaks every single-field Inspector control app-wide.

- Separately, even if the Inspector worked, there is **no in-canvas slider drag** at all today. `node-graph-host.tsx` wires `GraphParamOverlays` (→ `session.setNeuronParams`) and `GraphStepperOverlays` (→ `session.setStepperFieldValue`) but has no `GraphSliderOverlays` counterpart. The Rust/WASM side is actually half-ported already: `flow/core/rs/lib.rs` has a working `FlowHost::set_slider_value` / `#[wasm_bindgen(js_name = setSliderValue)]` export, and `os-shell.tsx`'s `FlowWasmSession` TS interface already declares `setSliderValue(...)`, but nothing calls it. What's missing is the overlay-geometry accessor: `dag`/`flow_core` has `stepper_overlay_state_json()` (used by `GraphStepperOverlays`) but no `slider_overlay_state_json()` counterpart, even though the underlying geometry helpers (`slider_track_bounds`, `slider_track_center` in [mathematical/graph/port/directed/dag/rs/lib.rs](mathematical/graph/port/directed/dag/rs/lib.rs)) already exist and are covered by existing tests (`canvas_slider_hit_adjusts_value*`, `widget_slider_track_screen_point`).

### Bug B — "Sphere Cut With Torus" hangs (verified via code path tracing)

- `procedural/plugin/rs/app_3d.rs`'s `render()` for the preview body calls `evaluated_preview_payload()`, which **constructs a brand-new `FlowHost::from_fixture(fixture.clone())` and calls `host.evaluate()` from scratch on every single call**:

```462:465:procedural/plugin/rs/app_3d.rs
fn evaluated_preview_payload(fixture: &FlowFixture, runtime: &Procedural3dRuntime) -> (String, String) {
    let mut host = FlowHost::from_fixture(fixture.clone());
    let eval_json = host.evaluate().unwrap_or_default();
```

  This throws away `FlowHost`'s own signature-gated eval cache (`flow/core/rs/lib.rs` `evaluate_internal`, which only recomputes when `tree_signature(...)` changes) because a fresh `FlowHost` never has a cached signature. For the sphere/torus example, evaluation runs `brep.bool.cut`, which — because a torus surface is involved and the bounds overlap — takes the expensive **mesh-based boolean path** in `kernel/3d/brep/rs/lib.rs`'s `boolean_mesh_sync` (tessellate both solids → `mesh_boolean` → re-import). This full recompute is expensive and pays no attention to whether the fixture actually changed.
- `render()` is invoked for **every window body on every single command** via [os-shell.tsx](framework/renderer/react/os-shell.tsx) `refreshUi` (`nextSession.app.windowKinds.map((kind) => plugin.render(...))`, called unconditionally after any command's ops are applied). Crucially, `nodeGraphViewport` (flow-graph pan/zoom) is dispatched **unthrottled on every wheel event** (`node-graph-host.tsx` `onWheel` handler calls `dispatch(nodeGraphCommands.viewport, ...)` directly, no debounce), and it mutates `envelope.fixture.camera`, which is enough to trigger another `set_document_op` → `refreshUi` → full `render()` → full CSG recompute.
- Net effect: with Sphere Cut With Torus loaded, **any interaction** (scroll-zoom, pan, hover, selection) re-triggers the full sphere∩torus mesh-boolean computation synchronously on the single UI thread, with no caching and no debounce — a burst of pointer/wheel events queues an ever-growing backlog of expensive recomputations, which is exactly what a user would perceive as "infinitely hangs" (worse in a non-release/debug wasm build). This reproduces the report even though a single, isolated `render()` call for this fixture completes in well under a second in an optimized build — it's not one call that's slow, it's the *lack of caching combined with unthrottled re-triggering* that compounds.

## Fix Plan

### 1. Fix the global Tree "closed by default" regression

- [ui/js/react/index.tsx](ui/js/react/index.tsx): change `const defaultOpen = hasControl ? false : getTreeItemDefaultOpen(item);` so that leaf property rows with a control and **no nested tree items** always default open (there is no other way to reveal their control), while rows that *do* have nested children keep respecting the "default all folded" intent. Concretely: `const defaultOpen = hasControl ? !hasNestedTreeItems || getTreeItemDefaultOpen(item) : getTreeItemDefaultOpen(item);` (or equivalent — leaf-control rows must always render their control).
- Extend the existing Tree tests in `ui/js/react/index.tsx` (the `data-slot="tree-property-content"` / `data-slot="property-control"` test block) with a case asserting a leaf property-item's control is present in the rendered markup **without** any explicit `open`/click interaction, through the actual `TreeDataItemView` production path (not just the low-level `TreeItem` primitive with `defaultOpen` forced true), so this regression can't reappear silently.

### 2. Wire up real in-canvas slider dragging (parity feature, not just unblocking the Inspector)

- [mathematical/graph/port/directed/dag/rs/lib.rs](mathematical/graph/port/directed/dag/rs/lib.rs): add `slider_overlay_state_json(&self) -> Result<String, String>`, mirroring `stepper_overlay_state_json`, iterating `DagNodeKind::Slider` nodes and emitting `{widgetId, value, min, max, step, x, y, w, h}` rows using `slider_track_bounds`.
- [flow/core/rs/lib.rs](flow/core/rs/lib.rs): add `FlowHost::slider_overlay_state_json()` delegating to `self.dag.slider_overlay_state_json()`, plus a `#[wasm_bindgen(js_name = sliderOverlayStateJson)]` export, mirroring the existing `stepper_overlay_state_json` (line ~2825) and its wasm export (line ~3604).
- [framework/renderer/react/os-shell.tsx](framework/renderer/react/os-shell.tsx): add `sliderOverlayStateJson(): string;` to the `FlowWasmSession` interface (next to the already-declared-but-unused `setSliderValue`).
- [framework/renderer/react/components/node-graph-host.tsx](framework/renderer/react/components/node-graph-host.tsx): add `parseDagSliderOverlays` + `GraphSliderOverlays`, mirroring `parseDagStepperOverlays`/`GraphStepperOverlays`, rendering a draggable `<Slider>` (from `@semio-tech/ui-react`, already used elsewhere) positioned via the returned rects; wire `onSliderChange` to `session.setSliderValue(widgetId, value)` + `commitFixture()` + `paintOverlays()`, in both places `GraphStepperOverlays` is currently instantiated (there are two host surfaces in this file).
- Extend `framework/renderer/react/index.test.ts` (existing file) with a test that a slider drag dispatches the expected fixture update and triggers a `render()` refresh.

### 3. Cache the procedural3d preview payload so unrelated commands stop re-running BREP CSG

- [procedural/plugin/rs/app_3d.rs](procedural/plugin/rs/app_3d.rs): add a `cached_preview: Option<{ signature: u64, meshes_json: String, instances_json: String }>`-shaped field to `Procedural3dRuntime` (or a sibling struct), and a `fixture_signature(fixture: &FlowFixture) -> u64` helper hashing only `widgets` + `synapses` (excluding `camera`, mirroring `flow_core::tree_signature`'s spirit). Replace the unconditional `evaluated_preview_payload` call in `render()`'s `PROCEDURAL_3D_PLAY_BODY_PREVIEW` branch with a read of the cache, and refresh the cache (recompute only on signature mismatch) at the single `set_document_op` write path so every command that returns `set_document_op(&envelope)` opportunistically keeps the cache in sync at negligible cost (hash compare only) when nothing actually changed.
- Apply the same signature-gated caching to the Generate-mode preview (`generation_preview_payload` / `render_generate_preview`), which has the identical defect (recomputes at every render), for consistency with the main preview and `refresh_generation_preview`'s existing `preview_text` caching.
- Extend the existing `procedural-plugin` Rust tests (`app_3d.rs` test module) with a case asserting that a viewport-only command (`nodeGraphViewport`/`setCamera`) does **not** change the cached preview signature/recompute, while a `setFixture`/`patchFlowWidgets` change does.

### 4. Kernel robustness check across the slider's full interactive range

- Extend `kernel/3d/brep/rs/lib.rs`'s boolean test module with a case at the slider's extreme (`radius = 10.0`, matching the fixture's `max`, against the default torus `major=2.0, minor=0.5`) to confirm `boolean_mesh_sync` remains correct and bounded-time now that this range is actually reachable through the UI for the first time.

### 5. Verify end-to-end

- Re-run the live browser repro (`bun run dev:procedural:3d`, `SEMIO_RENDERER=react`): confirm the Inspector slider Value field is visible/editable and updates the 3D mesh; confirm the new in-canvas slider drag updates the mesh live; confirm scroll/zoom/pan on the flow graph while "Sphere Cut With Torus" is loaded no longer stalls the tab.
- Run `flow_core`, `kernel_3d_brepkit`, `procedural-plugin` Rust test suites, the `ui/js/react` and `framework/renderer/react` vitest suites.
- Open a ticket via the repo MCP (`ticket_open`) for this work, associate with the relevant goal, and close it with a summary of all files touched, per repo workflow rules.

## Files touched (all edits to existing files — no new files)

- `ui/js/react/index.tsx`
- `mathematical/graph/port/directed/dag/rs/lib.rs`
- `flow/core/rs/lib.rs`
- `framework/renderer/react/os-shell.tsx`
- `framework/renderer/react/components/node-graph-host.tsx`
- `framework/renderer/react/index.test.ts`
- `procedural/plugin/rs/app_3d.rs`
- `kernel/3d/brep/rs/lib.rs`
