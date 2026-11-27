# SQLite Snapshot Receiving (semio-framework-artifact-space-collection)

Result: 24 of 29 lib tests pass (was 23). 5 still fail; they are not a single cause and need work outside the collection crate or a design decision. Not all 29 pass.

## Causes found (product, not assertions)

1. Collection receiving used the old `TypedFileSettlement` (control held by `borrow_mut` for the whole body). Native and SQL cancellation events raised during the body were swallowed, and `publish` ran before `finish_body`, so a cancelled body was published ("canceled boxed body cannot publish").
2. The domain-neutral `ReceivingSettlement::with_control` (the intended lend) queued lent events in a `Vec`. That allocation is unpaid heap, and replaying them with `settle` mid-body tripped "SQLite secondary allocation exceeds original native receipt" (registered_full_file regressed while fixing 1).
3. Collection had no own `validate_sqlite_snapshot_subset_{decoding,encoding}`. The default delegates to the legacy workspace path (`from_sqlite_database`), which allocates unpaid scratch (about 197 KB requested and released) and breaks the exact heap-equals-receipts laws (`complete`, `cumulative`, `refusals`).

## Fixes

- `🗂️collection/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs`: `receive!` now uses `ReceivingSettlement` with `with_control`; status and settlement are combined first and `publish` runs only when both are Ok; the cancellation cause becomes `Stage::Settlement` so the SQL cause message wins; published failures are stamped with the owner progress. Added paid, borrowed `check` (equality of the database against the snapshot, own indexes, chunked checkpoints), `coordinate`, and `validate_{decoding,encoding}` (dialect refusal before any allocation, with owner progress).
- `🗂️collection/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`: host overrides route `validate_sqlite_snapshot_subset_{decoding,encoding}` to the receiving module.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🫴️receiving/🦀️.rs`: `ReceivingSettlement` lent events are coalesced (first and last, no allocation) and replayed as body events (progress and cancellation only, no settle) while the body is open.

## Results

Passing now: `complete`, `registered_full_file`, plus the 22 that passed before. Failing:

| Test | Current failure | Diagnosis |
| --- | --- | --- |
| `cumulative` | "native decode has no available explicit retirement slot" on the second validate with one owner | Kernel `receive` always leaves its frame as retirement owner until closed (`with_retirement_owner`). The law requires two validations on one owner and no owner after drop. Needs an in-turn release of an empty, published frame in `io-control` `snapshot::receive` (as `release_empty_pending` does for children), or a validation path without a frame. Product design decision. |
| `boxed_custody` | `publish` reached for a cancelled body in one `boxed_receiving` mode | A cancel raised by the test's native observer is not surfaced by `ReceivingSettlement` (it replaces that observer inside `scoped_native`). Not diagnosed further. |
| `cancellation` | the operation returns Ok where Canceled is expected (sql cut, long-nul) | Same family as `boxed_custody`: coalesced replay loses the cut event for this case. |
| `refusals` | close turn fails "original construction grant no longer admits the posted progress" | The validation frame is the heavy receiving `Frame`; a lean frame holding only `ids`/`ordered` is the likely fix. Not verified. |
| `public_borrowed_native_preflight_admits_actual_owner` | "snapshot retirement exceeds original physical grant" | Not the receiver. `encode_sqlite_snapshot_record_native` on the long record needs more than the fixture `callerGrant` (copy 65536) in `retire_inputs`. Grant sizing vs the dsl record retirement demand. |

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🫴️receiving/🦀️.rs`

No `[DEBUG]` left; generated logs removed. No new test added: the six existing laws already cover each case. Another agent's transient compile error in `semio-os-store-test-support` (`dispatch_test_command` lifetime) briefly blocked one run; it cleared on retry.
