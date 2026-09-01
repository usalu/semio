# TiledMapHost / WasmSessionLoader perf fixes (T1–T6)

Files touched (only the ones this agent owns):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/TiledMapHost/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/WasmSessionLoader/🟦️component.tsx` (only the `MapWasmSession` interface)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/TiledMapHost/🧪️component.test.ts` (new)

## MapWasmSession contract (T0)
Added to the interface, matching the fixed Rust contract: `hasTile`, `hasVectorTile`,
`visibleTilesRevision`, `visibleVectorTilesRevision`, `prefetchTilesJson`, `prefetchVectorTilesJson`.

## T1 — `hasTile`/`hasVectorTile`-guarded upload
`MapRenderer.uploadTileRow(kind, row, abandoned)` now checks `session.hasTile`/`hasVectorTile`
right before calling `uploadTile`/`uploadVectorTile`, regardless of whether the bytes came from the
JS cache or a fresh fetch, and skips the wasm call when already present. Fetch-on-miss path is
unchanged (still guarded by the miss set). `refreshRasterTiles`/`refreshVectorTiles` and the new
prefetch path (T5) share this one method — raster and vector logic used to be duplicated, now
parametrized by `kind: "raster" | "vector"`.

## T2 — revision counters replace JSON string comparisons
`pollVisibleTilesForRefresh` now calls `session.visibleTilesRevision()` /
`visibleVectorTilesRevision()` (plain numbers) instead of `visibleTilesJson()` /
`visibleVectorTilesJson()`. The `lastPolled*VisibleKey` string fields became
`lastPolledRasterVisibleRevision` / `lastPolledVectorVisibleRevision` (numbers, seeded to `NaN` so
the first poll always triggers). `refreshRasterTiles`/`refreshVectorTiles` also switched their
miss-set-clearing check from the old `lastRasterVisibleKey`/`lastVectorVisibleKey` JSON-string
compare to `lastRasterVisibleRevision`/`lastVectorVisibleRevision`; the JSON string itself is still
read exactly once per refresh (needed for the actual rows) via `visibleTilesJson()`/
`visibleVectorTilesJson()`.

## T3 — bounded caches
Two new exported pure primitives in a new `TileCachePrimitives` region:
- `createByteLru(maxBytes)` — insertion-order `Map<string, ArrayBuffer>`; `get`/`set` both
  re-insert the key (moves it to MRU position), `set` evicts from the front until `maxBytes` is
  respected. Named budgets: `MAX_RASTER_TILE_BYTES_CACHED = 64 MiB`,
  `MAX_VECTOR_TILE_BYTES_CACHED = 96 MiB` (byte budget, not entry count, since vector tile payload
  size varies hugely).
- `createBoundedSet(maxEntries)` — insertion-order `Set<string>` capped at `maxEntries`, evicting
  the oldest key. Used for `tileMiss`/`vectorTileMiss`, capped at `MAX_TILE_MISS_ENTRIES` /
  `MAX_VECTOR_TILE_MISS_ENTRIES = 2048` each.
`MapRenderer.tileCache`/`vectorTileCache`/`tileMiss`/`vectorTileMiss` now use these instead of raw
`Map`/`Set`.

## T4 — leading+trailing debounce
New exported `createLeadingTrailingDebounce(run, waitMs)`: fires `run()` synchronously on the
leading edge whenever no window is open; any `call()` during the window is coalesced into exactly
one trailing `run()` at window close, which also re-arms a new window (so a sustained drag gets a
tile refresh roughly every `waitMs`, plus an instant first refresh, instead of nothing until the
gesture ends). `MapRenderer` now holds one `tileRefreshDebounce` instance (constructed once, wired
to `void this.refreshTiles()`); `scheduleRefreshTiles()` is now just `this.tileRefreshDebounce.call()`,
and `dispose()` calls `this.tileRefreshDebounce.dispose()`. The old `refreshTimer` field is gone.
The existing `refreshInFlight`/`tilesRefreshQueued` re-entrancy guard inside `refreshTiles()` itself
is untouched.

## T5 — prefetch ring
`refreshTiles()` now takes a `refreshGeneration` snapshot (`++this.refreshGeneration`) before
starting, and after the visible-tile tasks (`refreshRasterTiles`/`refreshVectorTiles`) resolve and
`invalidate()` fires, it awaits a new `prefetchTiles(generation)` step. That step reads
`prefetchTilesJson()`/`prefetchVectorTilesJson()` and reuses the same `uploadTileRow`/
`uploadTileRows` path (so it shares — never competes with — the `MAX_CONCURRENT_TILE_FETCHES = 12`
chunking, since it strictly runs after the visible-tile `Promise.all` has already resolved). It is
abandoned (checked before starting and before every fetch chunk) once `this.disposed` or once a
newer `refreshTiles()` call has bumped `refreshGeneration` past the snapshot it was given.

## T6 — LOD switch no longer drops the byte caches
`setLodMode` no longer calls `tileCache.clear()`/`vectorTileCache.clear()` (they're keyed by
`z/x/y`, which is LOD-independent). It still clears `tileMiss`/`vectorTileMiss` and resets all four
revision trackers to `NaN` so the next poll/refresh re-reads visibility fresh.

## Tests
`🧱️elements/TiledMapHost/🧪️component.test.ts` (new, colocated, mirrors `AgentBridge`/`TaskManager`'s
pattern of not being wired into `@semio-tech/framework-renderer-react`'s own nx `test` target):
- `createByteLru`: LRU eviction order, `get()` counting as a use, byte-budget (not entry-count)
  behavior, `clear()`.
- `createBoundedSet`: entry-cap eviction, `clear()`.
- `createLeadingTrailingDebounce`: leading-edge synchronous fire, burst coalescing into one
  trailing call, repeated per-window firing under sustained calls, `dispose()` cancelling a pending
  trailing call.
- `MapRenderer` end-to-end (fake `MapWasmSession` stub + stubbed `fetch`): asserts `uploadTile` is
  called exactly once and `fetch` exactly once across two `refreshTiles()` calls, once the stub's
  `hasTile` starts reporting the tile present after the first upload.

### How to run it
The renderer-react package's own `🧪️vitest.config.ts` sets `root` to the `⚛️react` package dir (a
sibling of, not an ancestor of, `🧱️elements/`), so its default file-discovery glob cannot reach this
colocated test file — same orphaned-from-any-target situation already documented by `AgentBridge`'s
and `TaskManager`'s own test-file headers. Verified with an explicit `--root` override instead
(mirrors the invocation recorded for `TaskManager` in ticket
`26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`'s `terra-T1-report.md`, adjusted because the
`ui-react` config referenced there has since had its `include` list narrowed and no longer reaches
into `renderer/…/🧱️elements`):

```
node node_modules/vitest/vitest.mjs run \
  --config "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts" \
  --passWithNoTests \
  --root "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements" \
  TiledMapHost/🧪️component.test.ts
```

Result: `Test Files 1 passed (1)`, `Tests 11 passed (11)`.

## Typecheck
`bun x tsc --noEmit -p "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/tsconfig.json"`
(the root `tsconfig.json` fails on these files regardless of this change — it lacks
`allowImportingTsExtensions`, and the file's pre-existing `.tsx`-suffixed imports on lines
30/31/33/34 already trip `TS5097` there; the react-target's own `tsconfig.json` has that flag and is
the correct one to typecheck this package against).

Only expected error surfaced:
```
WasmSessionLoader/🟦️component.tsx(257,3): error TS2740: Type 'MapSession' is missing the following
properties from type 'MapWasmSession': hasTile, hasVectorTile, visibleTilesRevision,
visibleVectorTilesRevision, and 2 more.
```
This is the wasm-bindgen `MapSession` class from `🗺️tiled-map/🦀️component.rs`, owned by the parallel
Rust-side agent adding these six methods — expected to disappear once that lands. No other
TiledMapHost/WasmSessionLoader errors.
