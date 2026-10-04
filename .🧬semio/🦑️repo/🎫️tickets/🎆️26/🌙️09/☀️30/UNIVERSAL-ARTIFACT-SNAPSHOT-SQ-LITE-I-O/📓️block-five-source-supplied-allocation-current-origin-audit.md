# Block5d Supplied Source Allocation Origin Audit

Root reports authentic registered Source412:409 pass/3 fail/2244 expectations. This audit ran no tests. The three failures are missing public allocationStage and zero supplied-operation debit in both semantic directions; they are not failed complete-owner equality assertions.

## Exact Current Origins

Paths below are relative to /Users/ueli/Documents/semio.

* `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts:66–89`: SqliteOperation retains one private allocation control, exposes allocateBytes/remainingBytes/ownedBytes, and sqliteOperation preserves instance identity. There is no public allocationStage. This is not an options-copy identity bug.
* Same file52–55: existing SqliteAllocationControl.stage constructs a child using remainingBytes and commits actual child ownedBytes in finally, on success/refusal/cancellation. Same file41–49 admits exact Uint8Array backing before requesting it. Child settlement does not refund retired backing.
* Same file210,315–328,330–342: controlled grammar means cancellation/work frontiers. parseSchemaWork owns token arrays, statements, names Set, table objects and sliced SQL strings; validateSchemaWork owns expected schema, Map/Set indexes and rowid sets. Neither generator nor grammarControlled calls allocateBytes or child stage. maxSchemaBytes bounds measured UTF8 semantics, not allocation debit. No legitimate JS object/Set/Map heap size follows from that semantic count.
* `.../sqlite-snapshot/🧩️artifact/🟦️.ts:14–21,42–58,173–194`: Projection owns ordinary rows/cell arrays. Only Uint8Array cells invoke supplied operation.allocateBytes at180. Strings/numbers/bigints/null are retained as primitive cells. Block5d SQL projection uses only those primitives; its binary64 words are signed bigint cells, not Blob copies. Thus its complete successful projection naturally leaves the currently defined byte ledger zero.
* Artifact helper199–202 and207–226: finish reparses/validates schema, table reconstruction measures semantic bytes and allocates ordinary indexes/ordered arrays. These preserve the same operation but do not debit it. Physical exportSqliteDatabase/importSqliteDatabase do own charged buffers, but the failing tests call semantic to/fromDatabase, not physical file conversion. Invoking physical conversion merely to create artificial charges would change the operation under test.
* `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts:65–68`: Reader constructs a separate NativeDecodeControl using maxValueBytes rather than caller remaining allocation. Its 64/384/16 credits settle nowhere into SqliteOperation. Their surrounding allocations are Map/Set/JS arrays and objects, not storage of proven those widths. Forwarding those guessed credits would fix the nonzero assertion without proving backing; do not do that.

## Safe Joins and Required Backing Work

A narrow public SqliteOperation.allocationStage<C extends {readonly ownedBytes:number},T>(create,operation) can delegate directly to its existing private allocation.stage. Preserve the existing finally settlement and remaining budget; keep callbacks/signal on the actual child. The direct closed 64-octet test at Block5d Source177–183 already exercises real NativeDecodeControl.copyBytes, whose Value decode source66 allocates an exact Uint8Array after charge. No new allocation implementation or ABI estimate is required for this stage API.

For the full domain routes, retain the same sqliteOperation once at entry and pass it through schema, capture, projection, ordering and restoration. A whole Reader lifetime must be inside the child stage, not just Reader.create: take/group/finish debit later. Removing the detached maximum and settling its existing guessed slots is still insufficient. A reviewed backing design must replace those guessed object credits with actual firstparty byte storage used by parsing/capture/indexing, whose lengths are checked before allocation and whose work frontiers preserve existing progress/cancel behavior. Do not allocate dummy bytes simply to charge an existing JS object graph. Shared schema parsing similarly needs a real storage contract if it is to claim paid backing; its current work controls alone are honest but not a debit.

The original test assertions at Source197–204 use observed probe debit for exact/short/two-call cumulative ceilings and retain full expected outputs. They do not establish JS allocator equality or diagnostic release; no test weakening or replacement numeric threshold is justified.

## Block2d/3d Design Qualification

The retained `📓️block-two-three-direct-source-implementation-design.md` correctly identifies the same gap and specifies direct ten/thirteen handcrafted tables, exact IEEE triplets, optional all-null groups, ownership/ordinal rejection and mutable input capture before callbacks. Its reuse of shared Projection is semantic/work reuse, not existing paid capture. Block2d/3d must not copy Block5d Reader guessed credits. Their proposed capture/index storage and supplied child-stage API remain concrete implementation work. A source facade that only wraps shared Projection will reproduce zero-byte projection on primitive-only owners. Complete independent SQLite/field-edit/optional laws remain unchanged; this audit grants no runtime readiness to an unmounted implementation.

No production, tests or scripts were edited. Only this report was written; current concurrent Value retirement repairs are outside this Source ledger scope.
