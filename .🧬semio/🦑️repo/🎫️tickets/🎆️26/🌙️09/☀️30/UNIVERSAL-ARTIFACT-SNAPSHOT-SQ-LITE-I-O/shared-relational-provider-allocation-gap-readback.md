# Shared Relational Provider Allocation Gap Readback

Read-only current source inspection. No Cargo, production/test/script changes or new runtime receipt. Root's earlier physical32 success predates the physical rewrite; the two public-transfer RED assertions and authorized physical repair do not prove these separate relational construction paths. No universal admission claim is justified by this audit.

## Confirmed Unadmitted Frontiers

`/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🦀️.rs`

- Projection::new (68–73) calls uncontrolled SqliteDatabase::from_schema and schema validation. Neither receives the caller's shared allocation control. max_schema_bytes is a semantic/schema ceiling, not admission of owned token, declaration, table or index backing.
- insert_key (91–92) allocates its SqliteValue vector with infallible Vec::with_capacity, copies text/blob cells, then pushes a SqliteRow into the table's potentially growing vector. There is no admit_allocation_bytes call before any of these backing frontiers. Null/numeric-only rows therefore still allocate even with zero text bytes; cell and row Vec ownership must be admitted independently of the 8-byte semantic scalar count.
- copy_text/copy_blob (45–56), used in projection and reconstruction, use fallible exact reserve and cancellable chunks but never charge max_allocation_bytes. project_text only checks max_value_bytes; Reconstruction::reserve (231–236) advances reconstruction semantic bytes/units. Neither substitutes for cumulative backing admission. Repeating reconstruct_text with one shared control currently advances semantic reconstruction admission but leaves allocation_remaining_bytes untouched.
- ordered_row_refs (218–222) builds an unadmitted BTreeMap of borrowed rows and collects an owned reference Vec. float_cells (153–158) creates an unadmitted cell Vec, grows it for companions and allocates a BTreeSet to detect duplicate scalar slots. Semantic max_columns prevents excessive columns but admits none of that backing.
- NativeEncodingBound::add (13–22) checks aggregate predicted bytes against max_value_bytes and max_file_bytes only. It creates no backing itself, so blindly charging add would turn estimates into double admission. Actual owner traversal/output allocations still need controlled producer admission at their real frontiers; this bound is not evidence that max_allocation_bytes is enforced.

`/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs`

- from_schema (85–100) owns lexed token Vec/String text, parsed definition fields, table Vec, table names and per-statement SQL Strings. parse_table (247 onward) additionally owns column/group vectors and joined/uppercase type strings. None receives allocation control.
- validate_sqlite_database_schema (70–81) parses a second expected database and allocates lowercased name Strings plus a BTreeSet; table validation parses/lexes further declarations. These transient copies are real cumulative ownership even though they are later retired.
- SqliteTable ordinal ordering (49–52) creates intermediate and returned reference vectors without caller control. It is a separate uncontrolled alternative to ordered_row_refs and must not be used to bypass a repaired controlled provider path.

These are direct source gaps, not inferred failed assertions. Physical page/export/import accounting is separate and was not re-audited here.

## Late Copy Ownership

Projection's cell-vector local drops already copied text/blob if a later chunk checkpoint returns Err; previously committed rows remain in Projection and drop if the projection is abandoned. This relational database ownership is flat and has no recursive native child tree. Failed copies should nevertheless retain their cumulative admitted backing after any future repair; retirement must not refund admission. Reconstruction returns each successful field to its domain owner, so complete native partial-tree safety remains the owner's explicit guard obligation (the mounted HTML OwnedNodeMap/OwnedChildren are one inspected example). Shared Reconstruction alone cannot prove every domain owner's cleanup.

## Minimal Clean Repair Boundary

Extend existing controlled producer paths and allocation_stage wiring instead of introducing a compatibility layer. Schema parsing/validation invoked by Projection must use the caller's remaining construction allowance through the existing producer/control mechanism. Admit String/blob requested backing before reserve; admit exact vector element backing before vector reserve, including each replacement allocation on growth rather than only net live bytes. Use overflow-checked count × size_of for actual vector element types. Allocation failure remains typed AllocationFailed; ceiling refusal remains typed OwnershipLimit; cancellation stays typed Canceled up to the declared terminal.

Opaque standard BTreeMap/BTreeSet allocation layouts are not knowable from public APIs: do not multiply a guessed node size or charge after insert. For these small borrowed ordering/duplicate frontiers, a controlled fallible Vec with in-place ordering and adjacent duplicate checks exposes the real admitted backing and preserves the existing result contract. Any schema/table metadata index must similarly use an actual controllable frontier. Do not count semantic scalar storage as a substitute for row/cell/header vector backing; do not reset control between schema, validation, insertion or reconstruction.

## Minimal Neutral Fixture And Independent Proof

Add one closed language-neutral fixture beside existing shared SQLite fixture/refusal laws with: authored two-table SQL, zero-length text and blob, nonempty Unicode text/blob, numeric-only rows, reordered ordinals, one duplicate ordinal, one IEEE field with exact companions, a 100000-byte text plus blob, small construction ceiling (0/1), generous fixed semantic/file ceilings, and repeated-producer/late-interior-cancellation scenarios. Keep exact host-independent field and SQL expectations in the fixture; avoid embedding Rust Vec/BTree layout bytes in the shared corpus.

Independent Source oracle: Ajv validates the closed fixture; Bun SQLite executes authored SQL, checks storage classes, exact ordered fields, foreign keys/integrity and reopen/serialize behavior; Buffer independently computes UTF-8/blob semantic bytes and the 65536 copy interruption frontier. IEEE oracle validates fixture bits/class with independent DataView. These prove semantic authority, not Rust allocations. The Native test alone verifies actual requested backing under a test allocation observer and typed refusal origin.

Native provider assertions: (1) zero/one allowance refuses Projection::new before its first allocation, with semantic ceilings otherwise generous; (2) exact observed/admitted requested backing succeeds once, one byte less refuses before the next frontier; (3) numeric-only row insertion pays cell/row backing; (4) cumulative repeated projection/reconstruction on one control refuses without resetting, even after retirement; (5) a later text/blob interior cancellation retains positive admission and safely retires partial fields; (6) ordered refs and IEEE companions account actual backing. Derive architecture-specific exact counts from the actual declared controlled element/reservation frontier and independent allocation observation, not guessed BTree ABI. Admission after allocation must fail the test observer condition.

Existing registered routes: `bun nx run @semio-tech/framework-sqlite-snapshot-rs:test-native --skip-nx-cache` covers the shared Rust producer laws; `bun nx run @semio-tech/framework-sqlite-snapshot-rs:test-refusal --skip-nx-cache` already executes strict Source/Ajv/Bun SQLite refusal oracles. Existing TIFF `test-snapshot-sqlite-native` and `test-snapshot-sqlite-source` owner routes can demonstrate one real provider integration after the shared laws. No new script/command is required. No route was executed in this audit.
