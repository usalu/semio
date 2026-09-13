# Native Dependency Boundary

## 2026-09-13

Nine root dependency targets now invoke the narrow caching/bootstrap/dependencies/native script directly. Python, Cargo, Go, browser tooling, WebAssembly tools, Trunk and development tools retain their existing locked commands; .NET and C++ retain aggregate completion leaves. All nine remain uncached with no deliverable outputs. Nx owns their prerequisite relationships. The root SetupScript delegates to the same implementation and awaits its completion.

The implementation imports only owned routing, process-budget, workspace-path, cache-path, Cargo-lock and child-process helpers. The pure wasm-bindgen lock parser moved to the narrow Cargo module and remains explicitly exported by the repository library. Tool work uses asynchronous process ownership, progress, signal cancellation and an optional orchestrator budget. Cancellation during a version probe cannot trigger installation; invalid binding locks fail before acquisition. No new runtime library is required.

The Trunk contract first failed because its target still loaded the root router (295 ms). After the change it passed in 454 ms, including ready/stale/missing tool vectors, probe cancellation, native Nx task graph and Bun/smol-toml configuration parity. esbuild independently verifies the command import closure excludes the root router and application taxonomy.

The new schema-validated native dependency fixture covers all nine command/environment contracts. Its native proof passed in 4.8 seconds: Cargo fetch and Cargo metadata agree in a private workspace, two native Nx runs execute the uncached Cargo leaf, both preserve committed lock bytes, and a stale lock is rejected without modification. The fixture's root application router deliberately throws. No compiler deliverables are created. Rejected CLI overrides, missing binding lock and non-system external imports are checked. Successful private fixture output was removed.

The actual repository target workspace:deps-trunk passed through Nx (1.2 seconds task-run duration, 322 ms task critical path, 0/1 cache hits). Installed Trunk, wasm-bindgen and the Rust WebAssembly target were already ready. This demonstrates native preparation no longer depends on application taxonomy; it does not certify application generation, compilation or other platforms.

Existing launch entries still name the same workspace:deps-* targets, so no new editor command is needed. Remaining end-to-end WGPU/full-suite qualification is recorded separately.

Sources: [Cargo fetch lockfile semantics](https://doc.rust-lang.org/cargo/commands/cargo-fetch.html), installed Nx 23.2.0 native task graph and execution, installed esbuild, Cargo and smol-toml oracles. The attempted Nx configure-caching documentation URL was unavailable; target behavior was verified with the installed implementation.

The subsequent full repo:test ran for 5m 29s and passed the native setup, finite WGPU compiler/server, native preparation, .NET and Python output proofs. It failed later because the Hub test still referenced the removed package-local Vite configuration. The fixture now names its current builder-owned configuration; its focused Nx/esbuild/browser-definition proof passed in 3.5 seconds. This full invocation is not a full-suite pass.
