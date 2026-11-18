# Round 3 native fixes (F1-F9)

Paths under `🧰️framework/🔨️modules/🖱️ui/` (`WGPU` = `🎯️targets/🧊️wgpu`).

- F1 kit Field: `WGPU/🪀️widgets/🦀️.rs` measures and renders through `field_chrome_metrics` (label cell, bottom hairline via `push_shared_border_edges`, control row, no gap). Test `the_kit_field_measures_the_shared_fixture_label_cell_without_a_gap` (fixture `labelCellHeight`).
- F2 kit Section/Group/Separator: zero gap between children, Separator one hairline tall. Test `kit_sections_groups_and_separators_add_no_gap_between_bordered_children`.
- F3 stack default: new `stack_gap_for_token` and `ui_node_is_bordered` in `WGPU/🧮️layout/🦀️.rs`; an unspecified gap is 0 when any child is bordered (retained admission in `📌️mounted_layout/🧱️job-state-and-admission`, kit `Stack`), text-only stacks keep the theme gap. Shell `build_stack` and `root_stack` use `gap: Some("none")`.
- F4 Section: React Section (`🎯️targets/⚛️react/🖲️section/🟦️.tsx`) is `mb-8`, title `mb-4`, children in a plain block (no gap). Native Section gap is now 0; title gap 16 and trailing margin 32 already matched. Fixture `🧪️tests/🧫️fixtures/📐️section-field-presentation/🔣️.json` gained `density.sectionChildGap` 0; the mounted-layout section test asserts it with two children.
- F5 CardStack: `🧱️elements/🃏️CardStack/🎯️targets/🧊️wgpu/🦀️.rs` (mounted as `card_stack` in the light tier, crate gained `semio-ui-draw-contract`): `layout_card_stack` (uses `resolve_card_bands`, lifts onto the previous bottom line, grows both rows of a shared band), `card_outline` (port of the silhouette stepped outline), `paint_card_stack` (shared-border segments). Tests in `🧱️elements/🃏️CardStack/stack.test.rs` consume `compositor.cases.json` `interlock`: tops of stacked windows, every vector line covered once, straight and shared, and the painted draw list holds each band line once.
- F6 nativeSeams: `🔨️modules/📏️border-presentation/native-seams.test.rs` (mounted in the light crate, because `semio-ui-draw-contract` is already over its 4000 active-line boundary and refuses to build with any added test) consumes the 4 `nativeSeams` cases from `compositor.cases.json`, mirrors x for `dir: rtl`, derives states from the markup/colour and asserts the one resolved seam. `dir` was already required in `compositor.schema.json`; the resolver segment needs no direction because it is symmetric under the mirror.
- F7 TUI frame rules: shared helper `paint_frame_rule` (`🧱️elements/💬️Dialog/🎯️targets/⌨️tui/🦀️.rs`) writes `├`/`┤`; used by ContextMenu separators and the Command palette query rule. Test `menu_separators_and_the_palette_query_rule_join_their_frame_with_tee_junctions` (`⌨️tui/🧪️tests/🔬️overlays`).
- F8 TUI Divider: `🧱️elements/➖️Divider/🎯️targets/⌨️tui/🦀️.rs` upgrades an adjacent `│`/`┃` wall to `├┤`/`┣┫`. Test `a_divider_meeting_a_wall_joins_it_with_tee_junctions`.
- F9 TUI table: header and row rules span `rect.width` (reach the scroll track column). Test `a_table_rule_reaches_the_scroll_track_when_the_rows_overflow`.

## Results
See the verification section below (cargo via `repo cargo ... test --lib`).

Command: `repo cargo --store=isolated --configuration=prototype -- test --offline --manifest-path <crate>/Cargo.toml --lib` (script `🗑️generated/r3n-run.sh`).

- `semio-ui-wgpu-widgets`: 8/8 (new kit Field and Section/Group/Stack tests).
- `semio-ui-wgpu-layout`: 37/37. `semio-ui-wgpu-mounted-layout`: 24/24 (section pair gap asserted from the fixture).
- `semio-ui-wgpu-light`: 26/26 (CardStack layout/outline/paint over `interlock`, `nativeSeams`, bands, tabs, presence).
- `semio-ui-wgpu-hosting` (shell, scene slots; needs `--features semio-ui-wgpu-mounted-layout/testkit`): 43/44; the failure `accessibility::...real_settings_fields_inherit_labels...` is "UI arena has no accepting lifetime registration", an unrelated arena-registration refactor.
- `semio-ui-tui-laws`: it did not compile before (`Constraint` lost a field, `render-equivalence/🦀️.rs:190`); fixed with `..Default::default()`. Then 97/101: new junction/rule tests pass (another agent added same-purpose overlay tests concurrently; they pass too). The 4 failures (`chrome_ancestor_emphasis...`, `clicking_a_window_focuses...`, `dragging_a_tab...` expecting `┬` got `┌`, pointer routing `WindowFocus`) concern chrome/docking/focus and are not touched by these changes.
- `semio-ui-draw-contract` cannot build any test: its source boundary is exceeded (4247/4000 active lines) already before this work, hence the nativeSeams test lives in the light crate.

Not done: `dir` needed no schema change (already required in `compositor.schema.json`); wiring `paint_card_stack` into a mounted retained node kind is not done (no native `OverviewCard` node exists to host it).
