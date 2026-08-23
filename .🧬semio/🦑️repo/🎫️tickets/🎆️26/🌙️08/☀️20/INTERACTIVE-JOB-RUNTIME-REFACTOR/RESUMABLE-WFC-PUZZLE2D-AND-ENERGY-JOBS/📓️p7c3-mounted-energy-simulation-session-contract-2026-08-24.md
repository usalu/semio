# P7c3 Mounted Energy Simulation Session Contract

Date: 2026-08-24  
Owner: next Sol High product executor after P2a1, P7c1, and P7c2 acceptance  
Status: prepared, not accepted  

## Packet Boundary and Preconditions

This packet mounts the accepted `EnergyJob` and P7c2 publication protocol as one real Energy model
editor operation. It owns product registration, action/command semantics, snapshot admission,
process WorkerPool session, live provisional result window, freshness validation, terminal result
exposure, and document/window/application close drain.

It starts only after:

- P2a1 provides the accepted universal retained job/session ownership and fixed child registry;
- P7c1 proves bounded numerical microcursors and exact numerical input admission;
- P7c2 proves fixed-page checkpoint/restore/preview/commit/fault ownership.

It must not add an Energy-owned pool, scheduler, thread, async runtime, blocking wait, unbounded
channel, production terminal drain, external runtime dependency, compatibility route, default
locale, or hidden headless solver path. `Engine::run` remains only the batch adapter over the same
job.

## Current Product Gap

The exact current Energy plugin declares only Model editor/viewer apps. Editor commands mutate
structure/zones; action folders are empty; descriptor commands do not expose simulation. The zones
window renders persisted model rows only. `EnergyJob::new`, `from_checkpoint`, and `Engine::job`
have no mounted production caller, and `Engine::run` is test-only.

There is no process job factory, retained session registry, live operation identity, simulation
configuration command, progress/cancel UI, four-tier provisional display, checkpoint/commit
consumer, stale publication guard, or Energy-specific close hook.

`EnergyModelSnapshot` stores composed child handles. The current convenience `energy_model`
working-scene cache and serde helpers may materialize the whole model; they are not an admitted
snapshot-to-job transfer boundary.

## Schema and Interaction Design

### Operation command taxonomy

Add schema-owned Energy editor commands/events for at least:

- start simulation with an explicit `SimulationConfig` projection and optional checkpoint token;
- cancel current simulation;
- retry a retryable rejected/terminal owner;
- discard/close the current simulation result;
- explicitly adopt/export a final result if the product exposes persistence.

Commands are CQRS events, not CRUD state setters. Starting a new simulation durably supersedes the
older generation; cancellation is idempotent. Simulation results are ephemeral local-only until an
explicit adoption/export event. A preview or automatic completion must never mutate the persisted
Energy model.

Update handcrafted Rust/TypeScript command schemas, descriptor semantics, plugin registration, and
permanent fixtures together. Every user-visible action/window/stage/tier label must provide English
and German variants with no implicit default language. Actions must be keyboard and screen-reader
accessible and expose busy/progress/cancel state without relying on color alone.

### Results/progress window

Add a dedicated Energy simulation window to editor and viewer surfaces where appropriate. It must
render:

- current stage and chronological progress;
- warmup convergence and checkpoint state;
- zone temperatures and heating/cooling demand from the bounded projection;
- surface heat-transfer and HVAC/fan/facility totals when present in that quality packet;
- running time-series blocks and final result availability;
- cancel/retry/discard actions and fault detail.

Render all four quality tiers as visibly provisional until `Final`: steady-state estimate,
design-day, coarse-timestep, and final. Use text/accessible state in addition to styling. A lower
tier may never overwrite a higher tier for the same operation/sequence, and no tier from a stale
generation may appear.

## Mounted Ownership Architecture

### Fixed operation arena

Register exactly one Energy simulation job kind with the process job registry during plugin
initialization. Use the accepted P2a1 fixed child/session registry and process WorkerPool; placement
is isolated but scheduling remains process-wide. Fixed operation slots carry nonzero generation,
application/document identity, canonical base revision, job operation ID, cancel authority,
snapshot/model admission cursor, P7c2 channel handles, immutable live view, and persistent close
cursor.

Slot/process MAX succeeds. MAX+1 rejects before snapshot/model ownership transfer and preserves the
exact action/snapshot/config owner for retry. Slot reuse is generation-tagged; stale factory tokens,
callbacks, previews, checkpoints, commits, cancels, and close handles cannot reach the replacement.

### Snapshot-to-model admission

The mounted action first acquires the exact store snapshot authority for the current document
revision. Resolve composed structure/zones child content through retained store reads. Build the
engine `Model` and `SimulationConfig` through the accepted P7c1 input census/build cursor and P7c2
page authorities; do not call whole serde/cache materialization on the UI/reactor turn.

Every child handle, link, model record, dynamic string, weather record/page, schedule/config owner,
and control backing is represented in the simultaneous operation credit. Missing child content,
revision change, cancellation, or admission failure returns/retire the exact owners without
partially mounting a job.

### Reconcile and spawn

Reconcile is event-driven and nonblocking. At most one bounded snapshot/admission/replacement unit
may run per host callback. Only after exact admission completes may reconcile emit the P2a1 spawn
effect/token. The WorkerPool factory reopens the same retained job authority; it does not clone,
decode, or reconstruct the Energy job from an opaque token.

A new revision/config/start event cancels and moves the prior operation into the fixed retirement
arena before the new generation becomes current. Retirement saturation leaves the exact new action
pending/rejected; it must not overwrite the current slot or inline-close the old job.

### Immutable live view

The UI never shares mutable `EnergyJob`, model, result, or channel state with the worker. It borrows
only an immutable, fixed/page-backed latest projection published atomically by P7c2. Each projection
carries app/document identity, canonical base revision, operation, generation, sequence, tier, and
stage. Renderer acquisition is try-only; contention returns the prior/empty UI immediately.

Checkpoint and commit queues remain lossless. Host maintenance consumes at most one queue/page
transition per callback. Full queues cause the worker to yield with the exact packet retained; the
UI must not synchronously drain them.

## Freshness and Terminal Semantics

Immediately before every preview install, checkpoint acceptance, final result exposure, explicit
adoption/export, retry, and close handoff, validate:

- application/document identity;
- canonical live base revision;
- nonzero current generation and operation ID;
- publication sequence monotonicity;
- tier monotonicity for the same operation;
- current configuration identity.

Stale packets move to bounded retirement without touching visible or persisted state. Final
completion publishes the already prepared P7c2 commit/result packet exactly once. Cancellation,
fault, completion, replacement, and close are distinct terminal reasons. A completed operation may
retain its final immutable result for the window while its worker/checkpoint/input authorities close;
that result uses its own explicit retained lease and discard/adopt path.

No terminal callback may allocate, serialize, traverse the model, run numerical work, block, or
drain until empty.

## Close and Lost-handle Recovery

Document close, editor/viewer window close, plugin/application close, generation replacement,
future/receiver/terminal-handle Drop, panic, and worker shutdown must all durably enqueue the same
operation generation in the fixed retirement arena.

The close cursor must explicitly retire one semantic owner or one admitted backing page per grant:
action/config, composed snapshot reads, input model, Energy numerical cursors, preview replacement,
checkpoint queue, commit/final result, faults, retry/worker handles, cancel authority, immutable
view, slot shell, and process credit. A handle dropped during partial `Closing` is rediscovered with
the same cursor; it is never reset or duplicated. Close makes progress without a mounted window.

Plugin `close_step` and `terminal_is_empty` must cover pending admission, current operation,
retirement arena, preview/checkpoint/commit channels, results, and faults. False terminal shells and
duplicate credit returns fail closed.

## Hostile Permanent Fixtures and Mutations

Add permanent mounted fixtures proving:

1. the exact localized start action registers and emits one process WorkerPool spawn only after
   multi-turn snapshot/model admission;
2. operation/process MAX succeeds and MAX+1 preserves exact action/snapshot/config identity;
3. UI/reactor callbacks never materialize the whole model or drive `EnergyJob::step` directly;
4. 1/2/4/default workers show the same deterministic tier/stage/progress sequence and final Energy
   tolerance result;
5. cancel/restart at every admission, numerical, checkpoint, queue-full, and terminal stage cannot
   publish or commit stale state;
6. preview replacement is latest-wins, checkpoint/commit delivery is lossless, and saturated queues
   preserve exact packet identity;
7. lower tiers never overwrite higher tiers and every nonfinal value is accessibly marked
   provisional in English and German;
8. document/window/application close and dropped handles during every partial close cursor reclaim
   every owner/page/slot/credit exactly once without a UI consumer;
9. restored checkpoint resumes the exact mounted generation only when identity is current;
10. no Energy-owned pool/thread/scheduler, blocking wait, production terminal drain, raw mutable
    worker alias, whole serde/cache materialization, unbounded channel, or stale mutation remains
    reachable.

Mutations must independently bypass exact admission, operation generation, canonical revision,
sequence/tier monotonicity, queue saturation, cancel at each stage, final single-transfer, localized
labels/accessibility state, close requeue, and credit handback. The focused mutation target must
kill every mutation.

## Acceptance Evidence

Source acceptance requires an independent Terra audit of the final diff, exact action/factory/
reconcile/render/close caller census, retained-owner inventory, and mutation inventory. Once the
entire Rust tree is source-quiescent, the serialized build/runtime owner must capture:

- Energy plugin/engine focused debug/release and strict-warning builds through Bun/Nx;
- native in-app start/progress/cancel/retry/final/discard behavior with console evidence;
- real WorkerPool 1/2/4/default deterministic replay;
- admission/queue MAX/MAX+1, saturation, cancellation, freshness, panic/stuck-job, lost-handle, and
  close-drain evidence;
- first substantive tier preview under 50 ms, active cadence under 33 ms, every UI/worker/channel/
  close turn below 8 ms, and allocation/process-credit evidence;
- `wasm32-unknown-unknown` and `wasm32-wasip2` gates with the same protocol semantics;
- accessible English/German UI evidence and final numerical tolerance parity.

Passing P7c3 completes the P7c source slice only. P7b and the final Phase 7 executable matrix still
gate Phase 7 closure.
