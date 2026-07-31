# w3-panel-dock-6anchor — final report

## Investigation findings

The earlier `report-w2-dock-dnd.md` was right that `dock` ≠ `PanelDock`. Traced the ACTUAL 6-anchor system and found **outcome 1 from the brief: wgpu has no generic 6-anchor system at all.**

- `shell::ShellState` (do-not-touch `ShellTypes` region) hardcodes exactly two independently-toggleable columns: `left_panel_open`/`active_left_kind: LeftPanelKind{Workbench,Display}` and `right_panel_open`/`active_right_kind: RightPanelKind{Details,Settings}`, each with a single width. No per-anchor list, no top/bottom distinction, no middle anchors anywhere.
- `group_side()` literally says in its own doc comment: "This renderer only has a 2-panel (left/right) layout; fold the framework's 6-anchor model back down to left/right." — self-aware of the gap.
- The real 6-anchor model already exists as a TYPE, just unused this side: `PanelGroup::anchor()` in `framework/core/rs/lib.rs:3698` maps `Workbench→top-left, Details→top-right, Display→bottom-left, Settings→bottom-right`, with its own doc comment noting the two middle anchors are drag/skeleton-populated only.
- The React reference (`ui/js/react/index.tsx:5164` `PANEL_ANCHORS`/`PanelDock`, `os-shell.tsx:4019` `DockAssembly`) implements the full generality: 6 independently visible anchors, each holding a tab tree, persisted via `dockLayoutStore`/`dockUiStateStore`, with drag re-anchoring (`moveTabInDock`, `computeTabDockDropZone`, etc.).
- No drag/re-anchoring of any kind exists in wgpu's panel system — only a resize-width drag (`panel_resize_origin_width`). Confirmed via grep for panel-drag payloads/sessions: none.
- Zero persistence: `ShellState::new()` always sets `left_panel_open: false`, `active_left_kind: Workbench`, etc.; the toggle handlers in `handle_shell_hit` (`"ui.panelToggle.*"`, in `ShellInput`) mutate fields and `return Ok(true)` directly — no save call, unlike `dock`'s `persist_dock_layout()`.
- `w3-prefs-i18n-themes`'s `PrefsStore`/uiPrefs work had not landed yet in `ShellChrome` at time of check (and later a mid-edit syntax error was observed there — see Build/verify below, tracked separately, not this agent's issue).

## What was built

All inside `framework/renderer/wgpu/rs/lib.rs`, within `shell::ShellLifecycle` only:

1. **`🧭PanelAnchorModel` subregion** (new, top of `ShellLifecycle`): `PanelAnchor` enum (6 variants, `as_str()`, `from_group()` mirroring `PanelGroup::anchor()` exactly), `PanelAnchorSnapshot` (visible/size/active_tab), `PanelLayoutPersisted` (serde struct), and load/save primitives — `js_sys::Reflect`-based localStorage access on wasm32 (avoids needing the `"Storage"` web-sys feature, not enabled, Cargo.toml off-limits), file-based (`$HOME/.semio/panel-layout.json`, `%APPDATA%\semio` on Windows) on native, no new crate dependencies.
2. **`🧭PanelAnchorAccessors`** (end of `ShellLifecycle`'s `impl ShellState`): `panel_anchor_snapshot(anchor)` — a real generic per-anchor projection over the existing hardcoded fields (re-homes Workbench/Display/Details/Settings onto TopLeft/BottomLeft/TopRight/BottomRight; middles always empty, matching upstream); `panel_layout_snapshot()`/`apply_panel_layout()`; `persist_panel_layout()`/`load_persisted_panel_layout()` (the latter now called from `ShellState::new()`).
3. **Important limitation, honestly scoped**: since `ShellState`'s fields live in the do-not-touch `ShellTypes` region, could not add true independent per-anchor storage (e.g., simultaneously-visible Workbench+Display) — the anchor model is a real, generic ACCESS SURFACE over the legacy 2-column storage, not new independent state.
4. **10 new tests** in `mod panel_anchor_model_tests`: anchor↔`PanelGroup` mapping, anchor id strings, visibility/active-tab correctness per anchor, middle-anchor emptiness, snapshot/apply round-trip, sparse-snapshot field preservation, native path-namespacing, and a real file round-trip.

## Wiring request
`handle_shell_hit`'s `"ui.panelToggle.*"` arms (currently ~4 arms, in the do-not-touch `ShellInput` region this wave) need one `self.persist_panel_layout();` call each, mirroring how `dock.focus.*`/`dock.close.*` call `persist_dock_layout()`.

## Build/verify
- `cargo check -p semio-framework-renderer-wgpu --lib`: clean at time of finishing (only pre-existing warnings).
- `cargo test -p semio-framework-renderer-wgpu --lib`: **134 passed, 0 failed** (baseline 121; +10 mine, +3 from other concurrent landings) — captured before a later concurrent edit broke the build.
- A LATER re-run failed with `E0753` (misplaced `//!` inner-doc-comment) — this is `w3-prefs-i18n-themes`'s in-flight edit landing a syntax error in the do-not-touch `ShellChrome` region, unrelated to this work (confirmed via grep: zero occurrences of `panel_anchor`/`PanelAnchor`/`PanelLayout` in the error output). **TRACKED SEPARATELY — must confirm `w3-prefs-i18n-themes` fixes this before its own work is considered done.**
- `cargo check --target wasm32-unknown-unknown` also currently fails on unrelated pre-existing/concurrent gaps (`apply_os_command`, `command_search_items`, `fuzzy_match_score` not found) — zero mentions of this agent's code; likely `w3-command-palette`'s in-flight work not yet complete.

## Files touched
`framework/renderer/wgpu/rs/lib.rs` — `shell::ShellLifecycle` only.
