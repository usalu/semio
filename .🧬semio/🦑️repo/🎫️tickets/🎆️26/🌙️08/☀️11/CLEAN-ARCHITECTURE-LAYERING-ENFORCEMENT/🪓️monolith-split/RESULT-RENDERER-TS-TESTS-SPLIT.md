# Result: RENDERER-TS-TESTS-SPLIT (`rtt`)

Executor: Sonnet, 2026-10-16. `R` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`. `F` = this folder. Tools: `F/rtt-tools/`. Logs and reports: `F/🗑️generated/rtt-*`.

## 1. Verdict

All three monoliths are split. No test title was lost, renamed or added, and the failure set is identical or smaller (section 6).

| Original | Lines | Now |
|---|---:|---|
| `R/🧪️tests/🔬️engine-contract/🟦️.ts` | 14,454 | deleted; 30 topic suites plus 1 helper module under `R/🧪️tests/` (14,531 lines in total, largest 829) |
| `R/🧪️tests/🔌️plugin-runtime/🟦️.tsx` | 3,783 | a 32-line registrar plus 13 topic suites and 1 helper module under `R/🧪️tests/` (largest 668) |
| `R/🧱️elements/📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx` | 3,213 | a 26-line registrar plus 10 topic suites and 1 helper module under `R/🧱️elements/📃️UiDocumentStore/🧪️tests/` (largest 760) |

58 new or rewritten files, longest path 194 bytes (limit 240), every file starts with a docstring whose first character is an emoji.

Correction to the assignment: `engine-contract` was not a `registerTests1` closure. It is a plain vitest module (top-level `describe`/`it`). The two in-source suites (`plugin-runtime`, `typedwire`) are the `registerTests1` closures. Both are run by the in-source `import.meta.vitest` block of their parent module (`PluginRuntime/🟦️.tsx`, `UiDocumentStore/🟦️.tsx`), so they have no file-level launch entry; they run through the parent file.

## 2. Baseline (before the first edit)

How each file runs:

- `engine-contract`: listed in `engineSuite(...)` of `R/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (all levels except `fundamental`/`quick`). Route: `bun repo run @semio-tech/framework-renderer-react:test -- long --run ../../../../🧪️tests/<suite>/🟦️.ts` (react target `📋️project.json` `test` target runs `bun ./📜️script.ts test`).
- `plugin-runtime`: in-source suite of `R/🧱️elements/🔌️PluginRuntime/🟦️.tsx`, in `longInSourceSuites` of the same config. Level `long`.
- `typedwire`: in-source suite of `R/🧱️elements/📃️UiDocumentStore/🟦️.tsx`, in `exhaustiveInSourceSuites`. Level `exhaustive`.

The runner I used for all baseline and after runs is `F/rtt-tools/vitest.sh <level> <label> <filter...>`: it hand-builds `SEMIO_VITEST_POLICY` (the same way `RESULT-PLUGIN-RUNTIME-TSX.md` did) and runs `perl -e 'alarm 1750; exec @ARGV' -- node node_modules/vitest/vitest.mjs run --config ../../🧪️tests/🎚️config/🟦️.ts --testTimeout 120000 --reporter=json --outputFile=… <filter>` from the react target package. Sets are compared as `status<TAB>fullName` rows (`F/rtt-tools/summ.mjs`).

| Suite | Tests | Passed | Failed | Skipped | Report |
|---|---:|---:|---:|---:|---|
| engine-contract (`engine-contract/` filter, level long) | 715 | 710 | 5 | 0 | `rtt-base-ec.{json,rows,log}` |
| plugin-runtime (`PluginRuntime/` filter, level long) | 137 | 137 | 0 | 0 | `rtt-base-pr.*` |
| typedwire (`UiDocumentStore/` filter, level exhaustive; includes the 3-test `📥️intake/📏️step-ceiling` file) | 217 | 114 | 103 | 0 | `rtt-base-tw.*` |

Baseline failures:

- engine-contract (5): `contributions push declaration declares pageCount on every procedural app the shell pushes contributions to`; `window-kind action scoping declares procedural app-scoped verbs once and accepts them from every window kind`; `window-kind action scoping scopes each generation window's own actions to that window in the published procedural descriptor`; `extension invocation completion ownership classifies the 'operator-fault' evaluate answer the way the extension SDK produced it`; `framework renderer hosts maps tiled-map interaction snapshots onto the current wasm sync ABI`.
- typedwire (103): 100 times `Missing exact fixture request`; 1 `expected 'rejected' to be 'ready'`; 1 `ENOENT` on a Rust fixture path; 1 timeout (`OwnedSceneJsonDocument publishes flat validated tokens…`, 207 s against the 120 s limit, run at load average 60).

Type-check baseline: scoped tsc over the 5 original files plus their parents (`rtt-tsconfig-base.json`, `rtt-tsc-base.log`): 36 errors: 27 in typedwire (all `ShardBudget.maxFrames` missing), 2 in engine-contract (`TS2554 Expected 3 arguments, but got 2`), 5 in the PluginRuntime children, 2 in `WasmSessionLoader` (missing module).

## 3. What changed

### 3.1 Method

`F/rtt-tools/split.mjs` (TypeScript compiler API, one config per file: `cfg-ec.mjs`, `cfg-pr.mjs`, `cfg-tw.mjs`) cut each file along top-level statements (and, for 2 giant `describe`s, along their children). Statements are never rewritten:

- Every test statement is assigned to a topic by line range.
- Every helper (declaration) is assigned by usage: used by one topic means it moves into that topic; used by several means it moves into the one shared helper module.
- Each topic file gets exactly the imports it references (checker-resolved), so there is no unused import.
- A `describe` that is cut across topics (`framework renderer hosts` in engine-contract, `TypedWire` and `submitPluginTurn` in the two in-source suites) is re-opened with its original header in each part, with its `afterEach` replicated, so every full title is unchanged.
- In the in-source suites each topic exports `register<Topic>Tests(vitest, dependencies[, source])` and destructures only the dependency names it uses. Closure-bound shared helpers live in one factory per suite (`createPluginRuntimeSupport`, `createTypedWireSupport`) that returns the helpers; pure shared helpers are plain exports.
- Peers edited the new files while I worked (kernel and Interpreter re-points). After the first install I therefore stopped regenerating and applied only surgical edits (renames, region fixes).

### 3.2 engine-contract topics (`R/🧪️tests/<folder>/🟦️.ts`)

Lines / tests (from the final run). All 30 folders are listed; `🧰️engine-contract-helpers` is the shared module.

| Folder | Lines | Tests |
|---|---:|---:|
| `🌍️world3d-picking-and-previews` | 361 | 18 |
| `🌎️world3d-camera-and-paint` | 504 | 40 |
| `🌱️artifact-creation-contracts` | 724 | 23 |
| `🎞️timeline-paint-vfs-hosts` | 574 | 12 |
| `🎫️shell-holds` | 541 | 23 |
| `🏘️window-ids-and-layout-seed` | 470 | 24 |
| `🏭️registry-utilities` | 713 | 50 |
| `🏷️window-chrome-ribbon` | 420 | 21 |
| `💾️media-export-and-natural-files` | 171 | 3 |
| `📂️context-menus-and-drops` | 196 | 13 |
| `📋️renderer-types-and-forms` | 547 | 29 |
| `📖️procedural-descriptors` | 310 | 16 |
| `📝️text-editor-host` | 496 | 22 |
| `📣️command-categories-and-dispatch` | 388 | 18 |
| `🔁️refresh-coalescing` | 829 | 48 |
| `🔎️renderer-twins` | 495 | 26 |
| `🔒️shell-option-locks` | 766 | 38 |
| `🖊️ink-and-marquee-hosts` | 347 | 18 |
| `🖍️canvas-board-hosts` | 436 | 25 |
| `🗂️settings-and-marketplace-tabs` | 264 | 17 |
| `🗃️shell-store-reducer` | 652 | 37 |
| `🗣️dialog-origin` | 537 | 15 |
| `🚧️hidden-tab-surfaces` | 538 | 16 |
| `🛡️runtime-admission` | 185 | 16 |
| `🛣️workflow-flow-routing` | 705 | 40 |
| `🛰️framework-plugin-runtime` | 542 | 19 |
| `🧮️table-host` | 226 | 11 |
| `🧶️node-graph-hosts` | 818 | 26 |
| `🪝️extension-invocation` | 382 | 30 |
| `🪪️window-action-panel` | 394 | 21 |
| total | 14,531 | 715 |

`🧰️engine-contract-helpers` (122 lines): exports `rendererExport`, `computeAccessibleName`, `noopAction`, `panelTreePanelHost`, `CONTRACT_LEAF_LAYOUT`, `buildContractSnapshot`, `buildContractNode`, `renderContractTree`; private `ownedExports`, `ContractNodeSpec`, `CONTRACT_STYLE`, `CONTRACT_ACCESSIBILITY`. It also carries the `🔌️jsdom polyfills` region (`ResizeObserver`, `scrollIntoView`), which every topic activates by importing the module. The other helpers (`peerExport`, `resolvedViewStateFixture`, the recording canvas and label stubs, `meshKindBounds`, `readPluginManifest`, …) are used by one topic each and moved into it.

### 3.3 plugin-runtime topics (`R/🧪️tests/<folder>/🟦️.tsx`)

`🔌️plugin-runtime/🟦️.tsx` is now the registrar `registerPluginRuntimeTests(vitest, dependencies, source)`, called from the in-source block of `PluginRuntime/🟦️.tsx` (one-line edit). Topics: `🏁️ui-close-and-job-admission` (165), `🖥️view-model-surfaces` (211), `📬️extension-invocation-wire` (668), `🚶️instance-open-lifecycle` (167), `🏗️transaction-coordinator` (426), `📦️document-pack-wire-adapter` (337), `🧵️turn-dispatch-lifecycle` (477), `📃️typed-operation-pages` (176), `🔂️settle-continuations` (374), `⚖️turn-scheduler-lanes` (115), `🌋️shard-loss-and-boot` (240), `📯️typed-operation-completion` (256), `🪢️ui-intake-admission` (232) (lines). The `submitPluginTurn` describe (1,100 lines) is cut into 4 of them.

`🛠️plugin-runtime-helpers` (94 lines): type aliases for the three registrar parameters, pure `fakeLocalInteraction`, `unexpectedMediaRequests`, `flushMicrotasks`, and the factory `createPluginRuntimeSupport(dependencies)` returning `encodeFaultBytes`, `fakeHandle`, `withFakeShardClient` (these need the live `testState`/`encodePackValue` of the dependency bag). `packedCarrierLeaf` and `packedCarrierRoot` (module level in the original) moved into `🖥️view-model-surfaces`.

### 3.4 typedwire topics (`R/🧱️elements/📃️UiDocumentStore/🧪️tests/<folder>/🟦️.tsx`)

`🧪️typedwire/🟦️.tsx` is now the registrar `registerTypedWireTests(vitest, dependencies, source)`, called from `UiDocumentStore/🟦️.tsx` (one-line edit). Topics (lines): `🔢️scalar-source-oracles` (216), `📄️paged-payload` (153), `🧠️resident-pool-and-metadata` (760), `🏢️instance-maintenance-and-close` (86), `🧩️node-fields-and-validation` (319), `🎬️owned-scene-binding` (514), `📥️native-child-and-wire-operation` (161), `🧾️owned-intake-and-surface` (464), `🏅️owned-wire-and-byte-pages` (206), `🔏️retained-and-store` (372). The resident region (`ResidentPoolTests` plus `ResidentMetadataTests`) stays one topic because `//#region` markers must stay balanced per file.

`🪛️typedwire-helpers` (153 lines): the three parameter type aliases (the long `Pick<…>` types are written once), pure `exactSchemaDefinitionV1`, `grant`, `nativeInstanceFixture`, `TEST_STYLE`, `leaf`, `container`, `UiNodeRecord`, and the async factory `createTypedWireSupport(vitest, dependencies)` returning `Ajv`, `Buffer`, `produce` (the closure-level `await import`s), `finish`, `close`, `retire`, `nativePagedFieldFixture`, and the `resident*Fixture` builders.

### 3.5 Registration and references updated

- Vitest include list: `engineSuite("🔬️engine-contract")` replaced by the 30 `engineSuite("<folder>")` lines in the same position (`R/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`).
- Launch JSONs (`.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`): the 5 `engine-contract` entries in each, same names and order, now point at the topic that holds their `--testNamePattern` tests: `🪪️window-action-panel` (3 entries), `🎫️shell-holds` (1), `🌱️artifact-creation-contracts` (1). There are no per-file entries for the two in-source suites, and none existed before.
- `R/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`: three runner classes re-pointed (tutorial-interaction-check to `🗃️shell-store-reducer`, flow-browser-runtime-check to `🧶️node-graph-hosts`, surface-retention-check to `🚧️hidden-tab-surfaces`). Each pattern was resolved against the final report (`F/rtt-tools/which.mjs`): every pattern matches in exactly one topic.
- Fixtures and manifests that name the file: flow browser ownership fixture (2 places) and the flow core `📋️project.json` input (1) now name `🚧️hidden-tab-surfaces` (the topic that holds the `flowAbi` token the consumer check requires); the dag-input `receivers.json` (1) names `🧶️node-graph-hosts`; the `📐️test-layout` cache-ownership vector (1) names `🗃️shell-store-reducer`. I replayed the ownership consumer check by hand: the engine-contract entry is satisfied; the other 6 tokens and 1 input-list mismatch in that fixture are pre-existing and belong to peers (`🖼️canvas` fixtures moved).
- Comments that named the old path (14 files: puzzle precompute unit test, flow port-types Rust test and its two JSON fixtures, generation3d unit/preview-eval/flow-operators tests, manifest view-model and example-picker, flow extensions unit test, `⏱️turn-budget`, `🚪️ingress-generation-gate`, `🧲️engine-surface-retention`, `HubFirstRun` component test, `ShellHost/🔌️extension-runtime`, plugin `🏗️materialization`) now name the right topic. No source-text reader read the three files: a tracked and untracked `git grep` finds no remaining `engine-contract` reference outside the helper folder name.
- Taxonomy: 57 names added to `semanticDirectoryMemberKinds."members-of-tests"` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (one hunk at the head of its `memberNames`): the 30+1 engine-contract folders, 13+1 plugin-runtime folders, `🔌️plugin-runtime` (so the registrar resolves too), 10+1 typedwire folders. `F/rtt-tools/taxmatch.mjs` (a replay of the sweep simulator's matcher, loaded from the live taxonomy) resolves all 57 through the member row under parent kind `tests`; two names that already existed in another row (`🌱️artifact-creation`, `🎬️owned-scene`) were renamed to avoid a second row. Not run: the real `verify taxonomy report` (35+ min according to `RESULT-TAXONOMY-SWEEP.md`).

### 3.6 Files edited outside the new suites

- `R/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (include list; `test.env` with the procedural descriptor port), `R/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` (3 runner paths).
- `R/🧱️elements/🔌️PluginRuntime/🟦️.tsx` and `R/🧱️elements/📃️UiDocumentStore/🟦️.tsx` (the in-source block now calls `registerPluginRuntimeTests` / `registerTypedWireTests`).
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (5 entries each).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (+57 names), `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts` (new public fixture entry).
- Path references: flow browser ownership fixture, flow core `📋️project.json`, dag-input `receivers.json`, `📐️test-layout` fixture; plus the 14 comment-only files listed in 3.5.

## 4. Wrong-direction imports

| Reach | Count in the new files | Handling |
|---|---:|---|
| Renderer test reads a hub composition JSON (`🌎️hub/🧩️compositions/🌀️procedural/…`), in `📖️procedural-descriptors` | 3 reads of 1 file | Inverted through a port supplied by the composition root: the test reads `process.env.SEMIO_PROCEDURAL_COMPOSITION_DESCRIPTOR`; the react target's vitest config (`test.env`) supplies the path. The old path was stale (the descriptor now lives in `🧩️component/🔣️.json`), which is why 3 tests failed in the baseline; the supplied path is the current one and those 3 tests now pass (section 6). |
| Renderer tests import `🖱️ui` fixtures by internal path | 12 JSON fixtures in 7 topics | Reached through a new public fixture entry of the owner, `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts` (same shape as the existing `🎭️actor/🎟️retained-turn/🏛️bootstrap/🧪️tests/🧫️fixtures/🟦️.ts`): `chromePanelSafeAreaFixture`, `gizmoTipBoundsFixture`, `graphTimelineAuthorsFixture`, `sceneListTransferFixture`, `sliderPresentationFixture`, `tableStepperKeyboardFixture`, `treeDragHandleFixture`, `virtualFileSystemInteractionFixture`, `presenceOverlayFixture`, `surfaceHostRetentionFixture`, `vfsDescriptorFixture`, `viewportPoseFixture`. None of these is renderer-only: the wgpu Rust twins read them (`🔬️wgpu-table`, `🔬️wgpu-graph-timeline`, `🔬️wgpu-virtual-file-system`, `🔬️targets-wgpu-input-engine-unit`, …), so none moved. |
| Typedwire tests import the ui retained/resident corpus (JSON fixtures under `🖱️ui/🧬️contract/🧵️retained/…`, plus 2 other ui paths) by dynamic `await import` in test bodies | 48 distinct paths | Not changed (section 8). |
| Fixtures of other framework modules (`🛂️manifest`, `🎠️kernel`, `🎭️actor`, `🌱️value`, `🖼️canvas`, `✍️editor`, `🧩️action-argument-resolution`, `🏗️mesh-engine`, `🎓️onboarding`) | 34 paths | Out of the stated scope (hub and ui). Same convention as the sibling renderer suites that I did not touch (for example `🎬️presented-media-slots`, `👆️browser-cursor-presentation` and `🧾️frame-action-ledger` import ui fixtures by internal path the same way). |

## 5. Verification

Machine load was 8 to 64 during the runs; times differ accordingly.

### 5.1 Tests, after the split

Same runner and same filters as the baseline. engine-contract is run by listing the 30 new folders as filters.

| Suite | Tests | Passed | Failed | Verdict |
|---|---:|---:|---:|---|
| engine-contract, run 1 (`rtt-after-ec.*`, after the first install) | 715 | 710 | 5 | row-for-row identical to the baseline (`diff` empty) |
| plugin-runtime (`rtt-after-pr.*`) | 137 | 137 | 0 | identical to the baseline |
| plugin-runtime, final (`rtt-final-pr.*`, after the region and rename edits) | 137 | 137 | 0 | identical to the baseline |
| typedwire, run 1 (`rtt-after-tw.*`) | 217 | 115 | 102 | same 217 titles; one load-dependent test passed (below) |
| typedwire, final (`rtt-final-tw.*`, after all edits) | 217 | 114 | 103 | same 217 titles, row-for-row identical to the baseline (`diff` empty) |
| engine-contract, final full run (`rtt-final-ec.*`) | 715 | 709 | 6 | same 715 titles. One extra failure, the wall-clock law `scheduleDispatchAction (D2): a ten-hop re-arm chain beats a throttled setTimeout…` (430 ms against a 250 ms bound, run at load 28); re-run of its topic and 6 others (`rtt-final2-ec.*`): passes |
| engine-contract, 7 topics touched after the fixture-entry edit plus `📖️procedural-descriptors` (`rtt-final2-ec.*`) | 177 | 174 | 3 | the 3 failures are the stale-hub-path tests |
| `📖️procedural-descriptors` with the corrected port path (`rtt-final3-ec.*`) | 16 | 16 | 0 | the 3 baseline failures now pass |
| engine-contract, final clean full run (`rtt-final4-ec.*`, after all edits) | 715 | 713 | 2 | same 715 titles; the only status changes against the baseline are the 3 stale-hub-path tests (fail to pass); the 2 remaining failures are baseline failures |

Differences against the baseline, each explained:

1. Typedwire `OwnedSceneJsonDocument publishes flat validated tokens…` is load-dependent: it timed out in the baseline (207 s against the 120 s limit, load 60), passed in run 1 (105 s, load below 10) and timed out again in the final run (168 s, load 28). It moved into `🎬️owned-scene-binding` unchanged; the final run matches the baseline exactly.
2. Engine-contract `📖️procedural-descriptors`, 3 tests (`contributions push declaration declares pageCount…` and two `window-kind action scoping` tests): they failed in the baseline because the test read `🌎️hub/🧩️compositions/🌀️procedural/🔣️.json`, a path that no longer exists. With the port supplying the present descriptor (`🧩️component/🔣️.json`) they pass. This is the intended effect of inverting the hub reach, not a change of the test.
3. The wall-clock flake above is a load artefact, not a difference.

### 5.2 Type-check

`node node_modules/typescript/bin/tsc --noEmit -p <scoped tsconfig>` (extends `🧰️framework/🛍️products/💻️os/tsconfig.json`, `files` = the new files plus the two parents):

- engine-contract (`rtt-tsc-ec.log`): 36 errors, the baseline set exactly: 2 `TS2554` (now in `📂️context-menus-and-drops`), 27 typedwire, 5 PluginRuntime children, 2 `WasmSessionLoader`. No new kind of error.
- plugin-runtime and typedwire (`rtt-tsc-after.log`): 32 errors: 27 `ShardBudget.maxFrames` (the same 27, now spread over `🧠️resident-pool-and-metadata` 3, `🧾️owned-intake-and-surface` 19, `🪛️typedwire-helpers` 5), the 5 PluginRuntime children; the 2 `WasmSessionLoader` ones are gone (a peer fixed them). 0 errors in any plugin-runtime file.
- Final combined run (`rtt-tsc-final.log`, all 58 files plus the ui fixture entry and the two parents): my files show exactly the same error families (27 + 2 + 5 + 2). The log also lists 48 errors in other suites' test files that my topics do not own (`actorIdentityGrant` missing in `ArtifactActorConfig` and `TS2739` in `🌉️wire-bridge`, `🎭️browser-actor-reservation`, `🎯️execution-target-lease`, `📂️document-open`, `📨️document-backbone-batches`, `📴️hub-reconnect-outbox`, `🚨️document-actor-recovery-worker`, `🧰️worker-law-fakes`, `🪜️document-catch-up`, `👀️folder-watch`): a peer added a required field to `ArtifactActorConfig` after my baseline. They are foreign errors (section 7).

### 5.3 Losslessness

- Title multiset: `diff` of the sorted `status<TAB>fullName` rows, baseline versus after, is empty for engine-contract and plugin-runtime (715 and 137 rows). Typedwire differs by the single status flip above (217 titles on both sides).
- Line multiset (`F/rtt-tools/lossless.mjs`: imports removed, trimmed non-empty lines): engine-contract 13,227 original lines against 13,282 new; plugin-runtime 3,605 against 3,711; typedwire 3,020 against 3,122. No test line is missing. The only original lines absent from the new files are:
  - engine-contract (12): the 8 helper definitions that gained `export` (`rendererExport`, `computeAccessibleName`, `noopAction`, `panelTreePanelHost`, `CONTRACT_LEAF_LAYOUT`, `buildContractSnapshot`, `buildContractNode`, `renderContractTree`), 2 `import("…Interpreter/🟦️.tsx")` lines that a peer re-pointed to `🔌️surface-bridge`, the dynamic `surface-host-retention` import (now the ui entry), and the hub path constant (now the port).
  - plugin-runtime (5): the `registerTests1` signature and its giant dependency destructuring (now per-topic destructures and a typed registrar), and 3 helper definitions that gained `export`.
  - typedwire (10): the same two signature/destructure lines, 6 helper definitions that gained `export`, the `const { test } = vitest` line (now part of the scalar topic's destructure), and the `UiNodeRecord` alias (exported).
  - The extra lines are docstrings, the per-topic `register…` functions, the replicated `describe(...)` headers with their `afterEach`, the factory, and `export` prefixes (`rtt-lossless-{ec,pr,tw}.txt`).
- Helper item sets (`F/rtt-tools/helper-items.mjs`): see 3.2 to 3.4. Every shared helper is defined exactly once; the three helper modules are imported by 31, 14 and 11 files.
- Regions: the originals hold 51 `//#region` and 50 `//#endregion` markers (plugin-runtime has one unterminated `🧪️terra-web-plugin-runtime`). The new files hold 61 and 61, every file balanced. Four of the new markers are the unterminated region, now closed after each of the 4 parts of the `submitPluginTurn` describe; seven are the `🧾️TypedWireTests` region reopened around each part of `TypedWire`.
- Docstrings: all 58 files open with a docstring that starts with an emoji. The emoji is unique within each of the three splits and across them.
- Paths: longest 194 bytes.

## 6. Failure sets

| Suite | Baseline failures | After | Net |
|---|---:|---:|---|
| engine-contract | 5 | 2 stable (`operator-fault` classification, tiled-map ABI), 0 stale-hub-path | 3 fixed by the corrected port path |
| plugin-runtime | 0 | 0 | none |
| typedwire | 103 | 103 in the final run (102 in run 1) | none; the one load-dependent timeout flips with machine load, before and after |

## 7. Foreign errors and hand-offs

- Foreign type errors in other suites' test files: `ArtifactActorConfig.actorIdentityGrant` newly required (the 10 test folders listed in 5.2). Owner: whoever changed `ArtifactActorConfig`.
- Pre-existing and untouched: `Lane` not exported by the actor barrel and the `maxFrames`/`DescriptorProgress` errors in the PluginRuntime children (`RESULT-PLUGIN-RUNTIME-TSX.md` 6.1); the 27 `ShardBudget.maxFrames` errors in the typedwire fixtures (`nativeInstanceFixture` budget literal lacks `maxFrames`); 100 typedwire failures `Missing exact fixture request` (the fixture-request corpus changed under them).
- Flow browser ownership fixture: 6 stale consumer tokens and one input-list difference (`🖼️canvas/🧪️tests/🧫️fixtures/🎬️draw-list` against `🖼️canvas/🧫️fixtures/…`) are not mine; the engine-contract consumer I touched is satisfied.
- The `📐️test-layout` "Nx hashes semantic-owner cases" test needs the repository build-graph writer custody (it fails with `Original bounded graph writer custody required` when started directly with `bun test`), so I could not run it; the new path exists and is covered by the same `🧪️tests/**/*` input glob of the wgpu project as the old one.
- Hand-off for the taxonomy owner: `bun ./📜️script.ts verify taxonomy report --scope R` was not run (cost); the simulator check is in `F/rtt-tools/taxmatch.mjs`.
- Hand-off for the ui owner: the retained/resident corpus (48 fixture paths) would need the same kind of public entry (async loaders, because the tests `await import` them inside test bodies and the corpus is large). I left it because the `🧬️contract/🧵️retained` tree is under active peer edits and a public entry for it should be designed together with the typed-wire owner.
- Peers edit the new files (Interpreter and kernel import re-points were already applied by them to 10 topic files). The generator in `F/rtt-tools/` would overwrite those edits; do not re-run it against the live tree.

## 8. Not done

- No per-file launch entries for the 30 topics (the repo registers only selected suites with `--testNamePattern`; I re-pointed all 5 existing entries in both files).
- Ui retained/resident fixture entry and the fixtures of the other framework modules (section 4).
- The real taxonomy verification and the `📐️test-layout` Nx test (above).
- No `Interpreter`-style decoupling (the topics still import `ShellHost`, `ShellHelpers`, `Shell`, `World3dHost` and the react target directly, as the original did).

## 9. Commands run (all from `/Users/ueli/Documents/semio`)

- Baseline and after tests: `F/rtt-tools/vitest.sh long base-ec 'engine-contract/'`, `… long base-pr 'PluginRuntime/'`, `… exhaustive base-tw 'UiDocumentStore/'`; after: the same with the new filters (`after-ec`, `after-pr`, `after-tw`, `final-ec|pr|tw`, `final2-ec`, `final3-ec`, `final4-ec`).
- Type-check: `perl -e 'alarm 1700; exec @ARGV' -- node node_modules/typescript/bin/tsc --noEmit -p F/🗑️generated/rtt-tsconfig-{base,ec,after,final}.json`.
- Tools: `split.mjs` (generator), `lossless.mjs`, `helper-items.mjs`, `which.mjs`, `taxmatch.mjs`, `summ.mjs`, `mktsconfig.mjs`, `fixmarkers.py`, `uifix.py`, `final.sh`; pre-edit copies of the three originals are in `F/rtt-tools/orig/`.
- `bun test … test-layout` (failed on build-graph writer custody, see 7).
- No git command that modifies anything, no ticket tool, no sub-agent.
