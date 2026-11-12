# React target and site: 3D integration

R = `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react`, RM = `R/🔨️modules`, SITE = `…/staffage/🌐️site`.

## Files
New, `RM/🧊️spatial/`: `🟦️.ts` (presets, `variantOf`, `qualityFor` level of detail, `modelKey`, `describeModel`), `🏭️prepare` (`prepareModel`: `modelOf`, `tessellate`, `featureEdges`, cache, progress, cancel; `inPlace`, `withoutGlyphEdges`), `🖥️stage` (`ModelStage`, vanilla DOM + owned `ModelViewer`), `⚛️figure` (`ModelFigure`), `🚪️gate` (lazy `SpatialFigure`), `📤️exports` (`exportSpatial`).
New i18n: `🌐️i18n/🔠️english/🧊️model.ts`, `🔡️german/🧊️model.ts` (merged into both bundles), plus 8 keys in the viewpoints parts.
Edited: `🔭️viewpoints` (+`🎚️selector`, `🪝️preference`), `🧍️figure`, `🧩️element`, `🧮️pipeline` (`DepictionRequest.viewpoint: Viewpoint`), `🗺️routing`, `🎛️viewer` (+`🗺️stage`), `📤️downloads`, `🔬️detail` (+`📤️export-panel`), `⚖️compare/🧮️panes`, `🧪️playground`, `📦️packages/🟦️typescript/{🟦️.tsx,tsconfig.json}`.
Site: `SITE/🏗️builder/🌐️vite/🟦️.ts` (`staffageModelPipeline`, release `treeshake.moduleSideEffects`).
Tests: `R/🧪️tests/{🧊️model-view,🧊️model-downloads,🧊️model-viewer-page}` (each with `🥒️.feature`), `🧰️support/🧰️.ts` (`sampleFigure`), `SITE/🧪️tests/🧊️model-split`; two existing tests widened (`📖️detail-sections`, `🔭️viewpoints`).
Requests: `T/kernel-requests.md` (follow camera, preset event, adopted stylesheet), `T/library-requests.md` (glyph edges, bundle size, sync axonometric cost, file-name convention).

## API additions
- `StaffageFigure`: `viewpoint: ViewpointKey` (`elevation|front|plan|axonometric|model`), `preset?`, `onPreset?`, `background?`, `toolbar?`. `model` mounts the lazy stage; `axonometric` goes through `depict` (it already works; an asset without volume gets a named alert via `refusalOf`).
- `<staffage-figure viewpoint="model|axonometric">`, attributes `camera`, `no-toolbar`, property `.stage`, events `staffage-model`, `staffage-camera`; CSS `--staffage-model-{background,foreground,border,accent,accent-foreground,focus,panel,min-height,height,target}`.
- Address: `vp=axonometric|model`, `cam=elevation|front|plan|axonometric|perspective`; `ViewerState.camera`.
- Exports: `ExportFormat` is now derived from the schema (`Exclude<…, "component">`), adds `glb fbx model model-zip`; panel lists `EXPORT_FORMATS`; `exportFilename`/`spatialFilename` `<asset>_<pose|clip|bundle>_model_<representation|all>_1-<den>.<ext>`; `ExportPanel` props `viewpoint`; GLB needs a pose, FBX a clip (selectors), model JSON either, bundle neither; `model-zip` is offered when `model` is in `asset.exports`; drawing formats are blocked under `model`.
- The refusal catch sites for `VolumeUnavailableError` are removed from the target (viewpoints, figure, element, downloads); the only remaining refusal is "asset has no volume".

## Tests
Commands (direct, bun): `vitest.mjs run --configLoader native --config ../../🧪️tests/🎚️config/🟦️.ts` in the React package; site: same with an absolute `--config` path and filter `model-split`.
React: baseline 158 passed / 6 failed / 1 skipped / 21 files (generated tokens and icons produced first with `T/react-site-generate-{tokens,assets}.ts`); now 204 / 6 / 1 / 24 files: +46 new passing, the same 6 failing (catalog-loading, 3 compare, 2 playground: other waves' catalog state). Site: 3 host-document + 2 new pass. tsc React: only pre-existing errors (WindowChrome, polygon). `repo run` was tried (typecheck) and stayed behind the capacity gate until cancelled.
Oracles: Khronos `gltf-validator` 0 errors 0 warnings, three `GLTFLoader` (7 variants) and `FBXLoader` (animation duration), fflate unzip, kernel `validateModel`.

## Real browser (Chromium pane, release build of the site served statically)
The dev server could not be started through the preview tool (processes it launches cannot read `~/Documents`: bun hangs at startup, python fails in `getcwd`), and `repo run …:dev` waited behind 19 queued jobs. I built the site with the project's vite config (`--mode release`) into the scratchpad and served that with python through the preview tool. Seen:
- `standing-adult` pose and clip, light-solid, thermal (front), forces (elevation), dark-line: rendered with WebGL2 (`data-backend=webgl2`); presets by URL, keys 1/2/4, button; arrow drag leaves presets unpressed; drag orbit worked; Space toggled the shared transport.
- `city-car` `drive-by` thermal and `excavator` `dig-cycle` forces: rendered; the excavator boom differed between two frames (shared clock). Rebuild of a clip model at another scale took 1.1 s with no long task over 50 ms.
- German UI text, software fallback (WebGL2 blocked by patching `getContext`): picture drawn by software with the translated note.
- Downloads (catalog `exports` patched in the served copy, since no catalog asset lists glb/fbx/model yet): GLB 885.7 kB, FBX 275.1 kB, model JSON 434.9 kB with the stated names; bundle cancelled at 10043/14000 permille (now percent) and reported cancelled.
Not verified in a real browser: touch and pinch, Firefox/Safari, a real WebGL context loss, forced-colours and reduced-motion media (jsdom and CSS text only), a real GPU frame-rate. Screenshots taken while the pane was hidden showed a stale canvas once (rAF throttled).

## Deviations and open issues
- 2D-only path: viewer, stage and GLB/FBX/bundle encoders are lazy chunks (test guards it). The volume module is in the entry because `depict(axonometric)` imports it (library request). Entry chunk 1,347 kB to 1,442 kB, part of it other waves' library growth.
- Clips play in place (root travel replaced by rest position) until the viewer can follow; glyph edges are stripped in the view only (GLB export still has them).
- Comparison and playground stay planar only (they say so and compare in the side elevation). Axonometric clip frames are drawn synchronously per clock frame (23-66 ms for the sample); no `depictAsync` progress yet.
- Analytic views in 3D show no legend (a note points to a drawing viewpoint); fixed display ranges are not applied in 3D (span only).
- Stage CSS uses an adopted stylesheet (the release CSP forbids `<style>`); `<style>` fallback otherwise.
- `T/verification.md` was not there yet: static batch downloads are not wired.
- Temporary `.claude/launch.json` and scratch builds were removed.
