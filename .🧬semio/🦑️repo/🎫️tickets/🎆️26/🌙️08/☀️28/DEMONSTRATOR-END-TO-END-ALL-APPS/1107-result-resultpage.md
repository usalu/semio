STATUS: wasm32-wasip2 GREEN (W-resultpage-1 rc 0, `-p semio-plugin-result-page`, 🧰️framework/Cargo.toml). Native W-resultpage-2 rc 0.

## Fixed (semio-plugin-result-page, `🔄️full-operation/📃️pending/🦀️.rs`)
- `PageEncodingReceiver::encode` now takes `FnOnce(usize, observer, &mut dyn NativeSendAllocationLoan) -> Result<(), ValueError>`. Both impls (`NativeDecodeControl`, `NativeRetirementDecodeControl`) share one `receive_page!` macro: it runs `with_encoding_receiver(&mut frame, &mut result, ..)` with a call-local `NativeEncodingReceiverFrame` and `NativeCallFailure<(), ValueError>`, then `settle_pair`, requires an empty `take_allocation_failure`, and `release_encoding_receiver`. Failure order: operation, context, receipt refusal, plumbing refusal. Success requires the output slot, so a closure that sets neither slot is an invariant fault, not a silent Ok.
- The frame is local, not a struct field: the receive completes inside one turn, so there is no cross-turn custody to keep resident (unlike the tick cursor, which polls Pending).
- `close_traversal(effect) -> OriginalRetirementPoll` forwards to `cursor.close_step(effect)`; `!framed` is a complete outcome carrying the invariant error. Mounted caller (`📃️mounted/🦀️.rs`) now uses `close_original_effect_step`; the two test call sites are ported too (unverified).
- `cursor.step` work is u128: narrowed with `u64::try_from(..).unwrap_or(u64::MAX)` so `PublicationPageWrite.work` and `cx.consume_fuel(u64)` stay unchanged (saturation over-charges, never under-charges).

## Shared additions
- `value/🛬️decode/♻️retirement/🦀️.rs`: `NativeRetirementDecodeControl::release_encoding_receiver(frame)` (delegates to the original decode control; additive, claimed in 1105b-claims).
- `plugin/🧪️testing/🧪️artifact-app-laws/🦀️.rs`: `assert_heap_witness_installed()` (allocates a probe Box inside `observe_heap_allocations_on_this_thread`, fails if it reads zero) called at the start of `take_fixture_result_page`.

## "48 B retained vs 0 measured"
Root cause is the harness, not the page: the energy mounted-tests binary had no `#[global_allocator] HeapWitness`, so `observe_heap_allocations_on_this_thread` read (0,0) while the page honestly reported 48 B retained capacity (runtime's 01:48 claim line adds the witness). Now a test binary without the witness fails loudly with a named cause instead of a number mismatch. Not re-run on energy (mounted tests not part of this proof).

## Not verified
- `pending/🧪️tests` and the artifact-app-laws crate were not compiled (test target / downstream crate); only the lib check ran.
