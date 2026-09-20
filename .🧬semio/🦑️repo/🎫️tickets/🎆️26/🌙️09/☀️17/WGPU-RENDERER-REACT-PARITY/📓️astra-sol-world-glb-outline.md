# World GLB Render-Only Outline Contract

## Executed neutral and third-party boundary

The neutral fixture `framework.world3d.glb-outline/v1` fixes the current React policy without adding semantic mesh edges:

- threshold angle: 1 degree;
- coordinate weld precision: four decimal places;
- local outline scale: 1.001;
- work credit: one triangle per derivation step;
- publication: mesh and outline become visible together;
- cancellation: partial mesh and outline owners retire without publication;
- semantic edge ids: empty.

The three indexed topologies distinguish the intended `EdgesGeometry` behavior from an all-triangle wireframe:

| Case | Expected render segments |
| --- | ---: |
| coplanar indexed square | 4 boundaries; shared diagonal removed |
| folded two-triangle crease | 5, including the sharp shared edge |
| coplanar square with duplicate coordinate values and distinct indices | 4 boundaries after coordinate welding |

The TypeScript oracle constructs an actual Three `BufferGeometry`, runs `new EdgesGeometry(geometry, 1)`, reads its emitted positions, canonicalizes undirected segments, and compares them to the neutral fixture. It also proves the 1.001 transform remains a render policy and that the fixture invents no semantic ids.

Executed command:

```sh
SEMIO_TEST_LEVEL=selected NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun x vitest run '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎨️world3d-glb-outline/🟦️.ts' --config '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts'
```

Result: 1 file passed, 3 tests passed, 958 ms. No native build or activation was run for this packet.

## Source-owned implementation design

Three's installed `EdgesGeometry` implementation is precise enough to reproduce without a runtime Three dependency. For each indexed triangle it rounds each transformed position component with `round(value * 10_000)`, skips a triangle when two welded hashes match, calculates one normalized face normal, and considers three directed edges. A reversed sibling nulls the earlier direction and emits the later endpoints only when the normal dot product is at most `cos(1°)`; unmatched directed edges become boundaries. A hash remains present after it is nulled, which matters for non-manifold repeated directions and must be preserved by the reducer.

The derivation belongs in the existing bounded `GlbMaterializeCursor`, after `Indices` and before either mesh or outline is sealed. Proposed owned phases are `OutlineReserve`, `OutlineTriangles`, `OutlineSort`, `OutlineReduce`, `SealMesh`, and `SealOutline`.

`OutlineTriangles` reads the already-written index and position fields, consumes exactly one triangle per call, and appends at most three `GlbOutlineEdgeRecord` values. A record owns the canonical welded endpoint pair, directed orientation, original endpoints, normalized face normal, and source ordinal. Degenerate welded triangles append nothing. There is no per-frame mesh readback.

`OutlineSort` is an iterative bottom-up merge cursor rather than one call to `sort`: one step compares and moves one record, and each completed width pass swaps the source and scratch vectors. Records sort by canonical undirected key and then source ordinal. `OutlineReduce` consumes one sorted record per step and simulates Three's two directed hash entries, including present/null state. It emits the later reversed endpoints for qualifying creases and emits each still-present direction at group end. This preserves Three's non-manifold order behavior without a runtime hash table or unbounded map.

The materializer reserves exact vector capacities once in `OutlineReserve`. The plan calculates the peak as existing mesh output plus two `index_count × size_of::<GlbOutlineEdgeRecord>()` buffers plus at most `index_count × size_of::<WorldGlbOutlineSegment>()`; arithmetic overflow or a peak above the existing `GLB_SCHEMA_OUTPUT_BYTES` 16 MiB ceiling is rejected before allocation. The pending records and final segment count can therefore never exceed the admitted index count, and no capacity increases.

The derived value should be a world-owned `WorldGlbOutlineLease` containing a boxed segment slice and its generation/revision. It is a render-only owner: it has no `Mesh3dSchema`, `Mesh3dField::Edges`, component ids, picking targets, or interaction revision. `GlbMaterializeCursor` owns its three bounded vectors and completed outline lease alongside the existing mesh write token/lease. `begin_close` changes the phase before any further reads; `close_step` clears one retained vector or lease owner per call and continues the existing paged mesh abort/close. `Ready` is reachable only when both mesh and outline have sealed.

`RendererAssetProbe` should exchange one indivisible `WorldGlbRenderAsset { mesh: Mesh3dLease, outline: WorldGlbOutlineLease }`. A failed publication restores the whole asset to the probe. A successful publication relinquishes both before the response owner begins closing. This avoids a state where the mesh is resident without the reference outline or where one of two owners is lost.

`World3dState` should retain outlines in a `WorldDynamicRegistry<WorldGlbOutlineLease, WORLD_DYNAMIC_MESH_CAPACITY>` under the same URL-derived mesh id. `publish_world3d_asset_mesh_lease` should become a render-asset transaction: preflight the mesh, mesh-version, interaction-mesh, and outline slots; commit all owners only after every plan succeeds; and return the exact bundle on refusal. Mesh eviction must remove and retire the same outline before deleting `mesh_source_urls`. World close must drain this registry through its own bounded close slot. The registry remains at the existing 256-mesh ceiling.

At paint time, a GLB draw reads already-derived segments only when its key still has URL provenance in `mesh_source_urls`. Each endpoint is multiplied by the local 1.001 scale and then by the instance model before it is appended to `ScenePass3d.line_draws`, matching React's scaled child `LineSegments`. No topology, welding, adjacency, or face normals are derived per frame. The visual remains absent from component selection and edge ids. Geometry, ownership, and scale are this packet's contract; the existing environment/palette stroke-color gap needs its own neutral color law rather than another inline constant.

## Draft fail-first native laws

The first compile-safe RED belongs in the World unit module and must use only current public production seams. For each neutral topology it should:

1. drive the actual URL scene bridge so `mesh_source_urls` owns the GLB provenance and the real instance draw survives;
2. publish a zero-semantic-edge mesh with the fixture positions transformed from glTF Y-up to World Z-up and `schema.edges == schema.edge_ids == 0`;
3. disable only the grid, render through `render_world_3d`, and canonicalize the resulting line vertices;
4. require exactly the fixture segments after local 1.001 scale and the delivered instance model;
5. require semantic edge counts and component ids to remain empty, then retire the full surface.

Current production reaches step 3 with no GLB line vertices, so this is a behavioral RED rather than a source-anchor assertion. It does not seed semantic `edge_positions` to make the test pass.

After the production bundle type exists, the existing renderer law `a_fetched_glb_becomes_the_resident_world_mesh_its_url_names` should be expanded rather than duplicated. Its actual GLB response already crosses response paging, retained structure/schema decode, instantiation, materialization, publication, and terminal handback. The expanded law should build each fixture topology as real GLB bytes, take one atomic `WorldGlbRenderAsset`, publish it under the URL key, render the same delivered draw, and assert the canonical line set plus zero semantic edges.

Three owner laws complete the packet:

- cancel after at least one `OutlineTriangles` step; `close_step` must empty response pages, mesh write/lease, both edge-record buffers, output segments, and any outline lease without either world registry becoming resident;
- fill the outline registry to `WORLD_DYNAMIC_MESH_CAPACITY`, reject the next atomic publication with its exact mesh and outline generation/revision still readable, clear one admitted owner, and publish that exact returned bundle on retry;
- evict one URL mesh through `sync_mesh_pool`; the matching outline must retire in the same bounded dynamic-retirement lane, the provenance URL must leave `pending_glb_urls`, and a later draw must be able to reserve the URL again.

## Ownership audit

The current probe takes only `Mesh3dLease`. On a runtime lock miss, missing interaction, missing surface, or registry refusal it restores that lease to `GlbMaterializeCursor`; on success it asserts the materializer no longer owns it and begins closing the response. The bundle must preserve these exact four branches. No branch may take the mesh and leave the outline behind.

The current world registry transaction preflights mesh, version, and interaction-mesh slots before its first commit, but a replacement parks only the prior mesh in `dynamic_mesh_close`. Outline admission must join that same preflight. A replaced outline needs its own `dynamic_outline_close` owner, and publication must refuse before the mesh map changes when either retirement slot is occupied. Full world retirement must begin-close the outline registry, take one outline entry per step, clear both active/blocked outline owners, and include every outline field in `world3d_dynamic_retirement_terminal_is_empty` and the non-test `Drop` witness.

`sync_mesh_pool` currently retires the mesh, then removes `mesh_source_urls`, then evicts GPU buffers. Its successful branch must first reserve both mesh and outline retirement slots. If either owner cannot advance, it returns with mesh, outline, and provenance still mutually readable. Once both owners detach, the URL ledger and GPU mesh can retire as they do today.

## Native laws required before production completion

The source law should materialize fixture-equivalent actual GLB bytes through the real `GlbMaterializeCursor`, publish the returned asset, draw one transformed instance, and compare canonical `ScenePass3d.line_draws` segments to the fixture including the 1.001 local scale. It must also assert `Mesh3dSchema.edges == 0` and `edge_ids == 0`.

A cancellation law should stop during `OutlineTriangles`, request closure, and prove the response owner, mesh write/lease, outline records, and outline lease all reach terminal empty without entering either world registry. A refusal law should saturate outline admission, prove the exact mesh-plus-outline asset returns to the probe, clear the refusal, and publish the same generation/revision pair on retry. An eviction law should prove the mesh and outline disappear together and the URL becomes fetchable again.

Production Rust remained unchanged in this test-first packet so the Native54/full-native/activation boundary stayed source-stable.
