# jobs3b: semio-framework-job-failure test targets

All targets compile and pass (`repo cargo ... test -p semio-framework-job-failure`, 0 failures).

| target | status |
|---|---|
| lib tests | ok (0 tests) |
| original-turn-failure | 18/18 |
| original-domain-reservation | 2/2 |
| original-source-qualification | 1/1 |
| original-work-failure | 1/1 |
| original-native-snapshot-projection | 3/3 |
| original-json-failure-projection | 2/2 |

## Test re-expression (no tests deleted, no floors)
- `ControlledRetirement::step(body, effect)` is effect-shaped and needs an original invocation, so the bank cannot drive it. Closes now run through `close_original_effect_step` under `OriginalJobFixtureCaller` turns (cancelled first). A `CloseCaller` helper widens a loan only to the owner's own quoted demand (copy, capacity, release, depth).
- Boxed diagnostics test: `boxed_original_retirement` runs under `EffectBank`; the payload then closes via `ControlledRetirement<Box<T>>` (a Box<dyn cursor> cannot be stepped directly any more).
- Prepared native work: `with_native_receiving(&mut out, |control, grant| ..)` plus `run_prepared_work`; child work asserted on the native ledger.
- Foreign domain publication now `Err` (production changed in 729); fixture/schema `foreignRecord` removed.
- Snapshot projection fixture keys updated (effect, snapshotReceiving, snapshotRetirement). `NativeAllocationRefusal::Owned` via `.into()`.
- New dev-dep `semio-framework-value-testing-effect-bank`; `NativeCallFailure` replaces `NativeDecodeCallFailure`.

## Production bugs fixed
- `OriginalDomainPrimary::retirement_birth_bytes` was `Some(0)`; must equal `size_of::<Box<dyn RetirementCursor>>()` (cursor-layout check refused every domain close).
- `DomainFrame` lacked `next_computational_work` ("retirement cursor lacks a work quote") and its `terminal_release_bytes` omitted the 16 B outer cursor shell.
- Snapshot projection ignored new fields: `workOverrun`, `receiptCompletion` (receiving), `context`, `markerFailure`, `workOverrun`, `physicalReceipt` (effect), `allocationRefusal` (retirement).
- Diagnostic crate `FaultParams`/`FaultScope` `boxed_original_retirement` (`⚠️diagnostic/🧬️retirement/{🎟️params,🎯️scope}`) did not record work or the allocation marker in the effect ("reported work differs from performed receipt"); now take the effect.

## Notes for others
- `Box<dyn RetirementCursor>::terminal_release_bytes` delegates only to the inner cursor; types using it as `RetirementCursor` must add the shell themselves (as DomainFrame now does).
- Nothing needed in other job crates.
