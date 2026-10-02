# Native Pack Admission for SQLite Export

The erased SQLite exporter currently decodes native Pack with default limits before applying the caller's relational budget. This allows a small compressed Pack to materialize a large owned snapshot before projection rejects it. A second gap is aggregate record storage: ordinary document and record-body decoders do not use the cumulative materialization authority already present for exact DslValue decoding.

The language-neutral fixture declares three actual intrinsic-buffer cases: one 131072-byte expansion, two individually admissible 61440-byte buffers whose aggregate exceeds 100000 bytes, and two admitted 16384-byte buffers. The runtime law uses the real compressed Pack encoder/decoder, the erased SQLite callback, a completion counter after native decoding, and independent Bun SQLite queries and Ajv fixture validation. Rejected cases must never reach native-decoder completion; early cancellation must also prevent completion.

The intended repair passes caller-specific Pack limits to the native decoder, and activates one cumulative owned-storage budget for document and inline-record decoding. Counts and owned buffer/string/collection/map storage are checked before allocation. Physical chunk lengths are admitted before payload integrity/decompression work. Bounds describe logical owned storage, not allocator bookkeeping or a total process-memory limit. Native typed binding may introduce further owned copies; those remain a separate owner audit.

Current validation is pending. The red run was started through the registered framework OS kernel native route with filter sqlite_snapshot_native_decoding_. Full PDF quick and full DSL quick runs were also restarted after the shared preparation queue repair. This report must not be read as a passing result until actual runtime evidence is appended.

## Public Admission Test Owner

The canonical kernel package now declares the integration test sqlite_snapshot_native_admission, whose wrapper imports public kernel/store/DSL/Pack interfaces and includes the authored decoding, encoding and intrinsic-octet laws. The route is `bun nx run @semio-tech/framework-os-kernel:test-snapshot-native-admission -- quick`; its launch entry is ⚖️gate🏪️store🪶️native-admission, group 4_gate, order 408.315 (408.32 already belongs to value-resident). This route retains existing test levels and budgets.

The original lib-test decoder attempt reached compilation but stopped at unrelated MutationLeaf source-authority and concurrently added HistoryLog/envelope fields. Full PDF quick stopped in the separate UI contract owner while Input/Slider/NumberStepper field catalogs and builders were being updated. The public admission target is now running so the actual erased callback law can be exercised without compiling unrelated private kernel test fixtures. Production library compilation remains required; it is not bypassed.


## Semantic Validation Boundary

SQLite export and import now call the concrete provider's four-argument owned subset policy over the typed snapshot and handwritten relational database. The shared wrapper preserves warnings and rejects Error/Fatal diagnostics. Exact dialect registry lookup, metadata comparison, schema validation and guest component rejection remain required. Ordinary native conversion routes retain their existing byte validators; those validators are no longer applied to SQLite state that the owner's logical model admits even when a physical RIFF/PDF/ZIP encoder cannot represent it.

The strict fixture now covers warning fidelity through both erased Binary and Text roundtrips, fatal severity and rejection by a different standard's strict owner callback. Existing raw branch, schema, foreign metadata and manually composed discontinuous route laws remain. These latest changes are authored but not yet claimed passing.

The first public integration compilation reached production kernel successfully, then failed only in the new wrapper's dependency aliases. The wrapper now uses the kernel's actual public root store facade and the canonical extern protocol name. The fresh registered target remains in preparation/compilation; no aggregate runtime result has yet been observed.

## Allocation Scope

The next repair accounts cumulative decoded intrinsic octets, UTF-8 copies, collection storage, record/map slots, inline symbols and columnar table storage before ownership allocation. A fixed floor is combined with the actual Rust slot size, rather than assuming every FieldValue fits 64 bytes. Frame concatenation checks its caller budget before copying. Physical symbol/chunk metadata allocations have their own caller bound; this is a logical storage bound, not a claim about allocator overhead or total process RAM.

Native decoding currently has cancellation checkpoints before and after the native codec. Interior native codec checkpoints and further typed binding ownership copies remain an explicit audit item; projection/reconstruction and physical SQLite already carry their controlled checkpoints. No unrestricted cancellation or universal completeness claim follows from the new admission law.


## Erased Snapshot Retirement

The trait now declares `retire_sqlite_snapshot(self)`, with ordinary drop for pure snapshot owners. Retained owners explicitly override it with their actual cold domain retirement. Generation2d and Generation3d ownership was confirmed against their inherent iterative FlowRetirement and GenerationRootRetirement implementations; rust_codec owns those overrides.

The new native-retirement fixture has twelve explicit cases covering Binary/Text success, cancellation before decode, after decode, before reconstruction, after reconstruction and before encoding, semantic owner rejection, and native encoder rejection. Its retained value refuses bare Drop. The runtime test catches that panic, checks exactly one owner retirement whenever a snapshot was constructed, and independently validates the authored scalar table and fixture with Bun SQLite and Ajv. Guard wiring and red/green runtime evidence remain pending; the fixture is included in the actual public kernel integration test.

The plugin semantic boundary runtime attempt reached its registered router but stopped before Cargo: the generated plugin inventory caller invokes the deployment owner `moduleDirectoryName` without its newly mandatory explicit inventory, producing `inventory.filter` on undefined. No compatibility inventory or implicit fallback was introduced by this task.

The admission corpus now also declares 2048 empty intrinsic buffers under a 16384-byte materialization limit. This distinguishes aggregate collection slots from octet length: zero payload bytes still require owned list/vector storage. The independent SQLite oracle must observe the empty octet sum faithfully. This fourth case is authored before the cumulative decoder repair.

## Owner-Authoritative Controlled Decode Proposal

The shared erased exporter must eventually replace its direct `ArtifactDsl::parse_dsl` / `ArtifactPack::decode_pack_with` dispatch with one explicit owner hook on `ArtifactSqliteSnapshot`:

```rust
fn decode_sqlite_snapshot_native(
    payload: &io_schema::IoPayload,
    control: &mut sqlite_snapshot::SqliteSnapshotControl<'_>,
) -> Result<Self, String>;
```

The initial default must checkpoint `DecodeNative` and refuse with an owner-missing-controlled-decoder error. It must never invoke the ordinary parser as a default. Each concrete owner has the exact Binary/Text model and chooses its borrowed parser, materialization admission and partial-construction retirement. The shared erased scope acquires the successfully returned `P` immediately, preserving the new retirement guarantee. Dynamic components invoke their registered owner's same controlled path.

Ordinary Pack decoding needs an internal callback-aware path through metadata, frame reading, record slots, symbols, chunk admission and decompression. It must not import the SQLite domain into the physical Pack crate: a first-party callback receives native work progress and the snapshot owner forwards it to its supplied control. Ordinary text parsing likewise needs control inside lexing, parsing and value construction; a checkpoint only before/after `parse_dsl` is insufficient. Text ingress limits must be checked before parsing, Binary ingress limits before opening, cumulative ownership before allocation, and callbacks before large copies/decompression plus bounded traversal intervals.

This API is not yet authored. The current public cumulative Pack regressions must first reach their actual RED/GREEN. The present export implementation forwards Binary byte/collection limits but only brackets native decode with checkpoints; Text still calls the ordinary parser. Owners which ignore Pack options remain uncovered. Prior owner fidelity greens are not proof of bounded/cancellable erased native decoding.

## Additional Public Inline Collection Law

The public kernel integration now also reads the neutral empty-buffer expansion case through `pack_rt::encode_record_body` / `decode_record_body`. Its compact input fits the caller's16384-byte ceiling, while2048owned empty buffer slots exceed it. The law asserts refusal before admitting that expanded owned collection. The current inline context still has no cumulative ledger, so this is an authored pending RED alongside the actual compressed-document counter law; it has not yet executed.

## Actual Native Admission Stack

The live public-kernel Cargo51224 was sampled read-only after approximately54minutes without a compiler child. Its main thread is entirely in `BuildRunner::prepare_units → Layout::new → Filesystem::open_rw_exclusive_create → flock`. This is layout admission, before the fixture compilation or assertions. Its open lock files are the canonical `build/debug/.cargo-build-lock`, `target/debug/.cargo-lock`, and `target/debug/.cargo-artifact-lock` (FD7/8/9). Numerous other live Cargo processes reference the same three nodes. No processes or lock/cache files were modified or removed.

The exact generated sample is retained as `🗑️generated/kernel-cargo-admission-stack.txt`; temporary diagnostic output is scoped to the ticket. The cause cannot yet be assigned to a specific holder from this sample alone. This waiting attempt provides no red/green assertion evidence.

## First Public Runtime Results

Registered public integration run `32c7c111-3807-44a8-af97-9e5999f6bb31` executed7laws in0.248seconds after59minutes of build admission:5passed and2failed. The new inline empty-collection law produced a genuine RED:2048empty buffers were admitted despite the16384-byte ownership ceiling. The retirement, encoding-preflight and three intrinsic-octet laws all passed through the public production kernel seam.

The compressed aggregate law's first failure was its prerequisite assertion (`fixture must exercise decoded expansion`), not the intended decoder counter. The current canonical Pack encoder writes intrinsic chunk payloads as identity fragments, so forcing chunking did not create a compressed expansion. This fixture is corrected to keep intrinsic bytes inline and split the compressed Document into32768-byte frames. Each frame fits below the caller ceiling while the aggregated body/owned buffers exceed it. No canonical chunk semantics were changed. A focused registered retry is pending; this first attempt is not claimed as an aggregate decoder RED.

## Separate Physical SQLite Storage Accounting Observation

The physical engine's `value_size` intentionally charges `Null=0`, numbers=8, and text/blob payload length. `read_record` grows an owned `Vec<SqliteValue>` while enforcing that semantic byte sum, and `import_sqlite_database` accumulates the same value sizes before pushing owned rows. It enforces row/column/file bounds separately. Thus `max_value_bytes` is a semantic payload ceiling, not a ceiling on total owned row/value slots. A wide row with nullable columns can consume significantly more owned slot memory than that value-byte count.

This observation is distinct from the Pack native materialization repair and has not been changed in this integration scope. If a strict total owned-memory ceiling is part of the shared control contract, it needs an explicit physical row/value allocation budget or separately named limit in both native and TypeScript physical engines, plus a language-neutral wide-null-row resource law. The present integer/blob/string budget tests should not be represented as proving that total slot-allocation bound.

## Controlled Native Decode Ownership And Admission Work

The trait now declares `decode_sqlite_snapshot_native(payload: &IoPayload, control: &mut SqliteSnapshotControl) -> Result<Self, String>`. Its default checks cancellation at DecodeNative and rejects a missing owner implementation. Erased dispatch has not yet been switched while the cumulative Pack law is awaiting its actual RED. This is an interface declaration, not an implemented universal bounded decoding claim. Concrete owners must carry callbacks and caller budgets inside parsing and final typed construction; wrapping an ordinary parser with start/end callbacks is insufficient.

The inline Pack context now enables its cumulative materialization ledger following the actual empty-collection RED. Document decoding remains awaiting its focused counter RED. A further neutral case separates body-only admission from simultaneous body plus typed ownership: six2048-byte buffers and a20000-byte ceiling. Both physical body and semantic fields fit separately, while simultaneous ownership exceeds the allowance. This case has not executed yet.

Additional metadata allocation admission is authored in the physical Pack symbol/chunk-table decoders: checked address-space lengths, cumulative symbol strings plus slots, bounded chunk slots, fallible pre-reservation, and source length admission before superblock reads. These changes have not yet passed a fresh native gate.

The current JPG Cargo70019 was sampled read-only after58minutes, showing the same Layout::new/flock admission stack and no compiler child. The sample is retained under generated output; no other process or shared cache was changed.

## Explicit Metadata Resource Boundary

The direct Rust Projection now counts only actual domain rows, matching TypeScript. Hidden reservation of a metadata row made a direct three-row provider fail under maxRows3 even though its returned database contained no metadata. Rust worker authored identical exact-three-row neutral Playbook and Procedure laws first; their native execution remains queued and is not reported as an observed RED.

Actual I/O metadata attachment now accepts caller control and checks the final domain-plus-metadata row total, semantic scalar/text/blob bytes, summed SQL-plus-table-name bytes, final table count and six required metadata columns before cloning or mutating the database. Cancellation is checked before traversal and before copying metadata. Typed and erased I/O, guest-host import, and the PDF forged-profile fixture use this canonical controlled boundary.

The public kernel integration adds `sqlite_snapshot_metadata_admits_final_file_before_mutating_domain_database`: a neutral three-domain-row/four-file-row fixture, five resource refusals and cancellation leave the original database unchanged; an admitted file is checked independently with Ajv and Bun SQLite integrity and row counts. This new law is authored but not executed yet. The public integration now contains eight laws; the last actual five-pass/two-fail run predates this addition.


### Retained DEFLATE Working Storage

After strict controlled dispatch activation, the actual 26-law kernel gate produced 25 passes and one failure (Nextest `af7eb6a2-f8c2-42ed-80ad-b7a1c9d17613`). The old positive case admitted two 16,384-byte buffers with a 100,000-byte cumulative native budget, which now correctly includes retained inflater history together with physical parser storage and final typed ownership. Each compressed segment's retained DEFLATE cursor requires a bounded history allocation of up to 32 KiB, admitted and charged before allocation. That cumulative storage remains charged across stages; the byte limit is not only the final intrinsic buffer sum.

The same 100,000-byte fixture is retained as the explicit `retained-inflater-working-storage` refusal. A separate `admitted-buffers` positive fixture requests a bounded 262,144-byte allowance. Neither the strict inflater physical ceiling nor any of the existing expansion/slot refusal cases is weakened. The changed neutral fixture passed the complete registered private-lane kernel gate: 26/26 laws GREEN, Nextest `1d1f77e8-7a05-437f-ad0c-168f698bdf92`, 0.764s assertions/33.5s uncached Nx. Strict dispatch, cumulative parser/inflater/typed ownership, all expansion refusals and cancellation remain covered. Independent Bun SQLite validates the exact logical intrinsic byte total for every case, and Ajv admits the neutral fixture against its owned schema.
