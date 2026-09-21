# Checkpoint 18 World Mesh Latency

## Scope

This read-only packet reconciles the recorded checkpoint-18 WGPU console, its
`12-settings-general.png` screenshot, and current source. The current source has
changed after the capture: its frame worker now constructs `WorkerTurnTaskQueue`.
The capture's timing is evidence about the old activated runtime only.

## Finding

The concrete-forest mesh is neither absent nor first made resident at 571.865 s.
The browser takes far too long to reach the *requestable* scene state, but after
the GLB probe reaches `Ready`, it is published immediately and becomes visibly
present shortly after.

The causal boundary is before the GLB request. URL meshes are requested only by
`render_world_3d` after the retained World scene bridge has produced
`state.draws`. That bridge advances one bounded `World3dSnapshot` transaction
turn at a time. The checkpoint's old Worker turn transport starved those turns.
The later generation-10 `Uploads` delay is real but not the cause of this mesh's
first availability: the mesh was already resident and drawn before it.

## Recorded lifecycle

All numeric prefixes are host-observed elapsed milliseconds since the probe's
post-page-creation `Date.now()` baseline in
`🗑️generated/astra-runtime/checkpoint-18-main/wgpu/console.txt`.

| Time | Receipt | Meaning |
| ---: | --- | --- |
| 9.495 s | Top World `meshesHead` names both forest URLs; `state-draws=0`, `state-instances=0`, `state-meshes=0`, `bridge=true` | The document declares the left URL. This is not an asset-request receipt. |
| 10.518 / 12.626 s | Two `ReferenceImage` assets report ready | The reference lane is live; neither line proves a GLB request. |
| 62.890–63.114 s | Generation 2 is blocked in `Ownership`, clears, then generation 3 is admitted | Retained frame progression already has a long pause. |
| 188.493–188.653 s | Generation 8 clears `Render`; generation 9 is admitted | Immediate predecessor to the usable World draw. |
| 189.918 s | Top has `state-draws=1`, `state-instances=1`, still no forest mesh | Earliest recorded state able to offer the GLB. The unlogged reserve is bounded by 189.918–191.003 s. |
| 191.003 / 191.690 s | Each surface logs `asset ready kind=Glb`, 86,112 bytes; `asset mesh published` follows at 191.005 / 191.690 | Completed GLB probe hands the lease to World state in at most 2 ms. |
| 194.313 s | Generation 10 admitted | Fresh render opportunity after publication. |
| 205.259 / 206.539 s | Top/Perspective each have `draws=1`, `instances=1`, `state-meshes=4`, forest `v847/i2874` | The real mesh is emitted by both World scenes. |
| about 215.9 s | Settings interaction; `wgpu/12-settings-general.png` shows the large grey forest mesh in Perspective | Physical presentation witness. This screenshot does not lack the mesh. |
| 259.787 s | Generation 10 blocked in `Uploads` | Later independent frame-liveness delay. |
| 571.865 s | Same `draws=1`, `state-meshes=4`, `v847/i2874` receipt | Repeated resident census, not first appearance. |

No relevant line reports `prepared world mesh was not resident`, a rejected asset,
or a World fault. The censuses say `fault=None`.

## What `asset ready` proves

`RendererAssetProbe::step` advances a GLB through `Reading`, `Parsing`,
`Semantic`, and `Materializing` before `Ready`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:2655-2823`).
Thus 191 seconds is a completed retained decode/materialization receipt, not an
HTTP-response timestamp. The capture lacks request-start, response-seal, and
per-probe phase timestamps, so it cannot divide 189.918–191.003 seconds between
network and decoding.

It rules out publication and GPU residency as the large delay: the ready probe
publishes its exact `Mesh3dLease` in
`RuntimeMailbox::pump_renderer_asset_decode_step`
(`…/🧊renderer/🦀️.rs:11324-11349`) without a queued replacement stage.

## Source-level causal chain

1. The bridge records a URL entry in `mesh_source_urls` during its `Meshes`
   phase but deliberately does not reserve it:
   `🧰️framework/🔨️modules/♾️infinite/🌍️world/🦀️.rs:12172-12228`.
2. `offer_missing_mesh_fetches` runs only in `render_world_3d`. It filters
   `state.draws` for nonresident mesh keys with a known source URL
   (`…/🌍️world/🦀️.rs:8059-8074,12716-12760`). A declared URL cannot issue a
   GLB request until bridge and snapshot apply have created that draw.
3. `AppFrameTransactionPhase::World3dSnapshot` advances one bridge and returns
   `Pending` on `World3dSceneBridgeStep::Pending`
   (`…/🧊renderer/🦀️.rs:13114-13205`). `world3d_cursor_work_pending` includes
   `scene_bridge` and `snapshot_apply`
   (`…/🌍️world/🦀️.rs:2485-2499`).
4. The old capture's Worker did not provide the owed turns promptly. The
   43.64-second generation-10 gap is independently recorded in
   `📓️astra-sol-checkpoint18-frame-liveness.md`. Current source uses
   `WorkerTurnTaskQueue` instead of the old timer-backed frame scheduler; a fresh
   activated run must measure the replacement.
5. The renderer holds one active `RendererAssetProbe` and drains bounded work in
   the transaction (`…/🧊renderer/🦀️.rs:11223-11349,12849-12858`). That can affect
   multi-asset fairness, but this capture contains no receipt that it caused the
   roughly 180-second pre-request wait.

## Correct classification

- **Confirmed live latency:** boot to first usable World draw is about 205 seconds.
  The URL cannot be requested for most of that time because its scene draw is not
  yet published.
- **Confirmed prompt post-ready behavior:** ready to publication is at most 2 ms;
  ready to scene draw is about 14–15.5 seconds.
- **Confirmed separate liveness defect:** the old Worker has a 43.64-second
  no-turn gap with retained `Uploads` pending. This is Sol's frame-owner packet,
  not an asset decoder defect.
- **Not established:** a slow HTTP request, a slow GLB decoder, an upload stall as
  the cause of first mesh absence, or a 9.5-minute residency delay.

## Fail-first receipt for the next activation

Extend the existing real URL-mesh lane with exact receipts at
`declare_scene_mesh_source`, `reserve_world3d_mesh_fetch`, browser request poll,
response seal, probe ready, publication, and first presented draw. The law is:

> Once a live World URL mesh has a bridge-published draw, it reserves exactly one
> matching GLB request. A sealed current response reaches ready, publication, and a
> presented draw without another input or document event; a stale response never
> publishes into a successor surface.

Use the actual URL-mesh lane in `📓️w3d-world3d-glb-url-lane.md` and the retained
`Uploads → Finish → Terminal` liveness fixture in
`📓️astra-sol-checkpoint18-frame-liveness.md`. Do not impose a wall-clock limit
until a fresh run on the repaired Worker gives one.

## Confidence

- **High:** recorded order from scene draw to probe-ready to publication to first
  rendered mesh, and the correction of the 571.865-second interpretation.
- **High:** URL fetch admission is draw-gated by current source.
- **Medium:** old Worker continuation starvation accounts for the long bridge wait;
  the capture lacks per-bridge-step timestamps.
- **Low:** a finer split of the 189.918–191.003 interval between transport and GLB
  decode; required instrumentation is absent.

## Retracted Clock-Basis Correction

The following source-path reconstruction remains useful, but the prior claim
that the console was deleted and its prefix clock unverified was false. The
superseding receipt correction below establishes the host-observed clock and
the exact ready, publication, and first-draw receipt separations.

The current browser path is:

1. A real render_world_3d pass turns a missing draw URL into a World claim in
   world line 8059, then the Worker adopts its exact owner in browser-worker
   line 129.
2. pumpAsset polls that owner, begins fetch, reserves response capacity, reads
   bounded BYOB pages, and seals the owner in frame-worker line 676. Its fetch,
   each stream read, and lock-backpressure retry are external parked
   suspensions; they are not decode turns.
3. The sealed owner is selected into the one active RendererAssetProbe at
   renderer line 11223. Its nonterminal phase sequence is Reading, Parsing,
   Semantic, Materializing, and Ready at renderer line 2647.
4. Ready moves the exact Mesh3dLease into the live surface using
   publish_world3d_asset_mesh_lease; a later render pass submits it at renderer
   line 11338.

A fresh probe needs one Worker-realm monotonic performance-now timestamp in an
explicit asset-stage receipt at: adopted request, fetch dispatch, response
headers, successful reserve, sealed response, probe Ready, mesh publication,
and the first render pass that submits the matching mesh. Each record must
carry request token, URL, kind, surface token, and stage. The test runner
should compare only timestamps from that receipt stream; it must not subtract
DevTools console collection times or timestamps from another realm.

This audit did not run a fresh activation. The preceding clock-basis correction
was based on a mistaken file lookup and is superseded below.

## Superseding Console Receipt Correction

The preserved WGPU console exists at
`🗑️generated/astra-runtime/checkpoint-18-main/wgpu/console.txt` and is
4,176,838 bytes. The probe creates `t0 = Date.now()` immediately after
`browser.newPage`, and `record` writes `Date.now() - t0` for both the page
console listener and every Worker console listener
(`🐍️parity-interact-probe.mjs:350-360`). Each numeric prefix is therefore a
verified host-observed elapsed-millisecond receipt. It is not an exact duration
of the Worker code which emitted the line: browser delivery, the Node event loop,
and any interval without a receipt are included. The duplicate `worker` and
`page` lines are two host observations of one Worker console event and are not
two pipeline transitions.

The first complete left-forest lifecycle has these unique Worker receipts:

| Host elapsed time | Surface / receipt | Exact observed interval |
| ---: | --- | --- |
| 189,918 ms | Top World has `state-draws=1`, `state-instances=1`, and three resident meshes, but `draws=0` | First recorded requestable top state. |
| 191,003 ms | `asset ready kind=Glb`, left forest, 86,112 bytes | 1,085 ms after that top-state receipt. This interval includes unlogged request, external fetch/stream/seal, scheduling, and decode/materialization work. |
| 191,005 ms | `asset mesh published`, left forest | 2 ms host-observed receipt separation from that ready line. |
| 191,243 ms | Perspective World has the same requestable state | First recorded requestable Perspective state. |
| 191,690 ms | `asset ready` and `asset mesh published`, left forest | Both receipts have the same host millisecond; their order remains line order. |
| 205,259 ms | Top World first reports the concrete forest as `draws=1`, `instances=1`, `state-meshes=4`, `v847/i2874` | 14,256 ms after Top Ready; 14,254 ms after Top publication. |
| 206,539 ms | Perspective first reports the same forest mesh emitted | 14,849 ms after Perspective Ready/publication. |

The console contains no request-dispatch, HTTP response, response-seal, or
probe-phase receipt for this GLB. Current source establishes their order
(`offer_missing_mesh_fetches` creates the claim during `render_world_3d`; the
Worker polls it; `pumpAsset` fetches and seals; the renderer probes to Ready),
but the capture cannot partition either 1,085 ms or 447 ms into those stages.
It also cannot partition the roughly 14-second ready-to-first-draw intervals
between later scheduled work, GPU submission/presentation, and console delivery.

There is direct execution evidence only for individual sampled frame turns:
the 189,918-ms worker receipt says `executing=11.400ms` for one `frame-step`.
That establishes neither the duration nor the cause of the adjacent unlogged
pipeline intervals. The authoritative classification is host-observed latency
with an uninstrumented stage split, not a measured HTTP, decode, or GPU duration.
