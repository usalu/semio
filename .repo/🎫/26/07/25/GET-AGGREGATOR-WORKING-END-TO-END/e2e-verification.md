# Aggregator End-To-End After Restructure (2026-07-30)

## Result

`bun run dev:mit-bestand:aggregator` boots the React Aggregator on `http://127.0.0.1:6023/` with brand chrome, puzzle3d scene, and Abbau Aufbau mesh.

## Evidence

- Title: `Entwerfen mit Bestand · Aggregator`
- Canvases: 2 (Top + Perspective)
- Example label: `Abbau Aufbau`
- GLB: `/mesh/hexagonal-cut-concrete-forest-left.glb` → 200, 86112 bytes, `model/gltf-binary`
- Screenshot: `aggregator-viewport.png` (Perspective pane shows the seeded 3D object)
- Verifier: `verify-aggregator-e2e.json` (`ok: true`)

## Breakage Chain Fixed

1. Empty / broken `node_modules` after KEEP rename leftovers → restored package names + `bun install`
2. Dev routed to compose-desktop / forwarded `aggregator` as a Vite path → framework-os-dev + consume variant before Vite
3. Missing flow-core / surface wasm pkgs → unconditional flow-core build + wasm stub plugin in OS Vite config
4. Broken CSS `@import`/`@source` after `framework/` move
5. Missing `@testing-library/react` (vitest imports scanned by Vite)
6. `program` ReferenceError in renderer `onAction` after incomplete rename
7. Mesh roots still pointed at deleted `framework/asset/abbau-aufbau` → `mit-bestand/asset/abbau-aufbau`
