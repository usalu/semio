# DWG Exact Roundtrip Implementation

## Ticket Context

- Ticket: `2026/08/11/SEMIO-ARTIFACT-LOSSLESS-WELL-KNOWN-FORMAT-ROUNDTRIPS`
- Goal: `🎯aioptimizedrepo`
- Repository MCP fallback: the required repository MCP was unavailable with a broken pipe. The already-open ticket and its on-disk reports were used without opening a duplicate ticket.
- In-process integration: the AC1024 snapshot was found midway through an uncommitted `bytes` → shared `ArtifactSource` change while diff, mutation, inference, artifact, and facet consumers were also changing. The work preserved that intent and completed the live contract rather than reverting it.

## Result

The initial `ArtifactSource` whole-file replay design was rejected as non-semantic and has been removed from the DWG snapshot/artifact/diff path. A subsequently prototyped structured physical partition was also removed after the requirement was clarified to prohibit serializer materialization state in the schema.

The active implementation is a source-free logical AC1024 model. Named sections carry decoded semantic payloads plus standard-defined page/section metadata, while the writer materializes compression, checksums, encrypted page headers, system maps, page addresses, alignment, and the encrypted file header deterministically. The writer currently has a canonical literal-only R2004 compression fallback with boundary laws; it does not retain compressed tokens, padding, page headers, checksums, or any whole-file/page/section encoding.

The existing architectural example test now covers the real fixture end to end through raw binary I/O, DSL, ArtifactPack, empty diff, no-op mutation, supported header mutation, inverse diff, absorbed inverse, unsupported section mutation, and mutation inverse restoration.

Document discovery declares `.dwg` instead of `.bin`, and DSL/Pack/raw I/O route through logical decode and deterministic writer materialization.

## Exactness Constraint Found in the Standard

The official Open Design specification §4.7 states that R2004 compression match finding is not canonical: brute-force and hashed match finding produce different legal streams, and ODA explicitly trades compression ratio for speed. Consequently, decoded logical bytes do not determine AutoCAD's original LZ77 token stream. Exact import-byte reproduction is information-theoretically impossible for arbitrary files after all encoding decisions are discarded. The canonical writer can produce a deterministic valid DWG; matching this particular fixture requires its producer to have made the same canonical choices. The first byte mismatch will be recorded once the shared stdio crate is buildable.

## Exact Fixture Evidence

The requested source and the existing repository fixture are the same file image:

```text
[DEBUG] source=/Users/ueli/Documents/semio/temp/architectural_example.dwg
[DEBUG] bytes=148638
[DEBUG] sha256=52d14a7bdb946099d3cf16fd276d19bd8924348fd02b2ddd0003cd4f6b34cce7
[DEBUG] repository_fixture_sha256=52d14a7bdb946099d3cf16fd276d19bd8924348fd02b2ddd0003cd4f6b34cce7
[DEBUG] cmp_exit=0
```

## Validation

The original focused build used the repository's Bun/Nx entry point with an isolated ticket-local Cargo target:

```text
CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/SEMIO-ARTIFACT-LOSSLESS-WELL-KNOWN-FORMAT-ROUNDTRIPS/🎯️dwg-target' bun nx run @semio-tech/stdio-plugin:test-quick -- exact_fixture_roundtrips_through_snapshot_diff_mutation_and_raw_io --nocapture
```

The clean target compiled dependencies for 1,200,000 ms, reached the shared `semio-framework`/`semio-framework-3d` crates, and was then killed by the repository script's cargo-build time budget:

```text
[budget] cargo build --tests -p semio-s-plugin-stdio exceeded 1200000ms — killed.
NX target failed
```

No DWG-local or stdio test-crate compiler diagnostic was emitted before the budget termination, and the test executable was not reached. Therefore this report does not claim that the focused test passed or that its authored `[DEBUG] DWG raw import/export ...` runtime line executed. A second build was intentionally not started after the parent workflow requested that this build be treated as the only active run.

Read-only static validation completed successfully:

```text
[DEBUG] git_diff_check_exit=0
[DEBUG] dwg_json_jq_exit=0
[DEBUG] stale_ac1024_bytes_reference_count=0
```

The source-to-fixture `cmp` and SHA-256 evidence above independently confirm the exact byte image used by the extended test.

## Remaining Validation

The writer-stage focused Nx build reached the stdio crate but is currently blocked by concurrent XML/SVG schema work (`XmlLexicalToken`/`XmlLexicalDocument` and `prolog` errors) plus source-removal fallout in four semio DWG drawing/mesh bridges. No diagnostic in the new compression/checksum/header/map writer was emitted before those shared failures. Once the shared crate compiles, run the primitive laws, emit the canonical fixture, record its first mismatch against byte 0 of the source, and continue replacing opaque decoded section payloads with typed AC1024 records/entities.
# Strict physical-layout checkpoint (2026-08-14)

> Superseded by the later logical-only override below.

The selected AC1024 authority is a typed physical layout: partitioned preamble, decrypted primary and secondary file headers, ordered page-directory entries (including gap-tree records), ordered allocated physical pages with typed encrypted-data/system-page headers, retained per-page compressed/encrypted payloads and allocation tails, and a typed trailer. Logical sections are a projection. Export must serialize those records in physical order; unsupported dirty logical section edits must fail explicitly, while header edits and mutation inverse preserve physical provenance.

During the parallel edit window the completed physical decoder/writer block in the shared DWG IO file was overwritten by a concurrent canonical-writer edit. The schema/artifact/diff propagation remains in the live tree, but the current IO file therefore does **not** satisfy this checkpoint until `decode_r2004_physical` and the physical writer are restored. No compile/test claim is made; Nx/Cargo were intentionally not run under the root-agent restriction.

## Restoration result

The strict block was restored to the live IO file after the checkpoint above. `decode_r2004_physical` now partitions the preamble/header, decrypts and types both file headers, persists the page directory and gap-tree metadata, walks physical allocations in original order, types data/system page headers, and retains only page payload/tail or unknown-record bytes opaquely. `encode_r2004_physical` reconstructs those records in directory order, validates addresses/allocations and the retained decompressed page map, and `encode_r2004_snapshot` rejects dirty logical sections instead of calling canonical reconstruction. Physical state is again propagated through snapshot, artifact, diff, set-snapshot, absorb, and inverse paths.

Read-only verification performed under the root restriction:

- `rustfmt --edition 2021 --emit stdout` parse-checked IO, snapshot, artifact, diff, and inference Rust files successfully.
- `git diff --check` succeeded for the AC1024 tree.
- Live IO SHA-256: `70b81581db7859bc9690162cd1c8393cd8451c9dfd3b04a48656880b6aec12c8`.
- Fixture SHA-256: `52d14a7bdb946099d3cf16fd276d19bd8924348fd02b2ddd0003cd4f6b34cce7`.
- Nx/Cargo were not run, as explicitly required by the root agent; central compilation remains the root handoff step.

## Logical-Only Override and Concurrent Collision (2026-08-14)

The user's later explicit direction supersedes the physical-layout checkpoint: the persisted schema must retain no source file, encoded page, encoded section, compression token stream, padding, encrypted header, checksum, trailer, or other serializer materialization state. `ArtifactSource` and physical replay are prohibited. The shared authority is the typed logical drawing/entity model, and the writer must materialize native AC1024 structures deterministically.

The first removal completed in the live files, but an unknown concurrent worker restored the prohibited model during the focused Nx build. Exact collision evidence:

```text
[DEBUG] first_reintroduced_snapshot_mtime=1786709713
[DEBUG] first_reintroduced_snapshot_size=27890
[DEBUG] first_reintroduced_snapshot_sha256=e544e2b4f695d965f6b431df45cfee5be7322302f0d84863cf021867d02134d4
[DEBUG] first_reintroduced_io_mtime=1786709713
[DEBUG] first_reintroduced_io_size=118334
[DEBUG] first_reintroduced_io_sha256=e20fcb72fbc404c1f74afbc1b3239c91a8d5b82fe392ece15cd076827463e252
[DEBUG] second_reintroduced_io_mtime=1786709997
[DEBUG] second_reintroduced_io_size=134816
[DEBUG] second_reintroduced_io_sha256=f1dd6276b4a7a38d8537e6b8b6317e32a9d57b5565a307a2024a2483bae9617d
```

The second IO rewrite actively restored `decode_r2004_physical`, `opaque_payload`, `trailing_bytes`, `encode_r2004_physical`, and a physical-replay `encode_r2004_snapshot`. Per coordination direction, edits to the colliding snapshot and IO files were paused and the exact hashes were reported to the root workflow.

Outside those two collided files, the artifact/diff/inference propagation and TypeScript, GraphQL, JSON Schema, Proto, DSL grammar, and binary protocol documentation were changed to remove source/physical provenance and expose logical version/drawing state. The JSON facets were parsed at runtime:

```text
[DEBUG] validated DWG JSON facets 3
```

The one focused validation command was:

```text
CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/SEMIO-ARTIFACT-LOSSLESS-WELL-KNOWN-FORMAT-ROUNDTRIPS/🎯️dwg-target' bun nx run @semio-tech/stdio-plugin:test-quick -- lz_writer_roundtrips_every_literal_length_boundary --nocapture
```

It reached the stdio crate and failed with 76 compile errors. Most were concurrent IFC/XML/SVG/MP4 integration failures. The first DWG-local error was the reintroduced snapshot call to missing `dwg_engine::decode_r2004_physical`; this is direct build evidence of the race rather than a logical-writer diagnostic. The test body did not run, so no primitive-law or byte-roundtrip pass is claimed.

The subsequent non-collided audit removed the reintroduced physical propagation from artifact, diff, and inference files; removed stale source/codec references from bridge docs and imports; changed both drawing/mesh exporters and their importer test fixtures to construct `DwgLogicalDrawing` directly; and replaced mutation facet `bytes`/`bytes_wire` placeholders with logical variant fields. Final audit evidence:

```text
[DEBUG] non_collided_physical_source_matches=0
[DEBUG] drawing_mesh_bridge_encoded_intermediates=0
[DEBUG] remaining_snapshot_physical_matches=10
[DEBUG] remaining_snapshot_sha256=e544e2b4f695d965f6b431df45cfee5be7322302f0d84863cf021867d02134d4
[DEBUG] remaining_io_physical_matches=27
[DEBUG] remaining_io_sha256=70b81581db7859bc9690162cd1c8393cd8451c9dfd3b04a48656880b6aec12c8
```

Thus every remaining prohibited physical/source reference is confined to the two explicitly paused collision files: AC1024 snapshot and IO.

## Strict Physical Recovery (2026-08-14, Final Handoff)

The root workflow's latest direction supersedes the logical-only collision note above and makes the typed physical AC1024 layout authoritative again. The live shared tree now contains the complete strict model alongside the existing logical drawing and D1/D2 section projection:

- `DwgPreamble`, decrypted `DwgFileHeader`, `DwgPageDirectoryEntry`/`DwgGapTree`, typed data/system `DwgPhysicalPage`, `DwgTrailer`, and `DwgPhysicalLayout` are persisted snapshot state.
- Snapshot, artifact, diff, mutation set-snapshot/inverse, and structural inference paths propagate `physical` without a whole-document source field.
- `decode_r2004_physical` partitions the native document into typed fields and confines opacity to allocated page payload/tail or unknown physical records.
- `encode_physical_page` and `encode_r2004_physical` write every typed field in physical order, validate allocation/address/directory invariants, and never replay a retained whole-file buffer.
- `encode_r2004_snapshot` compares the logical D1/D2 projection against the physical pages and rejects unsupported dirty section edits.
- `ArtifactDsl` and `ArtifactPack` now lower `DwgSnapshot::__dsl_to_record()` and restore it through `__dsl_from_record()`; they no longer hex-wrap or binary-wrap a reconstructed native DWG file.
- The real-fixture test explicitly rejects a complete native-file occurrence in DSL/pack, then verifies direct/raw/DSL/pack/self-diff/no-op/set-snapshot/diff/absorb/header-mutation/inverse exact reconstruction.

Static validation only, per root restriction:

```text
[DEBUG] rustfmt_edition_2024_parse_only=pass
[DEBUG] ArtifactSource_matches=0
[DEBUG] snapshot_whole_file_source_or_bytes_fields=0
[DEBUG] native_DSL_or_pack_wrapper_matches=0
[DEBUG] fixture_bytes=148638
[DEBUG] fixture_magic=AC1024
[DEBUG] fixture_sha256=52d14a7bdb946099d3cf16fd276d19bd8924348fd02b2ddd0003cd4f6b34cce7
[DEBUG] final_snapshot_sha256=aa6a8df86f6140791fbb0865481209303c07bc2ea5faba4d329a2bc8267a1a74
[DEBUG] final_io_sha256=65db43a8671fa196be5436dab92a948b935ca10b301b793f47db59a69a8d4778
[DEBUG] final_30_second_mtime_hash_stability=pass
```

Nx/Cargo were not run. The existing handcrafted snapshot `.grammar.semio`, `.protocol.semio`, and shipped demo DSL/pack fixtures still describe the superseded native-byte container and must be regenerated centrally from the now-authoritative derived record spec before running the broad conformance-law group. This does not affect Rust parse shape or the focused real-fixture roundtrip test, but it is known runtime conformance fallout rather than a claimed pass.
