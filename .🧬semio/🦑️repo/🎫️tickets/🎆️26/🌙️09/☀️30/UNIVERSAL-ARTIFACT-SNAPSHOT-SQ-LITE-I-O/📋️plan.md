# Universal Artifact Snapshot SQLite I/O

## Objective

Every artifact snapshot supports lossless SQLite-file export and import through framework I/O, including future artifact kinds, without runtime SQLite-library dependencies. The updated user objective requires handcrafted relational schemas for every snapshot, with meaningful entities, attributes and relationships understandable through ordinary SQLite queries. Serialized snapshot packs, opaque JSON payloads and reflection-generated generic value trees do not satisfy this requirement. Snapshot transfer does not imply exporting history or a recursive document archive.

## Updated Semantic Scope

The user expanded the objective on 2026-10-01 after the initial byte-preserving container implementation. That container is insufficient and must be replaced, without a legacy compatibility path. Its physical page parsing, independent SQLite oracle, progress controls and atomic exact-dialect I/O registration can be reused.

The physical database engine exposes owned typed relational cells (NULL, INTEGER, REAL, TEXT and intrinsic binary data), rows with signed 64-bit row identifiers, and named tables with explicit SQL definitions. Each snapshot declares its own SQL schema and typed relational export/import implementation. Domain tables use meaningful names and foreign keys; a small explicit metadata table identifies artifact, standard, subset and schema version. A required provider contract makes missing snapshot implementations visible rather than silently falling back to opaque serialization.

Inventory covers all stdio and nonstdio snapshot types, including headless registrations and guest-loaded artifacts. Executor ownership is split between the Rust and TypeScript physical engines, framework provider/registration integration, and successive artifact families after the read-only inventory establishes exact coverage. Every family needs language-neutral examples, independent SQLite queries and native snapshot reconstruction tests.

## Superseded Initial Contract

SQLite 3 database, application_id `0x534D534E` (1397576526), user_version `1`, UTF-8, with exactly one snapshot record:

```sql
CREATE TABLE artifact_snapshot (dialect TEXT NOT NULL, encoding TEXT NOT NULL, snapshot BLOB NOT NULL)
```

The initial row held exact native bytes. This is retained here as work history, not as the final target or an accepted compatibility format. The updated semantic contract supersedes it.

Implement independent TypeScript and Rust codecs under framework I/O `🪶️sqlite-snapshot`, test a shared language-neutral JSON corpus, and validate both directions with independent SQLite engines. Support progress, cancellation and configurable limits at page boundaries. Universal I/O edges must not create false Exact conversions between unrelated artifact dialects.

## Coordination

Requested main-chat settings are GPT 6.1 Sol Extra High; available fleet has three worker slots. Initial read-only exploration used GPT 6.1 Sol Low (the exposed effort corresponding to the requested Light). Execution uses GPT 6.1 Sol High. Rotate freed slots into independent read-only audits. Do not alter unrelated concurrent edits, modify AGENTS.md, use modifying Git commands or create worktrees.

After DWG/OBJ completion, two attempts to create a new Low audit agent were rejected by the collaboration tool with `agent thread limit reached`, including after its final answer arrived. The existing High executor slot was reused for the newly assigned STL/OBJ/PLY fidelity repairs to retain the maximum working fleet. Earlier multiple Low explorations/audits remain retained ticket evidence. Current execution owners: rootJPEG/GIF/archives/taskregistration, nativeworkerTIFF after Semio/markup, integrationworkerPDF/Note/DXF/guest, geometryworkerSTL/OBJ/PLY after DWG. No rejected spawn was counted as an active audit.

Rust codec worker owns its codec/package/unit tests. TypeScript codec worker owns its mirror/unit tests. Integration worker owns I/O registry and plugin assembly coverage/tests. Coordinator owns the contract, corpus, task/launch registration, combined interoperability checks and ticket lifecycle.

## Source

SQLite's authoritative [database file format](https://sqlite.org/fileformat.html) defines the header, table B-tree cells, serial types and overflow storage used by both implementations. Its main database file is self-contained only after transactions/checkpointing; this contract emits rollback-mode standalone files.

## Validation Plan

1. Shared corpus: empty binary/text, Unicode and embedded NUL, arbitrary bytes, field-size/varint boundaries, multi-page payloads.
2. Independent SQLite reads each exporter and creates databases consumed by each importer, including altered page sizes/root locations and overflow.
3. Malformed identity/version, schema and row types/counts, truncated/cyclic pages and resource/cancellation controls.
4. Registry/assembly laws prove all native dialects receive discoverable Exact routes and cross-artifact misuse fails.
5. Run registered Bun/Nx targets and record exact output and limitations here before ticket closure.


## Next Root Owner: CommonMark

The inspected Markdown owner has a complete seven-variant block and nine-variant inline model, including recursive lists/quotes/emphasis/strong/link text. Its handwritten SQL will contain individual subtype entities and explicit ordered document/block/list-item/inline ownership relationships. The native and TypeScript facets must preserve every owned state (fullu8/u32 fields, absent/empty optional strings, empty collections and custom schema), without rendering/reparsing Markdown. Iterative entity traversal must permit deep trees within database limits, and independent SQLite queries/edits plus neutral fixtures must establish semantic interpretation. Root owns this family after the cancellation lanes; workers retain TIFF, geometry and Note/PDF/guest scope.
