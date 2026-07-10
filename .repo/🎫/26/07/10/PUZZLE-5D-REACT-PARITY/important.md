# Important

- The 5d document format uses `source`/`target` on fasteners (premigration naming); the Nakagin fixture never parsed while the plugin expected `attracting`/`attracted`. The plugin struct now follows the fixture; only the puzzle-3d engine bridge maps to `attracting`/`attracted`.
- The 5d `kindCatalogs` bundle (`parts/grips/fasteners/ropes` with `grips` templates carrying paired `2d`+`3d` aspects) must be projected per consumer: board wants `nodes/handles/edges/wires`, the puzzle-3d engine wants `objects` with `vortices` templates.
- Engine fixture adoption must merge, not replace: keep existing parts' flat aspects, synthesize flat centers for new parts by walking fasteners from placed neighbors (grip `2d.angle` + radii + gap).
- `Puzzle2dBoardScene` gained `hovered_id`/`active_tool` (this ticket) so plugins can drive board hover highlight (paired 3d→2d hover) and board-native tool gestures; the react board session exposes `setHoveredIdSilent`/`setActiveTool`.
- The in-app browser suspends WebGL contexts when backgrounded ("THREE.WebGLRenderer: Context Lost") and reports a 0×0 viewport until `resize_window` runs — both look like rendering bugs but are environment artifacts.
- Vite does not hot-swap the plugin wasm: rebuild (`cargo build -p puzzle-plugin --target wasm32-wasip2 --release`), restart the os/dev server (it re-transpiles into `framework/product/os/dev/plugin-modules/puzzle/`), then hard-refresh.
