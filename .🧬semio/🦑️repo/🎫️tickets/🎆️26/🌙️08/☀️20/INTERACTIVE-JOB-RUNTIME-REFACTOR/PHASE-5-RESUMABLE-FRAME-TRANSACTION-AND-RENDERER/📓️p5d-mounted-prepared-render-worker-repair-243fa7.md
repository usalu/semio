# P5d Mounted Prepared-render Worker Repair Contract — 2026-08-23

## Status

**RED contract prepared for implementation.** The mounted OS renderer has a useful retained frame
preparation and presentation seam, and the independently accepted paged-raster sub-seam remains
GREEN. Generic draw, atlas, upload, tessellation/batching, command encoding, and present ownership do
not yet satisfy P5d. This is a read-only coordinator census; no Cargo, Nx, Wasm, browser, runtime,
timing, or network gate was run.

## Exact Mounted Route

The product path is `OsHost::redraw_core` -> frame worker -> `AppFrameTransaction` ->
`AppFrameBuild::into_preparation` -> `PreparedRenderJob` -> capacity-one prepared packet ->
`AppPresenter::present_step` -> `GpuContext`. Revision/generation admission, last-valid preservation,
presentation ACK/abort, raster witnesses, and incremental paged-raster retirement are already mounted.

The separate public `ui_render::FrameEngine::build_frame` -> `Scene::finish` family has no non-test
caller. It still performs compose/layout/prepaint/paint and validate/snap/order/batch/hash
synchronously. P5d must consolidate or delete that duplicate public family after parity; leaving an
unmounted run-to-completion alternative is not acceptance evidence.

This packet depends on independently accepted P2a1 worker sessions and the accepted P3/P5 raster
page protocol. It must reuse the process `WorkerPool`; no renderer pool, private executor, blocking
channel, UI-thread job driver, or run-to-terminal adapter is allowed on the mounted route.

## Current Source Defects

### Input is materialized before aggregate admission

`PreparedRenderInput::new` accepts an already-owned `DrawList`/overlay and allocates dynamic upload
capacity. Callers later push damage, clips, directives, evictions, producers, mesh leases, and atlas
vectors. `PreparedRenderJob` only walks that completed graph and compares saturating measured usage
against defaults of 262,144 draw items/64 MiB and 256 uploads/32 MiB. The allocation and ownership
transfer have already happened; max + 1 faults without returning the exact producer owner.

`DrawList` is a nested graph of standard vectors and strings. Measurement uses `len`, not actual
capacity or nested owner/control allocation, and several headers account many elements as one
semantic item. A measured credit is not a reservation and cannot prevent later allocation.

### A worker grant is not one semantic unit

`AppFrameBuild::into_preparation` hard-codes 64 items. `PreparedRenderJob::step` processes them in a
`while`, while `measure_next` recursively falls through sections and empty collections. `complete`
allocates and clones a dynamic 16-byte result. The job measures a packet but does not perform the
plan's tessellation, ordering, batching, or upload-packet construction as resumable work; much of the
draw graph was synchronously built before this seam.

The dormant `Scene::finish` validates complete graphs, snaps all layers, orders all layers/passes,
batches complete outputs, scans all quads for animation, and hashes the complete packet in one call.
Those operations need owned cursors or deletion with a single mounted replacement.

### Atlas and generic upload ownership is whole-buffer

The live renderer clones the complete icon atlas in `frame_before_input` and complete glyph atlas in
`frame_after_input`. `PreparedRenderUpload::{GlyphAtlas,IconAtlas,Raster}` retain contiguous
`Vec<u8>` buffers. Presentation calls `upload_glyph_atlas`/`upload_icon_atlas` on the complete vector
in one opportunity. Only `RasterPages` currently has accepted <=16 KiB page ownership and resumable
GPU realization.

### UI-side GPU work is indivisible

`GpuContext::render_prepared` acquires the surface, creates views and two command encoders, renders
the complete 3D scene, submits it, encodes complete composition, submits it, and presents in one
`AppPresentPhase::Render` opportunity. Native/browser presentation therefore performs scene
encoding and full composition rather than only bounded pre-prepared submission. Resize still
reconfigures surface/depth resources synchronously in the separate P5e seam.

GPU/platform calls cannot be preempted after entry. Their input must be bounded before the call and
watchdog evidence must prove each admitted call is below its UI/worker ceiling.

### Terminal retirement is logical, not physical ownership retirement

Generic upload retirement truncates contiguous vectors in 16 KiB logical lengths without reducing
their capacities, then pops the upload and drops the allocation in one later step. Nested draw
retirement pops elements but eventually drops outer vector/string/control allocations. Receiver,
packet, job, frame, window, device-loss, panic, and ordinary `Drop` paths can therefore free whole
retained graphs outside the close cursor. There is no exact rejected generic input take/resume
authority or process-credit handback witness.

## Required Retained Design

### Paged admission at the producer boundary

- Replace generic nested draw/upload owners with actual fixed or <=16 KiB page storage. Admit
  operation, draw primitive, vertex, instance, key/string, clip, damage, directive, upload, eviction,
  atlas pixel, packet, surface, and process items/bytes before transfer. Credits use real allocated
  capacity/control ownership, not requested length, schema estimates, or unrelated decorative pages.
- Build the input through a persistent census/copy cursor. One producer opportunity transfers at most
  one scalar owner or page. Max + 1, stale generation, cancellation, and allocation failure return the
  identical source owner and leave the last-valid packet active.
- Preserve the accepted raster producer/page ledger. Glyph and icon atlas changes become immutable,
  generation-tagged dirty pages or page references; never clone a complete atlas. A page has one
  checked-out owner and one release-mode checked credit until upload ACK/abort/close returns it.
- Non-wrapping scene, atlas, surface, and packet generations permanently exhaust at `u64::MAX`.
  Duplicate/stale/ABA witnesses fail closed and cannot free the current occupant.

### One retained worker unit per opportunity

- One preparation turn advances at most one census scalar/page, primitive validation, snap record,
  ordering edge, tessellation fragment, batch record, hash block, upload page, packet scalar, child
  result, or close owner. Remove the 64-item loop, recursive section fallthrough, complete-vector
  reserve/collect/sort/hash, and dynamic completion clone from production reachability.
- Tessellation, draw ordering/batching, clip expansion, resource resolution, atlas dirty-page
  preparation, and command-buffer preparation are explicit generation-tagged child cursors. Parent
  completion is release-mode blocked until every child is complete and terminal ownership is
  accounted. Check deadline/cancel/freshness immediately before and after every opaque library or
  platform call.
- P5c supplies accepted layout/text/glyph page references; engine/plugin producers supply paged draw
  references. P5d must not re-clone their complete graphs to cross the seam.

### Atomic prepared packet and bounded presentation

- Seal one immutable fixed/page `RenderSnapshot` containing scene revision, preview generation,
  draw/batch pages, clips, hit index reference, directives, damage, and upload page references.
  Publication revalidates all producer generations and performs one O(1) last-valid pointer swap.
- Native and browser UI callbacks may acquire a prepared snapshot, submit a fixed bounded number of
  already-encoded command packets, apply fixed directives, and present. They may not traverse draw
  lists, tessellate, batch, compose a complete scene, clone an atlas, or create complete command
  encoders. If platform command encoding must retain device affinity, mount a dedicated bounded
  retained presentation cursor on the platform-authorized worker and prove the final UI submission
  unit independently below 2 ms.
- A device-loss/surface-loss failure aborts the candidate through witnesses and retains the last
  valid logical snapshot. GPU cache state changes only after matching packet/page ACK; stale or
  partial pages never become current.

### Exact terminal ownership

- Expose exact `take_rejected`, `take_terminal`, `resume`, `close_step`, and
  `terminal_is_empty` across input construction, preparation session, receiver, packet, presenter,
  atlas pages, GPU candidates, and child cursors. Worker panic, cancel, stale packet, saturation,
  receiver/session/window/app Drop, device loss, and shutdown converge on a wake-safe close pump.
- One close grant releases at most one actual scalar owner, key/string, primitive, page allocation,
  child handle, lease, packet slot, GPU witness, or process credit. `truncate`, `clear`, outer-vector
  `pop`, `take`, `None`, and ordinary populated `Drop` cannot masquerade as physical incremental
  retirement.

## Required Fixtures

- primitive/vertex/instance/key/clip/damage/directive/upload/eviction/atlas-page/packet/child/surface/
  process caps at max and max + 1, with exact pointer/page identity and capacity accounting;
- deep layers, many passes, empty-section storms, large meshes/textures/atlases, low fuel at every
  cursor, deadline before/after every opaque call, and proof that one turn advances one unit;
- dirty atlas spanning many pages, duplicate page, missing page, stale/ABA atlas and packet
  generations, interrupted upload, ACK/abort races, and unchanged atlas producing no copy/upload;
- stale/cancel/fault/panic/device-loss/surface-loss at every phase, checked-out terminal handback,
  receiver/session/window/application Drop, interrupted close, quiet wake, and lost-wake races;
- atomic last-valid snapshot and GPU cache witnesses: pointers/generations remain unchanged until
  complete ACK, then change once; rejected work returns all exact producer/process credits;
- mounted native and Wasm-shaped large-tree/effect/resize/multi-window storms at 1/2/4/default
  workers, every worker step below 8 ms, UI callback p99 <=2 ms, deterministic packet bytes/hashes,
  and no visible partial frame.

## Permanent Verifier Requirements

Faithful mutations must restore post-materialization measurement, saturating capacity counters,
dynamic draw/upload ownership, decorative pages, whole atlas clone/upload, contiguous generic raster,
64-item or recursive fallthrough, whole validate/snap/order/batch/hash, completion allocation/clone,
UI-side full scene encoding/composition, unbounded platform call input, logical truncate retirement,
bulk/drop terminal cleanup, missing exact rejection/take/resume/close, missing freshness/ACK, dormant
duplicate frame construction, or a second scheduler. Each mutation must be rejected before baseline
success is evidence.

P5d and Phase 5 remain open.
