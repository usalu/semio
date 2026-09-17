# Demonstrator Nx build cache (2026-09-17)

## Symptom

Rebuilding `@semio-tech/mit-bestand-demonstrator` (`prepare-dev` / `activate-dev` / `dev`) recompiles many plugin wasm components even when plugin sources are unchanged.

## Graph

- `prepare-dev` depends on nine `framework-os-dev:prepare-<variant>-react-dev` targets (one per pane variant in `🧫️pipeline.json`).
- Each `prepare-*-react-dev` depends on `materialize-dev` for every plugin in that variant's runtime closure (`playgroundPreparationTargets` in `🟨️.mjs`).
- `materialize-dev` depends on `component-dev` (wasm in `dist/component-dev`) and stages browser modules under `framework-plugin-web/dist/dev/🔌️plugin-modules/…`.
- `activate-dev` merges lane receipts into `dist/♻️activation/dev` via `publishDemonstratorUnionReceipt`.

Nx already fingerprints these targets with `{ dependentTasksOutputFiles: "**/*", transitive: true }` (lane R1).

## Root cause (native input churn)

`emitOwnerDescriptorPairV1` stages descriptors under `.🛂️descriptor-staging-*` at the plugin **owner** root while `describe` runs. Those directories are not declared outputs and are not in `generatedDirectories`, so they remain inside `{workspaceRoot}/<owner>/**/*` native inputs. Any leftover staging directory (or partial run) changes the `component-dev` hash and forces a full wasm + materialize cascade for every downstream playground prepare.

## Fix

1. `ephemeralOwnerGlobs` in `⚡️caching/🔣️policy.json` — exclude `.🛂️descriptor-staging-*` from cargo `nativeSources` for plugin owner trees (`🟨️.mjs` `projectInputs`).
2. `activate-dev` declares `outputs: ["{projectRoot}/dist/♻️activation/dev"]` so the merged union receipt restores from Nx cache.
3. Keep `parallelism: false` on `component-*` and `activate-*-react-*` so shared cargo target-dir builds stay deterministic for cache keys.

## Verification

- `testDemonstratorRuntime` in `⚡️cache-contracts` asserts `activate-dev` outputs.
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorcompileclosure/🟦️.ts` — wasm link closure (separate from Nx, shrinks demonstrator crate).
- Manual: two consecutive `bun nx run @semio-tech/mit-bestand-demonstrator:prepare-dev` with no source edits — second run should report cache hits for `component-dev` / `materialize-dev` / `prepare-*` leaves.
