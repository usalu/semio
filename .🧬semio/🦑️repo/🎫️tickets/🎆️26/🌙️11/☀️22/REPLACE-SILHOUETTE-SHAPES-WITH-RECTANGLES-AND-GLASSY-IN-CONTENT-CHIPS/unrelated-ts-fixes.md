# Unrelated TypeScript fixes

All counts from the shared worktree, native tools (`repo run` stalls on the graph read). `R` = `/Users/ueli/Documents/semio`. No `.rs` and no `📓️print` file was touched. Helper scripts in this folder: `os-vitest.sh` (OS renderer suite, `SEMIO_TEST_LEVEL=long`), `tui-vitest.config.ts` (runs the bun test under vitest, bun is not installed), `ui-direct-vitest.config.ts`, `os-direct-vitest.config.ts`.

## 1. UI typecheck: 7 errors to 0
Command (cwd `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript`): `npx tsc --noEmit -p tsconfig.json`.
- `chooseFile` x2 (`🌐️i18n/⚙️configuration`): the configuration carries `chooseFile`, `previousMonth`, `nextMonth`, `colours`, `noFileChosen` but the `ui.common` type in `🧱️elements/📚️I18n/🟦️.tsx` did not declare them. Added the five keys to the type (the Rust `common-labels` already lists them).
- Tree `🧲️drag-drop.tsx` test: `data` was an object literal union with `undefined` values; typed `Record<string, string>`.
- ContextMenu `📝️text-selection` (source and test): the schema JSON is inferred as a union with optional fields, so `"verb" in row` / `"labelKey" in row` did not narrow. Source now tests `!row.verb || !row.chord`, test tests `!row.labelKey || !row.label`.
- Tests run: both touched test files, 7 passed (`ui-direct-vitest.config.ts`). Full UI suite (`sh compositor-vitest.sh`): 1629 passed, 5 failed, all Playwright chromium (unchanged).

## 2. OS typecheck: 6 errors to 0
Command (cwd `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript`): `npx tsc --noEmit -p tsconfig.json` (3 errors at start of this run, 6 in the handoff; the other three had already gone).
- `refresh.sections`: hop-trace stage missing from the schema-owned list. Added to `🔨️modules/⏱️trace/🟦️.ts` `HOP_TRACE_STAGES` and to the shared fixture `🧪️tests/🧫️fixtures/🪃️hop-stages/🔣️.json` (fixture first, same order).
- `Canvas2dHost/🧪️tests/🎨️paint` `fontFamily`: the PDF fixture has no family (Rust never reads one), the test cast a missing field. Test now passes an explicit catalog family. That exposed a real stale expectation: text layers use a top-left `y` with baseline `y + size`, so the PDF node now states `y: -size` to land the baseline at 0. Test passes (27 passed with hop-trace).

## 3. Demonstrator: 2 failed + 1 load error to 0 (105 passed)
Command: `node R/node_modules/vitest/vitest.mjs run --config 🧪️tests/🎚️config/🟦️.ts --configLoader native` in `♻️mit-bestand/🧺️demonstrator`. Before: 94 passed, 2 failed, 1 suite error.
- `chargeOf is not defined` (`demonstratorcompileclosure/🎭️e2e`): the test evals the library's `commandUrlReferences` with a hard-coded state preamble; the library now has a `chargeOf` helper. The test now takes the module's own `facts`/`chargeOf`/`bytes` declarations from the parsed source instead of a copy.
- `lang` rule: `🌐️.html` carried `lang="mul"`; the contract (`🧪️demonstratorlocale/🧫️cases.json`) forbids any static language on the document. Removed it.
- `activation` module-load error: `⚡️caching/🚀️bootstrap/📦️dependencies/♻️collection/🟦️.ts` imported its schema JSON without `with {type:"json"}`, which native ESM (externalised library) rejects. Added the attribute like every sibling.

## 4. OS renderer suite (long level, native vitest, `sh os-vitest.sh`)
Before: 3504 passed, 53 failed. After: 3516 passed, 41 failed (of 3578). Of the 41, 22 are the missing chromium.
Fixed (root cause, no weakening):
- Import cycle: `Interpreter` in-source test imported `uiIntentToActionDescriptor` from `ShellHelpers/tree-windows` (higher layer). Moved `uiIntentToActionDescriptor`, `actionBindingToActionDescriptor` and their two private helpers to the new leaf `🗣️Interpreter/🎬️intent-descriptor/🟦️.ts`; every importer (ShellHost x3, ShellHelpers, tests, react barrel) points at it; added package entry `Interpreter/intent-descriptor` (`📦️packages/🟦️typescript/package.json` and its re-export file); `tree-windows` entry no longer re-exports it.
- `shell-holds`: test expected `reserve + spacing(1)`; the cap row geometry (and the test's own box math) is `reserve` flush. Test corrected.
- `command-panel`: staged-arg inputs have unique ids `<item id>.input` (accessibility); test looked up the bare arg id.
- `actor-window-actions`, `scene-pointer-cancellation`: source-text pins pointed at code since split into `input-funnel/program-dispatch` and `map-pointer`; pins follow the code (cancel reaches `pointerUpScreen` through `finishMiddlePan` and `endPan`).
- `world3d-interaction`: fixture `🖱️pointer-gestures.json` lacked the `vortexHover` row the test reads; authored it (markers a, b, c; held a; settled a, c).
- `workflow-flow-routing`: `readFileSync(new URL(...))` fails under jsdom URL; uses `resolve(import.meta.dirname, ...)`. Also `wgpu-document-reconcile` import repoint.
- `renderer-types-and-forms`: previewed presence renders as the hover border state by design; test now expects `hovered || previewed`.
- `navigator-camera`: `resizeCanvasBackingStore` wrote width and height whenever either changed; now writes only the changed dimension (fixture `writes`). 115 tests of backing-store users pass.
Remaining non-browser:
- `hidden-tab-surfaces`, `node-graph-hosts`: missing built wasm binding `🎁️browser-package/🕸️bindings/flow_core.js`. Environment (artifact not built).
- `package-integration` (9): "Current WGPU package catalog digest drift", generated package catalog stale. Environment/regeneration.
- `procedural-descriptors` (1): the published procedural descriptor (env `SEMIO_PROCEDURAL_COMPOSITION_DESCRIPTOR`) declares `setLodMode` on the preview windows too; ownership lives in the Rust plugin definition (`window_kind_action_refs`), outside this scope.
- `embedded-mount` (9): boot spawns 4 frame Workers where the test expects 2 and the boot message is not found on the last worker; needs a design decision on the new worker topology (not traced).
- `turn-patch-batch` (1): 60 s timeout, not traced. `♻️lifecycle` (1): chromium bundle cannot import `@xyflow/react` css (browser suite).

## 5. Course `local-stack`
Cause: environmental. `viteDevInvocation(..., "bun")` spawns `bun`, which is not installed here (`which bun` fails), so `spawnSync` returns status null. The node path works (`node --experimental-transform-types node_modules/vite/bin/vite.js --configLoader bundle --version` prints vite/7.3.7). No code change.

## 6. Coordinator addition: TUI (`⌨️tui`)
`🎨️emphasis/🟦️.ts` run under vitest with `bun:test` aliased (bun absent): 5 passed (3 passed, 2 failed before).
- Ajv: schemas mixed draft-07 and 2020-12 while the test uses `Ajv2020`. Moved `window-contour`, `chrome-emphasis`, `drag-preview`, `shared-splitters` to `https://json-schema.org/draft/2020-12/schema` (none used draft-07-only keywords). `🧲️docking` and `📏️text-width/🪟️console` schemas remain draft-07 (not validated by this test).
- Role expectation: fixture and feature say hover has priority over active (`active 1, hover 1` is `emphasized`); the test computed active first. Test now checks hover first.
- Rename `window-contour-v2` to `window-contour`: the id was only in the schema `contract.const` and the fixture `contract`; both changed. No file, folder or Rust string carried the suffix (grep over the repo is clean except `tui-handoff.md` history).

## Left
Playwright chromium tests (UI 5, OS 22 plus lifecycle), the items under section 4, and the local-stack bun item.
