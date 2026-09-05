# Exact Cargo Consumer Stage Environments

## Scope

- Kernel exact-law consumers now build with `SEMIO_BUILD_RUST_MIN_STACK` or 32 MiB and run list/native stages with 256 MiB.
- Hub exact-law consumers that previously forced 256 MiB for every stage now use the same split.
- Manual Cargo/probe routes, target identities, feature sets, law selectors, budgets, launch registrations, and Rust sources are unchanged.

## Implementation

Both task routers own one local `exactCargoStageEnvironments` helper returning:

- `env.RUST_MIN_STACK = SEMIO_BUILD_RUST_MIN_STACK ?? 33554432` for Cargo build.
- `nativeEnv.RUST_MIN_STACK = 268435456` for exact executable list and law stages.

The kernel router applies the helper to all ten `runExactCargoLaws` groups. The Hub router applies it to the thirteen exact groups that previously placed the 256 MiB value directly in `env`.

## Verification

- `NX_ISOLATE_PLUGINS=false bun ./📜️script.ts nx run '@semio-tech/framework-os-kernel:wal-writer-authority-check' --skip-nx-cache`: GREEN; `wal-writer-authority-independent-oracle: AJV=5 exact-u64=1 cases=3 mutations=6 remote=5 writer-slots=32 retained-result=1 directory-barriers=4 wal-open-owner=1`.
- `NX_ISOLATE_PLUGINS=false bun ./📜️script.ts nx run 'os-hub:presence-normalization-source-check' --skip-nx-cache`: GREEN; `presence-normalization-independent-oracle: AJV=1 LEB128=1 exact-vectors=17`.
- `git diff --check` for both task routers: GREEN.

These are source/task-router receipts. No Cargo build or native-law verdict was launched or claimed for this consumer-only change.
