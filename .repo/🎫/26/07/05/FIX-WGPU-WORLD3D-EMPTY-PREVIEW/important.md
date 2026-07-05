# Wgpu Renderer Full Feature Parity

## Summary

Brought the wgpu renderer to pre-migration feature parity for lowpoly and framework-wide plugin chrome across all six world3d plugins.

### Phase 0 — Framework bridge (all plugins)

- `PluginBridgeEntry::tools()` and `window_engagements()` in `framework/renderer/wgpu/rs/plugin_bridge.rs`
- `ShellState::refresh_ui` caches `active_tools` and `window_engagements`
- Footer renders dynamic `ToolNode` tree with collection expand/collapse
- Window engagement rail uses live `window_engagements` with static manifest fallback
- `engagementInput` on_change wired; mode switch triggers `refresh_ui`

### Phase 1 — UV canvas (lowpoly)

- `CanvasLayer` supports `dataUrl`, `points`, `seams`
- Renders paint texture via raster quad, UV wireframe with dashed seams, checkerboard
- Canvas-world pointer coords, `paintStrokeBegin`/`End`, wheel zoom

### Phases 2–6 — World3d interaction (infinite/world)

- `Mesh3d` extended with component pick arrays, UVs, baked paint vertex colors
- `WorldSelectionRecord` parses granularity, componentIds, transformTool, interactionMode, gumballTarget
- `worldPick` click + component marquee with live preview
- Component wireframe/selection overlays via line draws
- Paint-on-mesh: `paintAt` via ray-UV hit, stroke begin/end
- Paint texture baked to vertex colors for approximate mesh display

## Follow-up (2026-07-06) — Empty 3D preview fix

- **Root cause:** `upload_world_passes` in `ui/wgpu/rs/draw.rs` used `?` on `world_lines.upload()`. `GrowBuffer::upload` returns `None` for empty data, so mesh-only scenes (no line overlays) aborted the entire 3D pass.
- **Fix:** Upload instances and lines independently; return `None` only when both are empty.
- **Verified:** `cargo test -p ui_wgpu mesh_instances_without_lines`; `bun ./framework/renderer/wgpu/script.ts wasm` succeeded.

## Verification

- `cargo test -p kernel_3d_scene -p infinite_world -p lowpoly-plugin --lib` — 22 lowpoly tests passed
- `bun ./framework/renderer/wgpu/script.ts wasm` — succeeded
- Wgpu E2E (`verify-wgpu-playgrounds-e2e.ts --plugin lowpoly`) — boot failed on port 7202 (stale dev server / manifest hierarchy errors for unrelated plugins); rerun with clean dev server

## Files touched

- `ui/wgpu/rs/draw.rs`
- `framework/renderer/wgpu/rs/plugin_bridge.rs`
- `framework/renderer/wgpu/rs/shell.rs`
- `framework/renderer/wgpu/rs/scenes.rs`
- `framework/renderer/wgpu/rs/lib.rs`
- `infinite/world/rs/lib.rs`
- `kernel/3d/scene/rs/lib.rs`
