# Shared Typed-Command Full-Operation Partial

Date: 2026-08-26

## Verdict

RED, coherently fail-closed. The shared plugin source now contains an inspectable retained `TypedCommandFullOperationJob<A>` foundation and no longer contains the `BoundedFirstStepCommandJob` type or `bounded_command_output_bytes`. It is not a completed production command route: generic app reducers cannot be assumed bounded, immutable root refresh is not yet a retained producer, typed mutation publication lacks per-app cursors, and live exposure is not wired. `dispatch_typed_command_inner` therefore rejects with `interactive-job.full-operation-pending` before the first legacy refresh or snapshot call.

No production/runtime completion claim is made.

## Scope And Collision Boundary

Owned changes are limited to:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`;
- `🧪️shared-typed-command-full-operation-v1.json`; and
- this report.

Root `📜️script.ts` and launch configuration were read-only. No app source, Store source, ActionBus source, Cargo manifest/lockfile, or unrelated packet was edited. The repo MCP resource server failed its initialization handshake, so `repo://goals` and ticket status APIs were unavailable; the coordinator-selected existing umbrella ticket was used without making a false ticket-close claim.

## Retained Foundation

`TypedCommandFullOperationJob<A>` owns explicit state for:

1. `Prepare`: thirteen individually inspectable retained root slots and a `prepare_cursor`;
2. `Reducer`: a `reducer_cursor` that now rejects the generic bounded-proof path instead of directly calling an arbitrary app reducer;
3. `OutputValidation`: lane, item, and byte cursors plus checked `exact_output_bytes`;
4. `Ephemeral`: lane and item cursors;
5. `Emit`: lane and item cursors;
6. `Expose`: separately admitted revision/generation freshness and `validate_commit` immediately before completion publication;
7. `Complete` and `Fault`; and
8. incremental `begin_close`, `close_step`, and `terminal_is_empty` authority.

Every worker step checks cancellation, `should_yield`, and nonzero fuel before consuming exactly one unit. Prepare publishes preview/checkpoint output. Subsequent retained phases publish monotone checkpoints. The job contains the six permanent stage strings:

- `typed-command-prepare`;
- `typed-command-reducer`;
- `typed-command-output-validation`;
- `typed-command-ephemeral`;
- `typed-command-emit`; and
- `typed-command-expose`.

## Removed Monolithic Seams

The prior whole-output pass was deleted:

- `bounded_command_output_bytes`: zero definitions and zero calls;
- whole `serde_json::to_vec(&emit.effects/events/ui_scope)`: zero within the retained job;
- whole child-output loop: zero within the retained job;
- the `BoundedFirstStepCommandJob` type and call surface: zero production source occurrences; one unchanged `BoundedFirstStepCommandJobFactory` proof-catalogue string remains because app proof rows are outside this packet; and
- `ActiveToolCommand`/`ActiveToolCommandStage`: zero production source occurrences. The mounted worker-retirement shell was renamed to `MountedTypedCommandFullOperation` and no longer represents a prepare/commit compatibility dispatcher.

The retained job source census is exactly zero for `A::handle`, `A::ephemeral`, `resolve_ready(A::...)`, serde JSON, `bounded_command_output_bytes`, and `for`/`while` loops.

## Generic Reducer Blocker

The coordinator's hardened source law rejects treating `resolve_ready(A::handle(...))` or `resolve_ready(A::ephemeral(...))` as bounded without exact proof. Both calls were removed from this incomplete foundation. `TypedCommandFullOperationPhase::Reducer` now terminalizes with `generic bounded proof has no resumable app-owned reducer job`.

The refined governing boundary permits a genuinely small generic reducer only when construction is confined to `QualifiedToolProof::Bounded`, `ToolExecutionShape::BoundedFirstStep` is proven, and both `max_work_units_per_step` and `max_step_micros` are enforced. This partial does not yet establish those conditions across root preparation and publication, so it deliberately leaves even that narrow route closed. Resumable/app-owned commands must remain retained jobs and must never pass through the generic direct-call path.

The existing `QualifiedToolProof::AppOwned` builder surface remains in source for app-owned retained producers, but its root preparation still sits after the same unconditional shared fail-closed gate. It is not represented as production-reachable until immutable root admission moves into a bounded owner.

## Output Cursor And Fail-Closed Categories

Description and coalesce-key output census advances one UTF-8 byte per grant with checked addition and the exact contract byte cap. UI scope consumes one fixed typed field. Empty mutation/effect/event/child/task lanes advance one lane per grant.

Interrupted close removes at most one Unicode scalar from either retained string per close step, first checks that scalar's exact UTF-8 width against the supplied byte credit, and reports that exact width. Each remaining root owner is handed back at most one per step; terminal empty additionally requires the operation identity to be cleared.

The following nonempty output owners deliberately fail without transfer because no generic type-erased operation can provide their exact typed byte/page cursor:

- artifact, config, and draft mutations;
- presence and transient mutations;
- effects and events;
- child emits; and
- nested tasks without an app-owned retained producer.

This is a useful denial boundary, not a completed publication implementation. No compatibility encoder or whole `OpBinary::encode_op` measurement call was introduced.

## Immutable Root Boundary

The event-maintained command cache already returns document/config/history `Arc` roots in O(1), while child, peer-presence, presence-peer, and transient roots also have O(1) capture surfaces. Rebuilding the command cache calls history/store materialization and is not a bounded semantic unit.

The legacy tail still contains one `refresh_cache().await`, draft/interaction snapshots, presence/transient reads, revision/generation/envelope reads, and multiple root clones. They are source-visible and must be replaced by one retained capture per grant. They are currently unreachable because `interactive-job.full-operation-pending` occurs first.

The legacy tail contains no post-worker `presence_store.apply`, `transient_store.apply`, or `dispatch_emit` call. Live per-unit ephemeral/store/emit publication and final `InvocationResult` exposure remain missing rather than hidden behind that shell.

## Language-Neutral Fixture And Oracle

`🧪️shared-typed-command-full-operation-v1.json` declares the six stages, thirteen root slots, twelve validation lanes, a sixteen-byte fixture cap, and empty/single/maximum/maximum-plus-one cases.

The plugin component contains an owned test interface, `TypedCommandCensusOracle`, with:

- an owned one-byte-at-a-time implementation; and
- a test-only `serde_json` implementation.

Both produce the same `{bytes, accepted}` result for all four language-neutral cases. No third-party runtime type or dependency crosses the production API.

## Evidence

Safe commands only; no Cargo, Nx, Wasm, browser, or modifying Git command was run.

- `rustfmt --edition 2024 --config skip_children=true --check` on the shared plugin component: GREEN.
- Bun fixture/third-party-oracle parity: GREEN with `empty=1/true`, `single=3/true`, `maximum=16/true`, and `maximum-plus-one=17/false`.
- Focused Bun source census: GREEN for six retained stages, zero generic reducer calls, zero serde/whole-output helper/loops in the job, and the actual fail-closed helper call before preparation.
- Root source-law mutation suite: `365` self-tests GREEN.
- Root live coverage gate: RED (exit `1`). Exact headline census was `productionFactories=11`, `productionRegistrations=0`, `productionDispatches=3`, `boundedRows=0`, `remainingCommands=884`, `commandRows=775`, and `uniqueCommandRows=773`; its factory-contract ledger does see `TypedCommandFullOperationJobFactory<A>` as explicit, while activation/registration and complete prepare/publication remain absent.
- Rust fixture tests are source-present but were not compiled because Cargo was prohibited during overlapping Rust work; no Rust test pass is claimed.

## Remaining Acceptance Work

1. Replace cache rebuild with event-maintained root generations that can be captured one owner per grant.
2. Confine genuinely small generic reducers to `QualifiedToolProof::Bounded` with exact `BoundedFirstStep`, work-unit, and step-microsecond guards; require every resumable command to supply an app-owned retained reducer job.
3. Add typed mutation/effect/event/child/task item/page codecs with exact preflight and handback.
4. Drive ephemeral and persisted publication one proven store unit per grant under the same job authority.
5. Revalidate base revision and generation immediately before live exposure, then publish a lossless result page with explicit acknowledgement.
6. Replace the unreachable legacy preparation tail rather than moving it into another wrapper.
7. Prove zero fuel, expired deadline, cancellation at every transfer, stale base/generation, saturation/rejected admission, arithmetic overflow, interrupted close, terminal idempotence, and exact owner handback in compiled native and Wasm laws.
8. Run the serialized Cargo/Nx/Wasm/browser matrix only after overlapping Rust work is quiescent.
