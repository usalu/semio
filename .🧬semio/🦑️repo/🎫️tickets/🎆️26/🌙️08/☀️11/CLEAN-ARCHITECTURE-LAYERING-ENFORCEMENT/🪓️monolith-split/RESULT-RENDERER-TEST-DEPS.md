# 🔌️ Result RENDERER-TEST-DEPS: renderer tests no longer pull the `semio-framework-ui` aggregate

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. Prefix `rtd`. Tools: `rtd-tools/` (section 9). Logs and raw outputs: `🗑️generated/rtd-*` (nothing deleted there). Rules read: `AGENTS.md`, `📓️EXECUTOR-RULES.md`, `📓️WAVE5-BRIEF.md` (incl. Wave 6), `TEST-CLOSURE-CENSUS.md` section 4, `RESULT-RENDERER-INTEGRATION-2.md`, `PLAN-RENDERER-WGPU-CRATES.md` section 9, `RESULT-W4-UI-RENDER.md` and the ui resolution tooling.

Aliases: `RT` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `T` = `RT/🎯️targets/🧊️wgpu` (the crate manifests live at `T/<folder>/📦️packages/🦀️rust/Cargo.toml`), `UI` = `🧰️framework/🔨️modules/🖱️ui`. The aggregate is `UI/📦️packages/🦀️rust` (`semio-framework-ui`); every renderer crate names it `ui_wgpu` (`package = "semio-framework-ui"`).

## 0. Verdict

| | Before | After |
| --- | ---: | ---: |
| renderer crates with a `ui_wgpu` edge (the census set) | 77 | 77 |
| crates whose census lists the aggregate as an over-limit dev-dependency (native; wasm labels identical) | **66** (67 rows: 64 direct, 2 transitive, plus 1 `plugin` row) | **4** (5 rows) |
| dev-edge `ui_wgpu` (`features = ["testkit"]`, camera: `["wgpu-engine"]`) | 64 crates | **1** (the top crate `semio-framework-os-renderer-wgpu`, which calls `Ui::apply_tree` and `Ui::publish_document`) |
| normal `ui_wgpu` edges | 76 | 64 (12 test-skeleton crates lost an unused edge) |
| test-side mentions of `ui_wgpu::…` in test modules (path uses and `use` statements) | 1,222 | 577 (every item a ui producer crate owns was re-pointed; what is left names items the aggregate itself defines) |
| over-limit test units (the crate's own lib-test, bin, integration units) | 0 | 0 (the top crate's lib-test is not measurable, section 7.1) |
| ui producer crates added as `[dev-dependencies]` | 0 | 84 edges to 21 producers; all 21 producers are inside the gate (largest `ui-prepared` 1 file / 144 KB / 2,584 lines, `ui-retained-tree` 3 / 164 KB / 2,890) |

- **No real cargo check was possible** (section 7.2: the launcher stops before cargo). Everything below is static evidence: the declaration check, the stub-layer differential for `--test`, rustfmt parsing, TOML parsing, path checks and the census.
- The remaining 4 crates keep a dev row through other crates, not through their own test code: the top crate (its tests need the aggregate's `testkit` methods), and `hub-workspace`, `contracts` and `program-bridge` (their dev-dependencies `doors`, `infinite_canvas` and `semio-framework-plugin-host` carry the aggregate in their own production closure). Section 4.
- 588 test-side uses (43 distinct items) name items that **no producer crate exposes**; they live in the aggregate's own modules (`input`, `events`, `engine`, `scene_slots`, `chrome`, `paint`, `cursor`, `widgets`, `tree_window`, `presence_bar`, `accessibility`, `host`). They are listed in section 3.4. The aggregate and its producers are untouched.

## 1. Found state

- The brief says the renderer crates dev-depend on the aggregate "with `wgpu-engine`". The manifests differ:
  - 76 crates have a **normal** edge `ui_wgpu = { …, features = ["wgpu-engine"] }`: 73 unconditional, `native-probe` and the top crate under `cfg(not(target_os = "wasi"))`, `browser-worker` under `cfg(all(target_arch = "wasm32", target_os = "unknown"))`.
  - 63 of them (the top crate included) also have a **dev** edge `ui_wgpu = { …, features = ["testkit"] }`. `testkit` in the aggregate is `["wgpu-engine", "semio-framework-ui-retained-tree/testkit"]`, so it is the "wgpu-engine dev-feature" of the brief.
  - `semio-framework-os-renderer-camera` has only a dev edge, `features = ["wgpu-engine"]` (64 dev edges in total).
  - No renderer crate forwards `ui_wgpu/testkit` from its own `testkit` feature (checked in all 94 manifests under `RT`).
- What the dev edge's `testkit` adds on top of the normal edge: exactly three public methods, `Ui::apply_tree` and `Ui::publish_document` (`UI/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1856,2024`, `cfg(any(test, feature = "testkit"))`) and `UiTree::apply_tree` (`UI/🌳️retained/🌳️tree/🔀️reconcile/🦀️.rs:1834`, behind the producer's own `testkit`, which the aggregate forwards). The other `feature = "testkit"` gates in the ui tree sit in producer crates whose `testkit` the aggregate does not forward.
- Token scan of every compiled source file of every crate (all three cfg labels, all features, lib and tests; arguments counted): `apply_tree(` and `publish_document(` with two arguments occur only in the top crate (21 calls in `RT/🧱️elements/🐚️Shell/🧪️tests/🖥️wgpu-display-conflicts-marketplace`, `RT/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection`, `…/🔬️wgpu-ui-command-wiring` and `…/🧵️worker-cell/🧪️tests`). The one call in `media-slots` (`tree.publish_document(document)`, one argument) is the ungated `UiTree::publish_document`.
- The census (`TEST-CLOSURE-CENSUS.md` section 4) counts a row for `semio-framework-ui` as a dev-dependency of a crate when the aggregate is reachable through **any dev edge** of the crate (its own or one of a dev-dependency's normal edges); it does not matter that the crate also has a normal edge. So the row disappears exactly when no dev path to the aggregate is left. The normal edges stay where production code (or an unmovable aggregate-owned test item) uses them; section 7.4 lists the ones that are unused.
- Baseline (`🗑️generated/rtd-census-before-*`): 77 crates measured, 67 native over-limit dev rows in 66 crates (`ui` 31 files / 1,109,244 bytes / 19,372 or 19,483 lines; plus `semio-framework-plugin` 128 / 2,073,206 / 27,863 for the top crate), 0 over-limit test units, 2 unmeasurable units (the top crate's lib-test, native and `wasm32-unknown-unknown`, section 7.1).
- The tree is shared: I snapshotted the manifests of all 77 crates and the compiled sources of the 21 crates with planned source edits before the first edit (outside the ticket tree, section 9). At the end the live files differ from that baseline only by this slice's edits (checked file by file): no foreign change touched them in that window.

## 2. Method

1. **Reference scan** (`rtd-tools/rtd-refs.ts`): the compiled `source` files of each crate (the census counter's closure, `kind == "source"`, not the `include_str!` text embeds) for the three labels with all features, `lib` and test targets. Mentions of `ui_wgpu` are found in comment/string-masked text (`rtd-rust.ts`) and classified: production, test-only file (not in the lib closure, or any file of a `#![cfg(test)]` crate), test region (`#[cfg(test)]` item inside a production file), or *feature-compiled* region (`cfg(any(test, feature = "testkit"))` and variants: compiled in non-test builds, so it may only use normal dependencies). The cfg evaluator is three-valued (`test` false, everything else unknown); a first two-valued version misclassified `all(not(target_arch = "wasm32"), any(test, feature = "testkit"))`, found by the declaration check below and reverted (3.5).
2. **Ownership** (`rtd-world.ts`, `rtd-verdict.ts`): a static name resolver over the first-party crates (module trees with `#[path]`, `pub use` chains, globs, `extern crate`). It resolves `ui_wgpu::<segments>` from the aggregate's lib root to the defining item, reports the defining crate and the **public path inside that crate** (shortest route through public modules and `pub use`), and records `cfg` gates on the route (none of the 215 producer-owned paths carries one). Probes: `wgpu::Theme` -> `semio-framework-ui-theme::Theme`; `wgpu::ActionDescriptor` -> `semio-framework-ui-layout::layout::ActionDescriptor`; `wgpu::InputState` -> aggregate (`UI/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:302`). The owner is the **defining** crate, not a facade that re-exports it (`ui-declaration` re-exports `ui-layout` items; `ui_contract` re-exports `contract-presentation`'s `OverlayKind`).
3. **Rewrite** (`rtd-apply.ts`): in test files and in test-only regions only, `ui_wgpu::wgpu::<item>…` becomes `<producer_ident>::<public path>…` (prefix replacement, the rest of the path is kept) and `use ui_wgpu::wgpu::{…}` statements are rebuilt (aggregate-owned leaves stay in one `use ui_wgpu::…`, producer leaves go to one `use` per producer, attributes in front of the statement are copied, visibility kept). Items owned outside the ui family (`Vec3`, `semio-framework-geometry-primitives`) and aggregate-owned items are left. Manifests are edited as text: the dev `ui_wgpu` line is removed unless a testkit-only method is called, the producers are added as `[dev-dependencies]` (`path`, relative, byte-exact directory names), and the normal edge is removed only in test-only crates (`#![cfg(test)]` roots, `shell-tests-*`, `*-tests`) that have no use left. Production code is not touched.
4. **Import order**: rustfmt would sort the new imports into the block. `rtd-fmt-imports.py` applies only the rustfmt hunks that sit in `use` blocks and mention a new import, with a statement-multiset proof per file (applied to 16 files; the 4 files whose proof failed keep the as-written order: `shell-tests-chrome-6`, `shell-tests-input-3` and the two `🛰️Dock/🧪️tests` files).

## 3. What changed

### 3.1 Manifests (67 manifests, 75 edges removed, 84 edges added)

- **63 dev edges removed** (`ui_wgpu` with `testkit`, and camera's with `wgpu-engine`), 1 kept (top crate, section 3.3).
- **12 normal edges removed** in test-only skeleton crates that mention the aggregate nowhere: `shell-tests-chrome-1/2/3/5`, `shell-tests-input-1/2`, `shell-tests-windows-1`, `shell-tests-docs-1` (empty bin roots), and the companions `scene-ink-tests`, `engine-attach-tests`, `ui-engine-tests`, `runtime-mailbox-tests`.
- **84 dev-dependency edges added** to 19 crates (21 distinct producers): `ui-draw-contract` 11, `ui-layout` 11, `ui-text` 11, `ui-theme` 9, `ui-scene-views` 6, `ui-scene-plane` 5, `ui-declaration` 5, `ui-retained-tree` 4, `ui-scene-camera` 3, and 2 each for `ui-arena`, `ui-action`, `ui-prepared`, `ui-prepared-staging`, `ui-scene-mesh`, `ui-scene-pass`, `ui-scene-world3d`, 1 each for `ui-scene-node-graph`, `ui-scene-picking`, `ui-scene-surface`, `ui-surface-identity`, `ui-contract-presentation`. All 151 `semio-framework-ui-*` path dependencies in the 77 manifests (old and new) name directories the git index knows byte for byte (`rtd-check-tracked.ts`).
- `Cargo.lock` (repository root): the dependency arrays of the 31 packages whose edges changed were patched to the exact result of `cargo update --workspace --offline` for this change (`rtd-lock.ts`, mode 600 kept). **The lock is still stale for 40 renderer packages for reasons that predate this slice** (section 7.3), so a `--locked` run needs the update first.

### 3.2 Test modules (75 files in 19 crates)

- 54 `use` statements rebuilt and 599 qualified paths re-pointed (653 edit sites; the reverted statement of 3.5 is not counted); the test-side mention count fell from 1,222 to 577.
- Per-crate table below (columns: test-side mentions before -> after, dev edge, normal edge, dev-dependencies added). `n/a` = the crate never had that edge.

| crate (`semio-framework-os-renderer-…`) | test mentions | dev edge | normal edge | dev-deps added |
| --- | --- | --- | --- | --- |
| `wgpu-scene-ink` | 18 -> 4 | removed | kept | draw-contract, layout, scene-plane, text |
| `wgpu-shell-tests-windows-2` | 4 -> 2 | removed | kept | layout, scene-camera, theme |
| `wgpu-shell-tests-chrome-4` | 35 -> 18 | removed | kept | arena, draw-contract, layout, retained-tree, text, theme |
| `wgpu-shell-tests-chrome-6` | 11 -> 8 | removed | kept | declaration, draw-contract, layout, text, theme |
| `wgpu-shell-tests-input-3` | 5 -> 5 | removed | kept | draw-contract, layout, text, theme |
| `wgpu-media-slots` | 11 -> 3 | removed | kept | retained-tree |
| `wgpu-scene-canvas2d` | 46 -> 19 | none | kept | action, declaration, draw-contract, layout, prepared-staging, scene-plane, text, theme |
| `wgpu-engine-board` | 1 -> 0 | none | kept | layout |
| `wgpu-icon-host` | 22 -> 6 | removed | kept | draw-contract, prepared, scene-camera, scene-mesh, scene-pass, text, theme |
| `wgpu-scene-text-vfs` | 32 -> 24 | removed | kept | draw-contract, layout, scene-views, text |
| `wgpu` | 939 -> 460 | kept (testkit) | n/a | action, arena, contract-presentation, declaration, draw-contract, layout, prepared, prepared-staging, retained-tree, scene-camera, scene-mesh, scene-node-graph, scene-pass, scene-picking, scene-plane, scene-surface, scene-views, scene-world3d, surface-identity, text, theme |
| `wgpu-scene-feeds` | 39 -> 10 | removed | kept | draw-contract, scene-views, text |
| `wgpu-ui-engine` | 8 -> 0 | removed | kept | declaration, scene-plane, scene-views, scene-world3d |
| `wgpu-agent-elements` | 2 -> 0 | none | kept | theme |
| `wgpu-dock` | 3 -> 0 | removed | kept | declaration, layout |
| `wgpu-shell-tests-docs-3` | 5 -> 2 | removed | kept | draw-contract, text, theme |
| `wgpu-scene-tables` | 21 -> 14 | removed | kept | draw-contract, scene-views, text |
| `wgpu-scene-render` | 10 -> 0 | removed | kept | layout, scene-plane, scene-views |
| `camera` | 8 -> 0 | removed | none | retained-tree |

Crates with the dev edge dropped and nothing else changed (36; their tests did not use `testkit` and mention no aggregate item): `wgpu-shell-history`, `wgpu-shell-async`, `wgpu-shell-dock`, `wgpu-shell-view-chrome`, `wgpu-shell-chrome-state`, `wgpu-shell-prefs`, `wgpu-host-winit`, `wgpu-scene-state`, `wgpu-shell-commands`, `wgpu-host-base`, `wgpu-shell-state`, `wgpu-shell-doc-open`, `wgpu-shell-docs`, `wgpu-shell-directory`, `wgpu-runtime-mailbox`, `wgpu-frame-presentation`, `wgpu-shell-actions`, `wgpu-shell-sync`, `wgpu-canvas2d-host`, `wgpu-ui-engine-dump`, `wgpu-engine-attach`, `wgpu-host-plumbing`, `wgpu-shell-windows`, `wgpu-engine-graph`, `wgpu-shell-input`, `wgpu-shell-view-paint`, `wgpu-shell-pointer`, `wgpu-shell-view-overlays`, `wgpu-shell-session`, `wgpu-frame-actions`, `wgpu-shell-tests-docs-2`, `wgpu-shell-plugins`, `wgpu-shell-view-panels`, `wgpu-shell-window-actions`, `wgpu-program-bridge`, `wgpu-shell-surfaces`.

Test-only skeleton crates (12) with both edges removed: `wgpu-shell-tests-chrome-3`, `wgpu-shell-tests-chrome-1`, `wgpu-shell-tests-chrome-2`, `wgpu-shell-tests-chrome-5`, `wgpu-shell-tests-input-1`, `wgpu-shell-tests-input-2`, `wgpu-shell-tests-windows-1`, `wgpu-shell-tests-docs-1`, `wgpu-scene-ink-tests`, `wgpu-engine-attach-tests`, `wgpu-ui-engine-tests`, `wgpu-runtime-mailbox-tests`.

### 3.3 The top crate keeps its dev edge

`semio-framework-os-renderer-wgpu` (the facade whose tests are the 32 Shell/Interpreter tests that stay in the monolith, plus the hubs' test hosts) calls `Ui::apply_tree` and `Ui::publish_document` 21 times, which only exist with the aggregate's `testkit`. Its dev edge stays (`features = ["testkit"]`); 511 producer-owned test-side uses in 42 files were re-pointed and 21 producer dev-dependencies were added; 460 mentions of aggregate-owned items remain. The census still lists the aggregate (and, transitively, `semio-framework-plugin`) for this crate: that is its own need. A `Ui::apply_tree`/`publish_document` pair in a producer (for example an `ui-engine-testkit` crate that owns the `Ui` test seam) would end it; the aggregate is the peer's.

### 3.4 Items no single producer exposes (aggregate-owned; left on `ui_wgpu`)

588 test-side uses, 43 distinct items, by the aggregate module that defines them (definition lines are in `UI/🎯️targets/🧊️wgpu/<module>/🦀️.rs`):

| module | items (test-side uses) | crates |
| --- | --- | --- |
| `📥️input` (target-neutral per its own docs) | `KeyAction` 190, `InputState` 142, `PointerModifiers` 17, `HitKind` 13, `HitTarget` 3, `ClipboardIntentSource` 2 (367) | 11 crates: the top crate, `icon-host`, `scene-canvas2d`, `scene-feeds`, `scene-ink`, `scene-tables`, `scene-text-vfs`, `shell-tests-chrome-4`, `-chrome-6`, `-docs-3`, `-input-3` |
| `⚡️events` | `UiEvent` 48, `UiCommand` 23, `AccessibilityUiEvent` 23, `PointerButton` 21, `EventModifiers` 13, `DragPayload` 1 (129) | top crate |
| `⚙️engine` | `Ui` 15, `UiFrameStep` 4, `UiLayoutStep` 2, `engine::UiDocumentIngressStatus` 2, plus the 21 `testkit` calls | top crate |
| `📨️scene_slots` | `ScenePaintStep` 11, `ScenePaintCursor` 4, `SceneHost` 2, `SceneSlot` 2 | top crate, `scene-canvas2d` |
| `🖥️chrome` | `UiDriverDrag` 13, `UiDriverLabels`, `UiDriverTooltips`, `UiDriverChrome`, `UiDriverLabelTier` (17) | top crate, `shell-tests-chrome-6` |
| `🖌️paint` | `RETAINED_NODE_TEXT_MAX_BYTES` 2, `RetainedGlyphCursor`, `RetainedGlyphStep`, `paint_retained_glyph_step` | `shell-tests-chrome-6` |
| `👆️cursor`, `🪀️widgets`, `🏃️host`, `♿️accessibility` | `SemioCursor` 7; `WidgetNode`, `SelectItem`, `render_widget`, `WidgetContext`; `ClipboardContent`; `accessibility::accessibility_projection` | `SemioCursor`: top crate, `host-plumbing`, `media-slots`; the rest: top crate |
| elements mounted by the aggregate | `tree_window::TreeWindowRequest` 2 (`🧱️elements/🌳️Tree/🪟️window`); `presence_bar_chip_text` 4, `PresencePeerRow` 2, `presence_empty_label`, `PresenceRole` (`🧱️elements/👥️PresenceBar`) | top crate, `shell-tests-windows-2`, `shell-tests-docs-2` |
| owned outside the ui family | `Vec3` 5 (`semio-framework-geometry-primitives`, reached through `ui-scene-camera`'s math) | top crate, `icon-host`, `shell-tests-windows-2` |

Reading: `input` and `events` are 496 of the 588 uses. A `semio-framework-ui-input` producer (the `📥️input` module is documented as 100 percent target-neutral) and an `…-events` producer would let nearly every remaining test mention move off the aggregate; the six `#![cfg(test)]` bins (`shell-tests-windows-2`, `-chrome-4`, `-chrome-6`, `-input-3`, `-docs-2`, `-docs-3`) hold the aggregate as a **normal** edge only for these items (their whole crate is test code); that is why the census shows no dev row for them. If those edges became dev edges the rows would return.

### 3.5 Reverted by the declaration check

Three `use` statements in `RT/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🦀️.rs` (`#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]`) had been re-pointed to dev-only crates. They are compiled in non-test builds when `ui-engine/testkit` is enabled by a dependent, where dev-dependencies do not exist. I restored the original `use ui_wgpu::wgpu::{Theme, UiComponentSceneNode, UiPresence};` (the file is byte-identical to the baseline again) and removed the then-unused dev-dependencies `ui-theme` and `ui-layout` from `ui-engine` and from the lock.

## 4. Census, before and after (native; the two wasm labels give the same counts)

`rtd-tools/rtd-census.ts` is the unchanged counter and test-closure rules of `tcc-tools/tcc-census.ts` over an explicit manifest list (the original writes fixed `tcc-*` names and reads the stale CSV, so it was not re-run). Outputs: `🗑️generated/rtd-census-{before,after,final,producers}-*`.

| | before | after (final run after the last edit) |
| --- | ---: | ---: |
| crates measured | 77 | 77 |
| over-limit dev rows, native | 67 | 5 |
| crates with an over-limit dev row | 66 | 4 |
| over-limit test units (any label) | 0 | 0 |
| unmeasurable units | 2 | 2 (same unit, section 7.1) |

Remaining rows and why (`rtd-tools/rtd-path.ts` prints the dev path the census follows):

| crate | row | dev path |
| --- | --- | --- |
| `renderer-wgpu` (top) | `ui` direct; `plugin` transitive | own dev edge `ui_wgpu` (`testkit`, section 3.3); `plugin` through the dev-dependencies that carry it |
| `wgpu-hub-workspace` | `ui` transitive | dev `renderer-wgpu-doors` -> `host-base` (production edge `ui_wgpu`); `doors` is used by a test (`🏘️hub-workspace/🦀️.rs:6`) |
| `wgpu-contracts` | `ui` transitive | dev `infinite_canvas` (`semio-framework-os-infinite`, production edge to the aggregate); used through `#[cfg(test)] extern crate infinite_canvas as infinite_world;` in its root |
| `wgpu-program-bridge` | `ui` transitive | dev `semio-framework-plugin-host` -> `semio-framework` -> aggregate; used by `🧪️tests/📦️frame/🦀️.rs:20` |

None of the three transitive cases is the crate's own use of the aggregate: they end when `host-base`, `semio-framework-os-infinite` and `semio-framework` stop depending on the aggregate in production (other slices). All other test units stay within the gate: the largest lib-test unit among the touched crates is `shell-actions` with 16 files / 227,274 bytes / 3,385 lines (unchanged; limits 32 / 262,144 / 4,000); the rewrites added at most 6 lines to a unit (`rtd-census-final-crates.tsv`).

## 5. Verification

All static; nothing was compiled.

| # | Check | Command | Result |
| --- | --- | --- | --- |
| 1 | Resolver | `bun rtd-tools/rtd-plan.ts 🗑️generated/rtd-refs.json 🗑️generated/rtd-plan.json`, `bun rtd-tools/rtd-probe.ts wgpu::Theme wgpu::OverlayKind …` | 298 distinct test-side paths resolved (215 producer-owned, the rest aggregate-owned); none of the producer routes carries a `cfg` gate (`gatesOf` does see gates: it reports `#[cfg(all(target_arch …, not(target_env …)))]` on `OffscreenPresentToken`); owners agree with the definitions read by hand for `Theme`, `Rect`, `UiNode`, `ActionDescriptor`, `OverlayKind`, `BoundedActionFault`, `FontAtlas`, `DrawList`, `Camera3d` |
| 2 | Declaration check (every `semio_framework_ui_*` / `ui_wgpu` use in compiled source has a manifest edge of the right kind: normal for production and feature-compiled code, normal or dev for test code), all 77 crates, 3 labels, all features | `bun rtd-tools/rtd-verify-closure.ts 🗑️generated/rtd-manifests.txt 🗑️generated/rtd-verify-closure.json` | `all ui identifiers declared`. It found 3 violations on its first run (3.5), fixed; the second run is clean |
| 3 | Idempotence | `rtd-refs.ts` + `rtd-plan.ts` + `rtd-apply.ts` (dry) on the final tree | 0 pending rewrites, 0 pending manifest changes |
| 4 | Stub-layer differential, `--test` (and `--lib`, and both with every crate feature as `--cfg`), baseline files against live files, externals stubbed per side from that side's manifest, 21 crates | `RTD_WORK=<scratch> python3 rtd-tools/rtd-oracle.py all` (outputs `🗑️generated/rtd-oracle-*.err`), `python3 rtd-tools/rtd-oracle-compare.py` | 46 A/B pairs (21 crates; configurations `test` and `lib`, and with every crate feature as `--cfg` where the crate has features): **37 identical, 9 differ only in typeck-class messages, 0 resolution-class differences** (resolution class = E0412, E0425, E0422, E0423, E0432, E0433, E0603, E0616, E0624, E0252, E0255, E0428 and the other name-resolution codes, plus errors without a code). Seven of the nine differing pairs (`engine-board`, `icon-host` x2, `scene-canvas2d` x2, `scene-render`, `shell-tests-chrome-4`) change the error count by at most 3 or only the wording (`for enum X` against `for struct X`, `must implement PartialEq<X>` against `must implement PartialEq`). The other two are the top crate (`test`, `test+features`): 10,617 against 11,083 errors in a 442-file closure, the extra 466 being E0061 and E0599 on stubbed functions and associated items. Both effects have one cause: the stub generator infers a type's kind and a function's arity from the paths used under one stub crate, and re-pointed items now sit under other stub crates, so a call that was one error under the aggregate stub can be one error under each of several stubs. They carry no information about the real types. The top crate's closure ran with two sandbox adjustments (`pub extern crate X as Y;` becomes `pub use X as Y;`, and a linear-time arity and trait scan; counts verified equal to the original scan on `scene-feeds` and `icon-host`) because the stock tools abort on the first and need hours for the second. |
| 5 | Oracle sensitivity (seeded) | `RTD_DROP_EXT=semio_framework_ui_retained_tree python3 rtd-tools/rtd-oracle.py camera --cfgs test` | caught: 8 new `E0433 cannot find module or crate semio_framework_ui_retained_tree` (the dev-dependency missing from the stub list is an error, so an undeclared producer cannot hide) |
| 6 | rustfmt parse of the 75 changed files | `rustfmt +nightly-2026-07-20 --edition 2021 --unstable-features --skip-children --check <file>` | 0 parse errors in 75 files, before and after the import-order pass. Style: 335 hunks at baseline, 355 now, all in 14 files that were not rustfmt-clean; the extra ones re-wrap lines that the longer producer paths pushed past `max_width = 250`. Six files that were clean at baseline are clean again after the import pass |
| 7 | TOML parse, path dependencies, byte-exact names | `bun rtd-tools/rtd-check-manifests.ts 🗑️generated/rtd-manifests.txt`; `bun rtd-tools/rtd-check-tracked.ts …` | `ALL OK 77 manifests`; 151 `semio-framework-ui-*` path deps all in the git index |
| 8 | Source-text laws | `git grep` for `ui_wgpu` in `*.ts/*.tsx/*.json` and in string literals; `rtd-tools/rtd-strmentions.ts` on the rewritten files | the only pins are `TS worker-cell` (`UiEngineCell(WorkerCell<Option<ui_wgpu::wgpu::Ui>>)` in production code, untouched, the aggregate-owned `Ui` stays) and one *absence* assertion (`!INTERPRETER_SOURCE.contains("reserve_actions(ui_wgpu::wgpu::action::…")`); no law reads a rewritten test file |
| 9 | WP0 static gates | `bun rwp0-tools/verify_static.ts` | 88 manifests, 1,993 path dependencies resolved, 1 failure: `wgpu member block is byte-sorted` (the member block of the root `Cargo.toml`; I did not touch it, section 7.3) |
| 10 | Lock consistency of my edges | `bun rtd-tools/rtd-lock-check.ts 🗑️generated/rtd-manifests.txt` | no drift involves a `semio-framework-ui*` entry I added or removed; 40 packages drift for other, older reasons (7.3) |
| 11 | Baseline integrity | file-by-file diff of snapshot against live (`rtd-final-stats.ts`) | 75 changed source files, 67 changed manifests; no change by anyone else in them |

## 6. Real checks for the compile queue

The launcher does not reach cargo at the moment (7.2). When it does, with the fleet mutex (`mkdir 🗑️generated/cargo-mutex.lock`, write `rtd <pid> <date>` to `owner`, `rm -f cargo-mutex.lock/owner && rmdir cargo-mutex.lock` after the call):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- update --workspace --offline
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --lib --tests --keep-going --message-format=short -p semio-framework-os-renderer-wgpu-scene-ink -p semio-framework-os-renderer-wgpu-shell-tests-windows-2 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-4 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-6 -p semio-framework-os-renderer-wgpu-shell-tests-input-3 -p semio-framework-os-renderer-wgpu-media-slots -p semio-framework-os-renderer-wgpu-scene-canvas2d -p semio-framework-os-renderer-wgpu-engine-board -p semio-framework-os-renderer-wgpu-icon-host -p semio-framework-os-renderer-wgpu-scene-text-vfs -p semio-framework-os-renderer-wgpu -p semio-framework-os-renderer-wgpu-scene-feeds -p semio-framework-os-renderer-wgpu-ui-engine -p semio-framework-os-renderer-wgpu-agent-elements -p semio-framework-os-renderer-wgpu-dock -p semio-framework-os-renderer-wgpu-shell-tests-docs-3 -p semio-framework-os-renderer-wgpu-scene-tables -p semio-framework-os-renderer-wgpu-scene-render -p semio-framework-os-renderer-camera -p semio-framework-os-renderer-wgpu-shell-tests-chrome-3 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-1 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-2 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-5 -p semio-framework-os-renderer-wgpu-shell-tests-input-1 -p semio-framework-os-renderer-wgpu-shell-tests-input-2 -p semio-framework-os-renderer-wgpu-shell-tests-windows-1 -p semio-framework-os-renderer-wgpu-shell-tests-docs-1 -p semio-framework-os-renderer-wgpu-scene-ink-tests -p semio-framework-os-renderer-wgpu-engine-attach-tests -p semio-framework-os-renderer-wgpu-ui-engine-tests -p semio-framework-os-renderer-wgpu-runtime-mailbox-tests
```

The root workspace owns these crates, so no `--manifest-path`. Batch 1 above is the 19 crates with source edits plus the 12 skeleton crates (their lib check proves the removed edge was unused; a bin crate with `#![cfg(test)]` needs `--tests` to compile anything). Batch 2 (the 36 crates that only lost the dev edge; `--tests` proves no test used `testkit`):

`-p semio-framework-os-renderer-wgpu-shell-history -p semio-framework-os-renderer-wgpu-shell-async -p semio-framework-os-renderer-wgpu-shell-dock -p semio-framework-os-renderer-wgpu-shell-view-chrome -p semio-framework-os-renderer-wgpu-shell-chrome-state -p semio-framework-os-renderer-wgpu-shell-prefs -p semio-framework-os-renderer-wgpu-host-winit -p semio-framework-os-renderer-wgpu-scene-state -p semio-framework-os-renderer-wgpu-shell-commands -p semio-framework-os-renderer-wgpu-host-base -p semio-framework-os-renderer-wgpu-shell-state -p semio-framework-os-renderer-wgpu-shell-doc-open -p semio-framework-os-renderer-wgpu-shell-docs -p semio-framework-os-renderer-wgpu-shell-directory -p semio-framework-os-renderer-wgpu-runtime-mailbox -p semio-framework-os-renderer-wgpu-frame-presentation -p semio-framework-os-renderer-wgpu-shell-actions -p semio-framework-os-renderer-wgpu-shell-sync -p semio-framework-os-renderer-wgpu-canvas2d-host -p semio-framework-os-renderer-wgpu-ui-engine-dump -p semio-framework-os-renderer-wgpu-engine-attach -p semio-framework-os-renderer-wgpu-host-plumbing -p semio-framework-os-renderer-wgpu-shell-windows -p semio-framework-os-renderer-wgpu-engine-graph -p semio-framework-os-renderer-wgpu-shell-input -p semio-framework-os-renderer-wgpu-shell-view-paint -p semio-framework-os-renderer-wgpu-shell-pointer -p semio-framework-os-renderer-wgpu-shell-view-overlays -p semio-framework-os-renderer-wgpu-shell-session -p semio-framework-os-renderer-wgpu-frame-actions -p semio-framework-os-renderer-wgpu-shell-tests-docs-2 -p semio-framework-os-renderer-wgpu-shell-plugins -p semio-framework-os-renderer-wgpu-shell-view-panels -p semio-framework-os-renderer-wgpu-shell-window-actions -p semio-framework-os-renderer-wgpu-program-bridge -p semio-framework-os-renderer-wgpu-shell-surfaces`

Expectations to attribute: the top crate's `--tests` stops on the stale `include_str!` of 7.1 until its owner fixes that path; any `E0432`/`E0433` on a `semio_framework_ui_*` path would be a wrong re-point (the declaration check says every identifier is declared, the resolver says every path is public; neither type-checks).

## 7. Foreign errors, observations and hand-offs

7.1 **Top crate lib-test unmeasurable** (census, both runs): `Missing Rust source input: 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🦀️.rs`. `RT/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:9` embeds that file (`DRAW_SOURCE`); of `UI/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline` only the producer folders `🎬️execution` and `🏭️factory` exist now. `RT/🧪️tests/🖼️wgpu-raster-residency/🟦️.ts:177` and `…/🧫️fixtures/🖼️wgpu-raster-witness/🔣️.json` name the same path. Owner: the renderer test split and the ui draw peer; not touched here.
7.2 **Launcher**: `bun repo cargo … -- metadata --locked --offline --format-version 1` (once, under the mutex, 15:09) ends `error: Exact canonical persistent Native publication required` at `inspectPersistentReceiver` (`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts:111`) before cargo starts. Mutex released. No compile was attempted.
7.3 **Stale lock and static gate failures that predate this slice**: 40 of the 77 `Cargo.lock` packages lack edges their manifests already declare (sibling renderer crates; `semio-framework-ui-wgpu-render` in several of them; `native-probe`'s conditional `semio-framework-ui`). `verify_static.ts` fails `wgpu member block is byte-sorted`. Neither touches ui producer edges.
7.4 **Unused normal edges (production crates, not mine to remove)**: `shell-history` and `shell-plugins` declare `ui_wgpu` (`wgpu-engine`) and mention it nowhere in any compiled source or test (all labels, all features). Also unused, already before this slice: `semio-framework-ui-viewport` in 35 crates, `semio-framework-ui-locale` in 16, and `semio-framework-ui-contract-validation` and `semio-framework-ui-testing-paint-policy` in the top crate (`🗑️generated/rtd-verify-closure.json`, field `unusedUi`).
7.5 **Peer state**: the 84 new edges are plain relative `path` lines to producer directories as they are on disk now; if the ui peer renames or moves a producer crate directory, those lines (and the manifest of the aggregate) move together.
7.6 **Shared files**: `RT/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🖌️paint2d/🦀️.rs` and `…/🗣️Interpreter/…/🧩️retained-ui-engine/🦀️.rs` belong to two crates (their own and the top crate); the declaration check covers both closures. One accidental `bun scan` in the repository root at the start returned at once without output (no such script; nothing ran).

## 8. Hand-offs

- **ui peer**: carve `📥️input` (and the `⚡️events` value types) into producer crates and re-export them from the aggregate (496 of the 588 remaining test mentions, and the production edges); give the `Ui` test seam (`apply_tree`, `publish_document`) a producer that tests can dev-depend on; then the top crate loses its dev edge and the 6 bins can drop the aggregate.
- **Owners of `host-base`, `semio-framework-os-infinite`, `semio-framework` (plugin-host closure)**: their production edge to the aggregate is what keeps `hub-workspace`, `contracts`, `program-bridge` in the census.
- **Renderer test owner**: 7.1; optionally the 14 files whose long lines now exceed 250 columns (`rustfmt` re-wrap).
- **Compile queue**: section 6.

## 9. Files and tools

Edited: the 67 manifests of section 3.1 (every one under `T/<folder>/📦️packages/🦀️rust/Cargo.toml` plus `RT/🧱️elements/🎞️Scenes/⏱️camera/📦️packages/🦀️rust/Cargo.toml`), the repository `Cargo.lock`, and these 75 test/support source files (all under `RT/`; per crate: `camera` 1, `wgpu` 42, `agent-elements` 2, `dock` 1, `engine-board` 1, `icon-host` 3, `media-slots` 2, `scene-canvas2d` 2, `scene-feeds` 2, `scene-ink` 2, `scene-render` 1, `scene-tables` 3, `scene-text-vfs` 2, `shell-tests-chrome-4` 2, `shell-tests-chrome-6` 3, `shell-tests-docs-3` 2, `shell-tests-input-3` 1, `shell-tests-windows-2` 2, `ui-engine` 1):

- `RT/🎯️targets/🧊️wgpu/🌆️shell-tests-windows-2/🦀️.rs`
- `RT/🎯️targets/🧊️wgpu/🌴️shell-tests-chrome-4/🦀️.rs`
- `RT/🎯️targets/🧊️wgpu/🍃️shell-tests-chrome-6/🦀️.rs`
- `RT/🎯️targets/🧊️wgpu/🍰️shell-tests-input-3/🦀️.rs`
- `RT/🎯️targets/🧊️wgpu/🎬️media-slots/🦀️.rs`
- `RT/🎯️targets/🧊️wgpu/🥣️shell-tests-docs-3/🦀️.rs`
- `RT/🎯️targets/🧊️wgpu/🧊️renderer/📥️mounts/🧵️wp3-frame-core/🧪️test-hosts/🔎️interpreter-introspection/🦀️.rs`
- `RT/🎯️targets/🧊️wgpu/🧊️renderer/📥️mounts/🧵️wp3-frame-core/🧪️test-hosts/🧷️interpreter-command-wiring/🦀️.rs`
- `RT/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `RT/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs`
- `RT/🧪️tests/🔬️wgpu-media-slots-unit/🦀️.rs`
- `RT/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs`
- `RT/🧪️tests/🔬️wgpu-winit-app-p3c/🦀️.rs`
- `RT/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs`
- `RT/🧪️tests/🧾️frame-action-ledger/🦀️.rs`
- `RT/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🖌️paint2d/🦀️.rs`
- `RT/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`
- `RT/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs`
- `RT/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs`
- `RT/🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs`
- `RT/🧱️elements/⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs`
- `RT/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs`
- `RT/🧱️elements/⚙️EngineCanvas/🧪️tests/🫳️node-graph-gestures/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/⏱️camera/🧪️tests/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-block-list/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-canvas2d/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-diff-view/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-event-feed/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-graph-timeline/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-icon-render/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-ink-canvas/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-raster-frame-cost/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-surface-context-menu/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-table/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-text-editor/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-virtual-file-system/🦀️.rs`
- `RT/🧱️elements/🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🧱️chrome-paint-and-navbar/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/👕️board-presence/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-chrome-overlays-tour/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-command-registry/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-retained-chrome-text-laws/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-tutorial/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🖥️wgpu-display-conflicts-marketplace/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🚪️wgpu-host-door-remainder/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs`
- `RT/🧱️elements/🐚️Shell/🧪️tests/🪟️window-lifecycle-template-drag/🦀️.rs`
- `RT/🧱️elements/🖼️IconRenderHost/🧪️tests/📤️export-batch/🦀️.rs`
- `RT/🧱️elements/🖼️IconRenderHost/🧪️tests/📤️gpu-export/🦀️.rs`
- `RT/🧱️elements/🖼️IconRenderHost/🧪️tests/📤️svg-export/🦀️.rs`
- `RT/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
- `RT/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs`
- `RT/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-render-plan-validator/🦀️.rs`
- `RT/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs`
- `RT/🧱️elements/🗣️Interpreter/🧪️tests/🖼️wgpu-ui-image-loading/🦀️.rs`
- `RT/🧱️elements/🗣️Interpreter/🧵️worker-cell/🧪️tests/🦀️.rs`
- `RT/🧱️elements/🚦️AgentPresence/🧪️tests/🔬️wgpu-unit/🦀️.rs`
- `RT/🧱️elements/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs`
- `RT/🧱️elements/🛰️Dock/🧪️tests/🪟️app-mode-layouts/🦀️.rs`
- `RT/🧱️elements/🤖️AgentApprovals/🧪️tests/🔬️wgpu-unit/🦀️.rs`

No production source, no ui crate, no registry, no launch route, no taxonomy name changed.

Tools (`rtd-tools/`, Bun and Python; read-only except `rtd-apply.ts`, `rtd-fmt-imports.py --apply`, `rtd-lock.ts --apply`): `rtd-lib.ts` (exported copy of the census logic), `rtd-census.ts`, `rtd-refs.ts`, `rtd-rust.ts`, `rtd-world.ts`, `rtd-verdict.ts`, `rtd-plan.ts`, `rtd-apply.ts`, `rtd-snap.ts`, `rtd-verify-closure.ts`, `rtd-check-manifests.ts`, `rtd-check-tracked.ts`, `rtd-lock.ts`, `rtd-lock-check.ts`, `rtd-oracle.py` (reuses `frs-tools/oracle.py` and `stubgen.py`, with a linear-time arity/trait scan and `pub extern crate` handling so the 440-file top crate runs), `rtd-oracle-compare.py`, `rtd-fmt-imports.py`, `rtd-final-stats.ts`, `rtd-table.ts`, `rtd-path.ts`, `rtd-ident-use.ts`, `rtd-strmentions.ts`, `rtd-probe.ts`, `scan-ui-edges.ts`, `list-pkgs.ts`. Baseline snapshot and oracle sandboxes are **not** in the ticket tree: copies of tracked sources exceed the path limit there (`AGENTS.md`), so they live in the session scratch directory (`$RTD_WORK/base`, `$RTD_WORK/oracle`; wiped on restart; regenerate with `rtd-snap.ts` only from a tree before the edits). I moved the first copies out of `🗑️generated/rtd-work` for that reason; the only file left there is `Cargo.lock.before-patch*` (pre-patch copies of the lock).
