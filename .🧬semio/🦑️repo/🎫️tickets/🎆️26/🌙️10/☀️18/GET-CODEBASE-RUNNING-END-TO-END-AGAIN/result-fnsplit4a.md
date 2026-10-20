# Result fnsplit4a (function decomposition under `🧰️framework/**`)

Executor `fnsplit4a` (Sonnet 5.5), foreground, no sub-agents, no modifying git command, no cargo, no `bun repo` command. `C` = this ticket folder. Stopped on the coordinator's TOTAL WRITE PAUSE (no repo write after the pause message; the last batch had finished, no process was running).

## Headline (census `fnsplit2-tools/census.ts --min 300`, `🧰️framework/**` outermost functions)

| measure | before (11:05) | after (pause) |
| --- | ---: | ---: |
| outermost functions > 300 lines | 205 | 146 |
| outermost functions > 600 lines | 4 | 1 (`run` of `🎛️commands/✅️verify`, 656 lines, skipped: file edited < 15 min ago) |

Repo-wide the census now reports 162 outermost > 300 (279 before; part of the delta comes from other shards). 60 framework functions got shorter (table at the end, also `C/fnsplit4a-tools/table.md`); the count drop (205 to 146) also includes concurrent edits by other agents, which are not separated here.

## Functions over 600 (all three done)

| function | before | after root | largest new piece | method |
| --- | ---: | ---: | ---: | --- |
| `planArtifactCratesOnce` (`🏗️authoring/📦️crate-tree/🗺️plan/🧮️pack`) | 899 | 43 | 84 (`resolveSliceEdges`; 21 phase functions in 8 folders, biggest file 205 lines) | two inner `type`s hoisted to module level (`📚️pack-types`), then `psplit2` phases (21 phases, 8 folders), `typeOverrides: {options: "PlanOptions"}` |
| `run` of `TestScript` (`🧪️tests/🔬️typescript-package-commands`) | 894 | 12 | 184 (`runReferenceAuthorityRoutes`) | new `ladder.cjs`: the ladder of ~140 `if (segments…) { …; return; }` blocks became 7 route-group functions `(context, segments) => Promise<boolean>` plus a `TestRouteContext` type (`this.x` -> `context.x`, bare `return;` -> `return true;`, dynamic `import("./…")` specifiers rebased). Lossless text check: normalised token stream of all 7 groups equals the original ladder (69273 tokens each) |
| `applyTaxonomyPlan` (`🧹️normalization/🚀️apply`) | 870 | 20 | 100 (`runTransactionAttempt`, then 6 step functions in `🔧️transaction-steps`) | param reassignment removed (`rawPlan`), 6 `let`s boxed into `txn` (`letbox`), the trailing 273-line `try` extracted with `bx` (range with returns, `paramWrites: ["journal"]`), the remaining body cut into 9 phases (`psplit2`, `allowLets: ["journal"]`, `prepareAttemptJournal` returns `TaxonomyApplyResult | {journal, sourceSet}` so the committed/rolled-back early returns stay behaviour-identical), then the run cut into 6 steps |

Ripple of `applyTaxonomyPlan`: the engine owner list `NORMALIZATION_ENGINE_OWNERS` (`🧹️normalization/🧪️support/🏗️source-services`) got the 7 new owner files in dependency order; three source-text tests read the old single function and now read the phase functions: `🧪️tests/🛫️preflight-reference-basis`, `🛟️transaction-recovery-authority`, `🚚️readme-move-source-authority` (phase-name lists `APPLY_PHASES`). `📦️packages/🟦️typescript/📋️project.json` already globs `🧹️normalization/**/*.ts`.

## Functions between 300 and 600

- 30 module-level `describe` callbacks split with `dsplit.cjs` + new `autod.cjs` (auto spec: parts of <= 165 lines named after the first test title). Each: `verify-expand` LOSSLESS OK (token stream of callback body equals the expansion of the parts) and `tsdiff` CLEAN. Files: artifact-path-projection, mutation-ownership, home-grid, surface-survey, wgpu-accessibility-interaction (one module-level destructuring moved to `🟣️shared` by hand), force-tests, text-editor-host, world3d-camera-and-paint, micro-commit, app-channel-codec, framework-plugin-runtime, layout-tests, Select component, pet-companions, taxonomy-loading, canvas-board-hosts, outbox-delivery, document-backbone-worker-wire, interaction-geometry, package-language-handoff, directory-client-stream, Dialog component, test-hoisting, figure-scroll-and-zoom-rendering, world3d-picking-and-previews, frame-pacing, challenge-clock-views, pets-on-stage, thought-expansion, presence-screens, challenge-session, pet-allowances, Popover component, settings-general-layout, gear-depiction, pet-depiction (list of ids: `C/fnsplit4a-tools/specs/d-*.json`).
- Phase splits with new `autop.cjs` + patched `psplit2` (`pone.sh` auto-reverts when `tsdiff` is not clean or the largest piece stays >= 90 % of the original). Kept: `interactivityLiveReconcileFailures` 547->180, `LayeredOverview` 530->163, `DiagramInner` 550->163, `InteractiveDisposition` 513->155, `useShellUriAndDocuments` 492->184, `usePanelHosting` 491->200, `useSpawnedSession` 483->144, `toolJobFemLiveVisualPublicationExact` 479->217, `toolJobLiveFixedReplayExact` 524->408 (partial), `registerTreeHelpers4AndMoreTests` 567->188, `registerUiintroductionAppearance2AndMoreTests` 562->299, `useArtifactOpening` 459->198, `useTutorialOrchestration` 441->160, `WasmGraphSurface` 427->203, `useDirectoryAndRoute` 427->179, `useDocumentTransferAndTabs` 411->167, `useWorldPointerAndDrop` 390->154, `useCanvasAndFooter` 390->167, `useWorldFitAndPresence` 379->183, `useDirectoryLaneAndLayout` 361->169.

## Tool defects found and fixed (copies in `C/fnsplit4a-tools/`)

- `psplit2`: a `return` nested in a phase statement silently changed behaviour (now an error); no-final-return functions supported; parameter types are taken from the annotation (fixes `typeof import(...)` / `any` unnameable errors); `allowAny`.
- `bx`: `TS_TYPES=node` (without node types every `node:*` value is `any`), `paramWrites` for a reassigned destructured parameter inside a returning range.
- `tsdiff`: `TSDIFF_AT` shows line numbers; `--spec` ignores directories a split did not create.
- `dsplit` verification needs the snapshot with the same extension as the file (a `.ts` snapshot of a `.tsx` file tokenises JSX wrongly); `shape.sh` collides between parallel runs (shared temp files).
- Environment: workspace links such as `@semio-tech/framework-transient` are missing in `node_modules`; tests run with a scratch `--tsconfig-override` (`C/🗑️generated/fnsplit4a-census/ov.json`) that maps that package to its source.

## Verification

- `tsdiff` (distinct compiler diagnostics of the original snapshot vs the new tree, React types via `TSDIFF_REACT=1`): CLEAN for every kept split.
- `comment-check` (all comment lines retained): 0 missing for `pack` and `apply`.
- `dsplit`: `verify-expand` LOSSLESS OK for each; `ladder`: normalised-token equality; `psplit2` register functions: `titles.cjs` (ordered `describe/it/test` titles incl. descent into the new phase functions) identical before/after.
- `shape.sh` (path <= 255 bytes, folder emoji = module doc emoji, unique doc emoji, RGI): problems 0 for `pack`, `apply`, `typescript-package-commands`, `artifact-path-projection`, `mutation-ownership` and others run through `dchk.sh`; the taxonomy resolver step (`tax.ts`) could not run (missing workspace link). Phase splits applied after the last shape run were not shaped (parallel runs collide): run `C/fnsplit2-tools/shape.sh <dir>` over the dirs of the kept phase splits after the pause.
- Owning tests with `bun test ./<file>` (scratch tsconfig override): crate-tree tests (`measure-and-pack`, `module-cuts`, `hard-budget`, `facade-weaving`, `root-aliases`, `test-hoisting`, `docs-and-mounts`, `test-support-wiring`, `virtual-crate`): 80 pass, 0 fail with the new `pack`. Normalization/transaction family (`preflight-reference-basis`, `transaction-recovery-authority`, `readme-move-source-authority`, `pattern-compiler-reuse`, `artifact-path-projection`, `frozen-markdown-coordinates`, workspace-contract taxonomy-normalization x4, rust-physical-reference-context, historical-package-owner-identity, artifact-support-leaf-authority, readme-reviewed-fixture-inputs, cargo-transaction-command-source): failures are exactly the baseline set measured against the original `apply` (Nx-registration tests, physical Git-index oracles, plan-parser grammar, artifact-support-leaf, etc.); the two tests the split broke were fixed. `transaction-v2`: 62 failures before and after (environment), so no functional proof of `applyTaxonomyPlan` here. The describe suites that run at `long` level and the UI/pets suites could not run (missing workspace links); not run.

## Open items

- `run` of `🎛️commands/✅️verify` (656) untouched (edited < 15 min ago); next candidates not processed because of the pause: `plist2.txt` lines 27-49 (`C/🗑️generated/fnsplit4a-census/plist2.txt`).
- Not splittable with the tools without a design step: `inputSchemaReader`, `createCachePolicyTests` (inner 566-line async function, dynamic imports are `any`), `testCanonicalActorAsyncImport`, `stepU`, `ecmaDispositionOracle`, `useShellCoreState` (mutable crossings / declared-later closures), `Paint2dCanvasSurface`, `TreeItem`, `WasmEditorSurface`, `World3dHost`, `applyCommand` (single big statement or exported types), `registerExecutionTargetLeaseTests`, `registerBrowserActorReservationTests`, `registerVendorTransportTests`, `registerSettleContinuationsTests`, `registerHeartbeatAndLivenessTests` (one huge `it`/`describe`; split reverted automatically).
- `🧪️owned-locale-detector-retirement/**` register functions (about 30 x 360-500 lines): the generated sources are structurally broken (statements such as `it(...)`/`expect(...)` at function level, `container` used outside its `it`), so a split cannot be verified; they need regeneration by the tests owner.
- `describe` callbacks not split: mutable module bindings (`pet-handling`, `fixture-lifting`), shared non-const callback locals (`pdf-and-figure-interaction`, `canvas-disposition-interaction`, `morph-ghost`, `media-and-tile-rendering`, `text-splice`), region markers (`Layered` `component` suite), `body-and-gear` (dsplit dry run did not terminate in 500 s), `mutation-leaf-generics` (shared rewrite), second `describe` of `taxonomy-loading` (287 lines).
- Rust functions (`try_build_definition`, `step` x7, `present_step_inner`, `advance`, `lex_with`, ...) not touched: no `rsfn` specs were written; `plugin_exchange` was already done by fnsplit3.
- Tool defect to fix before the next shard: `psplit2` moves root module declarations into the shared file but does not rebase relative `import("…")` type specifiers inside them (observed in `registerArtifactBootstrapRestoreTests`, reverted).

## Files

Changed or created (directories; all new children are `🟦️.ts(x)` in emoji folders next to their root): `…/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🧮️pack/**`, `…/🧹️normalization/🚀️apply/**`, `…/🧹️normalization/🧪️support/🏗️source-services/🟦️.ts`, `…/🧪️tests/🔬️typescript-package-commands/**`, tests `🛫️preflight-reference-basis`, `🛟️transaction-recovery-authority`, `🚚️readme-move-source-authority`, the split describe suites and phase splits listed above (roots plus new child folders). Tools, specs and logs: `C/fnsplit4a-tools/`, `C/🗑️generated/fnsplit4a-census/` (delete `🗑️generated` when done).

## Per-function table (before -> after, framework, shortened or gone)

| function | before | after (largest remaining of that name, "<300" if gone) | file |
| --- | ---: | ---: | --- |
| `planArtifactCratesOnce` | 899 | <300 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🧮️pack/🟦️.ts` |
| `run` | 894 | <300 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️typescript-package-commands/🟦️.ts` |
| `applyTaxonomyPlan` | 870 | <300 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🚀️apply/🟦️.ts` |
| `(callback of describe.if(testLevelAtLeast("long")))` | 595 | <300 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🎨️artifact-path-projection/🟦️.ts` |
| `registerTreeHelpers4AndMoreTests` | 567 | <300 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/📌️tree-helpers-4-and-more/🟦️.tsx` |
| `registerUiintroductionAppearance2AndMoreTests` | 562 | <300 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🎬️uiintroduction-appearance-2-and-more/🟦️.tsx` |
| `DiagramInner` | 550 | <300 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧩️inner/🟦️.tsx` |
| `interactivityLiveReconcileFailures` | 547 | <300 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🖥️mounted/🟦️.ts` |
| `(callback of describe)` | 536 | <300 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🧿️mutation-ownership/🟦️.ts` |
| `LayeredOverview` | 530 | <300 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx` |
| `(callback of describe)` | 526 | <300 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts` |
| `toolJobLiveFixedReplayExact` | 524 | 408 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔁️live-replay/🟦️.ts` |
| `(callback of describe)` | 517 | <300 | `🧰️framework/🛍️products/📚️course/🧪️tests/🏠️home-grid/🟦️.tsx` |
| `InteractiveDisposition` | 513 | <300 | `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/🫳️interaction/🧲️interactive-disposition/🟦️.tsx` |
| `(callback of describe)` | 498 | <300 | `🧰️framework/🛍️products/🐾️pets/🧪️tests/📡️surface-survey/🟦️.tsx` |
| `useShellUriAndDocuments` | 492 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🟦️.ts` |
| `usePanelHosting` | 491 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📋️panel-hosting/🟦️.tsx` |
| `(callback of describe)` | 484 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx` |
| `(callback of describe)` | 483 | <300 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component/🧲️force-tests/🟦️.tsx` |
| `useSpawnedSession` | 483 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🛫️spawned-session/🟦️.ts` |
| `(callback of describe)` | 482 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📝️text-editor-host/🟦️.ts` |
| `(callback of describe)` | 481 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌎️world3d-camera-and-paint/🟦️.ts` |
| `toolJobFemLiveVisualPublicationExact` | 479 | <300 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏗️fem-visual/🟦️.ts` |
| `(callback of describe)` | 479 | <300 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🔬️micro-commit/🟦️.ts` |
| `(callback of describe)` | 478 | <300 | `🧰️framework/🛍️products/💻️os/🧪️tests/🧾️app-channel-codec/🟦️.ts` |
| `(callback of describe)` | 471 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🛰️framework-plugin-runtime/🟦️.ts` |
| `(callback of describe)` | 470 | <300 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component/📐️layout-tests/🟦️.tsx` |
| `(callback of describe)` | 469 | <300 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🧪️tests/🧩️component/🟦️.tsx` |
| `useArtifactOpening` | 459 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📬️artifact-opening/🟦️.ts` |
| `useTutorialOrchestration` | 441 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎥️tutorial-orchestration/🟦️.ts` |
| `(callback of describe)` | 437 | <300 | `🧰️framework/🛍️products/📚️course/🧪️tests/🐾️pet-companions/🟦️.tsx` |
| `(callback of describe)` | 429 | <300 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/📖️taxonomy-loading/🟦️.ts` |
| `useDirectoryAndRoute` | 427 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🛂️directory-route/🟦️.ts` |
| `WasmGraphSurface` | 427 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧭️wasm-graph-surface/🟦️.tsx` |
| `(callback of describe)` | 423 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖍️canvas-board-hosts/🟦️.ts` |
| `(callback of describe)` | 418 | <300 | `🧰️framework/🛍️products/📚️course/🧪️tests/📬️outbox-delivery/🟦️.tsx` |
| `(callback of describe)` | 416 | <300 | `🧰️framework/🛍️products/💻️os/🧪️tests/🪪️document-backbone-worker-wire/🟦️.ts` |
| `useDocumentTransferAndTabs` | 411 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🟦️.tsx` |
| `(callback of describe)` | 402 | <300 | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/📐️interaction-geometry/🟦️.tsx` |
| `(callback of describe)` | 399 | <300 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/📦️package-language-handoff/🟦️.ts` |
| `(callback of describe)` | 394 | <300 | `🧰️framework/🛍️products/💻️os/🧪️tests/🔔️directory-client-stream/🟦️.ts` |
| `useWorldPointerAndDrop` | 390 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/👆️pointer-and-drop/🟦️.ts` |
| `useCanvasAndFooter` | 390 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🖼️canvas-footer/🟦️.tsx` |
| `useWorldFitAndPresence` | 379 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧭️fit-and-presence/🟦️.ts` |
| `(callback of describe)` | 362 | <300 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🧪️tests/🧩️component/🟦️.tsx` |
| `useDirectoryLaneAndLayout` | 361 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📐️layout-hotkeys/🟦️.ts` |
| `(callback of describe)` | 361 | <300 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📦️crate-tree/🪝️test-hoisting/🟦️.ts` |
| `(callback of describe)` | 349 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚙️settings-general-layout/🟦️.ts` |
| `(callback of describe)` | 347 | <300 | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🔍️figure-scroll-and-zoom-rendering/🟦️.tsx` |
| `(callback of describe)` | 345 | <300 | `🧰️framework/🛍️products/🐾️pets/🧪️tests/🧰️gear-depiction/🟦️.tsx` |
| `(callback of describe)` | 344 | <300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌍️world3d-picking-and-previews/🟦️.ts` |
| `(callback of describe)` | 339 | <300 | `🧰️framework/🛍️products/🐾️pets/🧪️tests/⏲️frame-pacing/🟦️.tsx` |
| `(callback of describe)` | 338 | <300 | `🧰️framework/🛍️products/📚️course/🧪️tests/⏱️challenge-clock-views/🟦️.tsx` |
| `(callback of describe)` | 338 | <300 | `🧰️framework/🛍️products/📚️course/🧪️tests/🎪️pets-on-stage/🟦️.tsx` |
| `(callback of describe)` | 332 | <300 | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🪡️thought-expansion/🟦️.ts` |
| `(callback of describe)` | 322 | <300 | `🧰️framework/🛍️products/🐾️pets/🧪️tests/🖌️pet-depiction/🟦️.tsx` |
| `(callback of describe)` | 319 | <300 | `🧰️framework/🛍️products/📚️course/🧪️tests/🪟️presence-screens/🟦️.tsx` |
| `(callback of describe)` | 318 | <300 | `🧰️framework/🛍️products/📚️course/🧪️tests/🏔️challenge-session/🟦️.tsx` |
| `(callback of describe)` | 316 | <300 | `🧰️framework/🛍️products/📚️course/🧪️tests/🎛️pet-allowances/🟦️.tsx` |
| `(callback of describe)` | 305 | <300 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🧪️tests/🧩️component/🟦️.tsx` |