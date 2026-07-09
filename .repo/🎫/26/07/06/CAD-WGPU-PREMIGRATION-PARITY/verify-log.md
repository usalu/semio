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
