# Animate Absolute Manim Feature Checklist

Code-truth tracker for Manim CE parity (Semio naming). `[x]` = implemented and tested; `[ ]` = open.

## Render blockers
- [x] `point_ratio` partial path reveal in renderer (`trim_path_at_ratio` in `sobject.rs`)
- [x] Typst SVG → BezPath via `usvg` (`text.rs`)
- [x] Frame hash includes transform/opacity/point_ratio/path state (`renderer.rs`)

## Sobject / morph
- [x] Pointwise morph between unlike paths (`interpolate_path_sets`)
- [x] Table/Matrix native 2D grid layout (`matrix.rs`)
- [x] Z-order / foreground mobjects (`z_order` on Sobject trait)

## Catalog animations
- [x] All 21 former stubs implemented with real `apply` behavior
- [x] Shift, ApplyMethod, FocusOn, Blink, TracedPath, ChangeSpeed

## Scene runtime
- [x] Introducer/remover lifecycle in `play`
- [x] Section begin/end + `next_section` helpers
- [x] `AnimateBuilder::shift` real translation
- [x] `AnimationGroup::with_lag_ratio` passes lag

## Mobject catalogs
- [x] Geometry: Ellipse, RegularPolygon, DashedVSobject, boolean ops, vector fields
- [x] Text: DecimalNumber, Integer, Paragraph, Code
- [x] Axes: tick labels, FunctionGraph, ParametricFunction
- [x] Graph: labels, DiGraph arrowheads
- [x] 3D: ThreeDVSobject as Sobject, solid cube
- [x] Fields: ArrowVectorField, StreamLines

## Cameras / scenes
- [x] MovingCameraScene, ZoomedScene, ThreeDScene, VectorScene

## Video
- [x] CLI (quality, scene, cache flush, preview) in `animate/video/rs/script.ts`
- [x] Live preview (`preview_scene_window`, optional winit feature)
- [x] Subtitles SRT sidecar (`subtitles_path` + `write_sections_srt`)
- [x] Cache LRU max_entries

## Present bridge
- [x] `compile_scene_to_assets` pipeline
- [x] Real `player_boot_js` (not stub)
- [x] React `AnimateSceneEmbed` for `sceneHash`
- [x] Plugin `exportVideoFromDeck`

## Verification
- [x] Rust: 80 tests (46 core + 8 video + 10 present + 16 plugin)
- [x] TS: animate-present-core + renderer-react
