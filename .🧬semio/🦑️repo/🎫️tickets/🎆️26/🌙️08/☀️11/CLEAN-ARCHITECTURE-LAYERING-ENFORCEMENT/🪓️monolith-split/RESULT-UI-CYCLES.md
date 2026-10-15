# 🔁 Result: UI import cycles (work package 9)

Scope: `🧰️framework/🔨️modules/🖱️ui/` only. Executor: Sonnet, foreground, no sub-agents. No git command that modifies anything was run. Every number below comes from a command that ran in this session.

## 1. Outcome in numbers

Cruiser rule `no-circular`, command from the brief (`--output-type err` and `json`, config `🕸️dependency-boundaries/🟨️.cjs`), counted per top-level UI folder of the violating `from` file:

| UI folder | before | after |
| --- | ---: | ---: |
| `🧱️elements` | 61 | **0** |
| `🎯️targets` | 35 | **0** |
| `🧪️tests` | 1 | **0** |
| `🎨️styling` | 4 | 1 |
| `🎬️scene` | 2 | 2 |
| `🧬️contract` | 43 | 41 |
| **UI total** | **146** | **44** |
| whole cruise output (`no-circular` lines, incl. files outside UI) | 294 | 191 |

`🧱️elements` and `🎯️targets` are fully cycle-free. The 44 remaining violations are all in loops that pass through framework files outside `🖱️ui` (section 5). No pure-UI loop is left except two that I decided not to touch (section 5, items C and D).

Logs: `🗑️generated/ui-cycles-before.txt`, `ui-cycles-after.txt`, `ui-scc-before.txt`, `ui-scc-after.txt` (strongly connected components computed by my helper from the cruiser JSON).

## 2. Components found (before)

Computed with Tarjan over the cruiser JSON (`🔁️ui-cycles/📜️script.ts`).

| Component | Size | Edges | Content |
| --- | ---: | ---: | --- |
| A, elements and react target | 36 | 138 | `🎯️targets/⚛️react/🟦️.tsx` (8.7k-line barrel) imports 32 elements, and 14 of those elements import back from the barrel (about 125 imported symbols, 91 distinct declared in the barrel). Second loop inside: `🏷️Label` imports `🌳️Tree`, and Tree imports Label. Two in-source test files complete loops with the barrel. |
| B, framework crossing | 40 | 111 | UI `🧬️contract/🧵️retained/*`, `🎬️scene`, `🎨️styling/🌓️theme` with FW `🛂️manifest`, `🎠️kernel`, `🎭️actor/📮️shard-client`, `🖼️assets` and the FW root barrel. |
| C, root self-test registry | 45 | 89 | root `📜️script.ts` imports 34 test files, each of which imports the root script back (4 of them are UI tests). Not a UI loop. |
| small, pure UI | 2 to 3 each | 2 to 5 | `🌓️theme` with `🧪️theme-resolve`; `🏗️builder/🌐️vite` with `🧪️playgroundflowwasmdevstubplugin`; `📽️projection` with `🧪️levels-oklabmix`; `📦️wire/🧾️typed` with its test; `🧵️retained/🟦️.ts` with `🛡️validation/🔬️graph`; `🏘️instance` with `💾️resident` with `📃️pages` |

Important cruiser fact: the config sets `tsPreCompilationDeps: true`, so `import type` edges count. Making an import type-only never removes a cruiser cycle; every cut below moves a declaration instead.

## 3. Cuts made

### 3.1 Elements and react target (components A)

1. **Label to Tree.** Moved `TreeContext`, `TreeRowAlignmentContext`, `TreeAlignedRow`, `PropertyValueColumnContext`, the `detailPanel*` row metrics, `treeItemLabelStyle`, the tree header class constants and their private closure (30 statements) to the new leaf `🧱️elements/🌳️Tree/📏️row-alignment/🟦️.tsx`. Label, Input, Slider, Stepper, Textarea and the Tree tests now import it; Tree imports it too.
2. **Tree hover path.** `resolveHoverRow`, `markGhostTreeInteraction` and their selectors moved to `🧱️elements/🌳️Tree/🧭️hover-path/🟦️.ts` (the panel ghost needs them; Tree keeps using them).
3. **Barrel split.** Everything the elements imported from the barrel and that the barrel itself defined (closure of 91 symbols, 185 statements) moved into modules below the barrel. Layer rule: the elements import these leaves, the barrel imports and re-exports them, no leaf imports the barrel. Barrel public API is unchanged (checked with the TypeScript checker: 1621 exported names before, 1621 after, 0 missing). The barrel shrank from 8748 to 6568 lines.

   | New module | Lines | Holds |
   | --- | ---: | --- |
   | `🔨️modules/⚓️panel-anchor` | 107 | `Anchor` vocabulary and positioning styles |
   | `🔨️modules/🧲️panel-dock` | 87 | dock context, dock targets, tree-unit drag session |
   | `🔨️modules/🎨️chrome-class-names` | 131 | chrome class constants (tabs, drop zones, window bodies) |
   | `🔨️modules/🛟️chrome-safe-area` | 271 | panel safe-area bookkeeping, shell navbar trailing reserve |
   | `🔨️modules/📐️window-measures` | 31 | measure rail sizes, cleared top offset, pane icons |
   | `🔨️modules/🗣️ui-terminology` | 61 | terminology storage, hook, `resolveUiLocalizedText` |
   | `🔨️modules/📱️ui-mobile-context` | 20 | mobile context, provider, hook |
   | `🔨️modules/🗺️window-layout` | 41 | window layout node types, `createEvenWindowLayout` |
   | `🧱️elements/👻️PanelGhost` | 245 | panel ghost, interaction commands, ghost provider and shells, plus the ghost session bridge (`let panelGhostSessionBridge` and its setter, moved out of Canvas so the singleton lives in one module) |
   | `🧱️elements/🖱️ContextMenu/🥣️chrome` | 31 | `ContextMenuChrome` |
   | `🧱️elements/📌️WindowPane` | 328 | `PaneHost`, `Pane`, `PaneResizeHandle` |
   | `🧱️elements/🎮️Engagement` | 328 | engagement pane and its control views |
   | `🧱️elements/🔍️WindowSearch` | 546 | window search line, completion model, key routing |
   | `🧱️elements/🎬️Scene/📐️geometry` | 37 | `Point`, `Vector`, `Plane`, `Camera` |
   | `🧱️elements/🖼️Panel/🌲️tree-source` | 31 | `TreePanelConfig/Definition/Source` (breaks Panel with PanelTabBar) |

   I first created the leaves under `🎯️targets/⚛️react/` and then relocated them below `🔨️modules` and `🧱️elements`, because a lower layer must not live in the target folder that imports it (this also lowered `no-cross-package-relative-@semio-tech/ui-react` from 27 to 15).
4. **`HostThreeCanvas`** is now defined next to `HostReactFlow` in `🧱️elements/🔌️Ports/🟦️.tsx` as `sceneHostPort.fiber.canvas`. The mutable `threeHostPort` stays in the barrel with `configureHostPorts`. Before, `HostThreeCanvas` was `threeHostPort.canvas`, which equals `sceneHostPort.fiber.canvas` at module load, so the value is identical.
5. **Pass-through re-exports.** In 15 element files, imports of symbols that the barrel merely re-exported (theme, UiDriver, Icons, WindowChrome, status-border and so on) now import from the declaring module, or from `@semio-tech/ui-styling` for styling symbols.
6. **In-source tests of the barrel.** `owned-locale-detector-retirement` (11.9k lines) and `icon-render-camera` were run by two `import.meta.vitest` blocks in the barrel that dynamically imported them while the tests type-imported the barrel. The blocks are removed; the two test files import the barrel and register themselves, with a dependency bag that reproduces the former one exactly (barrel namespace plus Diagram, drei cameras, assets, `createBrowserStoragePort`, `createMemoryStoragePort`, `renderToStaticMarkup`, `React`, `THREE`). `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` now lists both files in `include` and no longer has `includeSource`.

### 3.2 Styling in-source tests

`🌓️theme`, `📽️projection` and `🏗️builder/🌐️vite` each had `if (import.meta.vitest)` blocks importing their own test while the test imported the module. Same conversion: the test files self-register; module blocks removed; `🎨️styling/🧪️tests/🎚️config/🟦️.ts` lists the three test files in `include`; the `test-asset-transport` route in `🎨️styling/📦️packages/🟦️typescript/📜️script.ts` now filters on the test file. The `// @vitest-environment node` docblock moved from the vite module to its test (without it the test ran in jsdom and `new URL(..., import.meta.url)` was rewritten by Vite, which broke an imported FW helper).

### 3.3 Contract

`RetainedUiNumericTable` and `RetainedUiSiblingKeys` (classes `Table` and `SiblingKeys`, `Program`, `KeyCell`, 163 lines) moved from `🧬️contract/🧵️retained/🟦️.ts` to the leaf `🧬️contract/🧵️retained/🧮️tables/🟦️.ts`. This breaks `retained/🟦️.ts` with `🛡️validation/🔬️graph`. Importers `graph`, `validation`, `surface`, `operations` updated; the old file keeps no alias.

## 4. Files changed (all under `🖱️ui`)

39 modified files, 198 insertions, 2714 deletions, plus 18 new files (the table above plus `🧮️tables`, `📏️row-alignment`, `🧭️hover-path`). Modified files:

- Barrel and config: `🎯️targets/⚛️react/🟦️.tsx`, `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`.
- Elements: `✏️Input`, `⭕️Ring`, `🌳️Tree` (+ 2 Tree tests), `🎚️Slider`, `🎨️Canvas`, `🎬️Scene`, `🏷️Label`, `📐️Layout`, `📨️UIDialog`, `🔌️Ports`, `🔤️Textarea`, `🖱️ContextMenu`, `🖼️Panel`, `🦴️Skeletons`, `🧭️PanelTabBar`, `🪜️Stepper`, `🪟️Window`.
- Tests: `🧪️tests/📣️engagement-status`, `🖼️icon-render-camera`, `🛟️chrome-panel-safe-area` (.ts and .tsx), `🧪️owned-locale-detector-retirement`.
- Styling: `🌓️theme`, `🏗️builder/🌐️vite`, `📽️projection`, `🧪️tests/🧪️levels-oklabmix`, `🧪️playgroundflowwasmdevstubplugin`, `🧪️theme-resolve`, `🧪️tests/🎚️config`, `📦️packages/🟦️typescript/📜️script.ts`.
- Contract: `🧵️retained/🟦️.ts`, `🖼️surface`, `🛡️validation`, `🛡️validation/🔬️graph`, `🩹️operations`.

The mover rewrote every importer repo-wide (tracked and untracked TS files); all of them were inside `🖱️ui`. Nothing outside `🖱️ui` was edited.

## 5. Remaining loops and why I stopped

All 44 remaining violations (`🧬️contract` 41, `🎬️scene` 2, `🎨️styling/🌓️theme` 1) are loops through files outside `🖱️ui`. I verified this: removing the FW to UI edges leaves only items C and D below.

A. **UI retained contract with the capability friends in FW** (parent-owned Actor/kernel). `🏘️instance`, `💾️resident` and `📃️pages` import `🎭️actor/📮️shard-client` and `🎠️kernel/📤️return/📦️content/📥️input`, and those two import the same three UI files back (`shard-client` to instance and resident; kernel input to instance, resident and pages). Every UI retained file that imports `🛂️manifest` (type-only: `retained/🟦️.ts`, `🛡️validation`, `🔬️graph`, `🖼️surface`, `📦️wire/🧾️typed`, `🏘️instance`) sits on the loop manifest to kernel (type `Effect`) to shard-client to instance to surface/validation to manifest. Cutting it from the UI side alone needs all four UI to FW edge families removed at once (barrel, manifest, shard-client, kernel input); the manifest types would have to move into a UI-owned contract leaf, which means editing `🛂️manifest` and its importers.
B. **FW root barrel.** `🌓️theme` (ephemeral state helpers), `🛡️limits` and `🧩️component` import `@semio-tech/framework`; the barrel re-exports UI `🎬️scene` and `🧵️retained`; `🖼️assets/🔍️resolver` imports `🎨️styling` which imports `🌓️theme`. This is work package 10 (UI stops importing the FW root barrel); importing the deep FW file by relative path instead would only trade this for `no-cross-package-relative` violations.
C. **Friend triple, pure UI.** `🏘️instance` to `💾️resident` to `📃️pages` (4 edges) use mutual `static matches` brand checks and `#private` access. Merging them into one module or inverting with a registered matcher is possible, but all three stay in loop A anyway, so the cruiser count would not move; I left it to avoid changing capability code.
D. **`📦️wire/🧾️typed` with its in-source test.** The module installs `saturationProbe` only when `import.meta.vitest` is defined and the suite is run by the renderer config (`longInSourceSuites`). Converting it means changing production code and a renderer-owned config.
E. **`🎬️scene/🟦️.ts` with `🛂️manifest`.** manifest imports `ContextMenuItemSpec` from UI scene while scene imports `ActionDescriptor` and `PluginContextMenuRequest` from manifest. The offending edge is on the manifest side.
F. **Root self-test registry (component C above).** Four UI tests import the root `📜️script.ts` that imports them; the cruiser reports these with the script as `from`, so they are not in the UI counts.

Suggested follow-ups: move the UI wire types that `🛂️manifest` declares into a UI contract leaf (A), WP10 (B), and have FW stop importing UI scene (E).

## 6. Verification

| Check | Before | After |
| --- | --- | --- |
| `tsc -p` ui-react package tsconfig (`📦️packages/🟦️typescript`) | 52 errors | 49 errors, 0 new (the 3 fewer are a peer's unrelated change in `🏃️process/…/consumption`) |
| `tsc` over 613 files (all UI TS files plus the files outside UI that import UI), my own tsconfig | 205 errors | 205 errors, 0 new |
| vitest `@semio-tech/ui-react` (`bun`-free direct run, explicit `SEMIO_VITEST_POLICY`) | 44 files, 4 failed; 1157 tests, 1156 passed, 1 failed | 45 files, 4 failed; 1157 tests, 1156 passed, 1 failed |
| vitest `@semio-tech/ui-styling` | 3 files, 2 failed; 82 tests, 80 passed, 2 failed | 3 files, 2 failed; 82 tests, 80 passed, 2 failed |

The failing ones are identical before and after and not caused by this work: `overlay-flow` (no suite found), `ShellScope` (package `color` missing), renderer `ShellSearch` (wasm package entry missing), the tutorial test (fixture `🛂️manifest/🧫️fixtures/…` ENOENT), the theme and vite builder tests (fixture ENOENT). The file count is 45 instead of 44 because the barrel's in-source file became two test files. In one full run the Diagram test "cursorizes live controlled 20,000-node …" failed once under machine load; it passed in two isolated reruns (55 of 55).

Extra smoke: renderer config at level `long` with six UI-heavy suites: 4 files and 38 tests passed; 2 files failed with fixture ENOENT (`ChromePanels`, `window-measure-controls`). I have no baseline for this subset.

Commands: tsc with `node_modules/.bin/tsc -p <tsconfig> --noEmit`; vitest with `node node_modules/vitest/vitest.mjs run --config … --testTimeout 120000` and `SEMIO_VITEST_POLICY` set by `🔁️ui-cycles/🧪️run-vitest/📜️script.ts`. Raw `bun repo run @semio-tech/ui-react:test` was blocked on `cache:dependencies-store`.

## 7. Side effects on other cruiser rules (not hidden)

Whole-cruise total of the same command: **1775 violations before, 1732 after** (1418 to 1453 modules cruised, because the new files exist). Per-rule changes of rules that my moved imports touch:

- `no-circular` 294 to 191.
- `no-cross-package-relative-@semio-tech/ui-react` 27 to 15 (elements no longer import the barrel).
- `no-cross-package-relative-@semio-tech/framework` 171 to 186, and its `-rs` twin the same: the barrel now imports about 19 new leaf files by relative path, each a new (from, to) pair of a rule the barrel already violated through the same imports.
- `ui-no-framework-packages` 231 to 240: moved code keeps its existing `@semio-tech/framework` and `@semio-tech/assets` imports, now spread over more files (work package 10 removes them).
- `process` +14, `schema-validator-*` +6, `repo-lib` +3 and `compiler-syntax-*` +2: these come from files under `🛍️products/🦑️repo` and other trees that the cruiser pulls in and that other agents changed meanwhile; none of my files touch them.

No new kind of violation was introduced.

## 8. Helper tools (kept, ticket-local)

All under `🪓️monolith-split/🔁️ui-cycles/`, each as `📜️script.ts` in its own folder: root `📜️script.ts` (SCC with edge dropping), `🔎️origins`, `🔗️edge-symbols`, `🧮️closure`, `🚚️move-declarations` (AST mover with repo-wide importer rewrite; specs in `🗃️specs`, some now stale because I relocated their targets afterwards), `🔀️redirect-reexports`, `🧹️merge-imports`, `🪝️insource-to-testfile`, `🚛️relocate-module`, `♻️recruise`, `🆚️tsc-diff`, `🧪️run-vitest`. Generated output is in `🗑️generated/ui-*`, `uicyc-*` and can be deleted at ticket close.
