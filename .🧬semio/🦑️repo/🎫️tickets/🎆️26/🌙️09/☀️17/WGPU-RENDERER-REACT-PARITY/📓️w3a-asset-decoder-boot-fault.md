# 🧊️ W3a — the asset-decoder boot fault (and the two faults behind it)

Packet W3a of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Every line number is post-edit; every claim
below is backed by a probe output or a test that was RUN, named inline.

**Result: the puzzle3d wgpu playground boots fault-free for the whole 75 s probe window.** The
reported fault is gone, and so are the two faults it was hiding. The per-frame `[DEBUG]` census is
now gated behind `SEMIO_RUNTIME_DIAGNOSTICS`. The scene still paints NOTHING on the canvas, for a
reason that is neither of these and is stated precisely in §6 — the wgpu World3d bridge drops every
mesh the puzzle3d wire declares by URL.

---

## 1 — Fault 1: the reference underlay was refused, and the refusal quarantined the surface

`frame fault recorded: asset retained structure decoder rejected malformed input`
(🗑️generated/wgpu-boot-2/console.txt:81, t=6442 ms).

The asset is the reference plan the puzzle3d play app publishes in its own `references` lane —
`/infinite-assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg`, asserted by the plugin's own law
`world_references_json_carries_infinite_asset_urls_the_dev_server_can_serve`
(`✏️s/🔌️plugins/🧩️puzzle/…/🪟️windows/🧊️main/🧪️tests/🔬️unit/🦀️.rs:117`). It is **2275×2560**, i.e.
**23 296 000 bytes** of straight RGBA, and `RENDERER_ASSET_PIXEL_BYTES` was **16 MiB**
(4 megapixels). `JpegStructureCursor::feed` therefore refused it at its SOF0 marker — byte 31 565 of
483 496, which is exactly the ~124 parse blocks the 2.8 s between the world3d ingest and the fault
buys at one 256-byte block per frame.

Two defects, not one:

1. **The ceiling was below the product's own content.** React's `WorldReferenceLayer` hands the same
   file to the browser decoder and paints it, so the scene lost its reference plane on wgpu only.
2. **A data-shaped refusal was escalated to a frame fault**, which quarantines the surface and stops
   input — for one texture. Worse, both structure-decoder sites discarded the cursor's own detail and
   reported one generic string, which is why the fault could not be read off the console at all.

| site | change |
| --- | --- |
| `📺️renderer/…/🧊️renderer/🦀️.rs:323` | `RENDERER_ASSET_PIXEL_BYTES` 16 MiB → **64 MiB** (16 megapixels = the 4096² surface every backend in `🖌️render/🔌️backend` guarantees), docstring names the asset and the boot fault |
| `🧊️renderer/🦀️.rs:2589` | new `RendererAssetProbeStep::Reject(&'static str)` beside `Fault`: `Reject` is a verdict about the RESPONSE BYTES (wrong format for the kind, truncated structure, over-budget dimensions), `Fault` stays a verdict about the RENDERER (a page cursor that lost ownership, a sealed byte witness that disagreed, a stale mesh publication) |
| `🧊️renderer/🦀️.rs:2654`, `:2671`, `:2688`, `:2727`, `:2734` | the format probe, both structure-decoder sites and the GLB schema step answer `Reject` **carrying the decoder's own detail** (`"JPEG dimensions exceeded fixed pixel credits"`, not `"…rejected malformed input"`) |
| `🧊️renderer/🦀️.rs:2750` | new `RendererAssetProbe::reject` captures `(detail, kind, url)` BEFORE `begin_close` clears the url, so the requesting lane can record its own miss |
| `🧊️renderer/🦀️.rs:10390` | new `record_rejected_asset`: the shared kinds record their miss immediately (`apply_ui_image_bytes`/`apply_map_tile_bytes` with empty bytes is the miss those lanes already speak), a World-owned kind carries its rejection to `finish_renderer_asset_owner` — the one place holding that surface's state — which marks it there. The refusal prints unconditionally (`asset refused: <detail> — url=<url>`): an asset that never paints is a defect report, not a trace |
| `♾️infinite/🌍️world/🦀️.rs:12671` | new `mark_world3d_asset_miss`/`world3d_asset_url_missed` + `asset_url_misses`, consulted by both request loops (`missing_mesh_urls` `:10556`, `reference_image_urls` `:10702`). Without it a refused url is re-fetched every frame forever, because both loops re-derive their missing urls from the scene each frame. Same shape as the existing `terrain_tile_misses` and as React's loader caches |

## 2 — Fault 2: `collect_world3d_asset_bytes` never advanced its page cursor

With the ceiling raised, the boot died ~3 s later on `capacity overflow` in `RawVec`
(🗑️generated/w3a-1/console.txt:79). The panic's own location names the allocator, so the hook was
extended to mint a JS `Error` and print its stack (`🌐️browser-worker/🦀️.rs:708`) and the frame worker
now raises `Error.stackTraceLimit` to 64 (`🎞️frame-worker/🟦️.ts:246`) — V8's default ten frames are
spent entirely inside the panic machinery. With two `[DEBUG]` traces around the apply the panic
localized between `asset ready` and `reference decode begin` (🗑️generated/w3a-2/console.txt:77-79),
which is `collect_world3d_asset_bytes`:

```rust
while let Some(page) = owner.decode_page()? {   // 🩸️ never advanced
    bytes.extend_from_slice(page.bytes());
}
```

It re-read page 0 forever and grew the payload until `RawVec` refused a capacity past `isize::MAX`
(wasm32 is 32-bit). It had never run in production: every lane that calls it was dead until
2026-09-17 (W1f terrain, W2a ui-image/map-tile, and the reference-image lane never got past the
probe). It is a **bounded walk over the sealed page index** now
(`♾️infinite/🌍️world/🦀️.rs:12734`), so the loop's length is the response's own page count.

## 3 — Fault 3: the decoded underlay did not fit the raster lane's credits

Next boot died on `frame world resource admission exceeded fixed credits`
(🗑️generated/w3a-3/console.txt:89, t=7103 ms), 400 ms after the decode completed:
`PreparedRasterProducer::try_admit` refuses a source over `PREPARED_RASTER_ITEM_BYTES` (16 MiB) and
`World3dResources::ensure_world_plane_texture` turns that refusal into a build fault. 23.3 MB of RGBA
cannot enter the lane at all.

Fixed at the producer, not by raising the lane: a reference underlay is scaled into the lane's own
credits (`♾️infinite/🌍️world/🦀️.rs:12788` `WORLD_REFERENCE_TEXTURE_BYTES` = **half** of
`PREPARED_RASTER_ITEM_BYTES`, because the producer ledger reserves a source twice over — source plus
peak workspace — against a 32 MiB ledger; `bounded_reference_image` scales aspect-preserving with a
triangle filter). The plane it textures never covers more than a viewport, so this is a texture LOD,
not a dropped feature. `PREPARED_RASTER_ITEM_BYTES` became `pub`
(`🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:339`, re-exported `🎯️targets/🧊️wgpu/🦀️.rs:337`) so the
producer sizes its decode against the real ceiling instead of a copied number.

## 4 — Throughput: one asset unit per frame was a stall, not a budget

The frame transaction's fuel is 1, so a single `Pending` bought exactly one 256-byte structure block,
one response page, or one GLB vertex — measured at ~55 units/s, i.e. 1 889 blocks for the underlay
alone and ~7 000 vertices per concrete-forest mesh. Nothing would have appeared for minutes. The
asset lane now spends the wall slice it was granted and yields on the DEADLINE
(`🧊️renderer/🦀️.rs:11838`), which is what the `INTERACTIVE_LANE_WALL_US` bound is for; each unit is a
bounded byte-wise scan. Measured effect: the underlay reaches `Ready` at t≈3.7 s instead of never
(🗑️generated/w3a-2 vs w3a-4).

## 5 — The per-frame `[DEBUG]` dumps are gated now

`SEMIO_RUNTIME_DIAGNOSTICS` was already the ONE switch (`⏱️trace/🦀️.rs:164`,
`runtime_diagnostics_enabled`), read from the process environment natively and from `localStorage` by
React's `ShellHost` — but nothing armed it in the renderer wasm, and a Worker realm owns no storage.
Wired end to end:

- `🚀️browser-boot/🟦️.ts:398` stamps the frame Worker's url through the existing
  `stampShardWorkerDiagnostics` (the UI isolate is the realm that holds the preference).
- `⏱️turn-budget/🟦️.ts:61` `TURN_DIAGNOSTICS_PARAM`/`stampedTurnDiagnostics` read that stamp without
  touching storage (this module is bundled into the frame worker, whose carrier census forbids it).
- `🎞️frame-worker/🟦️.ts:239` arms the UI-turn ledger and hands the answer to the renderer through the
  new `semioWgpuSetRuntimeDiagnostics` export (`🧊️renderer/🦀️.rs:15298`), BEFORE the first frame.
- Gated sites: `frame build admitted` (`🧵️frame-job/🦀️.rs:697`), `os_host frame gate`
  (`🪟️winit-app/🦀️.rs:239` via the new `log_debug_diagnostic_once_per_transition`),
  `wgpu-shell engine surfaces` (`🐚️Shell/…/🦀️.rs:5723`), the per-frame `world3d surface=…` census
  (`🎞️Scenes/…/🦀️.rs:1966` via a new `debug_log_diagnostic`, `:1989`), and the asset lane's own new census
  lines. New helper `log_debug_diagnostic` (`🧊️renderer/🦀️.rs:9083`).

Measured: **90 console lines in 75 s** unarmed (🗑️generated/w3a-5) against **1 288 lines in 35 s**
armed (🗑️generated/w3a-diag-1) with the whole per-frame census back. A real error is now visible.

## 6 — What still does NOT paint, and why it is not the asset lane

`final.png` is BLACK, headless and headed alike (🗑️generated/w3a-4/final.png,
🗑️generated/w3a-diag-headed/final.png — a headed run rules out a capture artifact). The introspection
oracle (🗑️generated/w3a-diag-1/dumps.json) says why:

- `dumpFrameStats`: `{"drawCalls":0,"quadCount":17,"glyphCount":0,"scenePasses":1,"sceneDraws":2,"sceneInstances":1}`
  — frames ARE built (121 admitted builds in 35 s) and the reference plane is in the textured lane
  (`world3d surface=… textured=1`), but nothing is drawn or presented, and no glyph is rasterised.
- `dumpMeshStats`: all four meshes carry `indices:0, positions:0` — including
  `mesh:🧊️hexagonal-cut-concrete-forest-left/right`.

Root cause of the empty 3D, found and NOT fixed here (it is a wire-level packet, not an asset one):
**the wgpu World3d scene bridge accepts only INLINE mesh buffers and drops every mesh the wire
declares by URL.** `World3dSceneBridgePhase::Parse`
(`♾️infinite/🌍️world/🦀️.rs:10030`) does
`cursor.meshes.retain(|mesh| mesh.data.vertex_count() > 0 && mesh.data.indices.len() >= 3)` and then
`cursor.instances.retain(|instance| cursor.meshes.iter().any(|mesh| mesh.id == instance.mesh_id))`,
so puzzle3d's `{"id":"mesh:🧊️…","url":"/mesh/🧊️….glb"}` entries and their instances are discarded —
hence `state-draws=0 state-instances=0` on both surfaces while the wire carries one instance. The
consequence is confirmed structurally: `mesh_source_urls` and `pending_glb_urls` have **no production
inserter anywhere in the repo** (only reads at `:10556`, removals at `:6896`), so
`WorldAssetRequestKind::Glb` is never reserved and no probe ever sees a GLB. React's `World3dHost`
loads those urls with `GLTFLoader`. The URL half of the mesh lane is dead, exactly like the three
asset lanes W1f/W2a revived.

`drawCalls:0`/`glyphCount:0` for the CHROME is a second, separate finding: the shell builds 17 quads
for the focused window and presents none. Recommended as its own packet — it is why the canvas is
black rather than merely empty of 3D.

## 7 — Tests

| test | what it pins |
| --- | --- |
| `async_boundary_tests::retained_image_decoder_admits_a_reference_plan_and_rejects_a_pixel_bomb_without_faulting` (`🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:449`) | a 2275×2560 plan decodes through BOTH image scanners and reaches `Ready` through the production probe; a 4097×4097 bomb answers `Reject("JPEG dimensions exceeded fixed pixel credits")` — never `Fault` — and its rejection carries kind + url captured before `begin_close` cleared them |
| `async_boundary_tests::renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length` (updated) | a malformed GLB length is now a `Reject`, not a `Fault` |
| `world::tests::collecting_a_multi_page_asset_response_answers_each_page_once` (`♾️infinite/…/🔬️unit/🦀️.rs:2588`) | the page-cursor bug: three pages collect to exactly seven bytes, twice, and the cursor is left rewound |
| `world::tests::a_reference_underlay_is_scaled_into_the_raster_lanes_own_credits` | an over-budget underlay is scaled inside `PREPARED_RASTER_ITEM_BYTES / 2` with its aspect intact; an in-budget image is untouched |
| `world::tests::a_refused_asset_url_is_never_offered_again_by_its_surface` | both request loops offer a url once and never again after `mark_world3d_asset_miss` |

VERIFY (all run, this packet's own code):
`cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` **0 errors**;
`--target wasm32-unknown-unknown` **0 errors**;
`cargo check -p semio-framework-ui --features wgpu-engine --lib` **0 errors**;
`cargo check -p semio-framework-os-infinite --lib` **0 errors**;
the five tests above **5 passed / 0 failed**;
wasm activation `activate-puzzle3d-wgpu-dev` **exit 0** (🗑️generated/w3a-activate-3/-5/-6.txt).

⚠️ The renderer's `--lib` test BINARY still aborts (SIGABRT) inside
`renderer_asset_probe_keeps_pages_owned_…`: `semio_framework::mesh_from_glb` — the legacy glTF oracle
that test compares against — returns `None` for its own synthetic GLB, and the unwinding trips a
`Drop` witness, which is non-unwinding and kills the process. Pre-existing (it is one of the 32
failures 📓️w1-integration.md gate 6 reports, and it is in code this packet did not touch), but it
hides every later test in that binary, so each test here was run with `--exact`.

## 8 — Probe evidence

| output | what it shows |
| --- | --- |
| 🗑️generated/wgpu-boot-2 (baseline) | `asset retained structure decoder rejected malformed input` at 6 442 ms, surface quarantined, input dead |
| 🗑️generated/w3a-1 | asset fault gone; `capacity overflow` panic at 10 713 ms |
| 🗑️generated/w3a-2 | the panic localized between `asset ready` (t=3 722 ms) and `reference decode begin` |
| 🗑️generated/w3a-3 | underlay decodes (377 ms + 303 ms in wasm); `frame world resource admission exceeded fixed credits` at 7 103 ms |
| 🗑️generated/w3a-4 | **no fault for 75 s**; `world3d delivery applied … state-meshes=1` on both surfaces; `textured=1` |
| 🗑️generated/w3a-5 (final build) | **no fault, no panic, no page error**; 90 console lines in 75 s |
| 🗑️generated/w3a-diag-1 | diagnostics armed: 1 288 lines in 35 s, per-frame census back, introspection dumps |
| 🗑️generated/w3a-diag-headed | headed run, still black → not a headless capture artifact |

Probe scripts: `🐍️wgpu-console-dump-probe.mjs` (the coordinator's) and the new
`🐍️w3a-wgpu-diagnostics-probe.mjs` (arms `SEMIO_RUNTIME_DIAGNOSTICS` in `localStorage` before load and
dumps `semioWgpuIntrospection.dumpStructure/dumpFrameStats/dumpMeshStats`; `SEMIO_PROBE_HEADED=1`
runs it headed).

## 9 — Hand-offs

1. **P0 — the URL half of the World3d mesh lane is dead** (§6): the bridge drops url-declared meshes
   and their instances, and nothing inserts `mesh_source_urls`, so no GLB is ever fetched. This is the
   whole of "no 3D content" on every World3d playground, not just puzzle3d.
2. **P0 — the shell presents nothing** (§6): 121 frame builds, `drawCalls:0`, `glyphCount:0`, black
   canvas.
3. **P1** — a refused raster admission (`World3dResources::rejected` → `append_step` → build fault)
   still quarantines the surface for one texture. With §3 in place nothing crosses it, so it now
   means a renderer-side credit bug, but the same demotion §1 applied to the decoder belongs here.
4. **P2** — the reference decode costs 300 ms–2.2 s inside ONE asset step (decode plus scale), which
   no deadline can preempt. A stepped decode (or a scaled decode at source) is the clean fix.
5. **P2** — the library door `🎬️renderer-boot/🟦️.ts` constructs no frame Worker of its own, so it does
   not carry the diagnostics stamp; only the trunk page arms diagnostics today.
