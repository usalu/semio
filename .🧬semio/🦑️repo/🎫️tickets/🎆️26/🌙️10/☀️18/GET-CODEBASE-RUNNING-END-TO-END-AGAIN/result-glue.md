# Result glue: every `exports` target of every workspace package exists

Executor `glue`. No `package.json` / `bun.lock` edit, no git command, no cargo, no sub-agent. C = this ticket folder.

## 1. Outcome

| item | result |
|---|---|
| workspace manifests checked | 198 manifests with `exports` out of 303 workspace packages (root `package.json` plus the 5 nested workspace roots `✏️s`, `🌎️hub`, `♻️mit-bestand`, `🎓️teaching`, `🏢️semio-tech`; the root list alone covers only 120 packages) |
| export targets listed (all conditions) | 1,705 |
| missing, glue from the repoapi batch | **0**. The 502 glue files named in BATCH3 already exist (repoapi `glue-others.ts --write` created 503 files, all present). Nothing had to be generated. |
| missing, build outputs (git-ignored, produced by builds) | 94 targets, expected: `./dist/🟦️.js` + `./dist/🟦️.d.ts` of 46 plugin artifact packages, `./dist/extension.js` (repo-vscode), `./🕸️bindings/framework_os_node_graph.js` (wasm-bindgen output) |
| missing, stale manifest entries | **7**, all listed in section 3 as manifest edits for the deps owner (cannot be fixed without touching `package.json`) |
| `.//` generator bug | fixed in `repoapi-tools/exports-final.ts` (`targetOf`) and `exports-final.json` normalised (section 4) |

Why the 7 are not glue to create: the 6 `@semio-tech/framework/repo/modules/library/*` entries (lay9 proposals) point framework at repo-product library files; their symbols (`runBunx`, `typecheckRepositoryProjectV1`, `runVitest`, `runRepositoryCargoTests*`, `buildRepositoryWasmWebV1`, `repoCacheDirectory`, `isGeneratedPath`) were classified by repoapi as (a) framework ports, and every importer was rewritten to `@semio-tech/process` entries (`git grep 'framework/repo/modules/library'` finds no importer). A framework package re-exporting the repo product is also a layering violation. The 7th, `@semio-tech/ui-styling/builder/python/inventory`, points at `🏗️builder/🐍️python/📋️inventory/🟦️.ts` in the styling package, but that module lives only in the repo library (`📚️library/🎨️styling/🏗️builder/🐍️python/📋️inventory/`, imported relatively by its sibling); no importer uses the entry.

## 2. Defects found by the stronger checks and fixed (source edits, no manifest)

Existing is not enough, so three more checks ran over all 1,486 existing TS/JS targets (details in section 5). They found:

1. `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📥️source/🟦️.ts` re-exported `elementsSurfaceDeviceForLayout` and `type ElementsSurfaceLayout` from `../../../🟦️.tsx`, which does not export them (owner is `🪟️chrome/🟦️.ts`). Runtime import of `@semio-tech/ui-react/source` would have failed at link time. Split into two statements, the second from `../../../🪟️chrome/🟦️.ts`.
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/🟦️.ts` re-exported `../../../../../../🔨️modules/🐚️shell/🟦️` without extension; now `…/🟦️.ts` (repo rule: explicit extensions).
3. `🧰️framework/📦️packages/🟦️typescript/🎚️config/🧬️schema/🧬️mutations/🟦️.ts` (the `@semio-tech/framework` config-mutation glue) lacked the diff API that 7 importers use through `@semio-tech/framework/config/schema/mutations`. Added 14 values (`absorbIdentityDiff`, `absorbLocalCatalogDiff`, `applyIdentityDiff`, `applyLocalCatalogDiff`, `decodeIdentityDiff`, `decodeIdentityValue`, `decodeLocalCatalogDiff`, `decodeLocalCatalogValue`, `diffIdentityConfigMutation`, `diffLocalCatalogConfigMutation`, `identityMutationDiffPair`, `inverseIdentityDiff`, `inverseLocalCatalogDiff`, `localCatalogMutationDiffPair`) and 2 types (`IdentityDiff`, `LocalCatalogDiff`), all re-exported from the owner `🔨️modules/🎚️config/🧬️schema/🧬️mutations/🟦️.ts` (which has `export * from ⚙️apply-diff`). Runtime check: `import('@semio-tech/framework/config/mutations')` exposes them as functions. The docstring still names the entry `config/mutations`; update it with the manifest rename in 3.2.
4. Importer with the wrong package: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎚️UiPreferences/🧪️tests/🌐️preference-lane/🟦️.ts` imported `UI_PREFERENCE_MUTATION_KEYS`, `uiPreferenceMutationDataClassV1` from `@semio-tech/framework/config/schema/mutations`; they live in the OS package: now `@semio-tech/framework-os/config/schema/mutations` (key exists, the renderer-react manifest already depends on `@semio-tech/framework-os`).
5. Importer with a file path as subpath: `✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🟦️.ts` used `@semio-tech/framework-ui-viewport-ts/3d/🧬️schema/🟦️.ts` (no such key); now `…/3d/schema` (the existing key). Same file, line 45: relative `…/🕹️interaction/schema` (directory without module, stale) now `…/🕹️interaction/🧬️schema/🟦️.ts`.

## 3. Manifest edits for deps owner

### 3.1 Remove 7 dead export entries

`🧰️framework/📦️packages/🟦️typescript/package.json`, delete these 6 lines from `exports`:

```json
"./repo/modules/library/build/typescript": "./🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟦️typescript/🟦️.ts",
"./repo/modules/library/caching": "./🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts",
"./repo/modules/library/process/bun": "./🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🍞️bun/🟦️.ts",
"./repo/modules/library/process/cargo/admission": "./🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts",
"./repo/modules/library/process/wasm/policy": "./🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🕸️wasm/🔣️policy/🟦️.ts",
"./repo/modules/library/testing/vitest": "./🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧪️vitest/🟦️.ts"
```

`🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/package.json`, delete this line from `exports`:

```json
"./builder/python/inventory": "./🏗️builder/🐍️python/📋️inventory/🟦️.ts"
```

After deleting these, `bun glue-tools/check-exports.ts` reports 0 missing.

### 3.2 Add the key 7 importers already use (and nothing resolves today)

`🧰️framework/📦️packages/🟦️typescript/package.json`, `exports`:

```json
"./config/schema/mutations": "./🎚️config/🧬️schema/🧬️mutations/🟦️.ts"
```

The glue behind it exists and exports every name those 7 files import (checked by name against the owner). The old key `"./config/mutations"` has the same target and 34 importers (OS config leaves, tests). Greenfield-clean is to rename `./config/mutations` to `./config/schema/mutations` (it sits beside `./config/schema`, `./config/schema/mutations/identity-config-mutation`, `./config/schema/mutations/ui-preferences`) and rewrite the 34 importers in one go (`git grep -l '@semio-tech/framework/config/mutations'`); I did not do that while the manifests are frozen because the rename would break those 34 importers until the manifest changes. Until then add the key as above.

## 4. Generator fix in `repoapi-tools`

* Bug: `targetOf` in `C/repoapi-tools/exports-final.ts` built `` `${dirOfOwnerRel}/🔣️json/…` ``; for a JSON file directly in the source owner the directory part is empty, giving `.//🔣️json/🟦️.ts` (compiler-syntax-rust-ts `./index.json`, framework-plugin `./runtime-fault-vectors.json`). Now the path is assembled from segments (`[...dir, "🔣️json", stem, "🟦️.ts"].join("/")`), no empty segment can occur.
* `exports-final.json` was not regenerated (the full scan takes long and the specs moved on); `glue-tools/fix-final-spec.ts` normalised the 4 affected strings (targets and glue paths with `//`) in place. The deps owner already normalised the two live manifest lines.
* `apply.ts` and `glue-others.ts` were not re-run (nothing to create; apply rewrites 1,059 importers and is the coordinator's step).

## 5. Tools (inputs, kept) in `C/glue-tools/`

All run from the repo root with `bun`. They read the root `package.json` and the nested workspace roots.

| tool | purpose |
|---|---|
| `check-exports.ts [--all] [--json]` | lists all exports targets (all conditions, nested arrays, `*` patterns), reports missing ones; git-ignored absent targets are reported separately as build outputs; exit 1 when something is missing |
| `verify-reexports.ts [--list-opaque]` | every relative `export … from` / `import … from` of every existing target resolves to a file and every named member exists in it (follows `export *`); Bun's bundler does not check re-exported names (verified with a control file), TypeScript-API-based `exportsOf` from repoapi does |
| `build-targets.ts [--only=] [--sample=]` | `Bun.build` (target bun, packages external) per manifest over all existing TS/JS targets; Vite-only specifiers (`?raw`, `?url`, `?inline`, CSS urls, wasm `bindings/` and `pkg/` outputs) are not counted |
| `check-importers.ts [--json]` | every bare import of a workspace package in the source tree (static, dynamic, export-from, require, import types) against that package's exports map (exact keys and patterns) |
| `sample-glue.ts`, `sample-specifiers.ts` | deterministic samples for the spot checks |
| `fix-final-spec.ts` | one-off normaliser of `exports-final.json` (section 4) |

## 6. Verification (exact commands, outcomes)

1. `bun glue-tools/check-exports.ts` (final): `manifests with exports 198, targets 1705, missing 7, gitignored build outputs absent 94`; the 7 are section 3.1. Before the nested roots were included the same tool said 82 manifests / 1,340 targets / 9 missing (the 2 others were build outputs, now classified).
2. `bun glue-tools/verify-reexports.ts`: `targets 1486, relative re-export/import specifiers 2739, … 25 glue files skipped because the target re-exports from a bare package, problems 0` (after fixes 1 and 2; before: 4 problems, of which 2 real (ui-react source, os-shell) and 2 environment (wasm bindings output)). The 25 skipped files were loaded at runtime instead (below).
3. Runtime load of the 25 glue files with bare re-exports (`bun -e "await import(<file>)"`): 21 load. 4 do not, none caused by the glue: 2 needed the key of 3.2 (`Cannot find module '@semio-tech/framework/config/schema/mutations'`: ShellHost `🟦️.tsx`, renderer-react `🟦️.tsx`); 1 `@dnd-kit/core` unresolved from the ui-react source owner (section 7.1); 1 uses `import.meta.glob` (Vite only, resolution fine).
4. `bun glue-tools/build-targets.ts`: `built 1520 entries across 162 manifests, failing entries 0` (final run).
5. `bun build --target=bun` on a deterministic sample of 20 new glue files (`sample-glue.ts --count=20`): 14 build; the 6 failures are all outside the glue: 1 Vite `?raw` fixture import, 5 `Could not resolve "@dnd-kit/core"` (section 7.1). Module resolution of the glue paths themselves is clean (it is the whole 1,520-entry `build-targets` run that passed).
6. Spot check, `bun -e "import('<spec>')"` from the repo root, 14 specifiers (`sample-specifiers.ts` plus 3 by hand): `@semio-tech/framework-kernel/return/content/input` (5 exports), `…/framework-os/modules/renderer/engine/elements/HubConnection/connection-contract` (0 runtime exports, types only by design), `…/ShellHelpers/window-panes/clipboard-activation` (1), `@semio-tech/framework-plugin/browser-bundle/descriptor` (6), `…/framework-plugin/model/contract` (4), `@semio-tech/framework-renderer-wgpu/turn-budget` (7), `@semio-tech/framework/manifest/testing/fixtures` (13), `…/framework/onboarding/testing/fixtures` (1), `@semio-tech/process/artifacts/cache/model` (1), `@semio-tech/process/compiler-capacity` (9), `@semio-tech/repo-lib/storage` (29), `@semio-tech/repo-lib/laws` (129), `@semio-tech/framework/config/mutations` (47 incl. the added diff API). `@semio-tech/nx-bootstrap-tools/sdk/binary/testing/phase` resolves and then throws `afterAll() outside of the test runner` (test-only module, expected).
7. `bun glue-tools/check-importers.ts` (final): `source files 15800, workspace packages 303, bare workspace imports checked 10880, uncovered imports 12 (6 distinct specifiers)`: `@semio-tech/framework/config/schema/mutations` (7 importers, fixed by 3.2) and 5 in `.storybook/preview.tsx` (section 7.3). Before the importer fixes of section 2 there were 7 specifiers / 14 imports.
8. Not run: the dependency-cruiser, `bun repo …` targets, vitest suites, `tsc` over the changed glue (the Bun link check and the name check stand in for it), `apply.ts`.

## 7. Open items (outside this task, found on the way)

1. **Third-party deps are not resolvable from sources outside the package directory.** The packages moved to `<owner>/📦️packages/🟦️typescript` but their sources stay in `<owner>`; `node_modules` lives only in the package dir. A scan (`Bun.resolveSync(dep, <owner dir>)`) finds 56 third-party dependencies of 285 packages that resolve from the package dir but not from the owner dir, e.g. ui-react `@dnd-kit/core|sortable|utilities`, `react-resizable-panels`, `tailwindcss`, `terser`, `katex`, `@types/d3-*`; print `d3-*`, `unified/remark-*/rehype-*` (presentation), `tree-sitter-wasms` (actor). This is why `ui-react/🗿️framework-exports/🟦️.ts` (and every glue that loads it) fails with `Cannot find module '@dnd-kit/core'`. Needs a decision from the deps owner (hoist those to the root, or project `node_modules` into the source owner directory).
2. Vite-only features show up as plain-Bun failures (`?raw`, `?url`, `import.meta.glob`); they are loaded through Vite in the products and are not counted as defects.
3. `.storybook/preview.tsx` imports `@semio-tech/framework-surface-{node-graph,paint,terrain,tiled-map}-rs/pkg/*.js` (packages no longer exist) and `@semio-tech/flow-core/flow_core.js` (flow-core exports only `.`; the wasm bindings are the git-ignored `🕸️bindings/flow_core.js` of the browser-package crate). Storybook config looks stale; not touched.
4. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust/🟦️.ts` imports `semioSetRuntimeAssets` from the git-ignored `🕸️bindings/flow_core.js`; the on-disk generated file is stale (no such export). A fresh wasm build regenerates it.
5. The glue docstring convention of the repoapi generator starts every file with the same emoji (🧩️ `Public …`), as in the 500+ existing glue files; I kept it for the one file I extended and did not renumber them.

## Files

Created (ticket inputs): `C/glue-tools/{check-exports,verify-reexports,build-targets,check-importers,sample-glue,sample-specifiers,fix-final-spec}.ts`, this report. Generated output (`C/🗑️generated/glue`) deleted.
Edited: `C/repoapi-tools/exports-final.ts`, `C/repoapi-tools/exports-final.json`; source: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📥️source/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/🟦️.ts`, `🧰️framework/📦️packages/🟦️typescript/🎚️config/🧬️schema/🧬️mutations/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎚️UiPreferences/🧪️tests/🌐️preference-lane/🟦️.ts`, `✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🟦️.ts`.
