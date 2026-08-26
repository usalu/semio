# Process-Global Payload Closure Contract

## Live census

The 2026-08-26 full verifier reports 37 blocking process-global payload-store rows. Ten rows contain fixed-looking registries; the other 27 are scratch maps, resizable operation registries, ABI retention, or mutable payload owners.

The ten fixed-looking rows are:

1. Procedural2d publication leases.
2. Procedural3d publication leases.
3. FEM2d mounted sessions.
4. FEM3d backing recovery.
5. FEM3d mounted sessions.
6. Process3d publication leases.
7. Forms input registry.
8. Energy simulation registry and recovery slots.
9. Draw mutation arena process state.
10. Puzzle3d fill-envelope registry.

None is exempt today. A bespoke array or locally named `Registry` is not proof of operation ownership, saturation safety, freshness, or bounded retirement.

## Exact exemption boundary

`📜️script.ts` now exempts only a process-global owner whose declared Rust type directly contains fully qualified `semio_framework_job::FixedOperationRegistry<...>` behind `OnceLock<Mutex<...>>` or `RefCell<...>`. An imported/unqualified same-name type, a comment, a proof annotation, a raw fixed array, or a bespoke registry remains blocking.

This boundary delegates semantics to the shared executable contract, which already proves:

- exact `OperationId` plus `Generation` identity;
- fixed capacity and byte credit;
- maximum and maximum-plus-one exact owner handback;
- slot collision and saturation fail-closure;
- stale-generation and ABA rejection;
- cancellation and interrupted one-owner close;
- terminal-empty witnessing;
- language-neutral fixtures plus concurrent initialization timing.

## Required migrations

- Convert only genuinely process-wide operation scheduling owners to the shared registry and implement `FixedOperationOwner` with truthful retained-byte, cancel, begin-close, close-step, and terminal-empty behavior.
- Move every scratch, staging, mutable payload, resizable map, and ABI-retained message owner into its app/operation/bridge instance. Do not seek an exemption for those categories.
- Preserve exact rejected-owner handback and incrementally retire it. No `drop`, `mem::forget`, collection-wide scan, wrapping generation, or whole-payload clone is acceptable.
- Run maximum/maximum-plus-one, collision, stale/ABA, cancel, interrupted-close, native, Wasm, and actual Nx/app gates for each changed owner.

## Verifier evidence

`bun ./📜️script.ts verify interactivity tool-jobs --self-test` exits `0` with `self-tests=464 clean`. The full verifier exits nonzero only for the three live ledgers: 37 global stores, 35 import-media owners, and 744 command registrations.
