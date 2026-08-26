# Publication Fixed-Operation Registry Cohort

## Scope

The authoritative input is `📊️coordinator-official-tool-jobs-current-2026-08-26.json`, whose
`globalPayloadStores` ledger contains exactly 36 production rows. This cohort owns the three
identical publication-authority rows only:

- Procedural2d `LEASES`;
- Procedural3d `LEASES`;
- Process3d `LEASES`.

Puzzle/BoardFill and Lowpoly/mesh sources were excluded.

## Production repair

Each former `OnceLock<Mutex<[Option<PublicationLease>; 4]>>` is now the exact shared
`FixedOperationRegistry<PublicationLease, 4>`. Each retained lease implements the shared
`FixedOperationOwner` cancellation, begin-close, bounded close, and terminal-empty laws. Admission,
refresh, validation, atomic validation, item credit, and release all address an exact
`FixedOperationKey(operation, generation)`.

The shared registry now exposes bounded `get_operation` and `get_operation_mut` queries. They scan
only the registry's statically bounded capacity, preserve same-operation ABA refusal before exact
release, and never expose an owner from another operation. Its hostile test asserts exact key and
identity, mutable identity preservation, and cross-operation isolation. The existing shared fixture
continues to cover maximum/maximum-plus-one owner handback, stale generation, interrupted close,
cancel, terminal empty, and repeated close. Existing per-app publication laws continue to cover
freshness, parent revision, atomic validation, and release.

## Source and static evidence

```text
rustfmt +nightly-2026-07-07 --edition 2024 <shared job + three publication sources>
exit 0

git diff --check -- <shared job + three publication sources>
exit 0

bun ./📜️script.ts verify interactivity tool-jobs --self-test
exit 0; self-tests=468 clean

bun ./📜️script.ts verify interactivity tool-jobs
exit 1 on expected aggregate failures;
process-global payload candidates=33 (authoritative 36 baseline minus exactly this 3-row cohort)
```

No Cargo, Nx, or runtime pass is claimed at this checkpoint because the shared compiler slot was
owned by the Puzzle cohort. Focused shared-registry and three app runtime reruns remain queued.
