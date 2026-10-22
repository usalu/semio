---
name: Wgpu Chrome Styling Parity
overview: Bring the wgpu OS-shell chrome (navbar, footer, panels, buttons, hover/borders) to visual parity with the React shell by extending the existing `ui/styling/tokens.json` -> codegen pipeline with a new "chrome" theme/metrics group, then rewriting `ui/wgpu/rs/theme.rs` to consume the generated constants (light + dark, with real theme switching) instead of hand-authored gray/blue values.
todos:
 - id: tokens-json
   content: Add chrome theme colors, chrome/typography metrics, chrome strokes+radii to ui/styling/tokens.json
   status: completed
 - id: codegen-script
   content: Generalize theme-group loop in ui/styling/rs/script.ts to include chrome; regenerate artifacts
   status: completed
 - id: styling-lib-rs
   content: Add ThemeName::chrome() accessor in ui/styling/rs/lib.rs; extend tests for chrome light/dark parity
   status: completed
 - id: wgpu-theme-rs
   content: Rebuild ui/wgpu/rs/theme.rs Theme struct from generated chrome constants; add Theme::light()
   status: completed
 - id: theme-switching
   content: Wire theme_id -> resolve_theme() (incl. system prefers-color-scheme) into AppRuntime.frame() in framework/renderer/wgpu/rs/lib.rs
   status: completed
 - id: shell-widgets-apply
   content: Apply corrected radius/borders/hover/accent/spacing/typography across shell.rs and widgets.rs; add navbar/footer hover-border emphasis
   status: completed
 - id: verify-chrome
   content: Run cargo tests, rebuild wasm, run 25-plugin E2E suite, visually verify navbar/footer/panel/button parity and theme switching
   status: completed
isProject: false
---

# Wgpu Chrome Styling Parity

## Root cause

`ui/wgpu/rs/theme.rs` is a **hand-authored, standalone** `Theme::dark()` — never sourced from `ui/styling/tokens.json`. Meanwhile React's chrome (navbar/footer/panels/buttons) gets its tokens from `ui/styling/js/ui.css`, which is **also hand-authored** (unlike the board/map/canvas node-graph themes, which already flow through `tokens.json` → `ui/styling/rs/generated.rs` → `ThemeName::board()/map()/canvas()`). The two chrome systems drifted independently:

| Aspect               | React (`ui.css`)                                  | wgpu (`theme.rs`)                                                                                                                                                                                                                                      |
| -------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Accent               | primary red `#ff344f`                             | blue `#598CF2`                                                                                                                                                                                                                                         |
| Chrome radius        | `0` everywhere                                    | `6px`                                                                                                                                                                                                                                                  |
| Navbar/footer height | `size-large` = 9 × 0.2rem ≈ 28.8px                | `40px` / `36px`                                                                                                                                                                                                                                        |
| Control height       | `size-medium` = 7 × 0.2rem ≈ 22.4px               | `28px`                                                                                                                                                                                                                                                 |
| Gap/padding          | `spacing-single` = 0.2rem ≈ 3.2px                 | `8px` / `12px`                                                                                                                                                                                                                                         |
| Surfaces             | level-based `base/canvas/window/panel` warm grays | flat `#1C1C1F`-ish                                                                                                                                                                                                                                     |
| Hover                | gray fill + border emphasis                       | lighter gray fill only                                                                                                                                                                                                                                 |
| Theme switching      | full light/dark/system via `.dark` class          | `theme_id` stored (`"system"`/`"light"`/`"dark"` in [shell.rs](framework/renderer/wgpu/rs/shell.rs) line 116/1015/1569) but **never applied** — `AppRuntime.theme` is always `Theme::default()` ([lib.rs](framework/renderer/wgpu/rs/lib.rs) line 272) |

## Approach: extend the existing single-source-of-truth pipeline

`ui/styling/tokens.json` already has a fully generic mechanism for exactly this: `themes.<light|dark>.<group>` (paint refs resolved via token/hex/mix) and `metrics.<section>` (arbitrary nested numeric groups), both looped over generically by [ui/styling/script.ts](ui/styling/script.ts) `emitRust`/`emitPython`/`emitTypeScript` — currently only used for `board`/`map`/`canvas`. Adding a `"chrome"` group needs almost zero script.ts logic changes.

### 1. `ui/styling/tokens.json` — add chrome data

- `themes.light.chrome` / `themes.dark.chrome`: resolve via existing `PaintRef` (`token`/`hex`/`mix`) mechanism, mirroring `ui.css` `:root`/`.dark` values:
  `base, canvas, window, panel, foreground, muted_foreground, accent, accent_foreground, active_base, active_foreground, active_hover` (mix `["primary","black",0.9]`), `hover_base, hover_canvas, hover_window, hover_panel, hover_overlay, border_normal, border_emphasized, border_element, overlay_bg` (temporary color, ~opaque since wgpu has no backdrop-blur).
- `metrics.chrome` (ui-spacing multiples, same convention as existing `metrics.dom`): `navbar_height_ui_spacing: 9`, `footer_height_ui_spacing: 9`, `control_height_ui_spacing: 7`, `panel_header_height_ui_spacing: 7`, `gap_standard_ui_spacing: 1`, `padding_standard_ui_spacing: 1`, `panel_inset_ui_spacing: 1`, `ui_spacing_compact_px: 3.2` (resolved `0.2rem` at 16px root — single derivation point).
- `metrics.typography`: `text_2xs_px: 9.6, text_xs_px: 11.2, text_sm_px: 12.8, text_base_px: 14.4, text_lg_px: 16` (compact scale from `ui.css` `@theme inline`, resolved at 16px root).
- `strokes`: add `chrome_border_hairline: 1.0, chrome_border_default: 2.0, chrome_border_focus: 3.0` (generic group, zero script changes needed).
- `radii`: add `chrome: 0.0` (generic group, zero script changes needed).
- Reuse existing `metrics.dom.LAYOUT_PANEL_MIN_UI_SPACING`/`MAX` for panel min/max width — no duplication needed.

### 2. `ui/styling/rs/script.ts` — one small generalization

In `emitRust`/`emitPython`/`emitTypeScript`, the hardcoded `["board", "map", "canvas"]` theme-group arrays become `Object.keys(resolvedThemes.light ?? {})` (or add `"chrome"` explicitly) so `ChromeTheme` struct + `CHROME_LIGHT`/`CHROME_DARK` constants fall out automatically, matching the existing `BoardTheme`/`MapTheme`/`CanvasTheme` pattern in [ui/styling/rs/generated.rs](ui/styling/rs/generated.rs). Run `bun ./ui/styling/script.ts generate` (registered as `@semio-tech/ui-styling-tokens:generate`) to regenerate `generated.rs`/`.ts`/`.py`/CSS.

### 3. `ui/styling/rs/lib.rs` — expose chrome theme accessor

Add `ThemeName::chrome(self) -> &'static ChromeTheme` alongside the existing `board()/map()/canvas()` in the `theme` module ([ui/styling/rs/lib.rs](ui/styling/rs/lib.rs) lines 43-63).

### 4. `ui/wgpu/rs/theme.rs` — rebuild `Theme` from generated constants

Replace all hand-authored RGBA/metric literals in `Theme::dark()` with values derived from `ui_styling::{CHROME_LIGHT, CHROME_DARK}`, `ui_styling::metrics::{chrome, typography}`, `ui_styling::strokes`, `ui_styling::radii`. Add `Theme::light()`. Compute px metrics as `ui_spacing_compact_px * N_ui_spacing` (matching the React formula exactly). Map wgpu's existing field names onto the correct semantic level per the OS shell's actual `LevelProvider` usage (root/navbar/footer = `window`; canvas content = `canvas`; floating panels = `panel`) — confirmed via [framework/renderer/react/os-shell.tsx](framework/renderer/react/os-shell.tsx) line 1590 (`<LevelProvider level="window">`). Keep `background`→canvas-level color, `navbar`→window-level color, `panel`→panel-level color. Replace the alpha-blended `selected` (blue 35%) with `active_base` solid for tabs/toggles (matching CSS's solid active-state fill); keep a lighter alpha variant only where a translucent selection tint is actually needed.

### 5. Wire up real theme switching

- `framework/renderer/wgpu/rs/lib.rs`: add a `resolve_theme(theme_id: &str) -> Theme` helper — `"light"` → `Theme::light()`, `"dark"` → `Theme::dark()`, `"system"`/other → resolve via `window().match_media("(prefers-color-scheme: dark)")`, falling back to dark. Call it each `frame()` (line ~41) using `self.shell.theme_id`, assigning the result to `self.theme` before drawing (cheap string compare + occasional JS call, no caching complexity needed).
- No other changes needed to the existing theme-select dropdown in [shell.rs](framework/renderer/wgpu/rs/shell.rs) (lines 1555-1580) — it already writes `"system"/"light"/"dark"` into `theme_id`.

### 6. Apply corrected theme in shell/widgets rendering

- **Radius**: `push_rounded(..., theme.border_radius)` call sites need no changes — `border_radius` becoming `0.0` automatically makes every chrome element sharp-cornered.
- **Borders**: keep the low-risk axis-aligned approach — draw real 1px hairline lines with `push_solid` for navbar bottom-border / footer top-border / button outlines using `theme.border_normal`, and keep the existing nested-inset-quad technique for the floating-panel frame (now with 1px inset + `border_normal` color instead of the old flat `panel_border`) — this avoids widening the shader/`push_rounded` API surface.
- **Hover emphasis**: add navbar/footer container-level hover detection (`ctx.input` pointer bounds against the bar rect) to swap `border_normal` → `border_emphasized` on the 1px border line, matching CSS `[data-slot="navbar"]:hover`. Add hover feedback to nav icon buttons that currently lack it (flagged gap in [shell.rs](framework/renderer/wgpu/rs/shell.rs) around line 976/1079).
- **Accent**: replace remaining ad-hoc blue accent usage in shell/scene chrome (not canvas board/map/dag content, which already correctly uses red via existing `BOARD_LIGHT`/`DARK` theme) with `theme.active_base`/`theme.accent`.
- **Heights/spacing**: `navbar_height`/`footer_height`/`control_height`/`gap_standard`/`padding_standard` now come from the theme and will shrink significantly (e.g. 40px→28.8px, 8px→3.2px) — after rebuilding, visually re-check hardcoded per-widget paddings in [ui/wgpu/rs/widgets.rs](ui/wgpu/rs/widgets.rs) (e.g. the `8.0` control inner text padding, `4.0` dropdown row radius which should become `0.0`, row heights) and tighten any that now look disproportionate against the new tighter scale.
- **Typography**: `font_size_body`/`font_size_small` sourced from `typography::TEXT_SM_PX`/`TEXT_XS_PX` (12.8/11.2 instead of 13/11); replace the "+1 for emphasized" hack with `typography::TEXT_BASE_PX` (14.4) explicitly.

## Verification

1. `bun ./ui/styling/script.ts generate` then `cargo test -p ui_styling` (existing light/dark parity tests in [ui/styling/rs/lib.rs](ui/styling/rs/lib.rs) lines 76-112 — extend with a chrome-specific assertion, e.g. `CHROME_LIGHT.base != CHROME_DARK.base`).
2. `cargo test -p ui_wgpu`.
3. Rebuild wasm: `bun ./framework/renderer/wgpu/script.ts wasm`.
4. Re-run the 25-plugin E2E suite (`.repo/🎫️/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts`) to confirm no regressions from the earlier empty-screen fix.
5. Visually diff regenerated screenshots against the React shell for navbar/footer height, sharp corners, red accent, and hover borders; toggle the theme selector (System/Light/Dark) and confirm the canvas actually re-themes.

## Todos
