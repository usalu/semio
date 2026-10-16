# 🧱 Result: UI barrel and package imports (slice UI-BARREL-IMPORTS, prefix `uib`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. Logs and raw outputs are in `🗑️generated/uib-*`; tools are in `uib-tools/` (each as `📜️script.ts`, drafts and specs next to them).

## 0. Verdict

- Rule `ui-no-framework-packages`: **256 to 244** edges in the UI-scoped cruise (static oracle: 256 to 244). My change removes **21** of them and moves **1** edge from production code into a `📜️script.ts` (net 0). The other 9 extra edges are new test imports that a peer added meanwhile (`interactivity-*` tests, listed in section 3).
- **Production** edges outside peer areas: **42 to 25** (UI-owned vocabulary moved down, one port injected). The 25 left all need a decision that is not mine: assets, interaction, manifest-generated vocabulary, validator.
- The 207 edges named in P3 can not be fixed by "specific subpath imports": the rule allows no framework path at all (section 1). 93 of the 187 non-peer edges are tooling entries (`📜️script.ts`, vite builder, vitest config) that the policy already exempts in its cross-technology rules but not in this one (decision T).
- Type-check: `tsc -p` ui-react, **51 errors to 50**, 0 new. Vitest ui-react: same failing files, **1162 of 1162 tests pass** (was 1156 of 1157; the one failure was the load-flaky Diagram test). Vitest ui-styling and the bun styling suite: identical failure sets.
- Side effect in other cruiser rules: `no-cross-package-relative-@semio-tech/framework` and `-rs` **+4 each** (116 to 120), from `react` target files importing the two new framework-owned leaves by relative path. No other rule moved because of my files.

## 1. What rule `ui-no-framework-packages` really allows

Built in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs` (loaded by `🧹️lint/🕸️dependency-boundaries/🟨️.cjs`):

```
from:    ^🧰️framework/🔨️modules/🖱️ui/
to:      ^🧰️framework/   or   ^@semio-tech/framework-
pathNot: ^🧰️framework/🔨️modules/🖱️ui/   and   ^@semio-tech/ui-
```

There is no allowed framework subpath. Swapping the root barrel for a "specific module path" (`🧬️schema/✅️validator`, `🖼️assets/…`, `🏃️process/…`) does not remove one edge, because every path under `🧰️framework/` outside the UI folder matches `to`. An edge goes away only when the capability lives inside UI (or in a package named `@semio-tech/ui-*`), or when the composition root hands it in. So each change below is a move of a declaration down into a UI leaf, an injected port, or a named decision to leave the edge with an owner.

`ui-no-framework-packages` has no tooling exemption, while the cross-technology rules of the same policy exempt `BOOTSTRAP_TOOLING_ENTRY_PATH` (`📜️script.ts`, `🏗️builder/🌐️vite/🟦️.ts`, `🧪️tests/🎚️config/🟦️.ts`, `vite|vitest.config.*`).

Oracle: `uib-tools/📊️rule-scan/📜️script.ts` re-implements the rule statically (TypeScript AST plus the workspace package map). On the untouched tree it lists exactly the 256 edges of the cruiser (same pairs, same per-folder counts; the only text difference to the audit file is mangled bytes in the audit copy), so it served for iteration. The cruiser results are in section 6.

## 2. Baseline (before any edit, 2026-10-16 05:25 to 06:00)

| What | Command | Result |
| --- | --- | --- |
| Static rule scan | `bun uib-tools/📊️rule-scan/📜️script.ts` | 256 edges: contract 67, styling 49, elements 36, scene 13, targets 14 (2 wgpu), tests 15, locale 9, modules 9, runtime 8, host 7, render 7, packages 7, viewport 7, axes 4, introduction 3, stories 1 |
| Cruiser, UI scope | `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework/🔨️modules/🖱️ui --do-not-follow '^(?!🧰️framework/🔨️modules/🖱️ui/)' --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err` | exit 171, 683 violations, 1017 modules, 2444 dependencies; `ui-no-framework-packages` 256 (log `uib-depcruise-ui-before.txt`, per-rule `uib-depcruise-ui-before.rules`) |
| Cruiser, exact §1 command | `… dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config … --output-type err` | The parent's audit run of the same day gives `ui-no-framework-packages` 256 (`dda2-depcruise-rule-counts.txt`). My own full runs: the first (05:25) died with `ENOENT … 🧪️test/🟦️.ts` because a peer renamed that module while it ran; the second ran 16 minutes at about 15 percent CPU (load 45 to 60) and I stopped it, since edits could not wait. The UI-scope run above is the same config and the same rule. A final full run is reported in section 6. |
| Type-check | `node_modules/.bin/tsc -p 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json --noEmit` | exit 2, **51** errors (`NodeListOf` iteration, `fixture-placement` missing contract module, `native-command` arity, one cache-service buffer type); log `uib-tsc-react-before.log` |
| Vitest ui-react | `bun uib-tools/🧪️run-vitest/📜️script.ts 🎯️targets/⚛️react/📦️packages/🟦️typescript ../../🧪️tests/🎚️config/🟦️.ts <log>` (explicit `SEMIO_VITEST_POLICY`, node vitest) | 45 files: 4 failed, 41 passed; 1157 tests: 1 failed, 1156 passed. Fail set (`uib-vitest-react-before.fails`): `overlay-flow` (no suite found), `ShellScope` component test (package `color` missing), OS `ShellSearch` test (`framework-os-node-graph-rs` entry missing), Diagram "cursorizes live controlled 20,000-node …" (load flake) |
| Vitest ui-styling | same tool, `🎨️styling/📦️packages/🟦️typescript` | 3 files: 2 failed, 1 passed; 82 tests: 80 passed (`playgroundflowwasmdevstubplugin` and `theme-resolve`, both fixture ENOENT) |
| Bun test styling suite | `bun test ../../🧪️tests/🧩️suite/🟦️.ts` in `🎨️styling/📦️packages/🟦️typescript` | 71 tests, 69 pass, 2 fail (font source identity, chrome contrast oracle) |
| Bun test kernel transient (untouched, for the record) | `bun test ./🧪️tests/🟦️.ts` in `🎠️kernel/🫧️transient` | 5 pass |

Raw `bun repo run @semio-tech/ui-react:test` is blocked on `cache:dependencies-store` (as in RESULT-UI-CYCLES), hence the direct runs.

## 3. Edge census (static oracle, same tree-state tables)

Areas: peer = `🧬️contract` (peer fleet) and `🎯️targets/🧊️wgpu`; parent = `🪟️viewport/📦️packages/🦀️rust/📜️script.ts`. "Tooling" = `📜️script.ts`, `🏗️build(er)/…`, vitest config, axes generator classes.

| Group | Before | After |
| --- | ---: | ---: |
| production (not peer, not tooling, not test) | 42 | 25 |
| tooling (all kinds) | 97 | 97 |
| tests and stories | 42 | 47 |
| peer or parent owned | 75 | 75 |
| total | 256 | 244 |

Per area and target cluster (full table in `uib-edge-table-final.md`):

| Area | Target cluster | Before | After |
| --- | --- | ---: | ---: |
| production | root barrel `@semio-tech/framework` | 23 | 7 |
| production | assets | 12 | 12 |
| production | interaction | 4 | 4 |
| production | manifest | 1 | 1 |
| production | schema validator | 1 | 1 |
| production | process | 1 | 0 |
| tooling `📜️script.ts` | process, other | 80 | 81 |
| tooling build/vite/vitest config | assets, process, validator, root barrel | 13 | 12 |
| tooling generator classes | process | 4 | 4 |
| tests and stories | root barrel | 9 | 5 |
| tests and stories | other (peer's new `interactivity-*` tests add 9) | 15 | 24 |

The 9 new test edges are in `🖌️render/🧪️tests/🔬️interactivity-mounted-prepared-render`, `🧪️tests/🔬️interactivity-mounted-layout-text`, `🧪️tests/🔬️interactivity-mounted-surface-lane` and `🧪️tests/🔬️interactivity-prepared-raster-producer` (edges to `🛍️products/🦑️repo/…/⚖️laws` and `🔍️discovery`). They also explain `framework-no-products` +9, `no-cross-package-relative-@semio-tech/repo-lib` +9 and `no-core-path` +1 in section 6.

## 4. Design decisions

Each decision names where the capability belongs and why.

**D1. Ephemeral lane: a UI-owned twin in `🎨️styling/🫧️ephemeral-state/🟦️.ts`** (16 UI files, 23 import bindings). `ephemeralBox/Map/Set` and `EphemeralBox` were imported from the root barrel by theme, vite builder, react target, 5 modules and 9 elements. The kernel store (`🎠️kernel/🫧️transient`) is an nx project whose file path is pinned in OS renderer closures (wgpu `📋️project.json` inputs, taxonomy `packageGeneration`), so moving it would have touched peer pins. I verified that the two lanes never exchange state: every UI key is `framework.modules.ui.*`, no non-UI file contains that prefix, and `defaultTransientStore.reset()` has no production caller. So UI got its own lane (box, map, set; no weak map, no reset, nothing UI never used), implemented as a class instance (the policy lint `OS-exclusive state authority` forbids module-scope empty `new Map()`), and exports it through `@semio-tech/ui-styling` (`📦️packages/🟦️typescript/🟦️.ts`). Placement in the lowest UI package is deliberate: elements, modules and the react target import the package name, styling-internal files import relatively, so no cross-package relative edge is added (the first attempt under `🔨️modules/` added 5 of them and was reverted). Follow-up, if wanted: the kernel store can import the same container once the OS renderer pins are updated.
 - Test: `🧪️tests/🫧️ephemeral-lane/🟦️.ts` plus fixture `🧪️tests/🧫️fixtures/🫧️ephemeral-lane/🔣️.json`, registered in the ui-react vitest config. It replays the fixture cases and four seeded 400-step random scripts against an independent `lru-cache` reference model and checks that a function init is stored, never called. 5 tests pass.

**D2. Storage port moved to `🔨️modules/🗄️storage-port/🟦️.ts`** (7 UI files, 11 non-UI importers keep working). `StoragePort`, `createBrowserStoragePort`, `createMemoryStoragePort`, `createScopedStoragePort` left `🖥️platform/🟦️.ts` (642 to 528 lines together with D3). The framework barrel `📦️packages/🟦️typescript/🟦️.ts` re-exports them from the leaf (the barrel already aggregates UI scene the same way), `platform` and `kernel` import the type from the leaf, no alias remains at the old place. Behavior is unchanged; the two `catch { /* ignore */ }` bodies became `catch {}` (no comments inside definitions).

**D3. Element ids and dock skeleton moved down.** `windowElementId`, `panelTabElementId`, `panelTabFirstDraggableElementId` (with private `elementIdSegment`) now live in the existing leaf `🔨️modules/🆔️element-identity/🟦️.ts`. `DockSkeleton`, `DockTabSkeleton` and `PersistedAnchor` live in the new React-free leaf `🔨️modules/🐳️dock-skeleton/🟦️.ts` (React-free, because the OS browser bundle must not pull React through the barrel). Barrel exports and the react target re-exports point at the leaves.

**D4. Styling projection gets a writer port.** `generateStylingArtifacts(write)` takes a `StylingArtifactWriterV1`; the composition root, `🎨️styling/📦️packages/🦀️rust/📜️script.ts`, passes `writeGeneratedFileIfChanged`. The edge production to process disappears; the script gains one (tooling category), so the total is unchanged. Runtime proof: calling `generateStylingArtifacts` from bun with a recording writer produced 8 writer calls, all byte-identical to the files on disk, and the styling tree status was unchanged afterwards; `checkStylingArtifacts()` reports the artifacts fresh.

**T. Tooling entries are policy, not code.** 93 of the 187 non-peer edges (175 after) come from `📜️script.ts` (87, of which 6 in the parent-owned viewport script), the vite builder entry (5) and the ui-react vitest config (1); 87 of the 93 import `🏃️process` routing, budget, execution, vitest, leases or artifacts helpers. They can not move into UI without moving the script runtime into UI. Recommended policy line (not applied, because the policy file is admitted and reviewed by another workstream):

```js
from: { path: "^🧰️framework/🔨️modules/🖱️ui/", pathNot: BOOTSTRAP_TOOLING_ENTRY_PATH },
```

The same applies to code that is tooling by nature but sits in other files: `🎨️styling/🏗️builder/🟦️.ts`, `🏗️builder/🌐️vite/🎚️mode`, `🏗️builder/🌐️vite/🕸️wasm/📦️delivery`, `🖌️render/🏗️build/🕸️browser`, `🎚️axes/🏃️execution|📤️publication` (10 edges). Either widen the pattern to `🏗️build(er)?/` and generator `🏃️execution` folders or move those classes into `📜️script.ts`.

### Decisions with an owner and a next step (not done, with reasons)

| Item | Edges | Why not now | Next step |
| --- | ---: | --- | --- |
| Icon catalog (`IconName`, `ICONS`, `isIconName`, metabolism) | 12 prod, 9 test | The catalog is 47 MB of generated assets under `🖼️assets`. Typing `IconName` as `string` would drop the compile-time check of every icon literal. An injected catalog port would force every host to configure it. | Policy: allow `^🧰️framework/🔨️modules/🖼️assets/` in `pathNot` (assets already import styling, so assets sit below UI), or move `🔣️icons` and `🌱️metabolism` under `🖱️ui`. |
| `🕹️interaction` (`MergeMode`, `nextSelection`, `validateState`, `DomainSelection`) | 4 | 326 lines plus Rust twin crate; itself imports assets `IconName` and the manifest-generated `LocalizedLabel`. | Move `🕹️interaction` below UI together with the icon decision, or inject a selection policy port into Tree. |
| Manifest-generated UI vocabulary (`UiNumberLimits`, `UiNumberScale`, `SliderAppearance`, `TreePresentation`, `TreeWindowRowExtent`, `ShellLocale`, `ShellTerminology`, dialog types) | 7 | These types are generated into `🛂️manifest/🤖️generated/📜️ui-contract` and `…/🎚️ui-axes`, not declared in UI. The contract part is the peer area. | Publish the TS twins next to their Rust twins (`🖱️ui/🎚️axes/🤖️generated/🟦️.ts`, `🖱️ui/🧬️contract/…`). Pins to update: taxonomy `generatorContracts["ui-axes"]` and `["ui-contract"]` output roots and two closure lists, `🏗️build/🚀️bootstrap/🌱️sources/🔣️.json`, fixture `🏭️generated-source-topology`, wgpu `dependentTasksOutputFiles`, importers in manifest, kernel, interaction. |
| `UIDialog` (`effectiveActionArgs`, `dialogChoiceArgs`, `ActionArgDef`, `DialogDefinition`) | 1 file | Pure functions over manifest-generated types. | Move `🧩️action-argument-resolution` with the manifest dialog types into UI, or pass the resolver as a prop from the shell. |
| `🎬️scene/🟦️.ts` imports `ActionDescriptor`, `PluginContextMenuRequest` from manifest and `IconName` | 2 | Cycle E of RESULT-UI-CYCLES (manifest imports scene back). | Move the two request types into scene; manifest then imports them. |
| `validateJsonSchemaSubset` (styling verification, vite wasm delivery, render browser build) | 3 prod + 3 tests | 222-line pure leaf with 60 importers; UI should not own a generic validator. `🛡️verification` is used only by repo-lib lint and its tests. | Move `🛡️verification` to `📚️library/🎨️styling`; inject the validator into the build helpers. |
| Asset delivery authority (`SEMIO_ASSET_ROUTE`, `assetPathFromRequest`) | 2 prod + 1 test | Only styling and the play site read it, but `🚚️delivery.json`, its schema and the `🧱️framework-source-topology` fixture pin the assets path. | Move `🌐️delivery`, `🚚️delivery.json` and the schema into styling. |
| Tests and stories (47) | 47 | They need the framework test adapter, fixtures and process vitest helper. | Policy: exempt `🧪️tests/**` and stories, or re-home the fixtures into UI. |

## 5. What changed

Created: `🎨️styling/🫧️ephemeral-state/🟦️.ts`, `🔨️modules/🗄️storage-port/🟦️.ts`, `🔨️modules/🐳️dock-skeleton/🟦️.ts`, `🧪️tests/🫧️ephemeral-lane/🟦️.ts`, `🧪️tests/🧫️fixtures/🫧️ephemeral-lane/🔣️.json` (all under `🧰️framework/🔨️modules/🖱️ui/`).

Edited under `🖱️ui` (imports repointed by `uib-tools/↪️repoint-imports`, formatting of multi-line imports kept): react barrel `🎯️targets/⚛️react/🟦️.tsx`, `🌓️appearance`, `🌐️i18n`, `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (test registered); modules `🆔️element-identity`, `💾️keybinding-persistence`, `🕹️control-keybinding-context`, `🗣️ui-terminology`, `🛟️chrome-safe-area`, `🧲️panel-dock`; elements `🌈️Surface`, `🌳️Tree`, `🎨️Canvas`, `🎬️Scene`, `🐚️ShellScope`, `🔣️Icons`, `🚗️UiDriver`, `📐️Layout`, `🖼️Panel`, `🧭️PanelTabBar` and the component tests of `ShellScope`, `UIDialog`, `Select`, `Window`; tests `🛟️chrome-panel-safe-area`, `🧪️owned-locale-detector-retirement`; styling `🌓️theme`, `🏗️builder/🌐️vite/🟦️.ts`, `📽️projection`, `📦️packages/🟦️typescript/🟦️.ts` (export of the lane), `📦️packages/🦀️rust/📜️script.ts`.

Edited outside `🖱️ui` (each a consumer re-point or the declaring file; no peer logic changed):
- `🖥️platform/🟦️.ts` (declarations removed, two type imports added), `🎠️kernel/🟦️.ts` (one import path), `📦️packages/🟦️typescript/🟦️.ts` (four export lines).
- Closure pins of the OS wgpu browser bundle, because `storage-port` and `element-identity` are now loaded by the barrel: taxonomy `generatorContracts["wgpu-frame-worker"]` `sourceModulePaths` and `inputPatterns` (+2 lines each), wgpu `📦️packages/🟦️typescript/📋️project.json` `frameWorkerSources` (+2 lines). `dock-skeleton` is type-only, so it is correctly not listed.
- Taxonomy rows `members-of-modules` (+`🗄️storage-port`, `🐳️dock-skeleton`) and `members-of-styling` (+`🫧️ephemeral-state`).
- Source-text reader: `🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs` read `🖥️platform/🟦️.ts` for the panel-tab id; it now reads `🆔️element-identity/🟦️.ts` (two lines, Rust not compiled here). The sibling `🗄️browser-prefs-persistence` reader scans `STORAGE_KEY` constants, which did not move.

Stale prose that still names the old place (comments only, peer area): wgpu `🚪️host-io`, `🧭️boot-descriptor`, Shell staging docs.

## 6. Verification after the change

| Check | Before | After |
| --- | --- | --- |
| Static rule scan | 256 | 244 (235 mine, +9 peer tests) |
| Cruiser UI scope, `ui-no-framework-packages` | 256 | **244** (`uib-depcruise-ui-after.txt`, exit 186, 698 violations total) |
| Cruiser UI scope, `no-cross-package-relative-@semio-tech/framework` and `-rs` | 116 each | 120 each (+4: react barrel to `storage-port` and `dock-skeleton`, `appearance` and `i18n` to `storage-port`) |
| `framework-no-products` / `no-cross-package-relative-@semio-tech/repo-lib` / `no-core-path` | 17 / 16 / 6 | 26 / 25 / 7: all added by peer files (the four `interactivity-*` tests, contract scripts, the runtime script), none by mine |
| `no-circular`, `ui-styling`, `ui-react`, `assets` cross-package rules | unchanged | unchanged |
| `tsc -p` ui-react | 51 errors | **50**, 0 new, 1 fixed (a peer's cache-service line); `bun uib-tools/🆚️tsc-diff/📜️script.ts` |
| `tsc` styling set (vite builder, projection, rust script, lane, leaves, barrel, test; 73 UI files) | n/a | 0 errors, files confirmed with `--listFilesOnly` |
| `tsc` over 32 non-UI importers of the moved names (OS shells, demonstrator, play) | n/a | 234 errors, none mention a moved name (all unrelated: `NodeListOf` lib, `Command` generics, `actorIdentityGrant`) |
| Vitest ui-react | 45 files, 4 failed; 1157 tests, 1 failed | 46 files, **3 failed**; **1162 tests, 0 failed**. Fail set is the baseline set minus the load-flaky Diagram test, same reasons (`overlay-flow`, `ShellScope` needs `color`, `ShellSearch` entry) |
| Vitest ui-styling | 2 failed, 80 passed | identical |
| Bun styling suite | 69 pass, 2 fail | identical failures |
| Runtime proof of touched elements (verbose run, `uib-vitest-react-after-verbose.log`) | | Tree 59 + 4, Canvas 14, Layout 4, UIDialog 9, Select 15, Panel 1, Window 12, chrome-panel-safe-area 4, owned-locale-detector-retirement 566 (renders the touched appearance, i18n, UiDriver, Icons, Surface and Scene code), ephemeral-lane 5. `ShellScope` has no runnable test at baseline (package `color` missing); it is type-checked only. |
| OS browser closure oracle `testBrowserInputClosure` (Bun.build load list against `sourceModulePaths`) | | Fails for peer reasons (41 files missing, 1 unused, mostly the kernel split), but none of my four files is missing or unused: `storage-port` and `element-identity` are observed and listed. One UI-related miss is not mine: `🧬️contract/🧵️retained/🧮️tables/🟦️.ts` from RESULT-UI-CYCLES. |
| Styling projection port, direct bun run (`generateStylingArtifacts(spy)`, `checkStylingArtifacts()`) | | 8 writer calls, 8 identical to disk, no tree change; check reports fresh. The launcher route `bun repo run @semio-tech/ui-styling-tokens:check-generated --skip-cache` (06:20) exited 1 with `BUILD_AUTHORITY_CHANGED` on `📋️build-owners.json`: a peer edited the build authority while the launcher inferred its graph, so it never reached the styling target (log `uib-styling-check-generated.log`). Not rerun; the direct bun run above is the evidence. |
| Full §1 cruiser run (`🧰️framework ✏️s 🌎️hub ♻️mit-bestand`, 06:19 to about 06:30) | audit: 256 | **244** `ui-no-framework-packages`; exit 142 is the cruiser's error count, 13454 violations in total over 17151 modules and 46523 dependencies (`uib-depcruise-full-after.txt`). Same number as the UI-scope run and the static oracle. |

## 7. Hand-offs and findings for the parent

1. Decision T above (one line in the dependency policy) clears 93 edges.
2. The OS wgpu closure lists in taxonomy and wgpu `📋️project.json` are stale for peer reasons (kernel split, `🧮️tables`); a peer was updating them while I worked, my additions merged cleanly with theirs.
3. Contract area (not touched): 67 edges; the biggest lever is publishing the generated TS mirrors next to their Rust twins.
4. `no-cross-package-relative-@semio-tech/framework` is violated by every relative import from the react target into `🔨️modules`. A real fix is a package for the UI leaves (`@semio-tech/ui-foundation`), which needs policy registration.
5. The lint `storage-outside-config-lane` names `🖥️platform/` as the only storage adapter; `🗄️storage-port` now holds `localStorage`. The scan skips index files (`🟦️.ts`), so no new breach is reported, but the adapter prefix list in root `📜️script.ts` (line 18361) should list the new leaf.

## 8. Not touched on purpose

`🧬️contract/**` (peer area, 67 edges), `🎯️targets/🧊️wgpu/**` (2 edges), `🪟️viewport/📦️packages/🦀️rust/📜️script.ts` (parent-owned cargo admission, 6 edges), `🎠️kernel/🫧️transient` (kernel lane stays; see D1).

## 9. Tools (kept, ticket-local)

`uib-tools/📊️rule-scan` (static rule oracle), `↪️repoint-imports` (AST rewriter for named imports and re-exports, with specs `spec-*.json`), `📈️edge-table` (area and cluster census), `🧪️run-vitest`, `🆚️tsc-diff`, `drafts/` (leaf drafts, platform cut script, closure inserter, closure runner). All raw output is in `🗑️generated/uib-*` and can be deleted at ticket close.
