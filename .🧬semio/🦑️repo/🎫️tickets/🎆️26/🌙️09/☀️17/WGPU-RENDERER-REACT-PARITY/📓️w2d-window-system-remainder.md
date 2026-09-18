# 🪟️ W2d — Window-system remainder (Dock drag lane, mobile, deactivate, fold chip, teardown)

Packet W2d of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Closes gaps **G1–G8** of
`📓️w1i-window-dock-semantics.md` §4 plus the React window-chrome affordances that were still open.

Surfaces touched:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
  (`ShellInput`, `📏️WindowMeasures`, the dock plan/chrome phases, `close_active_window`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs` (one fixture)

React reference: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx`,
`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx`,
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`,
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`.

---

## 1. The one structural change: a render-only dock derivation

Both G1 (mobile) and G2 (drag lane) are the same shape in React — the mode **renders** a tree it
derives from the committed layout and never writes back. That is now one method:

`DockState::render_view(&self, drag: Option<&DockDragPayload>) -> DockState`
(`🛰️Dock/…/🦀️.rs:416-441`), the twin of React's `dockOutLayout` memo + `mobileFlatStack`
(`🎨️Canvas/🟦️.tsx:1869`, `:1884-1891`).

The Shell keeps it in a new per-frame field `ShellState::dock_view`, written once by
`plan_dock_windows` (`🐚️Shell/…/🦀️.rs` `plan_dock_windows`, the line right after `dock.mobile`), and
**every** consumer of the rendered tree reads it: the window plan, `dock_drop_bodies`,
`dock_drop_tab_bars`, `window_content_rects`, `window_silhouettes`, `paint_chrome` and
`register_resize_hits`. `self.dock` stays the committed tree, mutated only by a committed edit
(close, drop, split, maximize, resize).

Consequence: a zone path resolved by the pointer against the rendered tree is *by construction* a
valid path into the tree `apply_drop` re-derives — which is what let the whole re-anchoring and
snapshot machinery go.

---

## 2. Per item

### G1 — mobile is a layout, not a flag

| Behaviour | React | wgpu now |
|---|---|---|
| Below the breakpoint every window becomes one flat tab stack | `mobileFlatStack` (`🎨️Canvas/🟦️.tsx:1884-1891`) | `mobile_flat_stack` (`🛰️Dock/…/🦀️.rs:936-950`), applied by `render_view` |
| Tab order / active tab | `modeCollectWindowIds(dockOutLayout)`, `activeId ?? [0]` | `collect_flat_tabs` in layout order, active window kept, else first |
| No maximize on mobile | the mobile branch precedes `maximizedStack` entirely | `render_view` clears `maximized_stack` and pins `active_stack` to the root path |
| Tab drag disabled | `startTabDrag: mobile ? noopDrag : startTabDrag` (`:1852`) | promotion gated on `!self.dock.mobile` in `handle_pointer_move` |
| Focus/Unfocus chip hidden | `showMaximize = !mobile && canMaximize` | unchanged (W1i) |

Because the flat stack paints every tab at the **root** path, focus and close had to stop being
path-addressed: `DockState::activate_window(window_id)` (`🛰️Dock/…/🦀️.rs:443-452`) is React's
`activateWindow(windowId)` (`🎨️Canvas/🟦️.tsx:1446-1452`), and the tab's close chip now routes through
`DockState::close_window` (already id-addressed since W1i).

Note the breakpoint still reads `screen_w`; whether that is logical or physical is **W1g's** business,
unchanged here.

### G2 — drag lane on the committed tree

- `DockState::apply_drop` (`🛰️Dock/…/🦀️.rs:454-521`) rewritten as React's `applyModeDrop`
  (`🎨️Canvas/🟦️.tsx:816-838`): it re-derives the removal itself
  (`remove_window_from_layout` for a tab, `extract_stack_from_layout` for a stack), then inserts /
  splits into that derived tree. No `target_anchor`, no `resolveStackPathForWindowId`.
- `ShellState::handle_pointer_move` no longer calls `dock.remove_window` at promotion; a drag is a
  pure state transition.
- `dock_drag_snapshot`, `restore_dock_drag_snapshot` and the three `restore` call sites are **gone**:
  an abandoned or refused drop cannot corrupt a layout that was never edited (no `sync_dock`
  round-trip either).
- `DockState::remove_window` and the whole `collapse_empty` / `is_empty_stack` / `is_empty_node` /
  `remove_window_from_node` family are **deleted**. There is now exactly ONE collapse rule in the
  element, `collapse_layout_node` (React's `collapseLayout`, prune + hoist keeping `only.size`), used
  by the close lane, the drop lane and the render derivation alike.
- New pure free functions, each named against its React twin: `empty_stack`, `stack_of`,
  `insert_tabs_at_corner_in_node`, `split_node_at_path`, `split_root_node`,
  `extract_stack_from_layout`, `dock_out_layout`, `mobile_flat_stack`, `collect_flat_tabs`
  (`🛰️Dock/…/🦀️.rs:825-951`). `insert_tabs_at_corner_with_kinds`, `split_stack_with_stack_and_kinds`
  and `split_root_with_stack_and_kinds` are now thin focus-bookkeeping wrappers over them, so the
  "open in new window" lane and the drop lane cannot drift apart.

The four drop regression tests were **rewritten to the new model, not deleted** — see §3. Three of
them changed their zone path because the new collapse rule hoists where the old one only pruned;
that difference is now asserted explicitly against `render_view`, which is the whole point of the
packet.

### G3 — click-to-deactivate

- `DockState::deactivate_active_window` (`🛰️Dock/…/🦀️.rs:454-464`) and
  `ShellState::deactivate_active_window` — React's `deactivateActiveWindow`
  (`🎨️Canvas/🟦️.tsx:1455-1459`).
- Two press paths in `handle_pointer_button`, both mirroring
  `handleCanvasBackgroundPointerDown` (`:1461-1471`):
  - a press inside `dock_canvas_bounds` that hits **nothing** (React's `event.target ===
    event.currentTarget` on `[data-slot="mode-body"]`);
  - a press on a `DockSplit` / `DockJoinCorner` gutter (React's `data-slot="resizable-panel"`
    branch) — it deactivates **and still starts the resize**, exactly like React.
- The layout is untouched either way.

### G4 — Window Options fold chip

- `ShellState::paint_measures_fold_chip` (`🐚️Shell/…/🦀️.rs`, `//#region 📏️WindowMeasures`) paints
  React's `Pane` toggle: `anchor="top-right"`, `WINDOW_PANE_MEASURES_ICON` = `settings-2`
  (`⚛️react/🟦️.tsx:8201`), one glass chip of `theme.control_height`, and registers exactly the ids
  the shell already dispatched — `shell.measures.unfold.<windowId>` when folded,
  `shell.measures.fold.<windowId>` when open (`🪟️Window/🟦️.tsx:326-330`).
- Placement: the window body's top-right corner when folded, the rail's own when open, so the rail
  is reachable in both states.
- `ShellState::measures_rail_folded` now defaults to **`true`** — React's
  `useState(true)` (`🪟️Window/🟦️.tsx:169`). W1i had to keep it `false` precisely because nothing
  painted a toggle.
- A window with no measures document paints no chip at all (React: `measures ? <Pane/> : null`).

### G5 — close tears down the spawned app

- One close lane: `ShellState::close_dock_window(window_id)`, used by the tab chip
  (`dock.tab.*.close`) and by `mod+shift+w` (`close_active_window`). It collapses, refocuses
  `children[0]`, tears down, and persists.
- `ShellState::teardown_spawned_window` + the pure `ShellState::take_spawned_entry` mirror React's
  `onWindowClose` (`🏛️ShellHost/🟦️.tsx:10533-10555`): the closed window's `SpawnedAppEntry` leaves
  the panel state, `active_spawned_id` re-points at the first survivor, the panel json is rewritten
  onto the session view state, and the owning plugin bridge gets `destroy_app(instance_id)`.
- A plain (non-spawned) window has nothing to tear down and is unaffected.

### G6 — drag-handle chip: implemented

- `dock_tab_actions` now ends with `("drag", "grip-vertical")`, React's `DragHandle`
  (`🎨️Canvas/🟦️.tsx:1101`, `🧱️DragHandle/🟦️.tsx:63` — `GripVerticalIcon`), so the painted chip run is
  `[focus?] [close] [drag]` exactly like React's.
- `tab_action_count()` is now derived from `dock_tab_actions` itself, so a measured tab width can
  never disagree with the painted run again.
- The grip is the **only** drag origin: `dock.tab.<path>.<id>.drag` arms the pending drag; the label
  target `dock.tab.<path>.<id>` sets `pending_dock_tab_select` and selects **on release** over the
  same label, which is React's `<button onClick={() => onSelectTab(tab.id)}>` (`:1049-1071`). New
  helper `dock_tab_select_window_id` splits the two id shapes.
- The dead `.strip_suffix(".focus")/.strip_suffix(".close")` unwinding in the old release path is
  gone (those ids are claimed by `handle_shell_hit` on press and never reached it).

### G7 + G8 — silhouette focus border: implemented, nothing left commented out

- `render_stack` now ends with
  `push_window_silhouette_border(draw, &silhouette, theme.stroke_hairline, if globally_active { theme.selected } else { theme.border_normal })`.
- `theme.selected` is `chrome.active_base`, i.e. React's `var(--active-base)` for the `active`
  silhouette kind; `theme.border_normal` is its `normal` kind
  (`windowSilhouetteBorderPaint`, `⚛️react/🟦️.tsx:7494-7509`).
- It is painted outside the body-fill branch, so it appears whether or not the dock is asked to fill
  bodies, and after the tab groups so it is not covered. The function had exactly one production
  call site and it was commented out; there is no commented-out code left in the element.

### Remaining React window-chrome affordances — checked, nothing to port

| Affordance | Finding |
|---|---|
| Double-click on a tab | React's `ModeDockTabBar` renders no `onDoubleClick` anywhere; `onDoubleClick` is a `🪟️Window` prop the dock never passes. Nothing to port. |
| Middle-click close | no `onAuxClick` and no `button === 1` handling in `🎨️Canvas/🟦️.tsx`. Nothing to port. |
| Window options chip content / z-order | that chip IS the measures fold chip (G4); it is painted after the window body and before the dock cap, so it sits above content and below the tab chrome, like React's `z-panel` pane toggle. |
| Tab title / labels | already per-instance from the app manifest through terminology + locale (`dock_chrome_maps`, W1i). Unchanged. |
| Maximize/restore animation-free parity | React's maximized stack is the same chrome at full Mode bounds with no transition; wgpu's `stack_frame_rects`/`stack_body_rects` special case is identical and has no animation. Unchanged. |
| Minimize, free drag, per-window resize corners, z-order raise | do not exist on either side (tiling manager). N/A. |

---

## 3. Tests

All in `🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs`, region `//#region WindowSystemReactParityTests`.

### New (W2d)

| Test | Pins |
|---|---|
| `mobile_collapses_every_window_into_one_flat_tab_stack` | four windows in a nested row/column become ONE stack in layout order; maximize cleared; `active_stack` is the root; the **committed** tree is never flattened; one body rect instead of four |
| `activate_window_focuses_by_id_across_the_committed_tree` | id-addressed focus resolves to the window's real stack, not the path its chip was painted at; unknown id refuses |
| `deactivate_clears_focus_without_touching_the_layout` | focus cleared, tree byte-identical, second call is a no-op |
| `the_drag_lane_hoists_a_single_child_axis_like_the_close_lane` | the docked-out derivation hoists keeping the child's own `0.75`, and equals what `close_window` produces — one collapse rule |
| `an_abandoned_tab_drag_leaves_the_committed_tree_untouched` | the derivation is not an edit; a zone that resolves to nothing refuses and still edits nothing |
| `the_active_stack_paints_a_silhouette_focus_border` | hairline strokes are actually emitted for the stack silhouette (G8's commented-out call) |
| `window_options_fold_chip_is_registered_on_both_sides_of_the_fold` | default folded; `shell.measures.unfold.main` when folded, `shell.measures.fold.main` when open; one glass chip |
| `closing_a_window_collapses_persists_and_refocuses_in_one_lane` | prune + hoist, `children[0]` refocus, `layout_override` persisted, second close refuses |
| `closing_a_spawned_window_takes_its_panel_entry_and_repoints_the_active_one` | the entry (and its `instance_id`, the thing the guest must destroy) leaves the panel, `activeSpawnedId` re-points, a plain window hosts nothing |

### Rewritten to the committed-tree model (the four G2 regressions + two W1i drag tests)

| Test | What changed |
|---|---|
| `apply_drop_tab_moves_window_across_stacks` | no pre-removal; now asserts the shift happens in `render_view`'s derivation while the committed tree still holds the window |
| `apply_drop_tab_reinserts_into_originating_stack_at_new_index` | zone path `[0]` → `[]`, because the new collapse rule hoists the one-child axis (asserted against `render_view`) |
| `apply_drop_tab_split_targets_the_post_removal_stack` | same hoist; now asserts the exact resulting axis instead of "both ids still resolvable" |
| `apply_drop_stack_split_reanchors_target_after_extraction_shifts_paths` → `apply_drop_stack_split_lands_on_the_derived_path_without_reanchoring` | renamed and re-pointed: the derived path IS the drop path, so there is nothing to re-anchor; asserts the whole source stack travels as one node |
| `apply_drop_stack_moves_whole_group_preserving_order_and_target_key` | the stack node travels whole (`extractStackFromLayout`) instead of being restitched around `tab_index` |
| `apply_drop_tab_moves_window_to_target_corner`, `apply_drop_tab_root_split_builds_axis_pair`, `apply_drop_stack_same_source_is_noop`, `drag_to_split_commits_an_even_two_child_axis`, `drag_to_merge_joins_the_target_corner_group_and_focuses_the_tab`, `concrete_window_instances_round_trip_without_kind_collapse` | pre-removal line dropped; assertions unchanged |
| `dock_tab_actions_match_react_mode_dock_tab_bar`, `dock_tab_hits_register_only_reacts_two_actions` | extended for the grip chip (`.drag` must be a registered target on both single- and multi-window canvases) |

Plus one Shell fixture: `finish_dock_drag_persists_layout_and_clears_drag_state_on_successful_drop`
(`🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs`) drops its pre-removal and moves its zone to the hoisted
root path.

---

## 4. Verification — what was actually run

Logs under `🗑️generated/w2d-*.txt`. All foreground, one cargo at a time, `-j 4`,
`CARGO_INCREMENTAL=0` (an incremental ICE, `incremental_verify_ich_failed`, showed up on run 2).

### ✅ `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` — GREEN

`🗑️generated/w2d-check-native-final.txt`: `Finished dev profile … in 1m 20s`, **0 errors**, 54
warnings, **none of them in `🛰️Dock` or in any line this packet touched** (they are pre-existing
`unnecessary qualification` / unused-import warnings in peer regions).

Runs 1–6 (`w2d-check-1…6.txt`) failed with 25 → 23 → 6 → 3 → 2 errors, **zero of them in this
packet's surfaces**: peers were mid-refactor on the Shell's settings-UI `UiNode` struct imports
(`UiSectionNode`/`UiFieldNode`/`UiButtonNode`/`UiToggleNode`/`UiSliderNode`/`UiInputNode`), the
Shell's `stageCommandArg` `DslValue`/`serde_json::Value` boundary, `settings_section`'s `&str`
argument, and `🗣️Interpreter`'s `queue_canvas_image_upload_sized`. Polling until their tree settled
was the whole cost.

### ✅ `cargo test -p semio-framework-os-renderer-wgpu --lib dock -j 4` — 83 passed, 0 failed

`🗑️generated/w2d-test-dock-8.txt`: `test result: ok. 83 passed; 0 failed; 699 filtered out`.

All 9 new W2d tests and all 7 rewritten ones are in that list by name. W1i closed at **70 passed /
3 failed**; the three it left open all pass now:

- `resize_hits_win_over_later_scroll_region` — a peer fixed the staged/resolved hit-registry split
  it was tripping on (the test now calls `publish_hits()`).
- `split_resize_gutter_hit_is_twenty_pixels_centred_on_the_seam` — W1i's unverified `staged_hits`
  rewrite is confirmed correct.
- `dock_stack_content_fills_full_bounds_through_one_silhouette_clip` — passes at 2 clip scissors;
  the silhouette border this packet re-enabled does **not** change that count (it pushes solids
  after `end_silhouette_clip`).

Runs 1–7 (`w2d-test-dock-1…7.txt`) failed to **build the test binary** on peer test files only
(`🔬️wgpu-panel-anchor-model`'s `DockTabSkeleton { trees }`, `🔬️wgpu-text-editor`'s removed
`InputState::staged_actions`). Not touched — a peer repaired them mid-poll.

### ✅ Adjacent suites — GREEN

- `shell_input_tests` — 19 passed, 0 failed (`w2d-test-shell_input_tests.txt`)
- `window_measures_tests` — 3 passed, 0 failed (`w2d-test-window_measures_tests.txt`)
- `panel_anchor_model` — 39 passed, 0 failed (`w2d-test-panel_anchor_model.txt`)

### ✅ `cargo check --lib --target wasm32-unknown-unknown -j 4` — GREEN

`🗑️generated/w2d-check-wasm.txt`: `Finished dev profile … in 53.37s`, **0 errors**, 58 warnings.
Nothing in this packet is target-conditional (no `cfg` added or touched).

### ❌ The whole `--lib` suite — could not be run to completion

`🗑️generated/w2d-test-lib.txt`: the binary **aborts** (SIGABRT, "panic in a destructor during
cleanup") inside `async_boundary_tests::renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length`,
which kills every other test in the process. Unrelated to this packet (no shared code path with the
dock or the window/input regions) and reproducible before the dock tests even start; it needs its
own owner. The four filtered runs above are what could be proven.

---

## 5. Gaps left open (not this packet)

- **The breakpoint's unit.** `dock.mobile` is still read off `screen_w`, which is physical until
  **W1g** lands logical units; on a Retina host the 767 px breakpoint under-triggers. The flat-stack
  mechanism itself is unit-agnostic.
- **Silhouette stroke style.** React's active outline is an SVG path with per-kind CSS classes
  (`celebrated`/`introduced`/`loading`/`waiting`); wgpu paints only the `active`/`normal` pair. The
  other four kinds are introduction/loading feedback owned by the tour and status packets.
- **`clearPendingWorldProjection` on close.** React's `onWindowClose` also drops a pending world
  projection for the closed window; the wgpu shell has no such pending-projection map to clear, so
  there was nothing to mirror — re-check if one is introduced.
- **Spawned-app teardown is unproven at runtime.** `take_spawned_entry` is unit-tested, but
  `teardown_spawned_window`'s `destroy_app` call needs a live session (the "impractically large
  90+-field" `ActiveSession` fixture) and was verified by inspection against the three other
  `destroy_app` call sites in the file, not by execution.
