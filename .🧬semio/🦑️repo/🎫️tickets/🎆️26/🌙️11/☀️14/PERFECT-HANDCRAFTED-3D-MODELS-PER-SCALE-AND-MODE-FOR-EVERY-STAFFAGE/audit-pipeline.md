# Audit: scale and view in the 3D pipeline

Read-only audit. No source file was modified, no code or tests were run, and no git command was used. This file is the only thing written.

## Roots used in this report

- S = `/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage` (paths below are relative to S unless absolute)
- M = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model` (kernel)
- T = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE`
- Background docs (ticket folder ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE, under `☀️10`): `design.md`, `staffage-volume-module.md`, `axonometric.md`, `react-site.md`, `model-viewer.md`.

## Summary of the answer

- `scale` (the denominator) enters `modelOf` in exactly three places: the solid filter `shownSolids` (maxDenominator), the glyph and arrow sizes (`mm` = scale/1000, also `longestMm`), and the alpha of the feature-edge materials (visible at scale <= 100). Tessellation, feature-edge angle, pose transforms, clip samples, rig, joints and the thermal solve do not depend on scale.
- `views` changes the variant list, the material bindings, the colour animation channels and which analyses run. It never changes solids, nodes, poses or the thermal/structural numbers.
- One GLB, one FBX and one model JSON each hold exactly one scale. Scale appears only in the file name. A GLB holds all seven views as variants; an FBX holds one view; a model JSON holds all seven.
- The interactive React viewer is the only path with a scale-dependent tessellation (`qualityFor`). The GLB, FBX, model and ZIP exporters, the budget oracle, and the axonometric depiction use other rules.
- The batch exporter deletes the files of other scales that share its `--out` directory. This is a data-loss path with no test.

---

## 1. `modelOf` (S/🔨️modules/🧊️volume/🟦️.ts, 283 lines)

### Options (`ModelOptions`, lines 48-59)

| Option | Default | Effect |
|---|---|---|
| `scale` (required, > 0, finite; line 75) | none | Filters solids (line 88), sizes glyphs (line 98-99, 177), sets edge alpha (line 183, 195, 198) |
| `at` | first named pose (lines 78-81) | Static state: glyph placements and colours of the analytic variants |
| `poses` | `"all"` named poses | Pose list (lines 82-83); a pose other than `at` gets glyph nodes hidden (line 225) |
| `clips` | `"all"` clips | Clip list (lines 84-85) |
| `views` | `VIEWS` (all seven) (line 76) | Variants, colour channels, which analyses run (lines 92-95, 116-118) |
| `environment` | preset of the asset | Thermal solve only, passed at line 116 |
| `thermalSpanK` | absent = `shared` | Only the thermal look (line 90, 107) |
| `thermalQuantity` | `surface-temperature` (line 107) | Thermal look only |
| `forceQuantity` | `axial` (line 108) | Forces look only |
| `theme` | `light` (line 77) | Analytic looks, glyph paint and analytic edge ink (lines 107-109, 196, 206) |

The header comment at line 46 says `thermalSpanK` is "the span ±K around the air temperature of its scale". The code does not use scale for thermal at all: `solveThermalAt(asset, query, options.environment, ...)` (line 116) and `thermalLook(..., { range, theme }, ...)` (line 107) receive no scale. The air temperature is the environment's. The comment is wrong or means "the request's scale" loosely.

### What changes per scale (exactly these, nothing else)

1. Solids (line 88): `shownSolids(volume, scale)` keeps solids whose `maxDenominator` is absent or >= scale (`🧊️solids/🟦️.ts` lines 21 and 52). Solid geometry is not changed: the same `solidOf` output (line 167) with the same tessellation settings.
2. Surfaces and materials (line 89): `surfacesOf(solids)` (`🎨️looks/🟦️.ts` lines 48-60) builds surfaces from the shown solids only. Surface ids are `surface-${n}` in order of first use (line 56), so a dropped early solid can renumber later surfaces. Base materials (line 168), variant bindings (lines 185-190) and colour channels (lines 243-247) therefore differ per scale.
3. Glyph size (line 98-99, 177): `mm = scale/1000` (line 87) multiplies every paper size: arrow head and shaft (`🪧️glyphs/🟦️.ts` lines 53-73), arc radius (`frames/🧷️frames` line 107), centre-of-mass sphere 1.3 mm (`glyphs` line 102), emitter cone 5 mm (`glyphs` line 108). `longestMm = longestArrowMm(heightM/mm)` (line 99) sets the longest force arrow in paper mm, which is also scale-dependent.
4. Feature-edge alpha (line 183): the base `edge` material is alpha 1 only if `scale <= 100`, else 0.
5. Per-view edge alpha (line 195, 198): visible when the view is a line view, thermal, forces or moments, or when `scale <= 100`. So at scale > 100, `dark-solid` and `light-solid` have invisible (alpha 0) edge materials but the edge geometry is still in the document.

### What does not change per scale

- Tessellation: the scale-free `qualityOf(asset)` is used by the exporters (see section 2). `modelOf` itself emits procedural solids, so tessellation is not applied here.
- Feature-edge angle 30 degrees (`🗿️model/🟦️.ts` line 17), applied after tessellation.
- Rig nodes, joints and rest transforms (`🦴️skeleton/🟦️.ts`, `skeletonOf` line 165).
- Pose transforms (lines 138-149, 220-227) for bones.
- Clip sampling and bone channels (lines 151-163, 229-260): `frameTimes(clip)` and the sampled bone locals do not depend on scale.
- Glyph node ids and count (line 173-180): the slots come from the analytic frames, which are physical, not scale-based. Only their sizes change.
- Thermal and structural numbers: `solveThermalAt` (line 116), `solveStructure` (line 117), `solveTransport` (line 118) take no scale.
- Variant ids and titles (line 212), provenance (line 267), `title`, `id`.
- Materials for `dark-solid`, `light-solid`, `dark-line`, `light-line`: fixed colours (`solidLook` / `lineLook`, lines 101-106) with fixed `dark`/`light` theme. The `theme` option does not change them.

### What changes per view (`views`)

- Variants: one per requested view, in request order (line 185-213). Default is all seven (`VIEWS`, `🔗️contracts` line 76).
- Analyses: `wantsThermal` runs `solveThermalAt` and, if the asset has emitters, `solveTransport` (lines 116, 118). `wantsStructure` (forces or moments) runs `solveStructure` (line 117). A `views: ["light-solid"]` request runs none of them.
- Thermal colouring: `thermalLook` (`🌡️thermal/🟦️.ts`, `thermalLook` lines 27-48) paints one material per region key (surface temperature centred on the air temperature, `shared` range -20..60 C or the span).
- Forces and moments glyphs: `forces` and `moments` looks (`memberLook`, lines 108-109) and glyph materials created only for the view that shows them (lines 200-211): arrow and mass glyphs for `forces`, arc glyphs for `moments`, emitter cones for `thermal`. Glyphs are hidden in every other variant (`glyphEdgeOf` bound to the edge id, line 210).
- Colour animation channels: only for requested analytic views (lines 240-258).
- Output size: `views` only, not `scale`, changes the number of variants.

Analysis cost note: the default `views` is all seven (line 76), and the React viewer does not pass `views` (see section 4), so the thermal and structural solves run for every model build even when only one view is displayed.

### Design-doc deviation

`design.md` line 72 describes `modelOf(asset, { view, scale, environment? })`. The code has no `view` option. `staffage-volume-module.md` line 14 records this: "no `view` option; base materials are the light-solid look, `views` filters variants".

---

## 2. Exporters (S/🔨️modules/📤️export)

Scale is one value per run in all paths. A GLB, an FBX and a model JSON each hold one scale. Tessellation is `qualityOf(asset)` (`🧊️volume/⏱️job/🟦️.ts` lines 43-46): chord = largest dimension / 800 clamped to 0.5 to 20 mm, 12 to 64 segments. No scale input.

| Exporter | File(s) | Scale(s) | Views | Source |
|---|---|---|---|---|
| GLB (`🧊️glb/🟦️.ts`) | one static pose | one (`scale` in `GlbFileOptions`, line 18) | all seven as `KHR_materials_variants` (default `views`, line 33 via `encodeGlb(..., variants: true)`) | `writeGlbDetailed` lines 24-37; `qualityOf` line 30 |
| FBX (`🎬️fbx/🟦️.ts`) | one clip | one | one (`views: [view]`, line 28; default `light-solid`, line 25) | `writeFbxDetailed` lines 23-34 |
| Model JSON (`🗿️model/🟦️.ts`) | one pose or one clip, `.pose.json` or `.clip.json` | one | all seven (no `views` passed, lines 41-54 via `singleOptions`) | `writeModel` lines 41-54; `singleOptions` lines 20-29 |
| ZIP bundle (`🗜️zip/📦️models/🟦️.ts`) | GLB and JSON per pose, FBX and JSON per clip | one | GLB and JSON all seven; FBX one view | `writeModelBundle` lines 22-50 |
| Batch (`📦️batch/🟦️.ts`) | the whole catalog, per asset a folder | one per run (`--scale`, default 50, line 316) | GLB, JSON all seven; FBX one view (`--view`, default `light-solid`, line 156) | `exportModels` lines 152-272 |
| CLI (`🎬️cli/🟦️.ts`) | one file per call | one | glb/model: all seven; fbx: one; model-zip: all seven (`--view` rejected, line 96) | `runModelExport` lines 206-220 |

### Naming

- Stem (`🗿️model/🟦️.ts` line 32): `<asset>-<pose|clip>-<view|all>-model-1to<scale>`.
- GLB: `<stem>.glb`, view `all`. FBX: `<stem>.fbx` with the chosen view. JSON: `<stem>.pose.json` or `<stem>.clip.json`, view `all` (line 35, 138-142 in batch).
- ZIP: `<asset>-all-model-1to<scale>.zip` (batch line 144; CLI line 220).
- Batch and bundle members are `glb/`, `fbx/`, `model/` (batch line 149).
- React downloads use a different convention (section 4): `<asset>_<what>_model_<view|all>_1-<denominator>.<ext>`, and ZIP is named `bundle` (`📤️downloads/🟦️.ts` line 119). So the same GLB has two names depending on the path.

### Does a GLB hold several scales?

No. `scale` is a single `ModelOptions` field (`🧊️volume/🟦️.ts` line 49). The GLB encoder writes one scene (`scene: 0`, `scenes: [ { name: document.id, ... } ]`, `🧊️glb` encoder line 273). Multiple scales would need a kernel change (section 5).

### Batch directory behaviour (defect, code read, not executed)

- The output folder is not keyed by scale. `DEFAULT_BATCH_OUT` is one folder (line 89), and a file name carries the scale only inside its name (`1to<scale>`).
- Line 262: `for (const stale of await readdir(folder)) if (!produced.has(stale)) await rm(...)`. `produced` holds only files of the current run, so a file of another scale in the same asset folder is deleted.
- Line 171: `previous` is `undefined` when `settings.scale` differs (`sameSettings`, line 129). So the manifest is rebuilt from scratch and lists only the current scale. The earlier scale's files are gone from the disk and from the manifest.
- Effect: `batch --scale 50` followed by `batch --scale 100` into the same `--out` leaves only the 1:100 set. The default `--out` is shared by all scales.
- Test coverage: `🧊️model-batch/🟦️.ts` lines 86-98 (resume) changes to scale 100 in the same `out` and asserts only `skipped === 0` (line 98). It never asserts that the 1:50 files survive, so the deletion is not caught.

---

## 3. Axonometric depiction (S/🔨️modules/🎨️depiction/🧊️axonometric/🟦️.ts)

Scale enters in these places (all in `🟦️.ts` of that folder unless noted):

- Denominator `options.scale.denominator` (lines 106-107), `mm = denominator/1000`.
- Solids and surfaces at that scale: `surfacesAt(asset, denominator)` (line 109) and `poseSoup(..., denominator)` (line 134), both via `🧱️mesh/🟦️.ts` lines 56-121. Solids are filtered by `maxDenominator` (`shownSolids`, `mesh` line 64).
- Tessellation quality: `qualityAt(denominator)` (`🧱️mesh/🟦️.ts` line 38): chord = 0.04 paper mm times denominator, clamped to 0.5 mm..50 mm; 12 to 64 segments. This is a different rule from the exporters (section 2) and from the React viewer (section 4).
- Pens and line widths (line 136-137): `pens = source.pens` from the authored representation for the scale. A role whose pen is null is not drawn (`drawn`, line 137). Edge stroke width = pen x multiplier (line 280).
- Edge multiplier (line 265): 1 for line views; 0.5 for solid views at scale <= 100, 0 for solid views above 100; 0.5 for the analytic views.
- Solid base style: `baseStyle(view, theme, denominator <= 100)` (line 227; `🧩️shapes/🟦️.ts` line 59), so `outlineMm` is set only at scale <= 100.
- Glyph pen: `GLYPH_PEN_MM = 0.18` (line 51), constant.
- Seam stroke: `SEAM_MM = 0.05` (line 50), constant; applied to solid views with flat fills (line 253-256).
- Labels: `labelBudget(denominator)` (`🧱️canvas/🟦️.ts` lines 105-106) changes with scale; scale bar and title use the scale in `🏷️annotations/🟦️.ts` lines 29-32 and 107.

### Thermal, forces and moments in 3D

- Each solid is posed by the bone frames of the evaluated state (`frameDataOf`, `frame` line 114-131) and tessellated; visibility is exact hidden-surface removal (`📐️visibility`), and faces are merged per solid and shading band (`🖇️stitch`).
- Colour: `thermalView` (line 210), `memberView` for forces and moments (lines 212-226) paint the visible solids with the same thermal ramp and member colours as the 2D views. The legends are the 2D legends (lines 211, 217, 221, 224).
- Glyphs (arrows, arcs, centre-of-mass, emitters): 3D solids placed by `poseSoup` (lines 233-246); their fill is their glyph paint and they are stroked on the `forces` layer (line 280).
- Shading bands: four bands (`BANDS_TOP = 3`, line 318) for solid views unless `shading === "flat"` (line 173).

The 3D depiction draws what the 2D views draw. It does not use the 3D `ModelDocument` (`modelOf`); it builds its own scene from the same looks (`🧊️volume/🎨️looks`, `thermalLook`, `memberLook`).

---

## 4. React target and site (S/🎯️targets/⚛️react, S/🌐️site)

### Where scale and view are switched

- Controls: `🎛️viewer/🟦️.tsx` line 263 (Segmented 1:50, 1:100, 1:200, 1:500 and a custom option) and line 264 (NumberField, 1 to 100000). The title reads `1:<scale>` (line 169).
- The model key: `modelKey(asset, request)` (`🔨️modules/🧊️spatial/🟦️.ts` lines 55-59) contains asset id, `scale`, pose or clip, environment, thermal quantity, force quantity, span (for span mode only) and theme. The view and the camera are not in it. Changing the scale or any analytic option builds a new model; changing the view or the camera does not.
- View switch: `#applyLook` (`🖥️stage/🟦️.ts` lines 493-496) calls `viewer.setVariant(variantOf(view))` (`variantOf` = the view id, `🧊️spatial/🟦️.ts` line 28). No rebuild.
- Camera presets: `CAMERA_PRESETS` (line 16), `qualityFor` is not involved.

### Model build (`🔨️modules/🧮️pipeline` and `🧊️spatial/🏭️prepare/🟦️.ts`)

`prepareModel` (lines 87-107):

1. `modelOf(asset, modelOptionsOf(asset, request))` (line 95). `modelOptionsOf` (lines 44-49) passes no `views`, so all seven are built.
2. `inPlace(...)` (line 95, lines 52-70): clip root translation X and Z are replaced by the rest position, so the figure stays at the origin. This is a viewer-only transform.
3. `tessellate(procedural, qualityFor(asset, request.scale))` (line 96). This is the only scale-dependent tessellation in the product.
4. `featureEdges` with the 30-degree angle (line 97), then `withoutGlyphEdges` (lines 73-76), which strips glyph outline lines.
5. Cache: `CAPACITY = 3` per asset (line 28). Switching scale back and forth beyond three can rebuild.

`qualityFor` (`🧊️spatial/🟦️.ts` lines 47-52): chord = 0.1 mm of paper at that scale, bounded between 1/800 and 1/60 of the largest dimension; segments 12 to 64 for scale <= 20, 8 to 64 for scale <= 100 (48 max) and 8 to 32 for larger scales.

### What the user sees when the scale changes

- Solids disappear or appear (maxDenominator), and the surface materials are recomputed.
- The tessellation changes (coarser at larger denominators within the bounds above).
- Glyphs change size in metres (paper size x scale/1000).
- Feature edges appear or disappear for the solid views (alpha, section 1).
- A "preparing" bar runs through four phases (model, tessellate, edges, scene; `PHASE_WEIGHTS` lines 19-20), then the scene is shown.
- The text description (`describeModel`, lines 62-71) states the new scale.
- Changing the view does none of this: the variant switches instantly.

### Exports from the React path

`🔨️modules/📤️downloads` and `🔬️detail/📤️export-panel` call `exportSpatial` (`🧊️spatial/📤️exports/🟦️.ts` lines 44-64):

- GLB: `writeGlb(asset, { ...analytic, pose })` (line 51). Tessellation is `qualityOf` through `writeGlbDetailed`, not `qualityFor`. So the downloaded GLB at 1:500 is tessellated exactly as at 1:50, while the viewer shows the scale-dependent mesh.
- FBX: `writeFbxDetailed` with the chosen view (line 55). No `inPlace`: the downloaded FBX keeps root motion, the viewer plays in place.
- Model JSON: `writeModel` (line 60). Procedural, no `inPlace`.
- Bundle: `writeModelBundle` with `view` (line 63).

So the viewer and the downloads differ in three ways: tessellation (scale-dependent in the viewer only), root motion (in place in the viewer only), and file naming (section 2).

### Site (S/🌐️site/🟦️.ts)

- No scale or view controls. It declares the 3D modules for lazy loading (`staffageModelPipeline`, lines 78-79, a regex over module ids) and hosts the React build (line 146).
- The site test `🌐️site/🧪️tests/🧊️model-split/🥒️.feature` checks that the 3D code (GLB variants extension, FBX header, stage style) is in lazy chunks, not in the entry chunk.

### React tests that pin scale and view

- `🎯️targets/⚛️react/🧪️tests/🧊️model-view/🟦️.tsx` lines 84-100: all seven views are variants of one model; view and camera never rebuild the model; scale, pose, clip and analytic settings do.
- Same file lines 117-121: `qualityFor` is coarser at larger scales (a pure function test, not an export test).
- Same file lines 220-230: the description carries the scale (`scale 1:50`).
- Same file line 333: preparation can be cancelled at scale 77.
- `🧊️model-downloads/🥒️.feature`: GLB named `asset_pose_model_all_1-100.glb`, FBX named `asset_clip_model_thermal_1-100.fbx`.
- `🧊️model-viewer-page/🥒️.feature`: a viewpoint change keeps pose, time and scale.
- `📏️range-span/🥒️.feature`: the span travels in the address and reaches the cast exporter.

---

## 5. Kernel M: what `semio.model/v1` can and cannot carry

### Schema (M/🧬️schema/🟦️.ts, twin of M/🧬️schema/🔣️.json)

- `ModelDocument` (lines 74-88): `nodes`, `solids`, `meshes`, `skins`, `materials`, `variants`, `poses`, `animations`. No level-of-detail field, no representation field.
- `Node` (line 29): `id, parent, translation, rotation, scale, joint?, mesh?, skin?, tags?`. No `visible` or `lod`.
- `Solid` (lines 33-49): `tags` (free labels) and `repeat`, `transform`, `weights`. No visibility or scale-level field. The volume module encodes `maxDenominator` as nothing: it is dropped at filter time (`🧊️solids` lines 4, 8).
- `Variant` (line 61): `{ id, title, bindings: { [materialId]: materialId } }`. Variants bind materials only. They cannot show or hide nodes or solids.
- `Pose` (line 63): node TRS overrides only.
- `Primitive` (line 53): carries `solids` provenance (`{id, first, count}`), so a viewer can tell which solid made a triangle. This is provenance, not visibility.
- `Quality` (line 90): one quality per tessellation; no per-scale set.

### GLB encoder (M/📤️export/🧊️glb/🟦️.ts)

- Emits: TRS nodes, mesh primitives, skins, `KHR_materials_unlit` (line 63), `KHR_materials_variants` (lines 246-259), `KHR_animation_pointer` (line 183, 269), and `extras` for joints and tags.
- One scene: `scene: 0`, `scenes: [ { name, nodes: roots } ]` (line 273).
- Not emitted: `MSFT_lod`, `KHR_node_visibility`, `KHR_lod`, multiple scenes, node visibility. A grep of M and S for these names found only `KHR_materials_variants` and documents that mention it; no encoder or viewer uses the LOD or visibility extensions.
- What the GLB could carry per scale: several scenes (`scenes[]`, `scene` chosen by the reader), or several child nodes per scale with `KHR_node_visibility` (not in extensionsUsed today), or `MSFT_lod` with multiple meshes. Only the variants mechanism is in use. Whether three's `GLTFLoader` (0.182 in the tickets) reads `KHR_node_visibility` or `MSFT_lod` was not checked.

### FBX encoder (M/📤️export/🎬️fbx/🟦️.ts)

- One take per file (`🎬️fbx` `FbxOptions` line 31: `animation?`, `variant?`, `format`). `fbx-encoder.md` line 51: "Takes carries one take; events, clip `loop` and variant titles are not representable".
- Variant: resolved at encode time (`resolveMaterial`, line 195): the FBX bakes one look. The FBX cannot carry several views or several scales without a kernel change.
- Material colour animation for the exported variant only; others are skipped with a notice (`fbx-encoder.md` line 41).

### Viewer (M/🖥️viewer)

- `Scene.variants: Map<Slug, Int32Array>` (`🧬️contract/🟦️.ts` line 49; `🧱️scene/🟦️.ts` lines 174-181): a material index per variant. `setVariant` changes the material per primitive. There is no visibility layer and no LOD switch.

### Verdict for the kernel

Per-scale geometry cannot be expressed as a single `semio.model/v1` document today. One document is one scale and one geometry. Options for a multi-scale document, none implemented:
- a `levels` or `scales` array on `ModelDocument` (kernel change, schema change);
- node visibility in `Variant` (`bindings` extended to nodes);
- `Solid.tags` with `scale-<n>` labels plus a viewer rule (no consumer exists).

---

## 6. Tests and fixtures that pin scale or view in 3D

Staffage (S/🧪️tests and S/🧫️fixtures). Test bodies were read for the lines cited; not every assertion in every file.

- `🧊️model-exports/🟦️.ts`: L64 identical bytes; L83 model documents valid, procedural or resolved; L101-109 solids filtered by scale (nose and buttons present at 50, absent at 100); L110 progress and cancellation; L126 bundle members; L142 refusals; L155 CLI parse; L184 file stems (`1to<scale>`); L208 catalog format judgement.
- `🧊️model-batch/🟦️.ts`: L36 layout and manifest matching files; L59 pose and clip with the same id (herring-gull landing) written apart; L78 identical bytes and manifest in two runs; L86-98 resume: unchanged files kept, damaged file rewritten, `force` rewrites all, scale 100 into the same out gives `skipped === 0` (no check that the 1:50 files survive; see section 2); L101 producer digest; L123 cancellation; L136 malformed command line; L143 emit loops stop on abort.
- `🧊️model-looks/🟦️.ts`: L72 seven variants over one geometry, base look equal to `light-solid`; L89 thermal colours with default range and with span; L105 environment honoured; L127 animated colours equal the 2D colours at each instant; L150 force arrow length equals the 2D paper length; L175 moment arc radius; L191 glyphs hidden by default.
- `🧊️model-digests/🟦️.ts` with `🧫️fixtures/🧊️model-digests/🔣️.json`: GLB and FBX structure digests at scale 50 only (fixture `"scale": 50`). Covers every pose and clip; one scale.
- `🧊️model-catalog/🟦️.ts` (SCALE = 50, line 19) and its oracle `🧊️model-catalog/🔍️oracles/🟦️.ts`: `trianglesAt` (lines 268-274) uses `qualityOf` and `views: ["light-solid"]`; lines 307-315 check that triangles at 50, 100, 200, 500 never grow and that 1:500 is below 1:50 when solids carry `maxDenominator`; budgets: GLB 8 MB and 400 000 triangles at 1:50 (lines 31, 393, 315). The full export runs at 1:50 only.
- `🧊️model-bounds/🟦️.ts` L21 and L45: extent at rest and grounded standing pose. Scale-free.
- `🧊️model-silhouette/🟦️.ts` L142: volume fills its dimensions. Scale-free.
- `🧊️model-frame`, `🧊️model-skeleton`, `🧊️model-glyph-edges`, `🧊️volume-schema`, `🧊️model-split` (site): rig, frames, clip sampling, glyph edges follow variant, schema. Mostly scale-free. `🧊️model-skeleton` L149 replays the FBX through three's loader.
- `🧊️axonometric-depiction/🟦️.ts` (and `🥒️.feature`): L53 finite geometry per view and scale; L69 determinism; L89-111 flat shading equals planar item, legends; L123 colours equal the model's looks; L141 banded shading; L162-186 pens per scale, multiplier per view; L187 `maxDenominator` filter at 1:50 and 1:100; L198 chord 0.04 mm (`qualityAt(50).chordM` = 0.002); L205 vertical to scale; L218 ground outline; L247 posed vertices equal the kernel pose; L316-343 progress and cancellation.
- `🧊️axonometric-visibility/🟦️.ts`: hidden-surface and hidden-line removal, pixel agreement with the kernel rasteriser (L221-227). Scale-free.
- `🧊️depth-order`, `🧊️catalog-silhouette`, `🧊️solid-placements`, `🧊️thermal-scenarios`, `🧊️axial-twist`: solids and thermal; not read in detail.
- `🌐️thermal-multiview/🟦️.ts`, `🔭️multiview-coverage/🟦️.ts` (L70-552), `🎨️seven-modes/🟦️.ts`: 2D views across elevation, front and plan, and seven modes. `🔭️multiview-coverage` L394 and L419 cover pen-profile and view switching; L445 deterministic exports. These are 2D, not the 3D model.
- `📑️multiview-sheet`, `📐️paper-dimensions`, `📏️print-size`: 2D.
- Fixtures: `🧫️fixtures/🧊️axonometric-visibility`, `🧊️axonometric-depiction`, `🧊️axonometric-projection`, `🧊️model-digests` (scale 50), `🧊️model-frame`, `🧊️model-skeleton`, `🧊️volume-schema`, `📐️model-scene` (scale 50), `📑️sheet-model-scene`, `🧊️cast-lod` (2D cast levels).

React (S/🎯️targets/⚛️react/🧪️tests):
- `🧊️model-view/🟦️.tsx`: L84-100 seven variants, rebuild rule; L117-121 `qualityFor` monotone; L220-230 description carries scale; L333 cancellation at scale 77; L362 custom element attributes.
- `🧊️model-downloads/🥒️.feature`: GLB Khronos validator and seven variants; FBX loader; names with `1-100`; ZIP.
- `🧊️model-viewer-page/🥒️.feature`: address and viewpoint; scale kept.
- `🧊️model-viewer-page`, `📏️range-span`, `📤️downloads/🥒️.feature`: 2D exports.

Kernel (M/🧪️tests, M/📤️export/*/🧪️tests, M/🧫️fixtures):
- `M/📤️export/🧊️glb/🧪️tests/🧊️vectors/🟦️.ts`: L78 declared extensions only when used; L103 pose and clip time baked into node transforms; L124 skinned mesh hoisted to scene roots.
- `M/📤️export/🧊️glb/🧪️tests/🧪️oracles/🟦️.ts`: Khronos validator oracle; `M/📤️export/🧊️glb/🧫️fixtures/*`: rig fixtures.
- `M/📤️export/🎬️fbx/🧪️tests/*`: structure, rejections, second reader, three round trip, deflate.
- `M/🖥️viewer/🧪️tests/*`: scene, raster, picking, controller, frame, camera, follow.
- Not read in detail: test bodies inside the kernel tests folders; the kernel schema conformance tests (`🧬️schema-conformance`, S).

---

## 7. Gaps: where a 3D model is NOT different per scale or per mode

Scale gaps (`modelOf`, GLB, FBX, model JSON, ZIP, batch):
1. Tessellation of every exported file is the same at 1:50 and 1:500 (`qualityOf`). Only the React viewer uses a scale-dependent rule (`qualityFor`).
2. The volume solids that are shown at every scale are tessellated identically. Only solids with `maxDenominator` disappear.
3. No LOD: no simplified mesh, no reduced segments, no fewer sub-parts per scale in any export.
4. Feature edges: same 30-degree angle and same geometry at all scales; only the alpha changes at scale 100 (base) and per view.
5. Pose transforms, rig, joints, skins, clip samples and bone channels are identical at every scale.
6. Thermal, structural and transport numbers are identical at every scale; `thermalSpanK` is absolute, not scale-dependent.
7. Authored representations are not consulted. `modelOf` never reads `asset.representations`, `validRange`, or `status`. A model is produced at any positive denominator, including scales where the 2D drawing is unavailable or derived.
8. Line weights and pens: the 3D document has no pen weights. The GLB draws feature edges with a fixed material (no width). The axonometric depiction uses pens, the GLB does not.
9. Glyph pens and seams: none in the 3D document.
10. Theme: static solid colours ignore `theme` (only analytic variants and glyph paints follow it).
11. Views in the GLB and JSON: all seven are always present. A GLB at one scale cannot be asked for one view.
12. FBX: one view per file, and the chosen view is baked; the viewer cannot switch to another view from the FBX.
13. Multi-scale documents do not exist: a GLB, FBX or model JSON holds one scale; no scale switch in any file format.
14. Surface ids renumber with the shown solids (`surface-n`, `🎨️looks` line 56); a material id for the same surface can differ across scales.
15. Clip root motion: the FBX and the model JSON keep root travel; the viewer plays in place (`inPlace`). Not a scale gap, but the viewer and the files disagree.
16. Batch: one scale per output folder in practice; other scales in the same folder are deleted (section 2).
17. Naming: two conventions for the same file (`1to100` in the library and batch; `1-100` in the React downloads). ZIP is `bundle` in React and `<asset>-all-model-1to<scale>` in the library.
18. Analysis cost: the default `views` is all seven, so all solves run for every model; the React viewer does not restrict views.

Mode (view) gaps:
19. The seven views share one geometry; no view changes solids, nodes or tessellation. Thermal, forces and moments change colours and glyphs only.
20. Forces and moments: arrow and arc sizes are in paper mm times scale; the same arrows exist at every scale; no per-scale glyph simplification.
21. Views in the React viewer switch with `setVariant`, so the 3D display of one view does not reuse the 2D depiction; the 2D and 3D colours agree by test (`🧊️model-looks`), not by sharing the 3D document in the 2D path.
22. In the 3D axonometric depiction, view-specific line weights come from the authored pens; the React 3D viewer draws no line weights.

Tessellation rules in the product (three different ones):
- Exporters and budget oracle: chord = largest/800, 0.5 to 20 mm; 12 to 64 segments (`qualityOf`).
- React viewer: chord = 0.1 mm of paper at the scale, bounded by largest/800 and largest/60; segments depend on scale (`qualityFor`).
- Axonometric depiction: chord = 0.04 mm of paper at the scale, 0.5 to 50 mm, 12 to 64 segments (`qualityAt`).

---

## What I did not check

- I did not run any code or test, and did not run any export. The batch deletion (section 2) and the React/export mismatch (section 4) come from reading the code; they are not reproduced.
- Test bodies: I read the lines cited, not every test in every file. Kernel test bodies (M/🧪️tests and encoder test folders) were listed, not read in full.
- Whether three's `GLTFLoader` (0.182) or the Khronos validator accepts `KHR_node_visibility` or `MSFT_lod`: not checked against node_modules.
- The React stage's camera reset on a scale change, and the cache behaviour under real use: not checked.
- The `🏷️revision` and `🧊️cast-lod` 2D cast export: read only the LOD module head and the JSON; the cast exporter itself was not traced.
- The thermal solver (`🌡️thermal`) and the transport solver (`💨️transport`): read only the call sites; their internals were not checked for hidden scale input.
- `🦥️axonometric-lazy`, `🧊️depth-order` and some `🧊️*` tests in S/🧪️tests: not read.
- Authored representation data (`asset.representations`) for a sample asset: not inspected, so the consequence of gap 7 for real assets is unverified.
- Visual output: no screenshots, no rendering, no GPU or WebGL check.
- Files outside S, M and T that the 3D pipeline may touch: not searched beyond the grep results cited.
