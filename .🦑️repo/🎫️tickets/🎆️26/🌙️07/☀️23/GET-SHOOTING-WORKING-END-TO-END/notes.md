# Get Shooting Working End to End

## Root cause

`shooting/example/base-icon.shooting.json` references `/mesh/base.glb`.
`base.glb` lives at `asset/metabolism/representation/base.glb`.
Puzzle declares `[[package.metadata.semio.assets]]` `mesh-collection` for `/mesh`; shooting declared none.
Vite SPA fallback returned `index.html` for `/mesh/base.glb` → GLTFLoader JSON parse error
(`Unexpected token '<', "<!doctype "...`).

## Fix

1. Add mesh-collection asset row to `shooting/plugin/rs/Cargo.toml` (same roots/placeholder as puzzle3d).
2. Regenerate plugin registry so `PLAYGROUND_BUILD_TARGETS` for `shooting` includes the asset.
3. Extend standalone `startAssetServer` to serve mesh-collection + static-dir (wgpu Trunk/native).
4. Proxy `/mesh/`, `/infinite-fixture/`, `/cad-fixture/` in Trunk.toml to the asset server.
5. Native wgpu GLB fetch resolves absolute paths via `SEMIO_ASSET_BASE_URL`.

## Verification

- `curl -I http://127.0.0.1:6019/mesh/base.glb` → `200` + `model/gltf-binary` + `glTF` magic
- Playwright `verify-shooting-mesh-e2e.ts`: canvas present, no render error, chrome shows Scene + Icon
- `SEMIO_TEST_LEVEL=quick bun ./📜️script.ts test` in `ui/styling` → 54 passed
- `bun test ./ui/styling/js/index.test.ts` → 19 passed
