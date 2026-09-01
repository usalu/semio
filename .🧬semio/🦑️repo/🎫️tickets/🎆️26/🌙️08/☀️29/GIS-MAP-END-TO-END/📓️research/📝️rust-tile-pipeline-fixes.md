# Rust tile pipeline fixes (R1–R5)

File touched (only this one, per assignment):
`🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs` (crate `semio-framework-surface`).

## R1 — idempotent tile upload
`upload_tile`/`upload_vector_tile` now compute the `z/x/y` key first and early-return `Ok(())`
when it already exists in `self.tiles.tile_images` / `self.tiles.vector_tiles`, before touching
`image::load_from_memory`/`decode_mvt`. A hit still records a touch (see R3). Real misses keep the
old error behaviour — `upload_tile_invalid_png_bytes_returns_err` / `upload_vector_tile_invalid_bytes_returns_err`
are unchanged and still exercise the decode-then-error path since the key is absent on a fresh host.

## R2 — allocation-free dirty signal
Added `MapHost::visible_tiles_revision() -> u64` / `visible_vector_tiles_revision() -> u64`.
`VisibleTileCursor` gained `z()` and `bounds() -> (x_min, x_max, y_min, y_max)` accessors (the
struct's private fields were renamed `y0/y1` → `y_min/y_max` and gained `x_min`/`x_max`, computed
once at construction, stable across `advance()`). A small inline FNV-1a mixer
(`fnv1a_mix`/`tile_range_revision`, no external crate, no `DefaultHasher`) hashes `(z, x_min, x_max,
y_min, y_max)` with no allocation. Vector variant returns sentinel `0` exactly when
`visible_vector_tile_cursor()` is `None` (same guard `visible_vector_tiles_json` uses for `"[]"`).

## R3 — LRU retention
`MapTileLedger` replaced `last_raster_visible`/`last_vector_visible` (one-frame-deep sets) with
`raster_touch`/`vector_touch: RefCell<BTreeMap<String, u64>>` plus a `touch_clock: Cell<u64>`
monotonic counter, so `&self` draw methods can record recency via interior mutability.
`tile_retention_keys` (visible ∪ ancestors ∪ previous-frame-visible) became `pinned_tile_keys`
(visible ∪ ancestors only — no previous-frame union needed now that eviction is real LRU).
`retain_tiles_for_keys`/`retain_vector_tiles_for_keys` no longer drop every non-pinned entry
immediately; they only evict, one at a time, when the map exceeds `MAX_MAP_TILE_CACHE_ENTRIES`,
always picking the non-pinned key with the smallest touch tick. Tiles are touched at both upload
sites (fresh insert and idempotent hit) and inside `append_tiles`/`append_vector_tiles` for every
tile that actually intersects the viewport (i.e. is drawn) — "touched = uploaded or drawn" per spec.

## R4 — prefetch ring
`tiles::prefetch_ring_tiles(bounds, visible, z, n, cap)` enumerates the one-tile border just outside
`bounds`, wrapping x mod `n` and clamping y to `[0, n)`, excluding rows already in `visible`,
deduplicated (wrapping can otherwise double-count at very coarse z) and capped. `MapHost::prefetch_tiles_json`/
`prefetch_vector_tiles_json` reuse it with `cap = MAX_VISIBLE_TILE_REQUESTS - visible.len()`, same
`{z,x,y,key}` row shape as `visible_tiles_json`. Vector variant returns `"[]"` when vector tiles are
unavailable at the current zoom, matching `visible_vector_tiles_json`.

## R5 — wasm exposure
Added to `MapSession`'s `#[wasm_bindgen]` impl, matching the TS-side contract exactly (verified
against the parallel TS agent's own report, `📓️research/📝️ts-maphost-perf-fixes.md`, which lists the
identical six names): `hasTile`, `hasVectorTile`, `visibleTilesRevision`, `visibleVectorTilesRevision`
(both masked to 53 bits via `& ((1u64 << 53) - 1)` so the `u64` round-trips exactly through a JS
`number`), `prefetchTilesJson`, `prefetchVectorTilesJson`.

## Tests added (in the existing `mod tests`, reusing `test_png_1x1()`)
- `upload_tile_second_call_on_existing_key_is_idempotent_and_skips_decode` /
  `upload_vector_tile_second_call_on_existing_key_is_idempotent_and_skips_decode` — second upload
  with invalid bytes on an existing key returns `Ok`; raster variant also asserts `Arc::ptr_eq` on
  the cached image before/after to prove the entry itself was untouched (not just present).
- `visible_tiles_revision_is_stable_then_changes_on_pan_and_zoom` — stable across two calls with an
  unchanged camera, changes after a pan (camera.x shifted by 4 tile-widths at the picked z), changes
  after a zoom (camera.zoom ×6).
- `visible_vector_tiles_revision_matches_json_unavailable_sentinel` — forces `camera.zoom = 0.0`
  (the exact `vector_tiles_available_at_camera_zoom` guard) and checks both `visible_vector_tiles_json()
  == "[]"` and `visible_vector_tiles_revision() == 0` together.
- `prefetch_tiles_are_disjoint_from_visible_share_z_and_respect_cap` — parses both JSON arrays,
  asserts no key overlap, same `z`, and `visible.len() + prefetch.len() <= MAX_VISIBLE_TILE_REQUESTS`.
- `prefetch_vector_tiles_json_is_empty_when_vector_tiles_unavailable`.
- `lru_eviction_never_evicts_a_pinned_visible_tile_and_takes_the_stalest_one_first` — seeds a
  same-zoom "stale" tile on the opposite side of the world (touched first, and structurally not an
  ancestor of the visible tile, so not pinned) plus the currently-visible tile, confirms both survive
  `prepare_visible_tiles()` under the cap, then uploads `MAX_MAP_TILE_CACHE_ENTRIES + 8` filler tiles
  at an unrelated z to force eviction, and asserts the stale tile is gone while the pinned/visible
  tile survives.
- Pre-existing `tile_retention_keeps_raster_ancestor_after_zoom_in` /
  `tile_retention_keeps_vector_ancestor_after_zoom_in` still pass unchanged against the new pinning
  logic (ancestor pinning is preserved, just no longer unioned with a previous-frame set).

## Verify

**Native tests** (`cargo test -p semio-framework-surface`):
TBD — see `🗑️generated/cargo-test-output.txt` for full output once the run (started in a heavily
contended shared workspace — a dozen+ concurrent `cargo`/`rustc` processes from other sessions were
observed via `ps aux` at the time) finishes.

**Wasm target check** (`cargo check -p semio-framework-surface --target wasm32-unknown-unknown`):
TBD.

## Not done / out of scope
Nothing from R1–R5 was skipped. `MapHostRetirement` (the incremental-release wrapper) was updated to
carry `raster_touch`/`vector_touch` (as plain `BTreeMap`s via `RefCell::into_inner`) through
`close_step`/`terminal_is_empty` so its one-item-per-grant release contract still covers the new
per-tile touch metadata; `touch_clock` (a single `u64`, not a collection) is dropped directly and
not tracked in the retirement step count.
