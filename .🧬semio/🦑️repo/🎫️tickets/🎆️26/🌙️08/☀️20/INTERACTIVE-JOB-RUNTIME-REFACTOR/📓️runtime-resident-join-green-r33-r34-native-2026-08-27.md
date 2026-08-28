# Runtime Shared Resident Ledger — Native R33–R34

The old independent runtime ledger and credit struct were removed. Runtime reservations, shrink, paired output split, and exact return now use the neutral `UiResidentPermit` authority. The existing 64 slots, 8 MiB per reservation, 32 MiB aggregate, and item limits are unchanged. Deferred scalar returns are serviced by alternating bounded maintenance; return of quota is not reported as semantic byte retirement.

R33 command: `bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='--lib surface_ownership_resident_ -- --nocapture'`.

```text
[DEBUG] runtime-resident-join expected-bytes=65536 observed-bytes=65536 expected-slots=1 observed-slots=1
[DEBUG] runtime-resident-return mutex-busy-keeps-credit=true maintenance-resumes=true exact-return=65536
Summary [0.022s] 2 tests run: 2 passed, 99 skipped
NX Successfully ran target test for project @semio-tech/ui-runtime-rs
```

R34 command: `bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='--lib retained_patch_handoff_ -- --nocapture'`.

```text
[DEBUG] patch-handoff unwind-frontiers=3 exact-payload-pointer=true exact-authority-count=1 publish-bytes=3514 ack-bytes=3680
Summary [0.051s] 3 tests run: 3 passed, 98 skipped
NX Successfully ran target test for project @semio-tech/ui-runtime-rs
```

Both commands exited 0. Raw logs: `🧪️member-runtime-resident-join-green-r33-native-2026-08-27.txt`, `🧪️member-runtime-resident-patch-r34-native-2026-08-27.txt`. Existing patch contention now holds the actual shared permit ledger through its read-only observation capability, not a mock or replacement ledger.

Canonical document-slot binding and final-reader credit retention are still the next implementation. Original runtime R30/R31 and the inline resident-census RED remain unchanged. These five focused passes are not a full runtime pass or nine-surface proof.
