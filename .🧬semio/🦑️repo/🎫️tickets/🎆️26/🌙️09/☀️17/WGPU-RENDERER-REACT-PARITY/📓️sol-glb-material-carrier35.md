# Primitive-local GLB Material Carrier

## Neutral and actual Three proof

The permanent neutral asset `🎨️world3d-glb-material/🧊️two-primitive.glb` contains two reachable triangle primitives in one mesh:

- `vertex-alpha-blend` has a four-component normalized unsigned-byte `COLOR_0`, base color factor, metalness, roughness, emissive factor, `BLEND`, and `doubleSided`;
- `textured-alpha-mask` has UVs, an embedded PNG base-color texture with explicit sampler state, a distinct base color/metalness/roughness/emissive tuple, `MASK` with cutoff, and front-side rendering.

The sibling v1 JSON contract also fixes bounded ownership: primitive-local records, one material field per step, geometry and materials committed together, partial cancellation without publication, and an explicit Icon material replacing every inherited primitive material.

The installed Three `GLTFLoader` parsed the actual GLB, including its buffer-view image through `ImageBitmapLoader`. Its concrete output is:

| Primitive | Color / opacity | Metal / rough | Emissive | Alpha/depth | Side | Extra |
| --- | --- | --- | --- | --- | --- | --- |
| vertex-alpha-blend | `[0.8,0.2,0.1] / 0.75` | `0.65 / 0.2` | `[0.1,0.05,0.02]` | transparent, alphaTest 0, depthWrite false | DoubleSide | normalized RGBA bytes retained, vertexColors true |
| textured-alpha-mask | `[0.1,0.3,0.9] / 1` | `0.05 / 0.85` | `[0,0.1,0.2]` | opaque pipeline, alphaTest 0.37, depthWrite true | FrontSide | sRGB map, clamp/mirror, linear/nearest sampler |

The focused registered Vitest suite is GREEN:

```text
Test Files  1 passed (1)
Tests       2 passed (2)
Duration    683ms
```

It is permanently registered in the WGPU Vitest project and uses the explicit `@semio-tech/framework-renderer-wgpu` project with excluded task dependencies and one worker.

## Current native loss boundary

`GlbPrimitiveSchema` stores POSITION, NORMAL, TEXCOORD_0, indices and mode. It has no `COLOR_0` or material index. `GlbSchemaOutput` has no material, texture, image or sampler tables. `GlbInstancePlanCursor` flattens each reachable primitive placement into contiguous vertex and index ranges, but `GlbMaterializeCursor` seals only one geometry `Mesh3dLease`. The World publishes that lease under the URL-derived mesh id and assigns one environment material to every instance.

The missing identity is therefore the planned primitive range, not an arbitrary vertex tag. The current plan already knows `vertex_base`, `index_base`, `vertex_count`, `index_count`, and the source primitive for every reachable placement. Preserving those ranges avoids duplicating vertices and keeps the established transformed GLB mesh and outline publication.

## Carrier design

Replace the geometry-only asset publication value with one atomic `World3dAssetMeshLease` that owns:

1. the existing `Mesh3dLease`;
2. a fixed-credit ordered array of reachable primitive ranges `{firstIndex,indexCount,material}` taken directly from `GlbInstancePlanCursor`;
3. a fixed-credit material table containing base RGBA, metalness, roughness, emissive RGB, alpha mode/cutoff, side, and optional texture bindings;
4. decoded texture leases plus sampler/color-space records.

The GLB schema cursor should add bounded `materials`, `textures`, `images`, and `samplers` sections, `primitive.material`, and `attributes.COLOR_0`. `COLOR_0` accepts the glTF legal float or normalized unsigned-byte/unsigned-short VEC3/VEC4 shapes; VEC3 supplies alpha one and writes the existing `Mesh3dField::Colors`. Missing vertex color writes white when the combined mesh owns a color lane, keeping every vertex aligned.

Materialization remains incremental. Each JSON scalar, vertex color, primitive-range record, image byte, decoded image step, and texture publication consumes one bounded step. Output credits include material/range storage and decoded texture bytes before any claim. Geometry, ranges, materials and textures seal together. Cancellation closes the partial mesh and every partial texture lease incrementally; no material table or texture becomes visible alone.

Embedded buffer-view images read from the already-owned GLB pages. External image URIs use the existing bounded asset request/response owner and resolve against the GLB URL; they do not introduce a polling loop or another unbounded registry. A missing required texture rejects the same whole GLB load, matching `GLTFLoader` completion ownership.

## Draw projection

The GPU mesh stays one allocation. Prepared draw records gain an index range, so each primitive record draws the shared vertex/index buffers with its own material. This is cleaner than minting one mesh lease per primitive and preserves the existing URL mesh identity, AABB, picking, residency and outline ownership.

The material draw path needs explicit GLB variants:

- base color factor multiplies source vertex color and base-color texture;
- emissive RGB is primitive-local rather than the current environment-global emissive;
- metalness and roughness are primitive-local;
- OPAQUE forces opaque alpha, MASK discards below cutoff with depth writes, and BLEND uses the ordered translucent lane with depth writes disabled, exactly as the Three oracle records;
- `doubleSided` selects no culling; the default selects back-face culling;
- base-color textures retain sRGB decode and authored wrap/filter state.

Regular interaction styling composes after the authored material without erasing primitive identity. Icon's explicit `request.material` selects one replacement material for every primitive range, disables inherited vertex colors and textures, and uses the replacement material's default front-side/opaque behavior, matching `applyIconMaterial`. An absent request material selects the inherited records unchanged. Render-only one-degree outlines remain a separate presentation channel and do not become semantic edges.

## Required native acceptance

The implementation packet should add:

- the current neutral fixture to the Rust schema/materializer test, asserting both range records and exact material/texture values;
- one cancellation law proving no geometry, material, or texture lease publishes from a partial decode;
- one World law proving two primitive ranges prepare two material draws with the same logical instance transform;
- one explicit-override law proving both ranges use the request material and ignore inherited color/texture/alpha/side;
- production WGSL WebGPU readback for inherited vertex-color BLEND and textured MASK pixels, compared to an actual Three `WebGLRenderer` rendering the same GLB.

## Implemented checkpoint

The carrier is now implemented across the neutral scene contract, WGPU draw path, World residency owner, and renderer GLB materializer:

- `SceneAuthoredMaterial3d` carries primitive base color, emissive, metalness, roughness, alpha mode/cutoff, side, vertex-color policy, base-color texture lease, and authored sampler state. `SceneMaterialDraw3d` carries the exact shared-buffer index range.
- `World3dMeshAppearance` owns ordered primitive records and texture raster leases beside the mesh lease. `publish_world3d_asset_mesh` preflights mesh, appearance, interaction, and texture ownership, then publishes them atomically.
- The GLB cursor parses legal normalized `COLOR_0`, material/PBR, alpha, side, sampler, texture, and embedded image records. It copies embedded image bytes in bounded 256-byte steps, decodes into the existing raster lease interface, and closes every partial owner on cancellation or rejection.
- The prepared draw lane clamps each primitive range, selects opaque/mask/blend and front/double-sided pipelines, binds the authored sampler and texture, and retains the shared mesh transform. Missing Icon material keeps authored records; an explicit Icon material replaces each record and disables inherited vertex colors and textures.
- The production shader multiplies base color, vertex color, and base-color texture, adds authored emissive, discards masked alpha, and emits straight RGBA. WebGPU alpha blending stores the expected premultiplied composite; export converts that composite to straight RGBA exactly once during PNG encoding rather than changing draw semantics.

The actual installed Three `GLTFLoader` oracle remains GREEN 2/2. A native fixture law now drives the production incremental materializer against the same GLB and asserts exact primitive ranges, vertex colors, PBR values, alpha mode/cutoff, side, embedded texture, sampler, and exact owner retirement. The scoped renderer library compile and native law results are recorded below when their running jobs finish.

One boundedness limit remains explicit: byte transfer, lease admission, publication, and cancellation are stepped, while the existing first-party image decoder performs its internal decode in one call after all bytes arrive. No paint-time triangle or texture scan was added.

## Request-size export audit

`icon_render_world_scene` remains the correct shared preview/export authority. A request-size export must call it with `preview_scale = 1`, render the physical viewport at the admitted request width and height, and retain `presentation.sourceAspect = width / height`. Preview is the only caller that scales orthographic zoom by `content_frame.w / request.width`.

The current environment payload has the required material distinction. An absent request material serializes a material record whose color is null: `environment_authored_material` therefore yields no override, the GLB primitive records and embedded texture remain active, and the black default edge still comes from the stroke field. An explicit request material resolves a complete authored replacement and replaces every primitive record.

A standalone offscreen World cannot borrow a preview's `World3dMeshAppearance`. Embedded texture leases are exact, non-cloneable owners. The export job must therefore admit and publish the complete request-owned `World3dMeshAsset` (mesh plus appearance plus texture raster leases), advance its texture upload/residency to acceptance, and only then read back. Publishing the older geometry-only lease would silently revert to environment material and lose embedded textures, vertex color, alpha, and side semantics.
