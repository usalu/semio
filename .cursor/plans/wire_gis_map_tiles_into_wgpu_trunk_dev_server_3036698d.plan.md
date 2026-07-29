---
name: Wire GIS map tiles into wgpu Trunk dev server
overview: Fix the wgpu-native GIS map renderer so it actually paints raster/vector cartography instead of only pins+labels, by giving the Trunk dev server (and native-bin) a path to the same tile cache/proxy that the React/Vite renderer already uses.
todos:
 - id: tile-proxy-server
   content: Add startGisMapTileProxyServer() in ui/styling/vite-elements-assets.ts reusing existing osm/vt Connect middlewares
   status: completed
 - id: port-constant
   content: Add fixed GIS_MAP_WGPU_TILE_PROXY_PORT constant in repo/lib/js/index.ts
   status: completed
 - id: trunk-proxy-config
   content: Add [[proxy]] entries for /osm/ and /vt/ in framework/renderer/wgpu/Trunk.toml
   status: completed
 - id: wire-serve-script
   content: Start tile-proxy server from TrunkServeScript (gis2d only) in framework/renderer/wgpu/script.ts
   status: completed
 - id: wire-native-script
   content: Start tile-proxy server + pass SEMIO_GIS_MAP_TILE_BASE_URL env into NativeRunScript for gis2d
   status: completed
 - id: plugin-absolute-template
   content: "gis/2d/program/rs/lib.rs render_canvas: override tile_url_template/vector_tile_url_template from SEMIO_GIS_MAP_TILE_BASE_URL when set, plus test"
   status: completed
 - id: verify-close
   content: Reopen Map Wgpu Renderer Parity ticket, rebuild/verify tiles render in wgpu dev + run cargo tests, close ticket with summary
   status: completed
isProject: false
---

## Root cause

`GisMapScene` (`framework/core/rs/lib.rs`) defaults `tileUrlTemplate`/`vectorTileUrlTemplate` to same-origin relative paths:

```2242:2248:framework/core/rs/lib.rs
pub fn gis_map_default_tile_url_template() -> String {
    "/osm/{z}/{x}/{y}.png".into()
}

pub fn gis_map_default_vector_tile_url_template() -> String {
    "/vt/{z}/{x}/{y}.pbf".into()
}
```

These paths only exist because the **React/Vite** gis2d playground registers dev middleware for them in [ui/styling/vite-elements-assets.ts](ui/styling/vite-elements-assets.ts) (`createOsmTileMiddleware`/`createVtTileMiddleware`, wired via `osmTileProxyVitePlugin`/`mapLibreVectorTileProxyVitePlugin`), which fetch from `tile.openstreetmap.org` / OpenFreeMap on cache miss and serve from `.repo-cache/`.

The new wgpu-native renderer's dev server is **Trunk** (`framework/renderer/wgpu/script.ts` → `TrunkServeScript`, config in `framework/renderer/wgpu/Trunk.toml`) — a plain static-file Rust server with zero knowledge of `/osm/` or `/vt/`. Every tile request 404s, so `MapHost::tile_images`/`vector_tiles` (`gis/2d/rs/lib.rs`) stay empty forever and `append_tiles`/`append_vector_tiles_colored` never draw anything. Positions/routes/labels still render because they read purely in-memory fixture data, not fetched tiles — exactly the symptom reported ("just the pins with labels").

Native-bin is also broken today, for a different reason: `fetch_map_tile_bytes_blocking`'s `ureq` fetch only accepts absolute `http(s)://` URLs and rejects the relative `/osm/...` template outright.

Verified while investigating:

- `tile.openstreetmap.org` and `tiles.openfreemap.org` both send `access-control-allow-origin: *`, so cross-origin `fetch()` works once a reachable URL exists.
- OpenFreeMap has no stable non-versioned vector tile URL — the real path template must be resolved from its TileJSON (`https://tiles.openfreemap.org/planet`), which `ui/styling/vite-elements-assets.ts` already does. Re-implementing that resolution in Rust would duplicate logic that already exists and is tested; reusing the existing Node-based cache/proxy is the smaller, single-source-of-truth fix.
- Trunk 0.21.14 (installed) supports `[[proxy]] backend = "..."` sections in `Trunk.toml` that forward matching paths to a backend HTTP server.

## Fix

1. **`ui/styling/vite-elements-assets.ts`** — add `startGisMapTileProxyServer(repoRoot: string, port: number, mode?: GisMapTileServeMode): http.Server`, a small `node:http` server chaining the existing `createOsmTileMiddleware`/`createVtTileMiddleware` handlers (already plain `(req,res,next)` Connect handlers) with a final 404. Reuses all existing caching (`.repo-cache/osm-tiles`, `.repo-cache/openfreemap-vt`) and OSM-compliant `User-Agent` logic — no duplicated tile-fetch code.

2. **`repo/lib/js/index.ts`** — add a fixed port constant next to `FRAMEWORK_OS_PLAYGROUND_DEFAULT_PORTS` (e.g. `GIS_MAP_WGPU_TILE_PROXY_PORT = 6141`, the next free slot after `gis2d.wgpu = 6140`).

3. **`framework/renderer/wgpu/Trunk.toml`** — add:

```toml
[[proxy]]
backend = "http://127.0.0.1:6141/osm/"

[[proxy]]
backend = "http://127.0.0.1:6141/vt/"
```

so browser requests to `/osm/...`/`/vt/...` on the Trunk dev server are transparently forwarded to the tile-proxy server. Harmless no-operation for other plugins (route just never gets hit).

4. **`framework/renderer/wgpu/script.ts`**:
   - `TrunkServeScript.run()`: when the resolved program is `"gis2d"`, call `startGisMapTileProxyServer(repoRoot, 6141)` before `spawnSync("trunk", ["serve", ...])`.
   - `NativeRunScript.run()`: when program is `"gis2d"`, also start the tile-proxy server and pass `SEMIO_GIS_MAP_TILE_BASE_URL=http://127.0.0.1:6141` into the env for the `cargo run ... semio-wgpu-native` child process (native has no browser "same origin" to resolve relative paths against).

5. **`gis/2d/program/rs/lib.rs`** (`render_canvas`) — after building the base `GisMapScene`, if `SEMIO_GIS_MAP_TILE_BASE_URL` is set in the process environment, override `scene.tile_url_template`/`scene.vector_tile_url_template` with absolute URLs built from that base (`format!("{base}/osm/{{z}}/{{x}}/{{y}}.png")`, etc.); otherwise keep the relative defaults. `std::env::var` returns `Err` on `wasm32-unknown-unknown` (no env support), so the browser/Trunk build is unaffected and keeps using relative URLs proxied per step 3. This is the one Rust code change; extend the existing test file for `gis/2d/plugin` to cover the override.

## Verification

- Rebuild/serve the wgpu gis2d playground (`bun framework/product/os/dev/script.ts dev` with `SEMIO_PLUGIN=gis2d SEMIO_RENDERER=wgpu`), confirm `/osm/…` and `/vt/…` return 200 through the Trunk proxy, and visually confirm land/water/roads/labels now render (screenshot), not just pins.
- Run existing `gis/2d/plugin` and `gis/2d/rs` cargo tests plus the extended env-override test.
- Update the ticket this work belongs under (reopen the closed Map Wgpu Renderer Parity ticket via `ticket_reopen`, since this is a regression fix on the same feature) with a summary of the tile-proxy wiring and all touched files, then close it again.
