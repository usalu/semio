# P3m/P3n Mounted Engine Surface Lifetime Repair Contract

Date: 2026-08-24  
Owner: `/root` coordinator  
Verdict: **PREPARED — source and runtime acceptance are pending.**

## Purpose

Replace the split dynamic GPU presenter and unmounted/incomplete CPU surface disposer with one
fixed, generation-keyed, mounted EngineCanvas surface lifetime. Creation, resize, prepared packet
presentation, surface/device loss, document/window/app close, and replacement must share one
admission and terminal graph.

This packet composes the preserved fixed CPU `EngineSurfaceRegistry`, admitted scene-state maps,
prepared scene/raster authorities, and `GpuContext` allocation gate. It does not introduce a second
process pool, surface registry, or compatibility API.

## Preserved Foundation

Keep:

- the fixed 256-slot CPU `EngineSurfaceRegistry` shape;
- generation-tagged CPU tokens;
- `AdmittedSurfaceMap` for scene-state owners;
- retained prepared scene/raster packets and their exact freshness witnesses;
- direct UI-capability ownership of GPU allocation/presentation; and
- event-driven invalidation and the single process WorkerPool.

These primitives are useful but are not mounted lifetime acceptance by themselves.

## Current Blocking Boundary

### GPU presenter

`EngineCanvasPresenter` owns `HashMap<String, EngineGpuSurface>`. `realize_one` performs duplicate
lookup, texture/view/Vello renderer allocation, and cloned-ID insertion. Resize replaces GPU owners
without a presenter-owned retained retirement cursor. Ordinary presenter Drop can release every
surface and control graph at once.

`EngineCanvasBuildContext` owns an unbounded packet vector, copies surface IDs, and transfers the
whole vector. Packet item/byte/ID/output admission does not precede ownership.

### CPU disposer

The current `begin_engine_surface_close`, `close_engine_surface_step`, and terminal witness have no
production close caller. The disposer omits ordinary populated node graph, GraphHost, FlowHost,
NodeGraphSyncCache, map host/cache, editor host, and other dynamic/control fields. Its witness faults
forever on those real owners, while its final shell step can still deep-drop uncensused fields.

### Generation

CPU slot reuse uses wrapping/reset-to-one arithmetic. Exhaustion can alias a stale token. GPU
surface identity is string-keyed rather than slot/generation aligned.

## Unified Schema

Define a single schema-first `EngineSurfaceToken` containing fixed slot and checked nonzero
generation. One fixed slot owns:

- admitted bounded surface-ID backing;
- CPU `EngineSurface` candidate/live/retirement state;
- GPU candidate/live/retirement state;
- fixed pending prepared-packet slots;
- fixed pending asset/input/interaction slots;
- current document/window/realm generations;
- current metrics/resize generation;
- one retained creation/replacement cursor;
- one retained terminal cursor;
- one fault owner; and
- exact item/byte/page/control/GPU/process credit witnesses.

Lifecycle states are `Vacant`, `Reserved`, `BuildingCpu`, `BuildingGpu`, `Live`, `Replacing`,
`Publishing`, `Recovering`, `Closing`, `Faulted`, `Closed`, and `Exhausted`.

The schema declares exact maxima for surfaces, ID bytes, packets, packet bytes, CPU domain owners,
GPU texture bytes/extents, Vello controls, texture/view handles, asset owners, event owners,
candidate-plus-live replacement ownership, retirement stack pages, and terminal/output slots.

Admission reserves the complete simultaneous live-plus-candidate working set before moving a
producer or invoking any allocation. Standard-map entry-size estimates are forbidden.

Generation uses `checked_add`. Overflow moves a vacant slot to `Exhausted`; it never wraps,
saturates, resets, or becomes reusable.

## Fixed GPU Authority

Replace the string `HashMap` with fixed slots aligned one-for-one with the CPU registry. An ID is
stored once in fixed/observed admitted backing and referenced by token thereafter. Duplicate,
oversized, full, closing, stale, wrong-document, and exhausted-generation admission returns the
exact producer before texture/view/renderer allocation.

Candidate GPU construction advances:

1. reserve exact slot/ID/control/texture authority;
2. validate token/document/metrics/device generations;
3. allocate texture as one prepared UI-capability unit;
4. retain texture or its exact fault owner;
5. allocate view as one prepared unit;
6. retain view or fault owner;
7. allocate Vello renderer/control as one prepared unit;
8. render/stage one admitted page/packet unit at a time;
9. revalidate all freshness dimensions;
10. atomically publish the complete CPU/GPU pair; and
11. detach displaced CPU/GPU owners into the unified retirement cursor.

Every partial success remains reachable in the fixed slot. Cancellation, stale validation, device
loss, panic, or publication failure cannot rely on local destructor release.

The prior valid surface remains visible until the complete candidate publishes. Failed replacement
never leaves a partial live shell.

## Fixed Prepared Packet Authority

Replace the EngineCanvas build packet vector with a fixed/page queue. Enqueue preflights packet
count, aggregate bytes, ID bytes, scene/raster witnesses, control backing, output slot, and
live-plus-candidate surface ownership before transfer.

Each presenter grant advances at most one packet/page/control/texture/view/renderer/submit/ACK
semantic unit. Taking packets never moves an unbounded vector. Replaceable previews may be
latest-wins only through retained close of the displaced owner; commits and terminal events remain
lossless within fixed admission.

Completed-but-unclaimed, rejected, stale, cancelled, and faulted packets have exact take/resume/
close paths. A full result/terminal queue returns the exact rejected owner or leaves it discoverable
in its original generation slot.

## Exhaustive CPU Retirement

Extend `EngineSurfaceRetirement` through domain-owned retained child disposers for every field of
`EngineSurface`. The exhaustive catalog includes at least:

- pointer claims and interaction state;
- board events, board host, board sync caches, editor pack, and scene pack;
- `NodeGraphEngine`, GraphHost, FlowHost, graph nodes/edges/ports/previews/layout state;
- `NodeGraphSyncCache` strings, vectors, viewport, and scene owners;
- `MapHost`, `MapSyncCache`, tile requests, tile payloads, view state, and scene owners;
- `EditorHost`, editor document/history/selection/input/render owners;
- prepared scene/raster/asset references and pending delivery;
- every string/vector/map/box/arc/control allocation and its backing;
- candidate/displaced CPU roots; and
- registry slot and credit controls.

Opaque child types expose their own owned close API and exhaustive terminal witness. The outer
cursor cannot inspect external implementation details or deep-drop the child.

One grant retires one admitted semantic owner or one fixed backing page/control. Recursive child
graphs use an explicitly admitted fixed stack sized by a combined maximum nesting proof.

The final CPU shell may be dropped only when the exhaustive catalog and every child witness are
empty. Populated ordinary Drop fails closed.

## GPU Retirement

GPU close advances one admitted owner per UI-capability/worker grant across:

- pending allocation/control fault;
- staged raster/scene page;
- queued/presenting/completed packet;
- Vello renderer/control owner;
- texture view;
- texture;
- displaced candidate/live surface;
- ID backing; and
- fixed slot/credit witness.

Because GPU handles require UI capability, the retained worker cursor prepares exactly one action
and the UI callback executes only that bounded capability thunk. No worker accesses GPU handles and
no callback scans a surface graph.

Device loss freezes publication and routes all affected slots through the same close/recovery
graph. App close uses the same graph; presenter Drop is not a cleanup substitute.

## Mounted Lifecycle Callers

Mount the unified authority at all production roots:

- surface registration/open;
- first prepared EngineCanvas packet;
- document replacement;
- window close;
- document close;
- browser worker/canvas close;
- native app close;
- realm/shard loss;
- surface loss;
- device loss; and
- application fault shutdown.

Beginning close freezes new CPU registration, prepared packet enqueue, GPU allocation/publication,
interaction events, assets, preview/commit delivery, and resize for the exact token.

The fixed session registry pumps close to terminal even after the public handle is dropped. A
partial `Closing` cursor is generation-addressable and resumes exactly once. Replacement remains
blocked until terminal-empty invalidates the old token.

## Freshness and Resize

Validate surface slot/generation, document generation, scene revision, preview generation, metrics
generation, and device generation before every irreversible ownership transfer, GPU call,
publication, stage, submit, and ACK.

P5e owns callback coalescing and lane scheduling. This packet exposes a retained replacement token
and one-unit resize/recovery cursor; it does not perform platform resize synchronously in the
metrics callback.

Zero-size minimize parks presentation without destroying the last valid surface. Restoration uses
the newest nonzero metrics generation and atomic replacement.

## Hostile Fixtures

Provide source fixtures and matching verifier mutations for:

- surface count, ID bytes, packet count/bytes, texture bytes, extent, control owners, and close stack
  exact maximum/+1;
- ordinary populated board, Dag, Flow, map, editor, and all-simultaneous-owner surfaces;
- one-owner-per-turn close with zero/insufficient fuel and expired deadline;
- interruption/resume and dropped-handle partial `Closing` rediscovery;
- full terminal/output registry exact producer return without livelock;
- stale/duplicate/wrong token and checked-generation exhaustion/ABA;
- texture success/view failure, view success/renderer failure, renderer success/render failure,
  render success/stage failure, submit failure, and ACK failure;
- panic before and after every ownership transfer;
- cancel/device loss/surface loss during every construction/replacement/retirement phase;
- completed-but-unclaimed packets and surfaces;
- old live surface preservation until atomic replacement;
- registration blocked until old terminal-empty;
- process/page/item/byte/control/GPU counters returning exactly to zero;
- deterministic native and browser-Wasm reuse/close under resize and window churn; and
- every UI capability step and worker step below 8 ms, UI p99 at or below 2 ms.

Mutations remove each field phase, mounted caller, preflight, generation validation, credit,
one-unit boundary, lost-handle recovery, or terminal witness and must make the focused verifier fail.

## Permanent Verifier Predicates

Extend the existing root `📜️script.ts` interactivity region. Deny:

- `HashMap<String, EngineGpuSurface>` and dynamic packet vector ownership;
- direct string-keyed GPU insert/remove/replace;
- wrapping/saturating/reset generation reuse;
- production close caller count zero;
- any populated CPU field omitted from the retirement catalog;
- whole-surface/map/vector `clear`, `take`, or Drop on supersede/fault/cancel/close;
- more than one semantic owner transition per grant;
- UI callback graph scans or synchronous resize/allocation chains;
- stale candidate publication/staging/submission/ACK;
- ordinary populated presenter/surface Drop;
- missing registry rediscovery after handle loss; and
- nonexhaustive terminal-empty witnesses.

Text-presence assertions require semantic fixtures and discriminating mutations.

## Owned Files and Collision Boundary

Expected ownership is limited to the EngineCanvas CPU/GPU surface regions, minimum domain child
close APIs, prepared EngineCanvas packet envelope, presenter glue, mounted window/document/app loss
callers, root verifier, focused fixtures, and this ticket report.

Re-census exact current paths immediately before editing. Do not overlap active Raster, P4, FEM,
layout, prepared-render, or P5e internals. If a required child surface file is active elsewhere,
split it into a later domain-owned close packet rather than editing concurrently.

## Acceptance Gates

Source handoff requires scoped rustfmt, exact caller/owner census, verifier self-test/live focused
success, scoped and whole diff checks, deterministic ledgers, and independent Terra audit of every
CPU field, GPU partial stage, mounted close caller, generation, and terminal counter.

Final Phase 3 acceptance additionally requires serialized debug/release/strict-warning builds,
native and both Wasm targets, a real browser worker, populated graph/map/editor/board windows,
repeated open/resize/close, device/surface loss, memory pressure, cancellation/fault injection,
deterministic replay, and timing on the same final tree.

P3m/P3n and Phase 3 remain RED until both source and runtime gates pass.
