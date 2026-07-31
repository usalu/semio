# W2/W3 — Shell Chrome Feature-Parity Audit (WS6)

Audited `framework/renderer/wgpu/rs/lib.rs`'s `shell::ShellChrome`/`shell::ActionPanelAndUtilities`
against React's `framework/renderer/react/index.tsx`, re-verified the wave-3 reports (which were
stale), and closed the confirmed deltas. **Only file touched: `framework/renderer/wgpu/rs/lib.rs`.**
Native tests: **236 passed, 0 failed**, stable across repeated runs. `cargo check`/`cargo test -p
semio-framework-renderer-wgpu --lib` both clean.

## Drift found vs. wave-3 reports
`report-w3-command-palette.md`'s command table (`os.setExpertise`, `os.toggleCompact`) is stale — a
concurrent session already replaced `compact_mode`/`expertise` fields with a single `driver_id:
String`. React's chrome has no "expertise" concept at all (grep-confirmed), so that old concern is
moot.

## Confirmed, no gap
- Command palette execution (fuzzy search → category tagging → quick-search dispatch, including
  select-arg expansion) is fully wired end-to-end; Cmd/Ctrl+P confirmed.
- Ribbon nested-collection recursion fix still present/tested.
- Dialogs (`ChromeDialogRequest` stack, real Detach confirmation) still present/tested.
- Introduction/tutorial system present/tested.
- UISearch/find, context menus (light pass), selection marquee (scenes-layer, out of scope) all
  present.
- i18n key/lock/storage-key parity with React still byte-identical.

## Genuine deltas closed this pass
1. **Command palette panel wiring** (flagged gap: `build_command_panel_ui()` never inserted into
   `panel_ui`) — traced the real React target (a persistent `bottom-middle` dock anchor, which wgpu's
   2-column panel model has no equivalent of; building one means touching `dock`/restructuring
   `ShellTypes`, both out of scope). Wired the existing, tested `build_command_panel_ui()` output as
   a new `FRAMEWORK_SETTINGS_COMMANDS_TAB_ID` tab in the Settings panel column instead — an honest
   substitute.
2. **Theme settings tab was entirely unwired** — `PanelTabKind::SettingsTheme` existed in core but no
   UI could ever pick `"mono"` or a saved custom theme, even though `resolve_theme_for_ids` was
   already live in the render loop. Added `build_settings_theme_ui()` (picker + reset +
   delete-when-custom), wired as `FRAMEWORK_SETTINGS_THEME_TAB_ID`, dropped when `SEMIO_LOCKED_THEME`
   is set (matches React's lock behavior).
3. Added `os.setThemeId` command (+ `setThemeId`/`resetThemeId`/`deleteThemeId` dispatch arms) so
   theme switching also works via ⌘️K search.
4. **Panel-layout persistence wiring gap** — the flagged `ui.panelToggle.*` arms live in off-limits
   `ShellInput`, so instead added a render-loop dirty-check hook `persist_panel_layout_if_changed()`
   called once per frame from `render_chrome` (`ShellChrome`), achieving the same outcome without
   touching `ShellInput`.
5. **Storage duplication unified** — removed ~55 lines of duplicate `js_sys::Reflect`/native-file
   plumbing in `ShellLifecycle`; panel layout now persists through the same `prefs_get`/`prefs_set`
   (`PrefsStore`) as every other uiPref.

## Pre-existing bugs found and fixed (discovered while verifying, not caused by this pass)
- `build_command_panel_ui_groups_rows_under_category_headers` asserted a stale category count
  (leftover from a removed "general" category) — fixed to match current `build_os_commands`. **This
  is the same failure independently observed during the SceneHost workstream's verification run.**
- `persist_ui_prefs_if_changed_is_idempotent_when_nothing_changed` wrote to the real, non-isolated
  `~/.config/semio/ui-prefs.json` and never restored the value it perturbed, making a second suite
  run fail — fixed to toggle/restore properly; verified stable across 3 consecutive runs. **Also the
  same failure independently observed during SceneHost verification.**

## New tests added
`panel_layout_round_trips_through_prefs_store`,
`persist_panel_layout_if_changed_is_idempotent_when_nothing_changed`,
`build_settings_theme_ui_lists_builtins_and_gates_delete_on_custom_theme`,
`apply_os_command_set_theme_id_updates_active_theme`.

## Deliberately not done
- No real bottom-middle dock anchor (out of scope — touches `dock`/`ShellTypes`).
- No draft color/token theme editor UI (already-justified WP14 scope-down, stays unwired).
- No dedicated test for `right_tabs()`/`active_right_tab_id()`'s tab-list plumbing itself (thin glue,
  verified via full-suite green + unchanged generic tab-bar code).
- Left the General tab's outer label as `"General"` rather than React's literal `"Settings"` — a
  deliberate call since wgpu's Settings column now has 3 sibling tabs where React only ever has ≤2.
- New wgpu-only i18n strings are reasonable EN/DE pairs, not byte-verified against React's actual
  i18next bundle (lives in an external `elements/ui` package not vendored in this repo).

## Files touched
`framework/renderer/wgpu/rs/lib.rs` only, across `ShellLifecycle`, `ActionPanelAndUtilities`,
`ShellChrome`, and narrow additive arms in `ShellActions`. `ShellInput`, `dock`, `interpreter`,
`scenes`, `ui/wgpu/rs/lib.rs`, and the React reference were not touched.
