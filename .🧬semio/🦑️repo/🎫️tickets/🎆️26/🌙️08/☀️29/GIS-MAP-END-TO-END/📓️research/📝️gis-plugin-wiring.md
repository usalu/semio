# 📝️ GIS Plugin Wiring — Recon

> Source: read-only exploration agent, 2026-08-29. Claims marked ⚠️ are unverified by runtime.

## Plugin root
`✏️s/🔌️plugins/🌍️gis/🦀️component.rs:26-66`
- Plugin id `gis`; registers artifact kinds `gismap` (2D, schema `gis.map`) and `gisterrain` (3D).
- Activation `OnArtifactKind { kind: "2d.map" }`; execution mode `Isolated` (wasm sandbox).
- Capabilities: `documents.write`, `shell.navigate`.
- Build: `📦️packages/🦀️rust/Cargo.toml:58-59`, `crate-type = ["cdylib","rlib"]`, target `wasm32-wasip2`.

## Playground registration (authoritative)
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️playgrounds.ts:53`
```
variant: "gis2d", pluginId: "gis",
cratePath: "✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust",
app: "s.gis.gismap@1/*#editor", aliases: ["gis 2d"],
ports: { react: 6040, wgpu: 6140 },
engines: ["./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust"],
assets: [
  { kind: "tile-proxy", route: "/osm", upstream: "https://tile.openstreetmap.org/{z}/{x}/{y}.png", cache: "osm-tiles" },
  { kind: "tile-proxy", route: "/vt",  upstream: "https://tiles.openfreemap.org/planet",          cache: "openfreemap-vt" },
]
```
Default renderer for `bun ./📜️script.ts dev gis 2d` is **wgpu** (`framework-repo-lib/📦️index.ts:2467`,
`env.SEMIO_RENDERER ?? "wgpu"`), so the no-env default port is **6140**, not 6040.

## MapHost construction (current path — the framework docstring's `🎛️apps/◻2d/` path is stale)
`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗺️maphost/🦀️component.rs:1-62`
`map_host_from(document: &GisMapSnapshot, cfg: &Gis2dConfig) -> MapHost`:
1. `MapHost::new()` (from `framework_surface::tiled_map`)
2. `host.sync_map_json(&gis_map_descriptor_json(document))`  ← full re-sync of all features
3. `host.set_camera(x, y, zoom)` parsed from `cfg.camera_json`
4. `host.set_render_mode / set_vector_style / set_lod_mode`

## Config / presence
`✏️editor/🎚️config/🦀️component.rs:12-40` — `Gis2dConfig`:
`layer_visibility: BTreeMap<String,bool>`, `camera_json: String` (`{"x","y","zoom"}`),
`render_mode` (default `combined`), `vector_style` (default `colored`),
`lod_mode` (default `automatic`), `layer_stroke_scale: BTreeMap<String,f64>`, `locale`.

`✏️editor/👥️presence/🦀️component.rs:14-16` — `Gis2dPresence { camera_json: String }` only.

## View commands (all emit `Gis2dConfigMutation`, never document mutations)
`✏️editor/🎮️commands/👁️view/🦀️component.rs`
`toggle-layer-visibility`, `fit-world`, `camera`, `render-mode`, `vector-style`, `lod-mode`,
`focus-feature`, `layer-stroke-scale`.
Handler shape: `handle(payload, doc: &ArtifactView<GisMapSnapshot>, cfg: &ConfigView<Gis2dConfig>)
-> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault>`.

⚠️ **Suspected hot-path problem**: every camera-mutating command rebuilds a whole `MapHost` from
(doc, cfg) — including `sync_map_json` over all features — then throws it away after reading
`camera_json`. If the interactive pan/zoom path routes through the `camera` command, this is a
full host rebuild per input event. **Must be verified at runtime before acting on it.**

## Document mutation triads (12, all present with diff + inverse + one test vector each)
- positions: `create-position`, `delete-position`, `replace-position-data`, `reorder-positions`
- routes: `create-route`, `delete-route`, `replace-route-data`, `reorder-routes`
- regions: `create-region`, `delete-region`, `replace-region-data`, `reorder-regions`

## Test coverage
`🗺️maphost/🦀️component.rs:38-61` — two tests only:
- `the_host_mirrors_the_document_features_and_the_config_camera`
- `a_malformed_camera_json_leaves_the_host_at_its_own_default`

**Gaps: no zoom test, no pan test, no tile-selection test, no LOD-transition test, no
tile-cache/eviction test, no perf/regression test.** This is the biggest hole for this ticket —
CLAUDE.md requires at least one language-agnostic test per feature.

## Stubs
- `🎮️commands/📌️empty.md` — plugin declares zero *plugin-level* commands (all 12 are window-level).
- `🏔️gisterrain/.../✏️editor/🦀️component.rs` — deliberately unimplemented whole-document replace
  (per semantic-mutations taxonomy), not a defect.
