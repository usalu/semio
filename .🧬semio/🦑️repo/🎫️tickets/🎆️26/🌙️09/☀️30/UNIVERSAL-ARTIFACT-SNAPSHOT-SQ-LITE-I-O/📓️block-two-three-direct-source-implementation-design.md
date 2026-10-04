# Block2d and Block3d Direct Source Implementation Design

Root supplied genuine missing-provider baselines: Block2d59 selected/1 pass/58 fail; Block3d62/1 pass/61 fail. No execution by this audit. Both root Snapshot TS modules currently lack their own SQLite functions. The strict independent demand must stay selected.

## Direct Facets and Current Reuse

Implement each own `snapshot/🪶️sqlite/🟦️.ts` with exports `block2dSnapshotToSqliteDatabase`, `block2dSnapshotFromSqliteDatabase` and 3d analogues; root Snapshot TS reexports only its own functions. Each owns its own SQL schema constant beside its existing `🗄️.sql`; do not wrap Block5d or infer shape from another artifact. Match the ten/thirteen exact SQL tables, without narrowing unresolved native references or child addresses.

Reusable actual firstparty authority is `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts`: ArtifactSqliteProjection.create14, checkRowsAdditional24, insert42, finish64; artifactSqliteTables204 validates full schema/all tables/all aggregate values/row identities; artifactSqliteOrderedRowsControlled275 provides contiguous ordinal ordering; document/text/integer/boolean helpers235–260. `sqliteOperation` root89 preserves an existing operation; pass this exact object into ALL shared calls, never recreate plain options. Binary64 parse/value API is adjacent `🔢️ieee754/🟦️.ts`.

Block5d provides handwritten row/cursor examples, not a whole caller-ownership proof. Its Reader65 creates a NEW NativeDecodeControl from maxValueBytes, and charges guessed64/384 credits independently of SqliteOperation. Those credits do not settle the caller's cumulative allocation ledger. Do not copy that implementation to satisfy the exact caller tests. SqliteOperation currently exposes allocateBytes/remainingBytes/ownedBytes, but not its private allocation stage; SqliteAllocationControl.stage52 is public on a DIFFERENT owner. No public bridge is inferred or guessed. Source JS object/Array physical storage equality is not available from a Rust ABI number.

## Exact Row Mapping

All rows include surrogate id at index0 matching rowid. Parent document references are index1. Every list table uses ordinal index2; ordinals are contiguous zero-based and independent of literal IDs. Singleton document has one row; all singleton child tables have one row referencing that document. Every input row is consumed exactly once; reject orphan/unconsumed/duplicate ownership and extra/missing cells.

| Block2 table | Remaining columns after id, in order | Owner/cardinality |
| --- | --- | --- |
| document | schema | one |
| kind | document_id,native_id,name,label,variant,description,icon,unit | nodeKind one; three optional TEXT |
| presentation | document_id,shape,radius triplet,width triplet,height triplet,color,icon_kind | one; each optional word is ALL NULL or full triplet |
| handle_kind | document_id,ordinal,native_id,name,label,color,default_wire_kind | handleKinds ordered |
| handle | document_id,ordinal,native_id,handle_kind,angle triplet,radius triplet | handles ordered; unresolved handleKind retained |
| compatibility | document_id,ordinal,native_id,source,target,bidirectional | ordered; bool exactly0/1 |
| attribute | document_id,ordinal,key,value,definition | ordered optional definition |
| author | document_id,ordinal,native_id,name,email | ordered optional email |
| camera2d | document_id,x triplet,y triplet,zoom triplet | one |
| meta | document_id,description | one |

2d exact row forecast is 5 + handleKinds.length + handles.length + compatibility.length + attributes.length + authors.length. It includes document/kind/presentation/camera/meta, not a guessed constant from5d.

| Block3 table | Remaining columns after id, in order | Owner/cardinality |
| --- | --- | --- |
| document | schema | one |
| kind | document_id,native_id,name,label,variant,description,icon,unit | objectKind one |
| catalog_child | document_id,child_id,target_artifact_id,artifact_kind,standard,subset | one; local and target IDs distinct and may empty |
| representation | document_id,ordinal,native_id,name,mesh_url,lod,description | ordered |
| representation_tag | representation_id,ordinal,value | each representation's ordered tags; duplicates retained |
| representation_attribute | representation_id,ordinal,key,value,definition | each representation's own ordered attributes |
| vortex_kind_extra | document_id,ordinal,native_id,name,label,color,default_cable_kind | ordered |
| vortex | document_id,ordinal,native_id,vortex_kind,position_x/y/z triplets,direction_x/y/z triplets,radius triplet,label | ordered unresolved kind + optional label |
| compatibility | document_id,ordinal,native_id,source,target,bidirectional | ordered |
| attribute | document_id,ordinal,key,value,definition | parent ordered |
| author | document_id,ordinal,native_id,name,email | ordered |
| camera3d | document_id,position_x/y/z triplets,target_x/y/z triplets,zoom triplet | one |
| meta | document_id,description | one |

3d exact forecast is 5 + representations.length + sum(tags.length+representation.attributes.length) + vortexKindExtra.length + vortices.length + compatibility.length + attributes.length + authors.length. Five singletons are document/kind/catalog/camera/meta. Representation children reference the representation rowid, never the document or native representation ID.

Every float triplet is query REAL/INTEGER/NaN NULL, signed64 raw UInt bits, class finite/nan/positiveInfinity/negativeInfinity. Raw bit reconstruction uses BigInt.asUintN(64), preserving negativezero and all NaN payloads; reject class/query disagreement. Infinity remains REAL, NaN alone NULL. Strict three-word vectors and exact optional all-null words; do not normalize empty Some strings to null.

## Strict Capture, Controls and Remaining Work

Actual shared scalar row() accepts extra keys and parsers discard them. Therefore project MUST reject exact extra keys at every root/nested scope before ordinary parser lowers unknown fields. Include Binary64 {bits}, all kind/presentation/author/attribute/representation/vector/child.target/dialect scopes; exact-key validation must distinguish null/present options. Catalog parser deliberately admits literal address strings without materialization. No narrower membership FK should be added for native unresolved handle/vortex source names.

For mutable caller input, capture complete primitives/ordered arrays before asynchronous schema/row callbacks can change them; keep literal strings (immutable) and copy exact raw bits/each nested entity, not JSON stringify/parse or structuredClone that bypasses admission. Count all admitted work using checked safe integers and cancel every256 entity operations and bounded text scans. Shared Projection charges its actual byte backings; restore tables/order/text traversals must pass the same operation. Current public stage gap above must be resolved explicitly if new concrete capture backing is introduced; do not allocate fake byte arrays merely to impersonate JS object admission.

The existing independent laws require full optional states, strict extra-key refusal, semantic nested SQL edits, exact caller allocation budget, minus-one, cumulative second refusal and materialized interior cancel for projection AND restoration. All58/61 failing operations must remain. A complete production capsule is not supplied because the caller capture/stage authority needs a real reviewed join; the table mapping is ready for handwritten implementation, but this report does not claim that boundary implemented.
