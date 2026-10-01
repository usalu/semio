# Shared SQLite Import Ownership

The current logical IO owner chose a single owned transfer: the erased codec callback consumes `SqliteDatabase`, and its implementation borrows that owned database for `ArtifactSqliteSnapshot::from_sqlite_database`. Framework and OS mount the same IO source and codec definition. The shared caller now passes the database by value. This lane preserves that coherent design; no compatibility wrapper or caller retention requirement is needed for WGPU parity.

The neutral schema and fixture declare importer ownership, an internal immutable trait borrow, and completion as a native payload or fault. Relational integer/null boundary vectors are checked against first-party serialization and Bun's independent SQLite engine. A source law checks the three matching ownership interfaces and the internal trait borrow. The extra native law requiring reuse of a caller-owned database was removed.

Earlier borrow-oriented red/green receipts remain historical evidence of the compilation mismatch, not evidence for the final ownership contract. In particular `sqlite-import-ownership-green-3.log` predates the logical owner's final owned contract and is superseded. Parent-owned Wasm publication validates the current design; this lane will not change these production signatures again.

The final owned-contract source/SQLite gate ran uncached through Nx: two tests passed, zero failed, 27 assertions (`sqlite-import-ownership-owned-green.log`). The production callback, implementation and caller agree on owned transfer, with the trait borrowing inside reconstruction.
