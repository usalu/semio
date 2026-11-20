# Value frame refactor completion

## Half-done state
Another developer de-genericised the native receiving-effect frame logic in `🌱️value` so it compiles once per host and failure (`OriginalProducerDrive`, `BoundProducerDrive`, `drive_original_producer_dyn`, `drive_original_effect_dyn`). The driver macro dropped `drive_original_effect_bound_with`, but `advance_resident_success` in the `native_resident_success_actor!` macro still called it, so nothing compiled.

## Completed
- Added `drive_original_effect_bound_with<P>` to `🧳️frame/🎛️driver/🦀️.rs`. It wraps the invoke closure in `BoundProducerDrive` and routes through `drive_original_effect_dyn` with `retain_receipt` and `retain_work`.
- Earlier fixes by the requester: a stray `)` and the `admit_frame_slot` u64 to u128 change.

## Results
- Describe plugin `cargo check` (release): green (`🗑️generated/value-refactor/check1.log`).
- Hub `check --target wasm32-wasip2 --workspace`: the value and frame crates compile with no errors. It still fails, in a separate in-progress change by another developer (`RetainedOrderedMap` migration): `BTreeMap` fields in `💡️inferences/🎭️phase-visibility` lacking `RetainedClone`, `ModelSnapshot` in `🚪️io/💬️bcf` expecting `RetainedOrderedMap`, and missing `entry`/`insert`/`remove` on `RetainedOrderedMap` (`check-hub2.log`). A first run also hit a half-edited `StepContext::last_claim_reason`, which has since landed.
- Unit tests for `♻️retirement` and `🛬️decode`: not run (hub blocked by the unrelated edits; long builds).
