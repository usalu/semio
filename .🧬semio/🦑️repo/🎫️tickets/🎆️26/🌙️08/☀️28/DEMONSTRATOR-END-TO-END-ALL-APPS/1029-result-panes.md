# 1029 Result: panes (pane-level first-run risks)

STATUS: GREEN. Edits done; `:test` rc=0 (5 files, 61 tests passed) and `:test-runtime-catalog` rc=0 (PASSED). Logs: `🗑️generated/1029/panes-{test,catalog}.{txt,rc}`.

## 1. Energy descriptor: already truthful, spec note was stale
- The path `✏️s/🔌️plugins/🔋️energy/🔣️.json` no longer exists. The committed descriptor is `🌎️hub/🧩️compositions/🔋️energy/🧩️component/🔣️.json` (checked in; the served build gets its manifest from the built plugin wasm via the descriptor probe, which stages `🔣️.json` into the module dir).
- It already declares, for `s.energy.model@1/*#editor`: `framework.window.tree` (block-list), `framework.window.table` (table), `energy.simulation` (block-list), `energy.model.3d` (world-3d). This matches the Rust registration: `EnergyModelEditor` edit-mode `layout()` places structure/zones/simulation/model windows; `model` is `SurfaceKind::World3d`, `simulation` is `SurfaceKind::BlockList`, tree/table are the `TreeWindowKit`/`TableWindowKit` kinds.
- No descriptor edit needed. Fixed the stale spec note (acceptance `🟦️.ts`, `framework.window.tree` entry) which claimed the old path was stale.

## 2. statik default example
- Boot path: `ShellHost` brand panel -> `resolveBootExampleId(active, exampleOptions, defaults.exampleId)`; `defaults = resolveShellDefaults(brand, env)`; the env seed `VITE_SEMIO_DEFAULT_EXAMPLE` is not set anywhere in the demonstrator. `exampleOptions` = `examplesForApp(manifest.examples, app)`; the fem component declares `concrete-forest` for `s.fem.fem3d`. So the demonstrator statik pane boots the brand default `concrete-forest`. The 09-17 `demo` reading (3 meshes / 47 instances) predates that and is the standalone-lane/`demo` document.
- `Case: dead` and deformation scale 300 are authored by both `demo` and `concrete-forest` (verified in the fem component descriptor), so the `statik undo-redo` test is example-independent.
- Made it consistent and enforced: added `energie` (`bestest-600`) and `statik` (`concrete-forest`) rows to `🧪️tests/🧪️demonstratorexamples/🧫️cases.json` (language-neutral contract; the test checks brand constant == `pane.brand.defaults.exampleId` == registered `pub const ID` in the example source). `🔨️modules/🧩️runtime/🧫️cases.json` carries no example ids, so nothing to change there. Rewrote the statik spec note.

## 3. `pluginModuleDirNames.slice(2)`
- It sits in the `prepare` script (`PreparationScript`, run by `prepare-prototype`/`prepare-release`), not `prepare-e2e`. The layout is `[MODULE_VENDOR_DIRECTORY, MODULE_SHARD_DIRECTORY, ...plugin dirs]`, so slice(2) skips exactly the `🪞️vendor` and `🧵️shard` support dirs, which carry no `.semio-artifact.json`. Correct, no real component skipped; all 11 plugin dirs + 21 extension dirs are checked.
- Replaced the magic number with an explicit filter on the two `@semio-tech/plugin-registry/deployment` constants so a layout reorder cannot silently skip a component.

## Files changed
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts` (two notes)
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorexamples/🧫️cases.json` (2 rows)
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts` (support-dir filter + import)

SHARED CHANGE: none.
No project typecheck target exists for the demonstrator (targets: prepare-*, dev, build, test, activate-*, serve, prepare-e2e, serve-e2e, test-e2e, canonical-architecture, test-runtime-catalog).

## Verification
- `repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:test` -> rc 0; vitest 5 files / 61 tests passed (includes brand.ts include-source suites, i.e. the extended `demonstratorexamples` rows, and the acceptance console oracle).
- `repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:test-runtime-catalog` -> rc 0 (PASSED).
- No typecheck target exists; Bun/vitest transpile only, so the `script.ts` import of `@semio-tech/plugin-registry/deployment` is covered by the existing precedent in sibling `🟦️.ts`/`📦️assets/🟦️.ts` (same package dependency) but not by a tsc run.
- Not run (coordinator-only): activate/prepare/serve/test-e2e. First served run must re-measure statik counts (expected concrete-forest 2/234) and confirm the energie four windows.
