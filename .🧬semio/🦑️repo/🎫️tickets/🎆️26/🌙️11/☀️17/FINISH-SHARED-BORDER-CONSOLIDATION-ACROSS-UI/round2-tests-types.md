# Round 2: tests and types

## 1. Interpreter suites (20 "fail at collection")
Root cause: environment/config, not code. The Interpreter suites are in-source tests: `🗣️Interpreter/🟦️.tsx` registers all `🧪️tests/*/🟦️.tsx` modules under `if (import.meta.vitest)`. Those files are register functions with no `describe`, so a config with `include: **/🧪️tests/**` collects them as empty files ("No test suite found"); the `bun:test` plugin-interpreter files are bun-only and cannot run under vitest.
Fix: new ticket config `vitest-interpreter.config.ts` (`includeSource` of the Interpreter entry, jsdom, react env). Result: 195/195 pass (incl. row/section/group/field/stack container tests). No assertion needed changing.

## 2. puzzle3d-settings-document / wgpu fixture
- TS test (`💻️os/.../🧪️tests/⚙️puzzle3d-settings-document/🟦️.tsx`): replaced `gap-single` assertion on the field with the shared-border-row contract (vertical, `data-shared-border-cells`, no `gap-*`, nested `field-control` row). 7/7 pass.
- NOT done: the wgpu geometry fixture `🖱️ui/🧪️tests/🧫️fixtures/📐️section-field-presentation/🔣️.json` (density.gap, controlTop/errorTop/totalHeight) and the Rust renderer (`field_chrome_metrics` in `🎯️targets/🧊️wgpu/📐️flex/🎨️style-mapping-and-chrome-metrics`, mounted_layout, paint retained-text-steps, tests `targets-wgpu-mounted-layout-unit`, `targets-wgpu-paint-unit`) still model a gap-based Field. The numbers are Chromium-measured text bands; re-deriving them for cell padding + hairlines needs a browser measurement and a Rust layout change that cannot be validated here. Open item.

## 3. tsc (`--noEmit --incremental false`, tsconfigs: ui-react, course-react, os, hub admin, staffage, staffage site; presentation + demonstrator have none, checked with ticket `tsconfig-presdemo.json`)
Introduced by this ticket, fixed:
- `🪢️SharedBorderRow`: ref typing of the polymorphic tag (TS2322 x2) -> tag narrowed to `"div"` internally.
- `SharedBorderRowElement` lacked `header`/`aside`/`footer` used by SpaceBrowser, AgentDelegations, HubSignIn -> added.
- `Input` requires `id`: added ids in `InkCanvasHost/ink-overlays`, `TextEditor` rename input, `SharedBorderRow/shared-border.test.tsx`, Popover `shared-border-cells` test.
Result after fixes: no error remaining in SharedBorderRow, SpaceBrowser, HubSignIn, AgentDelegations, TextEditor, InkCanvasHost, Field, Interpreter-container code (os run 545 -> 537). Staffage package: 0 errors. Presentation: 0 own errors. Demonstrator: only implicit-any in `shell-brand` test, cascading from missing generated modules.
Pre-existing, not fixed (unrelated): missing generated modules `🛂️manifest/🤖️generated/*`, `🎭️actor/🤖️generated` (TS2307, cause most implicit-any cascades in os/course/hub/demonstrator); `🐚️shell` module exports; TS2366 "lacks ending return" in `introduction-demo`, `Tree/virtual-window`, `manifest/app-definition-refresh`, `ui-contract accessibility`; ui test-only type drifts (string vs UiLabel in Tree/Canvas/Window tests, ToggleGroup test props, `ModeWindowDescriptor.iconId`, `ShellScope` casts in Popover/ContextMenu/Select tests, `Select` missing `id`, `dataset` on Element); os tests (mcp, sqlite bindings, world-handle-drag, window-chrome-ribbon, view-model-surfaces, saved-state). Logs were in `🗑️generated/tsc-*.log`.
Note: 3 "failed" Popover files in the ui run are helper modules without suites (same collection artefact); all 25 real tests pass.

## 4. Staffage teaching suite
- 15 files failing on `🚶️reference-walker` ENOENT: root cause `🧪️tests/🧰️support/🧰️.ts` resolved the staffage root from `process.cwd()` (only right when run from `📦️packages/🟦️typescript`). Fixed to `import.meta.dirname` (cwd-independent).
- 2 catalog failures (119 ready vs 152 / `revision` undefined): not a regression. The ticket config lacked the suite's own Vite plugin (`staffageAssetVitePlugin`, serving `?staffage-asset`) and aliases, so every asset load failed. `vitest-teaching.config.ts` now re-exports the real config (`🧪️tests/🎚️config/🟦️.ts`); run with cwd `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/📦️packages/🟦️typescript`. Result: 29 files, 270 tests pass.
