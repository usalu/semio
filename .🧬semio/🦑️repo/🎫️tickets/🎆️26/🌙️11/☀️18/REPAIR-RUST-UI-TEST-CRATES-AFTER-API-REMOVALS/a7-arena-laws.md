# 🏟️ A7 Arena Laws Repair

## Result
- `--lib -- arena_laws`: `test result: ok. 5 passed; 0 failed` (log `🗑️generated/a7-arena-1.log`).
- Whole crate `semio-ui-contract-values-laws`: `test result: FAILED. 40 passed; 3 failed` (log `🗑️generated/a7-full-1.log`), down from 19 passed / 24 failed. The "UI arena already has an actual lifetime owner" cascade is gone.
- The 3 failures left do not come from the arena laws:
  - `bounded_receiving_tests::native_bounded_ui_collections_preserve_original_owners_on_refusal_and_cancellation`: "native receiver header lacks an admitted real terminal destructor". This is the other agent's port.
  - `bounded_receiving_tests::native_recursive_ui_values_keep_scalar_and_empty_owners_without_global_arena_birth`: `Number(9007199254740992.0) != Number(9007199254740992)`. Also the other agent's port.
  - `unit::fixed_owners_keep_bounded_payloads_off_the_stack` (`🔬️unit/🦀️.rs:62`): `size_of::<UiFixedList<[u8;32769],1>>()` is 64, `size_of::<UiFixedList<u8,65536>>()` is 48. This is fixed-list layout, not the arena. It was already failing before this change.

## Root causes
1. **Opening laws, `(64,0)` != `(0,0)`. Production fix.** `UiArenaRegistry` used `std::sync::Mutex`. On macOS (and other pthread targets) std lazily boxes a 64-byte `pthread_mutex_t` the first time a lock is taken, and a static never frees it. So the first `UiArenaRegistration::admit` on a registry made a physical allocation it did not receipt. The resident static budget (`resident_static_backing_bytes = size_of::<UiArenaRegistry>()`) also assumes the registry needs no heap. On Linux the futex mutex does not allocate, so the law only fails on macOS.
   - Fix: the registry now uses an allocation-free poisoning lock (`held`/`poisoned` `AtomicBool` + `UnsafeCell`). This follows the existing pattern in `🌱️value/🔗️read` `StateLock`.
   - Semantics kept: `borrow` uses try-lock and refuses when poisoned. `borrow_blocking` spins and yields. `admit` refuses when poisoned. The owner's `close_step` and `recover_borrow_step` continue through poison and clear it. `UiArenaBorrow` stays `!Send`.
2. **Source law, `born` 7056 != `freed` 192. Law updated to the current contract.** A native recipient now needs the Source's resident return escrow table (`admit_original_return_table` inside `prepare_return`). That table is born on the first `admit_original_native` turn and is only released when the Source root retires (`caller.retire`), outside the measured lifetime.
   - The law now births the table first with `caller.admit_return_table()`, as the job crate's own native-recipient laws do.
   - The previously unobserved `return_slot_step` turns are now observed: heap == receipt is asserted and they count towards born/freed.
   - Intent kept: Source custody precedes arena allocation, every allocation is receipted per turn, and born == freed for the lifetime.
3. **Cascade, a production-rooted panic.** `normal_arena_access_uses_the_actual_registered_source_backing` admits into the process-wide `UI_ARENA_REGISTRY`. It then panicked in `Bank::post` (cause 1) with the pointer still published. The registration was dropped during the unwind, so every later `UiArenaFixture` saw an owned registry. Fixing cause 1 removes the cascade.
   - Hardening: `original_…` and `arena_opening_quote_…` (opening laws) and the source law now hold `ui_arena_serial()`. They touch the process-wide handbacks (`UI_VALUE_HANDBACKS`), which the serial lock documents itself as covering.

## Files
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🏟️collection-arena/📌️registration/🦀️.rs` (production: allocation-free registry lock)
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🏟️collection-arena/🎟️source/🧪️tests/🦀️.rs` (law: return table born first, return-slot turns observed, serial)
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🏟️collection-arena/🌱️opening/🧪️tests/🦀️.rs` (law: serial)
- Ticket scripts: `a7-registry-lock.py`, `a7-source-law.py`, `a7-serial-laws.py`
- `semio-ui-contract-values` source boundary: about 256015 of 262144 bytes.

## Note
Other process-wide `std::sync::Mutex` statics on macOS (e.g. `🎟️resident` `RESIDENT_LEDGER`) have the same unreceipted lazy 64-byte box. They are not covered by these laws and were left as they are.

## Follow-up: `unit::fixed_owners_keep_bounded_payloads_off_the_stack` (64 vs 48)
- **What grew:** `UiFixedList<T,N>` holds only `semio_framework_value_persistent::list::PagedList<T,N>`. That owner is now `root: Vec<Page<T>>` (3 words) plus `length`, `capacity`, `allocated`, `payload_page_bytes` and `reservation_epoch`, which is 8 words. The UI law still hardcoded 6 words, from before the per-owner payload page ceiling and the reservation epoch were added. The value list's own law already pins the canonical layout: `🌱️value/📋️list/🧪️tests/📏️final` asserts `ownerWords == 8` from `🧫️fixtures/📏️exact-final-extent.json`.
- **Verdict:** the bound was stale. The production layout is correct, and payloads still live in paged backing rather than inline.
- **Fix:** the UI law now reads `ownerWords` from that same shared fixture (`🧰️framework/🔨️modules/🌱️value/📋️list/🧪️tests/🧫️fixtures/📏️exact-final-extent.json`) instead of a literal. Both laws now follow one fixture, so the bound cannot drift again. No TS twin carries this value. File: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🧪️tests/🔬️unit/🦀️.rs`.
- **Crate result** (log `🗑️generated/a7-full-2.log`): `test result: FAILED. 41 passed; 2 failed`. The only failures are the 2 `bounded_receiving_tests` laws, which belong to the other agent.
