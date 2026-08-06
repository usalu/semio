# w3-prefs-i18n-themes — final report

Implemented WP14 (uiPrefs persistence, `SEMIO_LOCKED_*` locks, named/custom themes + draft editor, DE chrome-string bundle) entirely inside `framework/renderer/wgpu/rs/lib.rs`'s `shell::ShellChrome` sub-region, plus one necessary one-line touch outside `mod shell` (the `frame()` call site) and one one-line bugfix in a concurrently-added function that was blocking compilation for everyone.

File touched: `framework/renderer/wgpu/rs/lib.rs` only (plus a scratch helper `resolve-mono-chrome.ts` used once to compute real color values, left in the ticket folder per policy).

## PrefsStore design
A `PrefsStore` trait with two backends, added as a new sub-region at the tail of `ShellChrome`'s `impl ShellState` block:
- **wasm32**: `WebLocalStorage` — reaches `localStorage` via raw `js_sys::Reflect`/`js_sys::Function` calls against the already-enabled `"Window"` web-sys feature, since `"Storage"` isn't enabled and `Cargo.toml` is a reserved choke point this wave.
- **native**: `FilePrefsStore` — a JSON file at `$SEMIO_PREFS_DIR/ui-prefs.json`, falling back to `$XDG_CONFIG_HOME`/`$APPDATA`/`$HOME/.config`/semio — zero-touch, no new dependency (`serde_json` already a dep).
- Both behind a `thread_local!` singleton (`PREFS_STORE`) with `prefs_get`/`prefs_set` free functions.

**Notable discovery**: a concurrent agent (`w3-panel-dock-6anchor`, in `ShellLifecycle`, off-limits to this agent) independently built an ALMOST IDENTICAL mechanism (`local_storage_get_item`/`local_storage_set_item` via `js_sys::Reflect`, plus a native `%APPDATA%`/`$HOME`/`semio/panel-layout.json` file store) for panel-layout persistence, arriving at the same `js_sys::Reflect` workaround for the same `Cargo.toml` constraint. This is a real, honest duplication from parallel work — **flagging as a wiring/dedup request**: a future integrator pass could unify both under one shared `PrefsStore`.

## Key-string parity with React (confirmed byte-identical)
Verified directly against `ui/js/react/index.tsx:2100-2318`: `ui.chrome.appearance`, `ui.chrome.locale`, `ui.chrome.terminology`, `ui.chrome.compact`, `ui.chrome.expertise`, `ui.chrome.layout`, `ui.chrome.theme`, `ui.themes.custom`, `ui.compute.workerCount`, `ui.introduction.seen.<appId>`, and `native` (terminology default) — all defined as `const` with doc-comments pointing at the exact React line.

## Lock mechanism
`ShellPrefLocks`/`shell_pref_locks()` reads `SEMIO_LOCKED_APPEARANCE`/`SEMIO_LOCKED_LOCALE`/`SEMIO_LOCKED_TERMINOLOGY`/`SEMIO_LOCKED_THEME` (matching `os-shell.tsx`'s `FrameworkOsLocks` — compact/expertise/layout/customThemes deliberately stay unlocked, matching React). `load_ui_prefs_once` prefers a lock over storage; `persist_ui_prefs_if_changed` skips writing locked fields. Native-only in practice — wasm32-unknown-unknown has no real process env, so locks are inert in browser builds (a kiosk/demo concern for the `semio-wgpu-native` binary).

## Theme registry + custom themes + draft editor (scoped down, documented why)
- `resolve_theme_for_ids(theme_id, appearance_id)` extends `resolve_theme` with `"semio"` (unchanged), `"mono"`, and `"custom.<slug>"`.
- `mono_theme()` uses REAL resolved RGBA values: ran `ui/styling/js/theme.ts`'s own `resolveThemeAppearancePalettes` against `mono.theme.json` (via the scratch script) rather than inventing colors, then hand-ported via `Rgba::from_srgb8` — necessary because this crate has no dependency on the `ui_styling` Rust codegen crate and `ui_wgpu`'s `ChromePalette`/`from_chrome` aren't `pub`.
- **Scoped down deliberately**: custom themes/drafts cover only 5 color slots (background/panel/navbar/text/accent) rather than React's full 8-section `UiTheme` — porting the complete token/mix resolver was out of proportion to this ticket. Draft editor is a programmatic API only, no visual color picker, per the ticket's own allowed scope boundary.

## i18n / terminology
`shell_chrome_string(key, is_de)` routes a curated, verified subset of previously-hardcoded strings through EN/DE pairs copied verbatim from `ui/js/react/index.tsx:2898-3975`: `display.tab.windows/layout`, `settings.tab.general`, `fullscreen.toggle`, `panelToggle.display/workbench/details/settings`, `common.home/windowOptions/focus/unfocus/execute/reset`. Also fixed the "Document" panel tab to route through the existing `ui_wgpu::framework_panel_tab_label`. Deliberately AVOIDED `render_overlay` (Search/Find/Examples/Attach/Detach) and the actions/engagement rails — overlap `w3-overlays-chrome-polish`'s claimed turf. **Terminology substitution**: found no hardcoded terminology-sensitive strings in `ShellChrome` — the actual substitution mechanism (`ViewModel.terminology` forwarding to `resolveAppDocument`) was already wired upstream.

## Tests added (14 new, in a new `mod ui_prefs_themes_i18n_tests`)
Round-trip (`FilePrefsStore` via disk, isolated from the real prefs path), lock env-var parsing, lock-wins-over-storage + "load once" gate, persist dirty-check idempotency, `resolve_theme_for_ids` semio-vs-mono divergence with a real hex spot-check, hex parsing/fallback, full custom-theme draft→save→resolve→delete flow, discard, layout round-trip/rejection, EN/DE bundle parity spot-checks, introduction-seen key format.

## Build/test results
- `cargo check -p semio-framework-renderer-wgpu --lib`: Finished, 0 errors (only pre-existing/dead-code warnings).
- `cargo test -p semio-framework-renderer-wgpu --lib`: **159 passed, 0 failed** (baseline had grown from 121 to 159 from concurrent agents landing first; all 14 of this agent's included, no regressions).

## Fix made outside own new code
`render_chrome_tooltip` (added concurrently by `w3-overlays-chrome-polish`) called `app_now_ms()` unqualified from inside `mod shell`, but that function lives at crate root — a compile error blocking everyone. Fixed with a one-line `crate::app_now_ms()` qualification; did not touch any other logic there.

## Dead-code triage
- **Not this agent's**: `chrome_start_introduction`, `engagement_completion_suffix` — belong to `w3-overlays-chrome-polish`'s introduction-tour/engagement-ghost-text work.
- **This agent's, genuinely unwired pending another agent**: `custom_theme_ids`, `begin_custom_theme_draft`, `set_draft_theme_color`, `save_draft_theme`, `discard_draft_theme`, `delete_custom_theme`, `set_active_ui_layout`, `set_active_worker_count`, `read_stored_introduction_seen`, `write_stored_introduction_seen`, `ChromePrefsState.draft_theme` field — only reachable once a settings-panel UI (or the command-palette agent's `os.setLayout`/theme commands) calls them. All exercised by tests, proven correct, just not yet invoked from production render paths.

## Wiring requests
1. Unify `local_storage_get_item`/`local_storage_set_item`/`panel_layout_store_path_under` (ShellLifecycle, `w3-panel-dock-6anchor`) with this ticket's `PrefsStore`/`WebLocalStorage`/`FilePrefsStore` — duplicate mechanisms solving the identical `Cargo.toml`-constraint problem.
2. `w3-command-palette`'s `os.setLayout`/`os.setThemeId`/theme commands should call into `shell::set_active_ui_layout`/`shell::set_active_theme_id`/the draft-editor functions above to actually surface this wave's persistence work to users.
