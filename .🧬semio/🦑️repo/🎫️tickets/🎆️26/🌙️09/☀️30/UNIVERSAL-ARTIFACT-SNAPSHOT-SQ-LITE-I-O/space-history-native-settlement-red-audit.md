# Space History Native Settlement RED Audit

Read-only source trace, 2026-10-13. Parent supplied actual Native36879 runtime RED; no new execution here.

Actual log `🗑️generated/typed-snapshot-file-native-canonical-fixtures.log:15452` records `InvariantViolated: SQLite secondary allocation exceeds original native receipt`, cumulative body progress 9207 items, 53698 copied bytes, 67393 retained capacity bytes and zero release. Parent reports successful export and failure at import line 151.

## Proven Missing Original Allocation Loan

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:193`, `validate_sqlite_snapshot_subset`, calls `Self::from_sqlite_database(database, control)` to construct a comparison candidate, then compares it to self. This is an actual second owned reconstruction, not a borrowed check.

Its `from_sqlite_database` at line 115 calls `fields::reconstruct`, charges the snapshot header, allocates keyed/ordered/grouped indexes, vectors, authors, members and strings. This candidate therefore advances the SQL allocation ledger.

The actual shared receiving defaults in `🏪️store/📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs` differ:

- `project` and `reconstruct` invoke `receiving_allocation!`: `scoped_native` -> `native.with_encoding_receiver` -> `control.with_original_allocation`, so SQL allocation admissions advance original native owned bytes.
- `validate_decoding` and `validate_encoding` only perform original native checkpoint, then call the cold `snapshot.validate_sqlite_snapshot_subset(dialect,database,control)`. They do not install an original allocation loan.

Thus the real comparison reconstruction performs SQL allocation admissions without the corresponding native allocation callback whenever its caller control lacks an already-installed native port. The typed file wrapper supplies native ownership through explicit Port operations and directional producer loans; it does not install a whole-operation SQL native allocation port around the validation call.

## Ledger Consequence

`SqliteSnapshotControl::admit_allocation_bytes` (`🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs:216`) always advances SQL ledger and forwarded counter, but only calls original native admission when `allocation: Some(port)`. Shared `with_original_allocation` installs that port for the producer child and resets its local forwarded counter to zero; this matches `scoped_encoding_receiver`'s start-relative request ledger. No double charge was proven in that original loan path.

`artifact::receiving::settle` correctly computes SQL admitted since baseline and requires it not exceed original native cumulative delta. The missing validation loan yields exactly the forbidden relation. Successful export does not disprove this path: unmatched native header/close admissions can mask unforwarded SQL work in one operation shape. Numeric attribution of the entire deficit still needs the parent's scoped counters; the source establishes an actual missing charge, not its final measured byte amount.

## Correct Frontier

Do not weaken `settle`, clamp the SQL delta, increase native owned bytes after the fact, or rebase away candidate work. Forward actual validation allocations through the same original native encoding receiver, as P/R already do. Capture and record body grant receipts as required by the owner contract. Also retain candidate/index/scratch custody under the original recipient: the candidate currently drops locally after comparison, so allocation forwarding alone does not establish controlled retirement. Prefer a paid borrowed validation algorithm or an explicit retained candidate validation frame for this actual domain. Both directions call this shared default and require the same correction.

Keep exact SQL/native cancellation cause capture and residual settlement on error. A comparison validation error must preserve its allocated candidate/index owners through original cancellation/close rather than dropping them while constructing diagnostics.
