# 🪟️ W1i — Window system parity (Dock / Shell window regions) vs React

Packet W1i of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Covers audit packets 4 and 12 of
`📓️audit-shell-window-system.md` plus the rest of that report's §4 window-system rows.

Surfaces touched:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (four surgical sites)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs` (new `WindowSystemReactParityTests` region)

React reference throughout: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx` (2 136 lines —
`Mode`, `ModeDockStack`, `ModeDockTabBar`, the layout-tree helpers) and
`🧰️framework/🔨️modules/🖱️ui/🔨️modules/🎛️chrome-control-presentation/🟦️.ts`.

---

## 1. Behaviour table — React → wgpu

Status: **fixed** = changed by this packet; **OK** = already matched, re-verified; **gap** = still
divergent, see §4.

### 1a. Tab bar

| Behaviour | React | wgpu before | wgpu now | Status |
|---|---|---|---|---|
| Action set on a tab | `[Focus/Unfocus?] [Close] [DragHandle]` — `🎨️Canvas/🟦️.tsx:1072-1101` | `[focus][new][close]` unconditionally, `🛰️Dock/…/🦀️.rs:1298` | `dock_tab_actions(show_maximize, maximized)` → `[focus?][close]`, `🛰️Dock/…/🦀️.rs:964` | **fixed** |
| Dead "new window" chip | no counterpart at all | icon `app-window`, registered as `dock.tab.<path>.<id>.new`, **zero** dispatch arms | removed; the two `.new` `strip_suffix` calls in `🐚️Shell/…/🦀️.rs` (drag promote / drag settle) are gone too | **fixed** |
| Focus/Unfocus gating | `showMaximize = !mobile && canMaximize`, `canMaximize = modeCollectWindowIds(layout).length > 1` (`:1158`, `:1800`) | painted always | `DockState::can_maximize()` / `show_maximize()` / `tab_action_count()` (`🛰️Dock/…/🦀️.rs:170-206`) | **fixed** |
| Focus icon | `Minimize2` when maximized, else `Maximize2` (`:1084`) | same | same, now inside `dock_tab_actions` | OK |
| Close | always shown (`:1088-1100`) | always shown | always shown | OK |
| Mobile | `mobile` prop from `UI_MOBILE_MEDIA_QUERY = "(max-width: 767px)"` (`⚛️react/🟦️.tsx:1648`) | no notion of mobile anywhere in the wgpu renderer | `DockState::mobile`, set once per frame in `plan_dock_windows` from `screen_w` vs `MODE_DOCK_MOBILE_MAX_WIDTH_PX` | **fixed** (flag only — see gap G1) |
| Label truncation | whole tab capped at `max-w-[12rem]`, label `truncate`s (`🎛️chrome-control-presentation/🟦️.ts:35`) | none — tab width was `measure_text(label)` + 3 chips, so tabs grew without limit | `MODE_DOCK_TAB_MAX_WIDTH_PX = 192.0` + `truncate_label_to_width` (ellipsis, char-boundary safe), applied in `dock_tab_chip` | **fixed** |
| Action chips right-aligned | flex row, label `min-w-0 flex-1`, actions trailing (`:1049`) | actions started right after the measured label, so a capped tab would have overflowed | `content_x = tab.rect.x + tab.rect.w - action_w * actions.len()` | **fixed** |
| Active / hover tint | `modeDockActiveTabClass` / `modeDockInactiveTabClass` / `modeDockActiveTabFillClass`, `data-stack-active` vs `data-active` (`:1036-1046`) | `active_foreground` when active **and** stack globally active, `border_emphasized` on hover, else `text_element` | unchanged | OK |
| Per-window options chip z-order | corner groups are one glass chip per corner, painted after the body fill | same | unchanged | OK |
| Tab select vs action hit split | separate `<button>` for the label, separate buttons per action | `select_w = tab.w - action_w*3` | `select_w = tab.w - action_w * actions.len()` | **fixed** (follows the new count) |
| Corner grouping (4 corners per stack) | `modeStackTabsByCorner` (`:320-333`) | `tabs_by_corner` | unchanged | OK |
| Empty-corner drop pad | `mode-dock-corner-drop-pad`, `min-h-medium min-w-medium` (`:1114`); `--size-medium = 7 × --ui-spacing` (`🖌️ui/🎨️.css:738`) | magic `8.0 * 3.0 = 24 px` | `theme.control_height` (= `chrome_px(7.0)`, the same token) | **fixed** |

### 1b. Close semantics

| Behaviour | React | wgpu before | wgpu now | Status |
|---|---|---|---|---|
| Closing the last tab of a stack | allowed; `collapseLayout(removeWindowFromLayout(prev, id))` (`:1475-1485`) | **silent no-op** — `if windows.len() <= 1 { return false }` (`🛰️Dock/…/🦀️.rs:211`) | `close_window_in_stack` → `close_window` → `remove_window_from_layout` (`🛰️Dock/…/🦀️.rs:246-289`) | **fixed** |
| Stack re-focus after close | `activeId = children[0]?.id` (`:313`) | `windows[idx-1]` (the tab to the LEFT) | `children[0]` | **fixed** |
| Canvas re-focus after close | `onActiveWindowChange(remaining[0] ?? null)` (`:1481`) | only touched when the closed tab was the stack's active | `collect_window_ids().next()`, `None` when nothing is left | **fixed** |
| Collapse rule | `collapseLayout`: drop empty stacks, **hoist** a single-child axis keeping `only.size ?? node.size` (`:225-236`) | `collapse_empty` pruned empties but never hoisted | `collapse_layout_node` mirrors React exactly (incl. the `only.size` rule) | **fixed** for the close lane; see gap G2 for the drag lane |
| Closing the only window | root becomes `{kind:"stack",children:[]}`, active `null` | unreachable (refused) | empty root stack, `active_window_id`/`active_stack`/`maximized_stack` all cleared | **fixed** |
| Maximized path after a close | effect drops it once `!canMaximize` (`:1802-1805`) | cleared only if the exact path vanished | re-anchored by window-id key, then dropped when `!can_maximize()` | **fixed** |
| Host teardown on close | `onWindowClose` also destroys a spawned plugin app and drops the extra window instance (`🏛️ShellHost/🟦️.tsx:10533-10555`) | `note_control_command` only | unchanged | **gap G5** |

### 1c. Layout tree — splits, ratios, resize, drag, maximize

| Behaviour | React | wgpu | Status |
|---|---|---|---|
| Row/column splits, stack leaves | `WindowLayoutNode` recursive tree | `DockNode::{Row,Column,Stack}` | OK (shared contract) |
| Split resize clamp | `applyAxisResizeDelta`, `minPct = 8` per side of the dragged pair (`:593-611`) | was: independent `clamp(0.08, 0.92)` per side + a `normalize_pair_sizes` that was **arithmetically a no-op** (`a*pair_sum == children[i].1`), so a clamped drag changed the PAIR total and silently restretched every other child of the axis | `new_left = clamp(origin_left + Δ, 0.08, pair_sum - 0.08)`, `new_right = pair_sum - new_left`; `SPLIT_MIN_FRACTION`, dead `normalize_pair_sizes` deleted | **fixed** |
| Sub-epsilon drag ignored | `if (Math.abs(deltaPct) < 0.001) return layout` (`:594`) | none | `SPLIT_DELTA_EPSILON` on the fraction scale | **fixed** |
| Resize gutter geometry | thin separator + fat grab zone | `SPLIT_VIS_PX = 6.0`, `SPLIT_HIT_MIN_PX = 20.0`, handle centred on the seam (`🛰️Dock/…/🦀️.rs:1129-1130`) | OK, now pinned by a test |
| Join-corner squares | `modeJoinCornerSpecsForSeparator` / `MODE_JOIN_CORNER_TOUCH_EPS` (`:514-583`) | `register_join_corner_hits`, 10 px squares | OK |
| Drop-zone resolution order | corner tab bars → stack body → mode root (`computeModeDropZone`, `:797`) | `compute_dock_drop_zone` — same order | OK |
| Split side from pointer | `resolveModeSplitSideInBody` dominant-axis-from-centre (`:778-796`) | `resolve_split_side` — exact port | OK |
| Tab insert index | measured against real DOM tab rects | assumed a **4 px gap between tabs that is never painted** (`layout_stack_cap` advances by the chip width alone) → the split point drifted by `4 × index` | `compute_tab_insert_index(…, 0.0)` | **fixed** |
| Drag-to-split commit | `splitWithWindow`/`splitRootWithWindow` build a bare two-child axis (`:403-421`) | `axis_pair_from_stacks` → `0.5 / 0.5` | OK, now pinned |
| Drag-to-merge commit | `insertWindowAsTabAtCorner` + `flatIndexForCornerInsert` (`:335-370`) | `insert_tabs_at_corner_with_kinds` + `flat_index_for_corner_insert` | OK, now pinned |
| Whole-stack drag | `extractStackFromLayout` + `targetAnchorId` re-anchoring (`:424-437`, `:816-834`) | `extract_stack_group` + `target_anchor` | OK (pre-existing tests) |
| Maximize / restore | `toggleMaximize(stackPath)`, maximized stack fills the Mode with the **same** chrome (`:1898-1906`) | `toggle_maximize` + `stack_frame_rects`/`stack_body_rects` special case → full canvas, `render_stack` with `maximized = true` | OK; the `!canMaximize` reset is **fixed** |
| Minimize | **does not exist** on either side (`onMinimize` is a `🪟️Window` prop the dock never passes) | absent | OK (N/A) |
| Free drag / per-window resize corners / z-order raise | **do not exist** — this is a tiling manager | absent | OK (N/A) |

### 1d. Focus / active window

| Behaviour | React | wgpu | Status |
|---|---|---|---|
| Click a tab → focus | `onSelectTab` → `activateWindow` → `setActiveWindowInLayout` + `onActiveWindowChange` (`:1195`, `:1446-1452`) | `dock.set_stack_active(path, id)` + `active_window_id` (`🐚️Shell/…/🦀️.rs:8496`) | OK |
| Click a window **body** → focus | `<Window onActivate={() => dock.activateWindow(activeId)}>` (`:1217`) | set `active_window_id` only — `dock.active_stack` stayed on the previously focused stack, so the body press focused a window whose stack never lit up | added `self.dock.sync_active_window(&owner)` in `route_retained_pointer_press` | **fixed** |
| Focus visual | SVG silhouette outline + tab z-classes (`⚛️react/🟦️.tsx:7787-7850`) | flat `theme.accent` border + `active_foreground` tint keyed on `state.active_stack` | cosmetic gap (audit P2), unchanged |
| Focus on drop | drag commit focuses the dropped window | `apply_drop` sets `active_window_id`/`active_stack` | OK |
| Keyboard cycling | `mod+shift+enter` etc. | owned by packet W1c — **not touched here** | — |
| Z-order | none (tiling) | none | OK (N/A) |
| Click canvas background → deactivate | `handleCanvasBackgroundPointerDown` → `onActiveWindowChange(null)` (`:1462-1472`) | no equivalent | **gap G3** |

### 1e. Window measures overlay

| Behaviour | React | wgpu | Status |
|---|---|---|---|
| Default width | `windowMeasuresDefaultWidthPx = domSizePx("layoutPanelRailUiSpacing")` (`⚛️react/🟦️.tsx:8128`) | `theme.window_measures_default_width = chrome_px(dom::LAYOUT_PANEL_RAIL_UI_SPACING)` | OK — same token |
| Resize clamp | `windowMeasuresMinWidthPx` / `…MaxWidthPx` = `layoutPanelMin/MaxUiSpacing` (`:8131-8134`) | `.clamp(theme.panel_min_width, theme.panel_max_width)`, same two tokens (`🐚️Shell/…/🦀️.rs:8000`) | OK |
| Placement | right-aligned rail inside the window body | `window_measures_rect` right-aligned, inset by `panel_inset` | OK |
| Fold state | rail is **folded by default** (`🪟️Window/🟦️.tsx:169` `useState(true)`), unfolded by a chrome chip | `measures_folded` defaults to **false**; the `shell.measures.fold.*` / `…unfold.*` dispatch arms exist but **nothing paints the toggle chip** | **gap G4** — deliberately left unfolded: flipping the default without a chip would make the rail unreachable |

### 1f. Persisted layout

Neither renderer persists the window layout. React holds it in the `shellLayoutReducer`
(`🐚️Shell/🟦️.tsx:919-920`, `SET_SHELL_LAYOUT`) — reducer state only, no storage key; there is no
`shellLayout` entry in any prefs store. wgpu holds `ShellState::layout_override`, restamped by
`persist_dock_layout` (`🐚️Shell/…/🦀️.rs:4415`) and re-applied through `apply_layout_diff`. **Parity is
already exact (both session-scoped), so nothing was changed** — switching renderers does not carry
windows across on either side, because neither one carries them across a reload. Verified by grepping
both `//#region 🔑️StorageKeys` (wgpu) and every `useLocalStorage`/prefs path (React) for a layout key:
none exists.

---

## 2. Tests

All new, in `🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs`, region `//#region WindowSystemReactParityTests`
(reached from the dock module's `#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"] mod tests`):

| Test | Pins |
|---|---|
| `dock_tab_actions_match_react_mode_dock_tab_bar` | the exact action set and icons; no `new` action |
| `show_maximize_follows_window_count_and_mobile` | `canMaximize > 1` and the `!mobile` gate |
| `dock_tab_hits_register_only_reacts_two_actions` | registered `control_id`s: `.close` always, `.focus` only when `>1`, never `.new` |
| `dock_tab_label_truncates_at_reacts_twelve_rem_cap` | chip width pinned to 192 px, ellipsis appended, short labels untouched |
| `closing_the_last_tab_of_a_stack_collapses_the_layout` | close is never a no-op; the sibling is hoisted to the root |
| `closing_the_active_tab_focuses_the_first_remaining_window` | `children[0]`, not `idx-1` |
| `closing_the_only_window_empties_the_dock` | empty root stack, `active_window_id == None` |
| `collapse_hoists_a_single_child_axis_keeping_the_childs_ratio` | React's `only.size ?? node.size` |
| `maximize_is_inert_and_self_clearing_for_a_single_window_canvas` | the `canMaximize` guard and the reset effect |
| `split_resize_conserves_the_pair_total_and_floors_at_eight_percent` | pair-sum conservation + the 8 % floor + untouched siblings |
| `split_resize_gutter_hit_is_twenty_pixels_centred_on_the_seam` | 20 px grab zone, `dock.split..0`, centred |
| `drag_to_split_commits_an_even_two_child_axis` | exact 50/50 on stack-split and root-split |
| `drag_to_merge_joins_the_target_corner_group_and_focuses_the_tab` | corner-local index → flat insert, focus moves |
| `corner_tab_bar_widths_match_the_painted_chip_widths` | drop-target widths == painted chip widths (capped) |
| `tab_insert_index_follows_the_painted_gapless_chip_run` | the removed phantom 4 px gap |

Pre-existing dock tests kept passing unchanged in intent: `close_window_collapses_stack_tabs`,
`dock_stack_glass_and_hits_exist_only_on_owned_chips` (still asserts `.focus` — its fixture has two
windows, so `canMaximize` holds), `maximized_stack_uses_full_canvas_bounds`, the whole
`DragDropAndLayoutDiffTests` region and `🪟️app-mode-layouts`.

---

## 3. Verification — what was actually run

Logs under `🗑️generated/w1i-*.txt`. Stated plainly, including what was **not** verified.

### ✅ `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going` — GREEN

`🗑️generated/w1i-native-check.txt`, 2026-09-18 01:49, `Finished dev profile … in 2m 07s`, **0 errors**,
58 warnings (all pre-existing except the one noted below, which was then fixed).

Getting there took ~1 h of retries; the log files `w1i-check-1…7.txt` record the reason and are worth
keeping for whoever integrates:

- `w1i-check-1` — 121 errors + a rustc ICE (`incremental_verify_ich_failed`). Root cause: a sibling
  packet had `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` mid-edit with an **unclosed delimiter**. Every later
  run used `CARGO_INCREMENTAL=0` to keep the ICE away.
- `w1i-check-2…7` — 3–62 errors per run, **0 of them in `🛰️Dock`** and none on a line this packet
  touched; all in `🎞️Scenes`, `⚙️EngineCanvas`, `🗣️Interpreter` and the `🐚️Shell` panel/native-directory
  regions belonging to sibling packets (W1a/W1b/W1h), plus `semio-framework-ui` itself.
- Several runs died `exit=137`. Not this packet: the volume is **99 % full** (8–18 GiB free of 926),
  the shared build dir is 295 GB, and another ticket's `🛟️disk-guard.sh` prunes every 4 minutes —
  with no swap headroom the kernel kills rustc.
- At 01:33 the whole fleet deadlocked: **five** concurrent `cargo check -p …renderer-wgpu` from
  different agents, **zero** rustc alive, every cargo parked in
  `compiler::prebuild_lock_exclusive → LockManager::lock → flock` (sampled, `/tmp/s23105.txt`).
  Clearing the stuck set (the documented remedy) let the very next check finish in 2 m 07 s.

### ⚠️ `cargo test -p semio-framework-os-renderer-wgpu --lib dock` — 70 passed, 3 failed

`🗑️generated/w1i-test-dock.txt`, 02:00.

**14 of this packet's 15 new tests pass.** Every pre-existing dock/window test passes, including
`close_window_collapses_stack_tabs`, `maximized_stack_uses_full_canvas_bounds`,
`dock_tab_content_width_reserves_icon_slot`, `dock_stack_glass_and_hits_exist_only_on_owned_chips`
and the whole `DragDropAndLayoutDiffTests` region.

The three failures, honestly attributed:

1. `resize_hits_win_over_later_scroll_region` — **pre-existing, not this packet.** Untouched by W1i.
   A sibling refactor split the hit registry into staged/resolved phases: `register_hit` writes
   `hits.staging`, while `hit_at` resolves `hits.resolved`, "the last COMPLETE frame's registry"
   (`🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:331-347`). Any test that registers hits and immediately calls
   `hit_at` now reads an empty registry. Needs a one-line fix in whichever packet owns that refactor.
2. `split_resize_gutter_hit_is_twenty_pixels_centred_on_the_seam` — this packet's test, failing for
   the same staged/resolved reason. **Rewritten after that run** to read `staged_hits()` like its
   passing sibling `dock_stack_glass_and_hits_exist_only_on_owned_chips`. **That rewrite was NOT
   re-run** — the coordinator asked for the packet to close without further cargo runs. Both files
   were re-checked with `rustc -Zparse-crate-root-only` and parse clean, but the assertion itself is
   unverified.
3. `dock_stack_content_fills_full_bounds_through_one_silhouette_clip` — expects 3 clip scissors, got
   2. **Not attributable to W1i by inspection, but not proven:** `git show HEAD` of `layout_stack_cap`
   builds spans exactly as the current code does (one span per non-empty corner, `continue` on
   empty), so a stack whose two tabs both sit in `TopLeft` yields **one** top span and therefore
   `body + 1` clip rects both before and after this packet — the tab-width change cannot move that
   count. The likely owner is whoever changed the silhouette clip construction or the default tab
   corner. Not re-verified against a clean tree (no `git stash`/`checkout` is allowed here).

### ❌ `cargo check --target wasm32-unknown-unknown` — NOT RUN

Stopped before this step on the coordinator's instruction. Nothing in this packet is
target-conditional: no `cfg` attributes were added or touched, and every new item is plain
arithmetic, `String`/`Vec` work and existing `ui_wgpu` calls already used on both targets. Still
unverified.

### Warning hygiene

The green native check surfaced one new dead-code warning caused by this packet —
`dock_tab_content_width` became unused once `dock_tab_chip_width` stopped calling it. Fixed by
routing `dock_tab_chip`'s width through it (`dock_tab_content_width = dock_tab_chrome_width(theme, 0)
+ measure`), which is arithmetically identical. **That fix landed after the green check and was not
re-compiled.** The other dead-code warning in the file, `dock_tabs_from_ids`, is pre-existing
(`git show HEAD` has the same single occurrence) and was left alone.

---

## 4. Remaining gaps (not in this packet)

- **G1 — mobile is a flag, not a layout.** React's `mobile` also collapses every window into ONE flat
  tab stack (`mobileFlatStack`, `🎨️Canvas/🟦️.tsx:1888-1901`) and disables tab drag
  (`startTabDrag: mobile ? noopDrag : startTabDrag`, `:1852`). wgpu now honours `mobile` for the
  Focus chip only; below 768 logical px it still paints the full split tree. Also note the flag is
  read off `screen_w`, which is **physical** pixels until packet W1g lands its logical-unit fix — on
  a Retina host `screen_w` is ~2× the CSS width, so the breakpoint currently under-triggers rather
  than mis-triggers.
- **G2 — the drag lane still uses the weaker collapse.** `DockState::remove_window` (the eager
  removal at drag promotion, `🐚️Shell/…/🦀️.rs:7431`) keeps `collapse_empty`, which prunes empty
  stacks but does not hoist a single-child axis; React's docked-out preview
  (`modeDockOutLayout` → `removeWindowFromLayout`) does hoist. Changing it would move every
  drop-zone path mid-drag and invalidates four regression tests written for real drop bugs
  (`apply_drop_tab_split_targets_the_post_removal_stack`, `apply_drop_tab_reinserts_…`,
  `apply_drop_stack_split_reanchors_…`, `apply_drop_stack_same_source_is_noop`). Wants its own
  packet: port React's model properly, i.e. commit `apply_drop` against the **committed** tree and
  keep the docked-out tree as a render-only derivation.
- **G3 — no click-to-deactivate.** React clears the active window when the canvas background (or a
  resizable gutter) is pressed; wgpu has no such path. Needs a background hit target in the Shell.
- **G4 — the measures rail has no fold chip.** `shell.measures.fold.*` / `shell.measures.unfold.*`
  dispatch arms exist but no chrome registers them, so the rail cannot be folded and its default
  must stay unfolded (React defaults to folded). Belongs to whoever owns window chrome chips.
- **G5 — close does not tear down a spawned app.** React's `onWindowClose` destroys the plugin
  instance and drops the extra window instance; wgpu only notes the shell command.
- **G6 — no drag handle chip.** React puts a `DragHandle` grip after Close and starts tab drags only
  from it; wgpu starts a drag from anywhere on the tab (25 px² threshold). Functionally close, but
  the grip is a visible chip React paints and wgpu does not.
- **G7 — focus visual.** React draws an SVG silhouette outline for the active window; wgpu draws a
  flat accent border. Audit P2, cosmetic.
- **G8 — `push_window_silhouette_border` is commented out** at the end of `render_stack`, so the
  stack border computed from `globally_active`/hover is never painted at all. Left alone: re-enabling
  it is a visual change that belongs with G7.
