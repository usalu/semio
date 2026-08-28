# Runtime Output Pool R62: Expected Compile RED

Canonical command: `SEMIO_COVERAGE=0 bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='exhaustive --lib surface_output_pool_ -- --nocapture'`, with the existing shared target and serialized Rust environment.

Actual exit: 1. No native test executed. The schema-first two tests refer to the intentionally absent `SurfaceReconcileOutputs` API.

Actual captured diagnostics:

```text
[DEBUG] surface-ownership-oracle checks=40
error[E0433]: cannot find type `SurfaceReconcileOutputs` in this scope
error[E0433]: cannot find type `SurfaceReconcileOutputs` in this scope
error: could not compile `semio-framework-ui-runtime` (lib test) due to 2 previous errors; 9 warnings emitted
NX Running target test for project @semio-tech/ui-runtime-rs failed
```

Full retained output: `🧪️member-runtime-output-pool-red-r62-native-2026-08-27.txt`.

This is missing-API TDD evidence only. Fixed-pool admission, transaction integration, full resident accounting, and the original inline census failure remain open. No limits, assertions, or production ownership guards were relaxed.
