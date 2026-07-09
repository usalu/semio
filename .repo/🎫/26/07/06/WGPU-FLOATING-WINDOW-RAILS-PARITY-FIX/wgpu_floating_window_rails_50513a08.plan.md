---
name: Wgpu Floating Window Rails
overview: Fix the wgpu command window (engagement) and window options (measures) rails so they are true glassy floating overlays on top of window content — matching the pre-migration React behavior — instead of docked panels that squeeze the window body, with matching auto-height and token-derived widths.
todos:
 - id: reopen-ticket
   content: Reopen WGPU-GLASS-PANEL-CRISPNESS-FIX ticket via repo mcp
   status: completed
 - id: stop-squeeze
   content: Stop rails from mutating/shrinking content rect; reorder render + hit registration so content renders first and rails float on top with edge inset
   status: completed
 - id: token-widths
   content: Replace hardcoded 240/280 width constants and 160/640 resize clamp with ui/styling token-derived Theme fields; compute engagement width against measures reserve
   status: completed
 - id: auto-height
   content: Add measures/engagement content height helpers and size rails to fit content (capped by window height) instead of full-height stretch
   status: completed
 - id: fix-stacking-bug
   content: Fix sibling/child y-threading bug in render_window_measure so measures stack vertically instead of overlapping
   status: completed
 - id: verify
   content: Run cargo tests, rebuild wasm, and visually verify floating/glass/z-order behavior
   status: in_progress
isProject: false
---

# Wgpu Floating Window Rails Parity Fix

## Root cause

In [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs), `render_window_measures_rail` (~9653-9787) and `render_window_engagement_rail` (~9936-10102) both **mutate the shared `content: &mut Rect`** after drawing their glass rail:

```9784:9786:framework/renderer/wgpu/rs/lib.rs
        draw.end_glass_content();
        content.w -= width + theme.gap_standard;
        None
```

```10098:10101:framework/renderer/wgpu/rs/lib.rs
        draw.end_glass_content();
        content.x += rail_w + theme.gap_standard;
        content.w -= rail_w + theme.gap_standard;
        None
```

This carves out space from the window body before `render_window_content` runs (call site ~9111-9153), turning them into **docked sidebars that squeeze content** instead of floating overlays. It also registers the rails' hit targets _before_ window content's hits, so — once we stop squeezing — content would win hit-tests in the overlap region (`InputState::hit_at` in `ui/wgpu/rs/lib.rs` picks the **last**-registered match via `.iter().rev().find(...)`).

In the React source of truth ([ui/js/react/index.tsx](ui/js/react/index.tsx) `Window` component, ~15503-15613), `children` (window body) renders first, and `window-measures-overlay`/`window-engagement-overlay` render **after**, as `absolute` siblings (`z-panel`) that never resize the body — confirmed by `windowMeasuresOverlayClass = "... absolute top-0 right-0 z-panel ..."` and `windowEngagementOverlayClass = "... absolute top-0 left-0 z-panel ..."` (~4179-4203). Side panels already render on wgpu's separate `overlay` draw list and composite after the main `draw` list's glass, so the "below side panel" z-order is already correct once rails stop being docked — no change needed there.

## Fix 1 — Stop squeezing content, float on top

- Change `render_window_measures_rail` / `render_window_engagement_rail` signatures: `content: &mut Rect` → `content: &Rect`; delete the trailing mutation lines.
- At the call site (~9111-9153), reorder to render window content **first** (full, unmodified `content` rect), then measures rail, then engagement rail — mirroring React's DOM order so their `register_hit` calls win the overlap region.
- Anchor rails with a `theme.gap_standard` inset from the window edges (matches React's `p-single` padding on the overlay wrapper, which is literally the same token: `--spacing-single: calc(1 * var(--ui-spacing))` = `GAP_STANDARD_UI_SPACING`), instead of flush-docking to the edge.

## Fix 2 — Token-derived widths (drop hardcoded 240/280 constants)

`ui/styling/tokens.json` / `ui/styling/rs/generated.rs` already define the real values React uses:

- `LAYOUT_PANEL_RAIL_UI_SPACING` (70 → 224px) — measures default width (React `windowMeasuresDefaultWidthPx`)
- `LAYOUT_PANEL_MIN_UI_SPACING` / `LAYOUT_PANEL_MAX_UI_SPACING` (150px / 480px) — already exposed as `theme.panel_min_width` / `theme.panel_max_width` in `ui/wgpu/rs/lib.rs` (~5232-5233), currently only used by side panels
- `LAYOUT_ENGAGEMENT_MAX_UI_SPACING` (140 → 448px) — engagement cap (React's `min(28rem, ...)`)

Changes:

- Add `window_measures_default_width` and `window_engagement_max_width` fields to `Theme` in `ui/wgpu/rs/lib.rs`, computed via the existing `chrome_px()` helper, replacing `DEFAULT_MEASURES_RAIL_WIDTH`/`DEFAULT_ENGAGEMENT_RAIL_WIDTH` constants in `framework/renderer/wgpu/rs/lib.rs`.
- Reuse `theme.panel_min_width`/`theme.panel_max_width` for the measures rail's resize clamp — currently hardcoded to `(160.0, 640.0)` at line ~6807 (`shell.measures.resize.` drag handler), which doesn't match React's 150/480 at all.
- Engagement width = `theme.window_engagement_max_width` capped by available room: `min(engagement_max_width, content.w - gap*2 - measures_reserve)`, where `measures_reserve` is the actual width just occupied by the measures rail/chip (mirrors React's measured `measuresReservePx`). Have `render_window_measures_rail` return its occupied width alongside its existing folded-chip hit info so `render_window_engagement_rail` can consume it.
- Also cap both rails' width by `content.w - inset*2` (React's `maxWidth: calc(100% - 0.5rem)`), so a narrow window never overflows a rail past its edges.

## Fix 3 — Auto-height cards instead of full-height rails

React's rail stack is `h-auto max-h-full` (~4184-4187) — a card that hugs its content height, capped by the window body height — not a full-height sidebar. Today both rails hardcode `rail.h = content.h`.

- Add height-measuring helpers in `framework/renderer/wgpu/rs/lib.rs` mirroring the existing width-measuring pattern (`measure_chrome_group_item`):
  - `measure_window_measure_height(theme, collapsed_sections, measure: &WindowMeasure) -> f32` — recursive: `Group` = `control_height` + (open children summed), `Select`/`Slider` = `16.0 + control_height`, `Toggle` = `control_height`.
  - An engagement content height helper mirroring `render_window_engagement_rail`'s own spacing math (options rows, input block, control, status rows, possible-engagement rows).
- Use these to size `rail.h = (header + 2*gap + content_height).min(content.h)` for both rails instead of always `content.h`.
- **Required side-fix for correct height math**: `render_window_measures_rail`'s top-level loop (line ~9760) and `render_window_measure`'s `Group` children loop (line ~9833) currently render every sibling at the _same_ `y` (no advancement between iterations) — a pre-existing stacking bug that must be fixed to thread a running `y` cursor (advanced by each measure's height from the new helper) so siblings actually stack vertically like React's flex column, and so the auto-height sum is accurate. Also add the missing `y += theme.control_height` after the `control` block in the engagement renderer (~10066), which today isn't followed by an advance before `status`/`possible_engagements` render.

## Verification

- `cargo test -p semio_framework_wgpu -p ui_wgpu` (or the workspace's existing test invocation) to confirm the glass-token parity tests (`ui/wgpu/rs/lib.rs` ~5297-5352) and any new coverage still pass.
- Rebuild wasm target and do a visual check (existing ticket has a Playwright-based `wgpu-e2e-verify.mjs` precedent) that: rails now float over window content with visible blur/frost, content is not squeezed, rails are sized to content, and side panels still render above them.

## Ticket

Reopen `.repo/🎫/26/07/06/WGPU-GLASS-PANEL-CRISPNESS-FIX` (closed today, directly covers "window measures/engagement rails" glass migration) rather than opening a new ticket, per repo convention for continuing directly-related work.
