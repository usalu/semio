# Verify Log — CAD E2E Concrete Forest Left

## 2026-07-09

### Root cause fixed
- `TypeError: ops is not iterable` — WASM `handleCommand` returns `CommandResult` with `operations[].diff.payload`, but `loadPluginModule` cast it to `string[]`.
- Fix: `patchOpsFromCommandResponse()` in `framework/core/js/index.ts` (+ wgpu `boot.ts`).

### Plugin bridge fixed
- Dev `createPluginApi` stubbed `tools` / `windowEngagements` / `windowMeasures` as empty.
- Extended WIT (`list-tools`, `window-engagements`, `window-measures`) and wired component + bridge.

### Automated
- `bun nx run @semio-tech/framework-renderer-react:test` — 15/15 pass (includes CommandResult patch op test).
- `cargo build -p cad-plugin --target wasm32-wasip2 --release` — OK via dev plugin build.

### Manual (http://127.0.0.1:6020/)
- Select **Hexagonal Cut Concrete Forest Left** — no console `ops is not iterable` errors.
- Document tree: Shape (1), Building (12), Energy (1), Structure Classic (11), Nodes.
- Four 3D panes render tessellated brep geometry (not placeholder GLB).
- Footer toolbar shows View pane toggles (tools API live).

## 2026-07-09 — CAD interaction + curve tube fixes

### Automated
- `cargo test -p kernel_3d_brepkit` — 14/14 pass (includes `sweep_wire_profile_produces_tube_mesh`).
- `cargo build -p cad-plugin --target wasm32-wasip2 --release` — OK.
- `cad/plugin/rs/geometry_import.rs` — `forest_structure_curve_wires_tessellate_as_tubes` (wasm compile validation; native `cargo test -p cad-plugin` blocked by wasm-only `plugin_exports!` macro).
- `bun nx run @semio-tech/framework-renderer-react:test` — extended `engagementPreviewJson` scene field test.

### Manual (http://127.0.0.1:6020/)
- Example combobox → **Hexagonal Cut Concrete Forest Left** loads; Structure Classic section shows 11 objects.
- Footer **Box** construct tool → engagement zone `Step: first_corner` (session active).
- Pointer gate fix: `handlePointerDown` dispatches `worldPointerDown` when `engagementSessionActive` before `event.target === hostRef` check (canvas child clicks unblocked).
- `engagement_preview_json` channel wired; `EngagementPreviewLayer` renders point/segment/box-preview items.
- Curve tessellation: `curve_mesh_from_wire` uses `regular_polygon_wire_sync` → `planar_face_from_wire_sync` → `sweep_sync` (face profile + wire path).

## 2026-07-09 — CAD interaction parity fixes (plan 9e8371d5)

### Implemented
- **Live preview:** `preview_display_items` generalized for all `is_two_point_height` / `is_base_height` interactions; slab + column unit tests in `interaction.rs`.
- **Reference overlay:** `cadFixtureVitePlugin` registered in OS dev vite config; `WorldReferenceLayer` wired with opacity, hover/select → `referenceHover` / `setReferenceSelection`; selection JSON exposes `hoveredId` + `referenceSelectedId`.
- **Viewport hover/pick:** `setHover` + `worldPick` handlers in CAD plugin; document tree `selected_ids` / `highlighted_ids` sync per-pane.
- **Primitive selection:** `selected_primitive_id` / `selected_primitive_kind` on runtime; primitive inspector panel.
- **Load:** `requestFileOpen` op handled in React `os-shell.tsx` (hidden file input → `importSpatialJson`).
- **Transformations:** Removed forest-example static bypass; live shape pane drives derive.
- **Inspector:** Rotation/orientation field + multi-selection mixed-value test.

### Automated
- `cargo build -p cad-plugin --target wasm32-wasip2 --release` — OK; jco transpile to `framework/product/os/dev/plugin-modules/cad`.
- `bun nx run @semio-tech/framework-renderer-react:test` — 27/27 pass.
- `cad/plugin/rs/lib.rs` tests added: `forest_transformation_uses_live_shape_pane`, `multi_selection_inspector_shows_mixed_values`, `world_pick_selects_visible_object_by_index`, `document_tree_reflects_viewport_selection` (native `cargo test -p cad-plugin` blocked by wasm-only `plugin_exports!`; wasm test binary compiles but cannot execute without wasmtime).

### Manual (pending dev server)
- Start `bun run dev:cad` on :6020, load **Hexagonal Cut Concrete Forest Left**.
- Confirm `/cad-fixture/concrete-forest-reference.png` serves (200) and reference plane visible in all four panes.
- Viewport hover tint + Document tree highlight; viewport click selects tree row (bidirectional).
- Construct tools (Box, Wall, Slab, Column, External Wall) show live preview at each step.
- Load toolbar opens file picker and re-imports spatial JSON.
- Transfer **From Geometry** after editing a Shape object reflects live edit in Energy pane.
