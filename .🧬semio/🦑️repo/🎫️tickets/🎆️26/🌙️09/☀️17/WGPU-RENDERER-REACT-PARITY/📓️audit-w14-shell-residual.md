# 🧊 W14 — residual React↔wgpu shell-chrome gap audit (post-Wave-13)

Read-only, 2026-09-18. Covers: window system, 8 panel anchors, navbar+footer, overlays, keybinding
registry, per-pane chip rows, focus/hover states, control-id coverage. Method: read `📓️status.md`
(Waves 0–13), the Wave-0 audit (`📓️audit-shell-window-system.md`), and w1h/w1i/w2c/w2d/w4b/w6a/w12d/
w13a/w13b in full, then **re-verified every claim relied on with a fresh grep against the current
tree** (not trusted from the reports alone) — several items below were found this way and are not
recorded in any prior report. All paths absolute under `/Users/ueli/Documents/semio` unless noted.
React shell: `🏛️ShellHost/🟦️.tsx`, `📌️ChromePanels/🟦️.tsx`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx`,
`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx`. wgpu shell: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (23 075 lines),
`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`, os renderer `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`.

**Headline: almost everything the Wave-0 audit called P0 is now fixed.** DPI/logical-px (W1g), the
8-anchor panel dock + production command dock (W1h/W2c), the dead "new window" tab button + maximize
gating (W1i), mobile flat stack + drag-lane collapse + click-to-deactivate + fold chip + teardown
(W2d), per-pane Actions/Search/Window Options/Utilities/Projection chip rows and their bodies (W4b/
W9b/W12d/W13b), Agent-approvals modal + transient-notice banner (W1j), outcome colors from tokens,
production flex engine (W1l), the full window-scope + panel-anchor keybinding set (W2c), remappable
keybinding registry with settings UI (W2c). Do not re-dispatch fixes for any of these.

---

## A. Window system

| # | Gap | Sev | React | wgpu | Fix direction |
|---|---|---|---|---|---|
| A1 | Silhouette focus border only paints `active`/`normal`; React's SVG outline has 4 more per-kind CSS classes (`celebrated`/`introduced`/`loading`/`waiting`) for tour/status feedback | P2 | `WindowChromeSilhouetteBorder`, `⚛️react/🟦️.tsx:7787-7850` | `push_window_silhouette_border` call in `render_stack`, `🛰️Dock/…/🦀️.rs` (W2d G7/G8) | add the 4 remaining kinds once the tour/introduction/loading state model exists on this target |
| A2 | Mobile breakpoint (window dock `MODE_DOCK_MOBILE_MAX_WIDTH_PX` and panel dock's copy) reads `screen_w` — confirmed logical since W1g, but never re-verified end-to-end for the flat-stack/panel-flatten paths specifically | P2 (verify) | `UI_MOBILE_MEDIA_QUERY` CSS px | `🛰️Dock/…/🦀️.rs` `DockState::mobile`, `🐚️Shell/…` `mobile_panel_active` | live boot at 767px logical width, confirm both flatten together |
| A3 | Drag lane / close lane collapse rule: confirmed unified (`collapse_layout_node`) — **no gap**, listed only because W1i's G2 could be mistaken for still-open; it closed in W2d | — | — | — | none — re-verified fixed |

## B. Panel anchor model (8 anchors) & panel content

| # | Gap | Sev | React | wgpu | Fix direction |
|---|---|---|---|---|---|
| B1 | **Display panel leaves have no body at all.** `framework.display.windows` and `framework.display.layout` are real dock tabs (`FRAMEWORK_DISPLAY_WINDOWS_TAB_ID`/`FRAMEWORK_DISPLAY_LAYOUT_TAB_ID`, `🐚️Shell/…:92-93,6971-6972`) but `publish_shell_panel_document`'s match (`:15926-15945`) and `shell_owned_panel_leaves()` (`:15913-15920`) have **no arm for either id** — selecting the tab publishes nothing. **Not recorded in any prior wave report** — found by fresh grep for `framework.display.*` control ids, absent from wgpu. | **P0** | `buildDisplayWindowsTree` (`📌️ChromePanels/🟦️.tsx:157`) — per-window visibility/rename rows; `buildDisplayLayoutTree` (`:190`) — save-current-layout input+button, builtin/user named-layout tree with apply/delete | absent (tabs exist, `Ok(None)` from the router) | add `build_display_windows_ui`/`build_display_layout_ui` beside the existing `build_settings_*_ui` family, wire into `shell_owned_panel_leaves`/`publish_shell_panel_document`; `session.app.named_layouts` and `shell.applyNamedLayout` already exist (`🐚️Shell/…:10474,15791`) so the read side is half-built |
| B2 | **General settings is missing the merge-policy selector.** React's `framework.settings.mergePolicy` (`📌️ChromePanels/🟦️.tsx:473-474`) has zero hits in the wgpu shell. Found by grep, not in any prior report. | **P1** | `Select id="framework.settings.mergePolicy"` | absent from `build_settings_general_ui` | add the Select beside layout/driver/language/terminology, same dispatch pattern as `setLayout` |
| B3 | **Theme leaf has no export/import.** `framework.settings.theme.export`/`.import` (`📌️ChromePanels/🟦️.tsx:801,806`) — zero hits in wgpu. Found by grep. | P2 | export/import buttons beside save/reset/delete | absent | extends the already-scoped theme leaf (w2c gap 5) |
| B4 | Tool branch leaves paint options + armed state but no `tool_measures` (React's `windowMeasuresToTreeItems(toolMeasuresByToolId[id])`) | P1 | `buildToolTree`, `🛠️ShellHelpers/🟦️.tsx:4734` | `build_tool_panel_ui`, `🐚️Shell/…:6467` | program-bridge packet: add a `tool_measures_section` fetch beside `window_measures_section` |
| B5 | Marketplace leaf has no install/uninstall/reload/enable actions or install-from-url/file lane | P1 | `buildMarketplaceTree`, `📌️ChromePanels/🟦️.tsx:1295-1374` | `build_marketplace_ui`, `🐚️Shell/…:6352` (list-only) | needs a plugin-source registry + extension ledger on this target first |
| B6 | Conflicts leaf never shows a real conflict — always React's own "unavailable" fallback row | P1 | `buildConflictsTree` over `selectOpenConflicts`, `📌️ChromePanels/🟦️.tsx:995-1031` | `build_settings_conflicts_ui`, `🐚️Shell/…:6331` | needs a conflict-projection reader from 🐚️Shell's merge state |
| B7 | No driver editor (7 per-axis driver selects + save/delete) | P2 | General tab's 3rd section | `set_driver_field`/`save_driver`/`delete_driver` have no dispatch arm (`w2c` gap 4) | port alongside B2 |
| B8 | Theme editor stays select/reset/delete only; 8 further React sections (colors, spacing, fonts, strokes, radii, opacities, metrics, per-appearance paints) unported | P1 | `buildSettingsThemeTree`, `📌️ChromePanels/🟦️.tsx:693-828` | `build_settings_theme_ui`, `🐚️Shell/…:6034` | large packet, scope per section |
| B9 | Default-app pins (`framework.settings.default-apps`) are in-memory only, not persisted to `os.config.opening` | P2 | `OpeningPreferences` | `default_app_pins` field, no store round-trip | persist like `dockLayouts`/`dockUi` already do |
| B10 | `HostContent` escape-hatch layout reservation exists (`📐️flex/🦀️.rs:54`) but nothing dispatches to a host painter for it yet — currently unexercised (chat panel sidesteps it with plain `UiNode`s) | P2 | `emptyState` hatch, `🌳️Tree/🟦️.tsx:1042` | reserved band only, no paint callback | add the paint-side dispatch when a real non-`UiNode` consumer needs it |
| B11 | `s-sync-status` remains a **footer pill** (`render_footer_pills_step`), not the bottom-left dock's own tab hosting `SyncAttachCard`; confirmed unchanged since W4b by fresh grep (`FRAMEWORK_SYNC_PANEL_TAB_ID` constant exists but nothing publishes it as a dock leaf body) | P2 | `frameworkSyncTab`, `🏛️ShellHost/🟦️.tsx:9070` | `render_footer_pills_step`, `🐚️Shell/…:13021-13037` | move the pill's content into a `s-sync-status` panel leaf, same shape as the other framework leaves |
| B12 | An open panel's own padding (outside its retained rows) does not claim the pointer — a press there over a pane chip resolves the chip, where React's panel `<div>` would take it | P2 | panel element covers its own box | `pointer_is_over_open_panel` exists but the panel walk registers no box-level hit | register one low-priority hit for the panel's own rect in the panel lane |
| B13 | Pane Actions/Search chips still mount **unconditionally**; React mounts Actions only for `engagement \|\| actionPane` and Search only for a `search` spec | P2 | `🪟️Window/🟦️.tsx:379-412` | `ShellState::window_pane_chips` (flagged, not fixed, in both w12d and w13b) | gate the chip mount on the same body-presence check the builders already compute |
| B14 | Keybinding capture has no `aria-keyshortcuts` twin and no inline hotkey badge on ordinary controls (only navbar role chips got one, via W6a) | P2 | `ControlHotkeyBadge`, `⌨️control-hotkey-presentation/🟦️.ts:32` | tooltip-only | extend `shell_control_hotkey_badge` (W6a) to every bound control, not just role chips |

## C. Navbar + footer composition

| # | Gap | Sev | React | wgpu | Fix direction |
|---|---|---|---|---|---|
| C1 | World-orbit projection template selection is UI-only — does not move the camera (no orthographic frustum on this target, `setCamera` has no `projection` field) | **P1** | same limitation exists on React too (view-state only) — **not a gap vs React**, but the single biggest visible "nothing happens" moment in the shell | `World3dState::OrbitController` has no ortho frustum | W4b §7 / W6a #2's own open P1 — give the orbit an orthographic mode and a `projection` field on `setCamera` |
| C2 | `#s-checkin` has no trigger anywhere on wgpu (footer chip correctly removed per React, but no History-panel row replaces it — the shared framework history tree only carries `commitCheckpoint`) | P1 | `framework.history.checkin` row inside the History panel, `🏛️ShellHost/🟦️.tsx:9016` | `handle_checkin_action`/`checkin_dialog_draft` exist and are unreachable | add the row to the shared `🔌️plugin/🦀️.rs` history tree (converges both renderers on one source, per W6a's own note) |
| C3 | Navbar centre cluster is left-packed; React's is `centered: true` | P2 | `🏛️ShellHost/🟦️.tsx:10054` | `render_navbar_step` | trivial once the navbar phase layout is touched again |
| C4 | Chrome controls publish no accessible names (ARIA mirror covers scene trees only) | P2 | `sr-only` labels on triggers | `accessibilityMirror`, `🚀️browser-boot/🟦️.ts` | extend the mirror to navbar/footer/panel chips |
| C5 | `🎬️renderer-boot/🟦️.ts` (embeddable multi-mount door) publishes no readiness signal at all — only the trunk door (`🚀️browser-boot/🟦️.ts`, W6a) got `data-semio-os-ready` | P2 | per-shell `data-shell-ready` on each `[data-shell-id]` root | absent | same beacon, multi-mount milestone |

## D. Overlays

| # | Gap | Sev | React | wgpu | Fix direction |
|---|---|---|---|---|---|
| D1 | Confirmed implemented and verified by fresh grep, **not gaps**: Agent approvals modal (`agent_approvals` module + `AgentApprovalPaintOp`, `🐚️Shell/…:16571-20648`), transient notice banner (`ShellTransientNotice`, `:16399-16490`, 4000 ms auto-dismiss matching React), tooltips (400 ms, matches), context menus (independently OK per Wave-0 audit, unchanged), tour (W7a/W8a/W9a), example picker / role switch (navbar dropdown + `shell_role_controls`). | — | — | — | none |
| D2 | Only two real overlay "dialogs" exist on wgpu (agent approvals, checkin draft card); no generic confirm/alert dialog primitive was inventoried against React's Radix `Dialog` usages elsewhere in ChromePanels (Settings resets, theme delete, extension uninstall) — **not independently re-verified this pass**, flagged for a follow-up sweep rather than asserted as a gap | P2 (unverified) | multiple `Dialog`/`AlertDialog` usages in `📌️ChromePanels/🟦️.tsx` | not inventoried | a future packet should grep `AlertDialog\|<Dialog` in ChromePanels and cross-check each against a wgpu confirm-overlay |

## E. Keyboard shortcuts / registry

| # | Gap | Sev | React | wgpu | Fix direction |
|---|---|---|---|---|---|
| E1 | Confirmed fixed, re-verified by grep: all 3 window-scope chords (`ui.window.close/focus/newWindow`) and all 8 `panelAnchor.*` chords are present in `SHELL_SHORTCUT_ROWS` (`🐚️Shell/…:15190-15200`) and remappable through the Settings keybindings leaf (W2c). Not a gap. | — | — | — | none |
| E2 | `os.toggleFullscreen` chord still keyed to `F11`/`ctrl+meta+f` rather than `mod+shift+f` (Wave-0 audit finding, not revisited by any later wave — **not independently re-verified this pass**, flagged) | P2 (unverified) | `mod+shift+f` | `F11`/`ctrl+meta+f` | one-line remap once confirmed live |
| E3 | `mod+y` Windows-redo alias still absent (Wave-0 finding, not revisited) | P2 (unverified) | `mod+y` alias | absent | add to `SHELL_SHORTCUT_ROWS` |

## F. Per-pane chip rows / focus / hover

| # | Gap | Sev | React | wgpu | Fix direction |
|---|---|---|---|---|---|
| F1 | Engagement pane paints status rows + quick-action options only; React's `<Engagement/>` also renders a slider/stepper/ring/toggle-group/select row | P1 | `🎬️elements` (unspecified engagement control set) | `build_window_actions_ui`, `🐚️Shell/…:14413` | own vocabulary, belongs with the engagement lane (w13b gap 3) |
| F2 | Search input has one binding (commit-or-change fallback); React's carries both `onChange` (per-keystroke autocomplete) and `onSubmit` — no `Trigger::Submit`/`Abort`/`RepeatLast` producer exists anywhere in `🖱️ui` | P2 | dual binding | single binding | framework-level gap in the retained-input trigger vocabulary (w13b gap 4) |
| F3 | Staged-command-arg FORM section header (`action.category.<id>.form`) registers no hit; React's is a collapsible button | P2 | collapsible header | plain `Section` | give the form section a `TreeSection`-style header (w13b gap 5) |
| F4 | React's window caps at `y=32..61` are occluded by its own top-left panel (paints from `y≈28`); wgpu's panel paints from `y≈60`, below the cap row — the two renderers are measuring **different applications** at that band, so the whole cap-focus/cap-close parity comparison is unsettled until one geometry is corrected | **P1** | `Catalogue` panel painting over the cap row | panel starts below the cap row | resolve which renderer's panel-top inset is correct (likely wgpu's content-hug height, §w2c §4, needs its "unmeasured surface keeps full band" fallback checked against React's actual top offset); owner: panel-anchor lane (w13a §4) |
| F5 | `role-editor` mount journals one extra `noteShellCommand` React does not explain from available evidence (`shell.windowActivate` vs `shell.panelTab` both fit); wgpu deliberately declines to guess | P2 (needs instrumentation) | unresolved | unresolved | carry `commandId` into `InputLedgerRecordV1` (w13a §7) and re-run the probe |

---

## Over-implementation (wgpu has, React does not)

Nothing outstanding. The one candidate found in the read reports — the footer utility rail (Transform/
Brush/Volume Brush/Relocate/File/Folder/Remote painted once per shell) — was itself a divergence from
React (which scopes those per-pane) and was **removed** in W4b. The native-only sync/checkin indicator
(`#[cfg(not(target_arch="wasm32"))]` branch in `render_footer_step`) is a deliberate native-target-only
affordance with no React equivalent to diverge from (React has no native build), not a parity bug.

---

## Top 10 packets to dispatch

1. **B1 — Display panel bodies (`framework.display.windows`/`.layout`).** P0. Tabs exist, publish
   nothing. `🐚️Shell/…/🦀️.rs:15913-15945` (router), React ref `📌️ChromePanels/🟦️.tsx:157-249`.
2. **F4 — Panel-top geometry vs. window caps.** P1, currently blocks every cap-focus/cap-close parity
   run from comparing anything real. Owner: panel-anchor lane. `w13a-dock-close-reopen-journals.md` §4.
3. **C1 — World-orbit projection → camera link.** P1, the most visible "the control does nothing"
   moment once B1/F4 land. `♾️infinite/🌍️world/🦀️.rs` `OrbitController` + `setCamera`.
4. **C2 — `#s-checkin` trigger.** P1, a whole workflow action unreachable. Add
   `framework.history.checkin` to the shared `🔌️plugin/🦀️.rs` history tree.
5. **B6 — Conflicts leaf projection.** P1, silently useless in exactly the situation (a live merge
   conflict) where a user needs it.
6. **B4 + B5 — Tool measures + Marketplace actions.** P1 pair, both "program-bridge missing reader"
   shaped; can share one packet's investigation of the program-bridge fetch surface.
7. **B8 — Theme editor sections.** P1, largest remaining single-feature gap by section count (8 of 9
   React sections unported); scope as N sub-packets by section.
8. **B2 + B3 + B7 — General/Theme settings leaf completeness** (merge-policy select, theme export/
   import, driver editor). All three are small, same leaf family, same dispatch pattern as existing
   `setLayout`/`setDefaultApp` arms — good one-packet bundle.
9. **B13 — Gate Actions/Search chip mounting on body presence.** P2 but cheap and flagged twice
   (w12d, w13b) without anyone taking it.
10. **C4 + B14 — Accessible names + inline hotkey badges on ordinary controls.** P2 accessibility
    sweep, bundles the ARIA-mirror extension with the hotkey-badge extension since both touch the same
    chip-painting call sites.

Deferred/low-cost cleanup not worth its own packet: A1 (silhouette kinds), A2 (mobile breakpoint
re-verify), B9 (default-app persistence), B10 (HostContent paint dispatch — no live consumer yet),
B11 (`s-sync-status` tab), B12 (panel padding hit), C3 (navbar centering), C5 (multi-mount readiness),
F1–F3 (engagement/search input vocabulary — framework-level), F5 (needs instrumentation first), E2/E3
(re-verify before touching — Wave-0 findings never revisited).
