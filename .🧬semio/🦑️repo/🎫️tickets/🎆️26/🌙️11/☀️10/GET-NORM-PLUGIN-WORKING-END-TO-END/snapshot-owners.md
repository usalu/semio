# snapshot-owners

## Finding 1: sqlite snapshot helpers did not lend the original construction recipient
- Symptom (din4108 `--lib`): 7 `sqlite_snapshot_tests` fail with `UnsupportedOwner: native encoder requires its original construction recipient` / `native construction requires its original recipient`.
- Root cause: the snapshot codecs quote and post every physical effect against the caller's original construction recipient (`NativeEncodeControl::construction_grant`, `submit_constructed_progress`; decode `hold_construction`). Production callers lend it together with the original Work loan around the native control (`with_work_recipient` then `with_construction_recipient`, see `semio_framework_job` encoding/decoding contexts). The test helpers in `🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners` only installed the retirement recipient.
- Fix (helpers only, no framework change): `original-snapshot-owners/🦀️.rs`
  - `Physical` (`OriginalConstructionRecipient`): grants `original_grant()` and totals every accepted receipt; `Lent` bundles `finite_work(u64::MAX)` and the construction authority; `lend!` lends both loans around a native control of either direction.
  - `with_decoding` / `with_encoding` create the owner inside the lent scope (so `decode_native`, `encode_native`, `export_sqlite`, `import_sqlite` all inherit it).
  - New `close_decode`: a selected original failure birth (`requires_original_birth`) is driven through `with_retirement(..).close_original_birth_step` before the general close effect; used by `with_decoding` and `io_run`'s decode direction (previously `original failure birth must precede the general effect`).
  - `io_run` (async) is not lent: a loan cannot be held across `.await`; it keeps the retirement recipient only.
- Status: run 1 (`d4108-snap`) moved the 7 failures from `UnsupportedOwner` to (a) decode close birth (fixed by `close_decode`, run 2 pending) and (b) `OwnershipLimit: snapshot retirement requires a larger original grant` in the encode-side tests (`genuine_output_admits_exact_row_and_file_frontiers`, `complete_independent_native_semantic_limits`) which means the effect quote returned not-admitted without work. Open: whether `FiniteWork`/`Physical` quoting differs from what production lends (candidate: per-effect grant equal to the quoted demand instead of the constant `original_grant()`, as `Authorities` in `value/testing/admission/driven.rs` does with its shared grant cell). Cancellation (`reached`/`rejected`) and the exact-limits assertion are consequences of the same refusal and were not separately analysed.

## Finding 2: din4108-mutations `document_text_round_trips_through_store`
- Symptom: `catalog constructor exceeds original full authority` (WorkLimit) from `DocumentStoreOwners::admit_constructor`.
- Root cause: the over-quote is not in the constructor. `complete_unscheduled_catalog` (store `initialization/owners`) derives the per-ticket grant from the quote `constructor_demands(0)` but copied only `capacity_bytes` and `depth`, leaving `maximum_copy_bytes` and `maximum_release_bytes` at 0, so any ticket with a non-zero copy/release demand was refused. The quote itself is fine.
- Fix: grant all five axes from the quote (`maximum_copy_bytes: demand.copy_bytes`, `maximum_release_bytes: demand.release_bytes`); `fund_document_store_owners` repeated the same loop, so it now delegates to `complete_unscheduled_catalog` (one copy of the logic). No magic numbers.
- Verified: run 1 moved this test past the constructor; it now fails later in `store.dispatch(Apply)` with `NativeEncoding(UnsupportedOwner: original work requires its original caller loan)`: the dispatch path of the plain test store encodes the operation under a native control that has no Work loan from the caller (same ownership class as Finding 1, in the store test-support / dispatch, not yet fixed). The same pattern recurs in every norm family's `binary` mutation tests and the raster plugin.

## Changed files
- 🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🦀️.rs

## Verification
- `wave-tests-ws.sh d4108-snap test din4108 --lib -- sqlite_snapshot document_text_round_trips` (log `🗑️generated/d4108-snap.log`, exit 101: 7/14 sqlite pass-through, see above).
- Rerun `d4108-snap2` queued/running with the `close_decode` fix; result in `🗑️generated/d4108-snap2.log`.
