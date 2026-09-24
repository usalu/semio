# WP-O1d — Plugin Registry Generate/Check + Renderer-WGPU Zero Failures

Slice: O1d. Scratch: `.tmp-ticket/wp-o1d/`. Private cargo: `.tmp-ticket/wp-o1d/target`.

## Status

| Item | State | Evidence |
|------|-------|----------|
| plugin-registry:generate | WAITING | O1c `*-82252-o1c` still queued; s14 holds wasm lock |
| plugin-registry:check green | PENDING | after generate |
| wfc/norm descriptor byte-identity | PENDING | compare generate output vs hand-sync |
| framework-renderer-wgpu:test 0 fail | IN PROGRESS | serial4 running after host_id cancel fix |
| wasm32 ui wgpu-engine check | PENDING | under wasm mutex after generate/check |

## Failure clusters (serial3: 1283 pass / 89 fail)

1. **MapTile lane pollution (root)** — `reserve_map_tile_fetches` keys claims by engine **host_id**; `close_map_sync` cancelled by wire **surface_id**. Cancel never matched; 64-slot shared lane filled; later maps reserved nothing. Cascade: capacity/seal/fault clusters.
2. Polluters confirmed: `retained_map_key_replacement_*`, `retained_map_same_host_sibling_*` then break `tiled_map_paint_*`.
3. Downstream: seal "could not be sealed", resident roots 64/64, ArenaFull, pointer document fault — treat as cascade until serial4.

## Fixes

- `EngineCanvas` wgpu: `EngineSurfaceRetirement::new(surface, host_id)` — MapTile cancel uses registry host id (same key as reserve).
- Prior (O1d earlier): toolbar TreeItem projection; world `close_step` clears `closing` when empty; `cancel_map_tiles_for_surface` + drain; EngineSurfaceSlot size 76472.

## Gaps

- serial4 result pending
- generate/check/wasm32 ui check pending on mutex

## Files changed

- EngineCanvas wgpu target `rs` (host_id cancel)
- boxed-fixed-slots EngineSurfaceRegistry elementSizeBytes (prior)
- Shell tree_item toolbar projection (prior)
- world asset close_step / cancel_map_tiles (prior)
