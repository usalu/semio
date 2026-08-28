# Neutral Resident Permit — Native GREEN R57

Canonical UI contract selector `retained_resident_permit_`: actual 3 passed, 0 failed, 147 skipped; nextest 0.400s, exit 0. Node Buffer/Ajv source oracle passed 63 checks.

```text
[DEBUG] resident-permit small=9 slots=64 aggregate=33554432 paired-return=0,65536 explicit-close-drop-does-not-return-again=true
Summary [0.400s] 3 tests run: 3 passed, 147 skipped
NX Successfully ran target test for project @semio-tech/ui-contract-rs
```

Native scope: nine small reservations (not nine rendered surfaces), four full-byte reservations and plus-one rejection, 64-slot exhaustion, paired root/output return, exact epoch reuse, explicit close disarming Drop, held-ledger close/drain refusal with nonblocking deferred Drop, and 32 cross-worker return/drain/reuse races. The exact primitive keeps the previous 64-slot/32-MiB/131,076-item aggregate and 8-MiB/4,097-item per-reservation limits.

Runtime migration is next: the old runtime ledger must be removed and all real reservations must use this one authority. Until then, the new ledger is exercised only by its isolated native laws; no second runtime quota is enabled. Canonical document final-owner credit binding and original runtime R30/R31 remain unimplemented.

Raw: `🧪️member-ui-resident-permit-green-r57-native-2026-08-27.txt`. Nx appended its historical flaky-task notice because the same target previously ran intentional RED tests; the actual run exited 0 with all three selected laws passing.
