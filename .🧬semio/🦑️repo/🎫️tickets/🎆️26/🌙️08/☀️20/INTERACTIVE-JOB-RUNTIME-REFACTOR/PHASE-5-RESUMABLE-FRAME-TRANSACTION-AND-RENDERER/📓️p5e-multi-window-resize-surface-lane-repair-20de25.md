# P5e Multi-Window, Resize, and Surface Lane Repair Contract

Date: 2026-08-24  
Owner: `/root` coordinator  
Verdict: **PREPARED — implementation must wait for P2a1, P5c, P5d, and P3m/P3n ownership prerequisites.**

## Purpose

This packet closes the mounted Phase 5 scheduling boundary: a fixed set of admitted windows and
surfaces must share deterministic interactive, user-visible, and background lanes; resize storms
must coalesce without performing platform allocation in the callback; and window, document,
surface, and device loss must use one retained close/recovery authority. The acceptance target is
the production native and browser host route, not the dormant `Ui` tests alone.

The packet does not reimplement layout, prepared rendering, or EngineCanvas retirement. It composes
the independently accepted P5c layout authority, P5d prepared-render authority, and P3m/P3n paired
CPU/GPU surface authority through the universal P2a1 retained session protocol.

## Current Source Boundary

### Dormant UI scheduling

`Ui` currently owns `HashMap<String, UiWindow>`, three `VecDeque<String>` layout queues, and a
six-position weighted wheel. This gives useful intended order, but every window identifier and
queue entry is dynamically allocated, all queues are uncapped, and production has no caller of
`Ui::step_layouts`.

`set_theme` clones every window identifier and scans every window in one call. `set_surface_lane`
retains through all queues before allocating a replacement identifier. `set_viewport` and
`apply_tree` replace `layout_job` with `None`, ordinarily deep-dropping retained work. Generation
advance uses wrapping arithmetic. `Ui::frame` clears and rebuilds a complete draw list and visits
every scene slot in one foreground opportunity.

### Mounted platform resize

The native winit callback correctly coalesces logical metrics into the fixed host event envelope,
but the mounted route also calls `AppPresenter::resize`/`GpuContext::resize` immediately.
`GpuContext::resize` reconfigures the complete surface, invalidates the prior scene target, and
recreates depth resources synchronously. The browser host has the same semantic boundary even
where its platform calls differ.

Therefore callback boundedness is not established by the coalescing event queue: the callback still
reaches opaque platform work before the shared worker scheduler can grant or cancel it.

### Surface lifetime

P3m and P3n record the current CPU/GPU surface authority gap. Window/document/app close does not
start one paired surface disposer, ordinary populated EngineCanvas surfaces cannot reach the current
terminal witness, and replacement/resize publication can overlap live owners without one admitted
transition. P5e must consume that repaired authority rather than add a second surface registry.

## Preconditions

Implementation may begin only after all of the following source packets are independently GREEN:

1. P2a1 provides fixed universal session registration, nonblocking one-step driving, and retained
   terminal take/resume/close.
2. P5c mounts fixed-admission layout/text work and exposes one semantic-unit step plus retained
   close without whole-tree or whole-string foreground work.
3. P5d mounts fixed prepared-render, atlas, upload, GPU submission, and packet retirement steps.
4. P3m/P3n provide a single generation-tagged CPU/GPU surface registry, resize replacement
   authority, and populated terminal disposer.

If any prerequisite changes its public schema, P5e adapts directly to the final owned interface. It
must not add a compatibility adapter or preserve the dormant dynamic API.

## Schema-First Contract

### Window and surface identity

Define one owned fixed-capacity registry schema with:

- a slot index;
- a nonzero checked generation;
- a bounded UTF-8 identifier stored in registry-owned fixed backing;
- one window/document/realm identity tuple;
- one current CPU/GPU surface token;
- one current layout-session token;
- one current frame/preparation/presentation token;
- one optional retained resize-replacement token; and
- an explicit lifecycle state.

Lifecycle states are `Vacant`, `Opening`, `Live`, `ResizePending`, `ResizeBuilding`, `Publishing`,
`Recovering`, `Closing`, `Faulted`, `Closed`, and `Exhausted`. Generation overflow permanently
exhausts the slot. No wrap, saturating reuse, reset to one, or stale-token alias is permitted.

The schema declares exact maximum window count, identifier items/bytes, queue slots, metrics
envelopes, pending session tokens, prepared packet owners, GPU replacement owners, fault owners,
and close owners. Admission checks the complete simultaneous live-plus-candidate working set before
moving any producer-owned value.

### Lane queues

Retain the three semantic lanes `Interactive`, `UserVisible`, and `Background` and the existing
deterministic weighted order unless a schema fixture proves a different explicit wheel. Replace the
dynamic queues with fixed slot-index rings. Queue entries contain only slot, generation, reason,
and a monotonic nonzero wake epoch; they never copy a window identifier.

At most one entry per `(slot, generation, work kind)` may be queued. A newer replaceable request
updates its retained payload and epoch in place. Nonreplaceable commands use fixed lossless slots;
admission rejection returns the exact command producer.

The scheduler advances one registered session by one semantic unit per worker grant. It must not
fall through to another stage, window, surface, queue scan, or close owner after consuming that
unit. Inspecting a fixed number of empty wheel positions is metadata work and must be separately
bounded by the schema.

### Work kinds

The registry discriminates at least:

- input/reconcile transaction;
- layout/text;
- paint/preparation;
- GPU upload/submit/present;
- resize replacement;
- device/surface recovery; and
- terminal retirement.

Work kinds share the process WorkerPool. P5e must not create a thread, runtime, nested pool, local
executor, per-window worker, or run-to-completion callback.

## Mounted Production Flow

### Metrics callback

Native and browser metrics callbacks perform only:

1. validate the exact live token;
2. normalize finite width, height, and scale-factor scalars;
3. replace the fixed latest-wins metrics payload;
4. advance its checked epoch; and
5. invalidate the exact surface once.

They do not call `surface.configure`, allocate a texture/view/depth buffer, clone identifiers,
render, await, block, lock through domain work, or pump a session.

Zero-sized/minimized windows remain live with a suspended presentation state. The newest nonzero
metrics request supersedes all older unstarted resize candidates without losing the current valid
surface.

### Resize replacement cursor

The admitted resize session retains the old live surface while it advances through:

1. claim latest metrics generation;
2. validate extent and backend limits;
3. reserve all replacement CPU/GPU/control ownership;
4. create/configure the candidate platform surface state;
5. allocate candidate color/depth/staging owners one at a time;
6. validate device/surface generation;
7. atomically publish the complete candidate;
8. detach the former surface into P3n retirement; and
9. return replacement credit and publish one completion invalidation.

Every opaque backend call is isolated as one admitted prepared syscall boundary. The foreground
host may invoke only a constant-time platform capability thunk; any backend that cannot provide a
bounded/prepared transition must remain RED and be represented by an explicit platform-specific
failure, not hidden inside a nominal one-fuel step.

Cancellation, newer metrics, surface loss, device loss, panic, and close preserve the last valid
surface until a candidate is completely published. A superseded candidate enters retained close;
it is never overwritten, cleared, or deep-dropped.

### Layout and frame driving

The production interpreter/host registers the exact admitted window with the fixed UI registry and
queues P5c layout work whenever tree, viewport, theme, font, locale, or scale generation changes.
The shared scheduler drives it. No render path may call a full layout oracle.

After P5c publishes a complete generation, P5e queues P5d paint/preparation for the same token.
Only an exact generation match may progress through submission. Completion or progress coalesces a
single redraw invalidation. A background window cannot starve; an interactive storm cannot create
unbounded duplicate entries.

Theme and locale changes use retained registry cursors. One grant marks or queues one window; they
never clone the complete ID set or scan every admitted window in one step.

## Freshness and Publication

Every request carries window slot/generation, document generation, surface generation, input epoch,
layout revision, scene revision, and metrics generation as applicable. Validation occurs before
each irreversible publication or platform capability call.

Latest-wins is allowed only for pointer movement, wheel deltas, metrics, hover, and preview-like
invalidations. User commands, commits, close, fault, and recovery events are lossless within their
declared fixed admission. A replaced metrics or preview owner is moved to retained close before its
slot is reused.

The last valid layout, frame, and surface remain visible during replacement, cancellation, and
recoverable failure. A stale generation may return credit and retire owners but may never publish,
present, dirty a newer generation, or acknowledge a command.

## Close and Lost-Handle Recovery

Window, document, realm, app, and device-loss closure freeze new admission for the exact subtree,
detach every queued entry/session/candidate/live surface into one retained close graph, and advance
one owner per worker grant.

The public protocol exposes:

- begin close;
- take close authority;
- resume close authority;
- advance one close unit;
- take terminal fault/result where applicable; and
- query an exhaustive terminal-empty witness.

Dropping the ordinary session handle while `Closing` must leave a generation-addressable authority
that the fixed registry rediscovers and resumes. Closing never depends on the original handle being
alive. A full terminal/result registry must return the exact rejected terminal owner or preserve it
for later retrieval; it must not livelock or silently drop.

The exhaustive witness covers window identifiers, UI tree/layout/router/draw owners, all lane
entries, metrics, sessions, prepared packets, CPU/GPU resources, platform handles, invalidations,
fault strings, control blocks, pages, and every admission credit. Only then may the fixed empty shell
be dropped.

## Hostile Fixtures

Add schema-first fixtures for:

- zero, maximum, and maximum-plus-one windows, surfaces, identifier bytes, and queued work;
- three simultaneously busy lanes with deterministic weighted progress and background fairness;
- one million pointer/resize samples coalescing to one newest payload without command loss;
- resize on every cursor phase, followed by a newer resize, cancel, close, and device loss;
- zero-size minimize then restore, repeated scale-factor changes, and extent maximum/+1;
- last-valid frame/surface preservation until atomic candidate publication;
- stale window, document, surface, layout, scene, and metrics generations;
- checked generation exhaustion and permanent slot retirement;
- worker counts one through the supported maximum with identical publication order/digests;
- interrupted terminal close resumed only from the fixed registry;
- full terminal/result slots returning exact producer identity;
- panic/fault before and after each owner transfer;
- native window churn and real browser worker window/canvas churn;
- device/surface loss during layout, preparation, submission, presentation, and old-surface close;
- exact process/page/item/byte/control counters returning to zero; and
- no UI callback or worker step at or above 8 ms, callback p99 at or below 2 ms, and interactive
  acknowledgement within one frame under simultaneous effect and resize storms.

Each semantic fixture has a paired verifier mutation that removes or weakens the required
production property and must make the focused gate fail.

## Permanent Verifier Predicates

Extend the existing interactivity region in root `📜️script.ts`; do not create another script. Deny:

- dynamic window maps/queue identifiers on the mounted P5e route;
- production `Ui::step_layouts` caller count zero;
- immediate callback reachability of presenter/GPU/platform resize;
- generation `wrapping_add`, saturating reuse, or reset-to-one;
- queue `retain`, complete ID collection/clone, unbounded push, or duplicate live entry;
- `layout_job = None`, `clear`, bulk `take`, or whole-owner Drop on cancel/supersede/close;
- more than one semantic owner transition per worker grant;
- stale generation publication or dirtying;
- missing mounted window/document/app/device-loss close callers;
- missing lost-handle registry rediscovery; and
- terminal-empty witnesses that omit any schema owner or credit counter.

Mutations must be production-reachable and discriminating. Text-presence assertions without a
semantic hostile fixture are not acceptance evidence.

## Owned Files and Collision Boundary

The executor re-censuses exact current paths immediately before editing. Expected ownership is
limited to the UI-WGPU engine/scheduler, mounted interpreter bridge, renderer host event/metrics
envelope, paired presenter/surface authority supplied by P3m/P3n, the narrow universal session
registration supplied by P2a1, root `📜️script.ts`, focused fixtures, and this ticket report.

Do not edit live P5c, P5d, or P3m/P3n internals while their prerequisite packets are active. Compose
only through their accepted owned interfaces. Any newly discovered overlap returns to coordinator
for a new disjoint packet boundary.

## Acceptance Gates

Source acceptance requires exact caller census, scoped and whole `git diff --check`, rustfmt,
verifier self-test, live focused verifier success, and an independent Terra audit of admission,
generation, single-unit progress, freshness, and terminal-empty ownership.

Final Phase 5 acceptance additionally requires the one serialized debug/release/strict-warning,
native, both Wasm targets, real browser worker, multi-window/resize/effect/device-loss stress,
allocation-pressure, deterministic replay, and timing matrix on the same final tree. Historical or
test-only latency results do not satisfy the gate.

P5e remains RED until all prerequisites and both source/runtime gates pass.
