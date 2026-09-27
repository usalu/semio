# Live Window Options Checkpoint

The live React renderer at `http://127.0.0.1:6013/` exposes the Perspective window's Window Options panel through its visible button. The observed panel contains Projection and its Orthographic/Perspective selectors, Field of View, Vortex Show/Direction, LOD, Grid, Select, and Sun groups. The current viewport places the Window Options trigger at x=1152.765625, y=63.9375 with width=117.671875 and height=22.390625. Its first two scoped comboboxes have 104×16 pixel bounds.

The DOM-derived geometry receipt is `🗑️generated/astra-runtime/browser26/react-window-options30.json`. It records only rendered controls from the actual tab panel. Settings and Commands remain open from earlier parity checks, and the dock currently has one Perspective tab. This is an observed React checkpoint, not a WGPU parity result. The fresh WGPU build must finish before the corresponding comparison is made.

The still-loaded earlier WGPU artifact was exercised once through the visible Top Focus control at (55,45), then the same Unfocus control. The accepted accessibility tree changed from Top and Perspective to only Top with Unfocus, then returned to both tabs with Focus. Warning/error console reads were empty. This checks that the older artifact can complete that isolated operation after reload; it does not establish current-source timing, the earlier focus-plus-Tab stall's resolution, or quantitative response latency.

## Fresh React Physical Baseline, 2026-09-27 01:40 Europe/Berlin

Reloading the React reference restored both Top and Perspective. Existing Settings and Command panel preferences survived reload, so both panels were explicitly toggled closed before measuring the base layout. At viewport 1280×720, the Top body rectangle is (6.375, 60.75, 417.09375, 627.265625); Perspective is (433.03125, 60.75, 840.59375, 627.265625). The Top Focus button is (47.640625, 35.171875, 22.390625, 22.390625).

A physical pointer click at (59,46) changes Focus to Unfocus, removes the Perspective tab/body, and expands Top to (6.375,60.75,1267.25,627.265625). The next physical click at the same point restores the two original body rectangles. Console warning/error logs are empty after both transitions. These are current React observations only; a corresponding fresh WGPU artifact is still pending.
