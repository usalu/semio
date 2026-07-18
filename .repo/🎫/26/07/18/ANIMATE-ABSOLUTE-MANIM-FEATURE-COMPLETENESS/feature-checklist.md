# Animate Absolute Manim Feature Checklist

Code-truth tracker for Manim CE parity (Semio naming). `[x]` = implemented and tested; `[ ]` = open.

## Render blockers
- [ ] `point_ratio` partial path reveal in renderer
- [ ] Typst SVG → BezPath (not placeholder rect)
- [ ] Frame hash includes transform/opacity/path state

## Sobject / morph
- [ ] Pointwise morph between unlike paths
- [ ] Table/Matrix native 2D grid layout
- [ ] Z-order / foreground mobjects

## Catalog animations (21 stubs → real)
- [ ] DrawBorderThenFill, FadeTransform, ReplacementTransform, TransformFromCopy
- [ ] MoveToTarget, Restore, Flash, Circumscribe
- [ ] GrowFromPoint, ShrinkToCenter, SpinInFromNothing
- [ ] ChangeDecimalToValue, Broadcast, ApplyWave, Wiggle
- [ ] CyclicReplace, Swap, TransformMatchingShapes, Homotopy
- [ ] ShowPassingFlash, SpiralIn

## Scene runtime
- [ ] Introducer/remover lifecycle in `play`
- [ ] Section begin/end + skip_animations
- [ ] `AnimateBuilder::shift` real translation
- [ ] `AnimationGroup::with_lag_ratio` passes lag

## Mobject catalogs
- [ ] Geometry: Ellipse, RegularPolygon, DashedVSobject, boolean ops
- [ ] Text: DecimalNumber, Integer, Paragraph, Code
- [ ] Axes: tick labels, FunctionGraph, ParametricFunction
- [ ] Graph: labels, DiGraph arrowheads
- [ ] Media: ImageSobject, SvgSobject
- [ ] 3D: ThreeDVSobject as Sobject, solids
- [ ] Fields: ArrowVectorField, StreamLines

## Cameras / scenes
- [ ] MovingCameraScene, ZoomedScene, ThreeDScene, VectorScene

## Video
- [ ] CLI (quality, scene, cache flush, preview)
- [ ] Live wgpu preview window
- [ ] Subtitles SRT sidecar
- [ ] Cache LRU max_entries

## Present bridge
- [ ] scene_hash compile pipeline
- [ ] Real player.js (not stub)
- [ ] React embed scene_hash
- [ ] Plugin video export
