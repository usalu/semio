# P3n — Engine Surface Terminal Retirement Audit

Date: 2026-08-23
Owner: `/root` coordinator
Verdict: **RED** — the fixed CPU registry is not mounted to production close and its disposer cannot retire ordinary populated surfaces.

## Preserved foundation

`EngineSurfaceRegistry` is a useful fixed 256-slot replacement for the earlier dynamic CPU map. It
has fixed ID admission, generation-tagged tokens, a retained `EngineSurfaceRetirement`, and an
explicit terminal witness. `AdmittedSurfaceMap` likewise fixed the shell scene-state authorities.
These changes must remain.

## Blocking findings

### Production has zero close callers

The exact repository scan finds `begin_engine_surface_close`, `close_engine_surface_step`, and
`engine_surface_terminal_nonopaque_is_empty` only at their definitions in the EngineCanvas module.
The only `EngineSurfaceRegistry::begin_close` caller is its focused test. Surface removal, window
close, document close, browser-worker close, and app close therefore never start or pump the CPU
engine-surface disposer.

### The disposer cannot close normal node-graph, map, or editor surfaces

`EngineSurfaceRetirement::close_step` handles pointer claims, board events/host, board sync fields,
one editor pack, and one map tile-request cursor. It contains no phases that retire:

- `node_graph: Option<NodeGraphEngine>` and the GraphHost/FlowHost owner graph;
- `NodeGraphSyncCache` strings, vectors, viewport, and scene pack;
- `map_host: Option<MapHost>`;
- `MapSyncCache` strings;
- `editor: Option<EditorHost>`; or
- the remaining surface-owned dynamic strings/vectors and their allocation backings.

The Witness phase merely checks that these owners are absent. If any is present it sets `faulted =
true`; the entry guard then returns false forever. Real production surfaces set these fields during
render/sync, so they cannot reach terminal-empty. The focused test uses `empty_engine_surface` with
only a board host, which does not discriminate the ordinary routes.

### The final shell step still deep-drops reachable owners

When the witness happens to pass, `ManuallyDrop::drop(&mut self.surface)` releases the complete
remaining `EngineSurface` shell in one turn. The proof relies on selected `Option::is_none`
predicates rather than an exhaustive owned-field taxonomy, and it reports one fuel unit regardless
of dynamic allocation/control backing that could remain in unexamined fields. This is not an exact
one-owner retirement contract.

### Token generation can alias

Reservation, test removal, and successful close use `wrapping_add(1).max(1)`. At exhaustion a slot
can reuse a historical nonzero generation. A stale token may then alias the new occupant. Generation
reuse must be refused through checked arithmetic and a permanently exhausted slot.

## Required repair

Mount CPU and GPU surface close together in one window/document/app disposer. Beginning close must
freeze CPU registration, prepared packet enqueue, GPU allocation/publication, interaction events,
and asset delivery for the exact surface token. Pump both sides from the single worker/UI capability
protocol until their explicit terminal witnesses agree, then invalidate the token before slot
reuse.

Extend `EngineSurfaceRetirement` with domain-owned retained disposers for every `EngineSurface`
field. GraphHost, FlowHost, MapHost, EditorHost, sync caches, tile requests, event queues, pointer
claims, strings, vectors, Box/Arc controls, and their fixed/page backings must advance one admitted
owner per grant. The source structures need their own close APIs where opaque internals prevent an
exhaustive outer cursor. No final whole-struct Drop is allowed until every dynamic/control owner has
an independently checked empty witness.

Use checked generation arithmetic. Exhaustion freezes the slot and returns the exact rejected
producer; it must not wrap, saturate, or reset to one.

## Required hostile fixtures

- populated Dag graph, Flow graph, tiled map, editor, and board surfaces each close to exact empty;
- one surface containing every simultaneously legal owner closes with one-owner fuel turns;
- zero fuel, insufficient fuel, expired deadline, interruption/resume, cancellation, and panic;
- close while prepared packets, asset I/O, pointer claims, board events, and GPU replacement are
  pending;
- window/document/app close actually invokes and pumps both CPU and GPU disposers;
- replacement registration remains blocked until terminal completion;
- generation exhaustion and slot reuse reject all stale/ABA tokens;
- ordinary Drop with a live owner fails closed; and
- verifier mutations remove every field phase/caller/witness and prove the focused test fails.

## Acceptance gates

The packet requires scoped format/static/self-test gates and independent source audit, followed by
the serialized native/Wasm/browser matrix with real graph/map/editor/board windows, repeated open/
close, device loss, memory pressure, and 8 ms watchdog evidence. Phase 3 remains open.
