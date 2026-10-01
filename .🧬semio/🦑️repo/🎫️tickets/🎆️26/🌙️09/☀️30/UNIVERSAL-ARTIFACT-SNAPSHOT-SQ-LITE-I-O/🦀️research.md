# Rust SQLite Snapshot Codec Research

The Rust codec implements the authoritative [SQLite database file format](https://sqlite.org/fileformat.html) with the standard library. The schema contract lives in framework I/O's `🪶️sqlite-snapshot/🧬️schema/🗄️.sql`.

The exporter uses a 4096-byte schema root page, one table leaf and overflow pages. The importer discovers the snapshot table root from SQLite's schema table, traverses table interior/leaf pages, and computes local payload lengths from the usable page size. SQLite's 512–65536-byte page sizes, nine-byte varints, big-endian signed serial integers and blob serial fields are parsed directly. Native payload bytes remain unchanged.

Every visited page provides progress and cancellation. File, page, coordinate and payload limits are checked before untrusted sizes allocate memory. Global page ownership catches cycles and duplicate references, and bounded cell/freeblock ranges catch overlapping or truncated structures. SQL schema validation protects the fixed field order and declared types. Independent SQLite oracle validation is coordinated by the main worker.

Tests are written before codec implementation. The shared JSON corpus exercises empty payloads, UTF-8 and embedded NUL, every byte value, serial/varint boundaries, and overflow payloads through one MiB.
