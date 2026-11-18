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

# Part 2: retained card nodes and root-cause fixes

Paths under `🧰️framework/🔨️modules/🖱️ui/` (`WGPU` = `🎯️targets/🧊️wgpu`). Same command as above.

## (1) Native CardStack / OverviewCard as retained nodes

Wire (schema-first): `ContainerRole` gained `cardStack` and `card` (`🧬️contract/🧩️component/🎛️container-roles-and-row-actions`, the kernel copy `🎠️kernel/📤️return/🔤️json/🖼️ui`, the typed TS wire choice list, the schema projection row, accessibility role name and label). A `card` container carries its title as `label` and its footer chip as `description`, exactly the React `OverviewCard` props.

Declaration and tree: `UiNode::CardStack(UiCardStackNode)` and `UiNode::OverviewCard(UiOverviewCardNode)` (`📜️declaration/🧩️ui-graph`), projected from records by `ui_node_from_record` and by the testkit `apply_tree`/`children_of`.

Layout (`WGPU/📌️mounted_layout`): a stack of cards is a hugging `Section` (not a growing `Stack`) with zero gap; a card is a `Section` whose top band is one control row (the cap row) and whose bottom band is one control row only for a last card with a footer. A footer chip of a non-last card therefore sits in the NEXT card's cap band, exactly the shared band `resolve_card_bands` describes (band = max(footer, cap) = one row). New `ChromeTextRanges::Banded` keeps the line/run bookkeeping consistent for text-less banded nodes (the first attempt faulted `layout.stale` because a `Section` kind counts a line without a run).

Paint (`WGPU/🖌️paint`): `paint_card_stack_node_step` builds `CardStackCard`s from the accepted layouts (title/footer chips measured with the atlas and clamped so cap and footer chips never touch, which is what makes the shared band unconditional), runs `layout_card_stack` + `card_stack_outlines` + `paint_card_stack` (the light-tier module from part 1, now with a `top` origin) as shared-border segments inside one budgeted fixed output, then draws the chip texts phase by phase. `semio-ui-wgpu-paint` now depends on `semio-ui-wgpu-light`.

Test: `a_native_card_stack_lays_out_gapless_and_paints_each_shared_line_once` (`🧪️tests/🔬️targets-wgpu-paint-unit/🦴️skeleton-celebrate-and-chrome-paint-laws`): two cards mounted, no gap, cap row = one control row, upper card's footer lives in the lower cap band, each coincident line painted once, title and footer glyphs drawn. The pure line geometry itself is `stack.test.rs` against `compositor.cases.json` `interlock`.

Not done: no React Interpreter mapping for the two roles was found in the UI module (the React `CardStack`/`OverviewCard` elements exist; the retained-record interpreter that would choose them by role lives outside what I could locate), so the web side renders them only through the typed wire/accessibility changes.

## (2) Root causes of the fundamental failures

- `semio-ui-draw-contract` 4247/4000 active lines: the shared-border resolver (`🔨️modules/📏️border-presentation`, 519 non-test lines, pure and independent of drawing) was `#[path]`-mounted into the crate. It is now its own crate `semio-ui-border-presentation` (`🔨️modules/📏️border-presentation/📦️packages/🦀️rust`), depending only on `semio-framework-job`; draw-contract depends on it and re-exports the same names. The resolver tests and the `nativeSeams` test moved with it. draw-contract now builds its tests (23 pass) and the resolver crate has 6 passing tests.
- `semio-ui-tui-laws` focus/pointer: `Tui::focus_target` took the first focusable node of the whole lineage, and since axes became focusable (shared-border splitters) a click inside a window focused the ancestor axis, so `WindowFocus` was never emitted. It now searches only the nodes inside the window, then the window's own ring (fixes `every_case_of_the_routing_table...`).
- `semio-ui-tui-laws` shared cells: three tests still probed cells that the tiling made shared. `chrome_ancestor_emphasis...` now expects the seam with the window to follow the emphasised side (the seam colour rule), `clicking_a_window_focuses...` clicks the unshared wall, `dragging_a_tab...` compares the ghost and the committed chip cell by cell except for shared-border junction glyphs (the ghost is a standalone closed chip over the pre-drop frame, the commit composes junctions with the neighbours, so those glyphs legitimately differ). An attempt to merge junctions into the ghost was tried and reverted: it cannot reproduce the commit (which removes the old line under the chip).
- hosting `real_settings_fields_inherit_labels...` ("UI arena has no accepting lifetime registration"): the test builds a retained document whose values live in the finite UI arena but never opened the serialized arena scope every other value test opens. It now holds `UiArenaFixture::new()` (dev-dependency `semio-ui-contract-values` with `testkit`).
- `semio-ui-wgpu-paint-laws` `retained_select_popup...` and `retained_input_paints_a_caret...` (the two failures found in round 2; they also blocked the card text): `DrawList::begin_retained_output` returned `Capacity`. `try_reserve_retained_items` reserved shared-border (and layer, glass) storage EXACTLY to `len + items`, and a paged list whose capacity ends mid-page cannot grow afterwards, so any retained step that follows one that pushed borders and needs more slots than the exact reservation failed. Fix in `WGPU/🖍️draw/🏷️types/📋️draw-list-and-retained-output`: reserve to `whole_page_capacity(len + items)` so the list can keep growing page by page (the helper already existed for exactly this). Both tests pass now.
- Removed foreign temporary `[DEBUG]` panics that referenced private cursor fields and stopped `semio-ui-wgpu-paint-laws` compiling.
