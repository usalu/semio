# Runtime Resident Ledger Join — Native R32

The canonical UI runtime test executed and failed semantically before the production ledger join. The exact runtime reservation was released before the assertion, retaining strict ownership cleanup.

Command: `bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='--lib surface_ownership_resident_reservation_uses_one_shared_aggregate_ledger -- --nocapture'`, using the existing master-ticket Cargo target and standard serial build environment.

Actual output:

```text
[DEBUG] surface-ownership-oracle checks=26
[DEBUG] runtime-resident-join expected-bytes=65536 observed-bytes=0 expected-slots=1 observed-slots=0
assertion `left == right` failed: the runtime must not retain an independent second aggregate ledger
  left: 0
 right: 65536
Summary [0.056s] 1 test run: 0 passed, 1 failed, 99 skipped
NX Running target test for project @semio-tech/ui-runtime-rs failed
```

Raw output: `🧪️member-runtime-resident-join-red-r32-native-2026-08-27.txt`. Exit status: 1. This is the expected independent-ledger RED, not a compile failure. The next change replaces the old runtime ledger with the neutral permit authority; no quota increase or canonical document-root completion is claimed.
