# Lane V — World3d Modelling Viewport Primitives (2026-10-08)

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, plan `📓️brep-mesh-widget-set-2026-10-07.md` (AD7 consumes this), audit `📓️wsx-ui-viewport-2026-10-07.md`.
No ticket was opened, closed or reopened; no git state was modified.

## 1. What shipped

Four domain-neutral viewport primitives, schema-first, typed end to end, rendered by the React `World3dHost`, with the native wgpu parity noted in section 6:

1. **Annotations** — linear dimension, angle, point marker, leader label. Caller-supplied EN/DE text, screen-constant size, each annotation listed for screen readers.
2. **Scalar field** — per-vertex or per-triangle values, colour ramp id, range, legend title/unit/ticks. Rendered as a heatmap with an accessible on-screen legend.
3. **Pick granularity filter** — `shape | face | edge | vertex`; hover and selection reach only the chosen granularity.
4. **Section plane** — origin + normal, optional stencil cap.
5. **Sub-element highlight tokens** — hover/selected theme tone and width per granularity.

### Transport decision (the one place I interpreted the brief)

The brief said "no JSON-string blobs in new fields". The scene already has a typed-lane mechanism (`SceneLane.encoding = "json"`, precedent: `NodeGraphScene.nodes/edges/...`). I used it, so:

- `World3dScene` (Rust) gets three **typed** fields: `annotations: Option<World3dAnnotationLayer>`, `scalar_field: Option<World3dScalarField>`, `modelling_options: Option<World3dModellingOptions>`.
- `World3dScene` (TS) gets the same three **typed** fields (`annotations`, `scalarField`, `modellingOptions`), never strings.
- Each rides its own optional lane (`framework.scene.world3d.annotations | scalarField | modellingOptions`, `encoding: "json"`); the lane text is produced and parsed only by the typed codecs (`ToValue`/`FromValue` in Rust, `sceneFromLanes` + `parseWorld3d…` in TS). Lanes 22 → 25 (Rust enum, name/field/body-key/optional tables, TS `WORLD3D_SCENE_LANES`, `🚚️world3d-scene-lanes` fixture all updated together).
- `SceneDoc::encode_pack` now refuses (with `PackError::Unsupported`) a scene that still carries a typed modelling field, instead of silently dropping it; `split_lanes` is the only route (pack has no enum support, and a scalar field exceeds the 32 KiB spine anyway).

## 2. Schema additions (exact field names)

Schema: `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧬️schema/📏️world3d-modelling/🔣️.json` (JSON Schema 2020-12, `additionalProperties:false` everywhere). Fixtures: `🧫️fixtures/📏️world3d-modelling/🔣️.json`.

```
World3dText            { en: string, de: string }                      both non-blank; no default language
World3dTone            neutral|primary|secondary|tertiary|success|warning|danger|info   (theme tokens)

annotations lane  → World3dAnnotationLayer { title?: World3dText, items: Annotation[<=512] }   ids unique
  Annotation (discriminator "kind")
    dimension { kind, id, from:[x,y,z], to:[x,y,z], offset:[x,y,z], text, tone=neutral }    from != to
    angle     { kind, id, vertex, directionA, directionB, radiusPx=48 (16..256), text, tone }  directions non-zero
    marker    { kind, id, position, shape=dot|cross|ring, text, tone }
    leader    { kind, id, anchor, labelOffsetPx:[dx,dy], text, tone }

scalarField lane  → World3dScalarField {
    meshId, domain: "vertex"|"face", values: (number|null)[1..4_000_000],
    ramp: "viridis"|"inferno"|"coolwarm"|"grayscale",
    range: { min, max }  (min < max),
    legend: { title: World3dText, unit?: string, ticks=5 (2..9) } }
  vertex domain = one value per mesh vertex; face domain = one value per triangle; null = no data (#808080)

modellingOptions lane → World3dModellingOptions {
    pickFilter?: "shape"|"face"|"edge"|"vertex",
    section?: { origin:[x,y,z], normal:[x,y,z] (non-zero), cap?: { tone=neutral } },
    highlight?: { face?|edge?|vertex?: { hover?: tone, selected?: tone, widthPx?: 1..8 } } }
```

Semantics pinned by the fixtures (values produced with three.js, not our code):

- Ramps: equally spaced sRGB stops, linear per channel (`WORLD3D_COLOR_RAMPS`); `Color.lerp` oracle.
- Section: geometry on the side `normal` points to is removed; clip plane = `[-n̂, n̂·origin]` in three.js convention (`Plane` oracle).
- Pick targets: `shape → {mesh}`, `face → {face}`, … exactly one granularity.
- GLB sub-element ids (native renderer): custom per-vertex attributes `_FACE_ID` (read at the triangle's first corner) and `_VERTEX_ID`, `SCALAR UNSIGNED_INT` (`glbSubElementIds` row).

## 3. Builder API

Rust (`semio_framework_plugin` re-exports every type, next to `World3dScene`; implemented in `…/🎬️scene/📏️world3d-modelling/🦀️.rs`):

```rust
let scene = world3d_scene(camera_json, meshes_json, instances_json, selection_json, &sun)
    .with_annotations(World3dAnnotationLayer::new(vec![
        World3dAnnotation::dimension("w", from, to, offset, World3dText::new("Width 40 mm", "Breite 40 mm")).with_tone(World3dTone::Primary),
        World3dAnnotation::angle("a", vertex, dir_a, dir_b, text),
        World3dAnnotation::marker("m", position, text),
        World3dAnnotation::leader("l", anchor, [48.0, -32.0], text),
    ]).titled(World3dText::new("Measurements", "Messungen")))?
    .with_scalar_field(World3dScalarField::new("mesh:part", World3dScalarDomain::Vertex, values, World3dColorRamp::Viridis,
        World3dScalarRange { min: 0.0, max: 1.0 }, World3dText::new("Wall thickness", "Wandstaerke")).with_unit("mm").with_ticks(5))?
    .with_pick_filter(World3dPickGranularity::Face)
    .with_section(World3dSection::new([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]).capped(World3dTone::Secondary))?
    .with_highlight(World3dHighlight { face: Some(World3dSubElementStyle { hover: Some(World3dTone::Info), selected: None, width_px: Some(3.0) }), ..Default::default() })?;
// also: with_modelling_options(World3dModellingOptions)? ; every fallible builder validates and returns ValueError
```

Pure helpers (same names in TS, camelCase): `World3dColorRamp::{stops, sample}`, `World3dScalarField::{color_of, color_bytes, legend_ticks}`, `World3dPickGranularity::targets`, `World3dSection::clip_plane`, `World3dText::resolve(locale)`; lane codec `world3d_modelling_lane_text` / `world3d_modelling_from_lane_text`.

TypeScript (`@semio-tech/framework`, `…/🎬️scene/📏️world3d-modelling/🟦️.ts`):
`parseWorld3dAnnotationLayer`, `parseWorld3dScalarField`, `parseWorld3dModellingOptions` (total, normalised, `null` on any schema or semantic refusal), `world3dRampRgb/Hex`, `world3dScalarColorHex`, `world3dScalarFieldColorBytes`, `world3dScalarLegend`, `world3dPickTargets`, `world3dSectionClipPlane`, `resolveWorld3dText`, types `World3dAnnotation*`, `World3dScalarField`, `World3dSection`, `World3dModellingOptions`, …

React host (`🌐️World3dHost/📏️modelling/🟦️.tsx`): `projectWorld3dAnnotations`, `World3dAnnotationOverlay`, `World3dAnnotationProjector`, `World3dScalarLegendView`, `world3dMeshDataWithScalarField`, `world3dSelectionWithPickFilter`, `resolveWorld3dHighlightPaint` / `world3dSubElementPaint`, `World3dSectionClip`, `applyWorld3dSectionClipping`, `World3dSectionCapStencils`.

## 4. Behaviour in the React host

- **Annotations**: projected each camera/size change (`useFrame`, change-detected) into a store; an SVG (lines, arrowheads, arcs, markers) plus absolutely positioned labels, all sizes in CSS px (`WORLD3D_ANNOTATION_METRICS`), `aria-hidden`. A visually hidden `<ul role=list>` inside `role="region"` (name = layer title or "Annotations"/"Anmerkungen") names every annotation (`Dimension: Width 40 mm`, German `Bemaßung: Breite 40 mm`) even before the first projected frame. Tone colours come from theme tokens (`neutral` = foreground, others = palette token). Occlusion by the model is deliberately not tested (CAD dimensioning convention).
- **Heatmap**: the field's mesh record is swapped for a painted twin before visuals are built (vertex field → canonical linear RGBA `colors`; face field → constant colour attribute; sRGB→linear verified against three.js). The mesh keeps its colours under hover/selection with a halved emissive tint (`WORLD_HEATMAP_TINT_SHARE`). Legend: `role=group`, title (+unit), gradient (decorative), ordered tick list, "No data" entry, EN/DE with locale number format. A field that does not fit the mesh shows a `role=status` message instead of a wrong heatmap.
- **Pick filter**: `world3dSelectionWithPickFilter` rewrites the effective selection (targets, `selectionMode`, drops other-granularity components/hover) and sets `targets.exclusive`, so sub-element misses and GLB/box instances no longer fall back to whole-shape hover/pick. Pointer handlers in `WorldInstanceNode` honour `exclusive`.
- **Section**: `gl.localClippingEnabled`; clipping planes assigned to materials under instance roots only (gumball/grid untouched); cap = two stencil passes (back-face increment, front-face decrement) on every `userData.world3dSolid` mesh plus one stencilled cap plane in the cap tone. Default GL context now requests `stencil: true` (`WORLD_CANVAS_DEFAULT_GL`, transparent variant in the host).
- **Highlight tokens**: one paint table (`world3dSubElementPaint`); defaults unchanged (secondary hover, primary selection), overridable per granularity via theme tones and `widthPx`.
- Lane identity is stabilised by the lane content hash (`useStableWorld3dLane`) so an unrelated refresh never re-validates a multi-megabyte scalar field.
- Probe attributes on the host root: `data-world-pick-filter`, `data-world-section` (`open|capped`), `data-world-scalar-field` (`<meshId>:<applied|mismatch|missing|none>`), `data-world-annotation-count`.

## 5. Files changed

New:
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧬️schema/📏️world3d-modelling/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/📏️world3d-modelling/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📏️world3d-modelling/🦀️.rs` and `🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/📏️world3d-modelling/🟦️.test.ts` (TS) and `🧪️tests/🔬️world3d-modelling-unit/🦀️.rs` (Rust)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/📏️modelling/🟦️.tsx`
- `…/🌐️World3dHost/🧪️tests/📏️modelling/🟦️.tsx`

Edited:
- Scene contract: `…/🎬️scene/🎬️scenes/🦀️.rs` (typed fields, 3 lanes, `put` now returns whether the payload was accepted, `encode_pack` guard), `…/🎬️scene/🟦️.ts`, `…/🎬️scene/📦️packages/🦀️rust/🦀️.rs`, `…/📦️packages/🟦️typescript/📜️script.ts`, `…/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json`.
- Plugin re-exports: `…/🔌️plugin/🦀️.rs` (two `pub use` lists only).
- Exhaustive `World3dScene {..}` literals in tests (3 fields added): `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs`, `…/🗣️Interpreter/🧪️tests/🔬️wgpu-render-plan-validator/🦀️.rs`, `…/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`.
- React: `…/🌐️World3dHost/🟦️.tsx` (lanes, filter, heatmap, overlays, clip, paint, exclusivity), `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` (extra three exports: `Vector2, Matrix4, ThreePlane, BackSide, MeshBasicMaterial`, stencil constants), `…/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` (`stencil: true`).
- Runner/registration: `…/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` (`world3d-modelling-check`), `…/📋️project.json` (target), `…/🧪️tests/🎚️config/🟦️.ts` (suite), `.vscode/launch.json` (`⚖️world3d-modelling-check🧑‍🎨engine🎯️targets⚛️react🟦️`, group `4_gate`, order 900.05385).
- wgpu: `…/🧊️wgpu/🧊️renderer/🦀️.rs` (GLB id attributes), `…/♾️infinite/🌍️world/🦀️.rs` (pick filter, heatmap), `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` (re-exports), test `…/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs`.

## 6. wgpu status

Done:
- **Sub-element ids on the GLB path** (`🧊️renderer/🦀️.rs`, the zeroed `face_ids/vertex_ids` at the old line 2774). The streaming GLB schema parser now reads `_FACE_ID` / `_VERTEX_ID` (UNSIGNED_INT SCALAR, aligned with POSITION; any other shape is refused), the plan counts instances carrying them (a mixed bank publishes no table rather than a table with anonymous gaps), the `Mesh3dSchema` declares `face_ids = triangles` / `vertex_ids = vertices`, and two new materializer phases (`FaceIds`, `VertexIds`) write exact `u32` ids (no `f32` widening). Inline meshes already carried ids.
- **Pick filter** (`infinite/🌍️world`): `apply_world3d_pick_filter` runs after every selection sync; idempotent.
- **Heatmap colours** (`infinite/🌍️world`): `world3d_apply_scalar_field` paints vertex (linear RGBA) and face (constant colour attribute) fields into the inline mesh before publication; the bridge digest keys on the lane hash.

Remaining wgpu work, exact:
1. **Legend and annotation overlay**: nothing draws them natively. Needed: publish the legend as framework UI components (text/container) next to the surface from the plugin, or paint it in the world pass; the same for projected annotation SVG/labels (needs a screen-space line/arrow/arc primitive and text in the 3D pass) and the accessibility mirror entries (`♿️accessibility-mirror`) for the annotation list.
2. **Section plane + cap**: needs a clip-plane uniform in the mesh shader (`🎨️shaders`) using `World3dSection::clip_plane()` plus a stencil cap pass; `World3dState` has no section field yet.
3. **Highlight tokens**: `World3dSceneSelectionRecord`/`component_overlay_color` still use the fixed palette; map `modelling_options.highlight` tones through the theme the way `resolve_pick_style` does.
4. **React GLB sub-element ids**: `GlbInstanceMesh` ignores `_FACE_ID/_VERTEX_ID`; React gets ids only from inline `faceIds/vertexIds`.

## 7. Tests run (all executed by me)

| Suite | Command | Result |
|---|---|---|
| Scene crate, Rust | `cargo test -p semio-framework-ui-scene --all-features` (target `target-g3d-lane-v`) | 184 passed, 0 failed (14 new `world3d_modelling::tests`, existing lane-contract/value/pack tests green with 25 lanes) |
| Scene package, TS | `bun ./📜️script.ts test` in `…/🎬️scene/📦️packages/🟦️typescript` | 91 passed (11 new in `📏️world3d-modelling`, Ajv 2020 against the schema + three.js oracles) |
| React host | `bun ./📜️script.ts world3d-modelling-check` | 38 passed (annotation projection exact pixels, overlay a11y EN/DE, heatmap vs three.js sRGB→linear, legend, pick filter ×4, tokens, section clip/stencil, mounted host lanes) |
| React regression | vitest: `🖱️world3d-interaction`, `🌐️World3dHost` component/projection-pane/multi-touch, `world3d-inline-surface/glb-material/scene-shading/mesh-residency/instance-delta` | 83 passed |
| Interpreter lanes | vitest in-source `-t world-3d` | 12 passed (25-lane contract) |
| tsc | `tsc` over the modelling module, the scene module, the test and `World3dHost/🟦️.tsx` (full import graph) | 0 errors |
| wgpu GLB ids | `cargo test -p semio-framework-os-renderer-wgpu --lib glb_sub_element_id` | 1 passed (ids survive; no-attribute primitive publishes none; float id accessor refused) — run before the world edits below |

Known limits of verification:
- The world-crate additions (pick filter, heatmap) and the `ui_wgpu` re-exports were written after that wgpu run; their tests passed once a peer's transient `semio-framework-artifact-infinite-dag` break cleared (section 8).
- `bun ./📜️script.ts world3d-interaction-check` fails in its first Node oracle (`interactionSelectionSetOracle`) because a peer edit to `🔌️plugin/🦀️.rs` changed the `let locked…` line the oracle greps; unrelated to this lane, the mounted vitest half passes.
- The plugin crate (`cargo check -p semio-framework-plugin`) fails on five pre-existing `BoundedConfigRetirementFactory: FactoryRetirement` errors from peers; none mention world3d.
- No GPU run: stencil cap and clipping are verified structurally (materials, passes, planes) and against three.js plane maths, not visually.
- Rust `jsonschema` is not in the workspace, so Rust validates the same fixtures through its typed codec while the schema document itself is validated with Ajv in TS.
- The `.vscode/launch.json` row was inserted by hand next to `world3d-interaction-check`; the generator will produce the same row from the new nx target.

## 8. Final wgpu verification

- `cargo test -p semio-framework-os-infinite --lib -- scene_pick_filter scalar_field`: **4 passed** (pick filter admits exactly the fixture granularity and is idempotent, vertex field paints canonical linear RGBA matching the fixture hex values, face field becomes one constant colour per triangle and replaces authored colours, wrong count leaves the mesh untouched).
- `cargo check -p semio-framework-ui` (the `ui_wgpu` re-export list): clean.
- `cargo test -p semio-framework-os-renderer-wgpu --lib glb_sub_element_id`: **1 passed** on the renderer code as it stands in this lane. The renderer crate was edited only before that run; the later edits (world crate, `ui_wgpu` re-exports) are compiled and tested through the two crates above. A re-run after them is currently blocked by a peer's in-flight change in `🎚️config` (`UiPreferencesConfigMutation::{Theme,Layout,Driver,Appearance,Terminology,Locale}` no longer generated), which fails before the renderer crate is reached; re-run the command once that lands to confirm the combined build.
