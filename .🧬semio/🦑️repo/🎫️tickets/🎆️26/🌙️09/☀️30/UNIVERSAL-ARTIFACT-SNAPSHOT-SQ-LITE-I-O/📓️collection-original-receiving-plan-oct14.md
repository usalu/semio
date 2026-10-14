# Collection Original Receiving Plan — October 14

Source-only independent plan; no implementation or runtime acceptance claimed. Authority is Collection snapshot SQL/native source and existing literal fixture, shared space SQL field helpers, and common receiving frame owner.

## Exact persisted structure

| Table | Width | Columns in exact order |
| --- | --- | --- |
| collection_document | 3 | id, schema, name |
| collection_folder | 6 | id, document_id, ordinal, folder_id, parent_folder_id nullable, name |
| collection_entry | 8 | id, document_id, ordinal, entry_id, folder_id nullable, name, kind_id, body_kind |
| collection_document_body | 3 | id, schema, addressed_document_id |
| collection_blob_body | 5 | id, hash, size_high, size_low, media_type |

Native CollectionSnapshot is schema:String/name:String/folders:Vec<CollectionFolder>/entries:Vec<CollectionEntry>. Folder is id:String/parent_id:Option<String>/name:String. Entry is id:String/folder_id:Option<String>/name:String/kind_id:String/body:Box<ArtifactBody>. ArtifactBody is Document{schema:String,document_id:String} or Blob{blob:BlobRef{hash:String,size:u64,media_type:String}}. The blob serde fixture calls media_type `mediaType`.

SQL row identity is structural i64, separate from native string identity. Current cold reader checks alias id==rowid, duplicates per table, exactly one document, exact parent document identity, and contiguous ordinal 0..len-1. It imposes no positive-rowid restriction: zero, negative, i64::MIN and i64::MAX identities should be accepted when every SQL structural foreign link is updated consistently. Ordinals remain nonnegative, contiguous, usize-convertible. Body id equals its entry id; exactly one correctly tagged body must belong to each entry.

Native ids, names, schema/hash/media strings are literal, including empty, NUL, duplicate ids, unresolved links, cycles and mismatched document address. The existing fixture deliberately contains duplicate `cycle` and `id`, a self-cycle, missing parent, unresolved folder, empty ids, and addressed document id different from entry id. Do not add domain reconciliation or positivity validation during snapshot read. Folder parent and entry folder are native text references, not SQL foreign keys. Preserve null vs Some(empty).

Blob high/low each strictly Integer in 0..4294967295. Reconstruct `(u64::from(high)<<32)|u64::from(low)`; projection uses upper32/lower32 without float conversion. Test sizes 0, 4294967295, 4294967296, 9007199254740993, u64::MAX. Neutral fixture stores u64 decimal strings; independent JS uses BigInt(high)*4294967296n+BigInt(low).

## Mutation and list taxonomy

CollectionMutation variants are RenameCollection, CreateFolder(index:u32), DeleteFolder, MoveToCollection(optional parent), RenameFolder, CreateEntry(index:u32), DeleteEntry, MoveToFolder(optional folder), RenameEntry, ReplaceEntryBody(boxed variant). CollectionDiff contains optional renamed_collection/folders/entries. Folder/entry deltas are positional removals/insertions/relocations/modifications keyed by string id, values_only. FolderPatch has optional present-link parent and name; EntryPatch has optional present-link folder/name/kind_id/boxed body. CollectionOptionalLink.value preserves move-to-root vs absent patch. Snapshot SQL scope does not include mutation/diff tables.

## Paid retained frame design

Use one typed static Frame implementing NativeSqliteReceivingFrame and RetireOwned: Projection, partial CollectionSnapshot, paid Vec<usize> identity/ordinal indexes for all five tables, typed pending boxed body slot if needed, and Option<Result<(),ValueError>> status. Keep only integer indexes, no borrowed references inside erased retained frame. Reuse common receiving Port, validate_schema_sql, check_database, paid index sort/lookup, text destination and vector destination.

Projection uses borrowed source fields into original paid SQL Projection: reserve five tables and exact per-table row census, widths3/6/8/3/5, output body_kind text, split u64, then publish database by taking retained slot. Existing RowWriter to_sqlite_database can remain the cold semantic reference; canonical receiving overrides must run this new frame.

Reconstruction first pays/validates declaration/indexes, root census and all aliases; orders folders/entries by ordinal. Install each empty typed folder/entry into reserved output vector before any string copy. For nullable links install Some(String::new()) before copying. For boxed body, admit native Box backing plus physical wallet currencies before Box::new of an empty selected variant, then install original Box in its entry BEFORE fallible body text copies. A temporary locally built body containing populated Strings defeats custody. Current common Port lacks a Box destination helper, so implement the smallest neutral admitted boxed-slot primitive with actual allocation and exact receipt; do not fabricate Box charge without matching actual heap.

Read must reject unused/orphan/multiple body rows, variant mismatch and foreign structural links while preserving complete partial frame. Validators use identical read(frame,Some(original),port) then full literal equality inside original status, without uncontrolled local candidate/drop. Four canonical hooks use common receive_sqlite_snapshot. Saved original status remains in frame across successful-body cancellation checkpoint.

## Neutral cases and edit corpus

Cases: existing complete mixed fixture (10 rows), empty snapshot (1), document-only, blob-only, optional-links absent vs present-empty, long Unicode/NUL in every text field, all five u64 boundaries, and signed structural identity remapping with shuffled physical row order. Keep duplicate native ids/cycles/unresolved references in complete case. Runtime laws cover four hooks, five physical currencies, both original callbacks, seeded prior32-byte native/SQL baseline, registered Binary/Text full SQLite files, finite cumulative close and prepublication modes3/4.

Valid-different semantic edits: document schema/name (2); folder id/parent/name (3); entry id/folder/name/kind (4); document body schema/document_id (2); blob hash/high/low/media (4): 15 column edits, plus nullable link null→text and text→null, blob size boundary edits, legal body variant replacement with matching body table rows, and legal ordinal permutation changing list order. body_kind alone is invalid; paired variant/table replacement is valid-different. Structural identity remap with references preserved is valid-same and must pass validators.

Invalid probes per all five tables: duplicate rowid, id alias mismatch, truncated/extra row width, missing/extra table, wrong declaration and table-name duplicate. Folder/entry: wrong document FK, ordinal negative/gap/duplicate/out-of-range, malformed id/parent/ordinal scalar type. Root: zero/multiple document rows and alias mismatch. Body: missing body, extra orphan body, both body variants for same entry, body_kind invalid/mismatch, orphan body FK, high/low -1 and4294967296, Null/Real/Blob/Text in integer cells, non-text required text and non-null/non-text optional links. Native text references that are unresolved remain legal.

## Independent SQLite witness

Use Bun Database deserialization and PRAGMA integrity_check/foreign_key_check for valid cases. Join folder/entry to document on document_id; order by ordinal, not rowid. LEFT JOIN each entry to document_body and blob_body by id; assert exactly one join matches tag, compare all returned columns, and reconstruct u64 with BigInt. Explicit anti-joins assert no body lacks an entry. Foreign-key-off/check-constraints-off SQL edits produce invalid physical witnesses without changing Rust source fixture. Preserve raw NUL text via bound SQL parameters when practical. Signed remap updates root, document_id columns, entry ids and matching body ids; separate negative and zero rounds, retaining contiguous ordinals. These joins must inspect every column, not only body tag or row count.

## Common body error precedence

Lower `⏱️control/🛫️snapshot/🦀️.rs` receive matches operation Err directly and retains frame without later checkpoint; only Ok performs retirement/publication checkpoints. Thus an already completed body Err is primary unless allocation settlement itself fails. Current unconditional post-finish_body checkpoint can replace saved body Err with newly canceled callback. Limit new publication checkpoint to saved Some(Ok(())) while keeping status untaken; then take Err unchanged. Add original literal/body-owned cause pointer witness with callback armed after error and assert no publication, exact cause, retained status custody and paid close. This follows concrete lower-owner source behavior; no documented broader precedence contract was found.
