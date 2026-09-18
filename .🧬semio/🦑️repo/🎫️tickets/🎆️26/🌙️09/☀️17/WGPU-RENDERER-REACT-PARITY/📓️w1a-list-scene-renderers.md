# 🧾️ W1a — list-like component scenes restored to production

Packet: restore production rendering + interaction for `Table`, `DiffView`, `EventFeed`,
`GraphTimeline`, `BlockList`, `VirtualFileSystem`.

Paths below are relative to the repo root. The two files that carry the work are

- `P` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (production)
- `S` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs` (the `#[cfg(test)] include!` quarantine)

Line numbers are from the state at the end of this packet; `P` is also being edited concurrently by
W1b (Canvas2d/Paint2d/Ink/TextEditor/IconRender), so they drift.

---

## 1. The defect this packet closes

`render_component_scene_step` was the only production paint entry for a component scene, and it only
knew two content kinds: `World3d` (direct retained draw) and the vello-composited
`engine_canvas::sync_engine_scene` set (NodeGraph/TiledMap/Board2d). Everything else painted the
rounded panel of phase 3 and finished. The six list kinds' renderers had been moved, on 2026-09-08,
into `S`, which the production file splices in behind `#[cfg(test)]` — so in a real build they did
not exist at all.

A second, separate defect made a naive restore useless: **a list surface cannot dispatch through the
retained hit registry.** `retained_hit_registration` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:754`)
mints exactly ONE entry per `ComponentScene` leaf (`retained_scene_hit`, same file:682 — `HitKind::Generic`,
`control_id = surface_id`, `action: None`), and the shell stages that entry *after* the scene paint has
staged its own rows (`Shell/🎯️targets/🧊️wgpu/🦀️.rs:3348` / `:13437` call `register_retained_body_hits`
once the document paint completes, and `HitRegistry::resolve` scans `.rev()`, so the later entry wins).
Any row hit a renderer registers is therefore shadowed by the surface's own catch-all. The press that
lands on the surface instead arrives as `UiCommand::Scene` → `interpreter::process_scene_interaction`
→ `scenes::passive_scene_*`. So **row resolution has to happen in the input path, against re-derived
row geometry** — which is what this packet implements.

## 2. What moved where

Cut from `S` (27 items, `S` went 2 808 → ~1 270 lines and now holds only test helpers plus the
still-test-only `handle_scene_wheel`/`handle_scene_pointer_move`/`handle_scene_pointer_button`,
`canvas_world_pointer_json`, `queue_surface_action`, the paint2d-navigator helpers, and W1b's
ink/tiled-map/puzzle/text-editor helpers), and re-landed in `P` as real (non-`cfg`) items:

| item | now in `P` region |
|---|---|
| `scene_action`, `now_ms` (wasm + native) | `SceneRuntime` |
| `merge_action_args`, `render_table_cell`, `render_table` | `Table` |
| `render_block_list` (rewritten, see §4) | `BlockList` |
| `diff_lines`, `render_diff_view` | `DiffView` |
| `event_feed_time_of_day_utc`, `event_feed_tone_color`, `event_feed_row_height`, `render_event_feed` | `EventFeed` |
| `history_lane_count`, `history_graph_width`, `history_lane_x`, `history_row_lane_guides`, `graph_timeline_avatar_initials`, `render_graph_timeline` | `GraphTimeline` |
| `vfs_glyph_icon`, `vfs_double_click_action`, `hit_double_click_target`, `double_click_action` | `VirtualFileSystem` |

`render_placeholder` and `draw_ink_rect_outline` were also cut from `S`, but W1b had already landed
them in production, so they were **not** re-added — production has exactly one of each.

`render_vfs` was not in `S` at all: it had been deleted outright. It is restored from
`git show 860e015bf6:"<P>"` (pre-refactor line 8549) together with `VfsVisibleRow`,
`vfs_children_by_parent`, `build_vfs_visible_rows`, `vfs_descriptor_label`, `vfs_descriptor_value`.

Every one of the six kinds' `#[cfg(test)]` wire structs (`TableColumn`, `TableSortJson`,
`TableCellPayload`, `TableCellButtonPayload`, `BlockList*Json`, `DiffLine*`, `DIFF_LCS_CELL_BUDGET`,
`EventFeedEntryJson`, `HistoryColumn*Json`, `HISTORY_*`, `Vfs*`) is now unconditional — 20 `#[cfg(test)]`
attributes removed. `SceneSurfaceState::last_click_ms`/`last_click_target` likewise (double-click is
production behaviour now).

**Not touched:** `engine_canvas::sync_engine_scene`, and W1b's `render_canvas_2d_step` /
`render_ink_canvas_step` / `render_icon_render_step` arms.

## 3. Paint dispatch

`P` `render_component_scene_step` now routes the six kinds from W1b's own top-of-function
kind switch, one added arm:

```rust
kind if scene_kind_is_list(kind) => return render_list_scene_step(scene, bounds, ctx, cursor),
```

`render_list_scene_step` (in the `RenderEntry` region, next to `ENGINE_PHASE`) pre-admits the per-row
draw backing and then paints the whole surface in one step, the same shape the `World3d` and
`Canvas2d` branches already have (attach/paint/composite are one step for a chrome-only surface).

**Bounded-cursor contract.** The reservation is `reserve_list_rows(ctx, list_visible_row_capacity(bounds, row_h))`
— `ceil(bounds.h / row_h) + 2` rows × `LIST_ROW_ITEM_BUDGET` (64), capped at `LIST_SURFACE_ITEM_CEILING`
(16 384). Rows can never exceed a single reservation because **every one of the six renderers culls to
the visible band before it draws** (each `for` loop skips rows outside `body`), so the cost is bounded
by the surface's height, not by its payload: a ten-thousand-row table reserves exactly one screenful.
Each renderer re-reserves right after its `push_scissor`, because `push_scissor` opens a new
`DrawLayer` and `try_reserve_retained_items` only reserves on the last one. There is therefore no
row paging across steps — by construction there is nothing to page. `scroll_offset("body"/"diff"/
"feed"/"history"/"vfs"/"blockList")` is read by every renderer and is what the wheel path writes.

`remember_scene_theme(ctx.theme)` was added to phase 2 (runs for every kind): the pointer path only
ever gets a `&UiComponentSceneNode` + a rect, never the frame's `Theme`, yet all row geometry is
theme-derived, so a hit resolved against `Theme::default()` would land on the wrong row.

## 4. Pointer / wheel / double-click wiring

`passive_scene_wheel` gained a `BlockList` arm (`"blockList"` scroll key); the other five were
already there.

`passive_scene_pointer_button` changed signature to
`(scene, bounds, x, y, down, button, input: &mut InputState<ActionDescriptor>) -> Result<bool, BoundedActionFault>`
and both `Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` call sites (~1125/1128) were updated to `…, input)?`.
On a primary-button **release** inside a list surface it: (a) tries `scene_double_click_action`, then
(b) resolves `scene_list_hit(…, activate = true)`, then (c) either toggles VFS expansion or publishes
the action through the bounded queue (`write_scene_action` → `reserve_actions(1, bytes)` →
`builder.value(None, args)`, byte credits from a new `dsl_value_bytes`). Paint2d's pan arming is
unchanged.

`passive_scene_pointer_move` now records `SceneSurfaceState::hovered_control_id` for list kinds
(`scene_list_hit(…, activate = false)` — no action built, so a move costs no allocation and does not
mutate the VFS selection anchor). The renderers read it back via `scene_hovered_control_id`, because
`InputState::hovered_id` can only ever name the whole surface (see §1).

Per-kind resolvers, all sharing the paint's own geometry:

- `table_hit` / `table_metrics` / `table_next_sort_direction` / `table_cell_hit` / `table_row_id` / `table_row_action`
- `event_feed_hit` (walks the same variable row heights via `event_feed_row_height`)
- `graph_timeline_hit`
- `vfs_hit` / `vfs_metrics` (+ `hit_double_click_target` / `double_click_action`, now theme-aware and
  indexing the **visible** row list — they previously used hardcoded `22.0`/`24.0` metrics and the raw
  row array, so they resolved a different row than the paint drew)
- `block_list_hit` — shares `block_list_plan`, see below.

`BlockList` has non-uniform row heights, so instead of a second geometry derivation it got a real
layout pass: `block_list_plan` returns `Vec<BlockListTarget> { rect, control_id, action, paint }` plus
the header/body/palette bands and the body index range. `render_block_list` walks the plan forwards
and draws; `block_list_hit` walks it backwards so a card's own buttons answer before the card band.
One geometry, two consumers — and a test pins that (see §6).

`DiffView` resolves no pointer target (React's host has no click verb) — it only scrolls.

## 5. Action payload table (kind → action id → args), against React

| kind | trigger | action id | args | React source |
|---|---|---|---|---|
| Table | header press (sortable) | `sortTable` | `{surfaceId, columnId, direction}` | `📊️Table/🟦️.tsx` `onSort` |
| Table | row press, no domain | `selectRow` | `{surfaceId, row}` (whole row object) | `onRowClick` else-branch |
| Table | row press, `domainId` + `domainGranularityId` | `interactionSelect` | `{domainId, targets: "[{\"granularity\":…,\"id\":…}]", merge:"replace", method:"pick"}` | `onRowClick` then-branch |
| Table | stepper − / + | the cell's own action | its args **merged** with `{delta: ∓step}` | `dispatchCellAction` |
| Table | stepper centre | — (swallowed) | — | React `stopPropagation` |
| Table | row button (`placement` absent or `"row"`) | the button's own action | unchanged | `renderTableCell` |
| EventFeed | row press | `scene.activateAction` | `{surfaceId, id}` | `📡️EventFeedHost/🟦️.tsx:117-126` |
| GraphTimeline | row press | `checkoutCheckpoint` | `{checkpointId}` (no `surfaceId`) | `🌳️GraphTimelineHost/🟦️.tsx` |
| BlockList | Add Step | `addStep` | `{}` | `🧩️BlockListHost/🟦️.tsx` |
| BlockList | step ▲/▼ | `moveStep` | `{stepId, index}` | `handleStepDragEnd` |
| BlockList | step 🗑 | `removeStep` | `{stepId}` | `StepCard` |
| BlockList | block ▲/▼ | `moveBlock` | `{blockId, fromStepId, toStepId, index}` | `handleBlockDragEnd` |
| BlockList | block 🗑 | `removeBlock` | `{stepId, blockId}` | `BlockCard` |
| BlockList | palette entry | `addBlock` | `{kind}` | `PaletteEntryRow` |
| VirtualFileSystem | row press | `selectRows` | `{surfaceId, ids}` | `VirtualFileSystemHost` `onSelectionChange` |
| VirtualFileSystem | chevron press | — (local expand/collapse) | — | ui-react `VirtualFileSystem` |
| VirtualFileSystem | double-click `os://instance/<id>` | `openInstance` | `{surfaceId, instanceId}` | `navigateUri` |
| VirtualFileSystem | double-click `os://export/<id>/…/<fmt>` | `exportMedia` | `{surfaceId, instanceId, format}` | `navigateUri` |
| VirtualFileSystem | double-click `/spaces/<id>` or `studio:<id>` | `navigateVirtualFileSystemNode` | `{surfaceId, spaceId}` | `navigateUri` |
| DiffView | — | — | — | no click verb in React |

### Drifts fixed while porting

1. **EventFeed** sent `{entryId}` — a key no host reads. Now `{surfaceId, id}`, matching
   `📡️EventFeedHost/🟦️.tsx:117-126`. Pinned by a test that also asserts `entryId` is gone.
2. **DiffView** row wash → text tint. The quarantined code had already been corrected; verified and
   kept (adds/removes tint `theme.accent`/`theme.error`, equal stays full `theme.text`, no
   `push_solid` row background) — `🔺️DiffViewHost/🟦️.tsx:93-97`.
3. **Table row ids** fell back to `""` when a row had neither `id` nor `pluginId`, collapsing every
   such row onto one id. Now falls back to the row's ordinal, like `TableHost`'s `rowIds` map.
4. **Table `interactionSelect`** branch did not exist at all — a domain-declaring table dispatched the
   plugin-private `selectRow` instead of picking into the framework interaction domain.
5. **Table `placement: "menu"` buttons** were drawn in the row and took row-width segments; React
   draws only `"row"` placement inline. `TableCellButtonPayload` gained the `placement` field.
6. **DiffView split mode** listed a changed line as two rows (left-only, then right-only). Ported
   `buildSplitRows` as `split_diff_rows`, so a remove/add run pairs onto aligned rows.
7. **VFS double-click geometry** used hardcoded `row_h = 22.0` / `body_y = inner.y + 24.0` and indexed
   the raw row array; the paint uses theme metrics and the expansion-flattened visible list. Both now
   go through `vfs_metrics` + `build_vfs_visible_rows`.
8. **Hover** never worked for any list kind (`InputState::hovered_id` can only name the surface). Now
   tracked per surface in `SceneSurfaceState::hovered_control_id` from the move path.

## 6. Tests

Existing modules kept compiling and were extended (all in `🎞️Scenes/🧪️tests/…`):

- `🔬️wgpu-table` — added `TablePointerTests`: row press → `selectRow` with the full row; row press →
  `interactionSelect` with the exact `targets` JSON string / `merge` / `method`; header press → next
  sort direction; stepper −/+ merged `delta` (and the centre swallowing the row click); row buttons
  vs `placement: "menu"`; ordinal row-id fallback.
- `🔬️wgpu-event-feed` — `{surfaceId, id}` payload (and that `entryId` is absent); a feed with no
  `activateAction` resolves hover only.
- `🔬️wgpu-graph-timeline` — row band → `checkoutCheckpoint {checkpointId}`, no `surfaceId`.
- `🔬️wgpu-block-list` — **drift law**: for every control the paint registered, pressing the centre of
  its rect through `block_list_hit` resolves the same `control_id` and the same verb; plus empty-body
  press dispatches nothing.
- `🔬️wgpu-virtual-file-system` — row press → `selectRows {surfaceId, ids}`; chevron press toggles and
  dispatches nothing; second press on an `os://instance/…` row → `openInstance`.

Also repaired a **pre-existing red guard test**: `production_action_ingress_has_no_legacy_queue_and_text_vec_helpers_are_test_only`
in `S` asserted `SCENES_SOURCE.contains("#[cfg(test)]\npub fn handle_scene_wheel")` etc. — but those
functions moved into `S` itself on 2026-09-08, so the production source has not contained those
strings since. Added a `STANDALONE_SOURCE` const (`include_str!("🦀️.rs")`) and repointed the three
stale loops (`handle_scene_*`, `ink_*`, `tiled_map_*`/`puzzle_board_*`) at it.

### Verification

See §7.

## 7. Verification

Log: `🗑️generated/w1a-native-check-final.txt` (a copy of `w1a-native-check.txt`).

### Native — `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going`

**This packet's code compiles clean. The crate does NOT, because of W1b's concurrent in-flight work.**

The crate reached a real compile of `🎞️Scenes` (`Checking semio-framework-os-renderer-wgpu`, typeck ran —
`E0609` field errors and `unused_variables` liveness lints were both emitted, so this is not an
aborted expansion). Two rounds:

- Round A (first complete renderer compile): **18 errors** — 2 mine, 16 W1b's. Mine were both
  `E0433: cannot find type IconName`, at `render_table_cell` and `paint_block_list_target`: the
  `use semio_framework::IconName;` import had been narrowed to `#[cfg(test)]` while these two call
  sites were quarantined. Fixed by un-`cfg`ing that import. Also 2 `unused_variables` warnings for
  `passive_scene_pointer_move`'s `delta_x`/`delta_y`, which W1b's removal of the Paint2d pan branch
  left dead — the two parameters and the interpreter's `x - last_x` computation were dropped.
- Round B (after those fixes): **16 errors, zero of them in this packet's code or regions.** All are
  `SurfaceKind::Canvas2d` / `InkCanvas` — W1b's packet:
  - `E0425 cannot find function canvas_lum` ×4, `canvas_set_lum` ×4, `canvas_sat` ×2, `canvas_set_sat` ×2
    (lines 3280-3283 — those five helpers were removed from production during W1b's move and their
    caller `canvas_blend_channel` was not)
  - `E0609 no field grid_visible / grid_spacing / grid_subdivisions / grid_opacity on InkDocumentJson`
    (lines 5689-5690)

  Three warnings touch `🎞️Scenes` and none is from the moved/added code.

### wasm32-unknown-unknown and `cargo test --lib`

**Not run.** Both need the crate to compile, and it cannot until W1b's 16 errors land. Re-run both
once that packet is green:

```
cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --keep-going
cargo test -p semio-framework-os-renderer-wgpu --lib scenes::table_tests scenes::block_list_tests \
  scenes::diff_view_tests scenes::event_feed_tests scenes::graph_timeline_tests \
  scenes::virtual_file_system_tests scenes::render_entry_tests
```

Nothing in this packet is `cfg(wasm32)`-gated, so the wasm check should add no new surface; the only
`cfg(target_arch)` split added is `now_ms`/`SCENE_INPUT_THEME`, which follows the file's existing
`thread_local!` vs `WorkerCell` pattern verbatim.

All six touched test files, the standalone include and the Interpreter parse clean under
`rustfmt --edition 2021 --check` (a full-file parse; only unresolvable `#[path] mod` siblings are
reported, which is expected when formatting a non-root file in isolation).

### Build-environment note

The shared `-Zfine-grain-locking` build dir deadlocked twice during this packet (18 cargos idle at
0 % CPU, no `rustc` anywhere, all in `cargo::core::compiler::prebuild_lock_exclusive` → `flock`;
oldest idle 21 min). Broke it by `kill -9` on the whole stuck set, per
`feedback-fine-grain-locking-wasm-deadlock`. Four of this packet's seven check runs were themselves
`SIGKILL`ed (`CARGO_EXIT=137`) by peers doing the same sweep, which is why the runs are wrapped in a
retry loop that re-runs on 137 — **a bare `cargo check` that prints no errors here may simply have
been killed; only a log containing `Checking semio-framework-os-renderer-wgpu` is evidence.**

## 8. Remaining gaps

- **Context menus.** Every React host opens a per-row surface context menu
  (`openSurfaceContextMenu`, `menu.id` = `table`/`eventFeed`/`graphTimeline`/`blockList`/`virtualFileSystem`,
  with `hits: [{domain:"row"|"entry", id}]` for Table/EventFeed/VFS). The wgpu side has no
  equivalent plumbed from a scene surface — `ui_wgpu`'s own `OverlayKind`/`open_overlay` is
  `pub(crate)`. Nothing in this packet dispatches a right-click.
- **Drag and drop.** `render_table`/`render_vfs` still attach `drag_data` to hit targets that are
  shadowed, so a row drag source is inert; `TableScene::drop_action_json` (React's `onDrop`) and the
  block-list palette drag (`PALETTE_DRAG_MIME`) have no route. BlockList reordering is buttons, not
  drag — deliberate, documented on `block_list_plan`.
- **Modifiers.** `UiEvent::PointerDown/Up/Scroll` carry no modifier fields, so shift-extend /
  ctrl-additive VFS selection always resolves as a plain replace (`vfs_selection_for_click(…, false, false)`).
  This is the pre-existing `apply_scene_ui_command` gap, not new.
- **NodeGraph double-click.** `hit_double_click_target`/`double_click_action` used to carry a
  NodeGraph arm (`openInstance` on a node). NodeGraph is `scene_has_bespoke_pointer_dispatch`, so it
  can never reach the passive path; the arm — and `find_graph_node`/`hit_graph_node` with it — was
  dropped rather than left dead. If NodeGraph node double-click is wanted it belongs in
  `engine_canvas`'s own NodeGraph input path.
- **DiffView line numbers.** React's gutter shows `beforeNo`/`afterNo` in `text-muted-foreground`.
  Not ported: the existing paint test `unchanged_lines_stay_full_brightness_not_dimmed` asserts
  `theme.text_muted` does **not** appear among the glyph colours, so adding the gutter needs that test
  reworked first.
- **VFS file-type glyphs.** React's ~40-entry extension→icon table is still unported (pre-existing).
- **`scene_action`, `render_placeholder`, `draw_ink_rect_outline`** now live in production and are
  shared with W1b's kinds; if W1b lands its own copy the duplicate needs collapsing.
  (`render_placeholder`/`draw_ink_rect_outline` were already in production when this packet landed —
  cut from the include but deliberately not re-added, so there is exactly one of each.)
- **Blocking for W1b, not this packet:** the 16 errors in §7 round B. `canvas_lum`, `canvas_set_lum`,
  `canvas_sat`, `canvas_set_sat` (and `canvas_clip_color`) were production functions in the
  `Canvas2d` region before this wave and are now gone while `canvas_blend_channel` still calls them;
  `InkDocumentJson` lost its four `grid_*` fields while `render_ink_canvas`'s grid branch still reads
  them. Left untouched on purpose — they are inside W1b's own regions.
