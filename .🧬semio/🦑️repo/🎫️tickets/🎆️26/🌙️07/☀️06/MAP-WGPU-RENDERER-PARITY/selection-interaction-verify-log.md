# Selection interaction fix verification

## Root cause

1. **Shell ate map pointer-down** — `gis_map_pointer_down` returns an empty command list for button 0 (marquee tracking only), so `AppRuntime::handle_pointer_button` fell through to `shell.handle_pointer_button`. Node graph avoids this because `node_graph_pointer_down` always returns interaction commands and returns early.
2. **Pointer-up/move gated on bounds only** — marquee completion and drag tracking stopped when the cursor left the map rect, so releases outside the surface never committed selection and drags could fail to activate `map_marquee_active`.
3. **Modifier parity** — `map_marquee_mode` used alt=subtractive / ctrl=invertive instead of ui-react `marqueeModeFromModifiers` (shift=additive, ctrl/meta=subtractive, shift+ctrl=invertive). Call site passed raw `modifiers.ctrl` instead of `ctrl_or_meta()`.

## Fixes

- Skip shell on map pointer-down (button 0/1) when pointer is on a gis map surface.
- Run map pointer-up before shell; commit when drag was active even if release is outside bounds.
- Continue map pointer-move while map drag is active even outside bounds.
- Align `map_marquee_mode` with ui-react; use `modifiers.ctrl_or_meta()` at call site.
- Add `gis_map_drag_active` helper.

## Automated evidence

```
cd gis/2d/plugin/rs && cargo test
# 17 passed; 0 failed

cd framework/renderer/wgpu/rs && cargo build
# Finished dev profile successfully

SEMIO_PLUGIN=gis2d bun ./📜️script.ts wasm
# trunk built wgpu renderer

SEMIO_PLUGIN=gis2d bun ./📜️script.ts serve
# listening at http://127.0.0.1:6140/
```

## Manual smoke

- Served wgpu gis2d playground at `http://127.0.0.1:6140/` with tile proxy on 6141.
- Map renders with tiles + pins (screenshot captured in session).
