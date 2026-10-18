# 🧪 Test Sweep: Renderer (tsr)

## 0. Verdict

DONE except for named items that need Rust or build-tool owners. The renderer-react vitest package went from 38 failing tests in 23 files (`shu-vt-a3`) to 1 failing test in 1 file (`tsr-vt-full3`: 222 files, 3,184 tests); the wgpu vitest package (never swept before) went from 38 failing tests in 15 files to 6; the three red script oracles are green (all eight oracles of the react script are green). Remaining red, all with a diagnosis in section 5: `⏱️frame-latency` (peer-held Rust dropped an instrumentation call), `🖌️wgpu-document-owner-move` (4 tests: the Rust asset-decode session was reverted to constructing its own job authority), `🪆️embedded-mount` in the wgpu package (a wasm package is not built), and one unreproduced 120 s stall of the first `Paint2dHost selection-focus` test in 1 of 3 full runs.

## 1. Tools and method

- `F/tsr-tools/vitest.sh <label> [vitest filters]`: renderer-react vitest package (long level, JSON reporter) into `F/🗑️generated/tsr-vt-<label>.json/.log`; `TARGET=🧊️wgpu` selects the wgpu package config. `vtsum.py <json> <tsv>` writes the per-test status list. `run-failing.sh` runs the 22 files that failed in `shu-vt-a3` plus every file I re-pointed. `softrun.py` runs a set of wgpu tests with every `toContain`/`toMatch` assertion turned into `expect.soft` (restores the files in a `finally`), so one run lists all stale anchors of a test. `oracles.ts` and `oracles2.ts` run the script oracles one by one. `probe.ts <rust root> <needle…>` checks needles against a Rust module tree. `fixpaths.py`, `closure.ts`, `bunbuild.ts`, `bindings.ts`, `echo.ts`, `spo.ts`, `wm.ts` are the one-off probes named below.
- A command that outlives the tool limit dies with its shell; `run_in_background` works. The first full-package attempt (`tsr-vt-base`) was killed by its own alarm under a machine load of 90 to 150 and left no JSON; the baseline is therefore `shu-vt-a3` (last full run, a few hours older). Full runs finished in 6 to 14 minutes once the load dropped.
- Long text anchors: a test that reads Rust or TS text now reads a module tree (`readModuleTreeSource`, `sliceModuleTreeSource` of `🔨️modules/📚️compiler/📖️syntax/🌳️module-tree`), and needles are checked against the live tree. Several needles differ only by whitespace (the original-turn rewrite of `browser-worker/🦀️.rs` compacted code); those tests compare whitespace-squashed text.

## 2. Script oracles (3 red before, all 8 green now)

| Oracle | Before | Cause | Fix |
|---|---|---|---|
| `world3dPointerGestureOracle` | ENOENT `…/🕹️interaction/🧫️fixtures/🎯️merge-modes.json` | stale fixture path | `World3dHost/🧪️tests/🧫️fixtures/🖱️pointer-gestures.json` `wiring.mergeVocabularyFixture` → `🧪️tests/🧫️fixtures` (75 checks) |
| `interactionSelectionSetOracle` | projection anchor missing | Rust anchors moved by the plugin carve (`pub(crate)` became `pub`, call sites in two other files) | `🔌️plugin/🧪️tests/🧫️fixtures/🕹️selection-set.json` `wiring` is now per anchor `{file, text}`; oracle reads each anchor in its own file (17 checks) |
| `selectionMergeVocabularyOracle` | wire-label encoder anchor missing | codec in `📡️wire/🕹️interaction/🦀️.rs`, decoder in `🔌️plugin/🕹️interaction/🧭️topology`, `merge_world_selection_ids` in `🪟️window-kits/🌍️world3d` | `🕹️interaction/🧪️tests/🧫️fixtures/🎯️merge-modes.json` `wiring.codec.file`, `domainPath.decoderFile`, `domainPath.worldPathFile`; oracle reads both plugin files (89 checks) |
| `scopedPresenceOracle` (not in the named three; found red when all eight were run) | `false == true` | the fixture law `ScopedPresenceCloseV1` has no production schema (section 4), the worker no longer has `inferencePort`, the worker wire moved to `🛰️backbone/🧱️worker-wire` | the oracle checks the close example structurally and reads the new anchors (29 checks) |

The Rust twins read `domain`, `cases`, `vocabulary` and so on, never `wiring` (checked in `plugin-runtime-plugin-builder-contract-part-4` and `🕹️interaction/🧪️tests/🔬️unit`). The `🎨️world3d-scene-shading` oracle script read a deleted schema file (`🖌️render/🧬️schema/🎨️world3d-inline-surface`); the schema read and its Ajv import are removed (the oracle also needs cargo and could not run here). Run: `bun F/tsr-tools/oracles2.ts`.

## 3. Failing tests before and after, by cause

Before = `shu-vt-a3` (renderer-react, 38 failed tests). After = `tsr-vt-full3` (222 files, 3,184 tests, 1 failed; 3,183 passed).

| Cause | Before | After | Where |
|---|---:|---:|---|
| Stale fixture path (`../../🧫️fixtures` after fixtures moved to `🧪️tests/🧫️fixtures`) | 8 | 0 | `window-measure-controls` 2, `browser-frame-transport` 5, `Interpreter` table 1 |
| Test reads text that moved or was reshaped by the carve and the original-turn work | 11 | 0 | `os-command-shortcuts`, `settings-general-layout` 2, `browser-frame-transport` 1, `shell-utility-leaves`, `more-work-drive`, `NodeGraph pinch-wheel`, `IconRenderHost` 2, `reprojection-status`, `icon-svg-lighting` (uniform 240 to 256 bytes) |
| Schema export dropped from a production schema | 2 | 0 | `scoped-presence`, `AgentDelegations` (section 4: the tests stop validating a fixture law) |
| Corpus or fixture behind a product change | 8 | 0 | `echo-pack` 4, `command-rejection`, `fault-notices`, `extension-invocation`, `World3dHost` zero-vertex wire |
| Stale package authority of the wgpu worker | 5 | 0 | `package-integration`: digest, module list, workspace imports (builder gap fixed, section 4) |
| Hollow tests | 1 failing (`puzzle3d-settings-document`) plus 19 always-green empty bodies | 0 (21 tests deleted or restored) | section 4 |
| Load-flaky | 2 | 0 (one unreproduced stall seen once in `full2`, item 4) | `embedded-mount` fixed (event-driven wait); `selection-focus` two races fixed, one 120 s stall seen in `full2` |
| Product regression in peer-held Rust | 1 | 1 | `⏱️frame-latency` |

The wgpu package config (`🎯️targets/🧊️wgpu/🧪️tests/🎚️config`, 56 files, 602 tests, not covered by the react config) had 38 failing tests in 15 files in its first run (`tsr-vt-wgpu`) and 6 in the last (`tsr-vt-wgpu2`):

| Cause | Before | After |
|---|---:|---:|
| Moved paths (UI crate moved from `🛍️products/🖱️ui` to `🔨️modules/🖱️ui`, retained tree to `🖱️ui/🌳️retained/🌳️tree`, theme to `🎨️styling/🎭️theme`, raster to `🖍️draw/🖼️raster`) | 17 | 0 |
| Anchors reshaped (minified original-turn code, `PointerCtx`, `self.settle`, `pub` fields, `OriginalBrowserFrameNative`) | 13 | 0 |
| Stale API of the product (server mount needs a canonical workspace, profile `prototype`, vendor font mount; a `[DEBUG]` log the product no longer prints) | 4 | 0 |
| Product regression in peer-held Rust | 1 + 4 | 1 + 4 |
| Environment (wasm package `@semio-tech/framework-os-node-graph-rs` not built) | 1 | 1 |

## 4. What changed (owners and evidence)

Production data and tooling:
- `🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: `catalogSha256` of the wgpu package catalog set to the digest of `🪪️package-catalog.json` (changed at commit 686); `browserProfile.sourceModulePaths` regenerated with the observation the library's own `generator-ownership` test uses (`tsr-tools/bunbuild.ts`; 101 modules added, 7 stale removed, 347 listed, byte ordered); `browserProfile.workspaceImports` extended from the 3 root packages to the 53 specifiers the framework barrel now imports (`tsr-tools/bindings.ts` resolves each from the package `exports` map).
- `🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` `parseSemanticPackageBrowserProfile`: the three roots are required, every other binding must be an `@semio-tech/…` specifier, keys byte ordered (it demanded exactly the 3 roots before).
- `📺️renderer/…/🧊️wgpu/📽️projection/🟦️.ts` `renderWgpuBrowserBundles`: a workspace import is checked as package name plus `exports` subpath (`manifest.name === package`, `manifest.exports[subpath] === entry`); it required `manifest.exports["."]` of the whole specifier. With this the worker renders again (`package-integration` "renders identical bytes twice" and "renders byte-identical worker bytes" green). The plugin-registry generator in `🔍️discovery` (line 9574) has the same root-only rule; not changed, nothing in my scope fails on it.
- Fixture-law schemas are not production schemas (ticket `26/05/30/FIXTURES-ARE-TESTING-EXAMPLES-ONLY`: "No separate schema should be introduced for examples"; commit 671 removed them). I first re-added `ScopedPresenceCloseV1` and `AgentMcpClientConfigLawV1`, then reverted both (the schema files and `🔣️schema-catalog.json` are byte-identical to before my work; I confirmed by hash) and changed their consumers instead: `scoped-presence` and `scopedPresenceOracle` check the close example structurally, the AgentDelegations component test replays the cases and still validates `AgentMcpClientConfigV1` and `AgentCredentialInstallRequestV1` against the owned schema.

Fixtures (examples, so edited freely): `🧫️time-travel-band` (`timeTravel.unchanged` after `timeTravel.empty`, the order of `TIME_TRAVEL_CODE_LABELS` in TS and Rust and of the module's conformance law); `🪞️echo-pack` (the product's pack is the canonical editor scene document: no `lanes`, no `language`; case `undeclared-lane-ref-stays` became `lane-refs-never-enter-the-pack`; evidence `tsr-tools/echo.ts` and the wgpu twin `text_editor_scene_document`); `🌳️wgpu-document-reconcile`, `🖼️wgpu-raster-witness`, `🫀️settle-pump` (`self.settle.owe_settle()` and `settle.owe_settle()`), `🔢️wgpu-u64-seam` (declaration and call-site holders per export), `🧩️wgpu-module-routes` (profiles `prototype`/`release`), `🖱️pointer-gestures`, `🕹️selection-set`, `🎯️merge-modes` (section 2).

Test readers (needles verified against the live tree): `os-command-shortcuts`, `shell-utility-leaves`, `settings-general-layout`, `hub-projection`, `scene-pointer-cancellation` (`PointerCtx::of(&mut self.shell).handle_pointer_cancel_for`), `frame-action-ledger` (`pub frame_actions`), `wgpu-worker-step-budget` (three tests, whitespace-squashed text: the continuation `continue_frame`, `request_frame`, `next_deadline_delay_ms` all still exist; `take_cursor_wake_directive` was dropped from the conjunction at commit 686 and no code in the wasm path retains that token any more, so the conjunct was dead), `wgpu-runtime-assets`, `tree-row-rects`, `wgpu-raster-residency`, `more-work-drive` (`originalDriveSource.operationClockUs() >= driveDeadline`), `pinch-wheel`, `reprojection-status`, `browser-frame-transport/🧬️boot-publication-and-seams` (retirement law now reads `OsHost::try_into_retirement(self.host, cx)?` and keeps its four mutation checks), `IconRenderHost/frame-presentation`, `icon-svg-lighting`, `wgpu-browser-media` (the test counted three `[DEBUG]` console lines of the media host; the host prints none).
- `puzzle3d-settings-document`: an earlier automated cleanup removed the whole per-control `forEach` of "mounts four uniform steppers" together with the schema validator lines (commit 671). The behaviour part is restored; the two tests that only validated a deleted schema are deleted.
- 19 hollow tests (`it("validates the neutral … contract", () => {})`) in 17 renderer test files deleted: an empty body proves nothing, their schemas are gone by design (list in `F/🗑️generated/` is not needed; run `git diff` on the files `retained-clock`, `wgpu-accessibility-interaction`, `command-panel`, `shell-utility-leaves`, `retained-section-collapse`, `maximize-resync`, `GraphTimelineHost` 2, `UtilityTree/picker-explicit-press`, `Canvas2dHost/input-contract` 3, `EventFeedHost`, `Paint2dHost/navigator-camera`, `Board2dHost/pinch-gesture`, `IconRenderHost` 3, `ShellHelpers/empty-dock-notice`).
- `extension-invocation`: a complete request can still be a fault; the law is "an incomplete request is a fault, an ok row has a complete request". `fault-notices`: the diagnostic `Fault` requires `retryable` and `retainedProgress`. `World3dHost/🧩️component`: the bounds law is the union of the position and the edge buffer (as the Rust `lease.aabb()` computes it), product and three.js oracle measured alike.

Races fixed (no retry and no timeout increase as the fix):
- `embedded-mount`: three 1 s `vi.waitFor` polls for a rendered shell replaced by `untilShellsReady`, a `MutationObserver` on the roots; only the test timeout bounds it.
- `Paint2dHost selection-focus`: (1) the "combination cancels" test read the progress element from a zero-delay timer, and whether React had committed 0.75 by then depended on the load (it read 0.375); it now holds the job's own next turn yield (`jobStepGate`, one shot, so it cannot starve the testing library's own timers), waits until the UI shows 0.75, cancels, releases. (2) every wait used the 1 s default of the owned `waitFor`; it waits under the test's own budget (`getCurrentTest().timeout`). Eight parallel and four parallel solo runs: all green.

## 5. Open items

1. `⏱️frame-latency`: commit 686 rewrote the winit host's snapshot publication and dropped the `FrameLatencyTimer::start(FrameLatencyAuthority::renderer_frame(generation.0), FrameLatencyStage::SnapshotPublish, 1)` line; the stage is defined in `⏱️frame-latency/🦀️.rs` and nothing emits it. Fix (not compiled; `🧊️wgpu` Rust is peer-held and cargo is blocked): add that timer, qualified `semio_framework_os_renderer_wgpu_host_base::frame_latency::…` as the browser worker does, in the `AppPresentStep::Complete` arm of `publish_presented_snapshot_on_original_caller` in `🪟️winit-app/🦀️.rs`.
2. `🖌️wgpu-document-owner-move` (4 tests): the asset-decode law ("the session receives original authority without constructing its own policy or cancellation") is violated by the current Rust. `🧊️renderer/🔁️asset-decode-jobs/🦀️.rs` `RendererAssetDecodeSession::new` calls `allocate_operation_id()`, `root_cancel_token()` and `WorkerJobSession::try_new(...)` itself; `RendererAssetDecodeInvocation` exists nowhere any more (only in the test and fixture); `🪟️winit-app` no longer names `INTERACTIVE_STEP_CEILING_US / 2` for the present deadline; `🧵️frame-job` still contains `WorkerJobSession::try_new`; `🏠️os-host` lacks `CancelToken::admit_root`, `JobPayloadAuthority::admit`, `StepContext::with_payload_authority`. The renderer file lost 353 lines at commit 686, so this looks like a regression of the original-authority migration, not a change of the law. Left red; it needs the owner of that migration (or an explicit decision to drop the law and delete the tests).
3. `🪆️embedded-mount` "boots an explicit empty variant without generated session authority" (wgpu package only): `@semio-tech/framework-os-node-graph-rs/framework_os_node_graph.js` is a wasm-pack output that is not built (cargo/wasm blocked). Environment.
4. `Paint2dHost selection-focus` "survives focus moving to Apply": one 120 s stall in `full2` (the first test of the file), not seen in `full`, `full3`, 8 parallel solo runs, 4 parallel runs of the whole file. Unreproduced; if it returns, capture the pending timers at the stall.
5. `🖌️Paint2dHost/✍️editing` bun suite (`paint2d-editing-check`): through `bun test` directly 65 of 66 pass; "complete owned suite" needs the launcher's process envelope. The launcher (`bun repo run @semio-tech/framework-renderer-react:paint2d-editing-check`) refuses with "Pending Native image publication must settle before ordinary admission" (cache storage custody of the compiler-storage fleet). Environment.
6. `📦️packages/🟦️typescript/📋️project.json` `frameWorkerSources` (and the library's `generator-ownership` test) lists 189 inputs; 179 of the 347 browser modules are not in it (79 of the old 253 already were not). Pre-existing; the Nx hasher would not invalidate the worker bundle for those inputs.
7. 14 `$defs` of the directory schema and the whole set of fixture-law schemas that existed at commit 649 are gone from production schemas; by the fixtures policy that is intended, so nothing is restored.

## 6. Final verification (commands run, results)

- `bun F/tsr-tools/oracles2.ts`: 8 oracles, all `clean`.
- `F/tsr-tools/vitest.sh full` (react, long): 222 files, 3,184 tests, 4 failed (`frame-latency`, `scene-pointer-cancellation`, 2 package-integration "renders"); then fixed.
- `F/tsr-tools/vitest.sh full2` (after the fixes above): 3,184 tests, 2 failed (`frame-latency`, one `selection-focus` stall).
- `F/tsr-tools/vitest.sh full3` (after reverting the two fixture-law schemas and changing their consumers): 221 of 222 files, 3,183 of 3,184 tests pass.
- `TARGET=🧊️wgpu F/tsr-tools/vitest.sh wgpu2`: 56 files, 602 tests, 6 failed (items 1, 2, 3).

`full3`: 222 files (221 passed, 1 failed), 3,184 tests (3,183 passed, 1 failed: `⏱️frame-latency`). Test count 3,205 (a3 summary) → 3,184 (-21: 19 hollow tests and 2 schema-only tests deleted; `echo-pack` keeps its 5).
