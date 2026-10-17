# Result: Renderer hosts only UI, third pass (slice RENDERER-HOSTS-UI-3, prefix `rhu3`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. 2026-10-16 20:00 to 21:35. Tools: `rhu3-tools/`. Logs and raw outputs: `🗑️generated/rhu3-*` (nothing deleted, no `[DEBUG]` log added).

STATUS: COMPLETE. Every row below was run.

Aliases: `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`, `RT` = `E/..` (the engine folder), `SDK` = `🧰️framework/📦️packages/🟦️typescript` (package `@semio-tech/framework`), `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`, `FM` = `🧰️framework/🔨️modules`, `F` = this folder.

## 0. Verdict

Every actionable production edge of `renderer-hosts-only-ui` is gone except the 12 r3f edges, which need one policy allow-list line (section 3).

| | Baseline (20:02) | Final (21:24) |
|---|---:|---:|
| all edges (scoped run, exact for this rule) | 391 | **371** |
| production edges | 204 | **185** |
| of which resolver artefact (`react`, `react-dom`) | 127 | 127 |
| of which parent-owned (Actor 34, Media 4) | 38 | 38 |
| of which tooling entries (`📜️script.ts`) | 8 | 8 |
| of which r3f package (policy line, section 3) | 12 | 12 |
| **actionable, not parent-owned, not tooling, not r3f** | **19** | **0** |
| test and story edges | 187 | 186 |

Exact audit cruiser command, full tree (`rhu3-tools/cruise.sh base-full full` 20:01 to 20:19, `final-full full` 21:24 to 21:32, 18485 modules, 51865 dependencies):

| Rule | Baseline | Final | |
|---|---:|---:|---|
| `renderer-hosts-only-ui` | 391 | **371** | -20 (19 production, 1 test) |
| `framework-no-products` | 176 | **176** | not raised; `rhu3-tools/fnp-check.mjs` finds 0 products imports in the three new framework modules |
| `no-circular` | 584 | **568** | -16; the 13 PluginRuntime topic suites no longer form a cycle through the in-source test hook; the renderer scope stays at 297 before and after |
| `not-to-unlisted` | 405 | 405 | |
| `no-cross-package-relative-@semio-tech/framework` | 471 | 471 | my new imports of the three lowered modules use SDK specifiers |
| all violations | 8856 | 8853 | peers' concurrent moves dominate the rest |

Honest side effect: relative imports of the new product composition `OS/🧑‍💻dev/🪝️host-ports` by ✏️s, play and demonstrator add 5 rows to `no-cross-package-relative-@semio-tech/framework-os-dev` (25 to 30) and 3 to `no-cross-technology-✏️s-to-🧰️framework` (172 to 175). A package `exports` entry plus dependency lines would remove them; I did not touch `package.json` dependency lists or lockfiles (hand-off 4).

## 1. Baselines (before the first design edit)

| What | Command | Result |
|---|---|---|
| Rule scan, scoped | `rhu3-tools/cruise.sh base-scoped scoped` (cruiser with the repo policy config over `RT/🎯️targets/⚛️react` and `E`, `--do-not-follow` outside the renderer roots) | 391 edges: 204 production, 187 tests and stories; classification `rhu3-tools/final-table.mjs` |
| Rule scan, exact audit command | `rhu3-tools/cruise.sh base-full full` | 8856 violations; `renderer-hosts-only-ui` 391, `framework-no-products` 176, `no-circular` 584 |
| Type-check, renderer closure | `tsc -p RT/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json --noEmit --incremental false` | 6 errors, all in test files (`rhu3-tsc-baseline.log`), identical to the previous slice |
| Renderer vitest | `rhu3-tools/vitest.sh renderer` (`SEMIO_TEST_LEVEL=long`) | 215 files, 30 failed; 3033 tests, 37 failed, 2996 passed; 46 failure rows (`rhu3-fn-renderer-baseline.txt`) |
| SDK surface law | `bun test ./🧪️tests/🔌️sdk-surface/🟦️.ts` in `SDK` | run after the first SDK entries (below) |

Found while taking the baseline: the react vitest config aliased `@semio-tech/ui-react`, `assets`, `framework-os` and others as string prefixes, so subpath specifiers a peer codemod introduced (`@semio-tech/ui-react/i18n`, `@semio-tech/assets/published-page`) resolved to `<file>/i18n`; the first baseline run failed 159 of 214 files to load (`rhu3-vt-renderer-baseline0-wrong-alias.log`, kept). Fixed before the real baseline, see 4.1.

## 2. What changed (19 production edges)

| Edges | Family | Mechanism | Declared in the renderer | Bound by the os composition root |
|---:|---|---|---|---|
| 3 | hub plugin source factory, `MODULE_EXTENSION_ROUTE`, the `?worker&url` service worker import | port | `PluginDistributionPortV1` in `E/🏛️ShellHost/🪝️host-ports` | `createHubPluginSource`, `HUB_SAME_ORIGIN_MOUNT`, the worker url, `MODULE_EXTENSION_ROUTE` in `OS/🧑‍💻dev/🪝️host-ports/🟦️.ts` |
| 2 | plugin registry root (`admitPluginCatalogV1`, `pluginCatalogV1`, the row types) | neutral types lowered, behaviour a port | `CatalogAdmissionPortV1` | `admitPluginCatalogV1` and `pluginCatalogV1` |
| 2 | access policy (`directoryAccessPermits`, the policy JSON) | port | `DocumentAccessPortV1.permitsDocumentWrite(role)` | the declared policy, parsed once |
| 4 | `engine/🎚️UiPreferences` | module moved into `E/🎚️UiPreferences`, the JSON-bound vocabulary a port | `UiPreferenceVocabularyV1` (`knows`, `keyOf`, `dataClassOf`) | `UI_PREFERENCE_MUTATION_KEYS`, `uiPreferenceMutationDataClassV1` |
| 3 + 1 test | shell state reducer (`defaultShellState`, `reduce`, `ShellCommand`, `ReduceResult`) | pure module lowered into the framework with its generated types | none | none |
| 4 | wasm bindings (`framework_surface.js`, `flow_core.js`, flow browser, node-graph package) | port, module-level registry | `WasmBindingsV1`, `registerWasmBindingsV1` in `E/🪪️WasmSessionLoader` | `OS/🧑‍💻dev/🪝️host-ports/🕸️wasm-bindings/🟦️.ts` |
| 1 | `PluginRuntime` imported its own test aggregator through an in-source hook | hook removed, suite inverted | none | none |

An absent port means the capability is absent, never a built-in default: no hub source and no extension route, nobody may write shared documents (the viewer opens), every preference stays on the device, no catalog is admitted (the boot refuses), the loaders throw `wasm-bindings-absent`.

### 2.1 How the ports reach the elements

- `FrameworkOsBootExecution.hostPorts` (the existing seam) and the new `FrameworkOsShell` prop `hostPorts` carry `ShellHostPortsV1 = { catalogAdmission?, pluginDistribution?, documentAccess?, preferenceVocabulary?, wasmBindings? }`. `useShellCoreState` returns it, so every hook reads `core.hostPorts`.
- `FrameworkOsShell` installs `hostPorts.wasmBindings` in a layout effect, before any passive effect of a child can ask a loader, and retires it on unmount. Installations stack, so the multi-pane demonstrator and play pages work.
- Composition roots now passing the ports: `bootFrameworkOsDev` (`OS/🧑‍💻dev/🟦️.ts`), `✏️s/🧑‍💻dev/🚀️entry/⚛️react/🟦️.ts`, the demonstrator and play pages, the multi-shell harness, and the storybook wasm gate (`.storybook/preview.tsx` installs the wasm bindings once).
- The product side is one file: `createFrameworkOsHostPortsV1()` in `OS/🧑‍💻dev/🪝️host-ports/🟦️.ts` (plus `🕸️wasm-bindings`). The renderer package root re-exports the port types and `registerWasmBindingsV1`.

### 2.2 Lowered neutral types and SDK entries (3 new)

| SDK entry | Facade | Source | Names |
|---|---|---|---|
| `./plugin-module/hub-source/port` | `SDK/🔌️plugin-module/🌎️hub-source/🪝️host-port/🟦️.ts` | `FM/🔌️plugin-module/🌎️hub-source/🪝️host-port` (types cut out of `OS/🔌️plugin/📇️registry/🌎️hub-source`) | `HubPluginSourceNoticeV1`, `HubPluginSourceV1`, `HubProgramModuleV1` |
| `./plugin-catalog` | `SDK/🗂️plugin-catalog/🟦️.ts` | `FM/🗂️plugin-catalog` (types cut out of the registry root) | `PluginCatalogAdmissionV1`, `PluginCatalogRowV1`, `PluginCatalogRowsV1` |
| `./shell` | `SDK/🐚️shell/🟦️.ts` | `FM/🐚️shell` (moved from `OS/🖥️shell/🟦️.ts`) | `defaultShellState`, `reduce`, `ReduceResult`, `ShellCommand`, `ShellState` |

Every importer of the moved or lowered names was updated at once (rewire by name, the static, dynamic and `import()` forms). No shim, no re-export.

### 2.3 Details worth reviewing

- **Registry root: port instead of move.** The previous slice proposed moving root and JSON. The `📇️registry/🧬️schema` folder also holds the descriptor decoder (`🟦️.ts`), which stays, so a move would have split a schema scope. Only the three plain types were lowered; the JSON-bound admission is the port. `resolveFrameworkOsCatalogConfigurationV1` now takes the port first (4 test call sites pass a real one).
- **UiPreferences.** The module moved to `E/🎚️UiPreferences/🟦️.ts` (SDK imports only). The lane functions (`parseUiPreferenceEnvelopeV1`, `queueUiPreferenceLaneV1`, `foldUiPreferenceLaneEventsV1`, `commitUiPreferenceOnLaneV1`, `foldUiPreferenceLanePageV1`) take the vocabulary as first argument. `LOCAL_ONLY_UI_PREFERENCE_VOCABULARY_V1` is what a host without the port uses. The in-source test hook was removed; the two test modules and their fixtures stay under `RT/🎚️UiPreferences/🧪️tests` (a renderer Rust test `include_str!`s the fixture, so it cannot move) and a new runner `RT/🧪️tests/🎚️ui-preferences-suite` registers them with the product vocabulary.
- **Shell reducer.** `OS/🖥️shell/🟦️.ts` and the gitignored typegen mirror `🤖️generated/🟦️.ts` moved to `FM/🐚️shell`. The reducer re-exports the mirror with `export type *`, so nothing needs the generated file at runtime (a fresh checkout can load the SDK entry before typegen). The typegen route follows: `generatedBindingsPath` in `OS/🖥️shell/📦️packages/🦀️rust/📜️script.ts`, the two `📋️project.json` pins (`renderedMirror`, `typegen.outputs`, TS package inputs), the taxonomy `shell-typegen.outputRoots`, the schema-module import and doc lines, the vitest coverage include, and the schema test that reads the mirror. One entry of `🔏️path-emoji-statutes/🔣️.json` named the old mirror path and was removed. Rust and fixtures stay with the OS shell module. **The typegen script was not executed** (it needs cargo); the new output path was checked with `path.join` only.
- **Wasm bindings.** Flow session types are declared by the renderer (`E/🪪️WasmSessionLoader/🌊️flow-session`, the 44 members the renderer calls, found with the type checker, verbatim signatures from the generated declaration); the composition root is type-checked against that shape. `openNodeGraphSessionV1` takes the namespace loader; the NodeGraph runtime contract test now reads the composition file for the `import("@semio-tech/framework-os-node-graph-rs/framework_os_node_graph.js")` text.
- **PluginRuntime.** `pluginRuntimeTestDependenciesV1` is exported; `RT/🧪️tests/🔌️plugin-runtime-suite` calls the 13 topic suites with it (137 tests before and after). `new URL(rel, import.meta.url)` is rewritten by Vite to an `http://localhost` asset url, so the runners build the module url with `pathToFileURL`.

## 3. Policy lines for the parent (not applied)

**r3f, 12 production and 2 test edges.** `@semio-tech/infinite-world-r3f` is a declared workspace dependency whose link `node_modules/@semio-tech/infinite-world-r3f` resolves to its source folder under the os tree, which the rule does not admit. Add to `RENDERER_HOST_ALLOWED_RESOLVED_PATHS` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`:

```
"^🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/",
```

Checked against the final edge list: it matches exactly the 14 r3f edges (all with specifier `@semio-tech/infinite-world-r3f`) and nothing else.

**Tooling, 8 production edges** (the previous hand-off HO-9a: exempt `BOOTSTRAP_TOOLING_ENTRY_PATH` as a `from`): the react target `📜️script.ts` to the process routing, the repo library build and package entries, and the oracle script `RT/🔮️oracles/🎨️world3d-scene-shading/📜️script.ts` (a script, 1900 lines, used only by that entry; moving it would import the tooling stack into the roots), `E/🎞️Scenes/⏱️camera/📦️packages/🦀️rust/📜️script.ts` (2), `E/🕸️NodeGraph/📦️packages/🦀️rust/📜️script.ts` (2).

## 4. Hand-offs of the previous slice

### 4.1 Wasm regex alias, react vitest config (done)

`RT/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`: helpers `exactPackage` and `packageTree`; the four wasm packages now match their whole subpath tree (`packageTree`); `@semio-tech/ui-react`, `assets`, `framework-os`, `framework`, `canvas-react-renderer` and `infinite-world-r3f` match exactly, so subpaths resolve through each package's own `exports` (this also fixes the 159 load failures above). The `ui-styling` directory alias and `ui-react/test` are unchanged. The in-source suite list lost `PluginRuntime` and `UiPreferences`; the engine suites `🔌️plugin-runtime-suite`, `🎚️ui-preferences-suite` and `🪝️host-ports` joined `engineTestSuites`.

### 4.2 Schema catalog (done through the sanctioned route)

`cd /Users/ueli/Documents/semio && bun ./📜️script.ts schema generate` (3560 scopes), then `schema docs` (`📓️schema-catalog.md`, 3560 scopes), then `schema generate --check` passes. Two consecutive generations are byte-identical (sha `862fca6a…`). The catalog was already stale (the markdown said 3754 scopes); the diff is 1917 insertions and 1698 deletions in the JSON, 461 and 660 in the markdown, and includes peers' concurrent moves.

### 4.3 The 12 files that failed to load at 18:28

`F/RESULT-TEST-FIXTURE-ENTRIES.md` does **not exist** (21:35), so I could not wait on it. The 12 files load in the current tree anyway; results after my changes equal the baseline row by row:

| File | Before | After |
|---|---|---|
| retained-toggle-semantics | 3 tests | 3 tests |
| puzzle3d-settings-document | 9 tests, 1 failed | same |
| diff-view-produced-surface | does not load: ENOENT `🖱️ui/🧬️schema/🆚️diff-view-presentation/🔣️.json` | same |
| resident-refresh-budget | 5 tests | 5 tests |
| world3d-camera-framing | 4 tests | 4 tests |
| icon-svg-lighting | 3 tests, 1 failed | same |
| world3d-glb-outline | 4 tests | 4 tests |
| world3d-scene-shading | 3 tests | 3 tests |
| projection-render-parity | 2 tests | 2 tests |
| browser-frame-transport | 62 tests, 6 failed | same |
| ink-canvas-clipboard | 8 tests | 8 tests |
| ink-canvas-editing | 1 test | 1 test |

## 5. Verification (every command and its real result)

| # | Command | Result |
|---|---|---|
| 1 | `rhu3-tools/cruise.sh {base,final}-scoped scoped` | 391 to 371; production 204 to 185; by edge text 20 removed, 0 added (`rhu3-s3-scoped-edges.jsonl`) |
| 2 | `rhu3-tools/cruise.sh {base,final}-full full` (exact audit command) | section 0; `renderer-hosts-only-ui`, `framework-no-products`, `no-circular` as listed |
| 3 | `tsc -p …/tsconfig.json --noEmit --incremental false` (`rhu3-tools/tsc-renderer.sh`) after the edits (s3, s5, s6) | the same 6 test-file errors as the baseline; `diff` of the logs is empty |
| 4 | `sdk2-tools/mk-tsconfig.ts` + `tsc-sem.ts` over 299 touched root files (`rhu3-tools/touched.txt`) | 23 diagnostics, none in a line I wrote: storybook preview stale packages (8), registry composition test (graphlib types, undefined arguments), `host-activation` `row`, `embedded-mount` `graphlib` and `dependencies`; the two I introduced while writing (implicit `any` in the composition) were fixed |
| 5 | renderer vitest after the edits (`rhu3-vt-renderer-s5.log`, final tree) | 215 files, 30 failed; 3043 tests, 37 failed, 3006 passed; **failing set identical to the baseline** (`diff` of the normalized lists is empty). +10 tests are new: 2 preference vocabulary, 1 no-port opening, 2 shell mounts, 5 composition |
| 6 | os vitest (`rhu3-tools/vitest-os.sh`) against the previous slice's final (`dlo-tools/vitest-diff.cjs`) | 609 tests, 499 pass, 110 fail before and after; all 22 files SAME |
| 7 | shell package vitest (`OS/🖥️shell/🧪️tests/🎚️config`) | reduce suite passes; the schema suite fails one test, `set-document-inference-port.json` against the JSON `ShellCommand` (the fixture is newer than the schema JSON, not touched by me); the mirror-path test I had to re-point passes |
| 8 | `bun test ./🧪️tests/🔌️sdk-surface/🟦️.ts` in `SDK` | 6 pass, 0 fail, 780 expects (loads every entry, including the 3 new, in a fresh process; runtime identity of every facade value) |
| 9 | `node rhu3-tools/vite-resolve3.mjs` (a real Vite plugin container, the repo builders' exact alias, importer `E/🏛️ShellHost/🚀️plugin-lifecycle`) | 3 of 3 new specifiers resolve to their facades, 0 unresolved |
| 10 | `bun test ./🧪️tests/🧩️composition/🟦️.ts` in the registry module | 37 pass, 1 fail (`generatorPreviewExecution` refuses the test's `nx:run-commands` executor; a repo library change, not related) |
| 11 | `bun -e` import of `@semio-tech/framework/shell` and one `reduce` call | loads, returns a result |

Runtime proof that the ports work, through renderer tests (all in the identical-failure-set run of row 5):
- `🚪️opening` (the hub's open-plan laws, language-neutral fixture `accessCases`): editor and viewer decided through the real `createFrameworkOsHostPortsV1().documentAccess`, plus a no-port case (every role opens the viewer).
- `🪆️embedded-mount`: the real `bootFrameworkOs` boots with a catalog admission port (the boot refuses without one), `hostPorts.wasmBindings` is installed by the real shell, read by `createRasterSession` and retired on unmount (`createEditorSession` rejects with `wasm-bindings-absent` before and after), and a configured plugin distribution port is not asked for a hub source while no hub is configured. The positive hub call could not be driven: `readViteSEnv` reads `import.meta.env` through a cast Vitest does not expose to `vi.stubEnv`.
- `🪝️host-ports` (new): the product composition answers from the registry admission (and refuses a bad inventory), the declared policy (author writes, spectator does not), the mutation schema (`setTheme` is `persistedShared`) and the module store (a hub source is built when the browser has service worker, cache and locks); 5 tests.
- `🎚️ui-preferences-suite`: the existing canonical and lane suites (6 tests, unchanged) run through the vocabulary, plus a shared change queued and a local-only vocabulary that never queues.
- `🔌️plugin-runtime-suite`: 137 tests as before. The extension-store route (`extensionRoute`) has no React test; it is covered by types only.

## 6. Found state, peers, foreign issues

- No predecessor of this slice. Peers moved the tree under me (package-specifier codemods, fixture moves, taxonomy); every write re-read the file first and the move tools skip a file that changed.
- Foreign, not fixed: three ✏️s catalog suites (`host-activation`, `integration`, `🎮️session`) fail to load (ENOENT `✏️s/🧑‍💻dev/🧩️catalog/🧫️fixtures/🔣️.json`), so my edited calls there are type-checked only; the AgentBridge component suite fails to load (ENOENT `🔗️AgentBridge/🧫️fixtures/🛑️cancellation/🔣️.json`); the os vitest config lists `../🗄️plugin-module-store/🟦️.ts` relative to the wrong folder, so that suite never runs there (under `bun test` it runs 15 pass, 11 fail on `vi.stubGlobal`); `.storybook/preview.tsx` `WASM_LOADERS` name packages that no longer exist.
- I touched the OS shell Rust package only in comments (`🧬️schema/🦀️.rs` doc line) and in its `📜️script.ts` and `📋️project.json` (the typegen route); no renderer Rust.

## 7. Hand-offs

1. Run `bun nx run @semio-tech/framework-os-shell-rs:typegen` and `schema-check` once cargo admission allows; they should write and read `FM/🐚️shell/🤖️generated/🟦️.ts`.
2. Taxonomy sweep for the new names: `FM/🗂️plugin-catalog`, `FM/🐚️shell` (and its `🤖️generated`), `FM/🔌️plugin-module/🌎️hub-source/🪝️host-port`, `SDK/🗂️plugin-catalog`, `SDK/🐚️shell`, `SDK/🔌️plugin-module/🌎️hub-source/🪝️host-port`, `E/🏛️ShellHost/🪝️host-ports`, `E/🪪️WasmSessionLoader/🌊️flow-session`, `OS/🧑‍💻dev/🪝️host-ports` and `🕸️wasm-bindings`, and the engine test folders `🔌️plugin-runtime-suite`, `🎚️ui-preferences-suite`, `🪝️host-ports`.
3. The two policy lines of section 3.
4. Package surfaces: `@semio-tech/framework-os-dev` should export `./host-ports` and list the four wasm packages as dependencies (the renderer package still lists `framework-surface-rs` and `framework-editor-rs` although it no longer imports them); ✏️s, play and demonstrator should then import the composition by specifier. I left `package.json` dependencies and lockfiles alone.
5. The three ✏️s catalog suites, the AgentBridge suite and the os `plugin-module-store` include need their fixture and include paths repaired by their owners; then re-run them.
6. `RESULT-TEST-FIXTURE-ENTRIES.md` was never written; the 12 files already load.

## 8. Files changed

- Renderer elements: `E/🎚️UiPreferences/🟦️.ts` (moved, vocabulary), `E/🏛️ShellHost/🟦️.tsx`, `🔐️identity-binding`, `🧩️shell-core-state`, `🚀️plugin-lifecycle`, `🎨️theme-driver-settings`, `🧭️opening`, `🔒️identity-document-ports`, `⚡️host-effects`, `🏗️dock-assembly`, `🪝️host-ports` (new), `E/🐚️Shell/🟦️.tsx`, `🧩️catalog`, `E/🪪️WasmSessionLoader/🟦️.tsx`, `🌊️flow-session` (new), `E/🕸️NodeGraph/📦️packages/🦀️rust/🟦️.ts`, its runtime test, `E/🔌️PluginRuntime/🟦️.tsx`, `E/🔗️AgentBridge/🟦️.tsx` and its test, `RT/🎯️targets/⚛️react/🟦️.tsx`, the react vitest config.
- Engine tests: `🪆️embedded-mount`, `🚪️opening`, `🎫️shell-holds` (re-pointed), fixture `🎬️boot-execution/🔣️.json` (`hostPorts`), new `🔌️plugin-runtime-suite`, `🎚️ui-preferences-suite`, `🪝️host-ports`.
- Framework: new `FM/🗂️plugin-catalog`, `FM/🔌️plugin-module/🌎️hub-source/🪝️host-port`; moved `FM/🐚️shell/🟦️.ts` and `🤖️generated/🟦️.ts`; `SDK/package.json` (+3 exports) and 3 facades.
- Product: `OS/🔌️plugin/📇️registry/🟦️.ts`, `🧬️schema/🟦️.ts`, `🧪️tests/🧩️composition`, `🌎️hub-source/🟦️.ts`, `OS/🧑‍💻dev/🟦️.ts`, new `OS/🧑‍💻dev/🪝️host-ports` (+ `🕸️wasm-bindings`), the shell module's schema TS, package glue, reduce test, schema test, vitest config, Rust package script and project json, `OS/🧪️tests/🗄️plugin-module-store`, five wgpu files (type specifiers), `✏️s/🧑‍💻dev` entry, catalog and three tests, `🏢️semio-tech/🎡️play/🟦️.tsx` and its pane-coverage test (pins `hostPorts`), `♻️mit-bestand/🧺️demonstrator/🟦️.tsx`, `.storybook/preview.tsx`.
- Pins and generated: taxonomy `shell-typegen.outputRoots`, `🔏️path-emoji-statutes/🔣️.json` (one entry), `📚️library/🔣️schema-catalog.json` and `📓️schema-catalog.md` (regenerated), a doc line in the ui styling vite builder.
- No file deleted. One empty ignored directory (`OS/🖥️shell/🤖️generated`) removed.

## 9. Tools (`rhu3-tools/`)

`cruise.sh`, `edges.mjs`, `final-table.mjs`, `rulecount.sh`, `vitest.sh`, `vitest-os.sh`, `vitest-s.sh`, `tsc-renderer.sh`, `work/vitest.only.config.ts`, `flow-members.mjs`, `sdk-entry.mjs`, `vite-resolve3.mjs`, `fnp-check.mjs`, `touched.txt`, `spec-uipref.json`, `spec-shell.json`, `map-catalog.json` and the copied rhu2 move and rewire tools (`move.ts`, `rewire.mjs`, `pins.mjs`).
