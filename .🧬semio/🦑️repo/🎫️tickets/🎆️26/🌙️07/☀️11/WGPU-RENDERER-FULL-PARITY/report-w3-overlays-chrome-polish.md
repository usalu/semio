# w3-overlays-chrome-polish — final report

Implemented all 6 items from WP15+WP16 in `framework/renderer/wgpu/rs/lib.rs`, entirely within `shell::ShellChrome` (one narrow, disclosed exception below), coordinating with the concurrent `w3-prefs-i18n-themes` agent by picking disjoint functions and re-grepping region bounds before every edit.

New sub-region: `//#region 🔖️ChromeOverlaysAndTour` (types, thread-locals, free functions) plus a new `#[cfg(test)] mod chrome_overlays_tour_tests` — both purely additive.

## 1. Tooltips
`chrome_register_tooltip`/`chrome_tooltip_titles_clear` (thread-local `HashMap`, cleared once per frame in `render_chrome`) populated from `render_navbar` (fullscreen/panel-toggle/mode/fixture items) and a new `chrome_register_utility_tooltips` walk over `self.active_utilities` in `render_footer`. `render_chrome_tooltip` arms a hover timer on `input.hovered_id`, paints via `chrome_now_ms()` + a 500ms delay, dismissing immediately on hover-out. Placement uses `ui_wgpu::resolve_overlay_placement` with a scratch `ui_wgpu::UiTree::new()` and `OverlayKind::Tooltip.default_placement()` (`AtPointer`).

## 2. Dialogs
Generic `ChromeDialogRequest{id,title,body,confirm_label,confirm_action,cancel_label}` + thread-local stack. `render_chrome_dialog` paints a scrim + centered box (`OverlayKind::Dialog.default_placement()` == `Centered`); Confirm's hit target carries the real `ActionDescriptor` so it dispatches through the existing pipeline unchanged. **Real call site**: the sync-backbone Detach button in `render_overlay` no longer dispatches `framework.sync`/`detach` directly — it now opens a confirmation dialog first. **This was a real destructive action with zero confirmation before this fix.**

## 3. Introduction tour
Reads `session.app.introduction: Option<IntroductionDefinition>` (was completely ignored before). `chrome_start_introduction`/`chrome_advance_introduction`/`chrome_skip_introduction` (thread-local step index) + `render_chrome_tour` (veil, anchored below Navbar/Footer via `BelowAnchorWithFlip`, centered fallback for other anchor kinds, Next/Skip). Trigger points: (a) a new navbar "help-circle" button shown only when the app declares an introduction; (b) auto-start once per app per session via `read_stored_introduction_seen`/`write_stored_introduction_seen` (functions `w3-prefs-i18n-themes` had already landed, explicitly earmarked for this wiring). Persists "seen" on Skip/Done.

## 4. Cursor polish
Spot-checked: `resolve_semio_cursor` (ui_wgpu, read-only) derives cursor purely from `HitKind`, so correctness reduces to "does the right kind get registered." Floating-panel and measures-rail resize handles already use `HitKind::PanelResize` correctly; every disabled-control site already gates hit registration on `enabled`/`!disabled`. No bug found — documented as the honest outcome rather than inventing changes.

## 5. Ribbon nested collections
Fixed in `render_footer_utility_nodes` (the one deliberate exception, see below): removed a `.filter()` that discarded nested `Collection`s before recursing, capping nesting at one level. Now recurses on the unfiltered `children` (arbitrary depth). Added `utility_subtree_has_active_path`: a collapsed `Collection` now highlights when a pressed `Toggle` lives anywhere in its subtree, not just when expanded.

## 6. Engagement ghost text
`engagement_completion_suffix` ports React's `engagementInlineCompletion`/`engagementCompletionSuffix` (char-boundary-safe). `render_engagement_input` now computes the live query from `input.text_buffer` when focused, draws the dimmed suffix after the current text. **Honest scope-down**: "Tab or Right-arrow accepts" isn't reachable — confirmed by grep that `InputState::pending_keys`/`queue_key` are never populated anywhere in either crate; real key routing happens inline in the off-limits `shell::ShellInput`. Implemented click-to-accept on the ghost-text region instead, documented as a wiring request for whoever next touches `ShellInput`.

## ui_wgpu overlay manager usage
`OverlayKind`/`OverlayAnchor`/`OverlayPlacement`/`resolve_overlay_placement` used for every placement decision. `EventRouter`/`open_overlay`/`OverlayStack` remain `pub(crate)` to `ui_wgpu`, reachable only via `engine::Ui` (a `NodeId`/`UiTree`-based retained system) — `ShellChrome` is immediate-mode with no such tree, so state/dismissal is thread-local instead, same conclusion `w2-text-editor` reached independently.

## Build/test
`cargo check -p semio-framework-renderer-wgpu --lib`: clean, no errors. `cargo test -p semio-framework-renderer-wgpu --lib`: **181 passed, 0 failed** (22 are this agent's — tooltip arm/hover-out, dialog open/nest/close/scrim-dismiss/modality-guard, tour start/advance/skip/past-last-step/no-op, ribbon active-path unit + a 2-level-deep recursion regression test, ghost-text suffix matching/multibyte-safety/accept-on-click). This includes fixing the failure the atlas-fix agent flagged (`render_engagement_input_click_accepts_ghost_completion`) — now green.

## Files touched
- `framework/renderer/wgpu/rs/lib.rs` — `shell::ShellChrome` (new `ChromeOverlaysAndTour` sub-region + test module, edits to `render_chrome`, `render_navbar`, `render_footer`, `render_overlay`, `render_window_engagement_rail`, `render_engagement_input`).
- One disclosed exception outside `ShellChrome`: `render_footer_utility_nodes` — sits in an unnamed gap between `ShellInput` and `ActionPanelAndUtilities` regions; a narrow, surgical bug fix.

## Deliberately avoided (coordination with `w3-prefs-i18n-themes`)
Never touched `shell_chrome_string`, `ChromePrefsState`, `UiPrefsSnapshot`, `load_ui_prefs_once`, theme registry/draft-editor functions, `FilePrefsStore`, or any prefs/i18n/theme region content — only called two of their already-landed, explicitly-earmarked hooks. Did not use new i18n string keys for new labels (Skip/Next/Done/Confirm/Cancel/"Start introduction") since that key list is their region — flagged as a natural follow-up.
