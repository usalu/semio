# Rust SQLite Snapshot Codec Result

Superseded by the requested relational artifact scope. The opaque snapshot API and tests described below were removed. Current implementation and verification are recorded in `🦀️relational-report.md`; this report remains as historical ticket evidence and is not a completion claim for the relational task.

Implemented the native Rust codec with the standard library only. The independent package has no runtime dependencies; serde_json is used only by tests to consume the shared corpus.

## Files Created

- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs`: codec, transfer types, limits, progress/cancellation and errors.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/Cargo.toml`: independent library and interoperability executable targets.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/🦀️.rs`: public package glue.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔬️unit/🦀️.rs`: twelve native tests, including the shared positive/negative corpus.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🧭️oracle/🦀️.rs`: std-only SQLite interoperability executable.
- Ticket `🦀️research.md` and this report.

## Public Contract

`ArtifactSnapshot` owns a dialect coordinate, `SnapshotEncoding::{Binary,Text}` and exact native bytes. `export_sqlite_snapshot` returns a complete standalone SQLite file. `import_sqlite_snapshot` reads that file and optionally enforces a requested dialect. Both take `SqliteSnapshotLimits` and a callback receiving `SqliteSnapshotProgress`; returning false cancels.

Defaults match TypeScript: file 272 MiB, native snapshot 256 MiB, dialect 65536 bytes, and one million physical pages. The coordinate uses the same first-at/last-slash component split as I/O's existing vocabulary and rejects empty components and control characters. UTF-8 text bytes, including BOM, Unicode and NUL, are retained exactly.

Export streams record segments directly to 4096-byte database/overflow pages rather than allocating an intermediate copy of the complete native payload. Allocation limits are checked first. Import reads schema roots, interior/leaf table B-trees, signed rowid/separator bounds, all SQLite page sizes from 512 through 65536, overflow chains, record serial fields and nine-byte varints. Global page ownership, cell/freeblock ranges, untracked free-space and fragment-byte accounting reject malformed structures. The fixed SQL schema is lexed by tokens with supported identifier quoting, whitespace and ASCII case differences.

Header page count follows SQLite's authoritative rule: it is used only when nonzero and the change/version-valid counters match; otherwise the physical file provides the page count. WAL-format files are rejected by this standalone snapshot contract. Identity/version, UTF-8, exactly one row, BLOB storage, schema and dialect mismatches are validated.

Every visited/written page checks cancellation. Native UTF-8 validation also checks cancellation between bounded chunks while preserving codepoints split across chunk boundaries; during validation the callback may repeat the current page boundary.

## Validation Executed

Final registered command: `bun nx run @semio-tech/framework-rs:test-snapshot-sqlite-native`, with `SEMIO_TEST_ARTIFACT_DIR` directed to this ticket's `🗑️generated` folder.

Final run: twelve tests passed, zero skipped; the interoperability executable compiled successfully without warnings. Nextest run ID: `9ba58004-39b1-43c3-99ab-38bf475636ff`. The final source edit afterward only clarified the progress docstring.

The test suite covers the shared fourteen-case language-neutral payload corpus; Unicode/NUL and mismatch rejection; malformed identity/version/truncation/cycles; limits and cancellation; schema-token boundaries; full-width varints and signed record integers; malicious declared allocation sizes and excessive field counts; the SQL source contract; shared malformed coordinates; independent-audit fragment/gap regressions; authoritative header-size regressions; and chunked UTF-8/cancellation.

One preceding run passed all eleven tests but its executable build hit the 15-second task budget while waiting for another worker's Cargo lock. The final twelve-test run completed the build in 0.66 seconds and supersedes that transient failure.

The coordinator owns independent SQLite and cross-language harness execution. Its earlier combined run reported success for fifteen interoperability tests, including independently created roots/page sizes and one-MiB overflow; the final combined rerun will validate subsequent audit fixes and byte parity. No broad final combined result is claimed here.

## Audit Repairs

Resolved Rust findings from `📓️codec-audit.md`: fragmented-byte/untracked-gap validation and SQLite header page-count authority. Also added preallocation guards, schema field validation and cancellable UTF-8 scanning. The coordinator owns shared negative mutation metadata and combined oracle coverage.

No Git mutation, worktree, AGENTS.md modification or external dependency installation was performed. Root registration, launch entries, I/O registry integration and ticket closure remain coordinator/integration ownership.
