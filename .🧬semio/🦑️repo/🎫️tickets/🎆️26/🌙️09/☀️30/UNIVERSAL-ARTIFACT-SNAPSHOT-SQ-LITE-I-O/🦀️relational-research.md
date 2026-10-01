# Rust Relational SQLite Engine Design

The user's clarified scope requires handwritten artifact entity tables with ordinary SQLite columns and foreign keys. Native pack blobs and serialized JSON/property trees are excluded. The prior opaque-file approach is superseded rather than kept as a compatibility API.

The new physical engine owns no artifact model. Each provider supplies `SqliteDatabase`, explicit `SqliteTable` SQL definitions, signed `SqliteRow` identities and `SqliteValue` scalars. Existing `SnapshotEncoding` selects native provider decoding before/after typed projections; it is not an opaque storage encoding.

The shared relational JSON schema and building/room corpus precede implementation. Native tests were rewritten for that contract first. The importer/exporter implement the authoritative [SQLite file format](https://sqlite.org/fileformat.html) with the Rust standard library and share application ID/version with TypeScript.

Roots are reserved for the schema and each table. Overflow, leaves and interiors are allocated as needed, with SQLite's local-payload formula and signed varint keys. A schema root with its 100-byte database header may require an interior root pointing to a single child when one schema row fits a normal leaf but cannot fit page one. Multilevel branches use each left subtree's maximum rowid as the separator.

The SQL lexer handles comments, strings, quoted identifiers, nested CHECK/REFERENCES groups and statement boundaries. INTEGER PRIMARY KEY aliases, including single-column table constraints, expose actual integers while their physical record slots are NULL. Integer storage in columns with REAL affinity is restored as a floating-point semantic value. Features requiring index B-trees or generated layouts are explicitly rejected: non-INTEGER/composite primary keys, UNIQUE, AUTOINCREMENT, DESC, WITHOUT ROWID, generated columns and non-table schema objects.

Limits match TypeScript and count semantic data across the database, including eight bytes for restored primary-key aliases, eight for integers/reals, zero for NULL and actual UTF-8/BLOB lengths. Schema limits count SQL plus table-name bytes. Page and row counts are bounded globally; columns are bounded per table.

Mandatory `SqliteSnapshotControl` checkpoints cover provider projection, reconstruction, native decode/encode and physical read/write phases. The physical API accepts the same callback type. Repeated boundaries during preparation/validation remain cancellable without pretending more pages have completed.

Independent validation uses Bun's existing SQLite binding through both the coordinator's cross-language harness and native tests. The std-only executable accepts schema SQL plus tagged tab-separated scalar rows, avoiding a JSON runtime dependency in the engine or executable. serde_json remains a dev dependency for the language-neutral fixture only.
