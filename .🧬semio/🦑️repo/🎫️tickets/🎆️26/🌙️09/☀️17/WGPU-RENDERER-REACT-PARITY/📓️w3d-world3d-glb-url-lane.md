# 🥽️ W3d — the URL half of the World3d mesh lane

Packet W3d of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, the P0 handed over by
`📓️w3a-asset-decoder-boot-fault.md` §6/§9.1. Every line number is post-edit; every command in
§5 was RUN.

**Result: on the live puzzle3d playground, a mesh the wire declares by URL now keeps its draw and its
instances, reserves exactly one bounded `WorldAssetRequestKind::Glb` fetch, is fetched through the
catalog transport path, decodes through the renderer's own streaming glTF path and becomes the
RESIDENT mesh under the id the wire names it by — in the world's Z-up frame — on BOTH World3d
surfaces, with its instance submitted to the scene pass and no fault for 40 s** (§6.4). Five defects
were fixed on the way; three of them (§6.1–§6.3) were only reachable once the first two were.
The scene still shows no 3D, for a reason that is NOT the mesh lane and that swallows the reference
underlay too — stated precisely, with its evidence, in §7.

---

## 1 — What was actually dead, and it was more than the url half

`World3dSceneBridgePhase::Parse` retained only meshes that already carried triangles:

```rust
cursor.meshes.retain(|mesh| mesh.data.vertex_count() > 0 && mesh.data.indices.len() >= 3);
cursor.instances.retain(|instance| cursor.meshes.iter().any(|mesh| mesh.id == instance.mesh_id));
```

`World3dSceneMeshEntry` had only `{ id, data }`. React's `WorldMeshRecord`
(`🌐️World3dHost/🟦️.tsx:160`) has **three** shapes, and the guest publishes all three
(`world3d_meshes_json_from_kinds_and_urls`, `🔌️plugin/🦀️.rs:39239`):

| wire shape | React | wgpu before |
| --- | --- | --- |
| `{id, data}` inline | `geometryFromMesh` | published |
| `{id: "mesh:x", url: "/mesh/x.glb"}` | `useLoader(GLTFLoader, meshAssetTransportUrl(url))` | **dropped** |
| `{id: "box", kind: "box"}` | `meshDataFromKind` | **dropped** |

So the packet's brief understated it: puzzle3d's `mesh_lane`
(`🧩️puzzle/…/🪟️windows/🧊️main/🦀️.rs:581`) publishes `box`, `vortex-marker` **and** the two
concrete-forest urls, and the bridge threw away all four plus every instance placed on them. That is
the whole of `state-draws=0 state-instances=0` and `dumpMeshStats` = `indices:0, positions:0` on four
meshes.

Downstream of that, `mesh_source_urls` had no production inserter, so the request loop that already
existed (`render_world_3d`, reading draws whose mesh is not resident) had nothing to offer and
`WorldAssetRequestKind::Glb` was never reserved in production, on any target, ever.

## 2 — Design: the wire declares, the pipeline resolves, nothing stands in

The lane is W1f's terrain lane, one surface over, with one extra rule the terrain lane does not need.

| site | change |
| --- | --- |
| `♾️infinite/🌍️world/🦀️.rs:9854` | `World3dSceneMeshEntry` gains `url` and `kind`, plus `has_inline_geometry`/`names_a_mesh` (`:9867`, `:9874`) — the docstring is the three-shape table above |
| `:10140` | `Parse` retains an entry that NAMES a mesh (inline geometry **or** url **or** kind). A wire-only polyline (data present, fewer than three indices, no url, no kind) is still dropped, which is what `🧫️fixtures/🌉️scene-bridge`'s `wireOnlyMeshesDropped` pins |
| `:10167` | `Meshes` phase: a geometry-less entry goes to `declare_scene_mesh_source` and the cursor advances — no `mesh3d_*` publication for it |
| `:9818` | new `declare_scene_mesh_source`: a `url` binds `id → url` in `mesh_source_urls` and stays PENDING; a `kind` resolves immediately through `WorldPlaceholderKind::resolve` on the same fixed-credit placeholder ladder inline buffers ride. Bounded by `WORLD_DYNAMIC_MESH_CAPACITY` rows and `WORLD_DYNAMIC_ID_BYTE_CAPACITY` per id/url |
| `:9837` | new `scene_mesh_awaits_its_asset` — the one predicate every stand-in site consults |
| `:10320` | `publish_world3d_scene_bridge_snapshot` keeps the draw for a mesh that is resident **or** awaits its asset. The draw is what the request loop reads, and what starts painting the frame the mesh lands |
| `:9635` | the typed-producer snapshot arm (`flags == 30`) no longer mints a placeholder box for a key the asset lane owes |
| `:6923` | new `offer_missing_mesh_fetches` — the per-frame offer, lifted out of `render_world_3d` so it is testable without a GPU context |
| `:6936` | new `reserve_world3d_mesh_fetch` — reserve, mark pending, treat back-pressure as back-pressure and a structural refusal as a miss (exactly `reserve_terrain_tile_fetch`'s split). `queue_lod_mesh_fetch` (`:6957`) now goes through it too |
| `:12899` | `publish_world3d_asset_mesh_lease` clears the url's pending mark on success |
| `:6895` | `sync_mesh_pool`'s eviction dropped `pending_glb_urls` by mesh KEY while the set is keyed by URL — a mesh evicted and then named again could never be re-fetched. It removes the url the ledger bound to the key now |
| `:12312` | new `ghost_mesh_id` (see below) |

**The extra rule.** `offer_missing_mesh_fetches` offers a url only while its key is NOT resident.
Any stand-in published under that key therefore silences the fetch *forever* — a placeholder is
strictly worse than drawing nothing. Two sites could do that and now do not: the snapshot arm above,
and the brush / catalogue-drop **ghost**, which keyed its translucent box by
`brush_preview_mesh_id(meshUrl)` = the same `mesh:…` id. React can afford its ghost's box because
it owns a separate three.js object; here one mesh table is keyed by mesh id. `ghost_mesh_id` draws
the ghost through the shared `box` primitive while the url is pending and switches to the real id
the frame its mesh lands — which is exactly what React's `BrushPreviewGhost` looks like either way.

**Why no re-stage is needed.** The draw exists from the first bridge pass, and `render_world_3d`
re-reads `state.mesh_versions` live per frame, so nothing has to invalidate `scene_bridge_digest`
when a GLB arrives.

### `pending_glb_urls` became a real ledger

The field existed with no inserter — a permanently empty set, the same "declared but never written"
shape that hid this bug. It is the lane's live predicate now: inserted on a successful reserve,
cleared on publication, on `mark_world3d_asset_miss` and on pool eviction. Without it the per-frame
offer re-walks the request table for every pending url on every frame.

## 3 — The frame: glTF is Y-up, the world is Z-up

React wraps every loaded GLB in `<group rotation={[GLB_MESH_FRAME_ROTATION_X, 0, 0]}>`
(`🎨️r3f/🟦️.tsx:310`, `= π/2`) — and applies the same rotation inside `extractGlbCollisionMesh`, so
the collision bodies an app registers for these meshes are already in that frame. The wgpu decoder
seeded its scene roots with the identity, so a building would have arrived lying on its side.

`🧊️renderer/🦀️.rs:2041` — new `glb_world_frame()`, the column-major `Rx(π/2)`; both root seeds
(`:1924` scene roots, `:1971` the node-less fallback) use it, and the now-unused `glb_identity` is
gone. Baking it into the decoded mesh rather than into each instance model keeps INLINE wire geometry
untouched, which is exactly the split React makes: only a GLB gets the frame.

## 4 — Transport URL: already correct on both lanes, now pinned

`/mesh/…` is a PUBLIC id; the file lives at the catalog's delivery path. React rewrites it in every
`useLoader` call. Both wgpu fetch lanes already did the same and I changed neither:

- browser — `🎞️frame-worker/🟦️.ts:566`, `fetch(meshAssetTransportUrl(request.url!))`
- native — `🧊️renderer/🦀️.rs:13617` `native_renderer_asset_path` → `mesh_assets::resolve_mesh_asset(url).source`

Nothing pinned that law, so it is pinned now (§5, `public_mesh_ids_reach_their_loader_…`), including
the "an unknown public id is returned unchanged" half — the resolver must never invent a filename.

## 5 — Tests (all RUN)

| test | what it pins |
| --- | --- |
| `world::tests::the_scene_bridge_keeps_url_and_kind_declared_meshes_with_their_instances` (`🌍️world/🧪️tests/🔬️unit/🦀️.rs:2693`) | the puzzle3d wire (kind + kind + url) keeps every draw and instance; `box` resolves, `mesh:🧊️left` is bound to its url and is deliberately NOT resident |
| `world::tests::a_url_declared_mesh_reserves_one_glb_fetch_and_draws_once_its_mesh_lands` | two offers → exactly one `Glb` request; a published lease lands under `mesh_id_from_url`, clears the pending mark, and the instance draws; a resident mesh is never re-fetched |
| `world::tests::a_refused_mesh_url_is_never_offered_again_by_its_surface` | `mark_world3d_asset_miss` clears the pending mark and ends the offer |
| `world::tests::a_ghost_never_stands_in_under_the_id_of_a_mesh_the_surface_is_still_fetching` | the ghost rule above, both directions |
| `async_boundary_tests::a_fetched_glb_becomes_the_resident_world_mesh_its_url_names` (`🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs`) | the whole renderer half over a surface's OWN lane: reserve → paged response → seal → probe → `publish_world3d_asset_mesh_lease` → resident with real positions/indices, and `(0,1,0)` arrives as `(0,0,1)` — the world frame |
| `async_boundary_tests::a_real_catalogued_glb_streams_through_the_surfaces_own_asset_lane_into_its_mesh_table` | a REAL export, not a fixture: 1 472 vertices / 5 250 indices / 1 472 uvs in 26 174 bounded probe steps, across many 16 KiB pages, inside every fixed credit on the way |
| `mesh_assets::tests::public_mesh_ids_reach_their_loader_through_the_catalog_transport_path` | the transport law of §4 |
| `async_boundary_tests::renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length` (updated) | compares the decode against `mesh_from_glb` **through** the world frame — and its pre-existing SIGABRT is fixed (below) |

**The SIGABRT W3a reported is gone.** That test's synthetic GLB declared no `"buffers"` array, so
`semio_framework::mesh_from_glb` answered `Err("gltf: buffer index 0 out of range")`, the `expect`
unwound through a non-unwinding `Drop` witness and aborted the whole binary, hiding every later test
in it. The fixture declares `"buffers":[{"byteLength":100}]` now and the oracle comparison — which
independently confirms this packet's rotation is exactly `(x, y, z) → (x, -z, y)` — actually runs.

### Commands

| command | result |
| --- | --- |
| `cargo check -p semio-framework-os-infinite --lib -j 4` | **0 errors**, no new warnings |
| `cargo check -p semio-framework-os-infinite --lib -j 4 --target wasm32-wasip2` | **0 errors** |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | **0 errors** |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4 --target wasm32-unknown-unknown` | **0 errors** |
| `cargo test -p semio-framework-os-infinite --lib -j 4 -- --test-threads=1 world::` | **149 passed, 7 failed** — the exact 7 `📓️w1f-world3d-tiledmap-wire-parity.md` lists as pre-existing (`live_renderer_retains_generation_wake_…`, `prepared_world_resources_are_send_…`, `world_authority_retains_front_plan_…`, both `world_component_marquee_…`, `world_object_registry_…`, `world_saturation_owner_…`). Re-run with this packet's four tests skipped: 145/7, same list |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 -- --test-threads=1 async_boundary_tests` | 24 passed **+ this packet's 2** ; 2 failed: `presenter_ack_retirement_source_mutations_are_denied` and `raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete`. Both are SOURCE-contract tests over `LIBRARY_SOURCE`/`DRAW_SOURCE`/`PREPARED_SOURCE`, neither mentions GLB, and the presenter one now requires a `glue.contains("runtime_presentation_authority_and_candidate_identity_change_independently")` whose symbol occurs **zero** times in the live `🧊️renderer/🦀️.rs` — a peer removed it. Not this packet |

Because the SIGABRT is gone, the renderer's `--lib` binary runs to the END for the first time:
`cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 -- --test-threads=1` answers
**771 passed / 33 failed** (`🗑️generated/w3d-renderer-tests.txt`), against
`📓️w1-integration.md` gate 6's 752/32. The 33 sit in `shell::`, `scenes::`,
`interpreter::`, `engine_canvas::paint2d` and the two source contracts above — peer lanes being
rewritten this session; none is in this packet's surface.

A note on the infinite suite: snapshot leases come from ONE process-wide fixed pool, so a test that
publishes a bridge snapshot and walks away starves later tests with
`World3dSnapshotFault::Unavailable`. All four new tests retire their surface
(`retire_bridged_surface`); without that teardown they flipped
`typed_camera_snapshot_matches_current_camera_fixture_…` red. Worth knowing before adding more
bridge tests.

## 6 — Two more defects the live boot found, both fixed

W3c's report landed and I took the activation. `SEMIO_RENDERER=wgpu NX_DAEMON=false bun nx run
@semio-tech/framework-renderer-wgpu:wasm --skip-nx-cache` then `…:activate-puzzle3d-wgpu-dev` —
the plain activate hits the Nx cache for the wasm task and does NOT pick up infinite-crate changes.

### 6.1 — The decode ladder runs inside the frame transaction, and the fetch lands after the last frame

First probe (🗑️generated/w3d-fetch-1): the bridge fix was live — `state-draws=1 state-instances=1`
where `📓️w3a-…` measured `0/0` — and the GLB was really requested and answered:
`REQUEST /mesh/🏚️abbau-aufbau/👈️hexagonal-cut-concrete-forest-left.glb` → `200`, **86 112 bytes** at
t=11 761 ms. The transport rewrite of §4 works on the live serve.

And then nothing, for 33 s. `pump_renderer_asset_decode_step` runs inside the frame transaction, so
on an event-driven shell it advances only while frames happen — and the boot's LAST frame was at
t=11 735 ms, 26 ms before the response. The seal's `wake` buys exactly one frame; a real mesh needs
tens of thousands of bounded decode steps.

| site | change |
| --- | --- |
| `♾️infinite/🌍️world/🦀️.rs` `WorldAssetIoAuthority::has_completed_step` | non-consuming twin of `take_next_completed_step` |
| `♾️infinite/🌍️world/🦀️.rs` `world3d_asset_decode_pending` | per-surface predicate over it |
| `🧊️renderer/🦀️.rs` `RuntimeMailbox::has_pending_asset_decode` | probe mid-flight, or an untaken sealed response on the shared authority or any World3d surface |
| `🌐️browser-worker/🦀️.rs:264` | it joins `request_frame`, beside `has_pending_world3d_work` |

### 6.2 — `world3d_scene_bridge_has_pages` carried the same residency filter as the publisher

A re-publish with an unchanged camera answered "no pages" for a scene whose only instanced mesh was
still pending, so the bridge completed without a snapshot and the draw was lost again
(`state-draws` 1 → 0 between two census lines, 🗑️generated/w3d-jiggle-1). Fixed at `:10299` with the
same predicate the publisher uses.

### 6.3 — A seal that raced a live apply quarantined the whole shell

With the mesh finally decoding, the SECOND World3d surface's copy failed 40 ms into its fetch:
`asset-stream-fault: asset response could not release its unused byte credits` → black canvas
(🗑️generated/w3d-fetch-3/final.png). That string names one of FOUR refusals `seal_renderer_asset_response`
collapsed into a single `false`. Making it answer why (`RendererAssetSealStep`) printed the real one
immediately: **`renderer holds no interaction state for the sealed response`** — `AppInteractionState`
was checked out by a live apply. Back-pressure, reported as a fatal fault.

| site | change |
| --- | --- |
| `🧊️renderer/🦀️.rs` | `seal_renderer_asset_response` and `reserve_renderer_asset_response` answer `RendererAssetSealStep::{Granted, Busy, Refused(detail)}`; both native callers carry the detail |
| `🌐️browser-worker/🦀️.rs` | `sealAssetResponse`/`reserveAssetResponse` return `false` on `Busy`, leaving the request exactly where it was; only `Refused` throws. The seal is skipped on retry when `WorldAssetFetchOwner::is_sealed` (new accessor) — the authority refuses a second seal |
| `🎞️frame-worker/🟦️.ts` | both are bounded retry loops over `macrotask()` (`ASSET_SEAL_ATTEMPTS = 64`) |

### 6.4 — Measured result

🗑️generated/w3d-final, both surfaces, no fault for 40 s:

- `asset mesh published url=/mesh/🧊️hexagonal-cut-concrete-forest-left.glb` — **twice**, once per surface
- `state-draws=1 state-instances=1 state-meshes=5` on `puzzle3d-main-top` AND `puzzle3d-main-perspective`
- `draws=1 translucent=0 instances=1` — the instance is submitted on both
- `dumpFrameStats` `sceneDraws` 2 → **3**, `sceneInstances` 1 → **2**
- the new resident-mesh census (below) reports
  `mesh:🧊️hexagonal-cut-concrete-forest-left:v847/i2874@[0.00,0.00,0.00]..[10.80,4.68,3.00]`

That bounding box is the proof of §3 on the live build: the GLB's own bounds are
`x 0..10.8, y 0..3, z -4.677..0`, and `(x, y, z) → (x, -z, y)` is exactly what landed.

**New permanent diagnostic.** `World3dState::mesh_geometry_census` (`🌍️world/🦀️.rs`, printed by the
per-frame `world3d surface=…` line in `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1967`) lists every resident mesh
with its vertex/index counts and local bounds. `state-meshes=5` cannot tell five real meshes from five
empty leases, and a draw that submits an empty GPU mesh paints nothing while every count above it reads
healthy — this session lost an hour to exactly that ambiguity.

## 7 — What still does NOT paint, and why it is NOT the mesh lane

`final.png` shows the full chrome (W3c's fix holds), both window frames, and the world grid — and no
mesh. The lane is complete up to submission; the last rung is in the world scene PASS:

- the mesh is RESIDENT with real geometry in the right frame (`v847/i2874`, bounds above);
- its draw is submitted on both surfaces (`draws=1 instances=1`, `sceneInstances=2`);
- the GPU mesh table HAS it at the requested version — `encode_prepared_world_instance` answers
  `Err("prepared world mesh was missing")` otherwise, which surfaces as
  `[DEBUG] os_host present_step faulted: …`, and there are **zero** such lines in any probe;
- in the same pass, on the same `view_proj` and the same scissor, the `world_line_pipeline` DOES
  paint: the LOD grid is the only thing visible in each pane, correctly centred.

So the world pass paints its LINE draws and neither its MESH nor its TEXTURED draws — the reference
underlay (which `📓️w3a-…` got as far as `textured=1`, decoded and admitted) is invisible for the same
reason. That is one renderer defect swallowing two lanes, not a mesh-lane gap. The suspect is the
depth/stencil state those three pipelines do NOT share: `world_pipeline` is
`depth_write_enabled: true, depth_compare: Less`, while `world_line_pipeline` is
`false`/`LessEqual` (`🖍️draw/🦀️.rs:2318`, `:2572`); every prepared world encoder attaches depth with
`LoadOp::Load` (`:3291`, `:3340`) and only `clear_prepared_scene` (`:3165`) ever clears it to `1.0`,
so a composite target that reaches these encoders without that clear rejects every `Less` fragment
and admits `LessEqual` ones at the cleared value. NOT verified — it is the next thing to test, with a
one-line `CompareFunction::Always` experiment. **Recommended as its own packet, with the renderer
owner**; it is the whole of "no 3D content" now that the lane feeding it is alive.

## 8 — Remaining gaps

1. **The world pass's mesh/textured draws** (§7) — the one thing between here and the React
   reference's grey slab.
2. **Frames no longer stop.** With §6.1 in place the probe recorded 336 frames over 45 s and the
   console never quiets. `has_pending_asset_decode` is only true while an asset is in flight, so the
   continuous ticking is something else (settle pump or world cursor work) that this packet made
   visible rather than caused — worth measuring before it is read as a regression.
3. **The LOD lane is still declared-only.** `mesh_lod_catalog`/`mesh_url_fallback` have no production
   inserter and `queue_lod_mesh_fetch` stays `#[cfg(test)]` — correctly, because React's
   `WorldMeshRecord` carries no LOD entries either, so there is nothing on the wire to read. It now
   shares `reserve_world3d_mesh_fetch`, so the day the wire grows a LOD lane it rides the same
   bounded path. A wire-shape packet, not a renderer one.
4. **Per-instance mesh material.** React's `GlbInstanceMesh` bakes colour, emissive, opacity, border
   and the `celebrated` shader into the cloned GLB scene; wgpu paints the instance tint only.
5. **`mesh_from_glb` and this decoder disagree about the frame.** The legacy oracle stays in raw
   glTF space; every wgpu world mesh is Z-up. The oracle has one production consumer left (the
   guest's collision bodies), and React already feeds THAT one rotated geometry
   (`extractGlbCollisionMesh`), so the oracle's own space is now the odd one out. Worth a ticket.
6. **The reference-image request loop** (`:10809`) still swallows its reserve result with `let _`,
   where the mesh and terrain loops distinguish back-pressure from a refusal. One line, W2a's lane.
7. **Probe scripts** added: `🐍️w3d-wgpu-mesh-fetch-probe.mjs` (every `/mesh/…` request with status,
   plus the console and the dumps; `SEMIO_PROBE_JIGGLE=1` keeps an event-driven shell ticking after
   it settles, which is what separates "the lane never asked" from "nothing drove the frames its
   decode needs").
