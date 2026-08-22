# Phase 5 Frame Transaction

Date: 2026-08-21

## Scope

This packet owns the Rust UI runtime transaction plus the UI-WGPU retained layout/surface scheduler. It replaces the public run-to-completion frame entry point with a persistent worker-owned transaction, makes presentation/reconciliation node-cursor resumable, and moves retained layout/text shaping out of `Ui::frame`. It does not edit renderer glue, stdio, Shell, directory/services, actor/job/plugin host, schema, database, or ticket lifecycle metadata.

Changed runtime paths:

- `Cargo.toml`: internal shared-job protocol dependency.
- `📦️glue.rs`, `🦀️dispatch.rs`: public documentation for the persistent transaction contract.
- `🦀️transaction.rs`: persistent frame transaction, credits, supersession, atomic publication, stress tests.
- `🦀️reconcile.rs`: persistent presentation/reconcile cursor and adversarial cursor tests.
- UI-WGPU `🦀️flex.rs`: persistent layout/text-shaping job and bounded large-tree solver.
- UI-WGPU `🦀️engine.rs`, `📦️glue.rs`: fair surface lanes, coalesced invalidation, public one-step scheduler, stress tests and exports.

There is no public `UiRuntime::transact` adapter. A private `#[cfg(test)]` helper drives the same public `FrameTransaction::step` API only to retain the existing semantic test matrix.

## P5a — Persistent Frame Transaction

`FrameTransaction` advances the required stages in this exact order:

1. `DrainProjectionDeltas`
2. `RouteIntents`
3. `FlushEffects`
4. `PresentSurface`
5. `ReconcileTree`
6. `BuildRenderPackets`
7. `PublishSnapshot`

Every work unit consumes scheduler fuel and is followed by cancellation, wall-clock, and hard-credit enforcement. `FrameTransactionLimits` bounds items, nodes, and bytes; exhaustion publishes a `CreditsExceeded` fault without publishing candidate patches or revisions. Projection draining retains the established 256-delta cap. Intent commands and deferred effects remain FIFO. Effect flushing remains bounded at 64 cycles and preserves the established `EffectStorm` fault semantics.

Surfaces are sorted before presentation, so output is independent of `HashMap` insertion order. An input epoch detects projection, intent, wake, registration, or direct-store input arriving between scheduler slices. Supersession discards only staged surfaces, trees, cursor state, patches, and shadow commits; it retains commands already accepted by the gateway and deferred/effect checkpoints. The transaction then re-enters `DrainProjectionDeltas` and forces current surfaces through the latest presentation.

Only `PublishSnapshot` installs completed per-surface reconcilers and advances retained revisions. Cancellation and hard-credit failure never install candidate state.

P5a stress coverage includes:

- one-fuel intent storms with FIFO accepted-command output;
- resumable effect storms with the exact 64-cycle fault;
- repeated resize-style input superseding staged presentation while preserving accepted commands;
- deterministic output across reversed surface registration order;
- an expired wall deadline consuming no queued input;
- hard item/node/byte credits faulting before candidate revision publication.

## P5b — Persistent Presentation and Reconcile Cursor

`SurfaceReconcileCursor` replaces the P5a full `SurfaceReconciler::reconcile` call inside `FrameTransaction::step`. Its internal phases are:

1. incrementally discover one presented node with an owned DFS frame;
2. allocate or reuse one keyed identity in deterministic preorder;
3. build and diff one record in deterministic postorder;
4. inspect one retained node for stale-subtree removal;
5. finalize one complete replacement reconciler and optional patch.

No recursive tree walk, whole retained-subtree walk, or whole reconciler clone occurs in a frame step. Child discovery and stale-child enumeration keep explicit persistent iterator/index frames. A single node's record comparison may compare or clone that node's immediate child-id list when emitting `SetChildren`; it never descends through those children. Candidate allocator state, key index, retained records, operations, root, and revision live only in the cursor. Dropping the cursor on supersession or cancellation leaves the retained reconciler byte-for-byte and revision-for-revision unchanged. A completed cursor is queued as a shadow commit and remains invisible until `PublishSnapshot`.

The cursor preserves the existing identity rule `(parent_id, key)`, monotonic non-reused IDs, duplicate-key rejection, targeted-operation versus `Upsert` byte selection, postorder patch ordering, root replacement, and revision guards. Its first-frame patch is exactly equal to the established keyed reconciler output in the parity test.

The `Present::present` callback itself remains an application-supplied synchronous work unit; the runtime cannot preempt arbitrary caller code. All framework-owned traversal of the returned `ComponentTree` is cursorized.

P5b adversarial coverage adds:

- exact patch/revision parity between resumable and established keyed reconciliation;
- abandoning a partially scanned 4,097-node candidate without changing the retained snapshot;
- 12,291+ independently measured node slices over a 4,097-node tree, each asserted below 8 ms;
- cancellation while a node cursor is active, followed by a live re-present at revision 1;
- the P5a repeated-input test now superseding an active P5b cursor and publishing only the latest model state.

## P5c — Resumable Layout and Text Shaping

`LayoutJob` owns persistent cursors for retained-node discovery, per-codepoint glyph preparation, stale Taffy mapping removal, postorder synchronization, solve, result collection, and publication. `LayoutJob: Send` is compile-asserted. Each call checks the shared `StepContext` cancellation/fuel/deadline contract and performs at most one node or glyph unit before yielding. Text shaping/raster cache misses are prepared one Unicode scalar at a time through `FontAtlas::ensure_glyph`; final-width text measurement is performed one node at a time while results are collected.

Taffy's solve is retained for trees up to 128 nodes, preserving the established exact flex/golden behavior. Taffy's solve is non-preemptible and measured 32.086 ms for the adversarial 1,025-node flat tree, so larger trees bypass it. The owned fallback has two cursor stages:

1. `MeasureFallback` visits one node in postorder, computes its intrinsic size, and contributes one aggregate update to its parent.
2. `ArrangeFallback` visits one node in preorder, resolves it from the already-arranged parent plus the parent's accumulated child metrics/offset, and prepares its final text-measure cache.

Neither fallback stage scans descendants or a wide child list. Stack direction, gap, padding, equal leftover distribution, Field label reservation, and Section header/gap semantics are retained. Complete geometry remains unpublished until all measure/arrange results exist, and `Ui::frame` returns the last valid `DrawList` while layout is active.

The 1,025-node/1,024-distinct-text stress completes in 11,183 scheduler calls. A diagnostic run observed a 0.360 ms maximum call, and the permanent test asserts every call remains strictly below 8 ms while checking root width and monotonic child placement. The earlier 32.086 ms opaque solve is no longer reachable for that workload.

## P5e — Multi-Surface Scheduling

Each retained window owns at most one coalesced `LayoutJob` and a `SurfaceLane`: `Interactive`, `UserVisible`, or `Background`. `Ui::step_layouts` advances exactly one selected surface with the caller's `StepContext`. The deterministic weighted wheel is `Interactive, Interactive, UserVisible, Interactive, UserVisible, Background`; empty lanes are skipped, and FIFO order is preserved within a lane. This prioritizes direct input while guaranteeing a queued background surface one selection within a six-slot wheel under a continuous interactive storm.

`set_viewport` invalidates only on an actual dimension change, abandons stale candidate layout state, marks root layout/paint dirty, and coalesces the latest dimensions into the existing queue entry. `apply_tree` similarly abandons only changed candidates; identical input does not restart an active cursor. A 2,000-update resize storm retains exactly one queued job and publishes the final 2,000 px viewport. The interactive-storm test repeatedly supersedes the interactive surface and proves the background surface still advances within one wheel.

The public host contract is `Ui::{set_surface_lane, step_layouts}`, `SurfaceLane`, and `UiLayoutStep`. `Ui::frame` no longer invokes synchronous layout; pending layout returns the last valid draw list, and painting occurs only after consistent geometry is ready.

## Verification

All Cargo-producing commands used the isolated target directory:

`PHASE-5-RESUMABLE-FRAME-TRANSACTION-AND-RENDERER/🧪️target-p5a`

Final gates:

- Debug quick: `CARGO_TARGET_DIR=…/🧪️target-p5a SEMIO_TEST_BUDGET_MS=120000 bun nx run @semio-tech/ui-runtime-rs:test-quick` — PASS, 72/72.
- Release quick: `CARGO_TARGET_DIR=…/🧪️target-p5a SEMIO_TEST_BUDGET_MS=120000 bun nx run @semio-tech/ui-runtime-rs:test-quick -- --release` — PASS, 72/72.
- Guest/browser wasm: `CARGO_TARGET_DIR=…/🧪️target-p5a bun nx run @semio-tech/ui-runtime-rs:check-wasm` — PASS for `wasm32-wasip2` and `wasm32-unknown-unknown`.
- Dependency ratchet: `bun ./📜️script.ts verify dependencies` — PASS; baseline 238, current 237, `rayon` removed concurrently, no new third-party dependency.
- Scoped formatting/diff: `cargo fmt --check` and unstaged `git diff --check` — PASS.
- Library clippy: `CARGO_TARGET_DIR=…/🧪️target-p5a bun x nx exec -- cargo clippy -p semio-framework-ui-runtime --lib --no-deps -- -D warnings` — BLOCKED only by the same three pre-existing `clippy::type_complexity` diagnostics at `🦀️context.rs:32`, `🦀️context.rs:79`, and `🦀️entity.rs:238`. There is no P5a/P5b diagnostic in `transaction.rs` or `reconcile.rs`.

The repository-wide cached diff check additionally reports three unrelated trailing-space lines in the Phase 1–5 repair-sweep diagnostic fixture; this packet did not edit that file. The owned unstaged diff check is clean.

P5c/P5e UI-WGPU gates, using the same isolated target:

- Focused Cargo library: `cargo test -p semio-framework-ui --features wgpu-engine --lib` — PASS, 264/264.
- Nx debug: `bun nx run @semio-tech/ui-rs:test-wgpu-engine` — PASS, 264/264.
- Release library: `cargo test -p semio-framework-ui --features wgpu-engine --lib --release` — PASS, 264/264.
- Browser wasm: `bun nx run @semio-tech/ui-rs:check-wgpu-engine-wasm` — PASS.
- Dependency ratchet: `bun ./📜️script.ts verify dependencies` — PASS; baseline 238, current 237 (`rayon` removed concurrently), no additions.
- Owned formatting and diff check — PASS.
- `cargo clippy -p semio-framework-ui --features wgpu-engine --lib --no-deps -- -D warnings` — no diagnostic in `flex.rs` or `engine.rs`; the crate gate remains blocked by 17 pre-existing diagnostics in `label.rs`, `paint.rs`, and existing element components.

## Outcome

The UI runtime now exposes scheduler-bounded transaction and retained-layout primitives, preserves accepted work across supersession, publishes only consistent completed transactions, fairly advances multiple surfaces, and cannot hide framework-owned whole-tree reconciliation/layout traversal inside one interaction slice. Debug, release, native dependency, and wasm gates are green; the only lint failures are recorded pre-existing diagnostics outside the owned files.
## 2026-08-22 Live Gate Rerun

The active `semio-framework-ui-runtime` package still contains the seven-stage persistent
`FrameTransaction::step` implementation and passed its current gates after the shared de-async
refactor:

- native debug quick: 72/72;
- native release quick: 72/72;
- `wasm32-unknown-unknown` check: pass;
- `wasm32-wasip2` check: pass.

The suite includes the large-tree per-slice eight-millisecond assertion, effect-storm resumability,
fuel-one input storms, cancellation without revision advance, stale-intent rejection, deterministic
surface ordering, hard item/node/byte credits, and newer-input supersession.
