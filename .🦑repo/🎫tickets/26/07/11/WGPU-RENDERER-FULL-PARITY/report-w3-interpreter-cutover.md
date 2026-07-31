# w3-interpreter-cutover — final report (THE pivotal architectural cutover)

File touched: `framework/renderer/wgpu/rs/lib.rs` (`interpreter` region, full rewrite of `render_ui_node`, plus a deliberate 2-line exception in `shell::ShellChrome`, recorded in `region-claims.json`).

## Cutover design

- **Per-window `Ui` lifecycle**: one process-wide `ui_wgpu::Ui` instance in a `thread_local!` (`UI_ENGINE`), mirroring this region's pre-existing `UI_IMAGE_FETCH_QUEUE`-style statics. `Ui` itself already partitions state per `window_id` internally (`HashMap<window_id, UiWindow>`), so one shared instance is correct. Avoided touching `shell::ShellTypes` (an Integrator choke point) entirely.
- **`window_id: &str` — a new, required parameter on `render_ui_node`**. Unavoidable: no existing way to derive stable per-window identity from anything already flowing into the function. Its two real call sites (`render_window_content`, `render_floating_panel`, both in `shell::ShellChrome`) got the smallest possible diff: appending the one identifier each already has in scope. Recorded explicitly in `region-claims.json`.
- **Frame flow**: `apply_tree(window_id, node)` → `set_viewport` → synthesize pointer events from the current `InputState` aggregate → `frame(window_id, w, h)` → composite the returned `&DrawList` into the caller's live `ctx.draw` (same submission path immediate mode used) with a manual translate-by-`(bounds.x, bounds.y)` copy. `UiCommand::App{action,..}` results push via `ctx.input.queue_event(action)` — confirmed this is the exact existing `drain_events()`→`app.dispatch_actions()` pipeline every other action source uses.
- **Event routing**: pointer-only synthesis (Move/Down/Up/Scroll) implemented inside `render_ui_node`. Deliberately did NOT drain keyboard here — that's a single shared, non-window-scoped queue needing focus bookkeeping that lives in `shell` (sibling `w3-shell-input-cutover` agent's job). Exposed `pub fn dispatch_ui_event(window_id, event, input)` as the sanctioned hook for that agent.
- **`SceneHost` — deliberately not implemented**: `scene_slots::SceneSlot` only carries `{surface_id, kind, rect}`, never the full `UiComponentSceneNode` payload `render_component_scene` needs, and the trait doesn't cover `Image`/`ExternalSlot` at all. Instead, `paint_unbridged_scene_and_image_leaves` walks the ORIGINAL `UiNode` with the pre-existing immediate-mode layout math purely to resolve `ComponentScene`/`Image` bounds, then calls the UNCHANGED `render_component_scene`/`render_ui_image` directly — real scene/image content keeps working, layered after the retained composite.

## What stayed the same vs changed
- Unchanged: `validate_ui_node`/`RENDER_PLAN_LIMITS` gate, `measure_ui_node`, `ui_node_to_widget` + helpers, `render_ui_node_inner` (kept fully intact, now dormant, `#[allow(dead_code)]` per the "don't delete" instruction), `render_ui_image`/image-loading machinery, `framework_widget_context`.
- Changed: `render_ui_node` gained the `window_id` parameter, entire body now drives `ui_wgpu::Ui` instead of `ui_node_to_widget`+`render_widget`. Two call sites outside `interpreter` got a one-token addition each.

## CRITICAL FINDING — biggest open risk, needs a follow-up fix before the Wave 4 sweep
`ui_wgpu::Ui::new()` owns a PRIVATE `FontAtlas`/`Option<IconAtlas>` with no injection API (`IconAtlas` doesn't even derive `Clone`). `GpuContext::upload_font_atlas`/`upload_icon_atlas` do a full, non-incremental `write_texture` of the ENTIRE atlas. Uploading both the shell's real `Shaped`-mode atlas and the retained engine's own `Bitmap`-mode atlas to the same GPU texture would non-deterministically clobber one or the other every frame.

**Decision made**: never call `gpu.upload_font_atlas`/`upload_icon_atlas` for the retained engine's atlas/icons — protects existing chrome/dock/panel text (unchanged, correct) at the cost of: **retained-mode text glyphs and icons will likely render blank or with wrong UVs** until `ui_wgpu` gains a way to share/inject an atlas.

**Wiring request (top priority)**: either derive `Clone` for `IconAtlas`, or thread `&mut FontAtlas` through `Ui::apply_tree`/`frame` instead of `Ui` owning one, mirroring how `flex::LayoutEngine::compute`/`paint::paint_tree` already take it as a parameter internally. **This should be fixed before Wave 4's visual sweep runs, or the sweep will show broad text/icon regressions unrelated to the 6 documented per-kind gaps.**

## The 6 known-gap kinds — code-level assessment (not visually spot-checked)
No display/runtime environment was available to launch the winit app in the sandbox — this is analysis, not visual confirmation:
- **Field, Section**: the shadow-walk for scene/image children under a `Field` uses the immediate layout math, so a scene/image nested in a `Field` may be positioned slightly differently than retained-painted siblings.
- **NumberStepper**: untouched, routes entirely through retained paint.
- **Image, ComponentScene**: render real content via the shadow-walk (unchanged functions). **ExternalSlot**: falls back to the retained placeholder (acceptable, real slots pre-resolved upstream).

## Build/test verification
- `cargo check -p semio-framework-renderer-wgpu --lib`: clean, 0 errors, 0 warnings attributable to this code. Hit repeated transient, unrelated compile breaks from concurrent sibling agents (`apply_os_command`, `fuzzy_match_score`, `CHROME_TOUR_AUTO_CONSIDERED`, a `Theme.scrim` field, `app_now_ms`, several `E0753`s) — waited these out, none referenced `interpreter`.
- `cargo test -p semio-framework-renderer-wgpu --lib`: **147 passed, 0 failed** (confirmed twice, stable). The higher count vs. the 121 baseline reflects other concurrent agents' new tests, not this agent's — all of `interpreter::render_plan_validator_tests::*` pass unchanged. No tests added or removed (nothing tested `ui_node_to_widget`/immediate-path implementation details directly).

## Wiring requests filed
1. `ui_wgpu`: add `#[derive(Clone)]` to `IconAtlas`, and/or a way for `Ui` to borrow a caller-owned `FontAtlas` instead of owning one — resolves the text/icon rendering gap above. **TOP PRIORITY.**
2. `shell::ShellInput` (`w3-shell-input-cutover`): call `pub fn dispatch_ui_event(window_id, event, input)` in `interpreter` for real keyboard/IME/focus-scoped event routing.
3. `UiCommand::FocusChanged`/`OverlayClosed`/`DropCommitted`/`DropCancelled`/`Clipboard*` are received by `apply_ui_commands` but not acted on beyond `App` — documented no-op (clipboard OS access is a pre-existing `host`-region gap).

## Files touched
- `framework/renderer/wgpu/rs/lib.rs` — `interpreter` region (new `RetainedEngineCutover` sub-region: `render_ui_node` rewrite, `dispatch_ui_event`, `apply_ui_commands`, `dispatch_pointer_events`, `pointer_button_from_code`, `composite_retained_draw_list` + shift helpers, `paint_unbridged_scene_and_image_leaves`); plus one line each in `render_floating_panel` and `render_window_content` (`shell::ShellChrome`).
- `.repo/🎫/26/07/11/WGPU-RENDERER-FULL-PARITY/region-claims.json` — recorded the 2-line `shell::ShellChrome` exception.

Did not touch `dock`, `engine_canvas`, `plugin_bridge`, `scenes`, or any other `shell` sub-region. Did not delete `widgets`/`ui_node_to_widget`/`render_ui_node_inner`.
