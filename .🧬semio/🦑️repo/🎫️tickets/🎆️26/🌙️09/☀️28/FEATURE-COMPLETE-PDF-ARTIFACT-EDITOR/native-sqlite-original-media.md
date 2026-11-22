# Native SQLite Original Media Replay

The existing artifact-engine law exercises an in-memory backend. The accompanying native law uses the repository's DB engine and SQLite backend dependencies already selected by its Cargo target. It submits an authenticated original-media group with Fsync, checks duplicate idempotency, shuts down the database and pool, constructs a fresh SQLite owner from the same file, and remounts the document. It checks the recovered frontier, grouped catch-up from inside the transaction, and byte-for-byte relay frames. No Cargo manifest or lockfile changes were needed.

The repository source closure measured 20 files, 3,371 active lines and 208,930 bytes for each native platform setting. The external Python SQLite oracle closed and reopened an on-disk WAL-mode database. It recovered one edit and three rows with SHA-256 fa01587e6b81d65ca1aa4b1856f4971634ede737df9942cdb69f2096995a39b4. The governed Rust test execution is still pending.

- [Shared authored-media fixture oracle](📜️native-hub-durable-oracle.py)
