# 🧭 W1h — 8-anchor floating panel dock + production command dock (wgpu)

Packet W1h of ticket 26/09/17 WGPU-RENDERER-REACT-PARITY. Closes audit §3 (“the rails placed totally
different”) and audit §1's *Command palette (persistent panel)* row — recommended work packets **2**
and **3**. All paths absolute under `/Users/ueli/Documents/semio`.

Primary file: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
(referred to below as **the shell file**).

---

## 1. What the shell had, and what it has now

**Before.** `PanelAnchor` existed as a type with all eight React anchor names, but nothing consumed it.
The real state was a hardcoded two-slot model — `left_panel_open` / `right_panel_open` booleans plus
`LeftPanelKind{Workbench,Display}` / `RightPanelKind{Details,Settings,Chat}` — and `group_side()` folded
even the four corners `PanelGroup::anchor()` names back down to `"left"`/`"right"`. There was **no panel
tab bar at all**: the `shell.panel.tab.left.*` / `shell.panel.tab.right.*` hit handlers had no painter
registering those ids. `build_command_panel_ui` was `#[cfg(test)]`, i.e. not in the product.

**After.** A real per-anchor dock:

| Concern | New API (shell file) |
|---|---|
| Anchor identity | `PanelAnchor::{as_str, from_str, index, vertical, horizontal, from_group, ALL}` |
| Tab tree | `DockTabNode` (leaf when `children` is empty, else branch), `ShellDock{anchors:[Vec<DockTabNode>;8]}` with `tabs`/`tabs_mut`/`locate`/`node_at`/`default_path`/`reconcile_path` |
| Per-anchor chrome | `PanelAnchorState{visible,size,path}`, `ShellState::panel_anchors: [PanelAnchorState; 8]` |
| Default arrangement | `ShellState::default_dock()` |
| Persisted arrangement | `DockSkeleton`/`DockTabSkeleton`, `dock_skeleton_of`, `dock_skeletons_equal`, `apply_dock_skeleton`, `load_dock_skeleton_from_store`, `save_dock_skeleton_to_store` |
| Persisted chrome | `DockUiState`/`DockUiPanelState`, `dock_ui_snapshot`, `apply_dock_ui`, `load_dock_ui_from_store`, `save_dock_ui_to_store`, `persist_dock_ui_if_changed` |
| Drag re-anchoring | `DockTabMoveTarget`, `move_tab_in_dock`, `ShellState::{move_dock_tab, finish_dock_tab_drag, anchor_at_point}` |
| Collapse | `dock_collapsed_branches`, `dock_branch_collapsed`, `toggle_dock_branch` |
| Geometry | `anchor_panel_rect` (free fn, the `anchorPositionStyle` twin), `ShellState::anchor_rect` (live, column-banded) |
| Paint / hit-test | `paint_anchor_tab_bar`, `anchor_tab_rows`, `anchor_tab_bar_height`, `anchor_content_rect`, `navbar_tab_row_item`, `footer_tab_row_item` |

`LeftPanelKind`, `RightPanelKind`, `group_side`, `panel_toggle_icon_id`, `floating_panel_rect`,
`has_left_tabs`/`has_right_tabs`, `PanelLayoutPersisted`, `encode/decode_panel_layout_field`,
`PANEL_LAYOUT_STORAGE_KEY` and `FRAMEWORK_SETTINGS_COMMANDS_TAB_ID` are **deleted** (no legacy layer,
per AGENTS.md). `left_panel_open()` / `right_panel_open()` survive only as *column projections* —
methods, not fields — because `⌘️B`/`⌘️⇧️B`, the overlay safe area (`open_floating_panel_rects` →
`chrome_panel_safe_area`) and the tutorial snapshot genuinely ask a column-level question.

---

## 2. Anchor geometry vs React

React: `anchorPositionStyle` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6861-6881`) insets each
panel by `var(--spacing-single)` from the edges its anchor names, centres on a `middle` axis with a
`translate(-50%)`, and clamps both axes to `calc(100% - (var(--spacing-single) * 2))`.
wgpu: `anchor_panel_rect(anchor, size, column_slots, column_index, body, theme)`; the inset is
`theme.panel_inset` = `chrome_px(PANEL_INSET_UI_SPACING = 1.0)` = 3.2 logical px, the same generated
token React's `--spacing-single` resolves from (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🦀️.rs:276`).

| Anchor | React inset (H, V) | wgpu `anchor_panel_rect` | Width | Height |
|---|---|---|---|---|
| `top-left` | `left: S`, `top: S` | `x = body.x + S`, band 0 | `size` clamped `[panel_min, min(panel_max, body.w − 2S)]` | column band |
| `top-middle` | `left:50% translateX(-50%)`, `top: S` | `x = body.x + (body.w − w)/2` | same | column band |
| `top-right` | `right: S`, `top: S` | `x + w = body.x + body.w − S` | same | column band |
| `right-middle` | `right: S`, `top:50% translateY(-50%)` | `x + w = body.x + body.w − S` | same | column band |
| `bottom-right` | `right: S`, `bottom: S` | `x + w = body.x + body.w − S` | same | column band |
| `bottom-middle` | `left:50% translateX(-50%)`, `bottom: S` | centred | same | column band |
| `bottom-left` | `left: S`, `bottom: S` | `x = body.x + S` | same | column band |
| `left-middle` | `left: S`, `top:50% translateY(-50%)` | `x = body.x + S` | same | column band |

`S = theme.panel_inset`. **Column band** = `(body.h − 2S − gap·(n−1)) / n` where `n` is the number of
*open* anchors in that anchor's column (`ANCHOR_COLUMN_ORDER`, top → middle → bottom), the anchor taking
band `column_index`. With one open anchor per column that is exactly the old full-height rect, so nothing
that worked before moved; with two it splits instead of overlapping. See §6 for why this is the one
deliberate divergence.

Widths: `DEFAULT_PANEL_WIDTH_PX = 300.0` (React `🛠️ShellHelpers/🟦️.tsx:221` `DEFAULT_PANEL_WIDTH_PX = 300`),
clamped by `theme.panel_min_width`/`floating_panel_max_width` from `LAYOUT_PANEL_MIN_UI_SPACING`/
`LAYOUT_PANEL_MAX_UI_SPACING`. Resize handles follow React's `resizeSides`/`deltaFactor`
(`🖼️Panel/🟦️.tsx:499`): a corner or edge-middle anchor registers **one** handle on its canvas-facing edge
(`panel.resize.<anchor>.inner`), a middle-column anchor registers **both** and applies the ×2 growth factor
because a centred panel's opposite edge moves too. Handle hit width stays `PANEL_RESIZE_HIT_PX = 20.0`.

---

## 3. Tab sources vs React

`ShellState::default_dock()` mirrors `🏛️ShellHost/🟦️.tsx`'s 🧭️DockAssembly (`:9424-9461`):

| Anchor | React source | wgpu source | Same ids |
|---|---|---|---|
| `top-left` | `workbenchLeftTabs` — app tabs whose `panelAnchorForGroup(group) === "top-left"` | `session.app.panel_tabs` filtered by `PanelAnchor::from_group(tab.group) == TopLeft` | ✅ app-declared ids |
| `top-middle` | `[]` | `[]` | ✅ |
| `top-right` | `detailsRightTabs` + `frameworkChatTab` | Details-group app tabs + `framework.chat` leaf | ✅ `framework.chat` |
| `right-middle` | `[]` | `[]` | ✅ |
| `bottom-right` | `settingsBottomRightDockTab` (framework Settings branch with app Settings tabs nested) + `frameworkMarketplaceTab` + `frameworkUtilitiesHistoryTab` | Settings-group app tabs, which already include the framework-injected `framework.panel.history` (`🔌️plugin/🦀️.rs:5193`) | partial — see gaps |
| `bottom-middle` | `framework.category.tool` branch + `framework.category.command` branch | `framework.category.command` branch, one `command.category.<id>` leaf per resolved palette category | ✅ branch + leaf ids |
| `bottom-left` | `framework.category.display` branch + `displayBottomLeftTabs` + `frameworkSyncTab` (`s-sync-status`) | `framework.category.display` branch over the Display-group app tabs + `s-sync-status` leaf | ✅ |
| `left-middle` | `[]` | `[]` | ✅ |

Labels come from the manifest's own `LocalizedLabel::resolve(terminology, locale)`
(`dock_tab_node_of`) — no shell dictionary, no default language — and icons from the existing
`panel_tab_icon_id`. Framework-owned leaf labels (`chat`, `display`, `command`, `sync`) resolve through
`shell_chrome_string`, with `panelToggle.command` / `panelToggle.tool` / `panelToggle.sync` added to the
EN/DE bundle.

Tab bars are painted in three places, matching React's `PANEL_TAB_BAR_HOSTS`
(`🛠️ShellHelpers/🟦️.tsx:230-237`):

* **navbar** — folded root rows of `top-right`, `top-middle`, `top-left` (`navbar_tab_row_item`), replacing
  the five hardcoded `ui.panelToggle.*` chips;
* **footer** — folded root rows of `bottom-left`, `bottom-middle`, `bottom-right` (`footer_tab_row_item`),
  which the wgpu footer had none of (audit §1's *Footer / utility bar* P1 row);
* **the open panel itself** — `paint_anchor_tab_bar` draws the root row plus one row per branch the active
  path has drilled into and not folded, with active-state fill, branch chevrons and `HitKind::PanelTab`
  hits at `shell.panel.tab.<anchor>.<tabId>`.

The `ui.panelToggle.*` control ids stay as **command** targets (`shell_command_for_control`, the command
registry test, keybindings); they now route to `toggle_anchor`/`toggle_anchor_tab` instead of the deleted
kind enums.

Drag-and-drop: pointer-down on a `PanelTab` arms `dock_tab_drag`; pointer-up resolves the anchor under the
pointer (`anchor_at_point` — an open anchor's painted box first, else the body third the pointer is in) and
runs `move_dock_tab`, which is `move_tab_in_dock` plus override recomputation, `sync_dock_tabs` and a reveal.
Dropping a subtree into itself is refused, exactly like React's `moveTabInDock`.

---

## 4. Persistence — one document, shared with React

Both dock documents now live in the **same `semio.os.config` JSON document** React's `OsShellConfig`
owns (`🧰️framework/🔨️modules/🖥️platform/🟦️.ts:265`), which this shell already read/wrote for `preferences`:

* `dockLayouts.os` / `dockLayouts.apps[<appId>]` ← `DockSkeleton { version: 3, anchors: { <anchor>: [{id, children?}] } }`,
  byte-shape identical to React's `DockSkeleton`, per-app layer winning over the shared `os` one exactly as
  `DockLayoutStore::getSnapshot` does;
* `dockUi.os` / `dockUi.apps[<appId>]` ← `DockUiState { version: 3, anchors: { <anchor>: {visible?, size?, path?} } }`,
  React's `DockUiState`/`DockUiPanelState`, storing only what differs from the computed default.

This **replaces** the old `semio.panelLayout.v1` preference, which carried a tab-separated left/right pair
no React shell could ever read. A dock rearranged in one renderer now opens rearranged in the other.

`apply_dock_skeleton` is a faithful port of React's `applyDockSkeleton`, mention-collection pass included:
mentions are gathered across the whole skeleton *before* any anchor resolves, so an anchor processed first
can never reclaim a tab moved to a later one; ids the default no longer declares are dropped; any default
tab the skeleton never mentions anywhere is appended at its default location.

`os.resetDock` now clears **both** layers plus the in-memory anchor state and collapse map, matching
React's `RESET_DOCK` resetting `dockLayoutStore` *and* `dockUiStateStore`. `persist_dock_ui_if_changed`
keeps the dirty-checked render-loop hook the old `persist_panel_layout_if_changed` had (no `ui.panelToggle.*`
arm has to persist itself); `sync_session_chrome` reloads the persisted layer only when the session's app id
actually changes, so a `setActivePanelTab` reveal is never clobbered.

---

## 5. Command dock (packet 3)

`build_command_panel_ui` and `build_command_panel_row` are **un-gated** (as are the `UiNode`/`Label`/
`UiPresence`/`UiSelectNode`/… imports, `command_categories`, `chrome_text`, `draw_text` and `IconName`,
all of which were `#[cfg(test)]` purely because that builder was).

Because wgpu panel bodies are retained documents and no `UiNode → UiNodeRecord` assembler exists in the
shell, the painted Commands panel is published through the assembler that **does**:
`command_panel_measures(category)` expresses one category's rows as `WindowMeasure`s (a `Select` per
single-`Select`-arg command, a `Toggle` per zero-arg command, grouped under one `measure_group`), and
`publish_command_panel_document` mints them through `publish_window_measures` — the same revision-keyed
ingress every window overlay uses, so an unchanged category pays no ingress. These leaves are published
alongside the guest's panel surfaces in `refresh_ui`'s panel loop and are added to `retained` so
`retire_documents_outside` never sweeps them. `build_command_panel_ui` stays as the language-agnostic
`UiNode` oracle the command-registry test asserts against.

The `command-form:` search redirect (an arg-carrying command's staged form) now opens the **bottom-middle
Command anchor** instead of the deleted Settings→Commands tab, matching React's
`SET_PANEL_VISIBLE`/`SET_PANEL_PATH` pair.

---

## 6. Tests

New shared fixture: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧫️fixtures/🧭️default-dock/🔣️.json`
— the fixture app's panel tabs, the expected `DockSkeleton` for exactly that app, the canonical anchor
order, and the `anchorInsets` table. Both renderers can assert against it; the wgpu side does below.

`🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs` (rewritten around the new model):

| Test | Asserts |
|---|---|
| `panel_anchor_as_str_matches_react_panel_anchor_ids` | all 8 ids, `from_str` round trip, `index()` agrees with `ALL` |
| `default_dock_matches_the_shared_react_fixture` | **the parity pin** — anchor-by-anchor id order, then whole-skeleton equality against the fixture |
| `default_dock_puts_the_command_branch_on_bottom_middle` | packet 3's headline: `framework.category.command` is a real `bottom-middle` branch |
| `anchor_rects_sit_at_the_react_anchor_insets` | every anchor's rect against `anchorPositionStyle`'s insets + the fixture's `anchorInsets` table |
| `two_open_anchors_in_one_column_split_its_band` | no overlap; the bottom band ends one inset above the body |
| `anchor_rect_bands_only_the_open_anchors_of_a_column` | live geometry shrinks when a column's second anchor opens |
| `all_eight_anchors_open_independently` | 8 open at once; folding one leaves seven |
| `an_empty_anchor_never_opens` | no empty glass box |
| `column_projections_follow_their_corner_anchors` | `left_panel_open()`/`right_panel_open()`/`toggle_column` semantics |
| `reconcile_path_drills_a_branch_to_its_first_leaf_and_drops_vanished_tabs` | React's `reconcileActivePath` |
| `apply_dock_skeleton_moves_mentioned_tabs_and_appends_unmentioned_defaults` | the rearrange-never-reconstruct contract |
| `apply_dock_skeleton_is_identity_for_none_and_drops_unknown_ids` | stale-skeleton behaviour |
| `move_tab_in_dock_reanchors_a_tab_and_refuses_self_drops` | drag transform + self-drop refusal |
| `dock_override_round_trips_through_the_os_shell_config_document` | **override round trip** through the real `semio.os.config` `dockLayouts` layer, and clearing at the default |
| `dock_ui_state_round_trips_visibility_size_and_path` | per-anchor chrome round trip |
| `persist_dock_ui_if_changed_is_idempotent_when_nothing_changed` | the dirty-check |

`🧪️tests/🔬️wgpu-tutorial/🦀️.rs` — the two UI-snapshot tests rewritten onto anchors (the snapshot stays
`PanelGroup`-keyed, so each open *corner* anchor reports under the group it is the default home of).
`🧪️tests/🔬️wgpu-shell-chrome-parity/` is untouched and does not reference the deleted fields.

---

## 7. Remaining gaps

1. **Content-hug height.** React's panel hugs its content up to `calc(100% − 2·spacing)` (`🖼️Panel/🟦️.tsx:463`),
   never a fixed `bottom`. That needs a measured retained-document height this renderer has no read of yet,
   so a column's open anchors share its height in equal bands instead (§2). Identical to the old behaviour
   for one open anchor per column; the visible divergence is a two-anchor column, where React would show two
   content-sized boxes and wgpu shows two half-height ones.
2. **Tool branch has no leaves.** `FRAMEWORK_CATEGORY_TOOL_ID` is declared and reserved on `bottom-middle`
   (ordered left of Command, as React does), but wgpu has no mode-level tool-leaf builder — React's
   `buildToolTabs` has no wgpu twin. The branch is therefore not emitted.
3. **Marketplace tab.** React's `framework.marketplace` leaf on `bottom-right` has no wgpu content source
   (no marketplace host); not emitted.
4. **Framework Settings branch.** React nests app Settings tabs under one `framework.settings` branch with
   `general`/`theme`/`keybindings`/`default-apps`/`conflicts` children. wgpu's settings builders
   (`build_settings_theme_ui`, `build_settings_general_ui`) are still `#[cfg(test)]` and have no retained
   publication, so `bottom-right` currently carries the app's Settings-group tabs flat (History included).
   Closing this is the same "shell-owned panel content needs a records assembler" problem §5 solved for
   Commands, and can reuse `publish_window_measures` the same way.
5. **`trees` (PanelTreeUnit).** React's per-leaf tree-unit lists have no wgpu counterpart (a wgpu leaf hosts
   exactly one retained document). `DockTabSkeleton` neither reads nor writes `trees`; a React-written
   skeleton still parses (serde ignores it) but a wgpu write drops it.
6. **Drop preview.** The drag resolves its target anchor on pointer-up; there is no ghost/insert-indicator
   paint during the drag yet (React's `panelTabInsertPreviewClass`).
7. **Mobile flattening.** React collapses all eight anchors into one mobile panel (`mobilePanelTabs`); wgpu
   has no mobile panel path at all — out of this packet's scope.

---

## 8. Verification — what was actually run

Logs under `🗑️generated/w1h-*.txt`. `📜️w1h-verify.sh` (kept) is the retrying chain that produced them:
the crate's whole-tree check was SIGKILLed three times by memory pressure from the concurrent packet
fleet before a window opened.

**Observed, in this order:**

| Command | Result | Source revision |
|---|---|---|
| `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going` | **exit 0, 0 errors** (`w1h-verify.log`: `attempt=3 cargo=0`) | the packet minus the last three edits listed below |
| `cargo check … --target wasm32-unknown-unknown --keep-going` | **exit 0, 0 errors**, `Finished dev profile … in 3m 37s` (`w1h-wasm-check.txt`, `w1h-verify.log`: `wasm=0`) | same revision |
| `cargo test … -- panel_anchor_model_tests --nocapture` | **exit 0 — 23 passed, 0 failed** (`w1h-tests-anchor-model.txt`) | **final** revision |
| `cargo test … -- panel_anchor_model_tests tutorial_tests command_registry_tests shell_shortcuts_palette_tests` | 85 passed, **1 failed** (`w1h-tests-groups.txt`) | near-final (before the three test rewrites below) |
| `cargo test … -- shell_chrome_parity` | 27 passed, **1 failed** (`w1h-tests-parity.txt`) | near-final |

The final `cargo test … panel_anchor_model_tests` run compiled the lib **and** its test target from the
final sources and passed, so the tree parses and type-checks natively as it stands. `cargo test` output
was captured with `--nocapture`; the three `[DEBUG]` lines in the log are the tests' own witnesses.

**Not verified at the final revision** (checks were stopped on the coordinator's instruction — the tree is
being churned by sibling packets and a dedicated integrator takes it from here):

* `cargo check --lib` and the `wasm32-unknown-unknown` check were **not** re-run after the last three
  edits: (1) `dock_ui_snapshot` omitting a path equal to the anchor's default, (2) splitting the dock
  layer accessors into pure `read_os_shell_config_layer`/`write_os_shell_config_layer_in` over a
  `serde_json::Value` plus thin global wrappers, (3) the three storage tests rewritten onto an in-memory
  document. All three are target-neutral Rust with no `cfg`-conditional code, and (1) and (2) were
  compiled by the passing final `cargo test` run; only the `wasm32` *target* is strictly unconfirmed for
  them.
* No runtime/visual confirmation: the dock was not exercised in a running wgpu shell. Every claim about
  geometry, tab rows and persistence above rests on the unit tests listed, not on observed chrome.

**The two failures above are not this packet's.**

* `shell_chrome_parity_tests::the_overlay_row_steps_clear_of_an_open_floating_panel` fails at its *flush*
  assertion (`flush_controls[0].1[0]` is `151.2`, expected `7.2`) — the branch that passes `panels: &[]`,
  i.e. with no panel involved. The delta is exactly one status-pill width plus a gap, so
  `world3d_status_pill_for` now emits a pill where that branch assumes none, while the test's own
  *reserved* branch below already assumes the pill exists. None of `surface_overlay_controls_for`,
  `surface_overlay_row_origin`, `world3d_status_pill_for`, `world3d_status_pill_width`,
  `world3d_status_is_visible` or `surface_fits_overlay` appears in this packet's diff (checked against
  `git diff HEAD`), and all are pure functions taking their panel list explicitly; this packet's only
  touch nearby is `open_floating_panel_rects`, one of their *callers*, which the failing branch never
  reaches.
* `command_registry_tests::directory_home_bootstrap_retries_cancels_and_rebootstraps_without_cursor_loss`
  is pure `DirectoryClient`/bootstrap-epoch machinery in the 🛂️SpaceAdministration lane.
* A third, seen in the earlier full run and outside these filters:
  `dock::tests::dock_stack_content_fills_full_bounds_through_one_silhouette_clip`, in
  `🛰️Dock/🧪️tests/🔬️wgpu-unit/` — the dock-chrome packet's lane.

Peers were mid-refactor in this crate throughout: `NativeDirectoryTransport` imports in 🐚️Shell's
DirectoryAndIdentity region, `canvas_sat`/`canvas_lum`/`InkDocumentJson` in 🎞️Scenes, and `push_glass`
arity in the AgentApprovals overlay each blocked whole-crate checks at various points. Each check was
re-run until the Shell file's own error list was zero, and then — once the peers' code cleared — until the
whole crate was zero.
