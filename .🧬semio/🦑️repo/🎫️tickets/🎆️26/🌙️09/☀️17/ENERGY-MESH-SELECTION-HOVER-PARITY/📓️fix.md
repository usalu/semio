# Energy model viewport — selection/hover paint parity with puzzle 3d

## Problem

The energy `Model` World3d window bakes per-vertex class colours (required for envelope swatches and simulation overlays). `PaintTexturedMesh` previously forced `color: white` and `emissive: 0` for every vertex-coloured mesh, so the host’s `MESH_STYLE_PAINT` selected/hovered tokens never applied. Energy compensated with its own hard-coded primary mix and white lighten — visually different from puzzle 3d / cad GLB meshes.

## Fix

1. **`World3dHost` / `PaintTexturedMesh`** — Keep vertex colours only for `neutral` and `disabled` styles. For `hovered`, `selected`, `highlighted`, etc., use the same solid fill + emissive as url-backed meshes.
2. **`energy` scene builder** — Stop baking selection/hover into vertex colours; publish interaction only via `selection_json` and instance `selected`/`hovered` flags (overlay colours unchanged).

## Tests

- `cargo test -p semio-s-artifact-energy-model scene::` — 17 passed
- `cargo test -p semio-s-artifact-energy-model selection_publishes` — passes

## Files

- `🧰️framework/.../🌐️World3dHost/🟦️.tsx`
- `✏️s/🔌️plugins/🔋️energy/.../🎬️scene/🦀️.rs` + scene/editor unit tests
