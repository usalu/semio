# Verify Log — Side Panel Tab Bar Clipping

## Root cause

- **React**: `panelChromeFrameLayerClass` sits at `z-30` while the tab strip used `z-20`, so the frame border painted over the first tab's left edge. The strip also lacked horizontal inset padding.
- **WGPU**: Panel border strokes were emitted as backdrop layers after `end_glass_content`, so the overlay pass repainted them on top of tab foreground glyphs. Tabs also started at `panel.x` with no inset scissor.

## Fix

- Added shared `PanelTabBar` for `SidePanel` and `MobilePanel`.
- Raised tab strip to `z-40`, added `px-single` / `scroll-px-single`, and `min-w-0` labels.
- Extracted `render_panel_tab_bar` in WGPU; inset tab row by hairline, scissor to inner width, draw borders inside glass foreground before tabs.

## Checks

- `cargo check` in `framework/renderer/wgpu/rs` — pass
- React vitest — blocked locally (`ui/js/react/script.ts` module resolution); added in-file `PanelTabBar` test
