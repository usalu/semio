# R19 — Typed Plugin App Fleet A

## Scope

This packet migrates only the mathematical, animate, DAG, process, flow, lowpoly, writer, shooting, sequence, raster, and reasoning plugin roots, their Rust glue exports, and the direct internal macro dependency needed by those roots. Shared framework, Store, stdio, plugin-host, renderer, and every other plugin remain outside the packet.

## Closed app fleets

Every owned plugin now has a plugin-specific closed `PluginApp` enum, typed `Plugin<AppEnum>` return, typed `Plugin::<AppEnum>::builder`, and two-argument `plugin_exports!` invocation.

Concrete builder-registered surfaces are represented one-for-one:

- `ProcessApps`: `Process3dEditor`, `Process3dViewer`.
- `FlowApps`: `FlowEditor`, `FlowViewer`.
- `LowpolyApps`: `LowpolyEditor`, `LowpolyViewer`.
- `ShootingApps`: `ShootingEditor`, `ShootingViewer`.
- `RasterApps`: `RasterEditor`, `RasterViewer`.

Mathematical, animate, DAG, writer, sequence, and reasoning register their surfaces through declaration trees rather than direct `.editor::<T>` or `.viewer::<T>` builder calls. They therefore own dedicated zero-variant `MathematicalApps`, `AnimateApps`, `DagApps`, `WriterApps`, `SequenceApps`, and `ReasoningApps` enums. None falls back to `NoPluginApp`.

All eleven roots import the exact closed-enum support surface: `PluginApp`, `__semio_dispatch_PluginApp`, and `plugin_app_close_prelude::*`. All eleven manifests directly depend on the internal `semio-framework-dispatch-macros` crate; animate and process already carried the row, while the other nine received the matching path dependency.

## Verification

- Owned one-argument export/default builder census: zero matches for one-argument `plugin_exports!`, `Result<Plugin, ...>`, or untyped `Plugin::builder` across the eleven root/glue pairs.
- Direct internal macro dependency census: 11/11 manifests.
- Focused `rustfmt --check` over all eleven root components: passed.
- The first all-package native check was blocked before owned crates by five concurrently repaired Store stale awaits at lines 8967–8975.
- The second and third all-package native checks advanced to the Store helper sweep and stopped at Store line 8780 (`Future<Result<String, VcsError>>` used as a concrete result). This is outside the packet and is not recorded as an owned failure.
- After those walls cleared, the all-eleven native check compiled the shared typed runtime and reached the owned roots. The new closed app enums and two-argument exports produced no errors. The boundary then stopped in pre-existing product-wide de-async fallout: reasoning reached its typed root before failing across its artifact tree and its trinity dependency; an isolated mathematical check likewise reached `MathematicalApps` and stopped on 2,450 stale async diagnostics in mathematical presence/config/editor/CAS/polynomial code.
- Required Bun+Nx routing was exercised with `bun nx run @semio-tech/animate-plugin:test-quick`. It reached `semio-s-plugin-animate`, including the new typed root, then stopped on 2,229 pre-existing product/test diagnostics such as removed mutation testkit helpers, the missing `Animations` closed enum, and stale futures throughout the animate artifact tree. No diagnostic named the typed root, `AnimateApps`, `Plugin<AnimateApps>`, or the two-argument export.
- Because native product compilation does not pass the artifact-tree wall, release, wasm/describe, and clippy cannot reach an owned binary result yet. Repeating them would classify the same non-owned de-async product wall under a different target/profile rather than verify this root-only packet.

## Opening Config and Process Transport Follow-up

The compiler follow-up removed the 24 stale asynchronous calls from the pure opening-preferences mutation dispatcher and its committed set/clear fixture suites. `apply_opening_config_mutation` is now direct synchronous mutation/diff/apply composition; all affected tests use ordinary `#[test]` functions. The owned `.await`/`async_test`/`block_on` census is zero. The process-transport decoder's `PipeState` now derives `Debug`, satisfying `Result::expect_err` without changing transport behavior.

- Focused `rustfmt --check` over the four follow-up files: passed.
- `cargo test -p semio-framework-plugin-host --lib --no-run`: passed with zero diagnostics in the four owned files.
- All 17 opening-config tests passed, including both seven-test committed fixture suites and the three dispatcher outcome/inverse tests.
- Every executed process-transport test passed. The external-component rebuild test remained its existing ignored case because `SEMIO_SCALE_FIXTURE_WASM` was not supplied.
- Full plugin-host boundary: 143 passed, 2 failed, 1 ignored. The two failures are unrelated shard-scheduler assertions in `cancel_job_effect_stops_a_job_before_it_is_ever_stepped` and `exclusive_placement_is_stepped_before_inline_placement_admitted_the_same_pump` (observed step count 2, expected 1); they were reported to the shard/runtime owner and are outside this follow-up.
- `wasm32-wasip2` plugin-host library check reached the target dependency graph but failed before plugin-host compilation on native-only dependency configuration: Tokio rejected unsupported WASM features and `zstd-sys` could not compile its C sources for `wasm32-unknown-wasip2`. Neither diagnostic names an owned opening-config/process-transport source file.
