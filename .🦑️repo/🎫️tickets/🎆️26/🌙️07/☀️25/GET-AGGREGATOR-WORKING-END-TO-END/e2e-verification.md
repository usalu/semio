# Aggregator End-To-End After Product/OS Restructure (2026-07-30)

## Result

`bun run dev:mit-bestand:aggregator` boots on `http://127.0.0.1:6023/` without SKIP flags.

## Evidence

- Title: `Entwerfen mit Bestand · Aggregator`
- Canvases: 2
- Example: `Abbau Aufbau`
- GLB: `/mesh/hexagonal-cut-concrete-forest-left.glb` → 200, 86112 bytes, `model/gltf-binary`
- Screenshot: `aggregator-viewport.png`
- Verifier: `verify-aggregator-e2e.json` (`ok: true`)

## Fixes In This Pass

1. Plugin guest WIT path `../../wit` → `../wit` (`framework/product/os/module/plugin/rs`)
2. OS Vite `repoRoot` depth corrected (was resolving to `/Users/ueli/Documents`)
3. Stale playground engine/asset crate paths updated to `framework/module/surface/*` and `framework/product/os/module/*`
4. Dev `globals.css` `@import`/`@source` paths updated for nested `dev/js` layout
5. Plugin registry regenerated
