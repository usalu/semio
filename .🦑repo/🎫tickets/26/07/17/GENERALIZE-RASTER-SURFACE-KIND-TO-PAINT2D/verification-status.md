# Verification status

## Completed
- `bun vitest run framework/core/js framework/renderer/react` → **200 passed (200)**, 3 test files. Full pass.
- Manual line-by-line diff review of every changed file (see `git diff --cached` on the paths below) confirms:
  - `SurfaceKind::Raster` (serde "raster") → `SurfaceKind::Paint2d` (serde "paint-2d") is symmetric across enum def, `as_str()`, and every match arm.
  - `RasterScene` → `Paint2dScene`, and the `UiComponentSceneNode.raster` field → `.paint_2d` (JSON `paint2d`, matching the existing `world_3d`→`world3d` / `canvas_2d`→`canvas2d` convention) is threaded through every constructor, accessor, and destructure site.
  - `build_raster_scene` → `build_paint_2d_scene`, `render_raster` → `render_paint_2d`, and the native-renderer document-parsing types (`RasterCameraFields`, `RasterTransformFields`, `RasterLayerJson`, `RasterDocSyncJson`, `RasterAssetJson`, `RasterFlatLayer`, `collect_raster_pixel_layers`, `raster_default_*`) are renamed to `Paint2d*`/`paint2d_*` consistently.
- Repo-wide grep for the old identifiers (`SurfaceKind::Raster\b`, `RasterScene\b`, `build_raster_scene\b`) outside `target/`, `.claude/worktrees/*`, and other tickets' `.repo` backups returns **zero matches**.
- Deliberately left unchanged (confirmed generic, unrelated to this app-specific rename): `RasterTexture`/`RasterTextureStore` (ui/wgpu GPU texture cache), `raster_key`/`push_raster_quad`/`PendingRasterUpload`/`pending_raster_uploads`/`raster_digest`/`queue_canvas_image_upload` (framework/renderer/wgpu generic pixel-upload infra used by many surfaces), `rasterize_svg` (icon atlas), and the `raster` app's own identity (crate name `raster-plugin`, WIT package `semio:raster`, `RASTER_PLAY_*` constants, `RasterSession`/`createRasterSession`/`RasterWasmSession` in os-shell.tsx — the app's own compiled WASM session type).

## NOT completed — cargo check pending due to environment contention
`cargo check -p ui_wgpu` (and by extension `-p semio-framework-plugin` / `--workspace`) could not complete in this session.

At the time of writing, `ps aux` showed **15+ concurrent `cargo check`/`cargo build`/`cargo test` processes** from other agent sessions all hitting the same workspace `target/` directory on a 10-core machine, most running since ~15:34–15:35 (4+ hours) with only 0.3–2s of accumulated CPU time each — i.e. severely CPU-starved, not stuck/deadlocked (rustc for `ui_wgpu` itself was observed actively running via `-Z threads=8` but advanced only ~0.03s of CPU over a 4-minute wait window). This matches this repo's documented "Concurrent Cargo Workspace Churn" pattern (expect 30–90+ minutes when many sessions build concurrently).

Background check process left running (not killed): `cargo check -p ui_wgpu 2>&1 | tail -150`, output file:
`/private/tmp/claude-501/-Users-ueli-Documents-semio/01e0d837-4db0-41d5-85a2-c5f85904cf24/tasks/bmzcswxur.output`

**Action needed before closing this ticket**: once machine load clears, re-run
```
cd /Users/ueli/Documents/semio && cargo check -p ui_wgpu && cargo check -p semio-framework-plugin && cargo check --workspace 2>&1 | tail -80
```
and close the ticket if clean, or fix and re-verify if not.
