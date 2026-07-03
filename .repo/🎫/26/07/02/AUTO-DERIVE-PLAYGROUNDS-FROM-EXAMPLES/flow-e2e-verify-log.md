# Flow E2E Verify Log

## Final verification (2026-07-02)

```bash
bun run dev:flow   # default port 6016
```

Cold start on `http://127.0.0.1:6047/`:
- title: `semio · flow`
- nav: true, panels: 5, buttons: 22, canvas: 4
- zero Vite `Failed to resolve` errors

## Fixes applied

### Vite config cycle
- `framework/product/playground/dev/vite.config.ts` re-export shim pointed at itself; restored `export { default } from "./js/vite.config.ts"`.

### Relative imports after package layout (`react/index.tsx` beside `rs/pkg/`)
- `flow/react`: `../core/rs/pkg`, `../worker-client.ts`, `../compute.ts`
- `puzzle/2d|3d`, `gis/2d`, `raster`, `writer` react: `../rs/pkg/` (was `../../rs/pkg/`)
- `dag/react`: `../rs/pkg/`
- `forms/react`: `../../procedural/3d/example/...`
- `puzzle/5d/react`: sibling `../../2d`, `../../3d` paths
- Vitest dynamic imports: `../example/`, `../../../compose/`

### LOD wasm boot
- `flow/react`: LOD helpers use `FlowSession.lodScaleJson()` after flow wasm init
- `dag/react`: safe `readDagLodScaleJson()` fallback
- wasm dev stub: `lodScaleJson()` on `FlowSession`, `DagSession`, `BoardSession`

### Vite aliases
- `infinite-world-r3f`, `infinite-cavas-react-renderer` → `index.tsx` (not `js/`)
