# P3m — Engine GPU Surface Authority Census

Date: 2026-08-23
Owner: `/root` coordinator
Verdict: bounded implementation packet prepared; source and runtime acceptance are pending.

## Current source boundary

The CPU-side engine registry and the general scene-state maps already use fixed slot authorities,
but the UI-capability presenter remains dynamically owned:

- `EngineCanvasPresenter` in
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs:419`
  owns `HashMap<String, EngineGpuSurface>`;
- `realize_one` checks `contains_key`, allocates `Texture`, `TextureView`, and Vello `Renderer`, then
  inserts a cloned surface ID into that unbounded map;
- resize replaces texture/view fields, while the old GPU owners have no presenter-owned retained
  retirement record at the map boundary;
- the presenter exposes no surface close/terminal-empty API and ordinary presenter Drop can release
  every map key, renderer, texture, and view in one destructor chain;
- `EngineCanvasBuildContext` owns an unbounded `Vec<EngineCanvasPacket>` and `take_packets` moves the
  whole vector; `enqueue` copies a dynamic surface ID and pushes without a fixed packet/byte
  admission; and
- the CPU `EngineSurfaceRegistry` uses `wrapping_add(1).max(1)`, so generation exhaustion can alias
  an earlier token rather than refusing reuse.

The relevant consumer is `PreparedFramePresenter` in the wgpu glue. It calls `realize_one` while
presenting prepared packets, but no paired presenter-surface close is visible in the exact caller
census. The existing `GpuContext` raster admission and retained allocation-fault paths are valuable
and must remain the sole GPU allocation gate.

## Required implementation

### Fixed generation-keyed GPU slots

Replace the presenter `HashMap` with an owned fixed slot authority aligned with the admitted engine
surface limit and surface-ID byte cap. Each slot owns the ID backing, nonzero checked generation,
optional live `EngineGpuSurface`, and optional retained retirement. Reservation happens before any
ID copy or GPU allocation. Full, oversized, closing, stale, duplicate, and exhausted-generation
rejections return the exact producer owner and do not allocate a texture/view/renderer.

Do not use standard-map node-size estimates or other external layout details. Slot/control backing
is an explicit fixed owner. Surface IDs use fixed storage or an observed-capacity allocation that is
admitted and exactly retired. `checked_add` refusal is required; wrapping/saturating token reuse is
forbidden.

### Staged allocation and publication

GPU allocation remains UI-thread-only and uses the existing `GpuContext` admission witness. New
surface creation and resize advance through a persistent allocation state:

1. reserve the fixed presenter slot and exact texture byte/extent authority;
2. validate candidate/expected generation immediately before each GPU allocation;
3. allocate texture, then view, then renderer as separate retained ownership stages;
4. validate freshness immediately before atomic publication;
5. publish the complete surface in one swap; and
6. move every displaced texture/view/renderer/control owner into retained GPU retirement.

Any validation error, device error, stale candidate, cancellation, panic, or publication rejection
must keep the exact allocated owner reachable from `GpuContext` or the presenter retirement slot.
No branch may rely on a local destructor to release a partially constructed GPU owner.

Rendering and staging the published raster must also preserve the prior valid texture until the new
render has passed freshness and staging. A failed render/stage cannot leave a replacement shell in
the live slot or lose the former surface.

### Bounded packet authority

Replace `EngineCanvasBuildContext.packets: Vec<_>` with fixed/page packet admission. Enqueue must
preflight item count, aggregate bytes, surface-ID bytes, scene owner, and output slot before transfer.
Packet consumption advances one retained packet/page/control owner per presenter grant. Taking
packets may not move an unbounded vector, and close must drain queued, blocked, presenting,
rejected, and completed-but-unclaimed packets with exact owner identity.

The accepted prepared-scene and raster producer authorities remain upstream; this packet governs
their EngineCanvas packet envelope and GPU-surface lifetime rather than cloning or repacking their
payloads.

### Retained close

Add a generation-token surface close API paired with CPU surface close and renderer/window/app
close. Beginning close freezes reservation and publication for that token. Each grant retires at
most one ID backing, Vello renderer owner, texture owner, texture view owner, pending allocation,
staged raster, packet, or fixed control slot. Terminal success requires an explicit nonopaque-empty
witness and invalidates the token before slot reuse.

Presenter Drop must fail closed while any live/pending/retiring owner exists. It must not iterate or
deep-drop all surfaces. Device-loss and app-close paths use the same disposer.

## Hostile fixtures and verifier mutations

Permanent source fixtures and verifier self-mutations must cover:

- exact surface count, ID bytes, packet count, packet bytes, texture bytes, and extent maximum/+1;
- exact rejected packet/ID/slot owner identity and zero producer invocation before preflight;
- slot reuse, stale/duplicate/wrong token, checked-generation exhaustion, and ABA;
- stale freshness before texture, view, renderer, render, publication, and stage;
- texture success/view failure, view success/renderer failure, renderer success/render failure,
  render success/stage failure, and panic at every ownership seam;
- resize preserving the old live surface until atomic publication;
- cancellation and device loss before and after every stage;
- zero fuel, insufficient fuel, expired deadline, interrupted close, and one-owner-per-close-turn;
- completed-but-unclaimed surface and packet retirement;
- no `HashMap<String, EngineGpuSurface>`, no unbounded packet `Vec`, no direct `insert`, no
  `wrapping_add`/saturating generation, and no cancel-by-Drop; and
- deterministic native/browser-Wasm surface reuse and close under repeated resize and window churn.

## Acceptance gates

Source handoff requires scoped edition-2021 `rustfmt --check`, permanent verifier self-tests, live
source predicates, deterministic ledgers, exact caller/owner scans, and scoped/whole
`git diff --check`. The serialized final lane must execute debug/release/strict-warning builds,
native and both Wasm targets, a real browser worker, device-loss/resize/window-close stress,
allocation traces, and max/p99 timing proving every UI presenter callback and worker grant stays
below the 8 ms ceiling.

This census is not an acceptance claim. Phase 3 remains open.
