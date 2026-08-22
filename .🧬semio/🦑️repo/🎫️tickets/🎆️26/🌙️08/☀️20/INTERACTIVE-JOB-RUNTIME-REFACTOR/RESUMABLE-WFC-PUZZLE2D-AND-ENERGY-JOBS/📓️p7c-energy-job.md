# P7c Persistent Energy Job

## Scope

- Removed 632 decorative `async fn` declarations and converted the remaining 239 non-suspending async tests throughout the Energy simulation engine. The final engine census has zero `async fn`, `.await`, or `async_test` hits.
- Preserved `Engine::run` as a synchronous batch driver over the same persistent `EnergyJob` used by interactive hosts.
- Split timestep execution, sizing, result finalization, and owned output encoding into persistent deterministic cursors.

## Timestep Kernel

`TimestepWork` is a serializable, synchronous state machine with ordered surface and fenestration identifiers, per-part cursors, zone work, system-substep cursor, secondary-component cursors, deterministic PV accumulation, weather/context data, and the stages `Surface`, `Fenestration`, `Zone`, `SystemSubstep`, `ZoneCommit`, `SecondaryPlant`, `SecondaryPv`, `SecondaryBattery`, `SecondaryServiceHotWater`, `SecondaryRefrigeration`, `SecondaryWater`, and `Complete`.

One call processes one surface, fenestration, zone preparation, system substep, zone commit, or secondary item. `SimulationKernel::advance_timestep` is now only the batch driver over this state machine. Model-owned identifiers are sorted before traversal, so `HashMap` order does not affect replay.

## Energy Job and Finalization

`EnergyJob` owns validation, incremental weather resolution, precompute cursors, state initialization, warmup and run timestep work, zone/facility aggregation, preview/checkpoint publication, sizing, final summaries, metrics, economics, result construction, and record/sample-bounded output encoding.

Final stages are `Finalize`, `Size`, `FinalizeSummaries`, `FinalizeMetrics`, `FinalizeEconomics`, `BuildResults`, `PublishFinal`, `EncodeOutput`, and `Complete`. `SizingBuilder` processes one zone or one equipment record per work unit.

Typed previews include quality tier, stage, warmup/run progress, sorted zone temperatures and loads, facility electricity, and sequence. Cancellation, operation/generation freshness, fuel, and deadline checks precede mutation.

## Full Checkpoint Restore

The `ENERGY2` checkpoint is serde-encoded and contains operation revision/generation/seed, thermal zone and conduction history, precompute state, active `TimestepWork`, run-period iterator, aggregation cursors, meters and time series with deterministic order, RNG state, sizing/finalization state, preview state, and output-encoder cursors. `EnergyJob::from_checkpoint` validates identity and restores every mutable field.

Tests verify byte-identical restored checkpoint state, identical active timestep substage, and result/output parity across fuel 1, 64, and 128.

## Verification

- Formatting: final changed Energy leaves formatted with `rustfmt --edition 2021`.
- Source census: `rg -n 'async fn|async_test|\.await' <energy-engine>` — zero hits.
- Full focused debug: `CARGO_TARGET_DIR=<ticket>/🧪️target-p7-energy-focused cargo test --manifest-path <ticket>/🧪️energy-focused/Cargo.toml -- --skip <three stdio-stub EPW parser tests>` — exit 0; **238 passed, 0 failed, 3 filtered** in 8.72 s (`📝️p7c-full-debug-1.txt`).
- Final focused release simulation/job suite: the same harness with `--release 'sim::tests::'` — exit 0; **12 passed, 0 failed** in 0.08 s (`📝️p7c-focused-release-3.txt`).
- Wasm: `cargo check --target wasm32-unknown-unknown --manifest-path <ticket>/🧪️energy-focused/Cargo.toml` — exit 0 (`📝️p7c-focused-wasm-2.txt`).
- Product structured gate: the preserved package check stopped before Energy on exactly 31 concurrent `semio-framework-plugin` diagnostics and emitted no Energy diagnostic (`📝️p7c-energy-check-1.json`, `📝️p7c-energy-check-1.stderr`). The focused harness mounts 50 production Energy modules; its only shim is ticket-local stdio EPW surface data needed to isolate the changing product dependency wall.

## Test Coverage

- output and checkpoint parity across fuel sizes;
- active-timestep checkpoint/restore;
- cancellation and stale-generation no-mutation;
- deterministic repeated runs and run-period/calendar parity;
- full-topology plant/PV/airflow/daylight execution;
- sizing, final summaries, metrics, and economics;
- 16,384-surface one-work-unit `<8 ms` adversarial watchdog;
- end-to-end step watchdog including previews, checkpoints, finalization, and commit encoding.

One release run executed concurrently with a compiler-heavy wasm build and observed an 8.81 ms scheduler outlier. The dedicated adversarial test remained green; an isolated uninstrumented rerun of the complete 12-test release suite passed with the hard `<8 ms` assertions unchanged. No timing threshold was relaxed.

## 2026-08-21 current-tree reverification

- Focused debug: **238/238 passed**, with only the three named ticket-stub EPW codec tests filtered.
- Focused release simulation/job suite: **12/12 passed**.
- `wasm32-unknown-unknown`: passed.
- `wasm32-wasip2`: passed.
- Removed two superseded whole-weather-vector helpers; production already resolves one deterministic
  weather record per resumable cursor step. The timestep-stage accessor is now test-only. The final
  focused native and portable checks emit no Energy warning.

The ticket remains open as required.

## 2026-08-21 product-crate restoration

- Repaired the product crate's stale de-async call shapes with compiler-driven codemod runs
  `r13-nvewnkgs` and `r13-kt0n71rv`, followed by typed manual normalization of the generated
  snapshot/UI test expressions. Pure snapshot, model, diff, binary-encoding and test-fixture helpers
  are synchronous again; genuinely suspending testkit calls retain their awaits.
- `cargo test -p semio-s-plugin-energy --lib --no-run`: passed for the complete product crate.
- `cargo clippy -p semio-s-plugin-energy --lib --no-deps -- -D warnings`: passed with zero
  Energy-owned warning. Dependency warnings remain owned by their respective framework/stdio
  packages.
- `bun nx run @semio-tech/energy-plugin:describe --skip-nx-cache`: passed. The regenerated portable
  artifact was described to `🛂️descriptor.semio` and `🔣️descriptor.json` with SHA-256
  `ce7be823d0478663fb72cd3195f2085073ff9772432f1d25bac1d014b2196caf`.
- Full runtime behavior is green when the test worker stack is raised to 16 MiB: **292 passed, 0
  failed** in 0.99 s. The identical exact viewer test passes 1/1 at that stack size.
- The ordinary default-stack product gate currently exposes a Phase-8 regression rather than an
  Energy kernel failure: the new generic `AppCommandJob<A>` carries the complete typed snapshot and
  adjacent state by value through factory/type-erasure frames before boxing, and
  `plugin::surface_tests::energy_model_viewer_never_mutates` deterministically overflows the default
  test-thread stack. The exact test fails alone with the default stack and passes alone with
  `RUST_MIN_STACK=16777216`; therefore the raised-stack run is diagnostic evidence, not the exit
  gate. Phase 8 owns boxing that payload boundary, after which the ordinary 292-test command must be
  rerun before P7c closes.
