# Persistent Credit Regression — R35–R36

R35 used the incorrect filter `retained_credit`; it selected zero tests, skipped 101, and supplies no native proof despite exit 0.

R36 used the exact existing selector:

`bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='--lib persistent_credit_transfers_ -- --nocapture'`.

```text
test reconcile::tests::persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement ... ok
Summary [0.016s] 1 test run: 1 passed, 100 skipped
NX Successfully ran target test for project @semio-tech/ui-runtime-rs
```

Exit 0. Raw files: `🧪️member-runtime-resident-credit-r35-native-2026-08-27.txt`, `🧪️member-runtime-resident-credit-r36-native-2026-08-27.txt`. This is the existing runtime paired-credit regression after factoring, not the canonical final-reader join.
