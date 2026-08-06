# Infinite crate status

## Progress
- Kernel aliases (`os_store`/`os_dsl`/`os_spr`/`os_vcs`/`os_pack`) + `extern crate self as infinite` in glue.
- Terrain: path-mounted `framework_surface_terrain` from surface/terrain (avoids surface↔infinite cargo cycle); imports use `crate::framework_surface_terrain`.
- Error count: ~186 → ~75.

## Remaining blockers (~75)
- `ui_wgpu::wgpu::{GpuContext, draw_text, HitKind, widgets, ...}` — symbols not present in current UI crate (likely dissolved).
- Canvas `self::` region imports (vello types via wrong parent).
- Board `self::canvas` / mid-path `crate` errors.
- `semio_s_3d` missing `project_point` / `MirrorAxis` etc.

## Impact
Blocks layout/flow/space/sequence plugins that depend on `semio-framework-os-infinite`.

## Next
Restore or reexport missing UI wgpu host APIs, or cfg-out world/canvas render modules behind `render` until APIs land.
