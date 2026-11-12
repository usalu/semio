# Axonometric viewpoint: report

S = `🎓️teaching/🏛️architecture/🧍️staffage`. `depict(..., viewpoint: "axonometric")` returns a normal `DrawingScene` everywhere the planar views do.

## Public API
- `depict(state: EvaluatedState, options: DepictionOptions, analysis?: Analysis, control?: JobControl): DrawingScene`; `depictAsync(state, options, analysis?, control?): Promise<DrawingScene>` (yields to the event loop between steps; `control = { signal?, progress?(done, total, label) }`, progress in permille 0..1000, bilingual labels, an aborted signal throws its reason).
- Request the view: `defaultDepictionOptions(view, lang, denominator, "axonometric")`. Camera: `DepictionCommon.axonometric?: { kind: "isometric" } | { kind: "dimetric" } | { kind: "free"; azimuthDeg; elevationDeg }` (default isometric); face shading of solid views: `DepictionCommon.shading?: "banded" | "flat"`.
- Camera module `🎨️depiction/🔭️views`: `axonometricCamera(spec?)`, `projectAxonometric(camera, p)`, `project(viewpoint, p, camera?)`, `nearness(viewpoint, p, camera?)`, `lookingOf`, `projectionOf`, `AxonometricCamera`, constants `ISOMETRIC_*`, `DIMETRIC_*`. Planar `cameraOf(PlanarViewpoint)`.
- `modelScene(state, { viewpoint: "axonometric", analysis?, axonometric? }): ModelScene` (DXF/DWG: visible linework in the picture plane, metres).
- Sheets: `resolveSheet(asset, query, source, mode, options, projection, language, axonometric?: AxonometricSpec | false)`, `composeSheet(scenes, projection, language, axonometricScene?)`, `modelSheet(..., { axonometric })`, `resolveViewpoints(...)`; `SheetInfo.axonometric`.
- CLI: `--viewpoint axonometric [--camera isometric|dimetric|<az>,<el>]`; sheets `--views elevation,front,plan,axonometric`; file stems `...-axonometric[-dimetric|-azA-elE]-1toN`, `sheet-<projection>-axonometric[...]` (D14). Preview CLI: same flags (`--views` value list). Casts: viewpoint `axonometric` (default isometric camera; JSON schema, TS validator and Python consumer accept it).
- Metadata: `DrawingScene.metadata.projection?: { kind, azimuthDeg, elevationDeg, axisScale: { x, y, z } }` (also in SVG `staffage:scene`).

## Files
New: `🔨️modules/🎨️depiction/🧊️axonometric/{🟦️.ts, 🧱️mesh, 📐️visibility, 🧭️edges, 🖇️stitch, 🌗️shading}`, `🔨️modules/🧊️volume/🧷️frames/🟦️.ts` (frame/glyph/paint computation extracted from `modelOf`, shared by it and the depiction); tests `🧪️tests/🧊️axonometric-{projection,visibility,depiction,exports}` (each `🟦️.ts` + `🥒️.feature`), fixtures `🧫️fixtures/🧊️axonometric-{projection,visibility,depiction}/🔣️.json` (generator `T/axonometric-fixture.py` for the first).
Edited: `🔗️contracts` (AxonometricSpec/Kind/Projection, options, metadata; deleted `VolumeUnavailableError`, `VolumeFeature`, `isPlanarViewpoint`, `requirePlanarViewpoint`), `🎨️depiction/🟦️.ts` (`depict` is a generator driven by `drive`/`driveAsync`; `baseStyle`/`themeOf` moved to `🧩️shapes`; `modelScene`), `🔭️views`, `🧱️canvas` (`Ctx.camera`), `💨️transport-view` (camera aware), `🏷️annotations` (height dimension), `🧊️depth`, `🏗️structural-view` (`memberView` exported), `🏗️structure/📽️projection`, `🖨️preview`, `🔭️multiview`, `📤️export/{🎬️cli,📑️sheet,🖼️svg,🎭️cast}`, cast JSON schema, `🧊️volume/{🟦️.ts,🚫️refusal}` (camera refusals bilingual), `README.md`, Python consumer `⚡️energy/…/🧍️staffage/🐍️.py` (refusal removed), `🧪️tests/🧊️volume-schema` (old refusal test replaced).

## What is to scale
Orthographic; picture scaled by 1/cos(elevation), so the **vertical axis is always true** (`mm = m·1000/denominator`). Isometric (azimuth 45°, elevation 35.264°): ahead, up, left all 1. Dimetric (ISO 5456-3, azimuth 69.295°, elevation 19.471°): ahead 1, up 1, left ½. Free: up 1, horizontal axes `metadata.projection.axisScale`. Only lengths along world axes are to scale; diagonals, angles and areas are not. The scale bar and the height dimension (true height) follow the vertical axis.

## Method
Pose: evaluated bone frames -> kernel `tessellateSolid` (chord 0.04 paper mm, 12..64 segments, cached per volume and scale) -> rigid per bone, uniform linear blend for weighted solids as the kernel skins. Hidden surfaces: per front-facing triangle, convex clipping against every nearer overlapping triangle with the affine depth difference split along the plane intersection (interpenetration and cycles need no ordering; coplanar by index); glass never occludes. Fragments are merged per solid and shading band by edge cancellation (one path per solid/band, nonzero-safe, even-odd as the SVG writer fills). Lines: silhouette, crease (30°) and boundary edges plus solid/solid intersection curves, hidden-line removal by exact parameter intervals; pen = role pen of the representation (line views x1; solid views x½ at 1:100 and larger, none below; analytic views x½); a role with pen `null` is not drawn. Ground: outline rectangle at y = 0 (footprint + 0.3 m), hidden by the figure. Colours: `baseStyle`, `thermalView`, `memberView` (the planar styles, patterns, values, legends) on synthetic shapes per visible solid; glyph solids (arrows, arcs, centre of mass, emitter cones) from the shared `frameDataOf`/`glyphOf`/`glyphPaintOf`.
Performance: 1.3 s for 108k triangles (small-leaved-lime at 1:50); all 295 catalog volume/scale combinations depict without error (5 refusals are the existing "no suitable representation at 1:200"). The first call per asset and scale pays the existing `resolveDepiction` suitability check (up to 30 s for the largest trees, cached afterwards, not part of this work).

## Tests and results
Commands (from `S/📦️packages/🟦️typescript`): `repo tools exec --light vitest -- run --config ../../🧪️tests/🎚️config/🟦️.ts axonometric` ; `repo tools exec --light tsc -- -p tsconfig.json --noEmit --incremental false`.
- `🧊️axonometric-projection` 15/15 (hand vectors 1e-12, three `OrthographicCamera` 1e-9, planar cameras, kernel viewer isometric preset).
- `🧊️axonometric-visibility` 11/11 (hand areas + polygon-clipping 1e-9, three Raycaster owners 0 mismatches, cycle/interpenetration/coplanar, hidden-line intervals; sample figure vs the kernel viewer's `SoftwareBackend` for the same camera: at most 1 % of covered pixels differ).
- `🧊️axonometric-depiction` 64/64 (7 views x 4 scales, colours/patterns/values/legends vs planar and vs `modelOf`, pens, LOD, to-scale ground and height, posed vertices vs kernel `poseModel`, progress/cancellation, four-cell sheets first/third, CAD scene, CLI, cast, Python).
- `🧊️axonometric-exports` SVG (fast-xml-parser + sharp rasteriser, 96 % pixel agreement), PDF (pdfjs), DXF group codes: 13/13 for the sample figure and `elderly-with-cane`, `delivery-van`, `duct-fan`; ezdxf audit and CLI export tests: see "Open".
- tsc library: errors only in other agents' files (cast-provenance test, `🧰️support/🐍️.ts`, kernel tessellation at times); none in mine.

## Remaining refusal sites
None in the library, CLI, cast or Python. `VolumeUnavailableError` and its helpers are deleted; React (`🧩️element`, `🗺️routing`, `📤️downloads`) had imports of them: see `T/library-requests.md`.

## Deviations and open issues
- Not run on this host in the final state: the full library suite and `repo run @teaching/staffage:test` (the host's capacity gate and the other agents' load; the direct vitest runs above are the evidence). Result of the ezdxf/CLI tests is recorded at the end of this file if they finished.
- Casts take no camera option (default isometric). Supports, free-body inset, member diagrams and value labels of the planar force/moment views are not drawn in 3D (glyphs are 3D arrows/arcs, legends say so). Transport overlays use the same camera but flat discs.
- `thermal` eligibility of members uses `dimensions.heightM` (as `modelOf`), the member legend uses the drawn state as the planar view does.
- Axonometric of a symbol representation is refused as `ViewMissingError` (no substitution).
- Seams between merged fragments are hidden with a 0.05 mm stroke in the fill colour in solid views.

## Final status of the slow tests
The ezdxf audit test and the CLI export test of `🧊️axonometric-exports` (uv provisioning of the Python oracle set plus a full catalog load) were still running after 25 minutes on the loaded host and have no result; rerun with `... vitest -- run --config ../../🧪️tests/🎚️config/🟦️.ts axonometric-exports`. The full library suite and `repo run @teaching/staffage:test` were not run. Log: `T/🗑️generated/axonometric/exports-slow.log` (delete when done).
