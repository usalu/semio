# Coordinator Independent P6g Mounted Fem2d Operation Session Final Audit

Date: 2026-08-23  
Verdict: **REJECT — the live mount is reachable and generation-tagged, but it does not yet publish live progress, its snapshot lease path saturates shared ownership, and live steps/close remain unbounded.**

## Scope

- the P6g implementation and census reports
- shared `ArtifactView`/`VcsArtifactApp` render and pending-effects authority
- retained reactor job registration and lifecycle
- the FEM2d editor and mounted session
- the selected FEM analysis constructor seam
- the permanent verifier additions

No Cargo, Nx, Wasm, browser, or runtime gate was run while overlapping Rust source packets remain active.

## Confirmed Progress

- FEM2d now has a production `pending_effects` route that can emit `SpawnJob` and `CancelJob` for a fixed session shell.
- The job input binds app instance, base revision, generation, canonical base bytes, operation, shell, and tagged job id.
- The mounted state exposes explicit graph, domain, mesh, assembly, PCG, commit, fault, and close phases.
- Nested input collections have persistent outer/inner/deep preflight cursors.
- The renderer has a real `Some(Fem2dLiveVisual)` source when the mounted identity matches.
- Fixed current/shell/retirement arrays and counter exhaustion checks avoid resizable session registries and obvious id reuse.

Those source improvements are retained. The packet is not source-accepted because the following live-route failures remain.

## Blocking Findings

### 1. Job progress never invalidates or rerenders the FEM surface

The mounted job mutates `Fem2dLiveVisual` during mesh/assembly/PCG stages, and `render` can borrow it. The production reactor, however, explicitly ignores `Event::JobProgress` (`reactor/component.rs:1023`). `dirty_render` is populated only by deferred surfaces, `SurfaceVisible`, and command/intent handling; job progress does not enqueue the owning FEM surface. `JobCompleted` only tries to resolve the request registry and also does not mark a surface dirty.

Therefore a surface rendered before the job starts is not rerendered for intermediate visual changes. At best, an unrelated later event can reveal the final retained state. The packet's live progress/preview claim is false on the mounted route.

Required repair: bind the job id to its exact instance/surface authority and coalesce a generation-validated dirty render on each accepted progress publication and completion. Add a production-route fixture proving multiple distinct patches/renders occur without unrelated UI input, and that stale job progress cannot dirty a newer generation.

### 2. Shared pending-effects eagerly issues snapshot leases that most calls never consume

`VcsArtifactApp::pending_effects` now calls `self.store.snapshot_read()` before invoking every editor's `pending_effects`, then embeds the lease in `ArtifactView`. All non-FEM editors use the default implementation and never call `take_snapshot_read`; FEM2d also returns early without taking it whenever the mounted identity is unchanged. Dropping the view marks the lease returned, but the fixed registry retains its owner until the store maintenance pump takes it.

The shared maintenance schedule services returned snapshot reads only in one of sixteen stages, while the eager pending-effects route can issue one every poll. Repeated ordinary no-op polls therefore accumulate returned owners and can saturate the 1,024-slot lease registry. Once saturated, `pending_effects` returns an empty effect list, so real revision replacement/cancellation can be silently delayed or lost.

Required repair: make snapshot ownership lazy or explicitly opt-in per editor, so a no-op/default pending-effects call issues no lease. If a lease is issued but not transferred, return and retire it in the same bounded owner path rather than accumulating one per poll. Add >1,024 no-op polls and unchanged-FEM-session fixtures that preserve lease capacity and still observe a later revision change.

### 3. Commit validation is not a live store revalidation

At `CommitReady`, `validate_commit` is called with an operation and the same operation's own base revision/generation, so that half of the predicate is tautologically accepted (`session/component.rs:438-450`). `current_identity` only reads the session registry. That registry changes when a later pending-effects pass observes the store; it is not the store's atomic current revision/generation/canonical root.

A document change can therefore race the worker: before pending-effects replaces the registry entry, the stale worker marks `validated_final = true`. The canonical base bytes are never reread at commit.

Required repair: revalidate against a live, atomically readable store authority at publication, including the full canonical base revision and generation. Add a controlled change-between-last-PCG-step-and-commit fixture.

### 4. The mounted worker still executes model-sized work in single opportunities

`BuildModel` directly calls `fem2d_engine::build_model` over the full snapshot (`session/component.rs:387-404`). Assembly completion constructs a full RHS and zero vector and calls `PcgJob::new` in one grant (`406-416`). Domain preparation performs full-length `try_reserve_exact` allocations in one opportunity (`255-276`). These paths only test that budget/deadline are nonzero; they do not check the deadline before or during the work.

The report labels numerical inner loops as P6h residuals, but this packet makes those residuals production-reachable from the live job. A source packet cannot be accepted as a bounded mounted session while its own live states invoke known monolithic work.

Required repair: fold P6h into the mounted path before P6g acceptance: cursorize model construction and all model-sized allocations/initialization, then prove every child constructor and child step honors remaining fuel/deadline before work.

### 5. Input ownership is counted after transfer and the accounting is not exact

The exact `SnapshotRead` moves into `MountedState` before preflight. Preflight counts string `len`, not allocation capacity, and omits vector/map backing capacities and owner overhead. A hostile snapshot larger than 4 MiB is already retained in the fixed shell while the counter eventually faults. There is no process/arena item+byte ledger reserving the actual retained snapshot owner across the 32 current and 64 shell slots.

Required repair: transfer an already-censused/paged snapshot authority or attach conservative exact credits that cover the whole owner before shell retention. Count capacities/backings symmetrically and add arena aggregate cap +1 identity fixtures.

### 6. Mounted close deep-drops entire numerical owners and reports zero released bytes

`MountedState::close_step` uses one `take()` each for `FemJobGraph`, `MeshJob`, `AssemblyJob`, `PcgJob`, `Arc<AnalysisModel>`, `PlanarDomain`, and `SnapshotRead` (`session/component.rs:458-491`). Each can own model-sized vectors/maps/matrices or the final reference to a recursive snapshot. Dropping it in one close opportunity is not bounded retirement, and every branch reports `released_bytes: 0`.

Required repair: give every child and retained input/output owner an explicit incremental disposer, forward one exact item/byte grant into it, and reach terminal-empty without whole-owner `take` drops. Add populated model/assembly/PCG/snapshot close fixtures under one-item/small-byte grants.

## Reaudit Gate

P6g remains source-rejected until the six findings are repaired with discriminating permanent verifier mutations. P6h is no longer merely an off-route residual because P6g mounts it; its constructor/step/disposal work must be corrected before this packet can pass. P6i's model-sized visual encoder and the serialized native/Wasm/browser timing matrix remain separate later gates.
