# Raster Semio Example Not Showing

## Root cause

Loading `semio.raster.json` via `setActiveExample` deserialized into the plugin domain, then `document_sync_json` re-serialized layers for the WASM paint host.

Plugin `RasterLayerNode::Adjustment` had **no `params` field**, so fixture params were dropped. Paint host `LayerNodeJson::Adjustment` required `params` with no `#[serde(default)]`, so `syncDocumentJson` threw and the compositor never received the Semio document (canvas stayed on the empty/boot document).

## Fixes

1. Paint: `params` defaults; group children accumulate parent transform once (no double camera).
2. Plugin: preserve adjustment `params` as a JSON map through serialize/deserialize.
3. wgpu paint2d: use `cameraJson`, render pixel layers without `imageKey`, center quads like WASM.
4. React: log `[DEBUG]` on sync/upload failures instead of swallowing/crashing silently.
