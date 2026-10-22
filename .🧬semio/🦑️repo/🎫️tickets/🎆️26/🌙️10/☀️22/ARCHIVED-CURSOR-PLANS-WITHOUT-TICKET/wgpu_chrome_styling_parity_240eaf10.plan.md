---
name: Wgpu Chrome Styling Parity
overview: Make the wgpu shell chrome pixel-consistent with the React shell by fixing a colorspace (gamma) bug that darkens every chrome color, restoring the borders that item fills currently overpaint, and aligning the remaining surface/state color mappings (panel glass fill, dock caps, default control fill/text, navbar item order).
todos:
 - id: ticket
   content: Read repo://goals and reopen/open the chrome-parity ticket
   status: completed
 - id: srgb-surface
   content: "Fix colorspace: sRGB swapchain view + pipelines, Srgb icon/raster texture formats, update draw.rs pixel tests"
   status: completed
 - id: group-borders
   content: Fix render_chrome_group draw order so borders/separators survive item fills; dedupe mode-switcher copy
   status: completed
 - id: cap-buttons
   content: Give dock Focus/Close cap buttons a bordered ActionGroup treatment with h-small sizing
   status: completed
 - id: widget-borders
   content: Border the icon-select button in widgets.rs like other controls
   status: completed
 - id: glass-panel
   content: Add glassPanelAlpha token, regenerate, paint wgpu panels as 0.58-alpha glass over canvas
   status: completed
 - id: mapping-fixes
   content: Align dock cap fill to window, default control fill/text to transparent+gray, navbar Fullscreen order, verify footer color
   status: completed
 - id: verify
   content: cargo tests, wasm rebuild, wgpu E2E with pixel-parity assertions vs React screenshots
   status: completed
isProject: false
---

# Wgpu Chrome Styling Parity

Both renderers already share one token source (`ui/styling/tokens.json` → generated `CHROME_LIGHT/DARK` in [ui/styling/rs/generated.rs](ui/styling/rs/generated.rs) for wgpu, hand-synced CSS vars in `ui/styling/js/ui.css` for React). The visible divergence comes from three verified defects, not from missing tokens.

## Root cause 1: colorspace bug — every wgpu chrome color renders too dark

Pixel-sampled proof from the E2E screenshots: wgpu navbar renders `#d4ceb1`, which is exactly the _linear-encoded_ bytes of the correct token `light-6-7` `#ebe8d9` that React shows. The theme constants are linearized (`rgba8_to_linear`), but the swapchain is non-sRGB on the web (WebGPU canvases only expose non-sRGB formats, so the `is_srgb()` probe in [ui/wgpu/rs/gpu.rs](ui/wgpu/rs/gpu.rs) lines 52–58 falls through), so linear values are written out as raw sRGB bytes.

Fix in `ui/wgpu/rs/gpu.rs` + `ui/wgpu/rs/draw.rs`:

- Configure the surface with `view_formats: vec![format.add_srgb_suffix()]`, create the per-frame texture view with the sRGB format, and build `UiPipelines` against that sRGB target format. GPU then encodes linear→sRGB on write and the navbar comes out `#ebe8d9` exactly.
- Switch textures that hold sRGB pixel data to `*Srgb` formats so they aren't double-brightened: icon atlas (`draw.rs:1012`) and the canvas2d raster store (`draw.rs:746`). Glyph atlas (`R8Unorm` alpha mask) stays.
- Fix the existing readback pixel tests in `draw.rs` (~line 1889+) for the new encoding, and run `cargo test -p ui_wgpu`.

## Root cause 2: buttons/toggles have no visible borders

- `render_chrome_group` ([framework/renderer/wgpu/rs/shell.rs](framework/renderer/wgpu/rs/shell.rs) lines 1858–1932): `chrome_group_border` draws the group border first, then each item's full-height background rect **overpaints all four border hairlines and the separators**. Fix the draw order: paint item fills first (inset by `stroke_hairline` from the group edge), then separators, then the 4-edge border last. Same fix for the inlined mode-switcher copy at lines 2498–2546 (deduplicate it into `render_chrome_group`).
- `render_cap_button` in [framework/renderer/wgpu/rs/dock.rs](framework/renderer/wgpu/rs/dock.rs) (lines 629–668, Focus/Close) draws a bare fill with no border. React renders these as an `ActionGroup`: one bordered container with `divide-x` separators, height `h-small` (16px). Wrap both cap buttons in a single bordered group using the shared `push_control_border`/group pattern and `h-small` sizing.
- Icon-select button in [ui/wgpu/rs/widgets.rs](ui/wgpu/rs/widgets.rs) line 643 uses a borderless `push_rounded`; give it the same `push_control_border` treatment as Button/Toggle.

## Root cause 3: mapping mismatches vs React semantics

From the React shell audit (`ui/js/react/index.tsx`, `ui/styling/js/ui.css`):

- **Side panel fill is frosted glass, not opaque**: React uses `ui-glass-panel` = panel color at `--glass-panel-alpha: 0.58` over the canvas (sampled `#d8d5c6`), while wgpu paints opaque `theme.panel` `#c9c8bd`. Add a `glassPanelAlpha: 0.58` token to `tokens.json` `opacities` (regenerate; keep `ui.css` value identical) and paint the wgpu panel body with `theme.panel.with_alpha(glass alpha)`.
- **Dock window cap**: React `windowCapFrameClass` is `bg-window`; wgpu uses `theme.panel` (dock.rs ~525). Change cap fill to the window color.
- **Default control state**: React chrome items are transparent with gray `text-element` text, hover = gray fill + foreground text, active = primary fill. Wgpu (`item_bg`/`item_text` in [ui/wgpu/rs/chrome.rs](ui/wgpu/rs/chrome.rs) and `render_chrome_group`) uses window fill + foreground text by default. Align: default bg transparent (let the surface show through), default text `text_muted`-gray, hover/active unchanged.
- **Navbar right-side order**: React ends with Fullscreen at the far right (…Settings | Fullscreen); wgpu places Fullscreen left of the toggle group (`render_navbar`, shell.rs ~2580–2600). Reorder to match.
- **Footer**: `render_footer` (shell.rs 2479+) already uses `theme.navbar` (window) like React, but the sampled screenshot shows canvas color — verify after rebuild and fix whatever draws over it if the discrepancy persists.

## Verification

- `cargo test` for `ui_wgpu`, `ui_styling`, and the wgpu renderer crate; rebuild the wasm bundles.
- Re-run the existing wgpu E2E (`.repo/🎫️/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts`) and pixel-assert chrome parity against the React screenshots: navbar `#ebe8d9`, canvas `#f0ecdd`, glass panel ≈`#d8d5c6`, footer `#ebe8d9` (light theme), plus border-presence samples on the toggle group and dock cap buttons. Side-by-side visual check on at least flow + one 3D app.
- Repo workflow: read `repo://goals`, reopen the existing chrome-parity ticket (or open a new one), keep all temp scripts/screenshots inside the ticket folder.
