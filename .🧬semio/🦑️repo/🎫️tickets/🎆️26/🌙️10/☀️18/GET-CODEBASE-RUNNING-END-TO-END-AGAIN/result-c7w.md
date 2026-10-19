# Result c7w (census 7: 10 packages, 42 files; wgpu tree + `surfaces` + BIM + TS)

Executor `c7w`, foreground, no sub-agents, no cargo / `bun repo` / nx / bazel, no modifying git command. `C` = this ticket folder. Tools: `C/c7w-tools/` (Rust: descendant of `c6ui-tools`; TS: `c7w-tools/ts/`, descendant of `c6repo-tools`). Logs: `C/🗑️generated/c7w-*` (nothing deleted). Sandboxes: `/Users/ueli/.cache/c7w/`. Package/manifest freeze respected: no `package.json` / `bun.lock` edit, nothing under `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/**` or `🏗️build/📥️inputs/🔏️revision/**`. Manifest changes needed: none.

## 0. Verdict

- 37 Rust files split (160 new Rust files; 182 new directories including the TS ones), 4 TS files split, 2 `jcoprobe.js` bundles excluded via the generated header, 1 file (`💠️values/🦀️.rs`) left unsplit because it is generated.
- Every produced file is under 1,000 lines (max 945). 32 of 37 Rust files have every child at most 600; the exceptions are single indivisible functions or regions: `paint` 841 (one 835-line fn), `fact` 945 (one 939-line `new`), `shact` 774 (one 761-line `dispatch_action`), `cdisp` 694 (one 686-line `#region` that cannot be cut), `winit` 613 (inline `mod native`). TS: `browser-frame-transport` leaf 869 (one 839-line class), `plugin-module-loader` 710 (one 694-line fn).
- Static evidence only (no compile). The typeck differential (pos oracle, `rustc +nightly-2026-07-20 --emit=metadata`, base = pre-split snapshots through the `overlay` rev) is clean: 0 new privacy / trait-scope / resolve / type / other rows for the UI family (lib, test, wasm32), the renderer family (lib, test, wasm32), `ui-wgpu-layout` (flex) and the BIM family. Details in section 4.

## 1. Verdict table per file

`before` = live lines at snapshot (equal census 7). `root` = lines of the original path afterwards. Mechanism: `mod` (modsplit.py, `#[path]` children, impl blocks cut at method boundaries), `ts` (c6repo `split.ts`).

| Pkg | tag | file (tail) | before | root | children (max) | crate (owner, from `#[path]`/`include!` closure) |
| --- | --- | --- | ---: | ---: | ---: | --- |
| over-10k | jcoprobe | `🧩️jcoprobe/🌐️browser-bundles/{⚡️out-jspi-explicit,📞️out-callback}/jcoprobe.js` | 10,980 | 10,981 | generated, not split | header added, section 3 |
| P01 | ev | `🧊️wgpu/⚡️events` | 3,360 | 90 | 8 (522) | `semio-framework-ui-wgpu-events` |
| P01 | paint | `🧊️wgpu/🖌️paint` | 3,111 | 99 | 7 (841) | `semio-framework-ui-wgpu-paint` |
| P01 | eng | `🧊️wgpu/⚙️engine` | 2,856 | 64 | 7 (486) | `semio-framework-ui-wgpu-engine` |
| P02 | prep | `🎟️prepared` | 2,620 | 80 | 7 (495) | `semio-framework-ui-prepared` |
| P02 | dexec | `🖍️draw/🔧️pipeline/🎬️execution` | 1,893 | 46 | 5 (491) | `semio-framework-ui-wgpu-draw-execution` |
| P02 | raster | `🖍️draw/🖼️raster` | 1,812 | 70 | 5 (466) | `semio-framework-ui-wgpu-raster` |
| P02 | dtypes | `🖍️draw/🏷️types` | 1,698 | 46 | 5 (443) | `semio-framework-ui-draw-contract` |
| P02 | mlay | `📌️mounted_layout` | 1,607 | 60 | 5 (434) | `semio-framework-ui-wgpu-mounted-layout` |
| P03 | fact | `🖍️draw/🔧️pipeline/🏭️factory` | 1,394 | 33 | 2 (945) | `semio-framework-ui-wgpu-pipelines` |
| P03 | surf | `⚙️engine/🪟️surfaces` | 1,260 | 59 | 4 (368) | `semio-framework-ui-wgpu-engine-surface` |
| P03 | pstage | `🎟️prepared/📤️staging` | 1,175 | 72 | 4 (354) | `semio-framework-ui-prepared-staging` |
| P03 | shad | `🖍️draw/🎨️shaders` | 1,142 | 27 | 3 (439) | `semio-framework-ui-render-prepared-shaders` |
| P03 | dexe2 | `🖍️draw/🎮️execution` | 1,089 | 56 | 4 (320) | `semio-framework-ui-wgpu-render` |
| P03 | flex | `📐️flex` | 1,069 | 55 | 4 (452) | `semio-framework-ui-wgpu-layout` |
| P04 | glb | `🧊️renderer/🦴️glb-decode` | 2,418 | 42 | 7 (479) | `…renderer-wgpu-asset-decode` |
| P04 | dock | `🛰️Dock` | 2,145 | 73 | 6 (480) | `…renderer-wgpu-dock` |
| P04 | ink | `🎞️Scenes/✒️ink-canvas-model-state` | 2,047 | 63 | 6 (433) | `…renderer-wgpu-scene-ink` |
| P04 | kpool | `🧊️renderer/📦️kernel-runtime/🏊️kernel-pool` | 1,963 | 18 | 5 (572) | `…renderer-wgpu-kernel-pool` |
| P04 | esr | `⚙️EngineCanvas/🗂️engine-surface-registry` | 1,420 | 63 | 4 (490) | `…renderer-wgpu-engine-registry` |
| P05 | pbridge | `🌉️ProgramBridge` | 1,849 | 110 | 4 (529) | `…renderer-wgpu-program-bridge` |
| P05 | mbox | `🧊️renderer/📮️mailbox-impl` | 1,599 | 17 | 5 (445) | `…renderer-wgpu-runtime-mailbox` |
| P05 | sstate | `🎞️Scenes/🗃️scene-state` | 1,506 | 42 | 4 (508) | `…renderer-wgpu-scene-state` |
| P05 | intro | `🗣️Interpreter/🔬️introspection` | 1,397 | 112 | 4 (423) | `…renderer-wgpu-ui-engine-dump` |
| P05 | shact | `🐚️Shell/…/🎬️shell-actions` | 1,357 | 18 | 3 (774) | `…renderer-wgpu-shell-actions` |
| P05 | ngraph | `⚙️EngineCanvas/🕸️node-graph` | 1,310 | 73 | 5 (310) | `…renderer-wgpu-engine-graph` |
| P06 | abridge | `🔗️AgentBridge` | 1,215 | 53 | 3 (541) | `…renderer-wgpu-agent-elements` |
| P06 | fpres | `🧊️renderer/🖥️frame-presentation` | 1,213 | 19 | 3 (493) | `…renderer-wgpu-frame-presentation` |
| P06 | retui | `🗣️Interpreter/🧩️retained-ui-engine` | 1,208 | 92 | 3 (461) | `…renderer-wgpu-ui-engine` |
| P06 | kbd | `🐚️Shell/…/⌨️keyboard-and-palette` | 1,194 | 15 | 3 (478) | `…renderer-wgpu-shell-pointer` |
| P06 | krc | `📡️kernel-protocol/📥️lifted/📬️kernel-replay-client` | 1,175 | 50 | 4 (357) | `…renderer-wgpu-kernel-protocol` |
| P06 | cdisp | `🐚️Shell/…/📨️command-dispatch` | 1,175 | 11 | 2 (694) | `…renderer-wgpu-shell-actions` |
| P06 | ted | `⚙️EngineCanvas/✍️text-editor` | 1,142 | 57 | 4 (319) | `…renderer-wgpu-engine-text` |
| P06 | ptr | `🐚️Shell/…/🖱️pointer-input` | 1,120 | 18 | 3 (424) | `…renderer-wgpu-shell-pointer` |
| P07 | esa | `⚙️EngineCanvas/🧩️engine-surface-attach` | 1,111 | 63 | 4 (333) | `…renderer-wgpu-engine-attach` |
| P07 | canv2d | `🎞️Scenes/🖌️canvas2d` | 1,047 | 32 | 3 (402) | `…renderer-wgpu-scene-canvas2d` |
| P07 | winit | `🪟️winit-app` | 1,019 | 59 | 2 (613) | `…renderer-wgpu-host-winit` |
| P08 | bimlib | `🩹️patches/📚️library` | 1,088 | 25 | 3 (516) | `semio-s-artifact-bim-model-tests-2` (and diff) |
| P08 | bimvalues | `📸️snapshot/💠️values` | 1,096 | 1,096 | not split | `semio-s-artifact-bim-model-schema`: generated (header `🤖️ Generated by …r3-f1-gen-model.ts … Edit the model there`; generator exists at `C/../🌙️10/☀️08/BIM-PLUGIN/r3-f1-gen-model.ts`) |
| P09 | hc | `🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🤝️collaboration` | 1,432 | 160 | 5 (451) | ts |
| P10 | pb | `🐚️plugin-bridge` | 2,096 | 121 | 5 (710) | ts |
| P10 | bft | `🚚️browser-frame-transport` | 1,315 | 869 | 2 (358) | ts |
| P10 | fw | `🎞️frame-worker` | 1,140 | 77 | 9 (384) | ts |

`…renderer-wgpu-X` = `semio-framework-os-renderer-wgpu-X`. Totals: 58,808 Rust lines in the 37 split files before, their roots now 1,982 lines.

## 2. Methods and declared deviations (everything else is move-only)

Rust: census-5/6 method (`modsplit.py`, `#[path]` children, explicit `pub use child::{…}`, one `use super::*;` per child, `nest` where the original used `pub(super)`). New in c7w (in `c7w-tools/`): `Cut` spec builder (original-numbered ranges, auto-mapped across cut impl headers, kebab budget assertion because the longest deep paths allow only 17 bytes), `snap.py`, `plan.py`, `crateof.py` / `crateindex.py` (owner crate by `#[path]`/`include!` closure), `mkoverlay.py` (pos base), `rsexpand*.py`, `tsreads.py`, `taxadd.py`. Splitter fixes: trait items whose methods are called from siblings are widened and re-exported; original `pub(crate)` / `pub(super)` items are always re-exported at the original path (a path used by a module outside the split scope was lost before); idents of every other file of the owning crate closure now count as external uses (the renderer files are `include!`-spliced siblings); block comments are leading trivia.

Declared source edits outside the splits (all comment-only unless noted):
- `🎟️prepared/📤️staging/🦀️.rs`: removed the stray unclosed `//#region 📦️Packet` (the matching orphan `//#endregion` stays in `prepared`).
- `🎞️Scenes/🗃️scene-state/🦀️.rs`: added the 5 missing `//#endregion` lines (`SceneInput`, `SceneRowTransfer`, `RenderEntry`, `Algebra`, `Input`).
- `🎞️Scenes/🖌️canvas2d/🦀️.rs`: moved 3 region marker lines above doc blocks they split from the item the docs attach to.
- code: `🖍️draw/🖼️raster/🦀️.rs`: 4 methods `pub(super) fn` to `pub fn` (`validate_engine_renderer_allocation`, `…_target_texture_…`, `…_target_view_…`, `retain_engine_allocation_fault`). `pub(super)` in a crate root is `E0433 too many leading super keywords` (pre-existing in HEAD) and `dexe2` calls them across crates. Lossless oracle declares the 4 lines.
- TS `🎞️frame-worker`: module-level `let` bindings written from another module are written through generated `setX(value)` functions in `🧵️state` (reads stay live ESM bindings; value-position `x++` becomes `(setX(x + 1) - 1)`); 7 non-null assertions / one cast keep the control-flow narrowing the assignments gave (`plans/fw.ts` `patches`). TS `🐚️plugin-bridge`: `nextGlobalInstanceId` lives with `loadPluginModule`, the only writer.

## 3. jcoprobe bundles (generated)

Both `jcoprobe.js` (identical, 10,980 lines) are `jco transpile` output of the `semio-jcoprobe-guest` component (command recorded in `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-jco-spike-report.md`: `jco transpile <wasm> -o <out> --name jcoprobe --map semio:jcoprobe/probe-host=<browser-host/🟨️.js>`, jco 1.27.0; `--async-mode jspi` is byte-identical, which is why two copies exist). No generator script lives in the tree, so the first line of both files is now `// @generated by jco transpile … (jco 1.27.0); DO NOT EDIT.` (matches the census header rule). A future generator must emit that line. Consumers `🧪️tests/🧩️jco-callback`, `🧩️jcoprobe-callback` import the files unchanged (`node --check` ok).

## 4. Oracles (static; all rerun after the final splitter state)

| check | result |
| --- | --- |
| losslessness | `modsplit.py lossless <tag>`: `missing 0` (4 declared raster lines) and every extra explained, 37 tags |
| structure | `vmod.py live`: children mounted once, docstring emoji unique/first/equal to folder emoji and different from the root, no sibling emoji clash, region balance, brace depth, rustfmt parse, include targets: OK for 37 tags |
| defscan / ambig / modclash | no cross-file private leftover except a cfg-variant false positive in `retui` (`RetainedInputReceiptReservation`); 0 glob ambiguity; 0 module-name clash |
| typeck differential (pos) | families in `c7w-tools/families/`, base `overlay` = snapshots (`pos-overlay.json`): `c7w-ui` lib/test/wasm32 (13 crates) NEW 0; `c7w-ui2` (`ui-wgpu-layout`) NEW 0; `c7w-rend` lib/test/wasm32 (20 renderer crates) 0 non-stub new (3 to 4 stub-noise rows paired with equal GONE); `c7w-bim` lib/test NEW 0. Earlier runs found and fixed: raster `pub(super)`, `DeterministicTextWorker` trait scope, 3 unresolved sibling uses (`ink`, `sstate`, `esr` re-exports) |
| taxonomy | `tax.ts` over the 182 new directories: 0 unresolved (a bogus directory is reported) |
| source gate | `census-ui.ts`: all 36 owning crates stay inside 4,000 active lines / 32 files / 256 KiB. Highest: `shell-actions` 0.874, `events` 0.873, `shell-pointer` 0.821 (files). Table: `🗑️generated/c7w-census-after-all.jsonl` |
| paths | longest new path 239 repo-relative UTF-8 bytes (limit 240), 200 absolute characters |
| TS | `tscheck.ts` (TypeScript 5.9.3, strict) over new files plus rewritten importers: same diagnostics as the baseline for `bft`, `hc`, 0 for `pb`, 0 for `fw`; `lossless.ts` (`bft`, `hc` 58 of 58 units, `pb` 99 of 99 modulo re-relativised `import()`), `lossless2.ts` (`fw`: 103 of 103 after reversing the setters, 5 declared patch units) |

## 5. Text readers re-pointed (bun suites run where the environment allows)

Rust test support: `🧯️test-support/📜️repo-text` got `repo_module_tree` / `repo_module_trees` (re-exported by the crate) and its four `*_SOURCE_FILES` lists were expanded with the child files of 25 split roots (parent first); tests switched to the module-tree readers: `🔬️wgpu-renderer-async-boundary/🧭️source-readers-and-frame-input-laws` (winit, `dexe2`, draw list, prepared), `🔬️wgpu-renderer-renderer-io-retained`, `🧊️wgpu-renderer-standalone` (winit), `Shell/🧪️tests/🔄️sync-owner-lifetime` (ProgramBridge), `🎨️wgpu-paint-policy` (4 allowlist paths now name `🖌️canvas2d/🪣️packet-image-and-blend`), `ui/🖌️render/🧪️tests/🔬️shader-contract-unit` (`include_str!` of root + 3 shader children), `ui/🧪️tests/🔬️targets-wgpu-flex-unit` (root + 4 children), infinite `world/🗃️model/🧪️tests/🔬️unit` (list expansion; `winit` via `read_joined`).

TS (switched to `readModuleTreeSource` / `sliceModuleTreeSource` unless noted): `🧪️tests/{🌳️tree-row-rects, 🖼️wgpu-raster-residency, ♻️frame-close, 🎨️icon-svg-lighting}`, `ui/🧪️tests/📦️prepared-close` (region markers repeat per child, so the slice uses `sliceModuleTreeSource`), `ui/🧪️tests/🔬️targets-wgpu-mounted-layout-unit`, `wgpu/📐️flex/🎟️close/🧪️tests`, `wgpu/🖍️draw/🧪️tests` (3 factory sourceAssertions: fixture + schema now name the two factory children, the first law is split into two rows because its symbols live in two children), `wgpu/🎟️native-caller/🧪️tests`, `🧱️elements/🗣️Interpreter/🧵️worker-cell/🧪️tests`, `IconRenderHost/🧪️tests/🖼️frame-presentation`, 3 shader oracles under `🔮️oracles/🎨️world3d-scene-shading`, plugin `🌐️browser-bundle/🏗️materialization/🧪️tests/{🏛️bootstrap, 🚪️lifetime}` (explicit plugin-bridge file list), the frame-worker laws `🧪️tests/📨️browser-frame-transport/{🔎️, 🖼️, 🧬️, 🫀️}` and `⏱️wgpu-worker-step-budget` through the new `📨️browser-frame-transport/📖️frame-worker-source/🟦️.ts` (fixed file order; the cross-module slice end markers were changed: `function progress` to `function requestHostIo`, `bootDeclarationsOpen = false;` to `setBootDeclarationsOpen(false);`). Library: `🔍️discovery/📖️source-access` `POLICY_SPLIT_SOURCE_ROOTS` gained the 37 split roots; `⚖️laws/🔬️interactivity-audit/🏁️run` and `🎛️commands/✅️verify/🎯️tool-job-lane` read module trees. Registries: `🔣️taxonomy.json` (`sourceModulePaths` and `inputPatterns` re-sorted bytewise, children of `plugin-bridge`, `browser-frame-transport`, `frame-worker` added), wgpu `📦️packages/🟦️typescript/📋️project.json` inputs.

Suites run (bun): `package-integration` 31 pass (includes the real worker bundle build with the schema-owned import list), `wgpu-intake-budget` 7, `wgpu-extension-dispatch` 21, `wgpu-selection-roundtrip` 4, `wgpu-typed-operation-reply` 6, `📨️browser-frame-transport/*` pass except `retained-clock-deadlines` and one image-decode test (`vi.stubGlobal` / `vi.setSystemTime` do not exist under bun), `⏱️wgpu-worker-step-budget` 15 pass / 4 fail (Chromium for Playwright not installed), `frame-close` 14, `wgpu-raster-residency` 5, `wgpu-document-owner-move` 11, `wgpu-raster-witness` 11, `scene-pointer-cancellation` 5, `bootstrap` 3, `prepared-close` 4, `flex/🎟️close` 2, `🖍️draw/🧪️tests` 9 pass / 5 fail (`tree-sitter-wasms` missing), `native-caller` 5 / 1 (same).
Not runnable here: `wgpu-document-reconcile`, `hub-projection`, `wgpu-accessibility-interaction` (`@dnd-kit/core` missing), `lifetime` second case and `worker-cell` lifecycle corpus (`SEMIO_TEST_ARTIFACT_DIR`), `tree-row-rects` last case (its text only exists in `🖌️paint/🔬️reference`, pre-existing). `interactivityMountedFrameTransactionSelfTests` fails earlier on `frame-job/📬️outcome/🧪️tests` (pre-existing).

## 6. Open risks

1. Nothing compiled (rule). The frame-worker `🤖️generated/🟨️.js` bundle (gitignored) is stale; regenerate through the worker generate target. The package-integration test already bundles the new modules successfully.
2. `frame-worker` setters are a code transformation (section 2); the vitest/Playwright laws that execute the worker (`⏱️wgpu-worker-step-budget` Chromium cases, `retained-clock-deadlines`) were not run.
3. `flex` belongs to `semio-framework-ui-wgpu-layout`, whose `🎟️close` tests now read `flex` and `mounted_layout` as module trees (bun: 2 pass).
4. `admitted-surface-map` (`Scenes/🧪️tests/🔬️wgpu-admitted-surface-map/🦀️.rs`) asserts `slots: Box<[…]>` in `scene-state`, text that is not in the file (pre-existing, unchanged).
5. A stale list in the infinite model unit test still names 7 non-existing Shell children (pre-existing).
6. Windows: `winit` and `pty`-style cfg code moved verbatim; only text equality / typeck on host cfg and wasm32 cover it.
7. The `retui` leftover (`RetainedInputReceiptReservation`, struct defined once per cfg) is a cfg-variant false positive: the root re-export is `#[cfg(any(test, feature="testkit"))]`.

## 7. Crates to compile

Native `cargo check -p <crate> --lib --tests` (with `--features testkit` where the crate declares it), and the same crates with `--target wasm32-unknown-unknown` for the wasm-capable ones (all ui wgpu crates, `program-bridge`, `kernel-protocol`, `kernel-pool`, `runtime-mailbox`, `frame-presentation`, `ui-engine`, `ui-engine-dump`, `agent-elements`, `scene-*`, `engine-*`, `dock`, `asset-decode`; `host-winit` is native only):

UI: `semio-framework-ui-wgpu-events`, `-wgpu-paint`, `-wgpu-engine`, `semio-framework-ui-prepared`, `-prepared-staging`, `semio-framework-ui-wgpu-draw-execution`, `-wgpu-raster`, `semio-framework-ui-draw-contract`, `semio-framework-ui-wgpu-mounted-layout`, `-wgpu-pipelines`, `-wgpu-engine-surface`, `semio-framework-ui-render-prepared-shaders`, `semio-framework-ui-wgpu-render` (also `--features canvas-raster`), `semio-framework-ui-wgpu-layout`, `semio-framework-ui-render-shader-contract`.

Renderer (`semio-framework-os-renderer-wgpu-` prefix): `asset-decode`, `dock`, `scene-ink`, `kernel-pool`, `engine-registry`, `program-bridge`, `runtime-mailbox`, `scene-state`, `ui-engine-dump`, `shell-actions`, `engine-graph`, `agent-elements`, `frame-presentation`, `ui-engine`, `shell-pointer`, `kernel-protocol`, `engine-text`, `engine-attach`, `scene-canvas2d`, `host-winit`, plus the aggregate `semio-framework-os-renderer-wgpu` and the test crates that read the moved sources: `runtime-mailbox-tests`, `frame-tests-1`, `shell-tests-docs-3`, `test-support`; and `semio-framework-os-infinite-world-model` (unit test list).

BIM: `semio-s-artifact-bim-model-tests-2`, `semio-s-artifact-bim-model-diff`.

TS laws after the compile: `package-integration`, `📨️browser-frame-transport/*`, `⏱️wgpu-worker-step-budget`, `wgpu-raster-residency`, `prepared-close`, `🖍️draw/🧪️tests`, `frame-close`, plus the interactivity audit lane (`verify interactivity`).
