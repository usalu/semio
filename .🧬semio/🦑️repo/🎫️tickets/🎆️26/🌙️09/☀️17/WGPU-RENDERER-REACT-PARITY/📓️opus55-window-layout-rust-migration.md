# Window layout unification: Rust migration (nothing compiled; build freeze)

Sub-reports: `📓️opus55-window-layout-rust-tui.md`, `📓️opus55-window-layout-rust-wgpu.md`.

## Decisions
- The contract `WindowLayout` / `WindowLayoutNode` (`Window`/`Stack`/`Split{axis}`, `Axis`, `WindowStackCorner`) in `semio-ui-contract-presentation` (re-exported by `semio-ui-contract`) is the only Rust layout model. No crate split: `semio-ui-layout` already depends on `semio-ui-contract` and nothing depends back, so there is no Rust cycle like the TS manifest/ui-contract one.
- Deleted: declaration `window_layout_tree` (`WindowLayoutRoot/Child/AxisNode/StackNode/WindowNode`, `even_window_layout`, `create_tab_stack_layout`, `collect_window_kind_ids_from_layout`, hand-written untagged `ToValue`), the TUI copy of the same five types and its `WindowStackCorner`, the generated TS rows for the old types.
- `NamedLayout` + `create_named_layout` + `merge_named_layouts` stay in `semio-ui-layout` (new `🧭️layout-contract/🏷️named-layout`), because they need `IconName` which the contract crate does not depend on.
- `semio_ui_declaration::layout` / `semio_ui_layout::layout` re-export `WindowLayout`, `WindowLayoutNode`, `WindowStackCorner`, `Axis` and the builders; `semio_plugin` re-exports `Axis`, `WindowLayoutNode`, builders, `collect_window_kind_ids`.
- wgpu: `DockNode` stays the view-only `f32` row/column/stack type (all drag/resize/maximize edits unchanged); `dock_from_window_layout(&WindowLayoutNode)` / `dock_node_to_layout_root` are the only conversions (Row = horizontal split, Column = vertical, sizes are fractions, normalised exactly as before). Maximize stays view state.
- TUI: `WindowLayout` is now a TUI view struct (contract `root` + `zoomed`); weights come from fractions.
- New contract builders in `window_layout_ops`: `create_window_layout`, `create_stack_layout`, `create_default_layout(ids, Axis, fraction sizes, titles)` (a lone window is the stack itself, like TS), `collect_window_kind_ids`, `collect_instance_titles`.
- Sizes are fractions everywhere. Plugin builders were converted by hand-rules (`"row"` -> `Axis::Horizontal`, `[60.0, 40.0]` -> `[0.6, 0.4]`, `100.0/3.0` -> `1.0/3.0`; puzzle3d/5d default-layout tests adjusted).

## Migrated
- Contract: ops builders; corpus test now imports `semio_ui_contract::window_layout_ops` and is removed from the presentation crate lib; unit test field rename; golden wire-format + value round-trip tests rewritten to the contract shape (and assert `kind: "row"` is rejected).
- Corpus test is now part of the workspace member crate `semio-renderer-wgpu-dock-laws` (`🛰️Dock/🧪️tests/🔬️laws/🦀️.rs`, `#[path]` module `window_layout_corpus_tests`; dev-dependency `semio-ui-contract` added).
- Generated declaration TS: `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` hand-edited (old five types deleted, `WindowLayout` imported from ui-contract); `🧬️schema/📽️projection/🦀️.rs` import line and `📖️rows-t-z` rows updated to match, so regeneration should be a no-op.
- Dock, Shell, ShellHost, plugin runtime/builder/tests, manifest, hub space, shared JSON fixtures (window-scope, app-mode-layouts, concrete-window-instances, safe-area): see wgpu sub-report.
- TUI + dashboard terminal: see TUI sub-report (adds `semio-ui-contract-presentation` dependency to tui `🧱️cell-model` and `🏛️frame`).
- ~265 plugin/hub files under `✏️s/` and `🌎️hub/` (builders + tests): scripted rewrite (scratchpad script, not kept), then the 19 files with pattern matches/field access hand-fixed (norm device-profiles/manifest-and-media/tests, en1993, details, xlsx, bim, puzzle 3d/5d, ifc inspector `layout()`, sourcing). Duplicate/missing `use` lines were deduplicated/added by a second pass.

## Known risks (check first on compile)
1. `Axis` is now in `semio_plugin`; a plugin file that glob-imports another `Axis` could clash.
2. (Resolved, see below.) One-child `Split` layouts were collapsed to their child.
3. Mechanically produced formatting is ugly (`Stack { size: None,\n active_window_id...`); run the formatter via `repo`.
4. TUI `window_node` takes `String`, not `impl Into<String>`.
5. `🐚️shell/🤖️generated/🟦️.js` was not found in the tree; regenerate it.

## Integrator run list
Generators (should be no-ops or produce only the already hand-edited result):
- `repo run @semio-tech/framework:generate` (manifest `🤖️generated/**`, `📜️ui-contract` TS)
- `repo run @semio-tech/ui-contract-rs:generate`
- the shell crate's generate target that writes `🐚️shell/🤖️generated/🟦️.js`

Compile (in order): `semio-ui-contract-presentation`, `semio-ui-contract`, `semio-ui-layout`, `semio-ui-declaration`, tui `🧱️cell-model`/`🏛️frame`/`engine`, `semio-plugin`, wgpu Dock + Shell crates, dashboard terminal crate, then a workspace `check` (plugins under `✏️s/` are excluded from the root workspace: check them via their own members or `repo` project graph; at least norm app-surface, en1993, puzzle 3d/5d, ifc inspectors, bim, sourcing, trinity, cad).

Tests:
- `semio-renderer-wgpu-dock-laws` (contains the contract corpus: 24 ops, 6 queries, 3 builders, roundtrip) and the Dock `app-mode-layouts` / `window-template-drag` / `stack-drop-destination` suites against the shared fixture.
- `semio-ui-layout` tests (`layout-wire-format`, `value-round-trip` includes: `window_layout_serializes_to_golden_json`, discriminator test, `named_layout_round_trips`).
- `semio-ui-contract-presentation` unit tests (`🔬️unit`), schema projection `validate()` (duplicate/mismatch of rows).
- Shell: `wgpu-shell-chrome-parity` (window-scope law, focus-window-of-kind, safe-area), `wgpu-window-pane-chrome`.
- TUI: `🧪️tests/*` (layout laws, chrome, pointer routing, drag preview) and dashboard `🧲️docking`.
- Plugin runtime: app-builder tests, manifest `app-label`/`window-view-context`.
- Plugin tests: norm `device-profiles`, `viewport-panels`, `live-acceptance`; en1993 editor devices test; puzzle3d/5d `default_layout_*`; details `split_layout_keeps_canvas_and_details_visible`.

## Follow-up: one-child splits and the schema law
- Collapsed every authored one-child `Split` to its child (sourcing viewer + editor, lowpoly editor); the norm visual layout builder now returns a bare stack/root when only one column or one pane in a column remains (`resize_to_root`, single-stack column). The remaining dynamic builders (`create_default_layout`, `even_layout`, device-profile tablet/mobile) always yield >= 2 children or a stack.
- New contract function `validate_window_layout` (`window_layout_ops`): root and split children are stacks/splits, stack children are windows with non-empty ids, splits have >= 2 children, sizes in (0, 1]. Re-exported from `semio_ui_layout::layout`.
- Regression guard at the single choke point: `AppBuilder::try_build_definition` now validates `default_layout` and every named layout (mode layouts included) and fails with `app-definition.invalid`, so every plugin/hub app that any existing test builds is checked. A hub/plugin that authors an invalid layout now fails its own app-build tests.
- Law: `builders_produce_schema_valid_layouts_and_the_validator_rejects_violations` in the corpus test (runs in `semio-renderer-wgpu-dock-laws`): builders for 0..3 windows validate; one-child split, window root, size 0 / 40.0, window-in-split are rejected.
- Check on compile: first run of plugin app-build tests may surface further invalid authored layouts (e.g. size > 1 left from percent values); fix those at the source.
