# Lowpoly Editor Framework View Re-exports

## Result

`semio_s_plugin_lowpoly::editor::lowpoly` now publicly re-exports `ArtifactView`, `ConfigView`, `Emit`, `Fault`, and `HistoryView` from `semio_framework_plugin`.

The generated Rust subject host for `command-lowpoly-1` can now express construction of the views and invocation of `commands::patch_object::handle` without declaring a direct framework dependency. The case asserts its emitted `RenameObject` document mutation and empty config mutation list.

## Verification Limitation

The repository test runner currently stops in its global contract gate before host materialization because of 1,814 pre-existing contract, fixture, dependency, oracle, and discovery breaches. A throwaway Cargo host with the exact generated Rust dependency set then reached the workspace baseline failure `E0433`: `semio_framework_os_kernel` references `zip` in its extension module without linking that crate. Consequently, neither the requested library check nor the generated-host runtime assertion can complete until the independent Cargo baseline failures are resolved.

## Generalization

Other editor crates that expose command handlers taking these framework-owned types should provide the same additive public shim at their idiomatic editor module boundary.
