# Stale Port: space collection and sqlite native-caller

Prefix `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/`. Build via `stale-cargo.sh` labels `coll-*`, `caller-*`.

## Result

| Crate | Before | After |
| --- | --- | --- |
| `🧪️tests/🪶️sqlite/🫴️native-caller` | lib: 13 errors | `cargo check --tests` clean, 0 tests |
| `🗿️artifacts/🗂️collection` | 12 errors (+3 dependency libs not compiling) | `cargo check --tests` clean, 15 of 29 pass |

## Dependency libs that did not compile (now fixed)

- `🧪️tests/🪶️sqlite/📏️preflight` (lib): `encode/decode_sqlite_snapshot_native` take an output slot and return `RetainedWorkOutcome<(), SqliteStageFailure>`; `ControlledRetirement::step(body, effect)` runs inside an `AuthoringWallet::run_settled`. Failure message now names the primary refusal.
- `🧰️framework/🔨️modules/🧪️test/⚖️typed-input-law` (lib): admission and `close_step` run through `AuthoringWallet` with the heap observed inside the effect.
- `🧪️tests/🪶️sqlite/🫴️native-caller` (lib): macro `original_native_caller!` now uses a `NativeAllocationAuthority` port (`with_port`, settles with `into_port`), installs work and construction loans (`with_original_authorities`, `install_work_recipient`, `install_construction_recipient`) and closes through `close_encode` / `close_decode` (birth-aware via `with_retirement`). New public helpers: `settle`, `authored_invoked`, `encode_step`, `decode_step`, `birth_required`, `slot`, `validated`, `received`, `primary`, `staged`, `with_original_authorities`, `with_port`. `with_native_control` installs both direction loans.

## Collection tests

- `🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🦀️.rs`: output-slot calls through `slot`/`validated`; `close` and `turn` use `authored_invoked`; `original!` macro installs loans and closes via `observed_decode`/`observed_encode`; the boxed-custody test drives the product `receive!` macro (`boxed_receiving`, `failing_decode`, `failing_encode`) instead of the removed `receive_sqlite_snapshot(Frame::new, ..)` shape. Operations 2 and 3 (validators with body failure injection) were dropped: the validators have no injectable body. Thread-local `BODY_DONE` replaces the macro-local flag.
- `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`: `exported`/`imported`/`decoded` adapt the codec fn pointers to output slots; the preflight test builds its controls inside the loans.
- `🧪️tests/🪶️sqlite/🔬️oracle` and the collection tests spawn `repo tools exec bun --` instead of bare `bun` (cross-platform, zero-touch; bare `bun` was not on PATH).
- Drop order: recipients are declared before controls (controls now have a Drop that borrows them).

## Remaining failures (14 collection tests; all product-side, not test shape)

1. Ledger exceeds accepted port events by exactly 392 B (`owned_bytes` 226944 vs 226552 etc.): the symbol-page signature of the marker defect the parent is chasing (borrowed_semantic_gate, erased_both_directions, native_input_retained, complete_native_semantic_text_roles).
2. Every receive turn shows a transient 2544 B allocation (heap `(requested, released)` = progress + `(2544, 2544)` per turn) in the receiving tests (complete, cancellation, refusals, cumulative, boxed_custody): the exactness law `heap == progress` no longer holds; likewise the presence json "no leak 2544".
3. Close under a forwarded allocation port: `forwarded retirement allocation requires actual receipt admission` (authored_schema_and_exact_semantic_row_admission, all_owned_phases_cancel_and_wrong_dialect_refuses). This is the forwarded-binding gap noted for `close_retirement_recipient` (charge refuses without receipt admission).
4. Forwarded encode `allocate_*` outside a quoted effect: `forwarded native storage requires its quoted allocation effect` (independent_sql_edit, independent_surrogate_renumber).
5. preflight: `snapshot retirement exceeds original physical grant` while closing the decoded specimen (`close_specimen` grant derived from `owner.remaining_grant()`).

No product code was changed by this slice.
