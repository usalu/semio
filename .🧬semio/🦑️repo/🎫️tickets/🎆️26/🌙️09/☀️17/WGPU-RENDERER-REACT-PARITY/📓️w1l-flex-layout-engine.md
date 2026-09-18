# W1l — Production layout engine (flex/grid parity with React)

Packet W1l of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. P0 for "elements placed totally
different". Closes findings §5.1–§5.2 and work packets 1, 3 and 4 of
`📓️audit-interpreter-elements.md`.

## 1. What was wrong

The wgpu renderer had a real flex engine (`🎯️targets/🧊️wgpu/📐️flex/🦀️.rs`) in which **every item that
performed layout was `#[cfg(test)]`**. Production ran `📌️mounted_layout` + `🧮️layout` instead: a
hand-rolled pass that summed intrinsic sizes and then handed **100 % of the leftover main-axis space
to every child, unconditionally**. `align`, `justify`, `wrap`, `grow`, per-side padding and all four
non-`Stack` `LayoutSpec` variants were structurally absent — `Grid`/`Scroll`/`Overlay`/`Absolute`
were flattened into a vertical `Stack`, and the 7-value `SpaceToken` ramp was collapsed into three
hardcoded buckets that disagreed between `gap` and `padding`.

## 2. Layout model mapping table

`LayoutSpec` field → React (`layoutSpecStyle`, `🗣️Interpreter/🟦️.tsx:875-934`) → wgpu, after this packet.

| Contract | React | wgpu now | File |
|---|---|---|---|
| `SpaceToken` (7 values) | `spaceTokenRem` = `SPACE_TOKEN_MULTIPLIER[t] × uiSpacingRem` → `0/3.2/6.4/12.8/19.2/25.6/38.4 px` | `SpaceToken::px()` — the SAME multiplier table × `ui_styling`'s generated `UI_SPACING_COMPACT_PX` | `🧬️contract/📐️layout` |
| `EdgeSpace::{All,Symmetric,Each}` | `edgeSpaceToPadding` — 1/2/4-value CSS shorthand | `EdgeSpace::px() -> EdgePx{top,right,bottom,left}`, four independent sides | `🧬️contract/📐️layout` |
| `Sizing::{Hug,Fill,Fixed}` | `sizingToCss` → `auto` / `100%` / rem | `Dim::{Auto,Fill,Length}` | `📐️flex` |
| `Stack.axis` | `flexDirection: row \| column` | `FlowStyle.row` | `📐️flex` |
| `Stack.gap` | `gap` | `FlowStyle.gap_main` (token-resolved) | `📐️flex` |
| `Stack.padding` | `padding` (per side) | `FlowStyle.padding: EdgePx` | `📐️flex` |
| `Stack.align` (5) | `alignItems` | per-child cross offset; `Stretch` fills the cross axis only when the child's own cross sizing is `Hug` | `FlexTree::arrange_one` |
| `Stack.justify` (6) | `justifyContent` | `distribute()` → `(lead, between)` for `Start/Center/End/SpaceBetween/SpaceAround/SpaceEvenly` | `📐️flex` |
| `Stack.grow` | `flex: grow ? "1 1 auto" : undefined` | `FlowStyle.grow` = 1/0, leftover split by grow factor; negative free space shrinks by `shrink × base` | `FlexTree::arrange_one` |
| `Stack.wrap` | `flexWrap` | `FlowKind::Wrapped` → taffy multi-line reflow | `📐️flex` |
| `Grid.columns/rows` | `gridTemplateColumns/Rows` from `GridTrack` | `FlowKind::Grid` → taffy `grid_template_*` (`auto`/`Nfr`/length/`min-content`/`max-content`) | `📐️flex` |
| `Grid.columnGap/rowGap` | `columnGap`/`rowGap` | independent `gap_cross`/`gap_main` | `📐️flex` |
| `Overlay.inset` | `position: absolute; inset: …` | `FlowStyle.absolute` + 4 insets; excluded from flow, stretched between opposite insets | `FlowStyle::absolute_rect` |
| `Scroll.axes` | `overflowX/Y` | `FlowStyle.clips` → `NodeFlags::CLIPS_CHILDREN` + `SCROLLABLE` (existing wheel routing and `hit_test` clip) | `🔀️reconcile` |
| `Scroll.sizing` | `width` | `Dim` on the viewport box | `📐️flex` |
| `Absolute.sizingWidth/Height` | `position: absolute; width/height` | `FlowStyle.absolute` + own sizing | `FlowStyle::absolute_rect` |
| `Leaf.width/height` | `width/height`, `minWidth/minHeight: 0`, **no `display`** (CSS block) | `Dim` + block-equivalent column flow | `📐️flex` |
| intrinsic text | browser shaping + UAX#14 | `measure_text` over the shaped advances: `MaxContent` = whole run, `MinContent` = widest word, `Definite(w)` = greedy first-fit wrap at space/newline | `📌️mounted_layout` |

Every React arm that sets no `display` (`leaf`, `overlay`, `scroll`, `absolute`) is a CSS **block**
container. The engine maps those to a column flow with the default stretch cross-alignment, which is
identical for this vocabulary (nothing here has margins). That also fixes `UiNode::Group`, which
previously relied on the crude pass's implicit vertical stacking.

## 3. What moved to production

- `📐️flex/🦀️.rs` is now entirely production code; the only `#[cfg(test)]` left in the file is the
  trailing `mod tests` declaration, and a law asserts exactly that (§5).
- The layout engine is **this crate's own arithmetic**, not a taffy wrapper. `FlowStyle` is the
  primary style vocabulary (`Dim`, `EdgePx`, `Align`, `Justify` — no taffy type is stored anywhere).
  `FlexTree::measure_one`/`arrange_one` resolve a plain single-line flex container directly:
  flex-basis, grow/shrink, justify distribution, per-child cross alignment, out-of-flow placement.
- **taffy is used for exactly two shapes** — `wrap: true` and `Grid` — where multi-line reflow and
  real track sizing are worth a solver. Those solves are local (one container + its direct children),
  built and dropped inside the method, so no taffy value is ever retained and `FlexTree` stays `Send`
  without an `unsafe impl`. taffy stays the third-party oracle the arithmetic path is checked against
  in the unit suite, which is what AGENTS.md asks for.
- `Node::layout_spec: Option<LayoutSpec>` (`🌳️tree`) carries the AUTHORED layout. `🔀️reconcile`'s
  document mount stamps `record.layout` on it verbatim — the same value React's `layoutSpecStyle`
  reads — and also raises `SCROLLABLE`/`CLIPS_CHILDREN`/`OVERLAY` from it.
- `🧮️layout`'s `gap_for_token`/`padding_for_token` now resolve through `SpaceToken::px()`;
  `🖌️render/📏️layout`'s own `4/8/12/16/24/32` table was **deleted** and delegates to the same
  function. There is one ramp in the repo, in `ui_contract`, derived from the generated
  `ui_styling` token — no second or third copy.
- `🔀️reconcile`'s `space_token`/`edge_token` no longer collapse 7 tokens into 3 buckets nor sample
  one side of an `EdgeSpace`; they stamp the token's own name and the full CSS shorthand, so the
  legacy string channel stays lossless for diffing while the real geometry travels on `layout_spec`.
- `🧪️tests/🔬️targets-wgpu-flex-legacy-layout-job/` (the pre-parity `LayoutJob`) is deleted — no
  legacy layer left behind.

### Two dialects, deliberately

A node whose `LayoutSpec` was authored (a `UiNodeRecord` — every plugin surface) is laid out by plain
CSS rules. A node from the legacy declarative `UiNode` path (in-crate chrome: `shell`, the wgpu
`Interpreter`/`Shell` elements — content with **no React counterpart**) keeps the pre-parity rule
that every child of a `Stack`/`Field` grows into the leftover main axis, because that vocabulary
carries no `grow` information to replace it with. `FlexTree::push` picks the dialect from
`Node::layout_spec`'s presence. Composite wgpu widgets (`Field`/`Section`/`Tree`/`TreeSection`/
`TreeRow`/`Control`) keep their own geometry expressed as flow styles, because `paint` and
`events::hit_test` derive their rows from those same `🧮️layout` constants.

## 4. Budget law

The bounded/stepped model is **kept, and strengthened**. The ladder is now

`CollectNodes → ShapeText → MeasureLayout → SolveLayout → CollectResults → PublishResults`

(`SyncNodes`/`PruneRemoved`/`MeasureFallback`/`ArrangeFallback` are gone.) `LAYOUT_NODE_CREDITS`,
`LAYOUT_GLYPH_CREDITS`, `LAYOUT_DEPTH_CREDITS`, `LAYOUT_ATLAS_PAGE_*` and every refusal site are
unchanged; a sixth fault, `layout.solver`, reports a refused solve instead of publishing a
half-solved surface. The close discipline is unchanged: one owner released per grant, `FlexTree`
popping one node per `release_one`, `terminal_is_empty` covering the new lists.

The law moved from "one NODE per grant" to **"one CONTAINER per grant, charged its direct child
count"**, with total work still linear in the surface's node count. This was measured, not assumed:

| variant | worst worker grant, 1 025-node surface, debug |
|---|---|
| one global taffy solve | 15.1 ms (`engine`'s 8 ms slice law fails) |
| one taffy solve per container | 12.7 ms — a 1 024-child container is still one big solve |
| **arithmetic per container (shipped)** | **1.76 ms** |

`large_layout_and_shaping_job_keeps_every_observed_slice_below_eight_ms` passes. A monolithic solve
also could not be made resumable at all, which is why the flex engine had never been wired up.

## 5. Tests

`🧪️tests/🔬️targets-wgpu-flex-unit` rewritten — 20 fixtures driving the real two-pass ladder:
- `the_flex_engine_carries_no_test_gating` — the law that no `#[cfg(test)]` returns to the engine.
- `every_space_token_resolves_to_reacts_own_pixel_ramp` — all 7 values, all distinct.
- `an_asymmetric_edge_space_keeps_all_four_sides`, `a_stacks_padding_is_applied_per_side_…`.
- `justify_space_between_…`, `justify_center_and_end_…`, `align_items_positions_the_cross_axis_…`,
  `wrap_reflows_an_overflowing_row_…`, `only_a_grow_marked_child_absorbs_leftover_space`.
- `a_two_column_grid_renders_two_columns_not_two_rows`,
  `an_overlay_child_is_out_of_flow_and_never_offsets_its_siblings`,
  `an_absolute_child_is_out_of_flow_with_its_own_size`, `a_scroll_container_spans_its_parent_…`.
- legacy dialect: equal-thirds stack, field label band, section header + intrinsic children.
- close discipline: one owner per grant; a solve against an unadmitted node is refused.

`🧪️tests/🔬️targets-wgpu-mounted-layout-unit` extended with 6 end-to-end fixtures that mount arena
nodes carrying an authored `LayoutSpec` and assert the rects the job actually **publishes**
(`tree.mounted_layout`): space-between, 2-column grid, out-of-flow overlay, wrap, cross-axis align,
and text wrapping inside a narrow flex item. The pre-existing credit/close/worker laws in that file
are untouched and still pass.

Both test dirs are wired by `#[cfg(test)] #[path = "…"] mod tests;` from their source region.
`📌️mounted_layout::layout_tree_now` (testkit-gated) drives the REAL job ladder to completion in one
call, so the paint/scene-slot/shell suites lay a tree out through the shipped stages instead of the
retired `LayoutEngine::compute`.

### Verification (all foreground, logs in `🗑️generated/w1l-*.txt`)

| command | result |
|---|---|
| `cargo check -p semio-framework-ui --features wgpu-engine --lib --keep-going` | 0 errors |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib wgpu::flex` | 20 passed |
| `cargo test … --lib wgpu::mounted_layout` | 16 passed |
| `cargo test … --lib wgpu::layout` | 5 passed |
| `cargo test … --lib -- --test-threads=1` (whole crate) | 504 passed, 6 failed |
| `cargo test -p semio-framework-ui-render --lib` | 129 passed |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going` | clean |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --keep-going` | clean |

The 6 crate-wide failures are **not from this packet** and were verified as such: a golden-JSON
mismatch for scene records whose struct gained `gridVisible`/`selectable*` fields elsewhere, and five
`ArenaFull`/"fixed process permits" refusals in `prepared`/`document_tree_reconcile`/
`retained_document_hostile` — a process-wide `UiResidentPermit` pool whose failing test NAMES ROTATE
between runs. None touch layout; `ui_contract` gained only methods and a standalone `EdgePx`, no
field on any resident type.

`🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` `elementSizeBytes` for `wgpu::engine::UiSurfaceRegistry`
updated `159936 → 160592` (the job now carries its flow styles).

## 6. Remaining gaps

1. **`align-content` for wrapped containers** is taffy's (stretch); React's default is the same, but
   the contract has no field for it either way.
2. **`Align::Baseline`** is treated as `Start` in the arithmetic path — no baseline metric is
   plumbed through the shaping stage yet. Taffy resolves it correctly for the wrap/grid path only.
3. **`OverlayLayout.anchor`** (9-point placement) is still unread — as it is in React, which reads
   only `inset`. Dead contract surface on both sides; `contract-layout`'s owner should confirm.
4. **A wrapped or grid container with very many children** is still one taffy solve in one grant.
   Bounded by its child count, and such containers are small in practice, but a 1 000-cell grid would
   re-introduce a long grant.
5. **`ui_render`'s own `LayoutSpec → taffy Style` mapping** is a second mapping of the same contract
   (different render stack). Only the space-token ramp was de-duplicated; unifying the rest needs a
   shared crate, out of scope here. Note `ui_render` maps `Overlay` to `position: relative` +
   padding, which does NOT match React's `position: absolute` + inset — a real bug in that crate,
   left to its owner.
6. **Density** (`StyleSpec.density`) is still unread by the wgpu theme; React's own layout path is
   likewise pinned to the compact ramp, so this is a shared gap, not a divergence.
7. `large_layout_and_shaping_job_…_below_eight_ms` passes single-threaded and in isolation but can
   flake when the whole 510-test binary runs in parallel on a loaded machine (load average ~18 while
   this packet was verified); the worst measured worker grant is 1.76 ms, so the spikes are OS
   scheduling, not layout work.
