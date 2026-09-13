# UI Retained Hit Registry Feature Ownership

Status: engine-tier native laws passed 2/2 on the configured 2 MiB stack; default-tier plugin compilation also passed.

The UI input module is intentionally mounted without the `wgpu-engine` feature. The recently added retained hit registry referenced engine-only arena, tree, mounted layout and layout modules from this unconditional module. The Generation2 executor observed E0433 at those references during its registered plugin check, before reaching the retained config loader. A later run stopped earlier in the concurrent ordered owner cutover, so that later failure does not validate this correction.

The retained registration type, implementation, private tree/scene helpers, registration function and retained registry test module now use the same `wgpu-engine` gate as the types they consume. Generic pointer input remains available in the target-neutral UI tier. The scene argument is already obtained by matching `&node.spec.0`; it was not changed based on a potentially cascading unresolved-type diagnostic.

The follow-up default-tier check found the remaining unconditional `RetainedHitRegistration` reexport. It now shares the engine feature gate as well. All other symbol consumers found in the actual source are already inside the gated engine module.

The existing retained hit registry laws with `wgpu-engine` passed 2/2 on the configured 2 MiB stack. The registered `@semio-tech/ui-rs:test-wgpu-engine` target exited successfully in 11.9 seconds, with 0/3 cache hits; nextest reported 2 passed, 402 skipped, in 0.039 seconds. Evidence: `🗑️generated/ui-retained-hit-feature-native-3.log`.

The default-tier registered `@semio-tech/framework-plugin:check` also passed in 1.8 seconds, with 1/4 cache hits. Root inspected the successful Rust/Nx footer in `🗑️generated/window-config-retained-api-coherence-3.log`. This establishes compile coherence of that consumer; it does not establish retained loader behavioral acceptance.

The first root native attempt stopped in Nx plugin loading because it invoked Nx directly under Bun from the full workspace. The second used the registered Bun bootstrap but stopped on shell quoting of the nextest expression. Neither earlier attempt reached Rust compilation or tests.

## Completed Temporary Nx Cache Removal

At 2026-09-13 03:44 UTC the volume had approximately 670 MiB free. Root removed only these confirmed completed ticket-local Nx workspace caches: `remodel-root-oracle`, `graph-check-generated`, `graph-generate`, `assets-build`, `native9`, `workspace-oracle`, `process3d-native-owner-2`, `native10`, `pcg-neutral-6`, `pcg-neutral-7`, `pcg-neutral-8`, and `pcg-native-green-4`. Free space after removal was 2.0 GiB. Shared Cargo output, the native Nx cache, active agent workspace data, all source inputs and test evidence logs were preserved.
