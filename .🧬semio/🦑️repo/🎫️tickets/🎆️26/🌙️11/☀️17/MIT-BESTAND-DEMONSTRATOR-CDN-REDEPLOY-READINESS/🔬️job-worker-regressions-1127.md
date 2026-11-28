# Job worker and job context regressions (2026-11-27)

Result: `semio-framework-job-worker` 31 of 31 green (parallel, `--test-threads=1` and one process per test). `semio-framework-job-context` re-run, see the end. `quote_floor` was not involved.

Method note: running the worker suite in one process cascades. A test that panics leaks session slots and payload bytes, so later admissions stall. Only isolated runs (one process per test, `CARGO_MANIFEST_DIR` set to the crate) show the true failures.

## Production bugs fixed (a)

1. Idle return frame close stalled at zero capacity. `OriginalReturnFrame` close of an untouched payload went through a retirement frontier that first reserves a cursor page (392 bytes capacity, 424 copy). Any close grant without capacity never finished ("original owner retained beyond declared finite close control", about 14 tests). Fix: an untouched frame removes its payload by one metadata step (demand depth 1, one performed work unit). File: `job/⚠️failure/📦️frame/🦀️.rs`.
2. Double-charged work in `RetainedPayloadBuilder::close_step`. With a reservation holding a ledger, the guard performed one work unit and then delegated to `close_ledger`, which performed another: "original effect close performed more work than it settled". Fix: the metadata-removal guard excludes the reservation case. File: `job/📬️outcome/📦️builder/🦀️.rs`.
3. Close quote smaller than charge. The receiver redeemed a held return witness (118 copy bytes locally) while quoting only depth 1, so an exactly-granted close would be refused. Fix: `OriginalReturnClaimState::witness_copy_bytes` and the receiver quote includes it. Files: `job/📬️outcome/🪙️budget/🦀️.rs`, `job/📬️outcome/⏸️continuation/🎟️origin/🦀️.rs`.

## Stale tests and fixtures updated to the current design (b)

- Return origin protocol: one action per admitted turn, outcome visible only after the receipt is taken (retained-ownership begin-close and wake tests, erasure test loop).
- Begin close and Outcome phases: outcome acknowledgement now closes inside the Payload phase; the old ladder asserted `Outcome` phases.
- Mounted close fixture (v1, Nov 9 phase names `PreadmittedFault`/`PayloadLedger`): regenerated for 25 turns and the current phases, plus schema and the SQLite/RFC6902 TS cross-check (`faultPageBytes` added, passes).
- Receipts: Source birth service (about 6864 bytes, 9 items per turn) is not session-owned, so conservation tests subtract `born_bytes()` and denied-turn tests compare `business_receipt()`.
- Indivisible loan admission costs 4 work units, so the admission law quantum 1 became 4 (`case.json`, schema).
- Erasure fixture grant raised to 16 items, 16384 copy, 262144 capacity so the Source birth chain can run (fixture and schema).
- Microsecond test: only the fault page counts toward `owned_bytes`; a quarantine verdict on a step that is already terminal keeps the Complete outcome (verdict stays visible). Open design question: a clock-fault verdict on a same-turn terminal step does not convert it to Fault.
- Slot test admits with the caller's remaining grant. Wake close test uses new `bank_run_invoked` so native retirement gets its producer invocation.
- Testing caller: birth close steps are not checked against the Source-root quote; added `bank_run_invoked`.

Removed: the temporary `[DEBUG]` zero-run `eprintln!` in `📬️outcome/👷️worker/🦀️.rs` (it allocated inside measured turns, causing "an allocation of this turn was not receipted in it").

## Job context
`native_parent_survives_foreign_context_cancellation_and_return` passed on the current tree before any edit of mine.
