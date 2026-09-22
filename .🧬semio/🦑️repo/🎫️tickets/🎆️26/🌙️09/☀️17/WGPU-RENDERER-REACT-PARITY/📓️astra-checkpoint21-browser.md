# Checkpoint 21 Browser Verification

This checkpoint is in progress. WASM21 is still compiling. No new WGPU browser acceptance is claimed yet.

The previous temporary tabs had been cleaned up, so the React reference was reopened through the existing in-app browser connection at port 6313. All browser input uses the in-app browser. Evidence is stored under `🗑️generated/astra-runtime/checkpoint21-iab`.

## React Reference Results

- At 1600×300, the Orthographic Select in Top Window Options exposes seven options. A physical wheel delta of 34 moved its viewport from scrollTop 29 to 42 (maximum). The browser reports clientHeight 143 and scrollHeight 185. Normal 1600×1000 popup geometry is recorded separately and revealed the additional WGPU sizing differences assigned to SolLocale.
- At 1600×1000, opening the same popup, dismissing it with a scene click, then immediately clicking the Top cap once removed Top and preserved Perspective. The popup count fell to zero before the cap click. No retry was needed.
- Closing Perspective removed the final tab and application surface. In Display → Windows, a single row click selected the Puzzle 3D template; dragging its visible handle into the empty dock reopened one Puzzle 3D tab and one application surface. The console error list remained empty after mounting.

These are reference-side observations, not paired acceptance. The full 57-step journey, Dock8 and surface-family matrix remain open.
