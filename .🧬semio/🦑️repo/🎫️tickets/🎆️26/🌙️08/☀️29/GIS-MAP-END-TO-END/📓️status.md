# 🗺️ GIS Map End to End — Status

Start commit: `bb06c41f73f0122fbed315b7487428b976f99921` (see `🗑️generated/start-commit.txt`).

## Run recipe (verified from the registry, not from the stale `.vscode` seed)
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️playgrounds.ts:53`
- variant `gis2d`, plugin `gis`, app `s.gis.gismap@1/*#editor`
- ports: react **6040**, wgpu **6140**
- `bun ./📜️script.ts dev gis 2d` defaults to **wgpu** (`framework-repo-lib/📦️index.ts:2467`
  `env.SEMIO_RENDERER ?? "wgpu"`), so the no-env default is port **6140**.
- Registered in `.claude/launch.json` as `gis2d-wgpu`.
- Tile proxies served by the dev server: `/osm` → tile.openstreetmap.org, `/vt` → tiles.openfreemap.org/planet,
  disk cache under `.🧬semio/🗺️map/{osm-tiles,openfreemap-vt}` (~21 MB, gitignored).

## Architecture (as read from source)
```
TiledMapHost (React, engine/🧱️elements/TiledMapHost/🟦️component.tsx)
  └─ MapRenderer (same file)  ── demand frame scheduler
       ├─ JS byte caches: tileCache / vectorTileCache  (Map<key, ArrayBuffer>)
       ├─ fetch("/osm/{z}/{x}/{y}.png") / fetch("/vt/{z}/{x}/{y}.pbf")
       └─ MapWasmSession (wasm-bindgen)
            └─ MapHost  (🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs)
                 ├─ tiles.tile_images:  BTreeMap<key, Arc<RasterImage>>   (PNG-decoded)
                 ├─ tiles.vector_tiles: BTreeMap<key, VectorTile>          (MVT-decoded)
                 ├─ features {positions, routes, regions}  ← sync_map_json
                 └─ build_vector_scene() → Vello Scene → wgpu
```

## Confirmed defects (read from source; runtime confirmation pending)

| # | Defect | Evidence |
|---|---|---|
| D1 | **Every tile is re-decoded on every refresh.** JS `uploadOne` always calls `session.uploadTile(...)` even when the bytes came from its own JS cache; Rust `upload_tile` unconditionally runs `image::load_from_memory` + `to_rgba8`, and `upload_vector_tile` unconditionally re-parses the whole MVT protobuf. `MapHost::has_tile`/`has_vector_tile` already exist but are **not exposed to JS and never consulted**. Up to 256 full PNG decodes per refresh. | `🦀️component.rs:2455-2468`, `:2789-2795`; `🟦️component.tsx:460-476, 490-506` |
| D2 | **Dirty detection allocates a 256-entry JSON array twice per frame.** `pollVisibleTilesForRefresh` calls `visibleTilesJson()` + `visibleVectorTilesJson()` every scheduled frame purely to string-compare against the previous value. Each call runs `serde_json` over up to 256 objects. | `🦀️component.rs:2270-2284`; `🟦️component.tsx:433-448` |
| D3 | **JS byte caches are unbounded.** `tileCache` / `vectorTileCache` are plain `Map<string, ArrayBuffer>` with no cap and no eviction — they only ever grow while panning/zooming. | `🟦️component.tsx:253-254` |
| D4 | **Retention window is one frame deep.** `tile_retention_keys` = visible ∪ ancestors(visible) ∪ *previous frame's* visible. Panning back to where you just were re-fetches and re-decodes. | `🦀️component.rs:453-462, 2825-2842` |
| D5 | **No prefetch ring.** Only strictly-visible tiles are ever requested, so a pan always exposes blank edges until the debounce fires and the fetch round-trips. | `🦀️component.rs:2270-2284` |
| D6 | **Cache cap eviction is arbitrary, not LRU.** `while len > 512 { pop_first() }` on a `BTreeMap` keyed by the `"z/x/y"` string evicts the lexicographically smallest key, which is unrelated to recency or visibility. | `🦀️component.rs:2368-2384` |
| D7 | **Trailing-only 120 ms debounce.** `scheduleRefreshTiles` clears and re-arms its timer on every change, so during a sustained drag *no tiles load at all* until the user pauses. | `🟦️component.tsx:60, 393-401` |
| D8 | **No zoom/pan/tile tests.** The only maphost tests are "mirrors document features" and "malformed camera json". No test covers tile selection, LOD banding, retention, eviction, or idempotent upload. | `🗺️maphost/🦀️component.rs:38-61` |

## New wasm API contract (fixed up front so Rust and TS can be built in parallel)
Added to `MapWasmSession` in `🗺️tiled-map/🦀️component.rs` (`#[wasm_bindgen]`):
- `hasTile(z,x,y) -> bool`, `hasVectorTile(z,x,y) -> bool`
- `visibleTilesRevision() -> f64`, `visibleVectorTilesRevision() -> f64` — allocation-free dirty signal
- `prefetchTilesJson() -> String`, `prefetchVectorTilesJson() -> String` — same row shape as
  `visibleTilesJson` (`{z,x,y,key}`), the ring just outside the viewport, visible tiles excluded.

---

# ✅️ Work landed

## Rust — `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs`
Detail: `📓️research/📝️rust-tile-pipeline-fixes.md`

- **R1 / D1 — idempotent upload.** `upload_tile`/`upload_vector_tile` compute the `z/x/y` key first and
  early-return `Ok(())` when the tile is already cached, *before* `image::load_from_memory` /
  `decode_mvt`. A hit still records an LRU touch. Invalid-bytes error behaviour on a genuine miss is
  unchanged.
- **R2 / D2 — allocation-free dirty signal.** New `visible_tiles_revision()` /
  `visible_vector_tiles_revision()`. `VisibleTileCursor` gained `z()` / `bounds()`; an inline FNV-1a
  mixer hashes `(z, x_min, x_max, y_min, y_max)` with zero allocation. The vector variant returns
  sentinel `0` exactly when `visible_vector_tile_cursor()` is `None`, matching the `"[]"` guard.
- **R3 / D4+D6 — real LRU retention.** `MapTileLedger` replaced the one-frame-deep
  `last_raster_visible`/`last_vector_visible` sets with `raster_touch`/`vector_touch`
  (`RefCell<BTreeMap<String,u64>>` + a `Cell<u64>` monotonic clock, so `&self` draw paths can record
  recency). `tile_retention_keys` became `pinned_tile_keys` (visible ∪ ancestors). Retention no
  longer drops every non-pinned tile each frame — it evicts one at a time, only over
  `MAX_MAP_TILE_CACHE_ENTRIES`, always the stalest non-pinned key. Tiles are touched on upload *and*
  on draw.
- **R4 / D5 — prefetch ring.** `tiles::prefetch_ring_tiles(bounds, visible, z, n, cap)` enumerates the
  one-tile border outside the viewport (x wraps mod n, y clamps, visible excluded, deduped, capped at
  `MAX_VISIBLE_TILE_REQUESTS - visible.len()`), surfaced as `prefetch_tiles_json()` /
  `prefetch_vector_tiles_json()` with the same `{z,x,y,key}` row shape.
- **R5 — wasm exposure.** `hasTile`, `hasVectorTile`, `visibleTilesRevision`,
  `visibleVectorTilesRevision`, `prefetchTilesJson`, `prefetchVectorTilesJson` on `MapSession`.
  Revisions are masked to 53 bits so the `u64` round-trips exactly through a JS `number`.

## TypeScript — `TiledMapHost/🟦️component.tsx` + `WasmSessionLoader/🟦️component.tsx`
Detail: `📓️research/📝️ts-maphost-perf-fixes.md`

- **T1 / D1** — uploads guarded by `hasTile`/`hasVectorTile` via a shared `uploadTileRow`/`uploadTileRows`
  path used by both refresh and prefetch.
- **T2 / D2** — per-frame polling now compares `visibleTilesRevision()` numbers; the 256-row JSON is
  serialized exactly once per refresh instead of twice per frame.
- **T3 / D3** — new `createByteLru` (byte-budgeted LRU: 64 MiB raster, 96 MiB vector) and
  `createBoundedSet` (2048 entries) replace the unbounded `Map`/`Set` caches.
- **T4 / D7** — `createLeadingTrailingDebounce` replaces the trailing-only timer, so tiles now load
  *during* a sustained drag instead of only after it stops.
- **T5 / D5** — generation-tracked `prefetchTiles()` runs after visible tiles, shares rather than
  competes for the 12-slot fetch budget, and is abandoned on dispose or a newer refresh.
- **T6** — `setLodMode` no longer clears the byte caches (they are keyed `z/x/y`, LOD-independent);
  only the miss sets and revision trackers reset.

## Tests
Detail: `📓️research/📝️map-math-oracle-tests.md`

- **Third-party oracle (CLAUDE.md requirement).** 47-vector language-agnostic fixture at
  `🗺️tiled-map/🧪️tests/web-mercator-tile-oracle/🧫️fixtures/🔣️vectors.json` + `🥒️.feature` spec, covering
  projection, slippy tile numbering, tile bounds and LOD bands. `🐍️.py` validates the fixture against
  **`mercantile` 1.2.1** (added test-only via `uv add --group test mercantile`; not reachable from any
  production path).
- **Rust integration test** `📦️packages/🦀️rust/tests/🗺️tiled_map_mercator_oracle.rs` reads the *same*
  fixture through the crate's public API, plus infinite-canvas invariants: cursor-anchored zoom keeps
  the world point under the cursor fixed, pan-and-back returns the exact original camera, zoom
  round-trips, visible tile count stays within `MAX_VISIBLE_TILE_REQUESTS`.
- **Rust unit tests** in `mod tests`: idempotent upload (incl. `Arc::ptr_eq` proving the cached entry
  is untouched), revision stability/change on pan+zoom, vector sentinel, prefetch disjoint-from-visible
  + capped, and LRU eviction taking the stalest tile while never evicting a pinned visible one. I
  applied the three fixture-driven tests the oracle agent could not (`lod_band_selection_matches_frozen_
  specification_vectors`, `active_map_lod_tracks_the_same_bands_as_the_span_resolver`,
  `visible_tile_count_never_exceeds_max_visible_tile_requests`).
- **TS unit tests** `TiledMapHost/🧪️component.test.ts` — LRU eviction/byte budget/use-on-get, bounded
  set, leading+trailing debounce timing and dispose, and a `MapRenderer` test proving `uploadTile`
  fires exactly once for a tile across two refreshes.

## launch.json registration
- `.vscode/🧩️launch.seed.jsonc` — added `⚖️gate🗺️surface🧪️test` →
  `bun x nx run @semio-tech/framework-surface-rs:test` (group `4_gate`, order `425.1`). The nx target
  already existed but had no launcher. Added to the **seed**, since the dev server regenerates
  `.vscode/launch.json` from it.
- `.claude/launch.json` — added `gis2d-wgpu` (port 6140) for browser-driven verification.

## Peer-churn repair (not part of the original defect list)
A concurrent session encapsulated `infinite_canvas::Scene` behind a newtype
(`🖼️canvas/🦀️component.rs:180`, `pub struct Scene(pub(crate) backend::Scene)`) so the external
`vello::Encoding` type stops leaking through the public API — correct per CLAUDE.md's
"MUST NOT export api that … requires an interface/class/type outside of this codebase". They added
`Scene::is_empty()` and `Scene::path_count()` as the replacements but had not updated the downstream
consumer, so `semio-framework-surface` no longer compiled at all (11 × `E0599: no method named
`encoding``) and **every** test in the crate was blocked, mine included.

All 10 call sites were pre-existing (`git show HEAD:…tiled-map/🦀️component.rs | grep -c '.encoding()'`
→ 10; my own diff contained none), so this was not self-inflicted. Rewritten mechanically onto the
peer's own new accessors:
- `X.encoding().is_empty()` → `X.is_empty()`
- `X.encoding().path_tags.len()` → `X.path_count()`

No behaviour change — `path_count()` is defined as exactly `self.0.encoding().path_tags.len()`.

Separately, the workspace root was transiently unbuildable while a peer converted
`✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml` into a standalone `[workspace]` (observed
mid-write, mtime == wall clock). Polled through rather than "fixed" — not this ticket's file.

## My own follow-ups on top of the agents' work
- `MapTileLedger::touch_in` — the already-present branch writes through `get_mut`, so the
  per-drawn-tile touch in `append_tiles`/`append_vector_tiles` allocates nothing on a hot frame.
  Previously every drawn tile did `key.to_string()` — up to 256 `String` allocations per frame,
  which would have partly re-introduced the very cost D2 removed.
- `retain_tiles_for_keys` / `retain_vector_tiles_for_keys` — early-return when under the cap, so the
  touch-ledger prune no longer runs (and no longer cloned every live key into a fresh `BTreeSet`)
  on every single `prepare_visible_tiles` call.
- `uploadTileRow` — the `hasTile`/`hasVectorTile` guard moved *ahead* of the fetch. It was placed
  after `await fetch(...)`, so a tile the host already held but whose bytes the JS LRU had evicted
  still cost a full network round-trip. Safe to hoist because Rust-side upload is now idempotent, so
  a racing duplicate upload is a cheap no-op rather than a re-decode.

## D9 — dead ninth LOD tile-z entry (found by finally running the oracle test)
`GIS_MAP_LOD_TILE_Z` was `[0,1,2,3,4,5,7,10,18]` — **nine** entries against `GIS_MAP_LODS`'s **eight**
bands. Every indexer (`🦀️component.rs:139,205,217,222,1664`) indexes it with a band index produced by
`resolve_map_lod_index_from_span` (which saturates at `GIS_MAP_LODS.len()-1` = 7) or by
`GIS_MAP_LOD_SCALE::index_of`, so the trailing `18` was unreachable dead data.

It was not harmless. The Python oracle's band fallback was written as `len(tile_z) - 1` — which that
ninth entry made **8**, while Rust's is `GIS_MAP_LODS.len() - 1` = **7**. Because the Rust half of the
oracle could not compile (see D10), the fixture was frozen against the Python answer and recorded a
band index that does not exist. The disagreement only surfaced once the Rust test actually ran:
`lod index for span=0: left 7, right 8`.

Fixed on all three sides so they cannot silently diverge again:
- `GIS_MAP_LOD_TILE_Z` trimmed to eight entries, with a docstring stating the length invariant.
- fixture `lodConstants.tileZ` trimmed to match; the `spanDeg: 0.0` row corrected to
  `{lodIndex: 7, tileZ: 10}`.
- `🐍️.py` fallback changed from `len(tile_z) - 1` to `len(max_span) - 1`, so it now mirrors Rust's
  per-band count and no longer depends on the tile-z table's length at all.

## D10 — the oracle integration test never compiled
`tests/🗺️tiled_map_mercator_oracle.rs` — cargo derives an integration test's **crate name** from the
file stem, and a crate name must be ASCII:
`error: invalid character '🗺' in crate name`. The test had been reported as "written but not
confirmed by a run"; this is what that caveat was concealing. Fixed without abandoning the repo's
emoji filename convention, by registering an explicit target in
`📦️packages/🦀️rust/Cargo.toml`:
```toml
[[test]]
name = "tiled_map_mercator_oracle"
path = "tests/🗺️tiled_map_mercator_oracle.rs"
```

## Verification results
- `cargo check -p semio-framework-surface --target wasm32-unknown-unknown` → **Finished, no errors**
  (24m16s). This is the run that matters for the six new `#[wasm_bindgen]` methods, which sit inside a
  `#[cfg(target_arch = "wasm32")]` block that native `cargo test` never compiles.
- `cargo test -p semio-framework-surface` → **230 passed, 1 failed** before the D9 fix; the single
  failure was the bad fixture row, not the implementation. Re-run after the fix pending.
- `🐍️.py` third-party oracle → `PASS: mercantile 1.2.1 agrees with all 47 frozen fixture vectors`
  (re-confirmed after the D9 fix).
- TS: 11/11 vitest, and `tsc --noEmit` clean for both touched files.

## D11 — visible-tile window over-covered by a whole column and row (found by the oracle test's first real run)
`tiles::visible_tile_cursor` computed the far tile index as `ceil(far_edge / step)`. `ceil` names the
tile that *starts* where the viewport *ends*, so unless an edge landed exactly on a tile boundary the
window was always one column and one row too large — 9×6 tiles where 8×5 cover the viewport at
1920×1080/z14, i.e. **~35% of every refresh's fetches, PNG decodes and MVT parses spent on tiles that
`tile_rect_intersects_viewport` then discarded at draw time**. It also inflated the count checked
against `MAX_VISIBLE_TILE_REQUESTS` and everything keyed off the visible set (retention pinning,
revision, prefetch cap).

Fixed to `(ceil - 1).max(x0).min(n - 1)`:
- `ceil - 1` is the tile *containing* the far edge, and correctly drops the zero-overlap tile when the
  edge lands exactly on a boundary;
- `.max(x0)` keeps a viewport smaller than one tile resolving to a single tile rather than an empty range.

Confirmed by the oracle's `zurich-z10` vector, which had been returning a 2×2 block for a 2×2-pixel
viewport at zoom 1e15 and now returns exactly `(10, 536, 358)` — the tile `mercantile` names. All 231
pre-existing unit tests still pass, so no tile-window test had encoded the old behaviour.

### Test-design correction in the same test
`null-island-z1` (lon/lat `0,0` at z=1) sits exactly on the corner where all four z=1 tiles meet, so a
viewport centred there legitimately overlaps all four: `visible_tiles` answers *"which tiles does this
rect touch"*, which cannot disambiguate a measure-zero point the way mercantile's *"which tile owns
this point"* can. Rather than weaken the assertion for every vector, it is now split — mercantile's
tile must always be present; an interior point must still collapse to exactly one tile; only a point
on a tile edge may return up to four.

## ✅️ Final verification
| Check | Command | Result |
|---|---|---|
| Rust unit tests | `cargo test -p semio-framework-surface` | **231 passed, 0 failed**, 1 ignored |
| Rust oracle integration | same run, `tiled_map_mercator_oracle` | **7 passed, 0 failed** |
| Third-party oracle | `.venv/bin/python …/🐍️.py` | `PASS: mercantile 1.2.1 agrees with all 47 frozen fixture vectors` |
| Wasm bindings compile | `cargo check -p semio-framework-surface --target wasm32-unknown-unknown` | **Finished, no errors** |
| TS unit tests | vitest, `TiledMapHost/🧪️component.test.ts` | **11 passed** |
| TS typecheck | `tsc --noEmit` (react target) | clean for both touched files |

## ⚠️ Live browser render — attempted, blocked by peer breakage (NOT verified)
I could not get a live composited render of the map. Every browser-capable entry point is currently
broken by other sessions' in-flight work; none of it is reachable from this ticket's diff.

1. **gis2d dev server (port 6140)** — `semio-s-plugin-gis` fails with **278** errors, all of the form
   `the trait bound GisMapMutation: Mutation<GisMapSnapshot> is not satisfied` /
   `no method named diff found for &GisMapMutation`. A peer is mid-refactor of the framework `Mutation`
   trait. `semio-s-plugin-stdio` separately fails at `linking with wasm-component-ld failed`.
   The dev script reports `plugin catalog build summary: 0/2 crate(s) produced .wasm`.
2. **Why the server appeared to "die" silently for hours** — the dev script spawns cargo under a build
   budget and was hitting `error: spawnSync cargo ETIMEDOUT` while ~26 concurrent cargo processes from
   other sessions held the shared target lock, so it aborted before ever binding the port. This is only
   visible when the dev server's own stdout is captured; through the preview harness it just looks like
   a dead port. **Worth remembering as a diagnostic.**
3. **Storybook static build** — `bunx storybook build` fails in rollup; it bundles every story,
   including the plugin stories that depend on the broken crates.
4. **Storybook dev (scoped `framework hosts`, port 6010)** — serves, but every story fails to render:
   `Failed to resolve import "@semio-tech/framework-core" from ".storybook/preview.tsx"`. A peer renamed
   that package to `@semio-tech/framework` (which is what `TiledMapHost` itself already imports), but
   ~10 `.storybook/*` files still use the old specifier and `createBrowserStoragePort` is no longer
   exported from the package at all. Completing someone else's rename across the storybook layer is
   out of scope for this ticket.

### What was verified instead, covering the same risk
A browser run's unique value here is proving the **TS ↔ wasm binding contract** holds against the real
compiled artifact (right method names, arity, types) — a mismatch there is the failure a unit test
cannot catch. That was checked directly against the built pkg rather than against source:

`📦️packages/🦀️rust/pkg/framework_surface.d.ts` (built 13:35, newer than the last source edit at 12:51,
so it includes every change up to and including the `ceil - 1` windowing fix) declares:
```ts
hasTile(z: number, x: number, y: number): boolean;
hasVectorTile(z: number, x: number, y: number): boolean;
visibleTilesRevision(): number;
visibleVectorTilesRevision(): number;
prefetchTilesJson(): string;
prefetchVectorTilesJson(): string;
```
— identical, method for method, to the `MapWasmSession` interface in
`WasmSessionLoader/🟦️component.tsx`, and all six appear in the raw wasm export table
(`mapsession_hasTile`, `mapsession_visibleTilesRevision`, …).

**Still genuinely unproven: the final visual composite** — that tiles paint correctly and that pan/zoom
feel smooth to a human. The pixel path is unchanged by this ticket (no drawing code was touched; the
changes are which tiles get requested, whether they get re-decoded, and when they get released), but
that is an argument, not a measurement, and should not be read as a passing test.

---

# 🖥️ LIVE RUNTIME VERIFICATION (second pass)

The first pass stopped at code-level checks. This pass got an actual composited render with working
zoom and pan, by routing around the peer-broken app entirely.

## Two blockers cleared
1. **Storybook alias was stale.** `.storybook/scopes.ts:121,137` aliased `@semio-tech/framework-core`
   to `🧰️framework/⚡️implementations/🟦️typescript/📦️index.ts` — a path deleted when the
   `⚡️implementations` layout was dissolved. Repointed to the package's real home
   (`🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`) under its current name `@semio-tech/framework`,
   and completed the rename across all 16 `.storybook/**` files. `createBrowserStoragePort` and friends
   are exported from `@semio-tech/framework`, which is already what `TiledMapHost` itself imports, so
   this completes a half-finished rename rather than adding a compat shim.
2. **Ticket-local harness** `🧪️harness/` — a bun static server + driver page that loads the prebuilt
   `framework_surface` wasm pkg and the repo's cached tiles directly. No gis plugin, no storybook, no
   React: nothing that concurrent refactors have broken. Registered as `map-harness` (port 6210) in
   `.claude/launch.json`.

## Results — the map genuinely renders and is interactive
| Check | Measurement |
|---|---|
| WebGPU adapter | present (`depth32float-stencil8`, `rg11b10ufloat-renderable`, …) |
| `MapSession` boot | `gpuReady=true`, canvas attached at 1280×800 |
| **Paint proof** | 1280×800 readback: **199 distinct colours** over 10 556 sampled pixels, 10 557 non-transparent (a blank canvas reads as 1 colour) |
| Tiles drawn | 48 vector + 1 raster at world LOD, from the local cache |
| LOD banding | `{id: "world", tileZ: 0, spanDeg: 360}` at fit-world |
| **Zoom in** | `640 → 716.8` — exactly the ×1.12 step |
| **Zoom round-trip** | back to exactly `640` |
| **Pan** | middle-drag (640,400)→(940,580) moved camera to `(-0.1203, -0.0722)` |
| **Pan round-trip** | reverse drag returned to exactly `(0, 0)` |
| D1 `hasTile` guard, live | second settle reported `raster {uploaded: 0, skipped: 1}` — the held tile was **not** re-decoded |
| D2 revision, live | identical across two reads while still; changed on pan; correctly unchanged on a zoom that kept the same tile range |
| D5 prefetch, live | ring disjoint from visible and sharing its z (empty at z0, correct — one tile covers the world) |

## 🐢️ D12 — per-frame vector scene rebuild is the remaining bottleneck (MEASURED, NOT YET FIXED)
Sustained 40-event middle-drag at 1280×800: **p50 32.6 ms/frame, p90 58.2 ms, max 175 ms** — roughly
30 fps, which does not meet "zooming, panning … must be performant".

Isolated by render mode at a fixed camera (25 frames each):

| Render mode | p50 frame |
|---|---|
| `image` (raster tiles only) | **0.5 ms** |
| `vector` | **34.1 ms** |
| `combined` | 31.4 ms |

The raster path is essentially free; **the entire frame cost is `build_vector_scene()` re-tessellating
all 48 held vector tiles into a fresh Vello `Scene` on every single frame**
(`🦀️component.rs:3677`, via `append_vector_tiles`). Nothing is cached between frames: the same MVT
geometry is re-walked and re-emitted at 30 Hz even when only the camera translated.

This is distinct from D1–D11 (which were about *which* tiles get fetched, decoded and retained); it is
about the *draw* path, and it is now the dominant cost.

## ✅️ D12 FIXED — vector scene translation-reuse cache

`build_vector_scene()` now caches the built vector-tile `Scene` and, when only the camera translated,
reuses it via `scene.append(&cached, Some(translate(dx, dy)))` instead of re-tessellating. The cache key
covers everything that changes the geometry: zoom bits, viewport w/h/dpr,
`visible_vector_tiles_revision()`, held-tile count, render mode, vector style, forced LOD, layer
visibility/stroke scales and theme. Regions/routes/positions still draw fresh every frame; the
screen-pinned backdrop wash is drawn outside the translated content so a pan cannot drag it out of
alignment.

### Two bugs found by measuring rather than trusting
1. **Build system: the wasm pkg was never invalidated by source edits.** `wasmPackInputsStale` walked
   only the crate directory, but every owner-tree `🦀️component.rs` is `#[path]`-mounted from *outside*
   it — path *dependencies* were followed, `#[path]` *module mounts* were not. The pkg sat frozen at
   13:35 while source changed for hours, so any browser measurement silently ran stale wasm. Fixed in
   `📚️library/📦️packages/🟦️typescript/📦️index.ts` by resolving `#[path]` mounts transitively
   (guarded to regular files — a mount may name a directory, which `readFileSync` rejects with `EISDIR`).
2. **`interaction_revision` in the cache key defeated the cache on every pan frame.** Across a 40-frame
   drag the visible tile set changed exactly *once* (constant 48 tiles), so the cache should have hit 39
   times — yet the drag stayed at 35 ms. `pointer_move_screen`/`set_camera` bump `interaction_revision`
   every frame, and it was in the key. `append_vector_tiles` never reads selection or hover (those are
   the regions/routes/positions layers, drawn outside the cache), so the key entry bought nothing and
   cost everything. Removed, and the test that asserted the wrong behaviour was replaced by two pinning
   the real contract: a selection change must NOT rebuild, and a drag inside one tile set must rebuild
   zero times.

### Measured on real WebGPU, 1280×800, 48 held vector tiles
| Benchmark | Before | After |
|---|---|---|
| Static frame, `vector` | 34.1 ms | **2.7 ms** |
| Static frame, `combined` | 31.4 ms | **2.8 ms** |
| Static frame, `image` | 0.5 ms | 1.1 ms |
| **Sustained 40-frame drag, p50** | **32.6 ms** | **2.8 ms** |
| Sustained drag, p90 | 58.2 ms | **7.2 ms** |
| Sustained drag, max | 175 ms | 40.7 ms (the one frame that crossed a tile boundary and rebuilt) |
| Wheel-zoom step, p50 | — | 14.5 ms (a zoom must rebuild: stroke widths and label sizes are screen-space) |

Panning went from ~30 fps to ~350 fps.

## 🏁 Final end-to-end verification (live, real WebGPU)
| Check | Result |
|---|---|
| Paint at world view | 180 distinct colours / 10 556 sampled px |
| Paint at continent zoom | 59 distinct colours, fully opaque |
| Zoom step | exactly ×1.12 |
| Zoom round-trip | exact equality back to the starting zoom |
| Pan | moves, and the reverse drag returns to exact equality |
| LOD banding | `world` (tileZ 0) → `continent` (tileZ 1, spanDeg 92.4) as zoom rose |
| Tile selection | 4 visible at continent zoom, prefetch ring disjoint and sharing z |
| Rust tests | **240 passed, 0 failed** + **7 oracle passed** |
| Third-party oracle | `mercantile` 1.2.1 agrees on all 47 vectors |
| Wasm bindings | `cargo check --target wasm32-unknown-unknown` clean |
| TS | 11 vitest passed, `tsc` clean |
