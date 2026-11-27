FINAL (2026-11-27 00:50: node ids bounded at 64 by the shared rule, three assets pass the catalog run, model-exports green; model-digests red only from the quantization change of another lane, not re-pinned)

# CX-ID: node ids over 64 characters

## Cause
Scene-scoped ids `<scene>-<id>` (scenes like `s100-thermal`, 8 to 14 characters, plus a long solid, material, node, mesh or skin id) exceeded the 64 characters of `Slug` in `semio.model/v1` (`maxLength` 64). Ajv reported `/nodes/N/id`. The schema limit is unchanged.

## Cure (shared, one rule)
`S/🔨️modules/🧊️volume/🪪️ids/🟦️.ts`: `scopedId(scene, id)` is `<scene>-<id>` up to 64 characters (`ID_MAX`); longer ones keep a readable head (no trailing `-`) and end in `-` plus an 11-digit stable hash of the full text, so ids are deterministic and distinct. It is used by the procedural model (`S/🔨️modules/🧊️volume/🟦️.ts`, `p(id)`) and by the resolved-scene join (`S/🔨️modules/📤️export/🗿️model/🧩️scenes/🟦️.ts`, `joinScenes`), so GLB, FBX and model JSON take their ids from the same function. No asset needed shortening. This code was already in the tree (19:51, after the failing shard logs); this lane verified it and did not change code.

## Test
`S/🧪️tests/🪪️scoped-ids/` (`🥒️.feature` + `🟦️.ts`): short id is the plain prefix; ids up to 120 characters stay at most 64, valid Slug, deterministic, distinct, readable head; `operating-table` procedural, clip and resolved model validate with Ajv (`semio.model/v1`) with unique ids for nodes, solids, materials, meshes, skins. References: Ajv against the staffage `Slug` and the kernel schema. Result: 3 of 3 passed.

## Proof
- Single-asset `🧊️model-catalog` for `operating-table`, `reclaimed-roof-tile-pallet`, `reclaimed-sandstone-window-surround`: 1 file, 5 of 5 passed, no id-length error.
- `🧊️model-exports`: 18 of 18 passed.
- `🧊️model-digests`: 37 of 43 passed; the 6 GLB pose digests of the sample figure differ: `KHR_mesh_quantization` is now in `extensionsUsed`, accessors 522 to 595, byteLength 3435372 to 3281488, new sha256. Node, mesh, material, skin and primitive counts are identical. The sample figure has no ids over 64 characters, so this is not the id rule; it comes from the GLB-size change in `S/🔨️modules/📤️export/🧊️glb/🟦️.ts` and `🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/` (modified after 19:52). The FBX digests pass. I did not re-pin: the owner of the quantization change must re-pin `🧫️fixtures/🧊️model-digests` once that change is final.
- Not run: the full `🧊️model-catalog` (other assets from shards still running).
