## Fix Panel Toggle Chip Jump Over Fullscreen

Chrome-hosted panel toggles (inspector / `top-right`) now unfold without shifting the fullscreen button or parking the chip on top of it.

### Changes

- `PanelChromeTabBar`: when `visible`, renders a non-interactive width placeholder (`data-panel-chrome-tab-bar-placeholder`) sized from the last folded measurement instead of unmounting.
- `Panel`: open chrome-hosted right anchors pass `capRowStyle` with `shellNavbarTrailingEndReserveCss` on `WindowChrome`.
- `WindowChrome`: optional `capRowStyle` on the cap row.
- Tests: placeholder width, cap reserve, updated ghost and open-on-press expectations.

### Files

- `🧰️framework/🔨️module/🖱️ui/⚛️react/⚡️implementation/🟦️typescript/📦️index.tsx`
