# Raster Semio Example Not Showing

## Root cause

Loading `semio.raster.json` via `setActiveExample` deserialized into the plugin domain, then `document_sync_json` re-serialized layers for the WASM paint host.

Plugin `RasterLayerNode::Adjustment` had **no `params` field**, so fixture params were dropped. Paint host `LayerNodeJson::Adjustment` required `params` with no `#[serde(default)]`, so `syncDocumentJson` threw and the compositor never received the Semio document (canvas stayed on the empty/boot document).

## Fixes

1. Paint: `params` defaults; group children accumulate parent transform once (no double camera); picking/marquee/navigator bounds follow the same parent transform chain.
2. Plugin: preserve adjustment `params` as a JSON map through serialize/deserialize; fill `WindowMeasure::Group` state fields required by the unified UI presence model.
3. wgpu paint2d: use `cameraJson`, render pixel layers without `imageKey`, center quads like WASM.
4. React: log `[DEBUG]` on sync/upload failures instead of swallowing/crashing silently.
5. Tests: assert `componentKind: paint-2d` and presence-stamped hover (`state: previewed`).

## Verification

- `cargo test -p framework_surface_paint --lib` → 9 passed
- `cargo test -p raster-plugin --lib` → 25 passed
