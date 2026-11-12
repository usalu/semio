# Staffage volume module: report

S = `🎓️teaching/🏛️architecture/🧍️staffage`. Frame, loft and lateralScale follow design.md amendments A1–A6 (solids pass through unchanged except the mirror across w).

## Files
- `S/🔨️modules/🧊️volume/`: `🟦️.ts` (modelOf), `🧭️frame`, `🧊️solids`, `🦴️skeleton`, `🎨️looks`, `🌡️thermal`, `🏗️structure`, `🪧️glyphs`, `🎞️clips`, `⏱️job`, `🚫️refusal`.
- Exporters: `S/🔨️modules/📤️export/{🧊️glb,🎬️fbx,🗿️model}/🟦️.ts`, bundle `📤️export/🗜️zip/📦️models/🟦️.ts`.
- Edited: CLI `📤️export/🎬️cli` (formats glb|fbx|model|model-zip, `--span-k`, exported `runModelExport`, `isModelFormat`), shared colouring extracted in `🎨️depiction/🌡️thermal-view` (`thermalPaint`) and `🏗️structural-view` (`memberPaint`, `eligibleBones`, `longestArrowMm`, `arrowLengthMm`, `momentRadiusMm`), `🗂️catalog/🧊️volume` (`modelFormats`, `exportIssues`, closed revolve loops), `🗂️catalog/🟦️.ts`, package barrel, `README.md` ("3D models"), `🧬️schema/🔣️.json` (loft, lateralScale, loop wording; twin docstrings unchanged), sample-figure torso loft got `rotation` [0,√½,0,√½], `🧪️tests/🧊️volume-schema` (format refusal removed, 2 loop vectors).
- Tests: `🧪️tests/🧊️model-{frame,skeleton,looks,bounds,exports,catalog}/{🟦️.ts,🥒️.feature}`, fixtures `🧫️fixtures/🧊️model-{frame,skeleton}`, `🧪️tests/🧰️support/🧊️.ts`, `gltf-validator.d.ts`.

## Public API
- `import { modelOf, type ModelOptions } from "S/🔨️modules/🧊️volume/🟦️.ts"`: `modelOf(asset: Asset, options: ModelOptions, operation?: Operation): Promise<ModelDocument>`; `ModelOptions = { scale; at?: EvaluationQuery; poses?; clips?; views?; environment?; thermalSpanK?; thermalQuantity?; forceQuantity?; theme? }`. Also `ModelRequestError` (code, bilingual `reason`), `frameTimes`, `skeletonOf`, `restPlacements`, `placementsOf`, `surfacesOf`, `toNeutral`, `pseudoToNeutral`.
- `writeGlb(asset, { pose, scale, ... }, control?)`, `writeFbx(asset, { clip, view?, scale, ... }, control?)`, `writeFbxDetailed` (+notices), `writeModel(asset, { pose | clip, resolved?, ... }, control?)`, `modelStem`, `writeModelBundle(asset, opts, control?)`; all `Promise`, `JobControl` (signal, progress), all re-exported from `📦️packages/🟦️typescript/🟦️.ts`.
- Deviation from design: no `view` option; base materials are the light-solid look, `views` filters variants.

## Tests (direct vitest via `repo tools exec --light vitest`; `repo run` not used)
- My cases: frame 26, skeleton 10, looks 13, bounds 8, exports 18 pass; model-catalog (≈470 tests over the sample and the ~70 authored catalog assets) 16 fail, all from in-progress authored assets (volume rule findings in drafts, e.g. date-palm extent 7.0×12.5×7.0 vs 6.3×12.5×7.0), none from the pipeline.
- Full library run before the amendments: 91 files, 1792 passed, 39 failed, 25 skipped; minus the 8 catalog-extent failures since fixed it equals the baseline's 31 failures (uv/ffmpeg/rsvg/qpdf missing, catalog and cast state of other waves). `🧊️volume-schema` shapely test needs uv (pre-existing).
- tsc: library unchanged (4 pre-existing errors), React 28 (baseline).

## Oracles
Khronos gltf-validator 0 errors/0 warnings for every pose GLB; three GLTFLoader/FBXLoader load all; FBX + AnimationMixer reproduces `evaluate` bone positions within 6 mm; three Matrix4/Quaternion/Box3 for frames, mirrors, extents; 2D `depict` for colours (1e-6 linear) and arrow/arc paper sizes; ajv with the kernel schema for documents; fflate for the ZIP. Pose extent equals the 2D elevation extent within 15 mm.

## Deviations and notes
- Thermal/structure member eligibility and arrow scale use `dimensions.heightM`, not the drawn extent. Emitters are cones (no parcels/streamlines); supports are not glyphs. Pose entries other than the static one hide glyphs. A clip always writes the root translation channel.
- Catalog bounds test uses `presentation.defaultPose`, then the best-matching pose (printed as `[extent]`), 8 % gate. A kernel loft default (A2) is pending; the sample torso now carries an explicit rotation.
- Remaining refusal sites: only `axonometric` (depiction, CLI viewpoint, cast, React, Python). Closed revolve loops accepted by the validator; the Python/ajv shapely oracle for them not run (no uv).
- Open: not compared 3D vs 2D front/plan for assets lacking `sweep`; `decentralised-hrv-fan` clip model takes ~8 s per 420 s clip.
