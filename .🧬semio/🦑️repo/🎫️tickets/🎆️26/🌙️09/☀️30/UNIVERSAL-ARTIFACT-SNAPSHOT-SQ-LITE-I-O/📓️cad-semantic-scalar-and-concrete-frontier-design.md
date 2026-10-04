# CAD Semantic Scalar And Concrete Frontier Design

## Numerical Hold And Existing Scalar Authority

This is a prepared design, not a mounted native or Source numerical repair. The original Native11 plus three staged System observer laws remain Root's baseline selection. The actual Source exact-scalar law still reaches the count*512 refusal after independently counting the six resulting SQL tables. Original laws are retained.

Current shared artifactSqliteTables already traverses every actual cell with artifactSqliteValueByteLengthControlled, checking cumulative scalar bytes and publishing every 256 rows plus long Unicode interior traversal. It counts null as zero, numeric identities as eight, complete UTF-8 TEXT bytes, and BLOB byte lengths. It then validates the authored schema through the same captured SqliteOperation. The CAD count*512 check duplicates that semantic authority with a physical estimate. The prepared Source fix is solely to remove this estimate; it must not replace it with an index/Map/Set VM heap guess. No second scalar counter or copied authority is necessary.

Provider SHA-256: `27dbcf4e94f37c0ee371edc827100b152623b3a5725c38a1b9cc80dabb0c8cdb`. Prepared scalar diff, held until the required owning baseline:

```diff
--- ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts
+++ ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts
@@ -43,7 +43,7 @@
 /** 📥️ Reconstructs complete persisted ownership after schema, workspace, parent and ordinal admission. */
 export async function cadSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<CadSqliteSnapshot>{
  options=sqliteOperation(options);
- const tables=await artifactSqliteTables(database,sql,options),count=tables.reduce((sum,rows)=>sum+rows.length,0);artifactSqliteValueBudget(count*512,options);const document=artifactSqliteDocument(tables[0]!);if(document.values.length!==3)throw new ValueError("invalidValue","CAD requires one complete document");
+ const tables=await artifactSqliteTables(database,sql,options),count=tables.reduce((sum,rows)=>sum+rows.length,0);const document=artifactSqliteDocument(tables[0]!);if(document.values.length!==3)throw new ValueError("invalidValue","CAD requires one complete document");
  const models=await keyed(tables[1]!,8,options,1n),drawings=await keyed(tables[2]!,8,options,1n),groups=await keyed(tables[3]!,3,options,1n),references=await keyed(tables[4]!,39,options),nodes=await keyed(tables[5]!,6,options,1n);
  const result:CadSqliteSnapshot={schema:artifactSqliteText(document,1),id:artifactSqliteText(document,2),drawings:[],referencesByModelDefinitionId:{},nodes:[]};const slots=new Set<string>();for(const row of models.values()){const slot=artifactSqliteText(row,2);if(!SLOTS.includes(slot as Slot)||slots.has(slot))throw new ValueError("invalidValue","CAD model slots must be declared and unique");slots.add(slot);result[slot as Slot]=readChild(row,3,"model")}
  for(const row of await artifactSqliteOrderedRowsControlled([...drawings.values()],2,options))result.drawings.push(readChild(row,3,"drawing"));

```

## Neutral Reference Index Contract

The existing strictly closed neutral fixture/schema now declare mapOfOrderedReferences, literalUtf8Bytes, duplicateKeyRefusal=invalidValue and aliasRole=localRelationshipOnly. The authored seven-key corpus includes empty, NUL, numeric-looking 10 and 2, __proto__, private-use U+E000 and astral U+1F600. Its independent expected order is explicitly authored; BunSQLite ORDER BY CAST(key AS BLOB) validates it. It distinguishes UTF-8 from UTF-16 ordering and numeric JavaScript property enumeration. Source actual projection/reconstruction and independent surrogate edits must preserve these literals and ordered reference lists. This does not claim that JavaScript object storage admits exact heap bytes or that the unmounted Rust owner is implemented.

The native wire remains its declared map shape. A future controlled native map duplicate law must expose the current BTree last-replacement behavior before changing that ingestion rule; the current SQL duplicate-map-key refusal corpus already exercises the same persisted semantic invariant. No unexecuted duplicate Native law is claimed.

## Actual Domain Authority And Direct Consumers

CadReferenceIndex directly owns Vec<(String,CadReferenceList)> sorted by UTF-8 bytes. It owns no hidden BTreeMap, mirror DTO, serialized carrier or second authority. Both actual persistent fields change together: CadSnapshot and CadArtifact. CadArtifact::to_snapshot clones that domain owner; from_snapshot and set_snapshot move it directly. The derived construction empty owner, root empty_cad_snapshot, forest_references_for_model_definitions return type, and empty inference scene must construct the same type. Sparse CadDiff maps are a separate mutation delta authority; their application iterates changed groups and updates the real index.

Required intrinsic ordinary methods are new/default, len/is_empty, get/get_mut/contains_key, iter/keys/values/values_mut, insert/remove, borrowed Index<&str>, borrowed iteration and owned FromIterator. Ordinary mutation/editor methods keep their intended synchronous role. Controlled methods reserve their real tuple Vec capacity before construction and never grow without admission. Key comparison traverses borrowed UTF-8 bytes with a 65,536-byte interior checkpoint; in-place comparison/swap/shifts publish integer work progress every 256 operations. A real ordered byte comparison is required instead of whole-string Ord inside the controlled path.

FromValue handles the actual DslValue::Object tuple Vec directly; FromValue::from_value_controlled reserves the owned tuple Vec, copies keys and constructs each reference list under the same native control. Missing-field default_value_controlled returns the empty Vec without a phantom heap charge. ToValue constructs the existing actual DslValue object; its controlled counterpart pays the tuple Vec and every real copied key/list under NativeEncodeControl.

DslField reports Shape::Map of CadReferenceList, with controlled boxed metadata from current producer::boxed. Controlled binding walks FieldValue::Map directly into the actual domain owner, without BTree lowering; controlled encoding pays its actual FieldValue::Map tuple Vec and each key/reference list. Current DslRecord derive uses this intrinsic field's real controlled methods. Both controlled derive methods return ValueError; the shared native codec callbacks require positioned TextError only at that explicit diagnostic boundary. Default, duplicate, cancellation and later-field failure must hold partially constructed owners with the current typed guard and retire them.

## Native Relational Frontiers

1. Use validate_sqlite_database_schema_controlled followed by control.check_database for actual schema and complete SQL scalar admission. Keep one caller SqliteSnapshotControl.
2. The allocation_stage closure receives remaining concrete bytes and checkpoint. NativeDecodeControl settles its actual owned_bytes back on success, error and cancellation. Never derive backing admission from max_value_bytes.
3. For each actual table, allocate a real Vec<&SqliteRow> index. Check complete positive row aliases and parents, sort in place by integer identity using bounded heap operations, and refuse duplicates. Fixed four model-slot flags remain an inline array with no physical root charge.
4. Model/drawing child construction copies exactly child_id, target artifact_id and three dialect strings through the native control, retaining local/target independence and exact model/drawing domain. Optional model slots receive their actual child owner directly. For drawings, reuse the admitted borrowed index by in-place ordinal sorting, then check each ordinal against its expected contiguous position; allocate the actual final Vec<CadDrawingChild> once.
5. References use the admitted borrowed row index, sorted by (owning group id, ordinal). Numeric parent lookup binary-searches the admitted group-id index; no BTree/Hash map or uncontrolled collection growth occurs. A paid borrowed group-key index supplies bounded literal key ordering and duplicate rejection. Group slice bounds derive from that sorted relation frontier; each actual final reference Vec is reserved to its slice count and receives complete literals, booleans and ten raw IEEE slots.
6. The actual final CadReferenceIndex tuple Vec is reserved to the real group count. Keys and rows are copied directly into it in validated literal order. Empty groups remain explicit entries. Local surrogate renumbering never changes the domain keys or relationships.
7. Nodes similarly reuse their admitted row index for contiguous ordinal order and construct the final Vec<CadNode> directly. Unknown kind strings remain literal semantic state.
8. Each owned partial field/list/index is guarded until transferred to the final owner. Borrowed scratch Vec owners are actual admitted allocations and can release normally without refunding cumulative admission. No guessed per-node ABI or dummy reserve buffer is valid.

Sorting index structures is O(n log n); numeric parent and slice lookup is O(log n) per group/reference rather than repeated whole relation scans. Long literal key work remains bounded independently of collection length.

## Actual Current Cold Retirement And Encoding Hooks

Current store/♻️retirement implements RetireOwned for ArtifactChild<S>; this was verified in the live checkout. The previous census's missing child retirement gap is superseded by this actual first-party implementation. Reuse it for model/drawing handles. Declare typed cold retirement for CadReference strings, CadNode strings, the direct index tuple Vec, CadSnapshot fields and their partial guards. Completed erased I/O uses retire_sqlite_snapshot. Current typed FromValue/DslField retire_decoded methods close the same actual owner after a later field fails.

Cold retirement constructs cursor/scaffold requests and is not allocation free; observation and qualifications must retain that fact. The existing shared observer excludes semantic comparison and explicit final retirement from construction observation, then verifies that retirement cannot refund admission.

The owner must additionally provide actual borrowed encoding preflight and complete controlled Native input/output/default routes. Their metadata/frontier allocations are concrete and cumulative, and their semantic row/file ceilings remain distinct. No output size claim is inferred from a parser or ordinary formatting pass.

## Actual Prepared Semantic Runtime Receipt

The registered CAD Source/type replay of the strict neutral index facet and real file/surrogate law executes 50 tests, 49 passing, one retained genuine count*512 scalar RED, 115 expectations, Bun 16.24 seconds and Nx 1 minute. The type target passes; only Source fails. Both new semantic index laws pass through independent Ajv strict:true and BunSQLite, preserving all prior Source48 laws. Native14 is unchanged and unexecuted here. Exact log: [strict current replay](🗑️generated/cad-strict-neutral-index-current-readiness.log).
