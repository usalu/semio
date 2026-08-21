# P7c Energy Job

## Scope

The Energy simulation engine now reserves `async` for genuine suspension. The engine subtree had 632 non-suspending `async fn` declarations and zero `.await` expressions; the decorative declarations were removed across the subtree.

`Engine::run` is now a batch adapter over the same persistent `EnergyJob` used by interactive hosts. The job owns operation/generation freshness, cancellation, progress previews, checkpoints, a typed result, and deterministic owned binary commit output.

## Persistent stages

`EnergyJobStage` contains:

1. Validate
2. ResolveWeather
3. Precompute
4. InitializeZones
5. InitializeSurfaces
6. WarmupTimestep
7. StartRun
8. RunZoneTimestep
9. AggregateZone
10. AggregateFacility
11. PublishTimestep
12. Finalize
13. PublishFinal
14. EncodeOutput
15. Complete

Weather is resolved one record per step. `PrecomputedModel::build` now drives `PrecomputeBuilder`, whose zone/surface/normalization/thermostat/fenestration work is one deterministic model record per step. Zone and surface initialization, warmup hours, per-zone meter aggregation, checkpointing, previews, and final output encoding have persistent cursors.

The output encoder does not serialize a full-year time series in one callback. It emits one meter, summary row, or time-series sample per `EncodeOutput` step and preserves model-defined order rather than relying on `HashMap` iteration.

## Preview and checkpoint contract

`EnergyJobPreview` publishes sequence, quality tier, stage, warmup/run progress, sorted zone temperatures, heating/cooling fields, and facility electricity. The four labels are `SteadyStateEstimate`, `DesignDay`, `CoarseTimestep`, and `Final`.

Checkpoints are emitted at the warmup boundary and every 24 run timesteps. The present checkpoint payload carries operation revision/generation and progress cursors. Full cross-process restoration of kernel thermal history is not yet implemented, so this part of the P7c checkpoint gate remains open.

## Verification status

- Static engine census after the rewrite: zero `async fn` and zero `.await` in the Energy engine subtree.
- Permanent tests cover monotonic previews, quality-tier progression, daily checkpoints, owned output framing, cancellation and freshness before mutation, deterministic batch results, and the hard 8 ms per-step assertion on the one-zone design-day fixture.
- The requested Nx quick test was started with `bun nx run @semio-tech/energy-plugin:test-quick`, then stopped while it was waiting on the shared Cargo build lock so it would not contend with the stdio repair fleet on the compiler critical path. No pass is claimed yet.
- `SimulationKernel::advance_timestep` and final sizing remain internally monolithic. The outer job makes timestep-level progress resumable but the strict large-model per-component/substep hard gate remains open until the kernel is split into surface, fenestration, zone, system-substep, and secondary-component cursors.

## Files

- `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/**/*.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️component.rs`
