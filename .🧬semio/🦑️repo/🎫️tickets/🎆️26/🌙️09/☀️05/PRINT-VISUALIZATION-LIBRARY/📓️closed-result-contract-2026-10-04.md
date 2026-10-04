# Closed Inference Result Contract

The final static audit exposed arbitrary-object holes in plan, frame, theme, item, scene node and paint admission. Nineteen language-neutral vectors reproduced malformed geometry, themes, palettes, fonts, paths, paints and incomplete-publication acceptance before the schema correction. After correction the owned validator and independent AJV agree with every expected admission.

The result schema now owns every render item, path command and arity, scene drawing node, path segment, gradient, stroke, transform tuple, frame, theme and palette structure. Completed outputs require nonempty TikZ. Failed outputs require empty TikZ plus diagnostic data and forbid partial plan/scene publication. No external schema or runtime dependency is introduced.

The exported owned output validator validates the canonical JSON wire projection, which omits optional undefined properties and rejects malformed or nonfinite output. The async execution lane invokes it before completed publication. Two differential checks are registered in the existing inference runner.

Exact inventory: print/🧬️schema/💡️inferences/🔣️.json; print/🧬️schema/💡️inferences/✅️validation/🟦️.ts; print/🧪️tests/📜️chart-inference-result/{🔣️.json,🥒️.feature,🟦️.ts}; print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes/🟦️.ts. Final registered build and cross-backend replay follow integration.

The current source root is `🧰️framework/🛍️products/📓️print`, following the concurrent repository taxonomy relocation. The final neutral fixture contains 24 admission cases, including a valid complete result exercising all 6 render item variants, all 8 scene drawing variants, all 8 path commands, all 6 path segments, all 3 paint forms and styled strokes. Four additional malformed ellipse, gradient, stroke and text variants reject as intended. Direct execution on 2026-10-04 showed both owned admission and independent AJV agree with all 24 expectations; the physical text test also passed. The registered exhaustive runner previously passed 408/408 before these fixture additions; final replay remains pending.

Two historical feature descriptions under `🧪️tests/🎬️render-scene/🥒️.feature` and `🧪️tests/👯️kernel-twin-parity/🥒️.feature` now name the current inference owner. No runtime import was affected.

## Physical Text Units

The shared render plan uses millimetre geometry and TeX point font sizes. The scene emitter previously copied point sizes as millimetres, producing 8/12/72.27 scene units where the independent D3 scale gives 2.811678/4.217518/25.4mm. The three neutral vectors failed before implementation and now pass after converting only scene text size. TikZ retains authored point sizes. Native TeX dimension probes are being added to the actual grammar gate for an independent physical measurement.

## Native Backend Style Admission

The six owned print plan variants now admit optional `tikzStyle` as text. Portable scene variants remain closed and do not admit the native backend property. Five added neutral vectors cover valid text on every render variant, a single styled rectangle, rejected numeric/object styles, and rejection of the property on a portable scene node (29 result vectors total). The registered contract baseline completed with exit code 1 and one failing check containing two expected admission mismatches, with zero execution errors. An independent runtime diagnostic recorded those two rejected valid-style vectors at indices 24 and 25. After schema admission, the same registered no-cache quick contract route passed all three checks with zero mismatches or errors, including agreement with independent AJV.

The TypeScript inference lane owns style emission and runtime geometry checks. Native TikZ style text is an explicit print backend extension; arbitrary TeX macros are preserved as authored rather than partially parsed into a portable scene. Existing primitive geometry and scene paints remain independently owned and validated. No standalone rendering module or external runtime interface was introduced.

## Owned Text Baseline Admission

The text render variant now admits the first-party baseline vocabulary alphabetic, middle, top and bottom, with middle as the physical print default. Other render variants remain closed and reject baseline. Six new neutral vectors bring the result fixture to 35 cases. The actual pre-change runtime diagnostic reproduced four valid-text admission failures; after schema-first admission the registered no-cache contract route passed all three checks, including independent AJV agreement. The TypeScript scene and TikZ emitters consume the same authored baseline under canonical inference ownership.


## Owned Text Font Result Admission

The neutral result fixture now contains44 vectors. Nine new font cases were added before implementation: tracked/custom font family names on text plans and scenes, empty/numeric names, and forbidden font properties on rectangles. The registered contract route first failed because four valid text-family cases were rejected. After admitting optional nonempty strings on text variants only, the same registered no-cache route passed3/3 owned/AJV checks with terminal exit0 in8.9s. The shared2D DrawingNode and Canvas painter now preserve explicit family names; the independent Chromium monospace/serif pixel replay is pending. Native current exhaustive inference passed619/619 and current focused legend/font-emission compiled67 geometry records plus three canonical font-selected plans. The final combined native route is still running.


## Actual Shared Canvas Font Pixel Replay

The neutral shared2D alignment fixture now contains six vectors: four anchor/baseline cases plus explicit monospace and serif families. Before the font propagation, the registered shared2D target failed its exact Chromium image comparison (8/9 tests passed). After the owned text font type and Canvas family selection, the same registered no-cache route passed all3files/9tests with exit0 (18.4s Nx,9.29s Vitest). Each of the six scene rasters equals the independent direct Chromium Canvas pixels. The esbuild import.meta/iife warnings belong to unused bundle metadata and were present in the successful runtime replay; the actual Canvas paint function executed.
