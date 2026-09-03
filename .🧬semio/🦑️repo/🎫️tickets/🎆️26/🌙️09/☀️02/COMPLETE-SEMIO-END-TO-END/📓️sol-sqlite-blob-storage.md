# SQLite BLOB Storage Repair

## Attribution

- Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`
- Goal: `🎯r2603`
- Lane: GPT-5.6 Sol SQLite payload persistence repair
- Owned implementation: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs`
- Deliberately not edited: generic DB output lifecycle and hub code, both concurrently owned by other lanes.

## Defect and repair

SQLite's `||` operator stores its result as `TEXT`, even when both inputs originate as BLOBs. A leading `0x00` therefore made SQLite's text `length()` report zero and made `substr()` expose a text value to the Rust `Vec<u8>` decoder. The defect occurred at all three byte concatenation uses:

1. WAL input staging in `write_stage_step`.
2. Content-addressed payload staging in `payload_stage_step`.
3. The staged-to-final `wal_segment.bytes` append.

The two staging sites now share `STAGE_APPEND_SQL`, and the final WAL append uses `WAL_APPEND_STAGE_SQL`. Both cast the complete concatenation expression back to `BLOB`. No decoding coercion, compatibility layer, migration, or runtime dependency was added.

Input staging still advances at most one `DB_IO_PAGE_BYTES` page per execution step, preserving the existing task-level progress and cancellation opportunity. Each public SQLite WAL append and payload put input is bounded by the existing `MAX_BLOB_BYTES` check; the final SQLite statement remains one atomic synchronous task step.

## Test-first evidence

The raw SQLite oracle was added first and run against the original SQL:

```sh
RUST_MIN_STACK=16777216 CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/sqlite-blob-storage/target" bun nx run '@semio-tech/framework-os-kernel:test' -- --package semio-framework-os-kernel-db --features sqlite 'db_storage_sqlite::sqlite_storage::tests::sqlite_blob_append_preserves_storage_class_length_and_hex' -- --exact --nocapture
```

Red result: `0 passed; 1 failed`. Rusqlite/SQLite reported `("text", 5, "00FF80C328")`, while the oracle required `("blob", 5, "00FF80C328")`.

The same command after the repair passed `1 passed; 0 failed; 584 filtered out`. Its independent SQLite `typeof(bytes)`, `length(bytes)`, and `hex(bytes)` assertions prove:

- staging: `blob`, length `5`, hex `00FF80C328`;
- final WAL after prefix byte: `blob`, length `6`, hex `7F00FF80C328`;
- final payload copied from staging: `blob`, declared length `5`, hex `00FF80C328`.

The public SQLite storage facade regression reuses the neutral JSON page-boundary fixture and adds explicit zero-length and arbitrary binary coverage:

```sh
RUST_MIN_STACK=16777216 CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/sqlite-blob-storage/target" bun nx run '@semio-tech/framework-os-kernel:test' -- --package semio-framework-os-kernel-db --features sqlite 'db_storage_sqlite::sqlite_storage::tests::payload_roundtrip_obeys_neutral_page_boundaries_and_arbitrary_bytes' -- --exact --nocapture
```

Result: `1 passed; 0 failed; 584 filtered out`. It verifies independent hash equality plus `put`, `contains`, `len`, exact `get`, `delete`, and post-delete `contains == false` for lengths `0`, `1`, `4096`, and `4097`, plus a `4097`-byte vector beginning `00 FF 80 C3 28` with another NUL at the page boundary.

The SQLite-owned module group was also run serially to separate global test interference from BLOB behavior:

```sh
RUST_MIN_STACK=16777216 CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/sqlite-blob-storage/target" bun nx run '@semio-tech/framework-os-kernel:test' -- --package semio-framework-os-kernel-db --features sqlite 'db_storage_sqlite::sqlite_storage::tests' -- --nocapture --test-threads=1
```

Result: both new BLOB tests passed. Two older sibling tests failed afterward in the concurrently changed generic lifecycle layer: `typed_lane_is_lossless_at_page_boundary_and_zero` returned `StaleGeneration { expected: 1, actual: 0 }` during close, and `typed_list_and_catalog_cas_are_stable` did not receive its expected fenced CAS error. These failures do not enter the repaired SQL paths.

## Downstream verification and blockers

The existing neutral generic lifecycle fixture was run with the correct non-exact filter:

```sh
RUST_MIN_STACK=16777216 CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/sqlite-blob-storage/target" bun nx run '@semio-tech/framework-os-kernel:test' -- --package semio-framework-os-kernel-db --features sqlite 'sqlite_payload_roundtrip_obeys_the_neutral_page_lifecycle_fixture' -- --nocapture
```

The SQLite `0/1/4096/4097` put/hash/contains/len/get/delete loop completed. The test then failed at generic storage line 7668 on its separate missing-hash `get()` assertion instead of returning `DbError::NotFound`. This is the concurrent output-page retirement blocker; the generic file was not edited here.

The focused hub route was run through its mandated all-feature Nx target:

```sh
RUST_MIN_STACK=16777216 CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/sqlite-blob-storage/target" bun nx run os-hub:test -- 'blob_put_get_head_round_trip' -- --exact --nocapture
```

The all-feature hub and dependency compile completed. The single route test passed PUT, returned identical GET bytes, reported the present blob via HEAD, and reported a missing blob via HEAD. It failed only at the final missing GET: HTTP `500` was returned instead of `404`, matching the generic missing-result lifecycle blocker above. Result: `0 passed; 1 failed; 39 skipped`.

A second dedicated all-feature DB compile was intentionally not started because the generic lifecycle lane was already running it. The focused SQLite tests and the hub all-feature dependency build both compiled this change successfully.

## Hygiene

```sh
git diff --check -- '🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs'
```

Result: clean. The isolated generated build directory was removed from the ticket after validation and moved to `/Users/ueli/.Trash/semio-sqlite-blob-storage-20260903`, so the 3.4 GiB output remains recoverable until Trash is emptied.
