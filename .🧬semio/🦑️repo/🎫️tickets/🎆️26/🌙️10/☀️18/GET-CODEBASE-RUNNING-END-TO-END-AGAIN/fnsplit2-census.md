# Function length census (> 300 lines)

Tool: `fnsplit2-tools/census.ts` (Rust: own lexer over `git ls-files -co`; TS: TypeScript compiler API; `--json` writes the rows). Files scanned (> 300 lines): 4291. Functions found: 348, outermost: 297, outermost > 600: 13. Excluded roots: /node_modules/ /target/ /🗑️generated/ /🎫️tickets/ /.cache/ /dist/.

| lines | kind | name | gen | age min | file:start |
| ---: | --- | --- | --- | ---: | --- |
| 941 | ts-FunctionDeclaration | `registerGisDocumentOpenTests` |  | 1186 | `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🗺️document-open/🟦️.ts:22` |
| 899 | ts-FunctionDeclaration | `planArtifactCratesOnce` |  | 2486 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🧮️pack/🟦️.ts:36` |
| 890 | ts-MethodDeclaration | `run` |  | 79 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts:24` |
| 870 | ts-FunctionDeclaration | `applyTaxonomyPlan` |  | 2478 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🚀️apply/🟦️.ts:36` |
| 773 | ts-FunctionDeclaration | `useShellActions` |  | 60 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🖱️shell-actions/🟦️.ts:121` |
| 759 | rust-fn | `plugin_exchange` |  | 512 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs:77` |
| 710 | ts-FunctionDeclaration | `createGisMapInferenceWorkerV1` |  | 4781 | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/👷️worker/🟦️.ts:16` |
| 688 | ts-FunctionDeclaration | `loadPluginModule` |  | 349 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🔌️plugin-module-loader/🟦️.ts:29` |
| 669 | ts-FunctionDeclaration | `createBrowserBundleTests` |  | 89 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts:6` |
| 663 | ts-FunctionDeclaration | `useApplyHostEffects` |  | 512 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/⚡️host-effects/🟦️.ts:81` |
| 656 | ts-MethodDeclaration | `run` |  | 86 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🟦️.ts:16` |
| 637 | ts-ArrowFunction | `(callback of describe.skipIf(!HUB_E2E))` |  | 1374 | `🌎️hub/🧪️tests/🤝️two-client-document/🟦️.ts:72` |
| 635 | ts-FunctionDeclaration | `useBackboneWorker` |  | 410 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧵️backbone-worker/🟦️.ts:81` |
| 600 | ts-FunctionDeclaration | `registerTests1` |  | 1424 | `✏️s/🧑‍💻dev/📐️cad/🧪️tests/🔮️spatial-kernel/📐️geometry/🧪️tests/🧪️semio-tech-cad-js-core-vec/🟦️.ts:39` |
| 595 | ts-ArrowFunction | `(callback of describe.if(testLevelAtLeast("long")))` |  | 103 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🎨️artifact-path-projection/🟦️.ts:37` |
| 585 | ts-ArrowFunction | `(callback of describe)` |  | 349 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🗃️shell-store-reducer/🟦️.ts:29` |
| 583 | rust-fn | `check_project` |  | 3770 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🏗️project-checks/🦀️.rs:5` |
| 573 | ts-FunctionDeclaration | `registerUiintroductionAppearance2AndMoreTests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🎬️uiintroduction-appearance-2-and-more/🟦️.tsx:5` |
| 572 | ts-FunctionDeclaration | `createCachePolicyTests` |  | 378 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:36` |
| 571 | ts-ArrowFunction | `(callback of describe)` |  | 275 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧪️tests/🖱️input-contract/🟦️.tsx:20` |
| 570 | ts-FunctionDeclaration | `useSpawnedSession` |  | 1396 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🛫️spawned-session/🟦️.ts:81` |
| 568 | ts-FunctionDeclaration | `registerTreeHelpers4AndMoreTests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/📌️tree-helpers-4-and-more/🟦️.tsx:6` |
| 568 | ts-FunctionDeclaration | `registerDocumentBackboneBatchTests` |  | 2407 | `🧰️framework/🛍️products/💻️os/🧪️tests/📨️document-backbone-batches/🟦️.ts:11` |
| 567 | ts-FunctionDeclaration | `usePanelHosting` |  | 1396 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📋️panel-hosting/🟦️.tsx:100` |
| 560 | ts-ArrowFunction | `(callback of describe)` |  | 349 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎞️timeline-paint-vfs-hosts/🟦️.ts:14` |
| 555 | ts-ArrowFunction | `DiagramInner` |  | 2436 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧩️inner/🟦️.tsx:94` |
| 553 | rust-fn | `step` |  | 512 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/🧮️frame-transaction/🦀️.rs:182` |
| 551 | ts-FunctionDeclaration | `inputSchemaReader` |  | 1332 | `🧰️framework/🔨️modules/🛂️manifest/📥️mutation-input-schema/🟦️.ts:161` |
| 547 | ts-FunctionDeclaration | `interactivityLiveReconcileFailures` |  | 2462 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🖥️mounted/🟦️.ts:3` |
| 546 | rust-fn | `vector` |  | 91 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🕸️mutate-fem2d-1-mesh/🦀️.rs:146` |
| 544 | ts-FunctionDeclaration | `useShellCoreState` |  | 512 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧩️shell-core-state/🟦️.ts:169` |
| 540 | rust-fn | `run` |  | 300 | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🏃️run/🦀️.rs:6` |
| 539 | rust-fn | `decode_r2004_object_records` |  | 315 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📥️import/📦️objects/🦀️.rs:46` |
| 539 | ts-ArrowFunction | `(callback of describe)` |  | 416 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🧪️tests/🧩️component/🟦️.tsx:63` |
| 536 | rust-fn | `check_piles_walls_slopes_and_uplift` |  | 3770 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧱️pile-wall-slope-and-uplift-checks/🦀️.rs:6` |
| 536 | ts-ArrowFunction | `(callback of describe)` |  | 1375 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🧿️mutation-ownership/🟦️.ts:32` |
| 534 | ts-ArrowFunction | `(callback of describe)` |  | 4853 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🛰️framework-plugin-runtime/🟦️.ts:8` |
| 534 | rust-fn | `try_build_definition` |  | 2751 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧩️app/🏗️try-build-definition/🦀️.rs:23` |
| 533 | ts-FunctionDeclaration | `Paint2dCanvasSurface` |  | 3653 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/🟦️.tsx:209` |
| 531 | ts-ArrowFunction | `(callback of describe)` |  | 349 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🗣️dialog-origin/🟦️.ts:24` |
| 530 | ts-FunctionDeclaration | `LayeredOverview` |  | 512 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx:43` |
| 527 | ts-FunctionDeclaration | `registerInteractionE2eTests` |  | 1425 | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🎞️e2e/🟦️.ts:20` |
| 526 | ts-FunctionDeclaration | `useShellUriAndDocuments` |  | 60 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🟦️.ts:63` |
| 526 | ts-ArrowFunction | `(callback of describe)` |  | 1924 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts:22` |
| 524 | ts-FunctionDeclaration | `registerArtifactBootstrapRestoreTests` |  | 2407 | `🧰️framework/🛍️products/💻️os/🧪️tests/🧱️artifact-bootstrap-restore/🟦️.ts:13` |
| 524 | ts-FunctionDeclaration | `toolJobLiveFixedReplayExact` |  | 2468 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔁️live-replay/🟦️.ts:93` |
| 521 | ts-ArrowFunction | `InteractiveDisposition` |  | 3028 | `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/🫳️interaction/🧲️interactive-disposition/🟦️.tsx:48` |
| 520 | ts-ArrowFunction | `TreeItem` |  | 2462 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🍃️item/🟦️.tsx:35` |
| 518 | rust-fn | `step` |  | 4164 | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🪜️candidate-step/🦀️.rs:7` |
| 517 | ts-ArrowFunction | `(callback of describe)` |  | 445 | `🧰️framework/🛍️products/📚️course/🧪️tests/🏠️home-grid/🟦️.tsx:155` |
| 516 | ts-FunctionDeclaration | `registerShellComponents8Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/📚️shell-components-8/🟦️.tsx:6` |
| 516 | ts-FunctionDeclaration | `registerExecutionTargetLeaseTests` |  | 3606 | `🧰️framework/🛍️products/💻️os/🧪️tests/🎯️execution-target-lease/🟦️.ts:11` |
| 515 | ts-FunctionDeclaration | `createFreshComponentTests` |  | 1186 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts:18` |
| 511 | rust-fn | `evaluate_beam` |  | 7320 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:97` |
| 511 | ts-FunctionDeclaration | `registerUiintroductionVeilAndElevationTests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🧭️uiintroduction-veil-and-elevation/🟦️.tsx:4` |
| 511 | ts-FunctionDeclaration | `registerUnifiedgumballMathAndMoreTests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🧮️unifiedgumball-math-and-more/🟦️.tsx:9` |
| 510 | ts-FunctionDeclaration | `registerOwnedSceneTests` |  | 275 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🎬️owned-scene-binding/🟦️.tsx:5` |
| 506 | rust-fn | `step_one` |  | 512 | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🧮️retained-input/🦀️.rs:212` |
| 506 | ts-ArrowFunction | `(anonymous)` |  | 512 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:43` |
| 504 | ts-FunctionDeclaration | `WorldInstancesLayer` |  | 2974 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧱️instance-layers/🟦️.tsx:75` |
| 498 | rust-fn | `lex_with` |  | 512 | `🧰️framework/🔨️modules/🗣️dsl/🔍️lexer/🦀️.rs:209` |
| 498 | ts-ArrowFunction | `(callback of describe)` |  | 512 | `🧰️framework/🛍️products/🐾️pets/🧪️tests/📡️surface-survey/🟦️.tsx:138` |
| 497 | ts-FunctionDeclaration | `registerBoxLengthEntryTests` |  | 1425 | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/📦️box-length/🟦️.ts:10` |
| 497 | rust-fn | `step` |  | 2465 | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/⏭️cursor/🚶️cursor-step/🦀️.rs:7` |
| 493 | ts-FunctionDeclaration | `registerTests1` |  | 1420 | `✏️s/🧑‍💻dev/📐️cad/🧪️tests/🔮️spatial-kernel/🧱️brepjs/🧪️tests/🧪️semio-tech-cad-js-brepjs/🟦️.ts:21` |
| 490 | ts-ArrowFunction | `(callback of describe)` |  | 2974 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌎️world3d-camera-and-paint/🟦️.ts:15` |
| 489 | ts-FunctionDeclaration | `registerTreeHelpers1Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🪄️tree-helpers-1/🟦️.tsx:6` |
| 488 | ts-FunctionDeclaration | `registerShellComponents1Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🎨️shell-components-1/🟦️.tsx:6` |
| 487 | ts-FunctionDeclaration | `registerSpaceAdministrationTests` |  | 4508 | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🏘️space-administration/🟦️.ts:10` |
| 485 | ts-FunctionDeclaration | `registerShellComponents7Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🎛️shell-components-7/🟦️.tsx:4` |
| 484 | ts-FunctionExpression | `Slider` |  | 1201 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🟦️.tsx:172` |
| 484 | ts-ArrowFunction | `(callback of describe)` |  | 1186 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx:205` |
| 483 | ts-ArrowFunction | `(callback of describe)` |  | 2430 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component/🧲️force-tests/🟦️.tsx:11` |
| 482 | ts-FunctionDeclaration | `registerShellComponents6Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🧩️shell-components-6/🟦️.tsx:5` |
| 482 | ts-ArrowFunction | `(callback of describe)` |  | 1186 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📝️text-editor-host/🟦️.ts:14` |
| 481 | ts-FunctionDeclaration | `registerTreeHelpers3Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🎯️tree-helpers-3/🟦️.tsx:5` |
| 479 | ts-FunctionDeclaration | `toolJobFemLiveVisualPublicationExact` |  | 4855 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏗️fem-visual/🟦️.ts:7` |
| 479 | ts-ArrowFunction | `(callback of describe)` |  | 2468 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🔬️micro-commit/🟦️.ts:10` |
| 478 | ts-ArrowFunction | `(callback of describe)` |  | 2789 | `🧰️framework/🛍️products/💻️os/🧪️tests/🧾️app-channel-codec/🟦️.ts:17` |
| 477 | ts-ArrowFunction | `(callback of describe)` |  | 349 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌱️artifact-creation-contracts/🟦️.ts:248` |
| 476 | ts-FunctionDeclaration | `registerTreeHelpers2Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🧵️tree-helpers-2/🟦️.tsx:6` |
| 474 | rust-fn | `gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer` |  | 2904 | `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/⚡️quick-and-binding-laws/🦀️.rs:16` |
| 470 | rust-fn | `turn` |  | 4164 | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🔄️initialization-turn/🦀️.rs:15` |
| 470 | ts-ArrowFunction | `(callback of describe)` |  | 2430 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component/📐️layout-tests/🟦️.tsx:10` |
| 470 | ts-FunctionDeclaration | `useArtifactOpening` |  | 60 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📬️artifact-opening/🟦️.ts:81` |
| 469 | ts-FunctionDeclaration | `registerOwnedLocaleDetectorRetirementAndMoreTests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🌳️owned-locale-detector-retirement-and-more/🟦️.tsx:7` |
| 469 | ts-ArrowFunction | `(callback of describe)` |  | 5221 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🧪️tests/🧩️component/🟦️.tsx:35` |
| 468 | ts-FunctionDeclaration | `registerControlChrome1Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/📦️control-chrome-1/🟦️.tsx:6` |
| 468 | ts-FunctionDeclaration | `registerContextmenuAndMoreTests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🧲️contextmenu-and-more/🟦️.tsx:4` |
| 468 | ts-FunctionDeclaration | `registerTurnDispatchLifecycleTests` |  | 349 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧵️turn-dispatch-lifecycle/🟦️.tsx:12` |
| 467 | ts-FunctionDeclaration | `registerShellComponents2Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🔤️shell-components-2/🟦️.tsx:6` |
| 467 | rust-fn | `present_step_inner` |  | 300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🎬️present-step-inner/🦀️.rs:24` |
| 466 | rust-fn | `check_dictionary_typing` |  | 1183 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📚️part4/🦀️.rs:309` |
| 465 | rust-fn | `serve_document_socket` |  | 3213 | `🌎️hub/🏗️bootstrap/🔌️document-socket/🦀️.rs:400` |
| 465 | ts-FunctionDeclaration | `registerTests3` |  | 4459 | `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts:129` |
| 464 | rust-fn | `check_part_2` |  | 1183 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📐️part2/🦀️.rs:12` |
| 464 | ts-FunctionDeclaration | `registerUiintroductionAppearance1Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🪟️uiintroduction-appearance-1/🟦️.tsx:6` |
| 463 | ts-FunctionDeclaration | `registerBrowserActorReservationTests` |  | 1186 | `🧰️framework/🛍️products/💻️os/🧪️tests/🎭️browser-actor-reservation/🟦️.ts:12` |
| 462 | ts-FunctionDeclaration | `useTutorialOrchestration` |  | 392 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎥️tutorial-orchestration/🟦️.ts:47` |
| 461 | ts-FunctionExpression | `WorldInstanceNode` |  | 2974 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧱️instance-layers/🧊️instance-node/🟦️.tsx:20` |
| 460 | ts-FunctionDeclaration | `registerOwnedIntakeAndSurfaceTests` |  | 275 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🧾️owned-intake-and-surface/🟦️.tsx:5` |
| 459 | ts-FunctionDeclaration | `registerShellComponents5Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🗂️shell-components-5/🟦️.tsx:5` |
| 459 | ts-FunctionDeclaration | `useDirectoryAndRoute` |  | 512 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🛂️directory-route/🟦️.ts:59` |
| 458 | rust-fn | `advance` |  | 286 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🚶️hydration-advance/🦀️.rs:10` |
| 454 | ts-FunctionDeclaration | `registerShellComponents3Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🧊️shell-components-3/🟦️.tsx:5` |
| 453 | ts-ArrowFunction | `step23` |  | 5151 | `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🕹️interaction/🟦️.ts:438` |
| 453 | ts-FunctionDeclaration | `registerShellComponents4Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/📐️shell-components-4/🟦️.tsx:4` |
| 447 | ts-FunctionDeclaration | `InteractionRepl` |  | 3782 | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🎛️repl/🟦️.tsx:31` |
| 447 | rust-fn | `turn` |  | 512 | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/📥️snapshot-import/🦀️.rs:199` |
| 446 | ts-FunctionDeclaration | `useContributionsAndRefresh` |  | 3069 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔁️contributions-refresh/🟦️.ts:84` |
| 443 | rust-fn | `direct_owners_descriptors_surfaces_and_catalog_correspond` |  | 3099 | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs:9` |
| 442 | rust-fn | `step_active` |  | 1912 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/⚙️authority-core/🦀️.rs:469` |
| 442 | ts-FunctionDeclaration | `useCanvasAndFooter` |  | 1396 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🖼️canvas-footer/🟦️.tsx:69` |
| 439 | rust-fn | `check_uls_actions` |  | 2 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧱️member-checks/🧮️action-resistance-checks/🦀️.rs:6` |
| 439 | ts-FunctionDeclaration | `useDocumentTransferAndTabs` |  | 3069 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🟦️.tsx:102` |
| 439 | ts-FunctionDeclaration | `WasmGraphSurface` |  | 290 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧭️wasm-graph-surface/🟦️.tsx:67` |
| 438 | ts-FunctionDeclaration | `registerVendorTransportTests` |  | 1186 | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🚚️vendor-transport/🟦️.ts:6` |
| 437 | ts-FunctionDeclaration | `registerControlChrome2Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🛠️control-chrome-2/🟦️.tsx:6` |
| 437 | ts-ArrowFunction | `(callback of describe)` |  | 445 | `🧰️framework/🛍️products/📚️course/🧪️tests/🐾️pet-companions/🟦️.tsx:43` |
| 436 | ts-ArrowFunction | `(callback of describe)` |  | 349 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎫️shell-holds/🟦️.ts:26` |
| 433 | rust-fn | `decode_entity_body` |  | 2809 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📥️import/🧱️records/📐️entities/🦀️.rs:155` |
| 432 | rust-fn | `step` |  | 3045 | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🔨️modules/🏠️host/🧰️owned/🧩️mutation-candidate/🦀️.rs:130` |
| 432 | rust-fn | `from_native` |  | 7369 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/✏️drawing/🦀️.rs:96` |
| 430 | rust-fn | `execute_step` |  | 2499 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🗃️memory/📨️memory-task-executor/🦀️.rs:69` |
| 429 | ts-ArrowFunction | `(callback of describe)` |  | 102 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/📖️taxonomy-loading/🟦️.ts:24` |
| 428 | ts-FunctionDeclaration | `testCanonicalActorAsyncImport` |  | 1186 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/🟦️.ts:25` |
| 425 | ts-ArrowFunction | `(callback of describe)` |  | 5265 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖍️canvas-board-hosts/🟦️.ts:11` |
| 425 | rust-fn | `render_overlay_step` |  | 5202 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🗨️view-overlays/🧭️navbar-footer-and-overlays/🦀️.rs:35` |
| 424 | ts-FunctionDeclaration | `proveGisMapProposalApprovalFixture` |  | 1186 | `🌎️hub/🧪️tests/🌍️gis-map-fixtures/🟦️.ts:261` |
| 424 | ts-ArrowFunction | `(callback of describe)` |  | 4508 | `🧰️framework/🛍️products/💻️os/🧪️tests/🪪️document-backbone-worker-wire/🟦️.ts:12` |
| 423 | ts-FunctionDeclaration | `registerHeartbeatAndLivenessTests` |  | 1186 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/💓️heartbeat-and-liveness-watchdogs/🟦️.ts:8` |
| 422 | rust-fn | `create_generation3d_app` |  | 7019 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs:147` |
| 420 | ts-FunctionDeclaration | `registerTransactionCoordinatorTests` |  | 391 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🏗️transaction-coordinator/🟦️.tsx:7` |
| 420 | ts-ArrowFunction | `stepU` |  | 4459 | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🌐️universal/🟦️.ts:225` |
| 419 | ts-FunctionDeclaration | `useDirectoryLaneAndLayout` |  | 3069 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📐️layout-hotkeys/🟦️.ts:67` |
| 418 | ts-ArrowFunction | `(callback of describe)` |  | 512 | `🧰️framework/🛍️products/📚️course/🧪️tests/📬️outbox-delivery/🟦️.tsx:117` |
| 417 | ts-FunctionDeclaration | `registerControlChrome3AndMoreTests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🧶️control-chrome-3-and-more/🟦️.tsx:8` |
| 413 | rust-fn | `build` |  | 1846 | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🕸️model-graph/🧭️plan/🦀️.rs:50` |
| 413 | rust-fn | `fixture_text` |  | 7319 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪵️mutate-en1995-1/🦀️.rs:108` |
| 413 | ts-FunctionDeclaration | `registerTests1` |  | 1545 | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts:31` |
| 411 | rust-fn | `render_world_3d` |  | 1912 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🎬️render/🦀️.rs:15` |
| 411 | rust-fn | `step` |  | 1525 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🧵️materialize-cursor-step/🦀️.rs:66` |
| 406 | rust-fn | `step_model` |  | 512 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/🧱️model-step/🦀️.rs:264` |
| 404 | ts-FunctionDeclaration | `useWorldPointerAndDrop` |  | 512 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/👆️pointer-and-drop/🟦️.ts:14` |
| 402 | ts-ArrowFunction | `(callback of describe)` |  | 3026 | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/📐️interaction-geometry/🟦️.tsx:12` |
| 402 | rust-fn | `step` |  | 1912 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/⚙️authority-core/🦀️.rs:64` |
| 402 | ts-ArrowFunction | `(callback of describe)` |  | 4508 | `🧰️framework/🛍️products/💻️os/🧪️tests/🔔️directory-client-stream/🟦️.ts:81` |
| 399 | ts-ArrowFunction | `(callback of describe)` |  | 1375 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/📦️package-language-handoff/🟦️.ts:39` |
| 398 | rust-fn | `decode_table_record_object` |  | 7364 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📥️import/📦️objects/🗃️tables/🦀️.rs:64` |
| 398 | rust-fn | `publish_mounted_typed_operation_unit` |  | 512 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs:6` |
| 397 | ts-ArrowFunction | `(callback of describe)` |  | 79 | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/✂️text-splice/🟦️.ts:13` |
| 397 | ts-FunctionDeclaration | `WasmEditorSurface` |  | 512 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx:38` |
| 395 | rust-fn | `step` |  | 512 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🔁️mounted-state-step/🦀️.rs:7` |
| 395 | rust-fn | `fixture_text` |  | 7352 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔩️mutate-en1993-1/🦀️.rs:121` |
| 394 | ts-FunctionDeclaration | `registerTests1` |  | 1186 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️actorreturnresponseframing-uses-canonical-vectors-with-no-payload-copies/🟦️.ts:5` |
| 393 | ts-ArrowFunction | `(callback of describe)` |  | 349 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🏭️registry-utilities/🟦️.ts:20` |
| 393 | ts-FunctionDeclaration | `useWorldFitAndPresence` |  | 512 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧭️fit-and-presence/🟦️.ts:24` |
| 393 | ts-FunctionDeclaration | `registerTests1` |  | 349 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx:7` |
| 388 | ts-FunctionDeclaration | `registerShellComponents10Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🔎️shell-components-10/🟦️.tsx:4` |
| 388 | ts-FunctionDeclaration | `registerUidriverAndMoreTests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🔗️uidriver-and-more/🟦️.tsx:8` |
| 386 | ts-FunctionDeclaration | `registerIntroductiondemoarcpointAndMoreTests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🎞️introductiondemoarcpoint-and-more/🟦️.tsx:4` |
| 384 | ts-FunctionDeclaration | `registerTests1` |  | 349 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🧪️unknown-component-placeholder/🟦️.tsx:3` |
| 383 | ts-FunctionDeclaration | `registerStatechartKernelAndModelSuites` |  | 123 | `🧰️framework/🧪️tests/🧪️docklayoutstore/🎰️machines/⚙️statechart-kernel-and-model-suites/🟦️.ts:6` |
| 382 | rust-fn | `turn` |  | 3090 | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🔨️modules/🏠️host/🧰️owned/🚀️store-initialization/🦀️.rs:42` |
| 380 | ts-FunctionDeclaration | `registerTests1` |  | 402 | `🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🧪️tests/🔬️unit/🟦️.ts:9` |
| 375 | ts-ArrowFunction | `(callback of describe)` |  | 21756 | `🧰️framework/🛍️products/🐾️pets/🧪️tests/🤏️pet-handling/🟦️.tsx:133` |
| 374 | ts-ArrowFunction | `(callback of receiveScriptProcessInvocation)` |  | 1380 | `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🧩️plugin-coverage/🟦️.ts:41` |
| 373 | ts-FunctionDeclaration | `ecmaDispositionOracle` |  | 1490 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts:124` |
| 371 | rust-fn | `evaluate_zone` |  | 3770 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🏠️zone-evaluation/🦀️.rs:5` |
| 371 | ts-FunctionDeclaration | `World3dHost` |  | 391 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:65` |
| 369 | rust-fn | `vector` |  | 91 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🏋️mutate-fem2d-1-load/🦀️.rs:124` |
| 369 | ts-FunctionDeclaration | `FrameworkOsShellInner` |  | 1259 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:226` |
| 368 | rust-fn | `every_editable_leaf_changes_a_check_when_perturbed_in_applicable_scope` |  | 2978 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:337` |
| 368 | ts-ArrowFunction | `(callback of describe)` |  | 3033 | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/📄️pdf-and-figure-interaction/🟦️.tsx:10` |
| 368 | ts-FunctionDeclaration | `registerSettleContinuationsTests` |  | 4920 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔂️settle-continuations/🟦️.tsx:6` |
| 368 | ts-FunctionDeclaration | `registerRetainedAndStoreTests` |  | 275 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🔏️retained-and-store/🟦️.tsx:5` |
| 367 | ts-ArrowFunction | `(callback of describe)` |  | 363 | `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🟦️.ts:114` |
| 367 | ts-FunctionDeclaration | `validateTaxonomyTree` |  | 1186 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts:356` |
| 366 | rust-fn | `step_one` |  | 152 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🏗️model-build-job/🦀️.rs:154` |
| 366 | rust-fn | `apply_in_place` |  | 3747 | `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document/🧷️pdf-conformance/⚙️forward-mutation/🦀️.rs:19` |
| 365 | rust-fn | `handle_shell_hit` |  | 1525 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🖱️pointer-input/🎯️shell-hit/🦀️.rs:58` |
| 365 | ts-FunctionDeclaration | `Canvas2dHost` |  | 512 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🟦️.tsx:38` |
| 364 | ts-FunctionDeclaration | `applyCommand` |  | 4295 | `🧰️framework/🔨️modules/🐚️shell/🟦️.ts:194` |
| 364 | ts-FunctionDeclaration | `registerTests1` |  | 8823 | `🧰️framework/🔨️modules/📡️replication/🧪️tests/🧪️artifact-bootstrap-protocol/🟦️.ts:3` |
| 363 | rust-fn | `check_full_actions` |  | 7321 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:158` |
| 363 | ts-FunctionDeclaration | `registerProjectionPoseAndMatrixTests` |  | 2976 | `🧰️framework/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🧪️tests/🧪️chunkkey/📐️projection-poses-and-matrices/🟦️.ts:3` |
| 362 | ts-ArrowFunction | `(callback of describe)` |  | 44308 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🧪️tests/🧩️component/🟦️.tsx:10` |
| 361 | rust-fn | `create_drawing_app` |  | 2748 | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🛂️manifest/🦀️.rs:195` |
| 361 | ts-ArrowFunction | `(callback of describe)` |  | 2478 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📦️crate-tree/🪝️test-hoisting/🟦️.ts:14` |
| 360 | ts-MethodDeclaration | `run` |  | 1186 | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:21` |
| 360 | ts-FunctionDeclaration | `useCommandBar` |  | 3069 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔎️command-bar/🟦️.tsx:89` |
| 357 | ts-FunctionDeclaration | `registerDocumentOpenTests` |  | 2407 | `🧰️framework/🛍️products/💻️os/🧪️tests/📂️document-open/🟦️.ts:34` |
| 354 | rust-fn | `to_value` |  | 3250 | `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🪢️shell-command-value-bridge/🦀️.rs:13` |
| 353 | rust-fn | `close_step` |  | 300 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🧯️surface-state-and-retirement/🦀️.rs:141` |
| 352 | ts-ArrowFunction | `(callback of describe)` |  | 416 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🧪️tests/🧩️component/🟦️.tsx:603` |
| 351 | rust-fn | `decode_mleader_style_object` |  | 7364 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📥️import/📦️objects/🖌️styles/🦀️.rs:130` |
| 351 | ts-ArrowFunction | `(callback of describe)` |  | 8824 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🧪️tests/🧩️component/🟦️.tsx:13` |
| 350 | rust-fn | `check_connection` |  | 4777 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🔗️connection-checks/🦀️.rs:6` |
| 350 | ts-ArrowFunction | `(callback of describe)` |  | 1376 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌍️world3d-picking-and-previews/🟦️.ts:14` |
| 349 | ts-ArrowFunction | `(callback of describe)` |  | 1376 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚙️settings-general-layout/🟦️.ts:57` |
| 348 | ts-FunctionDeclaration | `useCheckInAndSync` |  | 60 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/✅️check-in-sync/🟦️.tsx:55` |
| 348 | ts-FunctionDeclaration | `useAgentBridge` |  | 4349 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🟦️.tsx:464` |
| 348 | ts-FunctionDeclaration | `inspectRustNonRepoJoinBaseSpans` |  | 1375 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔀️rust-non-repo-join-bases/🟦️.ts:11` |
| 347 | ts-ArrowFunction | `(callback of describe)` |  | 3026 | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🔍️figure-scroll-and-zoom-rendering/🟦️.tsx:9` |
| 347 | rust-fn | `next` |  | 2503 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/📜️history/⏳️replay-future/🦀️.rs:210` |
| 346 | ts-FunctionDeclaration | `runNativeBuildDriverV1` |  | 1552 | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts:342` |
| 346 | ts-ArrowFunction | `(callback of describe)` |  | 3033 | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🎯️canvas-disposition-interaction/🟦️.tsx:9` |
| 346 | ts-FunctionDeclaration | `PixelEditingOverlay` |  | 3661 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🟦️.tsx:57` |
| 345 | ts-ArrowFunction | `(callback of describe)` |  | 512 | `🧰️framework/🛍️products/🐾️pets/🧪️tests/🧰️gear-depiction/🟦️.tsx:144` |
| 344 | ts-ArrowFunction | `(callback of describe.each(COMPANIES))` |  | 4737 | `🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🧪️tests/🦾️body-and-gear/🟦️.ts:218` |
| 344 | ts-ArrowFunction | `(callback of describe)` |  | 21756 | `🧰️framework/🛍️products/🐾️pets/🧪️tests/🪞️fixture-lifting/🟦️.tsx:175` |
| 343 | ts-FunctionDeclaration | `retainedUiNativeStripOnly` |  | 123 | `🧰️framework/📦️packages/🟦️typescript/📜️script.ts:17` |
| 343 | ts-FunctionDeclaration | `registerShellComponents9Tests` |  | 2396 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🧱️shell-components-9/🟦️.tsx:3` |
| 342 | rust-fn | `retained_tree_node_step` |  | 1557 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🌲️retained-tree-node-painting/🦀️.rs:6` |
| 341 | ts-FunctionDeclaration | `startShow` |  | 439 | `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🫧️layer/🟦️.tsx:244` |
| 341 | rust-fn | `frame_before_input_step` |  | 512 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧠️app-runtime/🦀️.rs:125` |
| 340 | rust-fn | `check_load_cases` |  | 2 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️masonry/🧱️evaluate-wall/🏋️load-case-checks/🦀️.rs:6` |
| 340 | rust-fn | `render_chrome_step` |  | 1912 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs:197` |
| 339 | ts-ArrowFunction | `(callback of describe)` |  | 21756 | `🧰️framework/🛍️products/🐾️pets/🧪️tests/⏲️frame-pacing/🟦️.tsx:94` |
| 338 | rust-fn | `accept` |  | 2510 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🔨️modules/🏠️host/🧰️retained-mutation-owner-accept/🦀️.rs:6` |
| 338 | ts-ArrowFunction | `(callback of describe)` |  | 512 | `🧰️framework/🛍️products/📚️course/🧪️tests/⏱️challenge-clock-views/🟦️.tsx:19` |
| 338 | ts-ArrowFunction | `(callback of describe)` |  | 512 | `🧰️framework/🛍️products/📚️course/🧪️tests/🎪️pets-on-stage/🟦️.tsx:42` |
| 335 | rust-fn | `create_puzzle3d_app` |  | 4158 | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📋️manifest/🦀️.rs:130` |
| 334 | rust-fn | `accept` |  | 3736 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/✉️envelope/📨️owner-accept/🦀️.rs:7` |
| 333 | rust-fn | `step_one` |  | 512 | `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🧷️job-state-and-construction/🦀️.rs:470` |
| 333 | rust-fn | `verify_selected` |  | 4154 | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🏗️loader/🦀️.rs:79` |
| 332 | ts-ArrowFunction | `(callback of describe)` |  | 3032 | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🪡️thought-expansion/🟦️.ts:5` |
| 331 | rust-fn | `step` |  | 3045 | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🔨️modules/🏠️host/🧰️owned/🚀️store-initialization/🦀️.rs:52` |
| 331 | rust-fn | `decode_r2010_header_section` |  | 7369 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📥️import/🧱️records/📄️document/🦀️.rs:444` |
| 331 | rust-fn | `expand_from_value` |  | 239 | `🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🎯️expand-entries/🦀️.rs:259` |
| 331 | ts-FunctionDeclaration | `registerSessionBrokerPortTests` |  | 4508 | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🔑️session-broker-port/🟦️.ts:10` |
| 330 | rust-fn | `check_ports_primitives_and_spaces` |  | 1183 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📐️part2/🦀️.rs:500` |
| 329 | rust-fn | `evaluate_structure` |  | 4777 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧾️structure-evaluation/🦀️.rs:57` |
| 329 | ts-FunctionDeclaration | `proveInviteRedemptionTransaction` |  | 1186 | `🌎️hub/🧪️tests/🎫️invite-redemption-checks/🟦️.ts:57` |
| 329 | ts-ArrowFunction | `(callback of describe)` |  | 3032 | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/👻️morph-ghost-and-auto-animate-rendering/🟦️.tsx:8` |
| 329 | ts-FunctionDeclaration | `registerDocumentPackWireAdapterTests` |  | 1376 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📦️document-pack-wire-adapter/🟦️.tsx:7` |
| 329 | ts-ArrowFunction | `(callback of describe)` |  | 299 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧩️component/🟦️.tsx:230` |
| 328 | rust-fn | `close_step` |  | 512 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🧹️mounted-state-close/🦀️.rs:7` |
| 328 | rust-fn | `frame_into_step` |  | 1557 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🖥️frame-into-step-and-frame/🦀️.rs:10` |
| 327 | ts-FunctionDeclaration | `registerTests1` |  | 4679 | `🧰️framework/🔨️modules/🎭️actor/📤️return/🧪️tests/🧪️actorreturn-codecs-load-natively-in-node-strip-only-mode-and-preserve-ev/🟦️.ts:5` |
| 327 | rust-fn | `advance_apply_batch` |  | 512 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs:78` |
| 327 | ts-FunctionDeclaration | `useWorldHoverAndPlacement` |  | 512 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/👇️hover-and-placement/🟦️.ts:15` |
| 326 | ts-ArrowFunction | `(callback of describe)` |  | 2485 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-leaf-generics/🟦️.ts:136` |
| 324 | rust-fn | `replacement` | gen | 3790 | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🗂️catalog/🧾️row-operations/🦀️.rs:217` |
| 324 | ts-ArrowFunction | `(callback of describe)` |  | 3032 | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🎥️media-and-tile-rendering/🟦️.tsx:8` |
| 324 | rust-fn | `new` |  | 1912 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🐙️state/🧩️session-and-topology/🦀️.rs:26` |
| 324 | ts-FunctionDeclaration | `toolJobCoverageRun` |  | 98 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts:106` |
| 323 | rust-fn | `render_navbar_step` |  | 5202 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs:9` |
| 323 | rust-fn | `reduce` |  | 49361 | `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🦀️.rs:50` |
| 322 | rust-fn | `step_one` |  | 512 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🧵️visual-job/🦀️.rs:204` |
| 322 | rust-fn | `wire_probes` |  | 357 | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixtures/🧵️probe-corpus/🦀️.rs:93` |
| 322 | ts-ArrowFunction | `(callback of describe)` |  | 21756 | `🧰️framework/🛍️products/🐾️pets/🧪️tests/🖌️pet-depiction/🟦️.tsx:129` |
| 321 | rust-fn | `step` |  | 2510 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🔨️modules/🏠️host/🏗️store-initialization-authority/🦀️.rs:40` |
| 321 | ts-ArrowFunction | `IntroductionDemonstrationOverlay` |  | 117 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎬️introduction-demo/🟦️.tsx:122` |
| 320 | rust-fn | `step_phase` |  | 3719 | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🔨️modules/🏠️host/🧰️owned/🏁️initialization-authority/🦀️.rs:143` |
| 320 | rust-fn | `encode_r2010_table_record_frame` |  | 7362 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📤️export/📦️objects/🗃️tables/🦀️.rs:80` |
| 320 | ts-MethodDeclaration | `executeCommandDiff` |  | 2971 | `✏️s/🧑‍💻dev/📐️cad/🧪️tests/🔮️spatial-kernel/🧱️brepjs/⚙️wasm-engine/🟦️.ts:215` |
| 320 | ts-ArrowFunction | `(callback of describe)` |  | 349 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🪝️extension-invocation/🟦️.ts:18` |
| 319 | ts-ArrowFunction | `(callback of describe)` |  | 366 | `🧰️framework/🛍️products/📚️course/🧪️tests/🪟️presence-screens/🟦️.tsx:17` |
| 318 | rust-fn | `execute_step` |  | 2499 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/💽️sqlite-storage/📨️sqlite-task-executor/🦀️.rs:48` |
| 318 | ts-ArrowFunction | `(callback of describe)` |  | 512 | `🧰️framework/🛍️products/📚️course/🧪️tests/🏔️challenge-session/🟦️.tsx:12` |
| 317 | rust-fn | `print_content` |  | 3098 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖋️content/🦀️.rs:397` |
| 316 | rust-fn | `step` |  | 1740 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/🌱️initialization/🦀️.rs:604` |
| 316 | rust-fn | `create_puzzle5d_app` |  | 2470 | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🚀️app-assembly/🦀️.rs:76` |
| 316 | rust-fn | `frame_step` |  | 1557 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🎞️frame-step/🦀️.rs:40` |
| 316 | rust-fn | `handle_pointer_button_for` |  | 5202 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🖱️pointer-input/🦀️.rs:53` |
| 316 | rust-fn | `time_travel_input_row` |  | 4627 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🪟️panel/🖼️sections/🦀️.rs:415` |
| 316 | ts-ArrowFunction | `(callback of describe)` |  | 330 | `🧰️framework/🛍️products/📚️course/🧪️tests/🎛️pet-allowances/🟦️.tsx:72` |
| 315 | ts-FunctionDeclaration | `registerNodeFieldsAndValidationTests` |  | 275 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🧩️node-fields-and-validation/🟦️.tsx:5` |
| 315 | ts-FunctionDeclaration | `transformTestClosure` |  | 1375 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts:344` |
| 314 | ts-FunctionDeclaration | `toolJobLatestWinsSelfTests` |  | 471 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:12` |
| 313 | rust-fn | `reject_vector_of` |  | 91 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🕸️mutate-fem3d-1-mesh/🦀️.rs:329` |
| 312 | ts-ArrowFunction | `(callback of describe)` |  | 275 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🪪️window-action-panel/🟦️.ts:29` |
| 311 | rust-fn | `close_step` |  | 350 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/🪜️solve-step/🥦️close-ladder/🦀️.rs:41` |
| 311 | rust-fn | `apply_structure` |  | 3719 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔮️oracles/🏗️structure-kinds/🦀️.rs:497` |
| 311 | rust-fn | `gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply` |  | 2904 | `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/📮️committed-publication-laws/🦀️.rs:4` |
| 311 | ts-ArrowFunction | `ContextMenuController` |  | 2418 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx:99` |
| 311 | ts-FunctionDeclaration | `registerPlaygroundActionAndPluginSourceTests` |  | 2968 | `🧰️framework/🧪️tests/🧪️docklayoutstore/🧭️playground-actions-and-plugin-sources/🟦️.ts:5` |
| 310 | ts-ArrowFunction | `(callback of describe)` |  | 28563 | `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🎞️react-deck/🟦️.tsx:9` |
| 310 | ts-ArrowFunction | `Window` |  | 6821 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx:128` |
| 309 | rust-fn | `step` |  | 3255 | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧩️command-decode/🧭️command-cursor/🦀️.rs:15` |
| 308 | rust-fn | `create_remodeling_app` |  | 3756 | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📜️manifest/🦀️.rs:9` |
| 308 | ts-MethodDeclaration | `run` |  | 1171 | `🌎️hub/🧪️tests/👤️browser-actor-checks/🟦️.ts:42` |
| 308 | ts-MethodDeclaration | `run` |  | 512 | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:19` |
| 308 | ts-FunctionDeclaration | `useInkPointer` |  | 3205 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/👆️ink-pointer/🟦️.ts:13` |
| 307 | rust-fn | `emit` |  | 3693 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🧭️encode/🦀️.rs:348` |
| 306 | ts-FunctionDeclaration | `registerKernelReturnInputSettlementTests` |  | 349 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🧵️kernel-return-input-settlement/🟦️.ts:8` |
| 305 | ts-ArrowFunction | `(callback of describe)` |  | 44308 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🧪️tests/🧩️component/🟦️.tsx:10` |
| 303 | ts-FunctionDeclaration | `registerTests1` |  | 7911 | `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧪️tests/🧪️actor-instance-close-fault-publication-fixture-preserves-watchdog-and-te/🟦️.ts:5` |
| 303 | ts-ArrowFunction | `(callback of describe)` |  | 21756 | `🧰️framework/🛍️products/🐾️pets/🧪️tests/🤏️pet-handling/🟦️.tsx:570` |
| 303 | rust-fn | `publish_world3d_plan_step` |  | 1912 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/📜️plan-and-publish/🦀️.rs:235` |
| 302 | ts-FunctionDeclaration | `inspectRustCompileReferences` |  | 21164 | `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts:270` |
| 302 | ts-ArrowFunction | `(callback of describe)` |  | 1186 | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔌️staging-root/🟦️.ts:154` |
| 302 | ts-FunctionDeclaration | `registerTests1` |  | 1186 | `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️effectbackbone-capability-gating/🟦️.ts:7` |
| 301 | ts-FunctionDeclaration | `remodelingMutationOutcome` |  | 1315 | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts:493` |
| 301 | ts-FunctionDeclaration | `registerTutorialEngineAndMoreTests` |  | 1181 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/📊️tutorial-engine-and-more/🟦️.tsx:6` |
| 301 | rust-fn | `record_codegen` |  | 512 | `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🏗️record-field-codegen/🦀️.rs:84` |
