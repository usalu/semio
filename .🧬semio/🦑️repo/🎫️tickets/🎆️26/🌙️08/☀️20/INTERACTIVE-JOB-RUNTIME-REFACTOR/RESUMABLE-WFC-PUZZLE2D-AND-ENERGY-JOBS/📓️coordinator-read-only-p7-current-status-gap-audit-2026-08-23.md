# Coordinator Read-Only P7 Current Status Gap Audit

Date: 2026-08-23  
Verdict: **Phase 7 RED — WFC is source/static-ready, while the live Puzzle 2D route and Energy job still violate the persistent-worker and bounded-step contracts.**

## Method

The audit reconciled the P7a–P7h ticket reports with an exact current-source constructor/caller census for `AssemblyInferenceJob`, `WfcJob`, `WfcRestore`, `BoardFillJob`, and `EnergyJob`. It inspected the mounted WFC relay/bridge route, the Puzzle 2D action/board route, and the Energy engine/job/kernel/precompute route. No Cargo, Nx, Wasm, browser, runtime, ticket-status, cache, or production-source mutation was performed while other Rust packets were active.

## P7a WFC Current Status

### Source/static gate: PASS, executable gate: pending

The latest independent P7h audit remains consistent with the current source census:

- the exact `semio.infer` / `s.assembly.solve` ActionBus factory is registered and discoverable;
- the mounted guest and host paths use persistent worker sessions rather than the old 50,000,000-fuel / 200-ms synchronous relay;
- the relay owns retained wake/cancel/cleanup authority and no longer parks the shared worker on a guest future;
- the preview bridge is latest-wins and the two-item checkpoint/commit bridge is lossless and byte-bounded;
- live revision/generation is validated immediately before terminal result exposure;
- WFC search, checkpoint construction, and restore have persistent stages, fixed-work sampling, one shared checkpoint header contract, exact-maximum/MAX+1 source fixtures, and retained cleanup/cancel race fixtures.

The current-tree debug/release, strict-warning, real WorkerPool 1/2/4/default, allocation/timing, native, and both Wasm gates remain unrun and cannot be inherited from older trees.

## P7b Puzzle 2D Blocking Findings

### 1. The mounted action reconstructs and serializes the entire job on every UI continuation

`set-fill-count` builds a full `board_fill_snapshot` synchronously, constructs `BoardFillJob`, and immediately serializes `job.checkpoint_bytes`. Every `brushFillSessionStep` clones the checkpoint from scene state, synchronously `serde_json`-decodes the full snapshot and job state into a new job, drives one unit on the action thread, then synchronously encodes the full state again.

The product therefore does not retain a worker-owned session. Its per-unit search cursor is wrapped in whole-snapshot clone/JSON encode/decode work on every UI callback.

### 2. Snapshot construction is an unbounded UI-thread graph traversal

`BoardHost::board_fill_snapshot` walks and clones every node, handle, kind, template, and weight, then builds `source_pairs` and a nested source-pair × kind × template compatibility census before a job or admission authority exists. This can dominate the interaction even if the later search step is bounded.

### 3. Core job publication still performs unbounded work inside one step

- `with_operation` serializes the complete snapshot in its constructor.
- `checkpoint_bytes` serializes the entire growing `BoardFillJobState` and recopies the snapshot.
- `restore` synchronously copies and deserializes both bodies.
- `accept_candidate` loops every handle template and constructs a dynamic JSON placement in one grant.
- `complete` serializes all placements and a full checkpoint in one grant.
- `preview_outcome` allocates/serializes JSON every step without declared channel byte admission.

The current hard cap of 1,000 placements does not establish a page, operation-byte, aggregate-byte, or per-step bound for dynamic IDs, templates, JSON values, or snapshot fields.

### 4. The mounted route bypasses the shared WorkerPool and live document authority

The action invokes `drive_step` directly in `Puzzle2dActionCtx`; the exact current source has no production `WorkerJobSession<BoardFillJob>` or `WorkerPool` owner. It supplies a fresh root cancel token each continuation, hardcodes base revision `0`, ignores the terminal `CommitCandidate`, and applies checkpoint-prefix placements directly to the fixture. Generation guards reject an old continuation, but there is no authoritative live revision/generation validation immediately before each mutation/terminal commit and no retained document/window close drain.

### Required P7b repair packet

Build a fixed generation-tagged retained Puzzle 2D fill session on the process WorkerPool. Cursorize snapshot capture/compatibility, dynamic template placement construction, preview/checkpoint/commit encoding, restore, and close. Reserve item/page/operation/aggregate bytes before ownership transfer; expose exact rejected/terminal take-retry-resume-close APIs; coalesce previews but never drop checkpoints/commits; validate live document revision/generation before applying a bounded prefix or final candidate. Add mounted saturation, cap/+1 pointer identity, cancel, stale commit, close, deterministic replay, and 1/2/4/default worker fixtures.

## P7c Energy Blocking Findings

### 1. Energy has zero mounted production job caller

The exact repository census finds `EnergyJob::new`, `EnergyJob::from_checkpoint`, and `Engine::job` only in the Energy engine's batch adapter and test module. `Engine::run` is referenced only by tests. No energy action, inference factory, model actor, WorkerJobSession, preview surface, or freshness-validated commit route constructs the job in production.

The job cannot satisfy the gate that all three operations display internal progress and cancel at every stage until a real energy-model operation mounts it.

### 2. Checkpoint and restore are whole-state clone/serde operations

`checkpoint` calls `encode_state`; `encode_state` clones weather, precompute, full simulation state, convergence maps, active timestep, meters, time series, histories, results, sizing, summaries, metrics, and the growing commit buffer, then serializes them all with `serde_json::to_vec` in one step. `from_checkpoint` performs the inverse full `serde_json::from_slice` in its constructor. There is no item/byte cap, page cursor, aggregate admission, exact rejected owner, or resumable restore.

The `Complete` arm repeats full-state checkpoint encoding in the terminal grant.

### 3. Preview publication is an unbounded sort/traversal/encoding step

`publish_preview` collects every zone ID, sorts the vector, traverses all zones into three dynamic arrays, computes facility totals, and encodes the complete zone-temperature vector in one grant. This is neither a bounded live projection nor a latest-wins channel with a declared byte cap.

### 4. Several simulation/finalization stages contain whole-collection work

- `Validate` runs complete model validation in one step.
- `TimestepWork::new` collects and sorts all surface and fenestration IDs in one step.
- zone preparation scans all people, lights, equipment, zone-node assignments, ideal loads, and zone-equipment assignments for a zone in one step.
- plant dispatch builds a collected equipment-priority vector in one step.
- warmup convergence performs two full-zone `all` scans and then rewrites both full-zone convergence maps in one step.
- final summary/metrics/economics/results stages traverse meters, zones, the complete temperature history, tariffs/LCCA, and dynamic result ownership without persistent microcursors.

The existing one-surface/one-fenestration/timestep cursors are valuable foundations, but they do not bound these adjacent calls.

### Required P7c repair packets

1. **P7c1 numerical microcursors:** cursorize validation, timestep ID ordering/admission, every per-zone component scan, plant dispatch construction, warmup convergence/history updates, final summaries/metrics/economics/result construction, and any dynamic per-record encoding.
2. **P7c2 checkpoint/publication:** replace whole-state clone/serde with fixed-page retained checkpoint build and resumable restore; add fixed item/page/op/aggregate caps, exact MAX/MAX+1, cancellation, retry, terminal close, and deterministic replay. Publish bounded progress projections through latest-wins preview authority and keep checkpoint/commit channels lossless.
3. **P7c3 mounted session:** register and mount one schema-owned Energy simulation operation on the process WorkerPool, bind it to live model revision/generation, render the four quality tiers as visibly provisional, reject stale publication/commit, and drain on document/window/app close.

## Phase 7 Exit Gate Still Required

After P7b and P7c source repair and independent re-audit, one serialized immutable-tree owner must run:

- focused and product debug/release tests;
- strict warning builds;
- real process WorkerPool replay at 1, 2, 4, and default workers;
- exact cap/MAX+1, allocation-pressure, cancellation/freshness, close-drain, panic/stuck-job, and queue-saturation cases;
- watchdog max and p99 evidence, first substantive preview under 50 ms, active cadence under 33 ms, and every UI callback/worker step below 8 ms;
- native runtime plus `wasm32-unknown-unknown` and `wasm32-wasip2` gates;
- deterministic WFC/Puzzle commits and Energy tolerance parity across worker counts.

Phase 7 must remain open until those gates pass on the final source tree.
