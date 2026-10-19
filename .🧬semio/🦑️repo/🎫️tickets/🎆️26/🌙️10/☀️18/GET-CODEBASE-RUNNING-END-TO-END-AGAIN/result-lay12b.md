# Result lay12b: ui foundation allow-list, ports and exports ledger

Executor `lay12b`, follow-up of `result-lay12.md`. Coordinator decisions applied: lay12's two policy exemptions kept (in-source test pairs in `no-circular`, scripts/bootstrap entries in the ui rule); the ui may use an explicit, schema-declared allow-list of foundation modules; every other ui dependency on the framework goes through a ui-owned port or moved to its owner; the cross-package relative imports are recorded in `C/lay12b-tools/exports-needed.json`. No `package.json`/`bun.lock` edit, no cargo, nothing under the frozen storage/revision paths, no modifying git command, no sub-agents. Tools stay in `C/lay12b-tools/`; every generated output was deleted.

## 0. Result (WP12 scope: `🔨️modules` + `📦️packages`, same cruise command as lay12)

| rule | lay12 after | lay12b after |
|---|---:|---:|
| `ui-no-framework-packages` | 174 | **0** |
| `ui-runtime-no-test-support` (new) | - | **0** |
| `ui-foundation-closure` (new) | - | **0** |
| `no-circular` | 12 | 12 (10 frozen process storage, 1 kernel input <-> shard-client index, 1 ui `resident` <-> `pages`) |
| `no-cross-package-relative` | 598 | 636 (see 4) |
| total | 784 | **648** |

The three ui rules were also cruised over ui plus every foundation owner (4,533 dependencies): 0 violations.

## 1. The allow-list: schema-first, in the policy source of truth

- `🔣️taxonomy.json` `dependencyDirections`: two new roles and one new declaration.
  - role `ui-foundation` (runtime allow-list), role `ui-test-support` (test infrastructure and contract corpora), `uiBoundary: { version, subjectRole: "ui", foundationRoles, testSupportRoles }`.
  - Schema `🧬️schema/🧱️dependency-direction/🔣️.json` requires `uiBoundary`; `🔍️discovery/📑️taxonomy-document` and the bootstrap `DependencyPolicyTaxonomy` types carry it; `🚇️audit-dependency-directions` validates it (version 1, known distinct roles, subject excluded, foundation and test support disjoint, paths are exact NFC workspace paths).
- `🏗️construction/🟨️.cjs`: `uiBoundaryRules()` replaces the hand-written ui rule and derives three rules from the roles (role patterns reuse the extracted `dependencyRolePatterns`, so workspace package names under an owner are covered too):
  1. `ui-no-framework-packages`: ui (not bootstrap entries) may depend on the ui, `@semio-tech/ui-*`, foundation and test-support roles only.
  2. `ui-runtime-no-test-support`: test-support roles only from tests, stories (`🧪️tests|🧪️testing/`, `*.test|spec|story.*`) and bootstrap entries.
  3. `ui-foundation-closure`: foundation runtime sources depend only on the ui and other foundation modules (scripts, tests, stories and `🏗️builder/` tooling are composition roots).
  The classification corpus now holds 29 cases for the three rules (14 + 5 + 6 new, 4 existing; each checked against our regex evaluation and dependency-cruiser's `validateDependency`) plus a test that the declaration is required, closed over declared roles, names existing and disjoint owners. `bun test` classification suite: 5 pass, 260 expects; bootstrap, vendored-exclusion and process-providers suites: 29 pass in total (with the cruise config fs-patch, see 6).

Foundation (derived from what the ui actually uses and verified closed by rule 3; the derivation criterion is "pure vocabulary or dependency-free primitive whose runtime sources import only each other and the ui"):

| entry | why |
|---|---|
| `🫧️transient` (+ package entry) | ephemeral state primitives (box/map/set) the shell resets |
| `🌱️value`, `⏳️async/🪃️continuation` (+ entry), `🚪️io/🔤️base64` | data structures (NumericIndex, resident ledger, decode), the continuation and byte codec they close over |
| `🧬️schema/✅️validator`, `🧬️schema/🧾️record` (+ entry) | validator and record admission |
| `🕹️interaction` (+ entry) | selection/merge state machine |
| `🖼️assets` | icon/mesh/delivery vocabulary and catalogs |
| `🛂️manifest/{🏷️ui-contract-and-canvas-types, 🤖️generated (+ entry), 🗺️window-layout, 🎬️action-types-and-semantics, 🔘️definitions-action-ids-and-shell-brand}` | the schema-generated ui contract vocabulary and the pure action-argument types |
| `🧩️action-argument-resolution` | 58 pure lines over those types (staged/default/choice resolution) |
| `🎓️onboarding/🏷️declaration` | the introduction/tutorial declaration types the ui player consumes (new leaf, see 2) |

Not foundation, deliberately: the `@semio-tech/framework` facade barrel (every one of its 31 uses was re-pointed to the precise leaf), `process`, `actor`, `kernel`, `replication`, `manifest` behavior/runtime leaves, `platform`. Identity/hash and i18n primitives are not in the list because the ui does not import them; adding one is a one-line taxonomy edit that rule 3 then verifies.

Test support (`ui-test-support`, tests/stories/bootstrap only): `🧪️test` (+ entry), compiler syntax readers (`📚️compiler/📖️syntax` + entry), the Rust source-closure reader, process script routing/testing/test invocation (the script-router tests), dsl binding-ownership testing, and five contract corpora (job, manifest, replication, tool-machine, trace watchdog tail).

## 2. Ports, moves and leaf extractions

All 174 edges are gone; none of them was silenced.

- Types and pure functions re-pointed to the allow-listed owner (26 import statements in ui): contract number/tree/slider/input types and canvas pick types to the generated contract leaf, `ShellLocale|ShellTerminology` to the package entry `@semio-tech/framework/manifest/generated/ui-axes`, introduction/tutorial types to the new `🎓️onboarding/🏷️declaration` leaf, dialog argument functions to `🧩️action-argument-resolution` and `🔘️definitions-action-ids-and-shell-brand`. `onboarding/🟦️.ts` is now an aggregator of the leaf; manifest's definitions import the leaf.
- `canvasPickTargetKey|parse…|sortCanvasPickTargetsGeneralFirst|pickMostSpecificCanvasTarget|canvasHoverFocusFromTarget`: 5 pure functions moved from `🖥️platform` to the new ui leaf `🖱️ui/🔨️modules/🎯️canvas-pick-targets` (platform imported ui for these anyway); the facade barrel exports them from there, ui-react's adapter exports and `canvas-pick` import the leaf.
- Context menu: `UiContextMenuRequest` and its parts are now owned by the ui scene (`🎬️scene/📋️context-menu`); manifest's `PluginContextMenuRequest|ContextMenuHit|…` are aliases of them. The scene components no longer import manifest.
- Script classes out of ui libraries into composition roots: `GenerateAxesScript|CheckAxesScript|PreviewGeneratedScript` to the new `🖱️ui/🎚️axes/📜️script.ts` (the deleted `🏃️execution` facet); `publishUiAxes(targets, write)` takes the injected writer (the axes producer test passes a plain fs writer); `bootstrapUiAxes` moved to the repository bootstrap (`📚️library/🏗️build/🚀️bootstrap/🎚️ui-axes`, registry JSON re-pointed, same import closure as before so it still runs on a fresh clone; ran it: no-op on the present projections); `StylingPythonGenerateScript|TestScript` to the styling python package script; the deleted `🏗️builder/🟦️.ts` is gone.
- Styling python port: `🔌️provider` is now the pure generic interface `StylingPythonProviderV1<Invocation, Phase>` (no process import). The provider token `STYLING_PYTHON_PROVIDER_V1` and the loaders live in the producer script `🏗️builder/🐍️python/📜️script.ts`; the repository implements the port in `📚️library/🎨️styling/🏗️builder/🐍️python/🔌️provider` with the process types. The process-heavy environment inventory (`observeStylingPythonEnvironmentV1`, its only consumer is the library) moved into the library product. The "producer must not import its test consumer" assertion was kept meaningful (it now names the package script).
- Vite build mode: `semioViteBuild(overrides, policy)` takes an injected `ViteBuildModePolicy {optimize, sourceMap}`; the 8 call sites (storybook, os dev, demonstrator, presentation, hub admin, quiz, play, the web-build test) pass `buildModePolicy(readBuildMode(...))` from the process owner. The ui no longer knows build modes.
- Tests: `process` helpers in ui tests remain only via the declared test-support roles; the onboarding local-interaction case (it tested onboarding, not the ui) moved into the onboarding module as a bun test with immer as oracle (1 pass, 14 expects); the replication type import became `Parameters<typeof …>`; the harness uses the ui's own storage-port instead of the facade; the UIDialog test uses the generated locale list.

## 3. Exports ledger (`C/lay12b-tools/exports-needed.json`, lay6 format plus `scope`, `subpathAlreadyExported`, `removeExports`)

- 155 target files, 308 importer edges for `@semio-tech/framework` (each edge is two cruiser violations: the package and its `-rs` twin): 143 targets / 278 edges are ui-internal (`ui-react` -> `ui/🧱️elements|🔨️modules|…`, `ui locale` -> `axes`), 12 targets / 30 edges are the actor/kernel/surface tests reaching ui seam leaves and a few peers' edges. Each row has the proposed subpath, the imported symbols and all importers; 6 proposed subpaths already exist as exports (switch the import instead).
- `removeExports`: `@semio-tech/ui-styling` `./builder/python/inventory` (target moved to the library product, stub deleted; the export entry is dangling until the manifest batch removes it).
- Relocation was not chosen for the ui-react -> `🧱️elements` edges: `🧱️elements` placement is constrained by taxonomy `elementsDirName`, the manifest batch is the right place.

## 4. Why `no-cross-package-relative` rose 598 -> 636

22 of the +38 violations (11 edges) are mine and intended: 7 ui-react files -> the onboarding declaration leaf, 2 -> `canvas-pick-targets`, 2 -> the generated contract leaf. They replace barrel imports that the new rule forbids and are in the ledger as export entries. The other +16 appeared between the two cruises from concurrent edits of peers (`schema/artifact-reference/*` -> `hash`, `component-receipt`, library `build/mode` -> ui render, kernel/surface tests -> `ui/tests/fixtures`).

## 5. Verification (all from `/Users/ueli/Documents/semio`)

- Cruise: `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs "🧰️framework/🔨️modules" "🧰️framework/📦️packages" --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type json` (via `lay12b-tools/cruise.sh`): table in 0. ui-only and ui+foundation cruises: 0 ui-rule violations.
- tsc, framework/ui/actor/kernel configs: scratch config extending the root, include `🔨️modules/**` + `📦️packages/**` + library styling/providers (the same 45 environment errors as before, `comm` shows 0 new). A second scratch config over `.storybook/main.ts`, library discovery/bootstrap/dependencies/web-build/python-outputs, the os dev builder and the five product vite configs: 0 errors; a deliberate-error probe in that config reported TS2322/TS2554, which also proved the new 2-argument `semioViteBuild`.
- vitest: ui styling config 3 files / 82 tests pass; actor 12 / 107; kernel 10 / 93 (the three seam wiring suites included); ui-react `Slider` (25) and `Stepper` (3) component tests pass after the contract-type re-pointing; `Tree` and `UIDialog` component tests are blocked by the missing `@dnd-kit/*` packages, as in lay12 (type-checked only).
- bun: ui `🎚️axes` 5 pass, ui `🧭️native-command` 17 pass (with the repository's script-invocation envelope), onboarding local-interaction 1 pass, process-providers 4 pass (this one validates the whole taxonomy, including the new audit and the re-sorted `ui-axes` generator inputs), laws `path-emoji-statutes` 37, `empty-folders` 1, `marker-only-folders` 3, `kind-only-basename` 7, `taxonomy-leading-grapheme` 10; `bun build --target=bun 📜️script.ts` bundles (2,463 modules).
- Not run: cargo (forbidden), the bootstrap-parsers suite beyond what passed (two cases fail on the missing `SEMIO_NX_TOOLING_ROOT`/deadline in this shell, unrelated to the registry), `🧪️tests/🧱️dependency-direction` (fails at load on a bun workspace path-capacity refusal for a cad extension package, pre-existing and unrelated).

## 6. Notes for the coordinator

- Peers' mid-flight edits broke the cruise config load twice (a prose key in `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/package.json` `exports`, and `".//…"` export targets in the compiler-syntax-rust-ts and plugin rust package manifests). I did not touch them; `lay12b-tools/patch-fs.cjs` (a `-r` preload) sanitizes `exports` in memory for cruising and for `bun test --preload`. The lint config module path also changed under me to `📚️library/📦️packages/🟦️typescript/📇️inventory/🟨️.cjs` (a peer's change; cruise still works).
- The wgpu frame-worker contract `generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile.sourceModulePaths` (exact list, asserted by `⚡️caching/🧪️tests/🧬️generator-ownership`) is stale by 31 modules in the bundle (`lay12b-tools/framebundle.ts` prints them): two are mine (`🎓️onboarding/🏷️declaration`, `🎯️canvas-pick-targets`), the rest are lay12 seam/sync-protocol leaves and peers' package stubs; the project's `frameWorkerSources` inputs need the same entries. Left for the owner of that contract.
- Follow-up candidate: promote `ui-no-framework-packages`, `ui-runtime-no-test-support`, `ui-foundation-closure` into the strict verify gate (they are at 0 and not in `selectedStrictRules`).
- Still open from lay12: the 10 frozen process storage cycles, kernel input <-> shard-client index (1), ui `💾️resident` <-> `📃️pages` brand pair (1, needs a brand leaf extraction).

## 7. Files

Policy: `📚️library/🔣️taxonomy.json` (roles, `uiBoundary`, `ui-axes` inputs); `📚️library/🧬️schema/🧱️dependency-direction/🔣️.json`; `📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`; `…/🏗️construction/🧪️tests/🚫️resolved-boundary-classification/{🟦️.ts,🧫️fixtures/🔣️.json}`; `…/🚀️bootstrap/🟦️.ts`; `📚️library/🔍️discovery/{📑️taxonomy-document,🚇️audit-dependency-directions}/🟦️.ts`; `📚️library/🏗️build/🚀️bootstrap/🌱️sources/🔣️.json`.

Created: `🖱️ui/🔨️modules/🎯️canvas-pick-targets/🟦️.ts`; `🖱️ui/🎚️axes/📜️script.ts`; `🎓️onboarding/🏷️declaration/🟦️.ts`; `🎓️onboarding/🧪️tests/🏠️local-interaction/🟦️.ts`; `📚️library/🏗️build/🚀️bootstrap/🎚️ui-axes/🟦️.ts`; `📚️library/🎨️styling/🏗️builder/🐍️python/📋️inventory/🟦️.ts` (moved from ui); tools and ledger `C/lay12b-tools/*`.

Deleted: `🖱️ui/🎚️axes/🏃️execution`, `🖱️ui/🎚️axes/🌱️bootstrap`, `🖱️ui/🎨️styling/🏗️builder/🟦️.ts`, `🖱️ui/🎨️styling/🏗️builder/🐍️python/📋️inventory` and its `📦️packages/🟦️typescript` stub.

Edited (ui): the 26 re-pointed import statements in `🧬️contract/{🧩️component,🛡️limits,🖼️presentation/♿️accessibility,🧵️retained/{🏘️instance,📦️wire/🧾️typed,🖼️surface,🛡️validation,🛡️validation/🔬️graph,🟦️.ts},🧪️tests/*}`, `🧱️elements/{🌳️Tree/*,🎚️Slider,🪜️Stepper,📚️I18n,📨️UIDialog/*}`, `🎯️targets/⚛️react/{🌐️i18n,🗣️i18n-terminology,🎓️introduction,🎞️tutorial-engine,🎥️tutorial-ui,🎬️introduction-demo,📐️introduction-geometry,🧭️introduction-resolve,📋️canvas-pick,📤️adapter-exports}`, `📖️stories/🎭️uiintroduction`, `🎬️scene/{📋️context-menu,🧩️components}`, `🎚️axes/📤️publication`, `🎨️styling/🏗️builder/{🐍️python/{📜️script.ts,🔌️provider},🌐️vite/🎚️mode}`, `🎨️styling/📦️packages/{🐍️python/📜️script.ts,🟦️typescript/🏗️builder/🐍️python/🔌️provider}`, `📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`, `🧪️tests/{🎚️axes,🧭️native-command,🧪️owned-locale-detector-retirement/{🧰️harness,📊️tutorial-engine-and-more}}`.

Edited (other): `🖥️platform/🟦️.ts`; `📦️packages/🟦️typescript/🟦️.ts` (barrel line); `🛂️manifest/{📋️app-definition-refresh-and-context-menu,🔘️definitions-action-ids-and-shell-brand}/🟦️.ts`; `🎓️onboarding/🟦️.ts`; library `🎨️styling/🏗️builder/🐍️python/{🟦️.ts,🔌️provider,🧪️tests}`, `🏃️process/🔌️providers/🧪️tests`, `⚡️caching/🧪️tests/{🏗️web-build,🐍️styling-python-outputs}`; vite call sites `.storybook/main.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/{🟦️.ts,🏭️build-owners/🟦️.ts}`, `♻️mit-bestand/{🧺️demonstrator,🎤️präsentation/📅️33.projektetage}/🏗️builder/🌐️vite/🟦️.ts`, `🌎️hub/🔨️modules/🛡️admin/🏗️builder/🌐️vite/🟦️.ts`, `🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts`, `🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts`.
