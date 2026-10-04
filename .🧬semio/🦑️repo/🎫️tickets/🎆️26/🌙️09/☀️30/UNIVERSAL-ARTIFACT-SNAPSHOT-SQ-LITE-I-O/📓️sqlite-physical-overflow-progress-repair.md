# SQLite Physical Overflow Progress Repair

## Authentic Failure and Diagnosis

Root's `Nextest 078214ba-815e-4c4a-b35f-babda11aeca7` physical whole32 receipt reports 30 passed and 2 failed. The failing assertions are `aggregate_value_budget_rejects_overflow_before_reading_or_allocating_it` and `schema_budget_rejects_large_schema_overflow_before_reading_it`, in physical `🧪️tests/🔬️unit/🦀️.rs` at the original lines 141 and 163. Receipt: `🗑️generated/root-authentic-shared-sqlite-physical-transfer-whole32-current.log`.

Both tests receive `OwnershipLimit`; the failures concern the maximum observed progress counter. The schema assertion received 33 rather than 1. The newly paid visited-page bitmap reports initialized bitmap bytes as `ReadPages.completed`; for this million-byte fixture its bitmap contains 33 bytes. Schema decoding and lexical work likewise emit their own byte/token counters through `ReadPages`. Therefore these two actual failures prove mixed progress units. They cannot by themselves prove overflow pages were traversed.

Fresh source readback confirms `Reader::table` checks declared record length before `Reader::payload`, which performs the payload reserve and overflow-page traversal. Ordinary records are bounded by remaining semantic budget plus `columns * 9 + 9`; schema records by twice the remaining schema budget plus 67, each capped at the actual file length. The million-byte blob has only 32 semantic bytes available, so its declared size fails that check before its payload reserve. The million-byte schema has only 8 schema bytes available, so its declared size fails against 83 bytes before its payload reserve. The visited bitmap is permitted, bounded, and paid before traversal. This check is a conservative physical size bound; it does not claim exact semantic admission for every malformed or scalar-only record.

## Mounted Repair

The caller's existing `SqliteSnapshotControl` now retains an optional physical read progress boundary. During actual physical import all `ReadPages` callbacks report visited pages and the physical page count. `Reader::page` alone advances that boundary after its page-reference validation. Bitmap initialization, record decoding, payload copying, schema lexing, comparison, sorting and frontier callbacks still execute at their existing bounded interior locations and can cancel. They no longer replace the page counter with unrelated byte/token counts. Other transfer phases retain their stage-local counters.

The import operation saves and restores the prior progress boundary on success, cancellation and ordinary refusal. The allocation ledger remains on the same caller control. No child operation-owned physical control, refund, guessed allocation measurement or unpaid fallback was introduced. All existing backing admission remains intact.

The existing aggregate-value regression additionally reuses one control for the actual controlled import, asserts admitted backing remains charged after the semantic refusal, then calls `checkpoint(ReadPages, 7, 11)` and observes that exact caller progress after the physical scope ends. The suite remains 32 tests. Existing strict overflow/page-boundary assertions remain unchanged.

## Changed Files and Evidence

- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs`: retained physical page progress context, caller checkpoint mapping, and progress documentation.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🦀️.rs`: page-only advancement and scoped physical import mapping/restoration.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔬️unit/🦀️.rs`: extends the existing aggregate regression with actual persistent-control failure cleanup and retained allocation assertions.
- This ticket report.

Rustfmt parser checks exited 0 for all three Rust files. Parser readbacks are `🗑️generated/sqlite-physical-overflow-progress-{root,transfer,tests}-parser.rs`; these establish syntax parsing only. No Cargo command was run in this lane. Root owns fresh whole32 and public2 Native execution.

The existing registered Source command completed uncached through Bun/Nx: `bun nx run @semio-tech/framework-rs:test-snapshot-sqlite-source --skip-nx-cache --args=quick`, with `SEMIO_TEST_LEVEL=quick`, `NX_DAEMON=false`, and `NX_ISOLATE_PLUGINS=false`. Its receipt is `🗑️generated/sqlite-physical-overflow-progress-source-current.log`: 41 passed, 0 failed, 385 assertions, 3 files, 2.21 seconds test runtime, 12.2 seconds Nx duration, exit 0. These TypeScript/independent BunSQLite Source laws are regression evidence; they do not execute the mounted Rust repair. Fresh Native whole32/public2 remains Root-owned and pending.
