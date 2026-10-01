# TypeScript Relational Snapshot Scope

## Superseding Requirement

The user's updated goal requires full semantic SQLite entities and relationships for every snapshot, readable independently of the native artifact codec. A generic `artifact_snapshot(dialect, encoding, snapshot BLOB)` carrier does not satisfy this requirement and must not receive additional feature development. No entire snapshot may be hidden in JSON text or a blob. Each snapshot dialect needs a handcrafted relational schema and a matching typed provider.

## Existing TypeScript Capability

The dependency-free codec's page/header, bounded overflow-chain, signed 64-bit rowid, table B-tree, cell/freeblock/fragment, UTF-8, progress, and cancellation validation remains useful low-level parser work. The private `Field` union already distinguishes null, text, bytes, numeric real and integer values. Its record decoder supports SQLite serial types 0 through 9 and text/blob types, including signed integer widths and float64. It discovers schema roots rather than fixing importer roots at page 2.

The current public API and writer remain specialized to the obsolete carrier. The writer emits exactly two table root pages, one schema row, one snapshot row, and overflow pages. It can write text, blob and a single-byte schema-root integer. It does not yet pack multiple entity rows into leaves, build interior trees, write arbitrary signed 64-bit entity columns or floats, allocate arbitrary table roots, or emit index B-trees. The record decoder also hardcodes schema five-field and carrier three-field rules; it must expose a bounded generic record layer with provider-specific validation above it. The SQL validator accepts only the old fixed three-column CREATE TABLE statement. No general SQL execution engine exists or should be introduced as a runtime dependency.

## Schema-First Provider Boundary

Place each handcrafted SQLite schema beside its existing artifact/standard/subset snapshot schema, and put each TypeScript/Rust provider beside that schema. The framework owns typed SQLite cells, table rows, bounded reader/writer primitives, transfer metadata and cancellation/progress. Domain providers own the semantic tables, column names, constraints and field mappings. The existing artifact parsers remain the final typed snapshot boundary; providers must construct their domain values from relational rows rather than invoking a native-byte codec on a hidden payload.

Use an owned discriminated cell contract so integers (`bigint`) and reals (`number`) remain distinguishable, plus explicit null/text/declared primitive binary fields. A table descriptor should carry ordered column definitions and keys/constraints; row streams should contain typed cells, and reference rows should expose their foreign-key columns. Providers must declare any elementary binary scalar field explicitly; a whole-snapshot JSON/blob field is forbidden. Future dialect registration must require a provider and schema and fail assembly when either is missing. A universal opaque fallback would conceal incomplete coverage.

For example, the current CAD snapshot's `nodes`, `drawings`, model-child roles and `referencesByModelDefinitionId` become distinct entity/relationship tables. Ordered lists need explicit ordinals; map keys become columns; child references expose artifact ID and full dialect coordinate in relationship rows. Missing optional model roles require actual absence/nullable columns according to the handcrafted schema, rather than a JSON dump. A text snapshot can have its semantic text as a declared TEXT column because text is its actual domain value; this must not become the representation for structured artifacts.

## Required Foundation Work

1. Separate the existing validated SQLite header/table-record reader from the carrier schema and expose owned typed rows with configurable table/column/row/record limits.
2. Implement generic record writing for signed 64-bit INTEGER, REAL, TEXT, NULL and declared scalar BLOB values; retain UTF-8 and overflow checks.
3. Pack many records per leaf, split pages, build ordered table interior pages, allocate multiple table/schema roots, and preserve deterministic layouts and standalone-file headers.
4. Implement SQLite index B-tree reading/writing when handcrafted schemas declare TEXT/composite primary keys or UNIQUE constraints. SQLite creates implicit `sqlite_autoindex_*` objects for these constraints; omitting their index roots is not an integrity-valid database. An INTEGER PRIMARY KEY can reuse a table rowid, but that does not remove the need to enforce natural semantic identifiers and relationship integrity.
5. Validate every known table's shape, storage classes, required/optional values, keys, references, ordering and domain invariants before import publication. Reject missing required tables, duplicate identities, dangling references and hidden whole-snapshot payloads.
6. Add per-provider language-neutral semantic fixtures and independent SQLite SELECT/FK/integrity assertions. Import independently inserted semantic rows without access to a native artifact codec. Validate cross-language relational parity and entity-level roundtrips.

The generic foundation should be authored independently in TypeScript and Rust and share schema/fixture contracts. Existing carrier tests remain evidence for the parser foundation until replacement, not completion evidence for the superseding relational goal.

## Public API Checks Completed Before Scope Change

The obsolete carrier functions are accessible through `@semio-tech/framework`, and the consumer probe plus targeted TypeScript diagnostics passed. The existing registered framework typecheck returned only three TS2769 errors in the newly added interoperability test (lines 38, 64, 80), caused by matcher generic disagreement between `Uint8Array<ArrayBuffer>` and `Uint8Array<ArrayBufferLike>`. The coordinator was notified because that file is outside this worker's current ownership. No runtime codec imports/dependencies were introduced.
# Execution Follow-Up

The physical relational engine, public package APIs, schema-only parser/validator and first three CSV/TXT/BINARY semantic facets are now implemented. Current implementation and exact checks are documented in [TypeScript Relational Engine And First Three Semantic Providers](./📓️typescript-providers.md). The previous opaque carrier API has been removed. The framework Nx package typecheck and expanded targeted source check pass; physical/facet tests pass 34 tests and 167 assertions.
