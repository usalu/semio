# 🌀️ Generator (`s.procedural.procedural3d@1/*#editor`) — fixture / window / interactivity

Plugin: `✏️s/🔌️plugins/🌀️procedural`, artifact `🗿️artifacts/🧊️procedural3d`.
Default example: `hexagonal-mushroom-column`.

## 1. Editor and default windows

`✏️editor/🦀️component.rs` — `create_procedural3d_app` (:510-651) sets
`default_mode_id(edit::PROCEDURAL_3D_PLAY_MODE_EDIT)`. Layout
(`🎭️modes/✏️edit/🦀️component.rs:15`):

```rust
create_default_layout(&[flow::PROCEDURAL_3D_PLAY_WINDOW_MAIN.into(), preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.into()],
                      "row", Some(&[68.0, 32.0]), Some(&["Flow".into(), "Preview".into()]))
```

| window id | title | surface | source |
|---|---|---|---|
| `procedural-main` | Flow | `SurfaceKind::NodeGraph` | `🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️component.rs:19` |
| `procedural-preview` | Preview | `SurfaceKind::World3d` | `🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️component.rs:18` |

Second, non-default mode `generate` adds `procedural3d-generations` (Canvas2d),
`procedural3d-generate-form` (Canvas2d), `procedural3d-generate-preview` (World3d)
(`🎭️modes/🧬️generate/🦀️component.rs:24-30`).

## 2. `setActiveExample` — real

`✏️editor/🎮️commands/🎨️set-active-example/🦀️component.rs:33-44` has a genuine branch:
`is_procedural3d_example_id` includes `PROCEDURAL_EXAMPLE_HEX_COLUMN`
(`🧬️schema/🦀️component.rs:295-308`), resolved by `example_snapshot` (:311-324) to
`PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT`:

```rust
pub const PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT: &str =
    include_str!("../../../📚️examples/🎬️hexagonal-mushroom-column/🖼️assets/🗣️hexagonal-mushroom-column.dsl.semio");
```

That 27-line DSL is real: camera, layout, 6 synapses, 3 `input-slider` widgets (height/radius/sides),
3 `neuron` widgets (`brep.curve.polygon`, `math.vector`, `brep.solid.extrude`), one `output-preview`
(`column-preview`). It is also `default_snapshot()` (`🧬️schema/🦀️component.rs:286-288`) — the app's
hard default document.

## 3. Document → surface

- **Flow window** — `flow_window::render` (`🕸️flow/🦀️component.rs:52-83`) converts `document.fixture` via
  `fixture_to_workflow` into nodes/edges and builds a `NodeGraphScene`. Direct, synchronous, correct.
  Renders the 7-widget graph on load. ✅
- **Preview window (edit mode) — STRUCTURAL GAP.** Both `Procedural3dPlayApp::handle` (`✏️editor/🦀️component.rs:381`)
  and `::render` (:441) construct a **fresh, empty** `FlowEvalSession` on every call
  (`FlowEvalSessionState { eval_json: String::new(), .. }`, `🖥️host/🦀️component.rs:2283-2300`).
  `render()` never calls `session.tick()`/`host.evaluate_step()`; it only reads `session.eval_json()`,
  which is therefore always `""` (`👁️preview/🦀️component.rs:62-63`).
  `preview_payload_from_eval_with_session` short-circuits on empty input to `("[]", "[]")`
  (`✏️editor/🦀️component.rs:943-945`) → zero meshes, zero instances.
  Each `flowEvalTick` dispatch also builds its own throwaway session
  (`🎮️commands/🧮️flow-eval-tick/🦀️component.rs:14`) and `Procedural3dConfig` has no field to persist
  `eval_json` or the mesh cache (`✏️editor/🎚️config/🦀️component.rs:70-92`).
  The framework documents the intended pattern as a **`Mutex<FlowEvalSession>` reached once per dispatch**
  (`🧰️framework/…/🔌️plugin/🦀️component.rs:10489`) — this app's overrides don't use it.
  The window's own test only asserts `json.contains("world-3d")` (`👁️preview/🦀️component.rs:110`), never
  that mesh/instance JSON is non-empty, which is why this went unnoticed.
- **Generate-mode preview works**: `update-generation-values` (`🎮️commands/🧬️update-generation-values/🦀️component.rs:24`)
  synchronously calls `evaluate_generation_preview` and persists into
  `Procedural3dConfig.generation_preview_text` via `Procedural3dConfigMutation::SetGeneration`;
  `🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️component.rs:48-54` reads that field. ✅

## 4. Interactivity

**Real:** `setActiveExample`, `nodeGraphEdit` (setFixture/deleteSelection/connect), `deleteSelection`,
`removeWidget`, `moveMediaNode`, `addWidget`, `patchFlowWidgets`, `reorganize`,
`translateSelection`/`rotateSelection`/`scaleSelection` (`🎮️commands/🧭️translate-selection/🦀️component.rs:22-38`),
generation CRUD, `nodeGraphViewport`, `setLodMode`/`setShowMode`/`toggleSun`/`setSun*`/`setCamera`/
`setActiveUtility`/`setLocale`, `flowEvalTick` (subject to §3).

**Deliberate no-ops:** `worldPointerDown` (`🎮️commands/🗂️world-pointer-down/🦀️component.rs:14-16`) and
`graphPointerDown` (`🕸️graph-pointer-down/🦀️component.rs:14-16`) — selection/hover is delegated to the
framework's generic `graph` `InteractionDefinition` (`✏️editor/🦀️component.rs:619-636`,
`HierarchyProvider::Topology` via `interaction_topology` :397-423). Not bugs.

No `todo!()`/`unimplemented!()` anywhere in the plugin's non-test code.

## 5. Panels

- **document** (`📌️panels/📄️artifact/🦀️component.rs:38-41`) — real widget tree. ✅
- **catalogue** (`📌️panels/🛍️catalogue/🦀️component.rs:26-40`) — from
  `flow::flow_palette_catalogue_sections()`, wires `addWidget`. ✅
- **inspection** (`📌️panels/🔍️inspection/🦀️component.rs:25-60`) — real per-widget rendering, but always
  invoked with an empty selection: `inspection_panel::render(&document.fixture, &[], labels)`
  (`✏️editor/🦀️component.rs:454`). `render` carries no `InteractionView`, so it always shows
  "no selection".

## 6. Selection-dependent UI is uniformly dead

Same root cause across three places:
- inspection panel (above);
- `context_menu` hardcodes `let selected: Vec<String> = Vec::new();` (`✏️editor/🦀️component.rs:488`), so
  selection-dependent items never appear;
- `preview_selection_json` hardcodes `selected=false`/`hovered=false` for every instance
  (`✏️editor/🦀️component.rs:707-716, 942-995`), so highlight/gumball never shows.

This is the **same cross-cutting framework gap** seen in aggregator and aussuchen: `render()` and
`context_menu()` receive no `InteractionView`.

## 7. Verdict

Today: the Flow window shows real, editable graph content; the edit-mode Preview window very likely renders
an empty 3D scene despite a non-empty fixture. Switching to Generate mode is the only reliable way to see
geometry.

Fixes, ordered:
1. **Persist the eval session** — hold a `Mutex<FlowEvalSession>` (the documented framework pattern) instead
   of constructing a fresh one in `handle`/`render`, or persist `eval_json` on `Procedural3dConfig`.
2. Strengthen the preview window test to assert non-empty mesh/instance JSON, so this cannot regress.
3. Thread `InteractionView` into `render`/`context_menu` (cross-cutting, shared with aggregator/aussuchen/generator).
