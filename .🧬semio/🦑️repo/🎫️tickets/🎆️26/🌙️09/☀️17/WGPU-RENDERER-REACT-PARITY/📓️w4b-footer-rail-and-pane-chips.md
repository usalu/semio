# 🪟️ W4b — the footer rail, the per-pane overlay rows, and the window cap label

Packet W4b of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. Input evidence: `🗑️generated/w3c-fix-3/run-1/final.png`
(wgpu, 1440×900, dpr 1) against `🗑️generated/w3b-react/final.png` plus W3b §5's measured React DOM
geometry (§5 is the only honest React reference for the pane rows — the React capture is blurred by the
boot-tour backdrop, so nothing here is claimed off that image's pixels). Line numbers are post-edit and
the file carries a peer's in-flight sync/presence rewrite in the same region (§6).

---

## 1 — The footer utility rail has no React counterpart

**Symptom.** The wgpu footer read `Transform · Brush · Volume Brush · Relocate · File · Folder · Remote ·
Display | Tool · Command | Settings · Marketplace · History`. React's reads
`Display · Remote: detached | Tool · Command | … Settings · Marketplace · History`.

**Root cause.** `render_footer_step`'s phase 2 walked `ShellState::active_utilities` — the APP's declared
utilities scoped to the active window — straight onto the footer, and `refresh_ui` appended
`framework_sync_utilities(sync_backbone_uri)` (`File`/`Folder`/`Remote`) to that same roster. React does
neither:

- its utilities live on each window pane's own `Utilities` `Pane`
  (`🪟️Window/🟦️.tsx:399-411`, `anchor="bottom-left"`, fed `utilityBarNode(utilities, instance.id, …)`
  from `🏛️ShellHost/🟦️.tsx:10263`), derived **per pane instance**, not once for the shell;
- its `buildFrameworkSyncUtilities` leaves (`🛍️products/💻️os/🟦️.ts:506`, the same three ids) are handed to
  the bottom-left `s-sync-status` tab's `SyncAttachCard` (`🏛️ShellHost/🟦️.tsx:9076`) — the ONLY consumer
  in the React tree. Nothing puts them in a bar.

**Fix.**
- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:17755` — `render_footer_step` phase 1 now goes straight to phase 3. The
  rail phase, its cursor descent and its `lead_x` hand-off are gone, so the bottom-left band opens at
  `theme.padding_standard` like React's.
- `🐚️Shell/…:5523` — the `framework_sync_utilities` append is gone; the builder and
  `backbone_kind_from_uri` were its only callers and are deleted with it (no legacy shim: the sync
  backbone selection belongs to the sync panel on this target too).
- `🐚️Shell/…:12183-12280` — `render_footer_utility_node` → `render_utility_node_step`,
  `footer_utility_at_path` → `utility_at_path`, `footer_utility_sibling_count` → `utility_sibling_count`,
  `render_footer_section_divider` → `render_utility_section_divider`, `footer_utility_label` →
  `utility_node_label`. Same steppers, no longer named after a bar that no longer exists; the pane rail
  drives them verbatim.
- `🐚️Shell/…:13697` — new `derive_window_utility_nodes(session, window_id)`: React's per-instance
  `resolveUtilityNodes`, resolving the kind from the window INSTANCE and marking **that pane's** active
  utility pressed. `derive_utility_nodes` is now a one-line wrapper over the active window, so
  `active_utilities` (the hit router's and the keybinding lane's roster) is unchanged in meaning.

## 2 — The per-pane overlay rows were absent

**Symptom.** React paints, per window pane, a top row `Actions` (left) · `Search` (centred) ·
`Window Options` (right) and a bottom row `Utilities` (left) · `Projection` (right). wgpu painted ONE
per-pane affordance: an unlabelled 22 px `settings-2` square at the pane's top-right corner
(`paint_measures_fold_chip`), which is the stray icon visible at (468, 68) and (1422, 68) in
`w3c-fix-3/run-1/final.png`.

**Root cause.** There was no pane-chrome lane at all. The state each chip drives already existed and was
written by keybindings, the palette and the action lane — `action_panel_folded`, `measures_folded`,
`active_utility_by_window` — but **nothing read it to paint a chip**, and `handle_shell_hit` carried
`shell.action.fold.*`, `shell.measures.{fold,unfold}.*` and `shell.engagement.toggle.*` arms that no
chrome ever registered.

**React reference.** `🪟️Window/🟦️.tsx:320-412` mounts four `Pane`s inside `window-body`
(`measures` top-right, `engagement` top-left, `search` top-middle, `utilityBar` bottom-left), and
`WorldOrbitProjectionSwitchPane` (`🌐️World3dHost/🟦️.tsx:5188-5215`) portals a fifth into the same host at
bottom-right. Each folds to a chip with a FIXED semantic icon plus label (`WINDOW_PANE_*_ICON`,
`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:8201-8204`) and is positioned by `anchorPositionStyle` (`:6864`): a corner
anchor insets `--spacing-single` on BOTH its edges, a middle anchor centres on that axis.
`--spacing-single` is `1 × --ui-spacing` (`🎨️styling/🖌️ui/🎨️.css:733`), the same token
`PANEL_INSET_UI_SPACING` gives `theme.panel_inset` — so both renderers resolve the inset from one source.

**Fix.** New region `🪟️WindowPaneChrome`, `🐚️Shell/…:13024-13270`:

| site | what |
| --- | --- |
| `🐚️Shell/…:13031` | `WindowPaneChip { Actions, Search, WindowOptions, Utilities, Projection }` with `anchor()`, `icon_id()`, `label_key()` and `control_id(window_id, folded)` |
| `🐚️Shell/…:13097` | `window_pane_chip_rect(theme, body, anchor, width)` — `anchorPositionStyle` on this target's own token |
| `🐚️Shell/…:13134` | `ShellState::window_pane_chips(window_id)` — which chips a pane mounts, each with its fold state and whether its toggle is live |
| `🐚️Shell/…:13151` | `paint_window_pane_chips_step` — one chip per opportunity through the existing `render_retained_chrome_group_item_step`, each on its own glass region, each registering its hit above the body's `ScrollRegion` |
| `🐚️Shell/…:13198` | `paint_window_utilities_step` — the UNFOLDED `Utilities` rail, to the right of its chip on the pane's bottom row, walked with the stepper the footer rail used |
| `🐚️Shell/…:16858` | new walk phases 9 (chips) and 10 (rail) between the measures rail and the next window, so the chips outrank the body and the rail in `hit_at`'s reverse scan |
| `🐚️Shell/…:3858` | `paint_measures_fold_chip` and `MEASURES_FOLD_CHIP_ICON_PX` deleted — the labelled `Window Options` chip in the top-right band replaces them, at the identical rect (the measures rail is right-aligned at the same inset, so the open rail's cap and the folded chip coincide, which is what React's `Pane` cap does) |

**Action ids dispatched** (`🐚️Shell/…:9623-9680`; existing arms reused where they existed):

| chip | wgpu control id | React `toggleId` | lane |
| --- | --- | --- | --- |
| Actions | `shell.action.fold.<windowId>` | `framework.window.<id>.engagement.toggle` | `action_panel_folded` |
| Search | `shell.window.search.toggle.<windowId>` **(new arm)** | `framework.window.<id>.search.toggle` | the SAME `action_panel_folded` React's two panes share (`🪟️Window/🟦️.tsx:369`/`:388`), plus focusing that pane and opening the palette — focusing is what scopes it to the window |
| Window Options | `shell.measures.{unfold,fold}.<windowId>` | `framework.window.<id>.measures.{unfold,fold}` | `measures_folded` |
| Utilities | `shell.utilityBar.{unfold,fold}.<windowId>` **(new arms + new `utility_bar_folded` state)** | `framework.window.<id>.utilityBar.{unfold,fold}` | `utility_bar_folded` |
| Projection | `shell.projection.fold.<windowId>` | `framework.worldOrbit.projection.<segment>.pane.fold` | **mounted disabled — see below** |

**Mount rule.** React mounts the projection pane only from inside a world surface and mounts every other
pane unconditionally, DISABLING the toggle of one with no body (`toggleDisabled={!utilityBar}`,
`🪟️Window/🟦️.tsx:403`). That is the rule implemented: `Utilities` is disabled when the pane's kind
declares no utilities, `Window Options` when the pane projects no measures document, and `Projection`
mounts only for a `world3d_states` pane and is **always disabled on this target** — `World3dState` holds
an `OrbitController` and no `WorldProjectionSpec` at all, so there is no switch to unfold. React's own
contract for a pane with no body is the disabled toggle; inventing a projection verb here would have been
worse. **This is a World3d-lane gap, not a chrome gap** — see §5.

**Bodies that still do not exist on this target** (chips are live, the pane they open is not): the
Actions/engagement pane (`window_engagements` is fetched at `🐚️Shell/…:5533` and read by nothing that
paints) and the per-window Search pane (the chip opens the shell palette instead). Those are their own
lanes; the Utilities rail is the one body this packet delivers.

## 3 — Window cap read `Puzzle 3D` where React reads `Top` / `Perspective`

**Root cause.** `dock_chrome_maps` (`🐚️Shell/…:16673`) labelled every pane from
`kind.label.resolve(terminology, locale)` — the window KIND's manifest label — because
`stack_from_node` (`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:748`) drops `WindowLayoutWindowNode::title` when it
builds a `DockStackTab`. puzzle3d's `defaultLayout` authors `{"instanceId":"puzzle3d-main-top","title":"Top",
"windowKindId":"puzzle3d-main"}` and `…-perspective`/`"Perspective"` (`✏️s/🔌️plugins/🧩️puzzle/🔣️.json`), so
both panes read the kind label.

**React reference.** `windowTitlesById[instance.id] ?? instance.title` (`🏛️ShellHost/🟦️.tsx:10296`), where
`instance.title` comes from `resolveFrameworkLayoutSeed`'s `extraInstances`
(`🛠️ShellHelpers/🟦️.tsx:1555-1566`) — a SIDE map keyed by instance id, populated only for a node whose
`instanceId` differs from its `windowKindId`. A bare kind keeps the manifest label (`:10255`).

**Fix.**
- `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:738` — new `window_layout_instance_titles(layout)`, the Rust twin of that
  side map, walking axis/stack/window nodes.
- `🐚️Shell/…:16679` — `dock_chrome_maps` prefers it (default layout, then any live `layout_override`
  merged over it) and falls back to the kind label. Keyed by instance id exactly like React's
  `extraWindowInstancesRef`, so a pane the user drags keeps its authored title even though this
  renderer's captured layout carries no titles on its window nodes.

## 4 — Other pane-level differences

**Fixed: the axes gizmo sat ON the pane's bottom edge, where the new bottom row goes.**
`orbit_view_gizmo_placement` (`🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs:1037`) priced BOTH axes at
`chrome_inset + gizmo_half_extent` and clamped both against the SHORTER side's third. React's
`resolveSceneGizmoViewportPlacement` (`🖱️ui/🧱️elements/🎬️Scene/🟦️.tsx:390-406`) gives the block axis
`chromeInset + foldedPaneChromePx + chromeInset + gizmoHalfExtent` precisely so the navigation cube sits
ABOVE the folded projection pane, and clamps each axis against its own third. Measured oracle (React's own
test, `🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx:9555`): `[32, 58]` at 1280×720, `[32, 53]` at
120×160, `[22, 22]` at 40×48 — against this renderer's `(32, 32)`, `(32, 32)`, `(22, 22)`. Now mirrored
term for term off the same `UI_SPACING_COMPACT_PX`/`CONTROL_HEIGHT_UI_SPACING` tokens, and
`world_orbit_view_gizmo_placement_matches_react_bottom_right_insets` re-pointed at React's numbers.

**Not actionable from the evidence.** Grid colours, pane border radii, gutter width and tab-bar height
cannot be compared honestly: the only React capture at this viewport (`🗑️generated/w3b-react/final.png`)
is blurred wholesale by the boot-tour backdrop, and the React serve on 6013 was still down during W3c.
Pane borders/radii and the split gutter are already pinned by their own laws
(`dock::tests::split_resize_gutter_hit_is_twenty_pixels_centred_on_the_seam`,
`…window_silhouette_v1_matches_typescript_fixture_…`, both green), and the cap row height is
`theme.control_height` on both sides. Whoever recycles 6013 should re-take the reference with the tour
dismissed before anyone changes a colour on a hunch.

## 5 — Tests

`🐚️Shell/🧫️fixtures/🪟️window-pane-chrome/🔣️.json` — new shared fixture: the chip table (anchor, icon, both
locales' label, both fold states' control id, React's own id, mount rule), the pane anchor insets, and the
footer composition with its forbidden control/utility ids.

`🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs` — new, **9 laws, all run**:

| law | what it pins |
| --- | --- |
| `pane_chips_match_the_react_pane_fixture` | anchor, icon, en+de label and BOTH fold states' id, chip by chip, in React's mount order |
| `pane_chip_rects_band_a_two_pane_split_at_reacts_anchor_insets` | every chip on its anchor's own two edges, inside its own pane, rows non-overlapping — for BOTH panes of a two-pane split at 1440×900, driven through the real `plan_dock_windows` |
| `pane_chips_mount_exactly_where_react_mounts_them` | the mount/disable rule, including a utility-less window and a measure-less window |
| `the_window_options_chip_registers_both_sides_of_its_fold` | painted through the REAL chip row: one glass region per mounted chip, a disabled chip registers no hit, an enabled one does |
| `each_pane_chip_dispatches_its_own_window_state` | every chip's id flips only ITS pane's lane; Search drives the same `actionsFolded` Actions does and focuses its pane |
| `a_panes_utility_rail_is_derived_per_pane` | two panes of one kind never share a pressed utility |
| `an_unfolded_pane_utility_rail_paints_inside_its_own_pane` | folded paints nothing; unfolded paints the app's utilities right of the chip, on the pane's bottom row, inside the pane |
| `the_footer_carries_no_utility_rail` | a whole painted footer registers no `framework.utility.*` hit and no `framework.sync.*` leaf, and the shell's roster no longer carries them |
| `window_caps_read_the_authored_instance_title` | `Top`/`Perspective` from the layout node; a bare KIND still wears its manifest label |

`🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs` — `window_options_fold_chip_is_registered_on_both_sides_of_the_fold`
removed; its law now lives in the pane-chrome suite, painted through the row that replaced the icon chip.

`♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs:1533` — the gizmo placement law re-pointed at React's own
`[32, 58]` / `[32, 53]` / `[22, 22]` oracle.

### VERIFY (all foreground, all RUN, logs under `🗑️generated/w4b-*.txt`)

| gate | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | **0 errors** | `w4b-native-check-final.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | **0 errors** | `w4b-wasm-check-final.txt` |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4` | **0 errors** | `w4b-check-ui.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --tests -j 4` | **0 errors** | `w4b-check-tests.txt` |
| `cargo test -p … -renderer-wgpu --lib -- --test-threads=1 window_pane_chrome_tests` | **9 passed / 0 failed** | `w4b-test-pane.txt` |
| `cargo test -p … -renderer-wgpu --lib -- --test-threads=1 dock::` | **67 passed / 0 failed** | `w4b-test-dock.txt` |
| `cargo test -p … -renderer-wgpu --lib -- --test-threads=1 panel_anchor dock window_measures chrome_overlays shortcuts` | **179 passed / 1 failed** | `w4b-test-neighbours.txt` |
| `cargo test -p semio-framework-os-infinite --lib -- --test-threads=1 gizmo` | **2 passed / 0 failed** | `w4b-test-gizmo.txt` |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- --test-threads=1 chrome widget text glyph` | **87 passed / 0 failed** | `w4b-test-ui.txt` |
| `cargo test -p … -renderer-wgpu --lib -- --test-threads=1 shell::` | **310 passed / 10 failed** | `w4b-test-shell-serial.txt` |

**The 10 `shell::` failures are the SAME set W3b §"remaining failures" attributed, none of them this
packet's**, each re-attributed by code path against files this packet never opened:

- `window_silhouette_border_emits_notched_outline_segments` — pure `WindowSilhouette::from_measured_top` +
  `push_window_silhouette_border` geometry: no atlas, no dock, no chip. Identical failure text to W3b's.
- `window_measures_overlay_paints_…`, `tool_run_panel_*` (2), `shell_document_retirement_*` (2) — all fail
  with `resident capacity exhausted … committed items 24595/131076`. Proof they are arena PRESSURE, not
  logic: `window_measures_overlay_paints_and_dispatches_every_gesture_like_react` **passes** in the
  narrower `w4b-test-neighbours.txt` run and fails only when the whole `shell::` suite shares one process.
- `ui_prefs_themes_i18n_*` (2) — process-global env locks.
- `the_overlay_row_steps_clear_of_an_open_floating_panel` — `world3d_status_pill_width`'s lead, W3b's own
  hand-off, untouched here.
- `directory_home_bootstrap_retries_…` — directory bootstrap frontier, unrelated lane.

Baseline for comparison: W3b recorded 291 passed / 10 failed on the same filter; this run is 310 / 10 (the
9 new laws plus a peer's).

## 6 — Concurrency note

A peer rewrote the footer's sync/presence lane in this same file while this packet ran
(`ShellState::sync_pill_text` → `shell_sync_pill_text` + `sync_pill`/`footer_presence_rows`/
`presence_bar_chip_text`, and `render_footer_step` phase 3 **un-gated from `#[cfg(not(wasm32))]`** — which
closes W3b §3c's "the browser boot paints no sync pill at all"). For about 20 minutes their production
rename had landed and their test file had not, so `--lib --tests` failed with 8 `sync_pill_text` errors in
`🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs` alone; nothing of theirs was touched or reverted, and the
gate above was re-run clean once they caught up. Their phase 3 and this packet's removed phase 2 are
adjacent — the current footer reads phase 0 → 1 → 3 (sync + check-in) → 5 (tab bands), with `cursor.x`
still opening at `theme.padding_standard`.

## 7 — What the coordinator must confirm live

Nothing below was seen in a browser: no `activate-*`, no `framework-renderer-wgpu:wasm`, no trunk.
After the next real boot of `?plugin=puzzle3d`, check in order:

1. **The footer** reads `Display · Remote: … | Tool · Command | Settings · Marketplace · History` — no
   `Transform · Brush · Volume Brush · Relocate` and no `File · Folder · Remote`.
2. **Both window caps** read `Top` and `Perspective`, not `Puzzle 3D` twice.
3. **Each pane** shows a top row `Actions` (left) · `Search` (centred) · `Window Options` (right) and a
   bottom row `Utilities` (left) · `Projection` (right, dimmed). The unlabelled `settings-2` square is
   gone.
4. **The chips are crisp, not blurred** — each opens its own glass region and re-enters it on every paint
   opportunity; W3c §3's "the glass erased its own label" is exactly the failure mode to watch for, and
   the fix here was written against it but only proven natively.
5. **Pressing `Utilities`** opens the app's utility chips on that pane's bottom row (`Transform · Brush ·
   Volume Brush · Relocate` for puzzle3d), inside that pane, and pressing it again folds them. Pressing it
   on ONE pane must not move the other's.
6. **The axes gizmo** now sits ~58 px above each pane's bottom edge instead of on it, clear of the
   `Utilities`/`Projection` row.
7. **Boot cost** — the window walk gained two phases per pane (chips, rail). Each chip is one
   `render_retained_chrome_group_item_step` walk, so the added work is ~5 chips × 2 panes per frame, but
   watch `declare_boot_subphase` and the present watchdog anyway.

Blocked here, for whoever owns the lane:

- **P1 — the Projection pane has no spec to switch.** `World3dState` (`♾️infinite/🌍️world/🦀️.rs:1315`)
  carries an `OrbitController` and no `WorldProjectionSpec`; React's pane switches
  `worldProjectionKindSwitchSpec`. Until that lane lands the chip is mounted disabled by design.
- **P1 — the Actions/engagement pane and the per-window Search pane have no body.**
  `ShellState::window_engagements` is fetched every refresh and read by nothing that paints; the Search
  chip opens the SHELL palette scoped by focus rather than React's per-window typed-action pane.
- **P2 — `s-sync-status` is still a footer lead, not a bottom-left tab.** React makes it the bottom-left
  band's first tab (`singleTreeLeaf({ id: "s-sync-status", order: 0 })`, measured at x78 after `Display`
  at x3); here it is painted ahead of the band by `render_sync_status_and_checkin`. Same visual order,
  different owner — and the peer's lane, not this one's.
