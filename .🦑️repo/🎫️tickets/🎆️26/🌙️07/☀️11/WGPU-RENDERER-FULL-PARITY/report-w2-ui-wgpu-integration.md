# w2-ui-wgpu-integration — final report

Closed out all five Wave-1 wiring requests in `ui/wgpu/rs/lib.rs`. Build clean (`cargo check -p ui_wgpu --features engine` and default features both pass with zero errors). `cargo test -p ui_wgpu --features engine`: **152 passed, 1 failed** (up from the wave-1 baseline of 140/141) — the one failure is the pre-existing, unrelated `component::ui::ui_node_wire_format_tests::scene_records_serialize_to_golden_json` fixture-drift failure (untouched `component` region, tracked from earlier waves), confirmed identical before and after.

## 1. `tree::WidgetState.open` (`tree` region)
Added `pub open: bool` (narrow additive field, as authorized). Toggled by `events::EventRouter::toggle_select_popup` (new) and cleared by `finish_close` for every dismissal path.

## 2. Select popup open/close wiring (`events`)
- `EventRouter::toggle_select_popup(tree, select_id)`: opens via `open_overlay(tree, select_id, OverlayKind::SelectPopup, OverlayAnchor::Node(select_id))` + sets `state.open = true`; closes via `close_overlay`.
- `finish_close` now also clears `state.open` when `overlay.kind == SelectPopup` — unifies outside-press, Escape, explicit close, and item-pick dismissal through one path.
- `dispatch`'s `PointerUp`/`CaptureKind::Press` arm: clicking a `Select` calls `toggle_select_popup`; clicking one of its synthesized item-row `Button`s (from `reconcile::children_of`) fires its action and closes the popup if it's the topmost open `SelectPopup`.

## 3. Select popup painting (`paint`)
- New pre-pass `sync_interactive_state`/`sync_select_popup_rows`/`select_popup_row_rect`: gives each open Select's synthesized item-row `Button`s real, hit-testable `LayoutBucket` geometry (flex gives them zero-size otherwise), matching `widgets::render_select_menu`'s exact layout.
- `paint_select` rewritten to accept `open: bool` and `retained: Option<(&UiTree, NodeId)>`; paints the popup (glass background, per-row highlight for hovered/selected value) when open.

## 4. Stack `activate`/`selected`/`dropAction` + Tree row hover/drag (`paint` + `events`)
- New `paint_stack_frame`: paints `activate` (bg+border, hover-brightened) and `selected` (accent border + outset ring), ported from `ui-interpreter.tsx`'s `case "stack"`.
- `sync_interactive_state` keeps `NodeFlags::DROP_TARGET` synced with `Stack.drop_action`.
- `events::is_plain_stack_container` (replacing the old blanket "Stack is always pass-through" hit-test rule): a Stack becomes a real hit-test target when it has `activate`/`drop_action`, is `DRAG_SOURCE`-flagged, or — via new `find_tree_item_spec`/`find_item_in_sections`/`find_item_in_items` — its original `UiTreeItemNode` spec has `hover_action`/`unhover_action`.
- `sync_tree_row_layout`/`sync_tree_item_layout`: gives every Tree row real geometry (mirroring `paint_tree_item`'s row-height/indent math) and syncs `NodeFlags::DRAG_SOURCE` from `item.draggable`.
- `update_hover` now returns `Vec<UiCommand>`, firing `hover_action`/`unhover_action` on chain enter/leave.
- `PointerDown` registers a draggable row's `drag_data` via `set_drag_payload`, feeding the existing `maybe_promote_to_drag`/`DragSession` pipeline unchanged.

## 5. `re-exports`/`engine`
Added `pub use engine::Ui;` (previously never exported at all) and flattened w1d's overlay/drag/scroll types into the curated surface: `DismissPolicy, DragGhost, DragPayload, DragSession, ImeEvent, OpenOverlay, OverlayAnchor, OverlayKind, OverlayPlacement, ScrollAxis, resolve_overlay_placement`, plus `EditState` into the `tree` re-export list. No changes needed to `engine::Ui::dispatch_event` itself — it already routes everything through `EventRouter::dispatch`, so all the above flows through automatically.

## Tests added
7 new `events::tests` (Select open/close, outside-press dismissal, item-pick closes popup, Stack activate firing + pass-through-without-activate, tree row hover_action/unhover_action, draggable row → DragSession) and 6 new `paint::tests` (popup paints more + highlights value, popup rows get real layout, Stack drop_action ↔ DROP_TARGET sync, activate/selected extra draw instances, Tree draggable row gets real layout + DRAG_SOURCE). All pass.

## Remaining honest gaps
- `EventRouter::clear_drag_payload`, `set_drop_accept`, `drag_session`, `register_scroll_thumb`, `hovered`, `capture`, `topmost_overlay` remain unused (flagged dead-code) — consumed by `cursor::resolve_semio_cursor_from_tree`/drag-ghost/scrollbar-thumb painting, both out of region ownership (`cursor` is must-not-touch; no scrollable widget painted in this ticket's scope).
- A row/select-popup one-frame lag exists by design: interactive geometry is written during `paint_tree` (after `flex::compute`), so a just-opened popup's rows aren't clickable until the following `frame()` call completes — consistent with this crate's existing "paint recomputes everything, no partial incremental path" philosophy.

## Files touched
- `ui/wgpu/rs/lib.rs` (`tree`, `paint`, `events`, `re-exports` regions)
- `.repo/🎫️/26/07/11/WGPU-RENDERER-FULL-PARITY/region-claims.json` (recorded the narrow additive `events` touches, mirroring w1a/w1d's own exception-recording precedent)
