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
