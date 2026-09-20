# World GLB Outline and Reference Raster Audit

Read-only source audit on 2026-09-20. No build, activation, or production edit was performed.

## Confirmed GLB outline gap

### Owned path

The source has one complete, retained GLB route:

1. A URL mesh entry is retained in `World3dState::mesh_source_urls` by
   `declare_scene_mesh_source`
   ([world](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:11330)).
   `render_world_3d` uses that same map to reserve its missing GLB
   ([world](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:8033)).
2. The renderer's bounded GLB probe materializes a `Mesh3dLease`; its plan bakes the
   glTF-Y-up to World-Z-up matrix into positions and normals
   ([renderer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:2096)).
3. A ready probe publishes that exact lease through
   `publish_world3d_asset_mesh_lease`
   ([renderer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11341)).
   Publication stores it under the URL-derived mesh id and only clears the pending URL
   ([world](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:15212)).

The existing end-to-end source law already proves that a fetched GLB becomes a resident
mesh with real positions and indices, but it neither requests nor observes outline
segments ([async boundary law](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:1292)).

### React behaviour and WGPU failure mechanism

React creates an `EdgesGeometry` for every loaded GLB mesh, caches it by source
`BufferGeometry`, creates a `LineSegments` child, and scales that child by 1.001
([World3dHost](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:2281)).
That call is unconditional in `GlbInstanceMesh`
([World3dHost](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:2357)).

WGPU only emits stored semantic mesh edges. `append_component_overlays` returns before
emitting a line when `schema.edges == 0`
([world](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:9713)).
The GLB materializer writes positions, normals, and indices; it does not generate the
`Mesh3dField::Edges` field. Its existing semantic GLB law reads precisely those three
fields ([async boundary law](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:454)).
Therefore a normally loaded GLB has zero WGPU outline vertices while its React peer gets
the Three outline. This is the direct source cause of the missing mesh outline in the
paired physical image. Confidence: high.

`show_edges` defaults to true
([world](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:1978)),
so it does not mask the observed default-state discrepancy. React's GLB outline code
does not inspect that flag. If product policy says the flag should suppress GLB outlines,
that policy needs a paired React change; it is not present in the current reference.

### Narrow repair ownership

Put the rendering policy in `infinite_world::world`, but build the data while the
bounded GLB materializer owns the decoded lease:

- Extend the renderer's `GlbMaterializeCursor` with a retained,
  one-triangle-at-a-time outline-derivation cursor. It must finish and seal a
  **render-only GLB outline lease/cache** before `take_ready_mesh_lease` can make the
  decoded asset publishable. This puts the O(triangle-count) work on the existing
  bounded asset lane; it must never run from `append_component_overlays` during a
  paint.
- Publish and retire that outline with the matching `Mesh3dLease` under the
  URL-derived mesh id. `infinite_world::world` then emits its precomputed segments
  into `ScenePass3d.line_draws` only for keys still present in
  `mesh_source_urls`, where all current World line geometry is collected
  ([world](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:12799)).
- Keep it separate from `Mesh3dSchema.edges` and `Mesh3dField::Edges`. Those values
  drive edge picking, component ids, interaction revisions, and selection eligibility;
  synthesizing them merely to draw a border would invent selectable domain components.
- Retire the derived cache together with its mesh. Mesh eviction already removes the
  corresponding `mesh_source_urls` record
  ([world](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:7989)).
  Its producer cursor needs the same cancellation/close path as
  `GlbMaterializeCursor`, and the cache needs the same fixed bound and removal path.
  Do not allocate or re-derive per frame.

This preserves the existing decoded-asset transport, lease ownership, mesh GPU upload,
and component-selection model. It changes only the visual companion that React attaches
to a GLB.

### Feasible neutral regression packet

Use a small neutral fixture beside the existing camera-framing fixture:
`🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/🎨️world3d-glb-outline/🔣️.json`.
It should contain three indexed cases, each with canonical undirected expected segments,
`thresholdAngleDegrees: 1`, and `outlineScale: 1.001`:

1. a coplanar two-triangle square with four outer segments and no shared diagonal;
2. a folded two-triangle crease, whose shared sharp edge is present;
3. a coplanar square whose neighbouring triangles use duplicate coordinate values under
   distinct indices, whose shared diagonal is still absent after position welding.

Those cases cover the boundary, crease-angle, and duplicate-position behaviour of
`EdgesGeometry`; a generic all-triangle wireframe would fail this oracle.

The TypeScript oracle should create a `BufferGeometry` from the fixture,
run `new THREE.EdgesGeometry(geometry, 1)`, canonically sort undirected segments, and
compare them to the fixture. This follows the already accepted neutral-fixture plus
actual-Three pattern in the camera-framing oracle
([camera oracle](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎥️world3d-camera-framing/🟦️.ts:17)).

The Rust law belongs in the World unit module, which already has access to the actual
private render path and can read a resulting `ScenePass3d`
([World unit tests](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs:3677)).
It should:

1. drive the actual GLB materializer over each fixture topology, publish its completed
   mesh-plus-render-outline result under a key in `mesh_source_urls`, add one
   transformed draw, and call the real `render_world_3d`;
2. canonically compare the produced line segment set, including the 1.001 local scale,
   to the fixture;
3. assert that the mesh schema's semantic edge count remains zero.

Together with the existing fetched-GLB resident-mesh law, this covers decode publication,
the visual derivation, and the independent Three result without importing Three into Rust.

## Reference-raster aspect: suspected cache cause rejected by current source

`apply_decoded_reference_raster` does publish a raster into
`reference_pixels` and does not advance `geometry_generation`
([world](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:15611)).
The observation is real, but `geometry_generation` is not a prepared-scene invalidation
key. Its only current consumers reset the camera-fit cursor on mesh changes
([world](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:11648)).

The actual aspect path is correct in isolation:

- A missing raster uses aspect 1.0; a decoded raster reads its descriptor width/height
  ([world](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:14524)).
- `render_world_3d` constructs the textured instance model from that current aspect
  ([world](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:12892)).
- The main retained route calls `render_ui_document_step`, which calls
  `Ui::frame_into_step`
  ([Interpreter](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2288)).
  Unlike dirty-gated `Ui::frame_step`, `frame_into_step` starts a fresh retained paint
  and its Scenes phase calls the supplied `FrameworkSceneHost` for the slot
  ([UI engine](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1731)).
  That host immediately calls `render_component_scene_step`
  ([Interpreter](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1964)).

Thus the loaded raster is observed by a fresh scene pass on the next frame build. The
renderer processes asset decode before it starts that build
([renderer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12945)).
The source does **not** substantiate a stale square prepared-scene cache. Calling
`RuntimePresentationAuthority::mark_scene_changed` from raster publication, or
repurposing `geometry_generation`, would add invalidation churn without a demonstrated
owner and can supersede the transaction that is currently applying the asset.

React also initially uses aspect 1 before media arrives, then recomputes plane dimensions
from the loaded media's width/height
([React World layer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3940))
and explicitly invalidates its demand frame on media arrival
([React World layer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:4014)).

### Appropriate regression before any raster repair

Add a real scene-pass law, not a generation-token law:

1. create a visible reference with `widthWorld: 10` and no raster; render it and
   observe a 10×10 textured model;
2. call the actual `apply_decoded_reference_raster` with a 2275×2560
   `SceneRasterLease`;
3. render again through `render_world_3d` and assert the next textured model is
   10×(10 × 2560 / 2275), keeps the same texture key, and has only one upload offer.

The existing unit helpers and direct scene-pass assertions make this feasible
([World unit test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs:3677)).
It should pass current source; it prevents a future regression. If a full retained-frame
reproduction still observes a square model after that law, its receipt must identify the
actual `TexturedInstance3d.model` dimensions and the window/surface id. Only then is
there evidence for a cache invalidation repair.

Confidence: high for the GLB outline gap and its ownership; high that
`geometry_generation` is not the aspect-cache mechanism; medium on the cause of the
physical square appearance because no model-dimension receipt was part of this audit.
