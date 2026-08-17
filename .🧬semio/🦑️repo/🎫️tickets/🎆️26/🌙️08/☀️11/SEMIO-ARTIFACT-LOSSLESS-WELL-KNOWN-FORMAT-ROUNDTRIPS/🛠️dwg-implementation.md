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
# Governing Logical-State Override (2026-08-14)

The final governing contract supersedes every physical-layout checkpoint in this report. The active AC1024 schema contains no DwgPhysicalLayout, retained encrypted/compressed page payload, padding, trailer, or source replay state. Snapshot, artifact, diff, mutation, inference, facets, and I/O now carry logical standard concepts only. encode_r2004_snapshot delegates to encode_r2004_canonical for deterministic materialization from logical drawing/section state.

The existing fixture lifecycle test remains the acceptance route for direct/raw/DSL/pack/diff/no-op/inverse/absorb behavior. Static parse checks completed; no runtime exactness claim is made in this override.

## Zero Imported-Byte State (2026-08-14)

`DwgSectionPage.decoded` and its base64 DSL annotation were removed. Snapshot state now retains
only the typed logical drawing plus named section/page descriptors, sizes, identifiers,
compression/encryption semantics, and decode diagnostics. The importer discards transient
decompressed page buffers after decoding; the only remaining `decoded: Vec<u8>` is the private
native-I/O work record used during deserialization and is neither serde/dsl state nor reachable
from snapshots, diffs, or mutations. Structure inference now computes counts from descriptors and
declared logical sizes. The canonical writer no longer reads retained page payload bytes.

DWG TypeScript, GraphQL, JSON Schema, and Proto snapshot facets now expose the named section/page
descriptor model without payloads. Stale diff/mutation grammar `source`, native `bytes`, and page
`decoded` productions were removed. An existing DWG test module now compile-time audits Rust and
all requested facets/protocols against reintroduction of those fields.

Validation:

```text
[DEBUG] rustfmt_parse_dwg=pass
[DEBUG] dwg_snapshot_json_jq=pass
[DEBUG] persisted_dwg_decoded_fields=0
[DEBUG] dwg_source_grammar_fields=0
[DEBUG] scoped_nx_lane_local_errors=0
```

The isolated scoped Nx build is currently blocked before test execution by unrelated PDF/MP4
compile errors. The exact DWG lifecycle therefore has not run. More importantly, eliminating the
page-byte shadow exposes an existing semantic gap: the AC1024 importer locates/decompresses named
sections, while the typed entity reader still accepts only the AC1015 writer dialect. The current
`temp/architectural_example.dwg` equality assertion cannot pass from logical state until AC1024
section contents are fully decoded into typed header/classes/objects concepts and the AC1024
writer materializes those concepts. No source replay or retained page payload remains to mask that
gap.

## Logical R2010 Object Decoder Progress (2026-08-14)

The AC1024 object reader now treats data and handle streams separately using the R2010 modular
handle-stream bit count. The bit reader implements the standard 3B, BLL, DD, and BT primitives.
Common entity decoding covers graphic presence, entity mode, reactors, extension dictionary,
encoded color, linetype/plotstyle/material flags, shadow, visibility, and lineweight; common
handles are consumed from the independent handle stream. Standard geometry prescriptions were
replaced for LINE, POINT, CIRCLE, ARC, ELLIPSE, 3DFACE, and LWPLINE.

The real fixture projection increased from 3 to 18 typed entities. The inventory remains 652
framed objects, so 634 frames are not yet represented in logical artifact state. This is an
intermediate metric only: table/control records, dynamic-class objects, remaining fixed entities,
classes, header variables, dependencies, summary, application, and template data still require
typed decoding and canonical writing.

Validation used the ticket-local target:

```text
[DEBUG] full_zero_selection_nx=pass (0 run, 3369 skipped)
[DEBUG] real_decode_projects_logical_state=pass (1/1; layers=1, entities=18)
[DEBUG] schema_facets_contain_no_container_shadow_state=pass (1/1)
[DEBUG] exact_fixture_lifecycle=fail (strict assertion retained)
[DEBUG] exact_first_mismatch=offset 11
[DEBUG] canonical_length=2476
[DEBUG] fixture_length=148638
```

The exact lifecycle failure is the intended honest signal: the deterministic logical writer
cannot yet reconstruct unsupported semantic concepts. No source/native bytes, decoded page
buffers, section/page descriptors, compression metadata, or physical layout were reintroduced
into snapshot, diff, mutation, DSL, pack, or language facets.

## Typed Named Sections and Layer Tables (2026-08-14)

The AC1024 snapshot now models all fields used by the fixture's standard AcDb:SummaryInfo,
AcDb:AppInfo, AcDb:FileDepList, and AcDb:Template payloads. Summary metadata includes edit
time, typed Julian create/modify dates, and custom key/value properties. Application checksums
are semantic UUID-form identifiers rather than retained byte arrays. Dependencies include their
feature, paths, GUIDs, timestamp, file size, graphics flag, and reference count. The template
contains its description and measurement system.

The deserializer reads the fixture's R2010 UTF-16 encodings into those types and the ordinary
section writers reproduce each decompressed section payload exactly. AcDb:Classes already has
the same typed decode/write property. Layer table records now use the R2010 separate string
stream and standard record fields instead of the obsolete inline T/RC approximation.

Validation used the ticket-local target:

    [DEBUG] full_zero_selection_nx=pass (0 run, 3375 skipped)
    [DEBUG] real_fixture_named_sections_roundtrip_as_logical_records=pass (1/1)
    [DEBUG] real_fixture_classes_roundtrip_as_logical_records=pass (1/1)
    [DEBUG] real_decode_projects_logical_state=pass (1/1; layers>=7, entities>=18)
    [DEBUG] object_inventory=652
    [DEBUG] projected_geometry_entities=18
    [DEBUG] remaining_object_frame_semantics=634

The canonical container writer now includes these typed named sections. It still writes the
objects section through the incomplete legacy drawing serializer and has no real AC1024 handle
map, so the strict whole-document equality test remains red. The next required implementation is
typed control/table/custom-class object projection followed by standard AC1024 object-frame and
handle-map serialization.

## Logical Object Identity Inventory (2026-08-14)

Every R2010 object frame in the fixture is now represented in `DwgLogicalDrawing.objects` by its
standard handle, type code, resolved fixed/custom class name, and semantic category (entity,
table control, table record, dictionary, object, or custom). This state contains no object-body
bytes, offsets, bit positions, frame sizes, compression data, or container descriptors. Artifact,
snapshot, and diff TypeScript/GraphQL/Proto facets plus artifact JSON Schema expose the same
logical record.

Validation used the ticket-local target:

    [DEBUG] full_zero_selection_nx=pass (0 run, 3377 skipped)
    [DEBUG] real_decode_projects_logical_state=pass (1/1)
    [DEBUG] logical_object_identities=652
    [DEBUG] empty_resolved_class_names=0
    [DEBUG] rustfmt_parse=pass
    [DEBUG] artifact_json_jq=pass

This is an inventory of logical identity, not a claim that all 652 object bodies are semantically
decoded. The strict exact writer remains blocked on typed body fields for the non-geometry table,
control, dictionary, xrecord, layout, and custom-class objects plus AC1024 object/handle stream
serialization. The original-byte assertions remain unchanged.

## SVG Structured Persistence Audit (2026-08-14)

The SVG DSL parser now accepts only an enveloped SEMIO structured snapshot. Native SVG text is
materialized exclusively at the analyzer/import boundary through `SvgSnapshot::import_utf8`; it is
no longer a fallback branch inside `ArtifactDsl::parse_dsl`. The diff and mutation text/binary
facets now describe the live tagged/hex and recursive binary codecs instead of an unrelated object
serialization. Mutation grammar doctype values are typed `XmlDoctype` records throughout.

Validation used the ticket-local target:

    [DEBUG] svg_structured_facets_anti_shadow=pass (1/1)
    [DEBUG] svg_native_text_dsl_rejection=pass (1/1)
    [DEBUG] svg_direct_dsl_pack_exact=pass (1/1)
    [DEBUG] svg_diff_mutation_inverse_absorb_exact=pass (1/1)
    [DEBUG] svg_diff_set_snapshot_codecs_exact=pass (1/1)
    [DEBUG] svg_analyzer_text_pack_exact=pass (1/1)
    [DEBUG] svg_composer_text_pack_exact=pass (1/1)
    [DEBUG] full_zero_selection_nx=pass (0 run, 3378 skipped)

## AC1024 Unsupported Object Decode/Write Matrix (2026-08-14)

This is a read-only implementation audit of the fixture inventory. It does not change the DWG
decoder or writer. The authoritative inventory logs are `🧪️dwg-object-type-inventory.log`,
`🧪️dwg-custom-class-inventory.log`, and `🧪️dwg-logical-object-identities.log`.
The fixture has 652 framed objects and 18 projected geometry entities. The remaining 634 frames
partition exactly as follows; no success count can be assigned separately to ARC, LINE, and
LWPOLYLINE without adding per-type projection diagnostics.

| Logical family | Inventory frames | Already projected | Unsupported | Type/class evidence |
| --- | ---: | ---: | ---: | --- |
| Fixed entities | 80 | 18 | 62 | ARC 12, LINE 40, DIMENSION_LINEAR 12, LWPOLYLINE 16 |
| Dictionary/XRECORD spine | 237 | 0 | 237 | DICTIONARY 83, XRECORD 145, ACDBDICTIONARYWDFLT 1, DICTIONARYVAR 8 |
| Block graph | 43 | 0 | 43 | BLOCK 10, ENDBLK 10, INSERT 12, BLOCK_CONTROL 1, BLOCK_HEADER 10 |
| Table graph | 48 | 0 | 48 | 8 controls and 40 records; seven layer values are partially projected, but their framed objects and references are not |
| Fixed support objects | 6 | 0 | 6 | VIEWPORT 2, MLINESTYLE 1, ACDBPLACEHOLDER 1, LAYOUT 2 |
| Documented style/context custom classes | 50 | 0 | 50 | TABLESTYLE 1, MATERIAL 3, VISUALSTYLE 19, SCALE 17, MLEADERSTYLE 1, SORTENTSTABLE 7, ACAD_EVALUATION_GRAPH 2 |
| Dynamic-block custom classes | 71 | 0 | 71 | 19 fixture class names, detailed below |
| Associative-constraint custom classes | 117 | 0 | 117 | 8 fixture class names, detailed below |
| **Total** | **652** | **18** | **634** | **572 wholly unmodeled non-entities plus 62 unsupported entities** |

### R2010 frame contract required by every row

The implementation must replace identity-only records with a typed object-body enum plus typed
common entity/object/table data. For each object, the decoder must create bounded main-data,
R2010 string-stream, and handle-stream readers from the modular handle-stream bit count; decode
type, object bit size, handle, typed EED, reactors, extension dictionary, owner and role-specific
references; and reject any unread or over-read bits. Entity common data additionally includes
graphic-presence semantics, entity mode, no-links, color/color-book, linetype scale and flags,
plot style, material, shadow, invisibility, lineweight, layer, and visual-style references. The
graphic value must be represented as a typed/regenerable concept, never as retained object-frame
or container bytes.

The symmetric writer must preserve logical object order and handles, emit the R2010 object type,
bitsize, handle, typed EED, main/string/handle streams and object CRC, then rebuild the handle map
from those emitted frames. It must not allocate replacement handles for imported objects. An
unsupported field or unresolved reference is an atomic error, not an identity-only object, raw
bit/body fallback, or lossy export.

### Prioritized decode/write matrix

| Priority | Family/count | Typed logical decode layout | Symmetric writer requirement | Dependency/acceptance reason |
| --- | --- | --- | --- | --- |
| P0 | Common frame foundation, all 652 | Common entity/object/table fields above; typed EED values; ordered handles; independent data/string/handle readers | Exact bounded stream sizes, string stream, handle references, CRC, object order and handle map | Every later body depends on this; exact reader consumption must be asserted per frame |
| P1 | Dictionary/XRECORD, 237 | DICTIONARY: ordered name/reference pairs, cloning and hard-owner flags. WDFLT adds default-entry reference. DICTIONARYVAR: schema byte plus Unicode value. XRECORD: ordered typed DXF-group values, cloning flag and object-id references | Emit names in the string stream and item/object-id references in handle stream; encode XRECORD strings, integers, reals, points, binary semantic values and handles by group type | Largest single coverage gain and ownership spine for custom objects. XRECORD `databytes` must be decoded to a typed value enum, never persisted raw |
| P2a | Block graph, 43 | BLOCK/ENDBLK markers; INSERT insertion/scale/rotation/extrusion and attribute/sequence references; BLOCK_CONTROL ordered headers plus model/paper-space owners; BLOCK_HEADER name, flags, base point, xref path, insert count, description, semantic preview image, units, explodability/scaling, owned entities and BLOCK/ENDBLK/INSERT/LAYOUT refs | Preserve header/entity/reference ordering and hard/soft ownership; derive counts from logical collections; encode preview only from a typed image concept | Required before dynamic-block and associative bodies can resolve ownership |
| P2b | Table graph, 48 | Generic controls: count and ordered record refs. Records: common owner/reactor/xdic data plus typed LAYER, STYLE, LTYPE, VIEW, UCS, VPORT, APPID, DIMSTYLE fields and all referenced handles | Emit R2010 record strings through string stream and record refs through handle stream; preserve logical record order and cross-table handles | Completes symbol graph. Existing layer projection is not sufficient until its record body roundtrips |
| P3a | Fixed entities, 62 unsupported | ARC: center, radius, thickness, extrusion, start/end angles. LINE: z-is-zero, start/end XY deltas, optional Z, thickness, extrusion. LWPOLYLINE: flags, conditional width/elevation/thickness/normal, point/bulge/vertex-id/width counts and ordered values. DIMENSION_LINEAR: common dimension text/placement/style/measurement fields plus extension-line points, oblique and rotation | Use the standard R2010 bit primitives (3BD, BD, BT, BE, DD and conditional fields), then common handles. Preserve vertex/bulge/width/ID ordering and dimension style/block refs | ARC/LINE/LWP failures are currently only known as a 50-frame aggregate; all 12 DIMENSION_LINEAR frames are unsupported. Add per-type projection and exact-consumption assertions first |
| P3b | Fixed support, 6 | VIEWPORT, MLINESTYLE, PLACEHOLDER; LAYOUT page setup, printer, plot flags, margins, paper/origin/units/rotation/type/window/scale/styles/shading, layout name/tab/UCS/limits/axes/elevation/extents/orthoview and viewport refs | Encode all page/layout strings in string stream and plot-view, visual-style, block, active-viewport, UCS and viewport references in handle stream | LAYOUT closes block/table graph references and is required for document-level equality |
| P4 | Documented style/context classes, 50 | Typed ODA records for TABLESTYLE, MATERIAL, VISUALSTYLE and MLEADERSTYLE. SCALE: schema/default, name, paper/drawing units and unit-scale flag. SORTENTSTABLE: ordered sort/entity handle pairs. Port and verify EVALUATION_GRAPH nodes/edges as typed graph concepts | Encode every documented conditional field and handle in prescribed order; derive counts; preserve sort-pair and graph order | Implement after dictionary/table primitives because these objects reference them. EVALUATION_GRAPH still needs a verified typed prescription |
| P5 | Dynamic block, 71 | Port typed base/action/parameter/grip and representation records, including class versions, names/descriptions, points/vectors, parameter values, dependency/action/selection sets and referenced handles | Emit class-specific data/string/handle fields in verified class order; preserve ordered selection and dependency sets | Public ODA object prescriptions do not define these bodies. Use LibreDWG `dwg.spec` only as a research source and validate fixture stream boundaries; never embed its runtime API or retain unknown bits |
| P6 | Associative constraints, 117 | Port typed ASSOCNETWORK, constraint group, value/dependency, geometry dependency, variable, parameter-dependency body and dimension-dependency body bases and derived fields | Emit base-before-derived records, class version and enabled/status concepts, expressions/evaluator IDs, compound-object paths, constraint nodes and all dependency handles | Depends on P1/P2/P5 graphs. Public ODA prescriptions are insufficient, so unresolved layouts must reject rather than fall back to raw state |

The fixed table inventory behind P2b is: BLOCK/LAYER/STYLE/LTYPE/VIEW/UCS/VPORT/APPID/DIMSTYLE
controls where present, with record counts LAYER 7, STYLE 2, LTYPE 3, VPORT 1, APPID 25, and
DIMSTYLE 2; the fixture has no VIEW or UCS record bodies. The control/body matrix must still model
their empty ordered reference lists.

### Dynamic class inventory and research boundary

| Family | Fixture class counts |
| --- | --- |
| Dynamic block (71) | BLOCKGRIPLOCATIONCOMPONENT 23; BLOCKMOVEACTION 2; ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION 2; BLOCKLINEARPARAMETER 2; BLOCKLINEARGRIP 4; BLOCKFLIPPARAMETER 3; BLOCKFLIPGRIP 3; BLOCKVISIBILITYPARAMETER 1; BLOCKVISIBILITYGRIP 1; BLOCKALIGNMENTPARAMETER 2; BLOCKALIGNMENTGRIP 2; BLOCKSTRETCHACTION 6; BLOCKSCALEACTION 1; BLOCKFLIPACTION 3; BLOCKBASEPOINTPARAMETER 1; BLOCKVERTICALCONSTRAINTPARAMETER 1; ACDB_DYNAMICBLOCKPROXYNODE 1; BLOCKHORIZONTALCONSTRAINTPARAMETER 1; ACDB_BLOCKREPRESENTATION_DATA 12 |
| Associative constraints (117) | ACDBASSOCNETWORK 5; ACDBASSOC2DCONSTRAINTGROUP 4; ACDBASSOCVALUEDEPENDENCY 23; ACDBASSOCDEPENDENCY 18; BLOCKPARAMDEPENDENCYBODY 6; ACDBASSOCGEOMDEPENDENCY 31; ACDBASSOCVARIABLE 18; ASSOCDIMDEPENDENCYBODY 12 |

### High-count concrete field-order handoff

The following matrix separates ODA-defined R2010 prescriptions from partial LibreDWG research.
`main` means the class-specific logical value order. `strings` means the R2010 string stream.
`handles` means role-specific references in logical handle-stream order after the common
owner/reactor/extension-dictionary references unless stated otherwise.

| Type/class/count | Status | Main and string order | Handle-stream semantics and writer gate |
| --- | --- | --- | --- |
| 79 XRECORD, 145 | ODA-defined | `xdata_size BL`, ordered typed resbuf values whose encodings are selected by group code, then `cloning BS`. Decode the value stream into a sum type for strings, binary semantic values, integer widths, reals, points and object IDs; size is derived on write | Object-ID references occupy the remaining bounded class handle stream in value order. Each object-ID value must bind to exactly one handle; no `xdata`/`databytes` field is allowed |
| 42 DICTIONARY, 83 | ODA-defined | `numitems BL`, `cloning BS`, `hardowner RC`, then exactly `numitems` Unicode names in the string stream | Exactly `numitems` item handles follow the common object handles. Preserve name/reference order. Ownership is hard when `hardowner` or the prescribed special entry requires it; otherwise soft. Derive count and reject duplicates or count/boundary mismatch |
| 67 APPID, 25 | ODA-defined table record | R2010 common table order: Unicode `name`, `is_xref_resolved BS`, `xref H`, then APPID `unknown RC`. Flags are semantic xref-ref/resolved/dependent/removed concepts, not a retained flag byte | APPID_CONTROL type 66 main is `num_entries BS`; its handle stream is owner, reactors, optional xdic, then exactly 25 ordered entry refs. APPID record handle stream carries owner/reactors/xdic plus nullable xref. Recompute table flags from concepts |
| 19 LINE, 40 | ODA-defined entity | `z_is_zero B`, `start.x RD`, `end.x DD(start.x)`, `start.y RD`, `end.y DD(start.y)`, conditional `start.z RD` and `end.z DD(start.z)`, `thickness BT`, `extrusion BE` | Common entity handle order only. Writer derives `z_is_zero`, emits conditional Z atomically, then owner/reactors/xdic/layer and conditional linetype/plotstyle/material/color-book/visual-style refs |
| 506 VISUALSTYLE, 19 | ODA-defined R2010 record; LibreDWG marks stable | String `description`, `style_type BL`, `ext_lighting_model BS`, `internal_only B`; then each property immediately followed by its `BS` operation/modifier: face lighting model `BL`, quality `BL`, color mode `BL`, modifier `BS`, opacity `BD`, specular `BD`, mono color `CMC`; edge model/style `BL`, intersection/obscured colors `CMC`, obscured/intersection linetypes `BL`, crease `BD`, modifier `BL`, color `CMC`, opacity `BD`, width/overhang/jitter `BL`, silhouette color `CMC`, silhouette width/halo gap/isolines `BL`, hide precision `B`; display settings `BL`, brightness `BD`, shadow type `BL` | No documented class-specific handles; after all 28 value/modifier pairs the ordinary object handle stream must be exhausted. Preserve typed color components and modifiers, not packed record bytes |
| 520 BLOCKGRIPLOCATIONCOMPONENT, 23 | LibreDWG typed research with no class-local raw-prefix macro, still fixture verification required | Eval expression: `parent_id BLd`, major `BL`, minor `BL`, `value_code BS`, conditional typed value (`BD`, `2RD`, string, `BL`, handle, or `BS`), `node_id BL`; then grip expression `grip_type BL`, Unicode `grip_expr` | A value-code 91 expression contributes one hard-pointer handle in its logical position; otherwise only common object handles are expected. Validate all 23 frames reach the same exact boundary before enabling the writer |
| 544 ACDBASSOCGEOMDEPENDENCY, 31 | Typed LibreDWG research, no class-local raw-prefix macro; public ODA body is unavailable, so fixture verification is still required | Dependency base: `class_version BS`, `status BL`, read/write/attached/delegating flags `B`, signed `order BLd`, `dep_on H`, `has_name B` plus optional Unicode name, `readdep H`, `node H`, `dep_body H`, `dep_body_id BLd`; then geom suffix `class_version BS`, `enabled B`, Unicode persistent-subentity class name, `dependent_on_compound_object B` | Dependency refs are soft/hard according to role: `dep_on`/`node` code 3, `readdep` code 4, `dep_body` hard owner code 4/360 in the research source. Implement this candidate ahead of the unknown-prefix association types, but enable writing only after all 31 frames consume exactly and reproduce symmetrically |
| 541 ACDBASSOCVALUEDEPENDENCY, 23 | **Incomplete:** LibreDWG invokes `HANDLE_UNKNOWN_BITS` before its known dependency suffix | Same known dependency suffix/order as type 544, with no verified value-specific logical fields in the public prescription | Same dependency handle roles. The 23 bodies cannot be accepted or written from this suffix alone; first reverse-engineer the unknown prefix into named concepts and assert exact stream consumption |
| 545 ACDBASSOCVARIABLE, 18 | **Incomplete:** LibreDWG invokes `HANDLE_UNKNOWN_BITS` before a known action/variable suffix | Known action suffix: `class_version BS`, geometry status `BL`, owning-network `H`, action-body `H`, action index `BL`, max dependency index `BL`, dependency count and ordered `(is_owned B, dependency H)` entries. R2010 variable suffix: `class_version BL`, Unicode name/expression-like value/evaluator/description, typed EvalVariant, `has_t78 B`, optional Unicode `t78`, final flag `B` | Owning network is soft pointer; action body and owned dependency refs are hard owners, non-owned dependencies soft pointers. R2010 class version normally omits the later owned-param/value block. Unknown prefix must become typed before any of the 18 frames are writable |

The type 541 and 545 association rows are deliberately marked blocked rather than
“implementable”: LibreDWG's `HANDLE_UNKNOWN_BITS` is precisely the raw shadow mechanism prohibited
by this ticket. Their named suffixes provide probe points and expected handle roles, not permission
to preserve an opaque prefix. Type 544 has a complete typed candidate in the current research
source and should be attempted first, but it still needs all-frame bounded consumption because it
is absent from the public ODA prescriptions. A decoder should instrument bounded bit positions in
the ticket log, infer named concepts across all same-class frames, and enable each variant only once
the writer reproduces every frame.

Primary format reference: [Open Design Specification for DWG Files](https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf), especially the fixed/dynamic type mapping and object prescriptions in section 20.4: common entity data/handles, ARC, LINE, dimensions, DICTIONARY, BLOCK control/header, table records, DICTIONARYVAR, LAYOUT, LWPOLYLINE, SCALE, SORTENTSTABLE and XRECORD. Dynamic type 500 maps to the first class-list entry, so class-list order is semantic.

Secondary implementation research: [LibreDWG `dwg.spec`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec) and its [object support inventory](https://www.gnu.org/software/libredwg/manual/html_node/OBJECTS.html). LibreDWG marks many dynamic-block and associative classes unstable, debugging, or unhandled. Those definitions are useful for discovering named logical fields, but they are not sufficient proof of a writable R2010 layout and must not become a runtime dependency. Each ported body needs fixture-bounded main/string/handle consumption and symmetric writer evidence.

### Per-family acceptance gates

1. Every inventory frame resolves to a typed body variant; identity-only fallback is forbidden.
2. Main data, string stream, and handle stream are consumed exactly with no ignored trailing bits.
3. Every owner/reactor/dictionary/table/block/dependency reference resolves in the logical graph.
4. Per-frame decode/write and the rebuilt object section/handle map reproduce the fixture bytes.
5. Snapshot DSL/pack, diff/apply/inverse/absorb, mutation/apply/inverse, analyzer/composer, and native export preserve all typed bodies.
6. Rust and language facets expose the same typed concepts and contain no source, physical, lexical, raw object-body, page/section payload, compression, or native-envelope state.
7. Any not-yet-modeled field fails import/export atomically; the writer never silently drops it.

## AC1024 Outer Materialization Policy and First-Difference Roadmap (2026-08-14)

This is a read-only audit of `temp/architectural_example.dwg`; no DWG production file was
modified. The fixture is 148,638 bytes (`0x2449e`), SHA-256
`52d14a7bdb946099d3cf16fd276d19bd8924348fd02b2ddd0003cd4f6b34cce7`. The source
authorities used to interpret the extracted bytes are the Open Design DWG specification and
LibreDWG's [`header.spec`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/header.spec),
[`r2004_file_header.spec`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/r2004_file_header.spec),
[`decode.c`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/decode.c), and
[`encode.c`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/encode.c). LibreDWG is
research evidence only and must not become a runtime dependency.

### File preamble and encrypted R2004 header

The first deterministic mismatch in the saved whole-fixture run is byte `0x0b`: the writer emits
`00`, while the fixture's `maint_rel_version` is `02`. The saved run reports
`[DEBUG] DWG raw import/export bytes=3468 identical=false`; therefore later equality cannot be
inferred from a canonicalized output. The complete fixture preamble policy is:

| Offset | Fixture value | Logical/materialized meaning |
| --- | --- | --- |
| `0x00..0x05` | `AC1024` | Version sentinel |
| `0x0b` | `02` | Maintenance release version |
| `0x0c` | `03` | R13+ `zero_one_or_three` discriminator |
| `0x0d` | `0x000001c0` | Preview payload address: page 2 address `0x1a0` plus 32-byte data header |
| `0x11` | `0x1d` | Storing application DWG version |
| `0x12` | `02` | Target maintenance version |
| `0x13` | `0x001e` | Code page |
| `0x15` | `00` | R2004 unknown-zero byte |
| `0x16` | `0x1d` | Application DWG version |
| `0x17` | `02` | Application maintenance version |
| `0x18` | `0` | Security type |
| `0x1c` | `0` | Unused R2004 address |
| `0x20` | `0x00000120` | SummaryInfo payload address: page 1 plus 32 bytes |
| `0x24` | `0` | VBA project address |
| `0x28` | `0x80` | Encrypted R2004 header address |
| `0x2c` | `0x000155e0` | Fixture-derived AppInfo payload address: page 3 plus 32 bytes |
| `0x30` | `0x00015900` | Fixture-derived AppInfoHistory payload address: page 4 plus 32 bytes |
| `0x34..0x7f` | zero | Remaining slack |

The values at `0x2c` and `0x30` are observed Autodesk extensions in the nominal slack: both land
exactly on the corresponding named-section payloads. They must be derived from the final page
addresses, never retained as imported offsets.

Decrypting bytes `0x80..0xeb` with the fixed LCG
`seed = seed * 0x343fd + 0x269ec3`, XORing the low byte of `seed >> 16`, yields this exact
108-byte record:

| Field | Fixture value |
| --- | ---: |
| file ID | `AcFssFcAJMB\0` |
| header address / size / `x04` | `0`, `108`, `4` |
| root / left / right gap nodes | `0`, `0`, `0` |
| unknown long | `1` |
| last section ID | `24` |
| last section address | `0x248a0` relative, hence cumulative allocated end `0x249a0` |
| second-header address | `0x24432` |
| gaps / page-directory entries | `0` / `22` |
| constants | `0x20`, `0x80`, `0x40` |
| section-map ID / relative address | `24` / `0x24260` (physical `0x24360`) |
| section-info ID | `23` |
| section-array / gap-array size | `24` / `0` |
| CRC32 | `0xfcad36c8` |

The CRC is IEEE CRC32 over the 108 decrypted bytes with the four CRC bytes zero. Bytes
`0xec..0xff` are non-zero standard R2004 magic-table padding, not encrypted record continuation and
not zero slack. The ODA rule is to generate the same 256-byte seed-1 LCG magic sequence, then copy
indices `0xec..0x100` directly. Those generated bytes are exactly
`f8466a0496730ed9162f6768d4f74a4ad0576876` in this fixture. XORing the extension against LCG
indices 108..127 incorrectly manufactures the apparent plaintext
`4134f74dbaf3701c8ffa8ee8661d838683e80fa0`; neither that value nor either of its apparent slices is
a format field. The writer derives the extension from the standard LCG and persists no padding state.

### Page directory, allocation, and physical stream order

The page map at `0x24360` is a system page with type `0x41630e3b`, decoded size `176`, compressed
size `170`, compression `2`, and checksum `0x01d7121f`. Its decoded body is exactly 22 `(i32 page
number, u32 allocation size)` records. Addresses are obtained only by cumulative allocation from
`0x100`; they are not persisted in the page-map body.

| Page | Section ID | Address | Allocation | Data bytes | Logical offset |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 9 SummaryInfo | `0x100` | 160 | 128 | 0 |
| 2 | 10 Preview | `0x1a0` | 87,072 | 87,040 | 0 |
| 3 | 11 AppInfo | `0x155c0` | 800 | 768 | 0 |
| 4 | 12 AppInfoHistory | `0x158e0` | 1,440 | 1,408 | 0 |
| 5 | 13 FileDepList | `0x15e80` | 800 | 768 | 0 |
| 6 | 8 RevHistory | `0x161a0` | 192 | 135 | 0 |
| 7 | 7 AcDbObjects | `0x16260` | 17,184 | 17,145 | 0 |
| 8 | 7 AcDbObjects | `0x1a580` | 11,136 | 11,080 | `0x7400` |
| 9 | 7 AcDbObjects | `0x1d100` | 4,416 | 4,380 | `0xe800` |
| 10 | 7 AcDbObjects | `0x1e240` | 2,304 | 2,246 | `0x15c00` |
| 11 | 7 AcDbObjects | `0x1eb40` | 3,424 | 3,378 | `0x1d000` |
| 12 | 7 AcDbObjects | `0x1f8a0` | 4,480 | 4,448 | `0x24400` |
| 13 | 7 AcDbObjects | `0x20a20` | 3,552 | 3,490 | `0x2b800` |
| 14 | 7 AcDbObjects | `0x21800` | 1,760 | 1,711 | `0x32c00` |
| 15 | 6 ObjFreeSpace | `0x21ee0` | 224 | 169 | 0 |
| 16 | 5 Template | `0x21fc0` | 192 | 129 | 0 |
| 17 | 4 Handles | `0x22080` | 1,952 | 1,907 | 0 |
| 18 | 3 Classes | `0x22820` | 4,704 | 4,656 | 0 |
| 19 | 2 AuxHeader | `0x23a80` | 256 | 205 | 0 |
| 20 | 1 Header | `0x23b80` | 992 | 946 | 0 |
| 23 | Section Info | `0x23f60` | 1,024 | 970 compressed | n/a |
| 24 | Section Map | `0x24360` | 1,600 declared | 170 compressed | n/a |

IDs 21 and 22 are intentionally absent; there are no negative gap records. The physical order is
therefore SummaryInfo, Preview, AppInfo, AppInfoHistory, FileDepList, RevHistory, eight object
pages, ObjFreeSpace, Template, Handles, Classes, AuxHeader, Header, Section Info, Section Map,
second header. It is neither numeric-ID order nor a global reverse sort. The first five fixed
property sections are stored pages; the remaining ordinary named sections are algorithm-2 pages.

Every ordinary page allocation is a multiple of `0x20`. Stored pages use a payload capacity of
`allocation - 32`, while their descriptor retains the smaller semantic section size. Compressed
pages split logical content at `0x7400`; their allocation is
`align32(32 + compressed_payload_length)`. The data-page header's fourth word is the page
allocation size in this fixture, not the semantic decoded byte count. The decompressed offset is
the fifth word and increases by `0x7400` for object pages.

Unused bytes after a compressed data payload are deterministic LCG filler, not zero padding. Each
page takes the required prefix of
`2923be84e16cd6ae529049f1f1bbe9ebb3a6db3c870c3e99...`; observed suffix lengths range from 4
to 31 bytes. This is derivable serializer state and must be regenerated. The Section Info page's
14-byte final alignment fill is the same sequence starting at its second byte,
`23be84e16cd6ae529049f1f1bbe9`; that one-byte phase difference needs an explicit system-page
policy rather than a stored layout flag.

### Section Info descriptors

The Section Info page at `0x23f60` has type `0x4163003b`, decoded size `1,684`, compressed size
`970`, compression `2`, and checksum `0xfdb4e5e4`. Its header is
`(num_desc=14, compressed=2, max_size=0x7400, encrypted=0, num_desc2=14)`. Descriptor order is
the section-ID order below, beginning with the empty reserved descriptor:

| ID / name | Semantic size | Pages | Max allocation | Compression | Encryption |
| --- | ---: | ---: | ---: | ---: | ---: |
| 0 / empty | 0 | 0 | `0x7400` | 2 | 0 |
| 13 / FileDepList | 726 | 1 | 768 | 1 | 2 |
| 12 / AppInfoHistory | 1,390 | 1 | 1,408 | 1 | 0 |
| 11 / AppInfo | 672 | 1 | 768 | 1 | 0 |
| 10 / Preview | 86,191 | 1 | 87,040 | 1 | 0 |
| 9 / SummaryInfo | 76 | 1 | 128 | 1 | 0 |
| 8 / RevHistory | 16 | 1 | `0x7400` | 2 | 0 |
| 7 / AcDbObjects | 213,182 | 8 | `0x7400` | 2 | 0 |
| 6 / ObjFreeSpace | 89 | 1 | `0x7400` | 2 | 0 |
| 5 / Template | 6 | 1 | `0x7400` | 2 | 0 |
| 4 / Handles | 2,085 | 1 | `0x7400` | 2 | 0 |
| 3 / Classes | 8,194 | 1 | `0x7400` | 2 | 0 |
| 2 / AuxHeader | 123 | 1 | `0x7400` | 2 | 0 |
| 1 / Header | 896 | 1 | `0x7400` | 2 | 0 |

Each page record is `(page number, compressed/stored data size, logical decompressed offset)`, not
the outer allocation. Consequently section-info encoding must be delayed until object/handle
materialization and compression have fixed every page's data length.

### Compression, checksum, trailer, and second-header policy

Compression flag `1` means stored bytes; flag `2` means the R2004 D2 LZ77 variant. Every system
page also uses D2. The live writer's literal-only stream is semantically decodable but cannot
reproduce the fixture's compressed payload choices. Exact equality requires a deterministic match
finder and tie-breaking policy proven page by page against all 15 D2 payloads; compressed length
alone is not sufficient.

For an ordinary data page at file address `A`, construct these clear little-endian words, then XOR
each with `0x4164536b ^ A`:

1. `0x4163043b`
2. named section ID
3. compressed/stored payload length
4. outer page allocation
5. logical decompressed offset
6. zero
7. page-header checksum
8. data checksum

The checksum is the DWG Adler-like checksum with modulus `0xfff1` and chunks of `0x15b0` bytes.
`data_checksum = checksum(0, payload)`. With word 7 zero and word 8 set,
`header_checksum = checksum(data_checksum, clear_32_byte_header)`. This formula was verified for
pages 1, 2, 3, 4, 5, 6, 7, and 17; for example page 17 has data checksum `0xf7a669c8` and header
checksum `0x6bc76e9e`. The live writer already has this seeded order; its word-4 meaning and filler
are the remaining header-stage defects.

For a system page, checksum the 20-byte header with its checksum field treated as zero, then feed
the compressed payload using the prior result as seed. This reproduces both fixture checksums
exactly. After each system payload, the fixture writes a 20-byte second system header
`(same type, 0, 0, compression=2, checksum=0)`. Section Info then adds its 14-byte LCG alignment
fill. Section Map's second system header occupies `0x2441e..0x24431`, and the encrypted header copy
starts immediately at the declared `secondheader_address = 0x24432`.

Bytes `0x24432..EOF` are exactly equal to the primary encrypted bytes `0x80..0xeb`; the second
header is not plaintext and has no separate encryption pass. The file ends after those 108 bytes.
The page map nevertheless declares a 1,600-byte allocation, so its cumulative allocated end is
`0x249a0`, matching `last_section_address + 0x100` but extending beyond physical EOF. A writer must
therefore distinguish the logical allocation cursor used by the page directory/header from the
physical EOF cursor used after the second-header copy.

### Handle-map serialization gate

The fixture's `AcDb:Handles` logical section is 2,085 bytes, compressed to 1,907 bytes in page 17.
Its semantic writer must be downstream of the exact 652-frame object writer:

1. emit objects in ascending logical-handle order and record each start offset within the decoded
   AcDbObjects section;
2. emit unsigned handle deltas as UMC and signed object-address deltas as MC;
3. split blocks at the producer's approximately 2,030-byte payload threshold, prefix each with a
   big-endian `RS`, append the seed-`0xC0C1` CRC in big-endian order, and reset the block-local
   address base (and the producer's handle base where prescribed);
4. terminate with the size-2 empty block and its CRC;
5. D2-compress the fully rebuilt map and only then write descriptor/page/checksum fields.

There is a concrete live blocker before this stage can be accepted. The current D2 decoder yields
2,085 bytes whose first block begins `07f1`, but the current `decode_r2004_handle_map` treats the
size as a fixed `block_start + block_size` data endpoint and then skips two more bytes. At that
boundary it lands on `d544` as the next alleged size, which is impossible for a <=2,040-byte
block. The decoded tail contains a plausible short block/CRC/terminator pattern
`...002cd544...56f0000201d00000`, demonstrating that block-size/CRC boundary semantics and/or the
D2 tail must be corrected together. Do not tune the writer to the current parser. Add a fixture
probe that proves all 652 handle/address pairs, exact block boundaries, every CRC, and the final
size-2 block before enabling handle-map export.

### Current writer comparison and first-difference roadmap

| Gate | Fixture policy | Current live writer divergence | Required byte-level assertion |
| --- | --- | --- | --- |
| 0. Logical inputs | 14 named descriptors, 652 complete typed frames | Eight sections only; Header and Handles empty; object writer is incomplete | Every named logical section reproduces its decoded semantic payload before container work |
| 1. Preamble | Derived versions and four payload/header addresses | Writes only magic, maintenance, code page, and `0x80`; first diff at `0x0b` | `output[0..0x80] == fixture[0..0x80]` |
| 2. Primary header | Exact 108-byte record, CRC32, encrypted signature/padding | Core encryption exists, but section counts/addresses are downstream-wrong and 20-byte tail is synthesized from encrypted zero bytes | Decrypted fields/CRC equal; ciphertext and `0xec..0xff` equal fixture |
| 3. Section identity/order | IDs 13..1 in descriptor order; specific mixed physical stream order | IDs 1..8 are reassigned and globally reverse-sorted | Descriptor names/IDs and stream page-number sequence equal fixture |
| 4. Objects + Handles | Eight `0x7400` logical object chunks; exact rebuilt 2,085-byte map | Legacy drawing bytes; empty Handles section | Exact decoded AcDbObjects and Handles equality, including 652 frames and handle-map CRCs |
| 5. Compression | Stored fixed pages; D2 for all other pages/system pages | Literal-only D2 for every ordinary section | Every compressed/stored payload equals fixture, not merely decoded content |
| 6. Data pages | Word 4 is allocation; seeded checksums; LCG fill | Word 4 receives `decompressed_size`; allocation fill is zero | Compare clear headers, ciphertext, payload, and filler page by page |
| 7. Section Info | 14 descriptors and exact per-page lengths/offsets | Uses `content.len()` as max size, compression=2 universally, and current eight-section inventory | Decoded 1,684-byte body, compressed 970-byte payload, checksum and 1,024-byte allocation equal |
| 8. Page Map | 22 entries including self; IDs 21/22 absent; declared final allocation 1,600 | Omits required inventory/self policy and derives smaller topology | Decoded 176-byte body, compressed 170-byte payload and checksum equal |
| 9. System trailers/fill | Repeated 20-byte system header and LCG fill | Emits type plus 16 zero bytes and zero alignment | Exact bytes from each system payload end to next page |
| 10. Second header/EOF | 20-byte map trailer, repeated encrypted 108-byte primary header, physical EOF `0x2449e` | Appends the decrypted/plain header without the 20-byte trailer-address relationship | Second copy equals primary ciphertext; output length and SHA-256 equal fixture |
| 11. Lifecycle | Original fixture is baseline through every route | Saved exact-native run fails immediately | Native, DSL, pack, diff/apply/inverse/absorb, mutation/inverse, analyzer and composer exports all equal the original bytes |

The implementation order must follow these gates. In particular, section-info/page-map/header
addresses cannot be stabilized before object/handle payloads and D2 compression are exact, while
the preamble and checksum/filler unit assertions can be implemented independently. No page,
section, offset, compression, filler, ciphertext, or source byte is valid snapshot state; every
byte above is a deterministic serializer product of named logical concepts.

## AC1024 Associative Value Dependency and Variable Resolution (2026-08-14)

This is a read-only class-layout audit. No DWG production file or test was modified or run. The
fixture inventory establishes 23 class-541 `ACDBASSOCVALUEDEPENDENCY` frames and 18 class-545
`ACDBASSOCVARIABLE` frames. The prior matrix called both classes blocked by an “unknown prefix.”
That characterization is incorrect and should not guide the implementation.

### What `HANDLE_UNKNOWN_BITS` actually does

LibreDWG's decoder macro expands to `dwg_decode_unknown_bits(dat, obj)`. That function records the
current bit position, copies every remaining object bit into its debugging `unknown_bits` buffer,
and then restores the original bit position. It does **not** consume a prefix. The typed fields
following the macro are decoded from the same cursor where they would be decoded if the macro were
removed. Its encoder counterpart merely replays that captured shadow state.

Consequences for the strict logical-only implementation:

1. neither class has a mysterious byte/bit prefix to preserve or skip;
2. the named inherited class state begins immediately after common `AcDbObject` data;
3. `HANDLE_UNKNOWN_BITS` must not be ported, modeled, serialized, or used as a fallback;
4. acceptance is exact bounded consumption of main, R2010 string, and handle streams; any residual
   bits are a missing named field, not permission to retain raw data.

Primary layout evidence is LibreDWG
[`dwg2.spec`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/dwg2.spec) and its
[`spec.h`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/spec.h) macro definition.
Semantic names are cross-checked against Autodesk's
[`AssocValueDependency`](https://help.autodesk.com/cloudhelp/2022/ENU/OARX-ManagedRefGuide/files/OARX-ManagedRefGuide-Autodesk_AutoCAD_DatabaseServices_AssocValueDependency.html),
[`AssocVariable`](https://help.autodesk.com/cloudhelp/2018/ENU/OARX-RefGuide/files/OREF-AcDbAssocVariable.html),
[`AssocDependency`](https://help.autodesk.com/cloudhelp/2018/ENU/OARX-RefGuide/files/OREF-AcDbAssocDependency.html),
and [`AssocAction`](https://help.autodesk.com/cloudhelp/2018/ENU/OARX-RefGuide/files/OREF-AcDbAssocAction.html)
class references. The public API supplies domain meaning; LibreDWG supplies a candidate bit order
that still requires fixture-bounded proof.

### Type 541: `AcDbAssocValueDependency`

LibreDWG's `Dwg_Object_ASSOCVALUEDEPENDENCY` exposes only an embedded
`Dwg_Object_ASSOCDEPENDENCY`, but the bounded fixture probe below proves the source prescription is
incomplete: every frame also carries a typed value-dependency version, `EvalVariant` cached value,
and value-name string. Autodesk defines it as a concrete `AssocDependency` that reads a scalar
through the depended-on object's value-provider protocol. Its `ValueName` is the referenced value
name, empty when the object exposes only one value. These are named standard concepts, not an
opaque prefix.

The typed logical record should therefore be:

| Concept | Wire primitive | Semantic type |
| --- | --- | --- |
| dependency class version | `BS` | bounded version; source candidate is `2`, reject unsupported values |
| status | `BL` | `AssocStatus` enum, never an untyped integer in the public model |
| read dependency | `B` | boolean |
| write dependency | `B` | boolean |
| attached to object | `B` | boolean |
| delegates to owning action | `B` | boolean |
| order | signed `BL` | signed ordering key |
| dependent-on object | `H`, code 3 | logical object reference |
| has value name | `B` | presence derived from `value_name` |
| value name | R2010 `T` string | optional referenced-value name |
| previous/read dependency link | `H`, code 4 | nullable logical dependency reference; verify precise role from graph links |
| next/node dependency link | `H`, code 3 | nullable logical dependency reference; verify precise role from graph links |
| dependency body | `H`, code 4 | nullable owned dependency-body reference |
| dependency body ID | signed `BL` | logical body identifier |

The two list-link labels above remain intentionally role-qualified candidates: source field names
`readdep` and `node` are not sufficient domain proof. Resolve them by comparing the referenced
objects with Autodesk's `PrevDependencyOnObject`, `NextDependencyOnObject`, and `OwningAction`
relationships across all 23 fixture instances. Handle code/order is known; final public naming is
gated on the graph relation.

Autodesk exposes `DependentOnObjectValue` and inherited `HasCachedValue`. The class source does not
declare a separate derived cached-value field. Therefore the first implementation must **not**
invent or persist one. If bounded consumption finds residual typed bits after the dependency core,
probe them as `(has_cached_value, typed result-buffer value)` against the public API behavior; do
not call the residue `unknown` or treat it as a raw suffix.

All 23 fixture frames share this same class number and have no further derived-class variant in the
class list. The class-wide comparison is therefore one schema with per-instance values and graph
references, not 23 unknown layouts. The implementation gate is 23/23 exact stream exhaustion and
23/23 symmetric frame reproduction.

### Type 545: `AcDbAssocVariable`

Autodesk defines `AcDbAssocVariable` as an `AcDbAssocAction` that stores a scalar evaluated value,
an optional expression, and the variables referenced by that expression through owned value
dependencies. LibreDWG's candidate order covers the complete public semantic surface. Rename its
placeholder fields as follows:

| Source candidate | Typed concept | Wire primitive / policy |
| --- | --- | --- |
| inherited `class_version` | action class version | `BS`; AC1024/R2010 should be version 1 |
| `geometry_status` | action evaluation status | `BL` -> typed action/geometry status enum |
| `owningnetwork` | owning associative network | soft-pointer handle, code 4 |
| `actionbody` | owned action body | hard-owner handle, code 3 |
| `action_index` | action index | `BL` |
| `max_assoc_dep_index` | maximum dependency index | `BL` |
| `num_deps` + entries | ordered dependencies | `BL`, then `(is_owned B, dependency H)`; code 3 when owned, code 4 otherwise |
| `av_class_version` | variable class version | `BL`; source candidate says 2 |
| `name` | variable name | R2010 `T` string |
| `t58` | expression | R2010 `T`; empty means constant |
| `evaluator` | evaluator ID | R2010 `T`; empty selects the default evaluator |
| `desc` | description | R2010 `T` |
| `value` | evaluated cached scalar | typed `EvalVariant` |
| `has_t78` | is mergeable | `B` |
| `t78` | mergeable variable name | R2010 `T`; serialize empty when not mergeable if fixture proves the field remains present |
| `b290` | must merge | `B` |

The last three names follow Autodesk's exact public methods `isMergeable()`,
`mergeableVariableName()`, and `mustMerge()`. They are named variable semantics, not flags to hide
behind a raw prefix. The fixture's decompressed object pages visibly contain architectural
variable/expression strings such as `bldDEPTH=50'-0"`, `bldWALL=6"`, and `Wall2=`. Those strings
are supporting semantic evidence only; the current object/handle-page boundary defect prevents
using an unbounded page-wide string scan as acceptance evidence.

For AC1024, action class version 1 omits the version>1 extension in `AcDbAssocAction_fields`
(`num_owned_params`, owned parameter handles, and value-parameter array). The reader must branch on
the decoded version rather than assume absence globally. Encountering version>1 in this AC1024
subset is an explicit unsupported-version error until those standard action concepts are modeled.

`EvalVariant` is a typed discriminated union whose stored code is a DXF/resbuf code. The candidate
wire supports real `BD`, signed/unsigned 32-bit `BL`, 16-bit `BS`, 8-bit `RC`, R2010 string `T`, and
handle `H` values. The logical model must expose variants such as Real, Integer32, Integer16,
Integer8, Text, and ObjectReference. Unsupported binary, object-ID, point, int64, or boolean codes
must reject atomically unless their named standard encoding is implemented. The union code is
derived from the variant on write.

All 18 fixture frames resolve to the same class-545 definition. Their expected variability is in
names, expressions, evaluator IDs, scalar variants, dependency lists, and mergeability values—not
in the structural layout. Acceptance is 18/18 exact bounded consumption and 18/18 symmetric frame
reproduction.

### Required bounded-consumption probes

The current live handle-map/D2 boundary issue must be fixed first so each target object has a
trusted object address and handle. Then instrument the existing AC1024 test, without adding a test
file, with these non-persisted diagnostics:

1. For every target frame, record object handle, class number, payload bits, string-stream bits,
   handle-stream bits, and the start/end bit position of each inherited/derived region.
2. Decode common `AcDbObject` state into a bounded main reader and common owner/reactor/extension
   references into the bounded handle reader. Assert the object handle equals the handle-map key.
3. For type 541, decode `AssocDependencyCore`; assert main reader lands exactly at its prescribed
   data/string boundary, string reader consumes exactly zero or one `value_name`, and handle reader
   consumes exactly four class references after common references.
4. Resolve all four type-541 class references against the 652-object graph. Prove which links are
   dependent object, previous dependency, next dependency, and dependency body before finalizing
   field names. Assert body ownership and list reciprocity where non-null.
5. For type 545, decode `AssocActionCore` first. On action version 1, assert the version>1 extension
   consumes zero bits. Then decode `AssocVariableState`; assert exactly four primary strings plus
   the mergeable-name string are consumed according to the fixture's presence policy.
6. Assert each variable's dependency count equals its ordered dependency-reference count and every
   owned bit agrees with handle code/graph ownership. For expression variables, verify referenced
   symbols are represented by owned/read value dependencies; constants may have an empty list.
7. Decode `EvalVariant` by code and assert exactly one typed payload is consumed. Re-encode the
   variant immediately and compare its bit slice before proceeding.
8. Assert the remaining main/string/handle bits are only the standard terminal padding required by
   the R2010 frame, then verify the object CRC over the original MS/MC/payload span.
9. Re-encode every target frame from typed values and compare main, string, handle, padding, and CRC
   slices independently. Only after 23/23 and 18/18 pass may the rebuilt frames enter AcDbObjects
   and the regenerated handle map.
10. Run the original-byte lifecycle only after those per-frame gates: native, DSL, pack,
    diff/apply/inverse/absorb, mutation/inverse, analyzer, composer, and final AC1024 export must all
    equal the original fixture.

### Revised implementation decision

Type 541 and type 545 are no longer blocked on an unnamed prefix. They are blocked only on
fixture-bounded verification of fully named standard concepts and the upstream object-address
probe. The old raw-prefix rows in the matrix should be read as superseded by this section:

| Type | Typed implementation status | Remaining proof |
| --- | --- | --- |
| 541 `ACDBASSOCVALUEDEPENDENCY` | Exact-frame checklist: dependency core plus typed cached `EvalVariant` and value name; no raw prefix | Production codec and reciprocal chain-role naming |
| 545 `ACDBASSOCVARIABLE` | Exact-frame checklist: action core, variable state, typed `EvalVariant` and expression value-dependency binding | Production codec and complete lifecycle proof |

No `unknown_bits`, raw prefix/suffix, object-frame bytes, or debug replay field is acceptable in
Rust snapshots, DSL/pack, diffs, mutations, or language facets.

## AC1024 D2 Exact Compression Policy (2026-08-14)

This is a read-only writer audit. No DWG production file or production test was modified or run.
The ticket-only probe is
[`🧪️dwg-d2-policy-probe.py`](./🧪️dwg-d2-policy-probe.py), with captured evidence in
[`🧪️dwg-d2-policy-probe.log`](./🧪️dwg-d2-policy-probe.log). It parses the fixture's native D2
tokens, decompresses each page, independently recompresses the decoded page from logical bytes,
and compares the resulting compressed bytes directly. The fixture SHA-256 is
`52d14a7bdb946099d3cf16fd276d19bd8924348fd02b2ddd0003cd4f6b34cce7`.

The matching state machine is the reverse-engineered AC18 compressor published as ACadSharp
[`DwgLZ77AC18Compressor.cs`](https://raw.githubusercontent.com/DomCR/ACadSharp/master/src/ACadSharp/IO/DWG/DwgStreamWriters/DwgLZ77AC18Compressor.cs),
source SHA-256 `b10d94245d1aab9545897b0c5f258daa18a0b5bb2eb2d0a25ad70ef489875610`.
This is stronger than a compatible-codec result: the algorithm reproduces every fixture opcode,
length extension, distance, literal byte, terminator, and trailing byte for all 15 ordinary D2
pages and both D2 system pages.

### Exact fixture result

The task's 15 compressed payloads are the ordinary data pages 6 through 20. Section Info and
Section Map add two compressed system payloads, so the full container proof is 17/17:

| Page | Compressed | Compressor input | Match tokens S/N/F | Maximum length / distance | Result |
| --- | ---: | ---: | ---: | ---: | --- |
| 6 RevHistory | 135 | 29,696 | 1/1/0 | 29,683 / 5 | exact |
| 7 Objects 0 | 17,145 | 29,696 | 1,304/780/36 | 187 / 27,858 | exact |
| 8 Objects 1 | 11,080 | 29,696 | 664/546/20 | 1,515 / 26,736 | exact |
| 9 Objects 2 | 4,380 | 29,696 | 327/469/0 | 316 / 13,627 | exact |
| 10 Objects 3 | 2,246 | 29,696 | 178/517/4 | 196 / 22,276 | exact |
| 11 Objects 4 | 3,378 | 29,696 | 374/467/0 | 7,270 / 14,438 | exact |
| 12 Objects 5 | 4,448 | 29,696 | 683/824/12 | 115 / 23,913 | exact |
| 13 Objects 6 | 3,490 | 29,696 | 508/619/0 | 5,722 / 15,631 | exact |
| 14 Objects 7 | 1,711 | 29,696 | 181/89/0 | 24,373 / 5,283 | exact |
| 15 ObjFreeSpace | 169 | 29,696 | 8/1/0 | 29,611 / 49 | exact |
| 16 Template | 129 | 29,696 | 0/1/0 | 29,691 / 1 | exact |
| 17 Handles | 1,907 | 29,696 | 78/6/0 | 27,610 / 1,463 | exact |
| 18 Classes | 4,656 | 29,696 | 351/355/0 | 21,509 / 7,183 | exact |
| 19 AuxHeader | 205 | 29,696 | 8/1/0 | 29,584 / 96 | exact |
| 20 Header | 946 | 29,696 | 43/2/0 | 28,794 / 395 | exact |
| 23 Section Info | 970 | 1,684 | 195/9/0 | 28 / 1,273 | exact |
| 24 Section Map | 170 | 176 | 20/0/0 | 4 / 80 | exact |

`S/N/F` means short, long-near, and long-far token families. In every row, rebuilt length equals
fixture length and the first differing offset is absent. Each parser stops after the `0x11`
terminator with exactly the two final `0x00` bytes left; the writer must emit the fixed three-byte
`11 00 00` end sequence, not only `11`.

### Page and system-page input policy

Ordinary compressed pages do not compress only the semantic byte count. Materialize a fresh
`max_decomp_size = 0x7400` buffer for every ordinary page, copy that page's logical section slice
at offset zero, fill the remainder with zero, and compress all 29,696 bytes. This is proven by all
15 pages decoding to exactly `0x7400`: RevHistory has 16 semantic bytes but a 29,683-byte terminal
zero match; Template has six semantic bytes and a 29,691-byte distance-one match; object page 14
has 5,310 semantic bytes and a 24,373-byte terminal zero match. The descriptor's semantic size
still truncates the concatenated decoded pages after decompression.

System pages are different: compress exactly the constructed body, with no `0x7400` padding.
Section Info takes 1,684 input bytes and Section Map takes 176. Start every ordinary and system
page with a newly initialized match table; no history crosses a page boundary. The fact that all
17 independently reset probe invocations reproduce the fixture is the boundary proof. Stored
flag-1 property pages remain outside D2 entirely.

### Exact match finder and tie-breaking

For an input page `source[0..end]`:

1. Initialize 32,768 signed table entries to `-1`. Set the literal-run start to zero and the scan
   position to four, guaranteeing at least four initial literal bytes.
2. Probe while `position < end - 0x13`; the last 19 bytes cannot start a new match and remain in
   the final literal run unless already covered by a greedy match.
3. Hash four source bytes in this exact order with ordinary integer shifts/XORs:
   `v = source[p+3] << 6; v ^= source[p+2]; v = (v << 5) ^ source[p+1];`
   `v = (v << 5) ^ source[p]; index = (v + (v >> 5)) & 0x7fff`.
4. The primary candidate is the table's last position for `index`, so collision tie-breaking is
   recency, not smallest distance after a global longest-match search. Reject candidates outside
   this page and distances greater than `0xbfff`.
5. For a candidate farther than `0x400`, require byte four to match. If it does not, probe exactly
   one alternate bucket `(index & 0x7ff) ^ 0x401f`. If that alternate also fails its page,
   distance, or fourth-byte gate, store the current position in the alternate bucket and emit no
   match. Do not search any other chain.
6. Require the first three bytes to match, then extend greedily byte by byte to `end`. Comparing
   against the complete logical input permits overlapping copies. Accept every length at least
   three; there is no lazy lookahead and no competing-candidate longest-match choice.
7. Store the current position in whichever bucket was actually probed. After accepting a match,
   jump the scan position by the full match length and do not insert skipped interior positions.
8. Defer each accepted match until the next match or end so its offset low bits can encode the
   following literal count. Emit the preceding literal run, then carry the new match as pending.

This table-update detail is essential. A conventional chained hash, global longest-match search,
lazy matching, interior-position insertion, or a different equal-length tie break produces a
valid D2 stream but not the fixture stream.

### Exact length, distance, and literal encoding

Let `L` be match length, `D` backward distance, and `R` the immediately following literal count.
The offset payload always uses `D - 1` for short/near distances, while the far family uses
`D - 0x4000`:

- **Short:** choose only when `L < 15` and `D <= 0x400`. Emit first byte
  `((L + 1) << 4) | (((D - 1) & 3) << 2)`, then `(D - 1) >> 2`.
- **Long-near:** choose when the short condition fails and `D <= 0x4000`. Encode length with base
  opcode `0x20`, immediate ceiling 33; then emit `((D - 1) & 0xff) << 2` and
  `(D - 1) >> 6`.
- **Long-far:** choose when `D > 0x4000` (bounded by `0xbfff`). Let `d = D - 0x4000` and encode
  length with base `0x10 | ((d >> 11) & 8)`, immediate ceiling 9; then emit
  `(d & 0xff) << 2` and `d >> 6`.

For long-near, if `L <= 33`, OR `L - 2` into the base opcode; otherwise emit the base and encode
`L - 33` with the extension rule. For long-far, use `L - 2` through length 9, otherwise extend
`L - 9`. An extension emits zero for each full 255 removed, followed by the positive remainder.

If `R < 4`, OR `R` into the low two bits of the first distance byte and write those literal bytes
without a separate literal opcode. Otherwise leave those bits zero, emit a literal-length opcode,
then the literals. Literal lengths 4 through 18 use opcode `length - 3`; longer runs use opcode
zero followed by the same 255-extension scheme for `length - 18`. The initial and final literal
runs use this same policy. A decoder should also honor the standard initial-opcode form
`opcode > 0x11`, which copies `opcode - 0x11` initial literals, even though this fixture's matching
writer does not select that alternate spelling.

### Production acceptance gate

Replace the literal-only encoder with the state machine above behind the existing D2 interface;
do not expose or persist match tables, tokens, page padding, compressed bytes, or producer flags.
Extend the existing AC1024 tests, not a new test file, to assert:

1. the 15 ordinary logical page slices materialize to zero-padded `0x7400` inputs;
2. Section Info and Section Map use exact unpadded bodies and independent compressor resets;
3. all 17 compressed payloads equal their fixture spans byte for byte, including `11 00 00`;
4. decompression consumes the terminator, accepts exactly two trailer zeros, produces the expected
   materialization size, and the descriptor truncation recovers the logical section size;
5. every resulting compressed length feeds the existing allocation, checksum, Section Info, page
   map, primary-header, and second-header derivations before full original-byte lifecycle export.

The earlier statement that fixture-identical D2 choices remained unknown is superseded by this
section. The exact deterministic policy is now proven; remaining outer-writer differences are
upstream logical section/object materialization and the already inventoried header/map/filler
stages, not D2 ambiguity.

## AC1024 Remaining Named Logical Sections (2026-08-14)

This is a read-only schema/writer audit. No DWG production file or production test was modified or
run. Ticket evidence is in
[`🧪️dwg-named-sections-probe.py`](./🧪️dwg-named-sections-probe.py) and
[`🧪️dwg-named-sections-probe.log`](./🧪️dwg-named-sections-probe.log). The authoritative field
orders are the Open Design Specification chapters 9, 14, 18, 21, and 27 and LibreDWG
[`header_variables.spec`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/header_variables.spec),
[`auxheader.spec`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/auxheader.spec),
[`revhistory.spec`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/revhistory.spec),
and [`objfreespace.spec`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/objfreespace.spec).

The current snapshot's four-field `DwgHeaderVariables` and absent five other section models are not
an exportable AC1024 document model. The replacements below contain named standard concepts only.
Sentinels, bit sizes, CRCs, fixed reserved values, D2 input padding, section offsets, image record
offsets, DIB headers, string terminators, and XML quoting are deterministic serializer products and
must never appear in snapshot/diff/mutation/DSL/pack/facets.

### AcDb:Header

The fixture section is exactly 896 semantic bytes. Its framing is:

| Offset/length | Fixture | Meaning / writer rule |
| --- | --- | --- |
| `0..16` | standard `CF 7B ... 33 5F` | Derived Header start sentinel |
| `16..20` | `858` | Derived section-data length |
| `20..24` | `6,136` | R2007+ combined bit-stream boundary value |
| `24..878` | typed bit streams | Main values plus separated R2010 string and handle streams |
| `878..880` | bytes `84 d0`, value `0xd084` | CRC16 seed `0xc0c1` over bytes `16..878`; independently reproduced |
| `880..896` | standard `30 84 ... cc a0` | Derived Header end sentinel |

Maintenance version 2 means the conditional R2010 high 32-bit section-size word is absent. The
logical `DwgHeaderVariables` must mirror the AC1024 branch of `header_variables.spec` in this exact
semantic order; grouping is for the schema API only and must not reorder serialization:

| Logical group | Ordered AC1024 concepts |
| --- | --- |
| Unit conversion | four named conversion ratios and unit names; internal header format revision/generation counters derived from AC1024, not persisted as unknown longs |
| Drawing modes | `DIMASO`, `DIMSHO`, `PLINEGEN`, `ORTHOMODE`, `REGENMODE`, `FILLMODE`, `QTEXTMODE`, `PSLTSCALE`, `LIMCHECK`, `USRTIMER`, `SKPOLY`, `ANGDIR`, `SPLFRAME`, `MIRRTEXT`, `WORLDVIEW`, `TILEMODE`, `PLIMCHECK`, `VISRETAIN`, `DISPSILH`, `PELLIPSE`, `PROXYGRAPHICS` |
| Integer/display settings | `TREEDEPTH`, `LUNITS`, `LUPREC`, `AUNITS`, `AUPREC`, `ATTMODE`, `PDMODE`, five `USERI`, `SPLINESEGS`, `SURFU`, `SURFV`, `SURFTYPE`, `SURFTAB1`, `SURFTAB2`, `SPLINETYPE`, `SHADEDGE`, `SHADEDIF`, `UNITMODE`, `MAXACTVP`, `ISOLINES`, `CMLJUST`, `TEXTQLTY` |
| Scalar drawing settings | `LTSCALE`, `TEXTSIZE`, `TRACEWID`, `SKETCHINC`, `FILLETRAD`, `THICKNESS`, `ANGBASE`, `PDSIZE`, `PLINEWID`, five `USERR`, four chamfers, `FACETRES`, `CMLSCALE`, `CELTSCALE` |
| Time/current state | `TDUCREATE`, `TDUUPDATE`, three derived AC1024 generation counters, `TDINDWG`, `TDUSRTIMER`, `CECOLOR`, `HANDSEED`, `CLAYER`, `TEXTSTYLE`, `CELTYPE`, `CMATERIAL`, `DIMSTYLE`, `CMLSTYLE`, `PSVPSCALE` |
| Paper-space geometry | `PINSBASE`, `PEXTMIN`, `PEXTMAX`, `PLIMMIN`, `PLIMMAX`, `PELEVATION`, `PUCSORG`, `PUCSXDIR`, `PUCSYDIR`, `PUCSNAME`, `PUCSORTHOREF`, `PUCSORTHOVIEW`, `PUCSBASE`, six paper-space orthographic UCS origins |
| Model-space geometry | `INSBASE`, `EXTMIN`, `EXTMAX`, `LIMMIN`, `LIMMAX`, `ELEVATION`, `UCSORG`, `UCSXDIR`, `UCSYDIR`, `UCSNAME`, `UCSORTHOREF`, `UCSORTHOVIEW`, `UCSBASE`, six model-space orthographic UCS origins |
| Dimensions | every ordered AC1024 `DIM*` value from `DIMSCALE` through `DIMLWE`, including typed colors, decimal/unit/fit flags, text/leader/block/linetype handles, alternate-unit strings and the R2010 `DIMALTMZF`, `DIMMZF`, `DIMALTMZS`, `DIMMZS` concepts |
| Controls/dictionaries | block, layer, style, linetype, view, UCS, viewport, app-id and dimstyle controls; group, mlinestyle and named-object dictionaries; text-stack settings; layout, plot-settings, plot-style-name, material and color dictionaries |
| Drawing policy | packed `FLAGS` exposed as named `CELWEIGHT`, `ENDCAPS`, `JOINSTYLE`, `LWDISPLAY`, `XEDIT`, `EXTNAMES`, `PSTYLEMODE`, `OLESTARTUP`; then `INSUNITS`, `CEPSNTYPE`/optional `CPSNID`, `SORTENTS`, `INDEXCTL`, `HIDETEXT`, `XCLIPFRAME`, `DIMASSOC`, `HALOGAP`, obscured/intersection display settings |
| Required terminal references | paper/model block-records, ByLayer/ByBlock/Continuous linetypes |
| R2007+/R2010 rendering | camera, stepping, lens, solid history, swept/loft, geolocation/timezone, light/frame/real-world-scale, interference visual styles, shadow mode/plane |
| String stream | unit names, `MENU`, `DIMPOST`, `DIMAPOST`, `DIMALTMZS`, `DIMMZS`, `HYPERLINKBASE`, `STYLESHEET`, fingerprint GUID, version GUID, `PROJECTNAME`, in the exact terminal string-stream order |

The encoder must bit-write every named value using its prescribed `B/BS/BL/BLL/BD/CMC/H/T`
code. Text goes only to the R2010 string writer and references only to the handle writer; concatenate
the three streams according to the standard flag/size rules, derive `6,136` from their final bit
positions, pad only the terminal partial byte, derive size and CRC, then add sentinels. A raw header
body, generic numeric slot vector, or imported bit-count field is prohibited. The fixture gate is
decode-to-fields, immediate field-by-field re-encode, all stream cursors exhausted, and exact 896
bytes before D2.

### AcDb:AuxHeader

This 123-byte record is redundant save provenance, not an opaque header. Use a typed
`DwgAuxiliaryHeader` containing target version, two legacy/source version stamps, save counters,
creation/update Julian timestamps, handle seed, educational-plot state, and a named AC1024
compatibility profile. The fixture field order/value inventory is:

| Ordered field | Fixture value | Logical treatment |
| --- | ---: | --- |
| fixed intro / target version / maintenance | `ff 77 01`, `29`, `2` | intro derived; version from document |
| total saves / minus-one marker | `105`, `-1` | total is semantic; marker derived |
| save partitions | `32`, `12`, then generation marker `1` | typed save-statistics counters |
| legacy stamp one / two | `(22,46)`, `(22,46)` | typed source-version provenance, not raw shorts |
| AC1024 compatibility profile | shorts `4,1381,261,2600,0,1`; longs `0,0,0,16908544,65538` | map from the named target/source/application version profile; never expose eleven unnamed slots |
| created / updated | `(2454804,72759955)`, `(2454806,74552875)` | `DwgJulianDate` values; must agree with Header |
| handle seed | `8845` | must agree with Header and exceed every allocated handle |
| terminal save generation / total | `40`, `105` | typed save-statistics values; total agrees with leading total |
| standard terminal reserved values | zero | derived zero, absent from schema |

The ODA canonical constants differ from this older-save provenance profile, so blindly writing its
documented `5,0x893,5,0x893,...` defaults would not reproduce the fixture. Implement an internal
typed compatibility-profile table keyed by target AC1024 stamp, legacy `(22,46)` stamp, and
application build provenance; reject an unrecognized combination rather than retaining or emitting
unknown numeric vectors. Write the table fields little-endian in `auxheader.spec` order and assert
exactly 123 bytes. No sentinel or CRC belongs inside this section.

### AcDb:RevHistory

Model `DwgRevisionHistory { format_major: u32, format_minor: u32, revisions:
Vec<DwgRevisionCode> }`. `DwgRevisionCode` is a typed 32-bit standard history marker, not section
bytes. Encoding is linear little-endian: major, minor, revision count, ordered codes. The fixture is
exactly `(major=0, minor=0, count=1, codes=[0])`, hence 16 semantic bytes. Preserve ordered markers
through lifecycle operations even when their public meaning is not further subdivided; do not call
them a raw payload or synthesize an arbitrary tail. Count must equal vector length and the parser
must consume the section's declared semantic size exactly.

### AcDb:ObjFreeSpace

The AC1024 branch is a fixed 89-byte typed statistics record:

| Offset order | Fixture | Logical/derived concept |
| --- | ---: | --- |
| leading 64-bit zero | `0` | derived reserved constant |
| approximate registered-handle/object count | `679` | derive from the complete logical object/handle graph |
| update Julian date | `(2454806,74552875)` | same `TDUPDATE` as Header and AuxHeader |
| numeric-bound count | `4` | derived standard constant |
| four unsigned 128-bit bounds, low then high halves | `50:0`, `100:0`, `512:0`, `0xffffffff:0` | derived AC1024 allocator constants |

The persisted model therefore needs no free-space byte array and normally no independent copy of
the constants: it is a deterministic projection of the complete object graph and document update
time. The fixture's `679` is also an acceptance check on object materialization; a writer with only
the currently inventoried 652 frames is incomplete and must not silently substitute 652. Encode
the listed little-endian fields, assert 89 bytes, then zero-pad the ordinary D2 input to `0x7400`.

### AcDb:Preview

Preview graphic values are legitimate document content; preview framing is not. Use a discriminated
`DwgPreviewImage` with a fixture variant equivalent to:

`IndexedBitmap { width, height, origin, palette: Vec<Rgba>, pixel_indices: Vec<u8>,
background_palette_index }`.

The fixture contains one 329 by 256, bottom-up, 8-bit indexed bitmap. Its semantic palette has 256
BGRA entries (all reserved/alpha bytes zero), and its logical pixel array has exactly
`329 * 256 = 84,224` indices with SHA-256
`6fcf843df14f3783b010a85458f2dfca5ec264bae11d1e997475e4d1ec957bcd`. Palette index `226` is
the dominant/corner background and is the row-fill index. Those palette colors and logical pixels
are genuine image data and may be stored; the following fixture bytes must instead be regenerated:

| Serializer construct | Fixture | Rule |
| --- | --- | --- |
| start/end sentinels | standard 16-byte pair | fixed AC1024 constants |
| overall size | `86,155` | derive from records/data; total section is 86,191 |
| record table | header `(code=1,start=487,size=80)`, bitmap `(code=2,start=567,size=86,056)` | global starts derive from final Preview page payload address `0x1c0` |
| header record | 80 zero bytes | fixed AC1024 bitmap-header record, never snapshot bytes |
| DIB header | size 40, width 329, height 256, planes 1, depth 8, BI_RGB 0, image bytes 84,992, resolutions 0, colors 256/important 0 | derive from typed bitmap |
| palette layout | 256 four-byte BGRA entries | derive from semantic RGBA palette |
| row layout | stride 332 | derive 4-byte alignment; write three background-index `0xe2` bytes after each 329-byte row |

The DIB header and row padding are image-container encoding, not “genuine preview bytes.” Decode
them into typed dimensions/origin/palette/pixels and validate every row. BMP code 2 and WMF code 3
must be distinct logical variants; unsupported image codes are rejected. Do not persist `RawHeader`,
`RawImage`, record offsets, DIB bytes, row padding, or the whole Preview section.

### AcDb:AppInfoHistory

The 1,390-byte fixture section is a typed application-provenance record set, not a native envelope.
Model:

- two typed 128-bit history identifiers/digests;
- `class_version`, typed list kind `AppInfoDataList`, and ordered typed entries;
- entry digest plus a variant `ApplicationVersion`, `TrustComment`, `SummaryPropertySet`, or
  `ProductInformation`;
- a property set with typed format GUID and ordered `(property id, String | DateTime)` values;
- product information with named application/build/registry/install/locale fields.

The exact fixture projection is:

| Field | Fixture |
| --- | --- |
| history identifiers | `53de381dec4321ca9619e1e2171a2a67`, `3bd97ff73cbbce08a053d8edd28dc5c7` |
| class/list/count | `0`, `AppInfoDataList`, `4` |
| application version | digest `1bd848f3cc0a3e4dbab1cf81f7b450b3`; `18.0.40.0.200` |
| trust comment | digest `b8d0f025a1d79349b2fa9bf9286fa1fd`; Autodesk trusted-DWG comment |
| property set | digest `e0859ff2f94f6810ab9108002b27b3d9`; format GUID `f29f85e0-4ff9-1068-ab91-08002b27b3d9` |
| ordered properties | 8 String `Brian`; 10 DateTime `2008-12-05T20:42:32`; 258 String `AutoCAD 2009`; 259 String `D.40.0.200`; 12 DateTime `2008-12-03T20:12:39` |
| product | digest `e8e09651c5ceb244a8bff6e83b859d44`; name `AutoCAD`; build `D.40.0.200`; registry `18.0`; install `ACAD-8001:409`; locale `1033` |

The 128-bit values are typed section-level integrity/provenance concepts, not arbitrary byte arrays;
facets should represent a validated fixed-width digest/identifier scalar. Property-set and product
XML-like strings must not be persisted. The encoder deterministically renders their canonical
AC1024 templates from typed fields, including property order, date form, braces, attribute order,
outer product quotes and escaped inner quotes. Then write: two identifiers, little-endian class
version, T16 zero-terminated list name, little-endian entry count, and for each entry its digest
followed by canonical T16 text. Every T16 count includes its terminating UTF-16 zero. The resulting
cursor must land exactly at byte 1,390.

### Cross-section invariants and implementation order

1. Header is the authority for document system variables; AuxHeader and ObjFreeSpace repeat only
   deterministic projections. All three update timestamps and handle seeds/counts must agree.
2. Complete objects/handles must be materialized before `HANDSEED` and ObjFreeSpace count; Header
   strings/handles must be finalized before its bit size and CRC.
3. AppInfo and AppInfoHistory share typed application version, trust comment, product information,
   and their entry digests. Parse product/property markup once into named records and use one
   canonical renderer; never keep the imported markup as a fallback.
4. Preview pixels/palette are snapshot state. Preview sentinels, code table, absolute offsets,
   zero header, DIB layout and padding are native serialization only.
5. Encode each logical section to its exact semantic bytes first: Header 896, AuxHeader 123,
   RevHistory 16, ObjFreeSpace 89, Preview 86,191, AppInfoHistory 1,390. Only afterward apply the
   stored-vs-D2 page policy and outer page/header/map derivations.
6. Extend the existing AC1024 lifecycle test with per-section decode/encode equality, mutation plus
   inverse on at least one named field in each section, anti-shadow facet scans, then final original
   fixture equality through DSL, pack, diff/apply/inverse/absorb, mutation/inverse, analyzer and
   composer.

## Live Canonical Writer Gap Checklist (2026-08-14)

This is a read-only audit of the current live AC1024 writer. No production file was edited and Nx
was not started. Locations below refer to the audit-time
`ac1024/.../io/🦀️component.rs`; the logical-model locations refer to
`ac1024/.../schema/📸️snapshot/🦀️component.rs`.

### Fixture page oracle

All 20 ordinary clear headers were decrypted directly from the fixture. Every listed checksum was
then independently recomputed: `data = checksum(0, payload)`; clear header word 6 is zero, word 7
contains data checksum, and `header = checksum(data, clear_header)`. Physical order is table order.

| Page | Section / ID | Address | Allocation | Payload | Offset | Header checksum | Data checksum |
| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | SummaryInfo / 9 | `0x100` | 160 | 128 | `0` | `8c790b1b` | `63990786` |
| 2 | Preview / 10 | `0x1a0` | 87,072 | 87,040 | `0` | `3ec2d57d` | `f187d0bf` |
| 3 | AppInfo / 11 | `0x155c0` | 800 | 768 | `0` | `553686f4` | `ce9e82f9` |
| 4 | AppInfoHistory / 12 | `0x158e0` | 1,440 | 1,408 | `0` | `c601065b` | `2bf00312` |
| 5 | FileDepList / 13 | `0x15e80` | 800 | 768 | `0` | `26bb777f` | `690074c6` |
| 6 | RevHistory / 8 | `0x161a0` | 192 | 135 | `0` | `945804cb` | `3ca700db` |
| 7 | Objects 0 / 7 | `0x16260` | 17,184 | 17,145 | `0` | `5af5984a` | `a8d3935a` |
| 8 | Objects 1 / 7 | `0x1a580` | 11,136 | 11,080 | `0x7400` | `d60a420b` | `fde43cb9` |
| 9 | Objects 2 / 7 | `0x1d100` | 4,416 | 4,380 | `0xe800` | `21f80074` | `78d2fb68` |
| 10 | Objects 3 / 7 | `0x1e240` | 2,304 | 2,246 | `0x15c00` | `19a6d5db` | `c18cd0d0` |
| 11 | Objects 4 / 7 | `0x1eb40` | 3,424 | 3,378 | `0x1d000` | `baf88ddd` | `61df88d7` |
| 12 | Objects 5 / 7 | `0x1f8a0` | 4,480 | 4,448 | `0x24400` | `ad436496` | `4a176151` |
| 13 | Objects 6 / 7 | `0x20a20` | 3,552 | 3,490 | `0x2b800` | `ecbc7d04` | `98467837` |
| 14 | Objects 7 / 7 | `0x21800` | 1,760 | 1,711 | `0x32c00` | `ede7e83a` | `4581e2ef` |
| 15 | ObjFreeSpace / 6 | `0x21ee0` | 224 | 169 | `0` | `20801131` | `43c80cd4` |
| 16 | Template / 5 | `0x21fc0` | 192 | 129 | `0` | `5d8c0428` | `12e90082` |
| 17 | Handles / 4 | `0x22080` | 1,952 | 1,907 | `0` | `6bc76e9e` | `f7a669c8` |
| 18 | Classes / 3 | `0x22820` | 4,704 | 4,656 | `0` | `ea11056c` | `5e6d0302` |
| 19 | AuxHeader / 2 | `0x23a80` | 256 | 205 | `0` | `80a113e2` | `323110de` |
| 20 | Header / 1 | `0x23b80` | 992 | 946 | `0` | `b7a10127` | `fb4cfbad` |

Pages 1-5 are stored payloads, not D2. Their payload lengths are their full fixed capacities and
their logical sections truncate to 76, 86,191, 672, 1,390, and 726 bytes. Pages 6-20 are D2 and
their exact compressed lengths are the table's payload column. Every D2 source is a fresh zero-
padded `0x7400` page. System oracles are:

- page 23 Section Info: address `0x23f60`, decoded 1,684, compressed 970, checksum
  `fdb4e5e4`, physical/allocation span 1,024;
- page 24 Section Map: address `0x24360`, decoded 176, compressed 170, checksum `01d7121f`;
  its directory allocation is 1,600 but its physical header/payload/trailer ends at `0x24432`.

### Implementation gaps by live function

- [ ] **Logical schema first — snapshot lines 566-575 and 686-723.** `DwgHeaderVariables` still
  has only four handles; snapshot has no AuxHeader, RevHistory, ObjFreeSpace, Preview, or
  AppInfoHistory. Add the named models/mirrored facets/diff/mutation codecs from the preceding
  matrix before writer changes. `DwgApplicationInfo` lines 654-662 also retains checksum strings
  and raw product markup; use typed fixed-width digests and typed ProductInformation shared with
  AppInfoHistory.
- [ ] **Exact D2 — `compress_r2004_section`, lines 211-232.** It remains literal-only, compresses
  the caller's short slice, and ends with one `11`. Replace with the proven 0x8000-table AC18
  state machine, per-page reset, exact tie-breaking/length-distance rules, and `11 00 00`.
- [ ] **Ordinary clear-header words/checksums — `write_data_page`, lines 561-578.** Word 3 must be
  outer allocation, not `decompressed_size` (line 570). Keep word 5 zero; write header checksum to
  word 6 and data checksum to word 7. Compute header checksum with word 6 zero, word 7 populated,
  and data checksum as seed. Lines 572-574 currently shift both checksums one word early.
- [ ] **Ordinary allocation fill — `write_data_page`, line 577.** Replace zero resize with the
  deterministic LCG filler. Required suffix lengths for D2 pages 6-20 are respectively
  `25,7,24,4,26,14,0,30,17,23,31,13,16,19,14` bytes; compare every suffix, not only allocation.
- [ ] **System-page trailers/fill — `write_system_page`, lines 581-597.** The repeated 20-byte
  trailer is `(same type,0,0,compression=2,checksum=0)`; line 595 currently writes compression 0.
  Section Info then uses 14 LCG bytes with its documented one-byte phase. Section Map must not be
  `align32` padded: its trailer ends exactly at `0x24432`. Split the two system policies rather
  than using unconditional line 596.
- [ ] **All 14 descriptors — `CanonicalR2004Section` and `encode_r2004_canonical`, lines 600-662.**
  Materialize IDs exactly `1 Header, 2 AuxHeader, 3 Classes, 4 Handles, 5 Template,
  6 ObjFreeSpace, 7 Objects, 8 RevHistory, 9 SummaryInfo, 10 Preview, 11 AppInfo,
  12 AppInfoHistory, 13 FileDepList`, plus empty descriptor 0. Current IDs are reassigned, six
  sections are absent, Header/Handles are empty, and `dwg_to_bytes` at line 646 is the legacy
  drawing envelope rather than the complete 652/ultimately-679-record object-section writer.
- [ ] **Physical page order — `sections.sort_by_key` and page loop, lines 662-673.** Do not infer
  fixture order by sorting a partial/reassigned section set. Emit pages 1-20 in the oracle order:
  Summary, Preview, AppInfo, AppInfoHistory, FileDepList, RevHistory, eight Objects,
  ObjFreeSpace, Template, Handles, Classes, AuxHeader, Header. Preserve logical object offsets
  `0,0x7400,...,0x32c00`.
- [ ] **Stored versus compressed materialization — page loop lines 667-670.** Pages 1-5 use fixed
  stored capacities `128,87040,768,1408,768`; pages 6-20 compress a full zero-padded `0x7400`
  buffer. `content.chunks(0x7400)` currently omits empty sections, compresses small semantic
  slices, and computes minimal allocations for stored pages. Rename/remove `decompressed_size`:
  clear word 3 is allocation, while semantic sizes live in Section Info.
- [ ] **Descriptor policy — `encode_section_info`, lines 606-637.** Required descriptor order is
  `0,13,12,...,1`. Semantic sizes are `0,726,1390,672,86191,76,16,213182,89,6,2085,8194,123,896`.
  Compression is 1 for IDs 9-13 and 2 for 0-8; FileDepList encryption is 2, all others here 0.
  Max allocation is stored capacity for 9-13 and `0x7400` for D2 sections. Lines 620-624 currently
  write content length as max size, compression 2 universally, and encryption 0 universally.
- [ ] **Page map topology — page-map block lines 683-690.** Emit 22 entries: ordinary pages
  1-20, Section Info 23/allocation1024, and Section Map 24/allocation1600. IDs 21/22 are absent.
  The current map neither reserves those IDs nor adds its own page-24 entry.
- [ ] **Preamble — lines 692-696.** Fill fixture-derived standard fields at `0x0b..0x34`, including
  maintenance-release byte, discriminator 3, Preview/Summary/AppInfo/AppInfoHistory addresses,
  storing/target/application versions, codepage, security and fixed zero fields. Current output
  populates only magic, target maintenance, codepage, and header address.
- [ ] **Encrypted primary header — lines 707-727.** Derive the exact 22-page topology:
  last section ID 24, relative allocated end `0x248a0`, second-header address `0x24432`, page count
  22, Section Map ID/address `24/0x24260`, Section Info ID 23, section-array size 24, CRC32
  `fcad36c8`. Current values derive from the partial sequential topology. Replace synthesized
  zero-pad ciphertext at lines 726-727 with the specified 20-byte primary-header tail derivation.
- [ ] **Second header and EOF — lines 704-705 and 728.** After page-24 trailer, append bytes
  `0x80..0xeb` exactly as encrypted ciphertext, not the plaintext `header`; physical EOF must be
  `0x2449e` while the directory allocation cursor remains `0x249a0`.
- [ ] **Boundary and lifecycle — `encode_r2004_snapshot`, lines 732-734.** Validate all logical
  cross-section invariants before native materialization, reject unsupported semantic states
  atomically, and add per-stage fixture assertions using this oracle before the full original-byte
  DSL/pack/diff/mutation/analyzer/composer lifecycle.

## AC1024 Table-Control and Table-Record Implementation Matrix (2026-08-14)

This is a read-only, code-ready specialization of LibreDWG's R2010 branches in
[`COMMON_TABLE_FLAGS`](https://github.com/LibreDWG/libredwg/blob/master/src/spec.h#L754-L800) and
[`BLOCK_CONTROL` through `DIMSTYLE`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec#L3125-L4453).
No production file was edited and Nx was not started. The fixture oracle is the independently
decoded fixed-type inventory; counts below are native framed-object counts, not synthesized drawing
collections.

### Fixture cardinality and control invariants

| Family | Control type/count | Record type/count | Required AC1024 control invariant |
| --- | ---: | ---: | --- |
| Block | 48 / 1 | 49 / 10 | `entries.len() == 10`; model-space and paper-space are required type-49 refs and each resolves to one of those ten entries. |
| Layer | 50 / 1 | 51 / 7 | `entries.len() == 7`; every ref resolves once to a type-51 record owned by this control. |
| Text style | 52 / 1 | 53 / 2 | `entries.len() == 2`; every ref resolves once to type 53. |
| Linetype | 56 / 1 | 57 / 3 | one ordinary entry plus required dedicated ByBlock and ByLayer type-57 refs accounts for all three records; the three refs are distinct. |
| View | 60 / 1 | 61 / 0 | empty entry vector and no type-61 frame. |
| UCS | 62 / 1 | 63 / 0 | empty entry vector and no type-63 frame. |
| Viewport table | 64 / 1 | 65 / 1 | one entry resolving once to type 65. |
| Registered application | 66 / 1 | 67 / 25 | 25 ordered entries, each resolving once to type 67. |
| Dimension style | 68 / 1 | 69 / 2 | two ordered entries plus a separately counted vector of additional hard references; `more_count == additional_handles.len()`. |

The fixture therefore contains exactly **9 controls and 50 records** in these families. The 48
non-block table graph previously inventoried is the eight controls plus 40 records after excluding
the 1 + 10 block family. Control order is semantic because the writer emits entry handles in vector
order. A decoder must prove each entry count from the main stream and each reference from the handle
stream; record counts alone are not permission to repair a mismatched control.

### Split-stream frame contract

For every table record, the frame/common-object decoder first consumes object handle, EED, reactor
count, extension-dictionary presence and the R2010 string-stream boundary. Common object relations
then occupy the handle-stream prefix in this order: owner (`code 4`), ordered reactors (`code 4`),
optional extension dictionary (`code 3`). `COMMON_TABLE_FLAGS` contributes:

1. string stream: Unicode `name T`;
2. main stream: `is_xref_resolved BS`, whose only accepted AC1024 values are `0` and `256`;
3. handle stream: nullable table `xref H` (`code 5`) immediately after common object relations.

`is_xref_reference` is the R2007+ constant true and dependency is derived from resolution `256`;
neither is another native bit in AC1024. Model this as a typed `DwgTableReferenceState` containing
the name, resolution enum and optional xref handle, not as a raw table flag. The writer rejects an
xref-state/handle contradiction rather than inventing a handle. Each family then consumes its main,
string and handle substreams independently and must finish main exactly, finish every declared
string, finish the handle stream modulo at most seven zero terminal bits, and reject nonzero terminal
padding.

Controls do not have a string stream. Their main and role-handle order is exact:

| Tagged control variant | Main stream | Handle stream after owner/reactors/xdic |
| --- | --- | --- |
| `Block` / 48 | `entry_count BL` | entries (`code 2`), model space (`code 3`), paper space (`code 3`) |
| `Layer` / 50 | `entry_count BL` | entries (`code 2`) |
| `TextStyle` / 52 | `entry_count BL` | entries (`code 2`) |
| `Linetype` / 56 | `entry_count BS` | ordinary entries (`code 2`), ByBlock (`code 3`), ByLayer (`code 3`) |
| `View` / 60 | `entry_count BL` | entries (`code 2`) |
| `Ucs` / 62 | `entry_count BS` | entries (`code 2`) |
| `Viewport` / 64 | `entry_count BS` | entries (`code 2`) |
| `RegisteredApplication` / 66 | `entry_count BS` | entries (`code 2`) |
| `DimensionStyle` / 68 | `entry_count BS`, `more_count RC` | entries (`code 2`), additional hard refs (`code 5`) |

Counts are writer-derived from vectors and never persisted twice. Every required role handle is
non-null, except that an empty ordinary entry vector is valid for VIEW/UCS. The specific compact
width is part of the family variant: the live universal `read_bs()` shifts BLOCK/LAYER/STYLE/VIEW
controls and loses DIMSTYLE's `RC` before handle decoding.

### Typed record variants and exact AC1024 order

The following orders begin after the shared table prefix. `T` declarations go to the R2010 string
stream in declaration order; `H` declarations go to the handle stream even where the specification
interleaves them with main fields. CMC values are typed colors whose optional standard color-name and
book-name concepts are handled by the CMC codec, never raw bytes.

#### `BlockHeader` / fixed type 49 / 10 fixture records

- Main: `anonymous B`, `has_attributes B`, `is_xref B`, `xref_overlaid B`, `xref_loaded B`; if
  neither xref nor overlaid, `owned_count BL`; `base_point 3BD`; `insert_count` as the R2000+
  terminated RC sequence; `preview_size BL` followed by the typed semantic block preview;
  `insert_units BS`, `explodable B`, `block_scaling RC`.
- Strings: `name`, `xref_path`, `description`.
- Handles after common/xref: block-start entity (`code 3`), exactly `owned_count` entities
  (`code 4`, AC1024 has no first/last pair), end-block entity (`code 3`), exactly the derived insert
  backrefs (`code 4`), layout (`code 5`).
- Schema: `DwgBlockTableRecord { common, anonymous, has_attributes, xref_kind,
  xref_loaded, owned_entities, base_point, xref_path, insert_backrefs, description,
  preview: Option<DwgBlockPreview>, insert_units, explodable, scaling, block_entity,
  end_block_entity, layout }`. Preview is decoded image content, not native preview bytes.
- Writer gates: derive both counts; xref/overlay forbids owned entities; require BLOCK/ENDBLK
  pairing and common ownership; model/paper control refs select two of these ten records; materialize
  preview deterministically and reject unsupported preview semantics.

#### `Layer` / fixed type 51 / 7 fixture records

- Main: packed `flag0 BS`, then `color CMC`. Decode `flag0` into frozen, off, frozen-in-new,
  locked, plottable and typed lineweight; never persist the packed word.
- Strings: `name`, plus any typed CMC names.
- Handles after common/xref: plot style (`code 5`), material (`code 5`), linetype (`code 5`).
  Visual style is R2013+, therefore forbidden in AC1024.
- Schema: `DwgLayerTableRecord { common, frozen, off, frozen_in_new_viewports, locked,
  plottable, lineweight, color, plot_style, material, linetype }`.
- Writer gates: reconstruct `flag0` without overlapping bits, resolve every typed ref, and reconcile
  the seven framed records one-to-one with the drawing's layer projection rather than serializing a
  second independent layer authority.

#### `TextStyle` / fixed type 53 / 2 fixture records

- Main: `is_shape B`, `is_vertical B`, `text_size BD`, `width_factor BD`, `oblique_angle BD`,
  `generation RC`, `last_height BD`.
- Strings: `name`, `font_file`, `bigfont_file`. Handles: common/xref only.
- Schema: `DwgTextStyleTableRecord { common, shape, vertical, text_size, width_factor,
  oblique_angle, generation: DwgTextGeneration, last_height, font_file, big_font_file }`.
- Writer gates: validate finite dimensions/angle, enum range and exact string-footer consumption.

#### `Linetype` / fixed type 57 / 3 fixture records

- Main: `description T`, `pattern_length BD`, `alignment RC`, `dash_count RC`; for each dash in
  order: `length BD`, `complex_shape_code BS`, style `H`, `x_offset RD`, `y_offset RD`, `scale BD`,
  `rotation BD`, `shape_flags BS`; if any dash is textual, the main stream then contains the
  R2007+ 512-byte UTF-16 strings area.
- Strings: `name`, `description`. Dash text is a logical per-dash string; it is not persisted as the
  512-byte native area. Handles after common/xref: each dash style (`code 5`) in dash order.
- Schema: `DwgLinetypeTableRecord { common, description, pattern_length, alignment,
  dashes: Vec<DwgLinetypeDash> }`, where a dash contains length, typed complex-shape/text role,
  optional style, offset, scale and rotation.
- Writer gates: derive dash count and text-presence; rebuild and bound the 512-byte UTF-16 area;
  validate shape-flag/shape-code/style/text consistency and pattern total; require control topology
  `1 ordinary + ByBlock + ByLayer == 3` without duplicating special records in the ordinary vector.

#### `View` / fixed type 61 / 0 fixture records

- Main: view height/width `BD`; center `2RD`; target/direction `3BD`; twist, lens, front and back
  `BD`; view mode `4BITS`; render mode `RC`; default-lights `B`, lighting type `RC`, brightness and
  contrast `BD`, ambient `CMC`; paper-space `B`; associated-UCS `B`, and when set, UCS origin/X/Y
  `3BD`, elevation `BD`, ortho view `BS`; camera-plottable `B`.
- Strings: `name` plus typed ambient-color names.
- Handles after common/xref: background (`4`), visual style (`5`), sun (`3`); when associated-UCS,
  base UCS (`5`) then named UCS (`5`); live section (`4`).
- Schema: `DwgViewTableRecord { common, camera, clipping, render, lighting, paper_space,
  associated_ucs: Option<DwgUcsPlacement>, camera_plottable, background, visual_style, sun,
  live_section }` using shared typed view/UCS concepts.
- Writer gates: the fixture control must remain a valid empty table; any authored record requires
  all conditional UCS fields/handles and exact handle order before it may be exported.

#### `Ucs` / fixed type 63 / 0 fixture records

- Main: origin/X/Y `3BD`, elevation `BD`, ortho view `BS`, orthographic-point count `BS`, then
  ordered pairs of ortho type `BS` and point `3BD`.
- Strings: `name`. Handles after common/xref: base UCS (`5`), named UCS (`5`).
- Schema: `DwgUcsTableRecord { common, placement: DwgUcsPlacement,
  orthographic_points: Vec<DwgOrthographicUcsPoint>, base_ucs, named_ucs }`.
- Writer gates: derive count, validate orthographic enum values and finite bases, and preserve the
  fixture's zero-entry control without inventing a default UCS record.

#### `Viewport` / fixed type 65 / 1 fixture record

- Main: view height/width `BD`; center `2RD`; target/direction `3BD`; twist/lens/front/back `BD`;
  view mode `4BITS`; render mode `RC`; default-lights `B`, lighting type `RC`, brightness/contrast
  `BD`, ambient `CMC`; lower-left/upper-right `2RD`; UCS-follow `B`; circle zoom `BS`; fast zoom
  `B`; UCS icon `BB`; grid mode `B`, grid unit `2RD`; snap mode/style `B`, snap isopair `BS`, snap
  angle `BD`, snap base `2RD`, snap unit `2RD`; UCS-at-origin/UCSVP `B`; UCS origin/X/Y `3BD`,
  elevation `BD`, ortho view `BS`; grid flags and grid-major `BS`.
- Strings: `name` plus typed ambient-color names.
- Handles after common/xref: background (`4`), visual style (`5`), sun (`3`), named UCS (`5`),
  base UCS (`5`). AC1024 includes snap angle/base; only AC1020/R2006 omits them.
- Schema: `DwgViewportTableRecord { common, view, clipping, render, lighting, bounds,
  navigation, grid, snap, ucs, background, visual_style, sun, named_ucs, base_ucs }`.
- Writer gates: validate bounds, finite camera/UCS values, grid and snap enums; derive the four-bit
  view mode from typed booleans; require exactly one control entry and one resolved type-65 record.

#### `RegisteredApplication` / fixed type 67 / 25 fixture records

- Main: group-71 registered-application marker `RC`. Strings: `name`. Handles: common/xref only.
- Schema: `DwgRegisteredApplicationTableRecord { common, group_71: DwgRegAppGroup71 }` where the
  reserved marker is a validated named standard scalar, not an `unknown` byte or container capture.
- Writer gates: accept only defined/fixture-proven marker semantics, preserve ordered names, and
  require 25 distinct type-67 refs from the control.

#### `DimensionStyle` / fixed type 69 / 2 fixture records

- Strings in declaration order: `name`, `DIMPOST`, `DIMAPOST`, `DIMALTMZS`, `DIMMZS`, plus names
  owned by typed CMC colors.
- Main group 1: `DIMSCALE BD(1)`; `DIMASZ`, `DIMEXO`, `DIMDLI`, `DIMEXE`, `DIMRND`, `DIMDLE`,
  `DIMTP`, `DIMTM BD(0)`.
- Main group 2: `DIMFXL BD`, `DIMJOGANG BD`, `DIMTFILL BS`, `DIMTFILLCLR CMC`.
- Main group 3: `DIMTOL`, `DIMLIM`, `DIMTIH`, `DIMTOH`, `DIMSE1`, `DIMSE2 B`; `DIMTAD`, `DIMZIN`,
  `DIMAZIN`, `DIMARCSYM BS`.
- Main group 4: `DIMTXT`, `DIMCEN`, `DIMTSZ`, `DIMALTF`, `DIMLFAC`, `DIMTVP`, `DIMTFAC`, `DIMGAP`,
  `DIMALTRND BD`; `DIMALT B`, `DIMALTD BS`, `DIMTOFL`, `DIMSAH`, `DIMTIX`, `DIMSOXD B`;
  `DIMCLRD`, `DIMCLRE`, `DIMCLRT CMC`.
- Main group 5: `DIMADEC`, `DIMDEC`, `DIMTDEC`, `DIMALTU`, `DIMALTTD`, `DIMAUNIT`, `DIMFRAC`,
  `DIMLUNIT`, `DIMDSEP`, `DIMTMOVE`, `DIMJUST BS`; `DIMSD1`, `DIMSD2 B`; `DIMTOLJ`, `DIMTZIN`,
  `DIMALTZ`, `DIMALTTZ BS`; `DIMUPT B`; `DIMATFIT BS`.
- Main group 6: `DIMFXLON B`; R2010 `DIMTXTDIRECTION B`, `DIMALTMZF BD`, `DIMALTMZS T`,
  `DIMMZF BD`, `DIMMZS T`; `DIMLWD`, `DIMLWE BS`; `flag0 B`.
- Handles after common/xref: `DIMTXSTY`, `DIMLDRBLK`, `DIMBLK`, `DIMBLK1`, `DIMBLK2`,
  `DIMLTYPE`, `DIMLTEX1`, `DIMLTEX2`, all code 5 in that order.
- Schema: a named `DwgDimensionStyleTableRecord` composed from typed dimension geometry,
  tolerance, primary/alternate units, text, color, fill, lineweight and arrow/linetype reference
  groups; do not flatten the 70+ concepts into a numeric map.
- Writer gates: apply the exact defaults encoded by BD0/BD1/BS0/B0, validate enums/ranges and all
  eight handle roles, reconstruct `flag0`, require two resolved record refs, and emit the control's
  additional-hard-reference count/vector independently of record count.

### Required schema variants and live implementation gap

The clean model is two tagged enums with shared components, not a generic body plus a `type_code`
switch:

```text
DwgTableControl = Block | Layer | TextStyle | Linetype | View | Ucs |
                  Viewport | RegisteredApplication | DimensionStyle
DwgTableRecord  = BlockHeader | Layer | TextStyle | Linetype | View | Ucs |
                  Viewport | RegisteredApplication | DimensionStyle
```

`DwgLogicalObjectBody` should retain one table-control and one table-record envelope only if those
envelopes contain the tagged variants above and their DSL/pack facets mirror every variant. Shared
components should be `DwgTableRecordCommon`, typed color, view, lighting, UCS and dimension-style
groups. Do not persist `entry_count`, packed flags, the linetype strings area, block preview native
bytes, split-stream offsets or handle encodings.

The live gaps are concrete:

- Snapshot `schema/📸️snapshot/🦀️component.rs:318-331` has one control with only entries and
  model/paper handles, and one record with only `name`. It cannot represent seven control-specific
  roles, any common xref state, or any family payload.
- `DwgLogicalObjectBody` at snapshot lines 353-368 exposes only the two generic variants, so current
  DSL/pack persistence cannot distinguish a layer from a dimstyle except through the surrounding
  numeric type code; an invalid body/type pairing remains representable.
- IO `io/🦀️component.rs:2927-2935` reads `BS` for every control, silently drops failed/null entry
  handles with `filter_map`, ignores LTYPE and DIMSTYLE special handles, and decodes every record as
  one Unicode name. The `if let Ok`/`if let Some` branches leave an unsupported object instead of
  rejecting it atomically.
- No production match arm writes `TableControl` or `TableRecord`; native serialization currently
  has no inverse for these bodies. Before enabling any family, implement a bounded decode/write
  pair and fixture same-frame equality, then enable full graph materialization only after all 59
  control/record frames and cross-reference gates pass.

Implementation order: (1) shared record state and nine tagged controls; (2) BLOCK/LAYER/STYLE;
(3) LTYPE including semantic string-area construction; (4) APPID and DIMSTYLE; (5) empty VIEW/UCS
plus the single VPORT; (6) two-pass object offsets/relative handles and exact handle-map rebuild.
Each stage extends the existing AC1024 lifecycle test with fixture counts, exact split-stream
consumption, mutation plus inverse of one family field, typed-body/type-code anti-mismatch, and
original-byte equality through DSL, pack, diff, mutation, analyzer and composer.

## AC1024 Exact Logical Frame Implementation Progress (2026-08-14)

The ordinary R2010 logical frame writer and strict native-slice verifier now pass these independently
filtered Nx cohorts against `temp/architectural_example.dwg`:

- XRECORD: 145/145 exact frames.
- DICTIONARY and ACDBDICTIONARYWDFLT: 84/84 exact frames.
- All nine fixed table controls: 9/9 exact frames.
- Tagged table records: 50/50 exact frames: BLOCK_HEADER 10, LAYER 7, STYLE 2, LTYPE 3,
  VPORT 1, APPID 25, and DIMSTYLE 2.

The accepted exact-frame total is therefore 288. Every verifier compares the complete native
`MS + UMC + BOT/data/string/handle streams + CRC` slice without weakening the original-byte
assertion. The writer derives terminal one-fill bits and all split-stream framing; none is persisted.

Important fixture-proven corrections made during the exact loop:

- Handle codes 6/8/A/C alone are relative; codes 2/3/4/5 carry absolute values.
- BLOCK_CONTROL contains ordered nullable entry slots; a code-2 zero is a semantic empty slot, not
  permission to drop the position.
- Every table control carries the derived false R2010 string-stream footer bit.
- BLOCK_HEADER owned-entity vectors use deterministic code 3 in this Autodesk AC1024 fixture. The
  `*Model_Space` frame has 48 such roles; writing code 4 was the sole first divergence at handle bit
  65 and changing the standard R2010 policy to code 3 made all ten frames exact.
- CMC state is logical index/RGB/optional name/book data; packed flags are reconstructed.
- Table record bodies are tagged variants. The former generic name-only record is removed.

Scoped command used for the current table record gate:

```sh
CARGO_TERM_COLOR=never CARGO_TARGET_DIR='.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/SEMIO-ARTIFACT-LOSSLESS-WELL-KNOWN-FORMAT-ROUNDTRIPS/🎯️svg-logical-target-quiet' bun ./📜️script.ts nx run @semio-tech/stdio-plugin:test-long --skip-nx-cache -- table_record_logical_frames_reencode_exactly --nocapture
```

Latest result: one test passed, 3383 skipped, Nx success. The expectation is 50 exact records.
DIMSTYLE is represented by named geometry, behavior, text, unit, R2010, color, string, and eight
role-handle groups; no numeric property map or native byte state is retained.

## AC1024 Fixed-Entity Decode/Write Matrix (2026-08-14)

This read-only matrix specializes the fixture's fixed geometry entities and the requested adjacent
INSERT/VIEWPORT bodies against LibreDWG's primary R2010 prescriptions:
[`common_entity_data.spec`](https://github.com/LibreDWG/libredwg/blob/master/src/common_entity_data.spec),
[`common_entity_handle_data.spec`](https://github.com/LibreDWG/libredwg/blob/master/src/common_entity_handle_data.spec),
[`COMMON_ENTITY_DIMENSION`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg_spec_shared.h#L27-L143),
and [`INSERT`, `ARC`, `LINE`, `DIMENSION_LINEAR`, `VIEWPORT`, `LWPOLYLINE`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec).
No production file was edited and Nx was not started.

### Scope and fixture oracle

| Fixed type | Class | Fixture frames | Current status relevant to this matrix |
| ---: | --- | ---: | --- |
| 17 | ARC | 12 | Included in the 80-frame geometry decode attempt; exact successful/failing split was not logged. Existing projections discard thickness and most common state. |
| 19 | LINE | 40 | Included in the same aggregate; exact successful/failing split was not logged. Existing writer uses the wrong point representation. |
| 21 | DIMENSION_LINEAR | 12 | All 12 are unsupported by the live geometry decoder and absent from the logical geometry enum. |
| 77 | LWPOLYLINE | 16 | Included in the aggregate; projections discard width, thickness, extrusion and vertex IDs. |
| 7 | INSERT | 12 | Separate block-graph blocker requested here; no typed framed-entity body. |
| 34 | VIEWPORT | 2 | Separate fixed-support blocker requested here; no typed framed-entity body. |

The four geometry types total **80** frames. The historical test reported 18 projected geometries
and therefore **62 unsupported projections**, but did not record the successful count per fixed
type. That 62 is a valid aggregate acceptance oracle only; it cannot be assigned to ARC/LINE/LWP
without a new per-type diagnostic. Moreover, none of the 18 projections is lossless because the
current drawing projection has only layer index, a reduced color and flattened geometry. The native
writer therefore needs all 80 handle-keyed typed frames, not only the 62 that failed projection.
INSERT and VIEWPORT add 14 adjacent entity frames, for 94 exact typed-frame assertions in this lane.

### R2010 common entity contract

Each fixed entity must be one handle-keyed authority:

```text
DwgEntityFrame { handle, extended_data, common: DwgEntityCommon, body: DwgEntityBody }
DwgEntityBody = Insert | Arc | Line | DimensionLinear | Viewport | LwPolyline | ...
```

It must not coexist with an independently ordered `entities` projection that can disagree with its
corresponding framed object. A drawing/mesh projection is derived from `DwgEntityFrame`, never a
second persisted source.

The exact AC1024 common **main-data** order before the class payload is:

1. semantic entity graphic presence `B`; when present, derived byte count `BLL` and a fully decoded,
   regenerable `DwgEntityGraphic` value. Raw graphic/native preview bytes are not schema state;
   unsupported graphic formats reject the entity.
2. ownership mode `BB`, reactor count `BL`, extension-dictionary-missing `B`. `nolinks` ends at
   R2002 and `has_ds_data` starts at R2013, so neither is an AC1024 bit.
3. entity color `ENC`: index plus typed RGB/alpha/reference/name/book concepts selected by its native
   flags. The packed color word and flags are derived, not persisted.
4. linetype scale `BD1`; linetype selector `BB`; plot-style selector `BB`; material selector `BB`;
   shadow mode `RC0`.
5. R2010 visual-style-presence bits in order: full `B`, face `B`, edge `B`; visibility `BS`;
   lineweight `RC`.

The live decoder at `io/🦀️component.rs:1496-1517` omits all three R2010 visual-style bits, so every
subsequent read moves three bits early whenever this path is used. It also drops graphic data,
linetype scale, RGB/alpha/color metadata, shadow, visibility and lineweight. The live writer at
lines 1462-1475 hard-codes only a reduced subset and omits the same three bits.

Handle emission follows field-declaration order across the dedicated handle stream. A referenced
entity color handle is emitted by the common color codec at its declaration point. At
`COMMON_ENTITY_HANDLE_DATA`, emit:

1. explicit owner (`code 4`) only for ownership mode 0; modes 1/2 derive paper/model-space ownership;
2. ordered reactors (`code 4`), then optional extension dictionary (`code 3`);
3. layer (`code 5`), optional explicit linetype (`code 5` when selector 3);
4. optional material (`code 5` when selector 3), optional shadow (`code 5` when shadow mode 3),
   optional plot style (`code 5` when selector 3);
5. optional full, face and edge visual-style handles (`code 5`) in that order;
6. the class-specific handles listed below.

`DwgEntityCommon` therefore needs typed ownership, semantic graphic, ordered reactors, optional
extension dictionary, complete entity color, linetype scale/selection, plot-style selection,
material selection, shadow mode, visibility, lineweight, layer and three optional visual-style
roles. Counts and selector bits are always derived. Decoder and writer must bound main, string and
handle readers separately and reject any over-read, leftover nonzero bit, missing conditional handle
or contradictory selector/handle state.

### Code-ready body matrix

#### `Insert` / type 7 / 12 fixture frames

Exact class **main** order after common entity data:

1. insertion point `3DPOINT`;
2. derived scale selector `BB` and its payload:
   - `3`: logical scale `(1,1,1)`, no values;
   - `2`: one X `RD`, copied to Y/Z;
   - `1`: X is one, Y and Z as `DD(1)`;
   - `0`: X `RD`, then Y and Z as `DD(X)`;
3. rotation `BD0`, extrusion `3DPOINT`, has-attributes `B`;
4. when has-attributes, owned-attribute count `BL`.

No class string is present. After common handles, class handles are block-header hard pointer
(`code 5`), then—only when attributes exist—exactly the derived number of attribute owners
(`code 4`) followed by SEQEND (`code 3`). AC1024 uses the R2004+ vector; the R13-R2000 first/last
attribute pair is forbidden.

Schema:

```text
DwgInsertEntity { insertion, scale, rotation, extrusion, block_header,
                  attributes: Vec<Handle<Attrib>>, sequence_end: Option<Handle<Seqend>> }
```

Writer gates: use the scale-selector priority exactly `all-one -> 3`, `all-equal -> 2`,
`x-is-one -> 1`, otherwise `0`; derive has-attributes/count; require empty attributes iff no
SEQEND, otherwise nonempty attributes plus exactly one SEQEND; every attribute/SEQEND common owner
resolves back to this INSERT; block-header resolves to type 49; its INSERT backref graph contains
this handle. Preserve the fixture's 12 INSERT handles and order.

#### `Arc` / type 17 / 12 fixture frames

Main order: center `3BD`, radius `BD`, thickness `BT0`, extrusion `BE`, start angle `BD`, end angle
`BD`. Handles: common entity handles only. Strings: none.

Schema: `DwgArcEntity { center, radius, thickness, extrusion, start_angle, end_angle }`.
Writer gates: finite values, nonnegative radius, valid nonzero extrusion; derive the BT default-zero
and BE default-Z selectors; preserve angles as native radians without normalization. The live decoder
at lines 1711-1720 has the correct class read order but discards thickness and the full common state.
The live writer at lines 1588-1594 is not its inverse: it omits thickness and emits
`center,radius,start,end,extrusion`, placing extrusion after both angles instead of before them.

#### `Line` / type 19 / 40 fixture frames

Main order:

1. derived `z_is_zero B`;
2. start X `RD`, end X `DD(start X)`, start Y `RD`, end Y `DD(start Y)`;
3. if either endpoint Z is nonzero: start Z `RD`, end Z `DD(start Z)`;
4. thickness `BT0`, extrusion `BE`.

Handles: common entity handles only. Strings: none. Schema:
`DwgLineEntity { start, end, thickness, extrusion }`; `z_is_zero` and DD selectors are writer policy.
Writer gates: derive zero-Z from both exact logical coordinates, preserve signed zero policy
deterministically, reject nonfinite coordinates, and use predecessor-based DD in the declared axis
order. Live decode lines 1675-1691 reads the right class layout but drops thickness/extrusion/common
state. Live writer lines 1573-1576 instead writes two `3BD` points, which is the wrong R2010 LINE
layout even when its geometry is numerically equal.

#### `DimensionLinear` / type 21 / 12 fixture frames

Exact class main order:

1. serializer-derived R2010 class version `RC = 0`;
2. common dimension: extrusion `3BD`, text midpoint `2RD`, elevation `BD`, derived `flag1 RC`, user
   text `T`, text rotation `BD0`, horizontal direction `BD0`, insertion scale `3BD_1`, insertion
   rotation `BD0`;
3. attachment `BS`, line-space style `BS1`, line-space factor `BD1`, actual measurement `BD`;
4. reserved `B = 0`, flip-arrow-1 `B`, flip-arrow-2 `B`, clone insertion point `2RD0`;
5. linear body: extension-line point 1 `3BD`, extension-line point 2 `3BD`, definition point `3BD`,
   oblique angle `BD`, dimension rotation `BD0`.

Only user text enters the class string stream. After common handles: dimension-style (`code 5`),
anonymous dimension block (`code 5`). Schema:

```text
DwgLinearDimensionEntity {
  extrusion, text_midpoint, elevation, status: DwgDimensionStatus, user_text,
  text_rotation, horizontal_direction, insertion_scale, insertion_rotation,
  attachment, line_spacing, actual_measurement, flip_arrow_1, flip_arrow_2,
  clone_insertion_point, extension_line_1, extension_line_2, definition_point,
  oblique_angle, dimension_rotation, dimension_style, dimension_block: Option<Handle<BlockHeader>>
}
```

The low dimension-type nibble is derived from fixed type 21 and must be linear. Derive native
`flag1 = 0x08 | (status & 0xe0) | (status bit 7 clear ? 1 : 0) | (status bit 5 set ? 2 : 0)` and
validate the inverse; `0x08` is the AC1024 native invariant measured on every fixture frame, not a
schema bit. Class version and the reserved zero bit are not schema fields. Writer gates: validate
attachment/spacing enums, finite geometry and positive spacing scale; dimension-style resolves to a
type-69 record, and a non-null block resolves to type 49 with coherent ownership/backrefs. The
class handle slot is still emitted when the optional dimension block is null. All 12
fixture frames must decode; the live `dwg_decode_entity`/`dwg_encode_entity` has no type-21 arm.

#### `LwPolyline` / type 77 / 16 fixture frames

Main order:

1. derived flag `BS`: extrusion present bit 1, thickness bit 2, constant width bit 4, elevation bit
   8, bulge vector bit 16, width vector bit 32, pline-generation bit 256, closed bit 512, R2010
   vertex-ID vector bit 1024;
2. conditional constant width `BD`, elevation `BD`, thickness `BD`, extrusion `3BD` in that order;
3. point count `BL`; conditional bulge count `BL`, vertex-ID count `BL`, width count `BL`;
4. first point `2RD`, remaining points as X/Y `DD(previous point)` pairs;
5. all bulges `BD`, then all vertex IDs `BL`, then all `(start width BD, end width BD)` pairs.

Handles: common entity handles only. Strings: none. Schema:

```text
DwgLwPolylineEntity {
  closed, pline_generation, constant_width: Option<f64>, elevation,
  thickness, extrusion, vertices: Vec<Point2>, bulges: Vec<f64>,
  vertex_ids: Vec<u32>, widths: Vec<DwgPolylineWidth>
}
```

Writer gates: at most 20,000 points; derive flags/counts; vertex IDs are absent or equal point count;
bulges and widths may be absent or are bounded by point count and retain native ordered vector
cardinality; constant-width and per-vertex-width representations are mutually consistent; default
extrusion/elevation/thickness omit their conditional fields. Never persist the flag or a 512/1024
selector. Live decode lines 1757-1794 discards constant width, thickness, extrusion, all vertex IDs
and all widths. Live writer lines 1618-1626 emits a nonstandard interleaved
`closed B,elevation BD,count,(point 2RD,bulge BD)*` layout rather than the native flag, grouped
counts, predecessor deltas and grouped arrays.

#### `Viewport` / type 34 / 2 fixture frames

Exact main order after common data:

1. center `3BD`, width `BD`, height `BD`;
2. target `3BD`, direction `3BD`, twist `BD`, view height `BD`, lens length `BD`, front clip `BD`,
   back clip `BD`, snap angle `BD`, view center `2RD`, snap base `2RD`, snap unit `2RD`, grid unit
   `2RD`, circle zoom `BS`, grid major `BS`;
3. derived frozen-layer count `BL`, status flags `BL`, style sheet `T`, render mode `RC`;
4. UCS-at-origin `B`, UCS-per-viewport `B`, UCS origin/X/Y `3BD`, elevation `BD`, orthographic view
   `BS`, shade-plot mode `BS`;
5. default lights `B`, lighting type `RC`, brightness `BD`, contrast `BD`, ambient color `CMC`.

The class string stream contains style sheet plus any strings owned by the typed ambient color.
After common handles: frozen-layer soft refs (`code 4`) in order, clip boundary (`code 5`), named
UCS (`code 5`), base UCS (`code 5`), background (`code 4`), visual style (`code 5`), shade plot
(`code 4`), sun (`code 3`). The obsolete viewport-entity-header reference ends at R2002. AC1024
includes snap angle/base; their omission applies only to AC1020/R2006.

Schema: `DwgViewportEntity { center, size, view, clipping, snap, grid, status, style_sheet,
render, ucs, lighting, ambient_color, frozen_layers, clip_boundary, named_ucs, base_ucs,
background, visual_style, shade_plot, sun }`. Writer gates: derive frozen count, validate bounds and
finite view/UCS/lighting values, preserve the two fixture handles, resolve all optional roles by
kind, and prohibit AC1020/R2002 fields. The live geometry decoder and writer have no type-34 arm.

### Acceptance and implementation order

1. Replace the flattened `DwgLogicalGeometry { kind, values, indices, text, closed }` and reduced
   `DwgLogicalEntity { layer index, color, geometry }` at snapshot lines 13-58 with typed entity
   bodies attached to the handle-keyed logical object. The flattened values cannot express common
   state, INSERT/DIMENSION/VIEWPORT, LWP widths/IDs, or ARC/LINE thickness.
2. Correct common main/handle codecs first, including the missing three R2010 visual-style bits and
   conditional reference order. Existing class decoders cannot be trusted until this lands.
3. Implement ARC and LINE typed pairs; assert exactly 12 and 40 frames and same-frame re-encoding.
4. Implement LWPOLYLINE with grouped arrays and assert 16 frames plus every count/vector invariant.
5. Implement all 12 DIMENSION_LINEAR frames and their DIMSTYLE/BLOCK graph; then all 12 INSERTs and
   both VIEWPORTs so ownership/layout references close.
6. Extend the existing test, not a new file, to print per-type attempted/decoded/rejected counts so
   the old 18/62 aggregate is replaced by `{17:12,19:40,21:12,77:16}` exact coverage. Assert main,
   string and handle reader exhaustion for every frame, mutation plus inverse for one common and one
   family field, body/type anti-mismatch rejection, and original fixture equality through DSL, pack,
   diff/apply/inverse/absorb, mutation/inverse, analyzer and composer.

No frame is accepted through a reduced drawing projection. Unsupported graphics, colors, flags,
conditional vectors or references reject atomically; no raw frame, raw common data or source-byte
fallback is permitted.

## AC1024 residual custom-object schema and graph matrix (2026-08-14)

This closes the fixture census left after XRECORD, dictionaries, table controls/records,
layout/block/placeholder/group/mlinestyle, and the 80 fixed geometry frames are removed. The source
inventory contains exactly **238 residual frames**:

| implementation tranche | type numbers | frames |
|---|---:|---:|
| priority 0: highest-count independently decodable bodies | 506, 520, 544 | 73 |
| priority 1: association action/dependency graph backbone | 539, 541, 542, 545 | 64 |
| priority 2: remaining association nodes/bodies | 540, 543, 549 | 22 |
| priority 3: remaining dynamic-block evaluation classes | 521-538, 546-548, 559 | 48 |
| priority 4: remaining style/context objects | 504, 505, 507, 508, 516, 517 | 31 |
| **total** | | **238** |

Primary field declarations were reconciled against LibreDWG
[`dwg2.spec`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/dwg2.spec). Its
`HANDLE_UNKNOWN_BITS` sites are diagnostic, non-consuming captures and are **not** fields, prefixes,
or writer input. Likewise, source-local names such as `b1`, `h1`, `bl96`, and `unknown_bool` do not
meet the semantic-schema contract: their standard role must be established from reciprocal graph
invariants or the class contract before those frames are accepted. No opaque bits or unnamed
scalars may enter the snapshot.

### Live implementation gap

The live `DwgLogicalObjectBody` at snapshot lines 471-527 has only `Dictionary`, `TableControl`,
`TableRecord`, and `XRecord`. The containing object at lines 531-548 has only a flattened
`referenced_handles` escape hatch. There is no typed custom-object body, class-specific decoder, or
class-specific writer for any of the 238 frames below. `referenced_handles` must not be used to
erase handle roles; each typed body below owns named handles in its declared order, while common
owner/reactor/extension-dictionary handles stay on `DwgLogicalObject`.

### Shared typed cores

The following cores should be implemented once and embedded structurally. Counts and duplicated
version/copy fields are derived writer values and import validators, not mutable schema fields.

#### Evaluation expression and dynamic-block elements

`DwgEvaluationExpression` main/string order:

1. signed parent expression ID `BLd`, major version `BL`, minor version `BL`, signed value tag `BSd`;
2. tagged `DwgEvaluationValue`: real `BD` for tag 40, point2 `2RD` for 10, point3 `2RD` for 11 as
   declared by the AC1024 codec, text `T` for 1, i32 `BL` for 90, object reference for 91, i16 `BS`
   for 70, or null for -9999;
3. node ID `BL`.

The sole conditional handle is the tag-91 value, emitted in the class handle stream with code 5.
Reject unknown tags and tag/payload mismatches.

`DwgBlockElement` then appends name `T`, validates the R2010 element versions (fixture writer
constants 33/29), and appends the standard element metadata/version `BL` currently named
`eed1071` upstream. That last scalar needs a public semantic name confirmed from the fixture before
schema exposure. Derived cores extend it in this exact order:

- `DwgBlockGrip`: grip state 91 `BL`, grip state 92 `BL`, location `3BD`, insert-cycling `B`, signed
  cycling weight `BLd`.
- `DwgBlockParameter`: show-properties `B`, chain-actions `B`.
- `DwgBlockOnePointParameter`: definition point `3BD`, two property-info records; each property has
  connection count `BL` and ordered `(code BL, name T)` pairs; trailing property-info count is
  derived and validated.
- `DwgBlockTwoPointParameter`: base/end definition points `3BD`, four property-info records, four
  property-state `BL` values, base-location `BS`.
- `DwgParameterValueSet`: flags `BL`, minimum/maximum/increment `BD`, value count `BS`, ordered `BD`
  values.
- `DwgBlockAction`: display location `3BD`, dependency count `BL`, dependency handles code 4,
  action-ID count `BL`, action IDs `BL`.
- `DwgActionWithBasePoint`: block-action core, offset `3BD`, two connection points `(code BL,
  name T)`, has-dependent `B`, base point `3BD`.
- `DwgActionOffsets`: X offset, Y offset, angle offset as three `BD`.
- `DwgBlockConstraintParameter`: two-point-parameter core, dependency handle code 5.
- `DwgBlockLinearConstraintParameter`: constraint core, expression name `T`, expression description
  `T`, value `BD`, value set.

All dependency handles from `DwgBlockAction` are written after the common object handle block and
before any derived class handles; the EvalExpr tag-91 handle follows its tagged value role.

#### Associative action/dependency cores

`DwgAssociativeAction` main order for AC1024:

1. class version `BS` (fixture gate 1), status `BL` enum 0-6;
2. action index `BL`, maximum dependency index `BL`, dependency count `BL`;
3. for every dependency, ownership bit `B` in the main stream;
4. R2013 owned-parameter/value extensions are absent.

Handle order: owning-network code 4, action-body code 3, then each dependency code 3 when owned and
code 4 otherwise. Schema fields are `{status, owning_network, action_body, action_index,
max_dependency_index, dependencies: [{owned, dependency}]}`. Version and count are derived.

`DwgAssociativeDependency` main/string order:

1. class version `BS` (fixture candidate 2), status `BL`;
2. read, write, attached-to-object, and delegating flags as four `B`;
3. signed order `BLd`, has-name `B`, conditional name `T`, signed dependency-body ID `BLd`.

Handle order: dependent-on-object code 3, action-chain link A code 4, action-chain link B code 3,
dependency-body code 4. The two chain links need reciprocal fixture validation before public names
are finalized as previous/next; until then they remain typed link A/B, never a generic handle
vector. Schema fields are `{status, is_read, is_write, is_attached, is_delegating, order,
dependent_on, name?, action_link_a, action_link_b, body, body_id}`; class version is derived.

`DwgEvaluationVariant`, used by associative variables, is a tagged union of `BD`, `BL`, `BS`,
`RC`, `T`, or object-reference handle. The type discriminator is derived from the variant. The
object-reference arm is emitted in the class handle stream only; reject unsupported discriminants.

### Priority 0: types 506, 520, and 544 (73 frames)

#### Type 506 `VISUALSTYLE` — 19 frames

Typed schema: `DwgVisualStyle { description, style_type, extension_lighting_model,
internal_use_only, properties }`, where `properties` is a fixed named record, not a map. Main order
is description `T`, style type `BL`, R2010 extension-lighting model `BS`, internal flag `B`, then the
following **28 value/modifier pairs**, each immediately followed by modifier `BS`:

1. face lighting model `BL`; 2. face lighting quality `BL`; 3. face color mode `BL`; 4. face
modifier `BS`; 5. face opacity `BD`; 6. face specular `BD`; 7. monochrome color `CMC`; 8. edge model
`BL`; 9. edge style `BL`; 10. intersection color `CMC`; 11. obscured color `CMC`; 12. obscured line
type `BL`; 13. intersection line type `BL`; 14. crease angle `BD`; 15. edge modifier `BL`; 16. edge
color `CMC`; 17. edge opacity `BD`; 18. edge width `BL`; 19. edge overhang `BL`; 20. edge jitter
`BL`; 21. silhouette color `CMC`; 22. silhouette width `BL`; 23. halo gap `BL`; 24. isoline count
`BL`; 25. hide precision `B`; 26. display settings `BL`; 27. brightness `BD`; 28. shadow type `BL`.

There are no class handles. Writer gate: all 28 fields and modifiers are mandatory, extension
lighting is present for AC1024, colors use typed CMC, and reader exhaustion must be exact.

#### Type 520 `BLOCKGRIPLOCATIONCOMPONENT` — 23 frames

Schema: `DwgBlockGripLocationComponent { expression: DwgEvaluationExpression, grip_type,
grip_expression }`. Main/string order is evaluation-expression core, grip type `BL`, grip
expression `T`. The only possible class handle is the expression's tag-91 object reference. This is
the clean first dynamic class because it has no inherited block-element/grip ambiguity.

#### Type 544 `ACDBASSOCGEOMDEPENDENCY` — 31 frames

Schema: `DwgAssociativeGeometryDependency { dependency: DwgAssociativeDependency,
enabled, persistent_subentity }`, with `persistent_subentity = { class_name,
dependent_on_compound_object }`. After the dependency core: derived class version `BS` (fixture 0),
enabled `B`, persistent-subentity class name `T`, compound-object flag `B`. No derived handles.
Writer validates the class version and requires the dependent object/persistent-subentity class to
form a legal typed pair; no source class string or unparsed subentity token survives import.

### Priority 1: association graph backbone (64 frames)

| type/class | count | main/string payload after shared core | class handle order |
|---|---:|---|---|
| 539 `ACDBASSOCNETWORK` | 5 | action core; network version `BS`; network action index `BL`; action count `BL`; one ownership `B` per action; owned-action count `BL` | action-core handles; every action code 3 if owned/code 4 otherwise; explicit owned-action list code 4 |
| 541 `ACDBASSOCVALUEDEPENDENCY` | 23 | dependency core; value-dependency version; typed cached `EvalVariant`; value-name `T` | dependency-core handles plus conditional EvalVariant object reference |
| 542 `ACDBASSOCDEPENDENCY` | 18 | dependency core only | dependency-core handles only |
| 545 `ACDBASSOCVARIABLE` | 18 | action core; variable version `BL` = 2; name `T`; expression `T`; evaluator ID `T`; description `T`; typed evaluation variant; mergeable `B`; mergeable-variable name `T`; must-merge `B` | action-core handles, then conditional evaluation-variant object handle |

For type 539, schema stores one ordered `actions: [action]` collection. The fixture proves the native
per-member Boolean is a handle-strength selector rather than semantic ownership: all four nested-network
members use false/code 4 although their action-core `owning_network` relations point back to the parent,
while all 24 leaf actions use true/code 3 and have the same reciprocal relation. The separate native
`owned_actions` vector is empty in all five frames. Counts, selector Booleans, handle strengths and that
empty AC1024 auxiliary vector are derived. Writer rejects duplicate/dangling members or any member whose
back-reference does not name the network.

For types 541/542, there is no raw prefix: the upstream `HANDLE_UNKNOWN_BITS` diagnostic consumes
nothing. Type 541 does carry the typed specialization proven below; type 542 remains dependency-core
only unless its own bounded evidence proves another named concept.

For type 545, the expression is a semantic expression string and evaluator ID is a typed evaluator
identifier. The encoded variant representation is selected solely by the typed variant. Variable
graph validation requires each owned value dependency to point back to this action, referenced
variable names to resolve, and value-dependency names to agree with expression resolution.

### Priority 2: remaining association classes (22 frames)

| type/class | count | exact declared payload | handles and acceptance gate |
|---|---:|---|---|
| 540 `ACDBASSOC2DCONSTRAINTGROUP` | 4 | action core; group version `BL` = 2; one source-local `b1 B`; work plane origin/X/Y as three `3BD`; one source-local `h1`; action count `BL`; node count `BL`; each node ID `BLd`, AC1024 status `RC`, connection count `BL`, connection IDs `BL` | action-core handles; `h1` code 4; action refs code 4. Reject until `b1` and `h1` have named roles and the fixture's node tags can select a complete typed constraint-node union; the upstream base-node-only model is insufficient. |
| 543 `BLOCKPARAMDEPENDENCYBODY` | 6 | dependency-body version `BS` = 1; dimension-base version `BS` = 1; name `T`; class version `BS` = 0 | no derived handles; body must be owned by exactly one dependency and its name must agree with the parameter role |
| 549 `ASSOCDIMDEPENDENCYBODY` | 12 | dependency-body version `BS` = 1; dimension-base version `BS` = 1; name `T`; class version `BS` = 1 | no derived handles; body must be owned by exactly one dimension dependency and resolve the associated dimension |

The eventual type-540 `DwgConstraintNode` union must cover the standard node classes present in the
fixture—constraint geometry, geometric constraint, point/implicit point, explicit/angle/parallel/
distance constraints, ellipse and spline variants—not retain a node tag plus raw tail.

### Priority 3: remaining dynamic-block classes (48 frames)

All field order below follows the shared core, and all listed handles follow common object handles.

| type/class | count | typed payload and order | derived handles / hard gate |
|---|---:|---|---|
| 521 `BLOCKMOVEACTION` | 2 | block-action core; two `(code BL,name T)` connection points; X/Y/angle offsets `BD` | action dependency handles only |
| 522 `ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION` | 2 | standard version/status `BS` (fixture declaration 1) | protected block code 5; prove public scalar name before accepting |
| 527 `BLOCKLINEARPARAMETER` | 2 | two-point parameter; distance name `T`; description `T`; value `BD`; value set | optional EvalExpr reference only |
| 528 `BLOCKLINEARGRIP` | 4 | block-grip core; orientation `3BD` | optional EvalExpr reference only |
| 529 `BLOCKFLIPPARAMETER` | 3 | two-point parameter; flip label, flip description, base-state label, flipped-state label `T`; label point `3BD`; source-local `bl96 BL`; tooltip `T` | reject until `bl96` has a named standard role |
| 530 `BLOCKFLIPGRIP` | 3 | block-grip core; combined state `BL`; orientation `3BD` | optional EvalExpr reference only |
| 531 `BLOCKVISIBILITYPARAMETER` | 1 | one-point parameter; initialized `B`; name/description `T`; source-local boolean; block count; state count; each state has name, block refs, parameter refs | blocks code 4, then per-state blocks code 4 and parameters code 4; reject until source-local boolean is named |
| 532 `BLOCKVISIBILITYGRIP` | 1 | block-grip core only | optional EvalExpr reference only |
| 533 `BLOCKALIGNMENTPARAMETER` | 2 | two-point parameter; align-perpendicular `B` | optional EvalExpr reference only |
| 534 `BLOCKALIGNMENTGRIP` | 2 | block-grip core; orientation `3BD` | optional EvalExpr reference only |
| 535 `BLOCKSTRETCHACTION` | 6 | block-action core; two connection points; point count `BL` plus `2RD` points; handle-entry count; each entry has index count `BS` plus `BL` indices; code-entry count; each has code `BL`, index count `BS`, indices; X/Y/angle offsets `BD` | action dependencies, then each handle-entry handle code 0. Reject debug tail; counts derived and every index bounded by point/action collections. |
| 536 `BLOCKSCALEACTION` | 1 | action-with-base-point core; three additional connection points in declaration order | action dependencies plus any EvalExpr object value; validate the base and derived connection groups independently |
| 537 `BLOCKFLIPACTION` | 3 | block-action core; four connection points | action dependencies only |
| 538 `BLOCKBASEPOINTPARAMETER` | 1 | one-point parameter; point `3BD`; base point `3BD` | optional EvalExpr reference only |
| 546 `BLOCKVERTICALCONSTRAINTPARAMETER` | 1 | linear-constraint-parameter core only | dependency code 5 plus optional EvalExpr reference |
| 547 `ACDB_DYNAMICBLOCKPROXYNODE` | 1 | evaluation-expression core only | conditional tag-91 object reference only; no diagnostic raw tail |
| 548 `BLOCKHORIZONTALCONSTRAINTPARAMETER` | 1 | linear-constraint-parameter core only | dependency code 5 plus optional EvalExpr reference |
| 559 `ACDB_BLOCKREPRESENTATION_DATA` | 12 | standard representation version/state `BS` | represented block code 3; distinguish this ownership role from type 522's code-5 protected block and prove scalar name before accepting |

For type 531, represent visibility states as ordered
`{name, visible_blocks, controlled_parameters}` records; counts are derived and every reference must
resolve to an eligible block/parameter. For 535, represent stretch selection as typed
`{points, handle_entries: [{object, point_indices}], code_entries: [{code, point_indices}]}`. No raw
selection payload is permitted.

### Priority 4: remaining style/context classes (31 frames)

#### Type 507 `SCALE` — 17 frames

Schema: `DwgAnnotationScale { name, paper_units, drawing_units, is_unit_scale }`. Wire order is
derived version/flag `BS` (fixture 0), name `T`, paper units `BD`, drawing units `BD`, unit-scale `B`.
No class handles. Validate finite positive units. The unit-scale marker is true only for the canonical
`1:1` entry with units `1/1`; equal non-unit values such as fixture `12/12` remain false, so it is not
derived from ratio equality alone. The complete 17-frame oracle is documented below.

#### Type 505 `MATERIAL` — 3 frames

Main/string order: name and description `T`; ambient and diffuse typed colors, each with enable flag
`RC`, factor `BD`, conditional RGB `BL`; diffuse map; specular color/map and gloss `BD`; reflection
map; opacity percentage `BD` and map; bump map; refraction index `BD` and map; then R2007+
translucence, self-illumination and reflectivity `BD`, illumination/channel/mode `BL`.

Each `DwgMaterialMap` is `{blend BD, projection RC, tiling RC, auto_transform RC, transform:
[BD;16], source}`. File source carries filename `T`; procedural source is a typed texture union
(wood, marble, or standard typed value/list), not a byte/string blob. There are no expected class
handles. Acceptance is blocked until exact fixture consumption proves whether the R2010 indirect
bump/reflection/transmission, two-sided, luminance, normal-map, global-illumination, final-gather and
color-bleed concepts that are disabled in the upstream declaration are physically present.

#### Type 508 `MLEADERSTYLE` — 1 frame

Main/string order: class version `BS`; content type `BS`; multileader order `BS`; leader order `BS`;
maximum points `BL`; segment angles `BD`; leader type `BS`; line color `CMC`; lineweight `BLd`;
landing flag/gap; dogleg flag/distance; description `T`; arrow size `BD`; default text `T`; left and
right attachment `BS`; AC1024 text angle `BS`; text alignment `BS`; text color `CMC`; text height
`BD`; frame `B`; always-left `B`; alignment spacing `BD`; block color `CMC`; block scale X/Y/Z
`BD`; use-scale `B`; rotation `BD`; use-rotation `B`; connection `BS`; overall scale `BD`; changed
`B`; annotative `B`; break size `BD`; attachment direction/top/bottom `BS`.

Handle order: line type, arrow, text style, block. All are named typed roles. R2013
text-extended fields are prohibited for AC1024.

#### Type 516 `SORTENTSTABLE` — 7 frames

Schema: `DwgSortEntitiesTable { block_owner, entries: [{entity, sort_key}] }`. Main-stream order is
derived entry count `BL`, then sort-key handles encoded as `H` code 0 **in the main object data**.
After common handles: block-owner code 4, then entity handles code 4. Zip the two equal-length
vectors into ordered entries at import; split them deterministically at export. Reject duplicate
entities, count mismatch, or entity ownership outside the named block.

#### Type 517 `ACAD_EVALUATION_GRAPH` — 2 frames

Schema: `DwgEvaluationGraph { nodes, edges }`. Wire order begins with a derived signed node-ID
watermark `BLd` and a duplicate copy that must match, then node count `BL`. Each node declares storage ID
`BL`, edge flags `BL` (fixture declaration 32), signed next ID `BLd`, four signed node slots, and a
conditional active-cycles `B` only when the graph condition requires it. Edge count `BL` follows;
each edge has ID `BL`, signed next ID, three signed endpoints/roles, and five signed outgoing-edge
slots. Each node's evaluation-expression reference is handle code 5 in node order.

Reject until every currently ordinal node/edge slot has a named graph role and the active-cycle
presence condition is established. `HANDLE_UNKNOWN_BITS` is not a field. Writer derives the
duplicate root and counts, validates all node/edge IDs and adjacency, and emits expression handles
after the common object handles.

#### Type 504 `TABLESTYLE` — 1 frame

The resolved logical target is `DwgTableStyle { description, bit_flags, template_style_handle, table,
title, header, data }`. AC1024 wire order begins with the derived R24 discriminator `RC=0`, description
`T`, derived format-version `BL=0`, public bit flags `BL`, optional template-style handle, typed Table
cell style, derived base identity, derived override count, then Title/Header/Data selectors, identities
and typed cell styles. The native identities/count/selectors are deterministic standard mappings and
are not snapshot fields; the complete fixture oracle is documented below.

`DwgCellStyle` order is type `BL`, data flags `BS`; conditional property-override mask `BL`, merge
flags `BL`, background `CMTC`, layout `BL`; content format `{value_data_type, value_unit_type,
format_string, rotation, block_scale, alignment, content_color, text_style, text_height}`; margin
override `BS` plus six conditional margins `BD`; border count `BL` bounded to six; each border has
index mask `BL` and conditional `{override_flags, type, color, lineweight, linetype, visible,
double_line_spacing}`. Handle order is optional template style, then each cell's content text style and
border line types in native Table/Title/Header/Data declaration order, after the common-object handles.

### Cross-object graph invariants and writer order

The typed bodies are not independently valid unless the fixture's dependency/action graph closes:

1. An associative network's owned action must resolve, and that action's `owning_network` must point
   back to the network. An unowned action cannot appear in the owned subset.
2. An action dependency must resolve to type 541/542/544 (or another standard dependency class) and
   its common object owner must be the action. Dependency action links must be mutually reciprocal;
   only then can link A/B be renamed previous/next.
3. A dependency's `dependent_on` must resolve to the object whose property/subentity it observes.
   Geometry dependencies additionally require a legal persistent-subentity class and compound-path
   flag. A dependency body is owned by exactly one dependency and its signed body ID agrees.
4. Constraint parameters' dependency handles must resolve to the matching block-parameter or
   dimension dependency body. Evaluation-expression parent/node IDs must resolve inside the named
   evaluation graph, and all connection-point/action IDs must be in range.
5. Common object main/string data is emitted first, then class main/string data in the tables above;
   common owner/reactor/extension-dictionary handles follow, then class handles in the exact order
   stated above. Counts, versions, duplicate roots and union discriminants are derived. No generic
   `referenced_handles` vector participates in class serialization.

### Code-ready implementation and acceptance sequence

1. Add typed body variants and the three shared cores first. Preserve the outer handle-keyed object
   ordering and class-list identity; validate class number/name/body agreement atomically.
2. Implement 506, 520 and 544 and require `{19,23,31}` exact attempted/decoded/re-encoded counts.
   These 73 frames exercise fixed properties, tagged EvalExpr values and the full dependency core.
3. Implement 539/541/542/545 together and run graph closure before serialization; require
   `{5,23,18,18}`. No type-541 cached/raw prefix and no type-545 encoded variant shadow state.
4. Implement 543/549, then the 48 dynamic frames whose fields are fully named. Keep 540, 504, 505,
   517 and the explicitly named uncertain scalars rejected until their gates above are resolved.
5. Extend the existing fixture test, not a new test file, with a per-class ledger totaling 238,
   main/string/handle reader exhaustion, body/class anti-mismatch rejection, reciprocal graph checks,
   one scalar and one handle mutation plus inverse in every shared core, and original-byte equality
   through DSL, pack, diff/apply/inverse/absorb, mutation/inverse, analyzer, composer and native
   serialization. A reduced body, ignored residual bit, unnamed scalar, opaque handle vector, or
   synthesized raw tail is a hard failure.

## Consolidated 663-frame acceptance ledger (2026-08-14)

This is the single progress ledger for the fixture's framed-object section. It deliberately does
not equate inventory identity, research completion, successful compilation, partial logical
projection, or whole-file source replay with frame acceptance.

A row is credited only when all four gates are green:

1. the live snapshot has a complete typed semantic body with no raw/source/physical/lexical tail;
2. the bounded AC1024 decoder exhausts main, string, and handle streams exactly;
3. the writer regenerates the same streams from that body, deriving counts/tags/versions;
4. an exact-frame assertion passes for every fixture instance of the row.

Status key: **green** = evidenced and credited; **WIP** = live work exists but is not credited;
**matrix** = field/handle prescription is documented but not live; **gate** = one or more standard
concepts still lack a semantic name; **reduced** = only a lossy drawing projection exists; **none**
= no typed implementation. The `remaining` column is therefore binary per row until per-instance
evidence exists; it never guesses partial success.

### Cohort totals

| cohort | fixture frames | credited exact frames | remaining | state |
|---|---:|---:|---:|---|
| fixed entities | 82 | 68 | 14 | LINE 40, ARC 12 and LWPOLYLINE 16 exact green; DIMENSION_LINEAR 14 remains |
| dictionary/XRECORD spine | 237 | 229 | 8 | XRECORD 145 and DICTIONARY/WDFLT 84 green; DICTIONARYVAR outstanding |
| block/entity graph | 32 | 0 | 32 | BLOCK/ENDBLK/INSERT matrices complete; live exact codecs outstanding |
| table/control/record graph | 59 | 59 | 0 | all nine controls and all 50 records, including DIMSTYLE 2, exact green |
| fixed support objects | 6 | 0 | 6 | matrices complete; live exact codecs outstanding |
| style/context custom classes | 50 | 0 | 50 | field matrices complete except explicit semantic gates |
| dynamic-block custom classes | 71 | 0 | 71 | field matrices complete except explicit semantic gates |
| associative custom classes | 126 | 0 | 126 | shared action/dependency matrix complete; constraint-group gate remains |
| **total** | **663** | **356** | **307** | **53.70% credited; 46.30% remaining** |

The 356 green frame assertions do not make the whole native DWG exact: the complete fixture export
remains blocked until all 663 rows are green and the object map/section/container writer is exact.

### Fixed entities — 14 remaining

| type/class | count | typed schema | decoder | writer | exact-frame test | remaining |
|---|---:|---|---|---|---|---:|
| 17 `ARC` | 12 | green | green | green | green 12/12 | 0 |
| 19 `LINE` | 40 | green | green | green | green 40/40 | 0 |
| 21 `DIMENSION_LINEAR` | 14 | exact-frame checklist | none | none | none | 14 |
| 77 `LWPOLYLINE` | 16 | green | green | green | green 16/16 | 0 |
| **subtotal** | **82** | | | | **68/82 credited** | **14** |

The next exact test for this cohort must report all four counts separately. The historical 18
projected geometries are not attributable as exact typed frames and receive no credit.

### Dictionary/XRECORD spine — 8 remaining

| type/class | count | typed schema | decoder | writer | exact-frame test | remaining |
|---|---:|---|---|---|---|---:|
| 79 `XRECORD` | 145 | green | green | green | green 145/145 | 0 |
| 42 `DICTIONARY` | 83 | green | green | green | green 83/83 | 0 |
| 500 `ACDBDICTIONARYWDFLT` | 1 | green | green | green | green 1/1 | 0 |
| 503 `DICTIONARYVAR` | 8 | matrix | none | none | none | 8 |
| **subtotal** | **237** | | | | **229/237** | **8** |

The DICTIONARY total communicated by the implementation lane is 84 and is split here into the
fixed DICTIONARY 83 plus the derived WDFLT 1 so the fixture census remains type-exact.

### Block/entity graph — 32 remaining

| type/class | count | typed schema | decoder | writer | exact-frame test | remaining |
|---|---:|---|---|---|---|---:|
| `BLOCK` | 10 | exact-frame checklist | none | none | none | 10 |
| `ENDBLK` | 10 | exact-frame checklist | none | none | none | 10 |
| `INSERT` | 12 | exact-frame checklist | none | none | none | 12 |
| **subtotal** | **32** | | | | | **32** |

### Table/control/record graph — complete, 0 remaining

All nine controls passed the scoped exact-frame route. This includes BLOCK_CONTROL plus the eight
symbol-table controls. The standard null entry slot exposed by BLOCK_CONTROL is an ordered optional
handle concept, not a filtered zero. Empty VIEW/UCS controls are typed empty ordered reference lists.

| type/class | count | typed schema | decoder | writer | exact-frame test | remaining |
|---|---:|---|---|---|---|---:|
| `BLOCK_CONTROL` | 1 | green | green | green | green 1/1 | 0 |
| `LAYER_CONTROL` | 1 | green | green | green | green 1/1 | 0 |
| `STYLE_CONTROL` | 1 | green | green | green | green 1/1 | 0 |
| `LTYPE_CONTROL` | 1 | green | green | green | green 1/1 | 0 |
| `VIEW_CONTROL` | 1 | green | green | green | green 1/1 | 0 |
| `UCS_CONTROL` | 1 | green | green | green | green 1/1 | 0 |
| `VPORT_CONTROL` | 1 | green | green | green | green 1/1 | 0 |
| `APPID_CONTROL` | 1 | green | green | green | green 1/1 | 0 |
| `DIMSTYLE_CONTROL` | 1 | green | green | green | green 1/1 | 0 |
| `BLOCK_HEADER` | 10 | green | green | green | green 10/10 | 0 |
| `LAYER` | 7 | green | green | green | green 7/7 | 0 |
| `STYLE` | 2 | green | green | green | green 2/2 | 0 |
| `LTYPE` | 3 | green | green | green | green 3/3 | 0 |
| `VIEW` records | 0 | matrix | n/a | n/a | n/a | 0 |
| `UCS` records | 0 | matrix | n/a | n/a | n/a | 0 |
| `VPORT` | 1 | green | green | green | green 1/1 | 0 |
| `APPID` | 25 | green | green | green | green 25/25 | 0 |
| `DIMSTYLE` | 2 | green | green | green | green 2/2 | 0 |
| **subtotal** | **59** | | | | **59/59 credited** | **0** |

`BLOCK_HEADER` is counted with the 50 record bodies because it shares the table-record body lane,
but retains semantic preview image data and ownership/reference collections; neither container bytes
nor a generic handle vector qualifies as its writer.

### Fixed support objects — 6 remaining

| type/class | count | typed schema | decoder | writer | exact-frame test | remaining |
|---|---:|---|---|---|---|---:|
| `VIEWPORT` entity | 2 | exact-frame checklist | none | none | none | 2 |
| `MLINESTYLE` | 1 | bounded exact-ready oracle; production pending | none | none | none | 1 |
| `ACDBPLACEHOLDER` | 1 | matrix | none | none | none | 1 |
| `LAYOUT` | 2 | matrix | none | none | none | 2 |
| **subtotal** | **6** | | | | | **6** |

### Style/context custom classes — 50 remaining

| type/class | count | typed schema | decoder | writer | exact-frame test | remaining |
|---|---:|---|---|---|---|---:|
| 504 `TABLESTYLE` | 1 | exact-frame checklist: R24 version + typed table-style flags + Table/Title/Header/Data styles | none | none | none | 1 |
| 505 `MATERIAL` | 3 | gate: verify R2010 extension presence | none | none | none | 3 |
| 506 `VISUALSTYLE` | 19 | exact-frame checklist: 28 typed property/modifier pairs | none | none | none | 19 |
| 507 `SCALE` | 17 | bounded exact-ready oracle; production pending | none | none | none | 17 |
| 508 `MLEADERSTYLE` | 1 | bounded exact-ready oracle; production pending | none | none | none | 1 |
| 516 `SORTENTSTABLE` | 7 | matrix | none | none | none | 7 |
| 517 `ACAD_EVALUATION_GRAPH` | 2 | exact-frame checklist: typed DAG; intrusive node/edge indexes derived | none | none | none | 2 |
| **subtotal** | **50** | | | | | **50** |

### Dynamic-block custom classes — 71 remaining

| type/class | count | typed schema | decoder | writer | exact-frame test | remaining |
|---|---:|---|---|---|---|---:|
| 520 `BLOCKGRIPLOCATIONCOMPONENT` | 23 | exact-frame checklist | none | none | none | 23 |
| 521 `BLOCKMOVEACTION` | 2 | matrix | none | none | none | 2 |
| 522 `ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION` | 2 | exact-frame checklist | none | none | none | 2 |
| 527 `BLOCKLINEARPARAMETER` | 2 | bounded exact-ready oracle; production pending | none | none | none | 2 |
| 528 `BLOCKLINEARGRIP` | 4 | bounded exact-ready oracle; production pending | none | none | none | 4 |
| 529 `BLOCKFLIPPARAMETER` | 3 | exact-frame checklist: updated-flip evaluation-node reference | none | none | none | 3 |
| 530 `BLOCKFLIPGRIP` | 3 | bounded exact-ready oracle; production pending | none | none | none | 3 |
| 531 `BLOCKVISIBILITYPARAMETER` | 1 | exact-frame checklist: typed evaluation-history policy | none | none | none | 1 |
| 532 `BLOCKVISIBILITYGRIP` | 1 | bounded exact-ready oracle; production pending | none | none | none | 1 |
| 533 `BLOCKALIGNMENTPARAMETER` | 2 | matrix | none | none | none | 2 |
| 534 `BLOCKALIGNMENTGRIP` | 2 | matrix | none | none | none | 2 |
| 535 `BLOCKSTRETCHACTION` | 6 | matrix | none | none | none | 6 |
| 536 `BLOCKSCALEACTION` | 1 | matrix | none | none | none | 1 |
| 537 `BLOCKFLIPACTION` | 3 | matrix | none | none | none | 3 |
| 538 `BLOCKBASEPOINTPARAMETER` | 1 | matrix | none | none | none | 1 |
| 546 `BLOCKVERTICALCONSTRAINTPARAMETER` | 1 | matrix | none | none | none | 1 |
| 547 `ACDB_DYNAMICBLOCKPROXYNODE` | 1 | exact-frame checklist | none | none | none | 1 |
| 548 `BLOCKHORIZONTALCONSTRAINTPARAMETER` | 1 | matrix | none | none | none | 1 |
| 559 `ACDB_BLOCKREPRESENTATION_DATA` | 12 | exact-frame checklist | none | none | none | 12 |
| **subtotal** | **71** | | | | | **71** |

### Associative custom classes — 126 remaining

| type/class | count | typed schema | decoder | writer | exact-frame test | remaining |
|---|---:|---|---|---|---|---:|
| 539 `ACDBASSOCNETWORK` | 5 | bounded exact-ready oracle; production pending | none | none | none | 5 |
| 540 `ACDBASSOC2DCONSTRAINTGROUP` | 4 | exact-frame checklist: validation policy, derived null slot, typed constraint-node union | none | none | none | 4 |
| 541 `ACDBASSOCVALUEDEPENDENCY` | 26 | exact-frame checklist | none | none | none | 26 |
| 542 `ACDBASSOCDEPENDENCY` | 20 | exact-frame checklist; dependency core only | none | none | none | 20 |
| 543 `BLOCKPARAMDEPENDENCYBODY` | 6 | exact-frame checklist | none | none | none | 6 |
| 544 `ACDBASSOCGEOMDEPENDENCY` | 31 | exact-frame checklist | none | none | none | 31 |
| 545 `ACDBASSOCVARIABLE` | 20 | exact-frame checklist | none | none | none | 20 |
| 549 `ASSOCDIMDEPENDENCYBODY` | 14 | exact-frame checklist | none | none | none | 14 |
| **subtotal** | **126** | | | | | **126** |

Type 541 consumes the dependency core plus the typed value-dependency specialization proven below;
type 542 remains dependency-core only unless bounded evidence proves another named concept. The
non-consuming LibreDWG `HANDLE_UNKNOWN_BITS` diagnostic is not a prefix and is prohibited from
schema/writer state.

### Ledger update protocol and next cohort

Every implementation handoff updates one row with an evidence log and per-instance numerator. A
row moves to green only as an atomic four-column transition; if 24 of 25 APPIDs pass, the row may
record `24/25` exact evidence and `remaining = 1`, but it cannot claim the cohort is complete. The
cohort subtotals and global `307` must be recomputed from row remaining counts on every update.

The next count-optimized queue is the 100-frame dependency/evaluation group documented in the
reconciled ranking below: types 542, 541 and 544 share one dependency core and contribute 77 exact
frames; type 520 reuses the typed evaluation-expression core and contributes the remaining 23.
LINE, ARC and LWPOLYLINE are already green and must not remain in any unresolved queue.

## AC1024 LINE exact-frame implementation checklist (2026-08-14)

The ticket-local read-only probe `🧪️dwg-line-frame-probe.py` independently decompressed the eight
`AcDb:AcDbObjects` pages and `AcDb:Handles`, decoded all valid object addresses, selected fixed type
19, and bounded the main and handle readers from each frame's R2010 `MC` value. It found exactly 40
LINE frames. It also verified the stored CRC of all 40 with seed `0xC0C1` over the contiguous
`MS + MC + payload` bytes. No production source or test was changed and Nx was not run.

### Authoritative logical body

The persisted body is only:

```text
DwgLineEntity {
    start: DwgPoint3,
    end: DwgPoint3,
    thickness: f64,
    extrusion: DwgVector3,
}
```

It is carried by the handle-keyed `DwgEntityFrame` beside the complete typed common entity. Native
selectors, `z_is_zero`, bit positions, frame sizes, padding, CRCs and encoded coordinates are writer
policy and must not enter snapshot/artifact/diff/mutation state. A separate flattened drawing LINE
projection cannot be an authority.

### Exact outer frame order

For every LINE, write and validate this order:

1. byte-aligned `MS payload_size`; all fixture values fit one 15-bit chunk, so this is two bytes;
2. byte-aligned unsigned modular-char `MC handle_stream_bits`; all values fit one byte;
3. payload data begins with BOT selector `00` plus raw type byte 19;
4. direct object handle `H` code 0; its value equals the handle-map key;
5. typed EED sequence and terminal zero-size `BS`;
6. common entity main bits in the next section;
7. LINE class bits in the declared compressed order;
8. one false string-stream-presence bit—LINE and the fixture common color branches own no strings;
9. common handles in their role order; LINE adds no class handles;
10. terminal **one-fill** included in `MC`, then little-endian `RS CRC` over `MS + MC + payload`.

All 40 have a three-byte outer prefix (`MS` two bytes + `MC` one byte) and two-byte CRC, hence total
frame size is `payload_size + 5`.

### Typed EED precondition

Twenty-five frames have only the zero-size EED terminator. Fifteen have one 17-byte typed EED record
before the terminator. Every such record references APPID handle `0x851` with code 5 and contains,
in order, group 1070 integer16 value 1, group 1071 integer32, and group 1005 object handle. The
integer32 histogram is `{0:3, 1:3, 2:3, 9:3, 10:3}`; the group-1005 value is the LINE's own handle.
Use the shared typed EED codec and preserve this ordered semantic collection; the 17 encoded bytes
are derived, not snapshot state.

### Common entity main-bit checklist and fixture histogram

After EED, consume/write every AC1024 common field in this exact order. The branch column is the
measured 40-frame oracle.

| order | logical concept / native encoding | fixture branch histogram |
|---:|---|---|
| 1 | entity graphic present `B`; if true, `BLL` length + parsed graphic | false 40; no graphic payload |
| 2 | placement/ownership mode `BB` | explicit owner mode 0: 18; model-space mode 2: 22; modes 1/3: 0 |
| 3 | reactor count `BL` | zero: 9 using selector 2; one: 31 using selector 1 |
| 4 | extension-dictionary-missing `B` | true 40; no xdictionary handle |
| 5 | typed entity color `ENC/BS` | ByLayer/index 256: 37 using BS selector 3; ByBlock/index 0: 3 using selector 2 |
| 6 | conditional transparency `BL` | absent 40 |
| 7 | conditional AcDbColor reference or RGB `BL` | both absent 40; no color/name/book string branches |
| 8 | linetype scale `BD` | value 1.0: 40 using BD selector 1, no payload |
| 9 | linetype selector `BB` | ByLayer 0: 37; ByBlock 1: 3; Continuous 2 / explicit 3: 0 |
| 10 | plot-style selector `BB` | ByLayer 0: 40 |
| 11 | material selector `BB` | ByLayer 0: 40 |
| 12 | shadow selector `RC` | 0: 40; no shadow reference |
| 13 | full visual-style present `B` | false 40 |
| 14 | face visual-style present `B` | false 40 |
| 15 | edge visual-style present `B` | false 40 |
| 16 | invisibility `BS` | value 0: 40 using selector 2 |
| 17 | lineweight `RC` | 29: 37; 30: 3 |

There is no AC1024 `nolinks` bit, previous/next handle, or R2013 data-store bit. Adding one shifts
the LINE class boundary. ENC optional ordering is transparency first, then either color reference or
RGB, with semantic color-name/book strings in the independent R2010 string stream when their flags
request them; the fixture takes none of those branches.

### Common handle roles and fixture graph

Handle stream order for a general AC1024 LINE is conditional AcDbColor, explicit owner for mode 0,
reactors, conditional extension dictionary, mandatory layer, conditional explicit linetype,
material, shadow, plot style, then full/face/edge visual styles. The class adds nothing.

The fixture exercises only three exact layouts:

| layout and encoded codes | frames | semantic resolution |
|---|---:|---|
| owner code 12, layer code 5 | 9 | explicit block owner + mandatory layer |
| owner code 12, reactor code 4, layer code 5 | 9 | explicit block owner + one reactor + layer |
| reactor code 4, layer code 5 | 22 | model-space placement, one reactor + layer; no owner handle is serialized |

Resolved owner histogram is `{0x110d:2, 0x1145:3, 0x195a:3, 0x1f57:2, 0x1fa4:2,
0x201e:3, 0x2077:3, none:22}`. Layer histogram is `{0x10:18, 0x83d:22}`. Reactor cardinality is
preserved exactly; a null or missing slot is an error, not filtered.

Measured handle count is two on 31 frames and three on 9. `MC` distribution is 37 bits on 6
frames, 39 on 3, 55 on 22, and 61 on 9. After the final semantic handle, the fixture fills the
remainder of the counted handle stream with ones: `11111` on 15 frames and `1111111` on 25. This
supersedes any generic zero-padding assumption for the exact AC1024 object writer.

### LINE class compression/default branches

Class-main order is immutable:

1. derive and write `z_is_zero B`;
2. start X `RD`, end X `DD(start X)`;
3. start Y `RD`, end Y `DD(start Y)`;
4. only when either logical Z is nonzero: start Z `RD`, end Z `DD(start Z)`;
5. thickness `BT0`;
6. extrusion `BE`.

All 40 fixture frames have `z_is_zero = 1`, thickness's one-bit default-zero branch, and
extrusion's one-bit default `(0,0,1)` branch. No Z `RD/DD`, thickness `BD`, or extrusion component
is present. Decode omitted Z as semantic zero and derive the same branch whenever both endpoints are
numerically zero; no persisted presence flag is needed.

The measured endpoint-delta selectors are:

| field | selector 0: exact predecessor | selector 1: low 4 bytes | selector 2: low 6 bytes | selector 3: full `RD` |
|---|---:|---:|---:|---:|
| end X from start X | 18 | 3 | 0 | 19 |
| end Y from start Y | 10 | 4 | 0 | 26 |
| end Z from start Z | absent 40 | 0 | 0 | 0 |

Writer DD policy compares the IEEE-754 little-endian bytes and chooses the shortest exact branch:
selector 0 when all eight bytes match; selector 1 when bytes 4-7 match and writes bytes 0-3;
selector 2 when bytes 6-7 match and writes bytes 4-5 followed by 0-3; otherwise selector 3 writes
the full `RD`. Reject nonfinite values. This deterministic priority reproduces all fixture choices
without storing selectors.

### Exact frame-size and handle-bit oracle

`data_bits` is `payload_size * 8 - MC`. `class_end` is the bit immediately after `BE`; every row has
exactly one remaining data bit, the false string-stream marker. The final column lists every fixture
handle in that signature, so failures can be localized without guessing.

| payload / total bytes | MC / data / class-end bits | semantic handles; terminal one-fill | fixture handles |
|---|---|---|---|
| 39 / 44 | 39 / 273 / 272 | owner, layer; 7 | `195e` |
| 42 / 47 | 55 / 281 / 280 | reactor, layer; 7 | `1f35 1f36 1f37 1f7b 1f7c 1f7d 1f81 1f82 1f86 1f87 1f88 21b1 222c 222d 2230` |
| 46 / 51 | 55 / 313 / 312 | reactor, layer; 7 | `1f38 1f7e 1f84 1f89 222e 222f 2231` |
| 47 / 52 | 39 / 337 / 336 | owner, layer; 7 | `195c 195d` |
| 60 / 65 | 37 / 443 / 442 | owner, layer; 5 | `1143 1f63 1f64` |
| 64 / 69 | 61 / 451 / 450 | owner, reactor, layer; 5 | `116c 116d 116e 2021 2022 2023 207a 207b 207c` |
| 68 / 73 | 37 / 507 / 506 | owner, layer; 5 | `1142 1fb0 1fb1` |

Aggregate payload histogram is `{39:1, 42:15, 46:7, 47:2, 60:3, 64:9, 68:3}` and aggregate total
frame histogram is `{44:1, 47:15, 51:7, 52:2, 65:3, 69:9, 73:3}`.

### Production and acceptance order

1. Complete the shared common-entity codec first, including graphic, full ENC, all selector bits,
   three R2010 visual-style bits, and role-specific handles. It must accept the measured branches
   above and reject selector/handle contradictions atomically.
2. Add `DwgLineEntity` to the authoritative typed body union and retire any independent persisted
   flattened LINE projection. Extend the existing schema facets/codecs in the same change.
3. Decode type 19 with independent bounded main/string/handle readers. Assert BOT/handle-map match,
   typed EED exhaustion, `class_end + 1 == data_bits`, false string marker, exact semantic handle
   exhaustion and the measured one-fill.
4. Implement `RD/DD/BT/BE` writer branches with the deterministic priority above. Write common
   handles before no class handles, calculate `MC` including one-fill, then derive MS and CRC.
5. Extend the existing fixture test, not a new test file, to assert the seven signature groups and
   40/40 original-frame byte equality. A frame test compares `MS + MC + payload + CRC`, not merely
   decoded geometry or payload length.
6. Exercise one endpoint/DD-selector mutation, one default-to-explicit thickness or extrusion
   mutation, and one common role mutation, then inverse each exactly. Finally require original DWG
   equality through DSL, pack, diff/apply/inverse/absorb, mutation/inverse, analyzer, composer and
   native serialization. Only then change the ledger LINE row from 40 remaining to zero.

## AC1024 ARC and LWPOLYLINE exact-frame checklists (2026-08-14)

The read-only ticket probe `🧪️dwg-arc-lwpolyline-frame-probe.py` reuses the verified LINE section,
frame, EED and common-entity primitives. It found exactly 12 fixed type-17 ARC and 16 fixed type-77
LWPOLYLINE frames and verified every stored CRC with seed `0xC0C1` over `MS + MC + payload`. No
production source or test was changed and Nx was not run.

### Shared outer/common contract for these 28 frames

All 28 use BOT selector 0, direct object-handle code 0, a two-byte MS plus one-byte MC prefix, no
entity graphic, explicit-owner entity mode 0, zero reactors encoded by BL selector 2, missing
extension dictionary, linetype scale 1 encoded by BD selector 1, plot style 0, material 0, shadow 0,
no full/face/edge visual styles, no common color reference/RGB/transparency/name/book strings, and no
class-specific handles. After the class body, every frame has exactly one false string-stream bit.

The cohort-specific common branches are:

| common concept | ARC 12 | LWPOLYLINE 16 |
|---|---|---|
| color | ByLayer/index 256 via BS selector 3: 12 | ByLayer/index 256 via selector 3: 15; ByBlock/index 0 via selector 2: 1 |
| linetype selector | ByLayer 0: 12 | ByLayer 0: 15; ByBlock 1: 1 |
| invisibility | 0 via BS selector 2: 3; 1 via selector 1: 9 | 0 via selector 2: 4; 1 via selector 1: 12 |
| lineweight | 29: 12 | 29: 16 |
| handle layout | owner code 12, layer code 5: 12 | owner code 12, layer code 5: 15; owner code 8, layer code 5: 1 |
| terminal MC fill | six ones: 12 | four ones: 16 |

ARC owners resolve to `{0x110d:4, 0x1f57:4, 0x1fa4:4}`. LWPOLYLINE owners resolve to
`{0x110d:5, 0x1f57:5, 0x1fa4:5, 0x238:1}`. Every layer is `0x10`. The type-77 `0x239` frame uses
relative previous-handle code 8 for owner `0x238`; preserving only the resolved handle is semantic,
and the deterministic shortest relative-handle writer must recover code 8.

ARC always has one 17-byte EED record via APPID `0x851`: group 1070 integer16 1, group 1071
integer32, then group 1005 self handle. The integer32 histogram is `{5:3, 6:3, 7:3, 8:3}`.
LWPOLYLINE has that same typed record on 15 frames with integer32 histogram
`{0:3, 1:3, 2:3, 3:3, 4:3}`; handle `0x239` has only the terminal zero-size EED. EED values remain
typed logical concepts, never retained 17-byte payloads.

### ARC logical body and exact writer order

Persist:

```text
DwgArcEntity {
    center: DwgPoint3,
    radius: f64,
    thickness: f64,
    extrusion: DwgVector3,
    start_angle: f64,
    end_angle: f64,
}
```

After common main bits, write center X/Y/Z as three `BD`, radius `BD`, thickness `BT0`, extrusion
`BE`, start angle `BD`, then end angle `BD`. ARC owns no string or handle. Do not normalize or swap
angles; preserve their semantic radian values.

Measured branches:

| field | branch | frames / value |
|---|---|---|
| center X/Y/Z | BD selector 2 | 12 each / all zero |
| radius | BD selector 0 full RD | 12 / all exactly 30 |
| thickness | BT default bit 1 | 12 / zero; no following BD |
| extrusion | BE default bit 1 | 12 / `(0,0,1)`; no component BD |
| start angle | BD selector 2 / selector 0 | zero branch 2; full RD 10 |
| end angle | BD selector 0 | 12 |

Writer selects BD 2 for exact semantic zero, BD 1 for exact one, otherwise BD 0 plus RD; BT and BE
choose their one-bit standard defaults from the logical values. Reject nonfinite values, negative
radius and a zero extrusion vector. No selector/default state belongs in the snapshot.

#### ARC exact frame signatures

All have `MC=38`, common handles `(owner,layer)`, one false main-tail bit, and six one-fill handle
bits.

| payload / total bytes | data / class-end bits | fixture handles |
|---|---|---|
| 53 / 58 | 386 / 385 | `1141` |
| 54 / 59 | 394 / 393 | `113e` |
| 61 / 66 | 450 / 449 | `1f62 1faf` |
| 62 / 67 | 458 / 457 | `113f 1140 1f5f 1f60 1f61 1fac 1fad 1fae` |

Payload histogram is `{53:1, 54:1, 61:2, 62:8}`; total frame histogram is
`{58:1, 59:1, 66:2, 67:8}`.

### LWPOLYLINE logical body and invariant-bearing vectors

Persist a typed body, not flattened coordinates:

```text
DwgLwPolylineEntity {
    closed: bool,
    constant_width: Option<f64>,
    elevation: f64,
    thickness: f64,
    extrusion: DwgVector3,
    vertices: Vec<DwgPoint2>,
    bulges: Vec<f64>,
    vertex_ids: Vec<u32>,
    widths: Vec<DwgVertexWidth>,
}
```

`DwgVertexWidth` is the semantic `(start_width,end_width)` pair for the same-position vertex.
Parallel collections retain standard meaning and order; they are not interleaved native bytes or a
generic numeric vector.

Exact class-main writer order:

1. derive flags and write `BS`;
2. conditional constant width `BD` for flag 4, elevation `BD` for 8, thickness `BD` for 2,
   extrusion X/Y/Z `BD` for 1;
3. derived vertex count `BL`;
4. conditional bulge count `BL` for 16, vertex-ID count `BL` for 1024, width count `BL` for 32;
5. first vertex X/Y as `2RD`, then each later X/Y as `DD` from the immediately preceding vertex;
6. all bulges `BD`, all vertex IDs `BL`, then all start/end width `BD` pairs;
7. false string marker, common owner/layer handles, four one-fill bits.

The closed flag is 512. Counts/presence flags are derived from semantic fields. If bulges,
vertex IDs or widths are present, their lengths must equal the vertex count; an empty optional
collection emits no count field. The standard maximum of 20,000 vertices is enforced before
allocation or writing.

#### Fixture branch histogram

| concept | measured branches |
|---|---|
| flags | 512 (closed only): 15 using BS selector 0; 4 (constant width only): 1 using selector 1 |
| vertices | four: 15; two: 1; every count uses BL selector 1 |
| constant width | absent: 15; full BD selector 0 value 0.15: one (`0x239`) |
| elevation / thickness / extrusion | all standard defaults, flags absent: 16 |
| bulges / vertex IDs / per-vertex widths | empty with flags/counts absent: 16 |
| closed | true: 15; false: one (`0x239`) |

Across the 46 predecessor-compressed X coordinates, DD selector histogram is
`{0:3, 1:1, 2:0, 3:42}`. Across 46 Y coordinates it is `{0:3, 1:3, 2:3, 3:37}`. Use the same
byte-exact shortest DD priority as LINE: exact predecessor, low four bytes, low six bytes, then full
RD. The first vertex is always two full RDs.

The exceptional `0x239` body is open with vertices `(-0.5,-0.5)` and `(0.5,0.5)` and constant width
0.15. The other 15 bodies are closed four-vertex polylines; closed serialization does not append a
duplicate first vertex.

#### LWPOLYLINE exact frame signatures

All rows have `(owner,layer)`, one false main-tail bit and four one-fill handle bits.

| payload / total bytes | MC / data / class-end bits | fixture handles |
|---|---|---|
| 56 / 61 | 28 / 420 / 419 | `239` |
| 81 / 86 | 36 / 612 / 611 | `1139` |
| 93 / 98 | 36 / 708 / 707 | `1f5a 1fab` |
| 97 / 102 | 36 / 740 / 739 | `1fa7` |
| 101 / 106 | 36 / 772 / 771 | `113d 1f5e` |
| 103 / 108 | 36 / 788 / 787 | `113c 1f5d 1faa` |
| 104 / 109 | 36 / 796 / 795 | `113a 1f5b 1fa8` |
| 105 / 110 | 36 / 804 / 803 | `113b 1f5c 1fa9` |

Payload histogram is `{56:1, 81:1, 93:2, 97:1, 101:2, 103:3, 104:3, 105:3}`; total frame
histogram is `{61:1, 86:1, 98:2, 102:1, 106:2, 108:3, 109:3, 110:3}`.

### Production and acceptance sequence

1. Land these bodies only on the complete common-entity/frame foundation documented for LINE;
   otherwise invisibility, ENC, owner and terminal-fill mismatches make class-body results invalid.
2. Decode each class with separately bounded main/string/handle readers and require
   `class_end + one false string bit == data_bits`, exact common handle exhaustion and the measured
   terminal one-fill. Reject any residual or contradictory flag/count/vector state.
3. Implement symmetric selectors/defaults entirely from logical values. Rebuild `MS`, `MC`, CRC,
   frame offsets and handle map from emitted frames; do not persist any signature-table value.
4. Extend the existing fixture test with the four ARC and eight LWP signature groups and exact
   `MS + MC + payload + CRC` assertions for 12/12 and 16/16 frames.
5. Mutate/inverse ARC radius, angle, default-to-explicit thickness/extrusion and visibility. For
   LWP, mutate/inverse closure, one predecessor-compressed vertex, constant width, and a parallel
   bulge/width collection while enforcing length invariants.
6. Require original DWG equality through DSL, pack, diff/apply/inverse/absorb, mutation/inverse,
   analyzer, composer and native serialization. Only then set ARC and LWPOLYLINE ledger remaining
   counts to zero.

## AC1024 INSERT, DIMENSION_LINEAR and VIEWPORT exact-frame checklists (2026-08-14)

The read-only `🧪️dwg-insert-dimension-viewport-frame-probe.py` reuses the bounded object/common/EED
primitives from the LINE and ARC/LWP probes. It found exactly 12 fixed type-7 INSERT, 12 fixed
type-21 DIMENSION_LINEAR and two fixed type-34 VIEWPORT frames. Every CRC was verified with seed
`0xC0C1` over `MS + MC + payload`. No production source or test was changed and Nx was not run.

### INSERT — 12 frames

Persist `DwgInsertEntity { insertion, scale, rotation, extrusion, block_header, attributes,
sequence_end }` on the handle-keyed entity. Attribute count, `has_attributes`, scale selector and
native default selectors are derived.

#### Measured common and class branches

All twelve use BOT selector 0/direct object handle, no EED, no graphic, model-space mode 2, zero
reactors via BL selector 2, **present extension dictionary**, ByLayer color 256 via BS selector 3,
linetype scale 1 via BD selector 1, ByLayer linetype/plot/material, shadow 0, no visual styles,
invisibility 0 via BS selector 2 and lineweight 29.

Class-main order and branches:

1. insertion X/Y/Z `3BD`: selectors `(0,0,2)` on all 12;
2. scale selector `BB`: value 3 on all 12, deriving scale `(1,1,1)` and no scale payload;
3. rotation `BD`: selector 0 full RD on all 12;
4. extrusion `3BD`: selectors `(2,2,1)`, deriving `(0,0,1)`;
5. has-attributes `B`: false on all 12, so no owned-count `BL`;
6. false string-stream bit;
7. common extension dictionary code 3, layer code 5, class block-header code 5;
8. five terminal one-fill bits.

All twelve are one exact signature: payload 45 bytes, total frame 50 bytes, `MC=77`, data end bit
283 and class end bit 282. Handles are
`1f3d 1f8a 1fdd 1ff7 2011 206a 20a7 20b4 20c1 20ce 20db 20e8`.

Resolved layer histogram is `{0x83f:4, 0x843:8}`. Block-header histogram is
`{0x1f57:1, 0x1fa4:3, 0x201e:5, 0x2077:3}`. Every extension dictionary is a distinct non-null
object owned by its INSERT. Require reciprocal block-header INSERT backrefs.

#### General symmetric INSERT writer

Choose scale selector in priority order: exact all-one → 3; all three bitwise equal → 2 plus X RD;
X exact one → 1 plus Y/Z DD from one; otherwise 0 plus X RD and Y/Z DD from X. Then write rotation,
extrusion, attribute flag/count. After common handles always emit block-header; if attributes are
nonempty, emit exactly the count of code-4 attribute handles followed by one code-3 SEQEND. Require
empty attributes iff sequence_end is absent and verify every common owner/backref. The fixture takes
the empty branch, but the typed decoder/writer must implement both without raw tails.

### DIMENSION_LINEAR — 12 frames

Persist the typed body already specified above, with `dimension_block` optional. Native class
version, `flag1`, reserved bit, string-stream size and compact selectors are derived.

#### Common, string and handle branches

All twelve have no EED/graphic, model-space mode 2, one reactor encoded by BL selector 1, missing
extension dictionary, ByLayer color/linetype/plot/material, linetype scale 1, shadow 0, no visual
styles, invisibility 0 and lineweight 29. `MC=87` on all frames. Handle order is reactor code 4,
layer code 5, dimension-style code 5, dimension-block code 5, then seven one-fill bits.

Every layer is `0x1b81`; every dimension style is `0x242`. The reactor is distinct per dimension.
The dimension-block slot is physically present but null on all twelve. Schema therefore uses an
optional reference; writer still emits the null code-5 slot. Only a non-null value must resolve to a
BLOCK_HEADER.

Each frame has one user-text `T` in the independent string stream. The twelve semantic values are:

```text
Room1=12'-0"   Room2=12'-0"   Room3=12'-0"   hall=12'-0"
Wall2=bldWALL  Wall3=bldWALL  bldDEPTH=50'-0"  bldWALL=6"
iWALL1=6"      iWALL2=iWALL1 iWALL3=iWALL1    iWALL4=iWALL1
```

String-bit histogram is `{154:1, 170:1, 186:1, 202:3, 218:5, 250:1}`. Encode the TU value from the
semantic string, then its RS bit size and true presence bit; no imported UTF-16 or size state is
retained.

#### Class-main branch oracle

| field | fixture branch |
|---|---|
| R2010 class version RC | 0 on 12 |
| extrusion `3BD` | selectors `(2,2,1)` = `(0,0,1)` on 12 |
| text midpoint `2RD` | two full RDs on 12 |
| elevation `BD` | selector 2 = 0 on 12 |
| derived flag1 RC | `0x09` on 12 |
| text rotation / horizontal direction | BD selector 2 = 0 on 12 each |
| insertion scale `3BD_1` | selectors `(1,1,1)` = `(1,1,1)` on 12 |
| insertion rotation | BD selector 2 = 0 on 12 |
| attachment `BS` | value 5 via selector 1 on 12 |
| line-space style `BS1` | value 1 via selector 1 on 12 |
| line-space factor `BD1` | value 1 via selector 1 on 12 |
| actual measurement `BD` | selector 0 full RD on 12 |
| reserved / flip-arrow bits | false / false / false on 12 |
| clone insertion point `2RD0` | two full RDs on 12 |
| extension-line 1/2 and definition point `3BD` | selectors `(0,0,2)` on 12 each |
| oblique angle | BD selector 2 = 0 on 12 |
| dimension rotation | BD selector 0 full RD: 9; selector 2 zero: 3 |

`flag1` is not snapshot state. For AC1024 derive
`0x08 | (status & 0xe0) | (status bit 7 clear ? 1 : 0) | (status bit 5 set ? 2 : 0)`; all fixture
statuses yield `0x09`. Reject impossible inverse combinations. The fixed linear subtype comes from
type 21, not a stored low-nibble flag.

#### DIMENSION_LINEAR exact signatures

All rows use `MC=87`, the four semantic handle slots above and seven one-fill bits.

| payload / total | data / class-end / string bits | fixture handles |
|---|---|---|
| 145 / 150 | 1073 / 870 / 186 | `2250` |
| 149 / 154 | 1105 / 870 / 218 | `2128 21ea` |
| 149 / 154 | 1105 / 934 / 154 | `2151` |
| 151 / 156 | 1121 / 934 / 170 | `211d` |
| 155 / 160 | 1153 / 934 / 202 | `2156 2161 2177` |
| 157 / 162 | 1169 / 934 / 218 | `2122 215b 2166` |
| 161 / 166 | 1201 / 934 / 250 | `2107` |

For every row, `data_bits = class_end + string_bits + 17` (RS size plus presence bit). Payload
histogram is `{145:1,149:3,151:1,155:3,157:3,161:1}`.

### VIEWPORT — two frames

Persist the typed viewport body from the fixed-entity matrix. Counts, compact selectors, status
encoding, CMC packing, native string sizing and nullable handle slots are derived.

#### Measured common and string state

Both frames have no EED/graphic, paper-space mode 1, zero reactors via BL selector 2, present
extension dictionary, ByLayer color/linetype/plot/material, linetype scale 1, shadow 0, no common
visual styles, invisibility 0 and lineweight 29. There is no common owner handle in mode 1.

Style sheet is semantically empty on both, but the declared `T` still produces a present two-bit
string stream (BS selector 2 encodes zero UTF-16 units), followed by the 16-bit size and true
presence bit. An empty string is not permission to omit this stream. Ambient color flag 0 adds no
name/book strings.

#### Class-main branch oracle

Exact order is center `3BD`, width/height `BD`, target and direction `3BD`, twist/view-height/lens/
front/back/snap-angle `BD`, four `2RD` pairs, circle zoom and grid major `BS`, frozen count and status
flags `BL`, style-sheet `T`, render mode `RC`, UCS flags/vectors/elevation/view/shade mode, lighting
flags/brightness/contrast and ambient `CMC`.

| field | two-frame branch |
|---|---|
| center | BD selectors `(0,0,2)` |
| width / height | selector 0 full RD |
| view target | `(2,2,2)` = zero |
| view direction | `(2,2,1)` = `(0,0,1)` |
| twist / front clip / back clip / snap angle | selector 2 = zero |
| view height / lens length | selector 0 full RD; lens is 50 on both |
| view center, snap base, snap unit, grid unit | full `2RD`; snap base zero, units `(0.5,0.5)` |
| circle zoom | 1000 via BS selector 0 |
| grid major | 5 via BS selector 1 |
| frozen layers | count 0 via BL selector 2; no vector |
| status flags | 819232 (`0x28b`) and 557152 (`0x290`), BL selector 0 |
| render mode | 0 |
| UCS at origin / per viewport | false / true |
| UCS origin / X / Y | `(2,2,2)`, `(1,2,2)`, `(2,1,2)` |
| UCS elevation / orthographic view / shade mode | zero via BD/BS selector 2 |
| default lights / lighting type | true / 1 |
| brightness / contrast | zero via BD selector 2 |
| ambient CMC | semantic RGB `#333333`, method ByColor (`0xc2`), flag 0; index BS2, RGB BL0 |

CMC schema stores the typed method/RGB and optional names, not packed `0xc2333333`; the writer
derives that BL word and flag byte.

#### VIEWPORT handle slots and signatures

After common extension dictionary and layer, emit zero frozen-layer refs, then clip boundary code 5,
named UCS 5, base UCS 5, background 4, visual style 5, shade plot 4 and sun 3. These seven slots are
native fields even when null. In both frames only visual style is non-null (`0x9f`); all other class
slots are zero. Both end with seven one-fill bits.

| handle | payload / total | MC / data / class-end / string bits | xdictionary / layer |
|---|---|---|---|
| `28b` | 161 / 166 | 111 / 1177 / 1158 / 2 | `0x28c` / `0x10` |
| `290` | 162 / 167 | 119 / 1177 / 1158 / 2 | `0x291` / `0x1b4` |

The semantic main data is identical in bit length; the one-byte payload/MC difference is solely
handle encoding length and must be recovered by the deterministic relative/absolute handle writer.

### Production and acceptance sequence

1. Use the complete common entity/frame writer, including present extension dictionaries, paper/
   model placement, typed color, independent strings and one-fill. Do not add obsolete nolinks or
   previous/next handles.
2. Add three tagged typed bodies and corresponding facets/codecs. Nullable standard handle slots are
   typed `Option<Handle<...>>`; a null value is serialized in its declared slot, not omitted.
3. Bound main, string and handle readers independently. Require main end to equal string start,
   exact TU/CMC string exhaustion, exact role-handle exhaustion and measured terminal one-fill.
4. Extend the existing fixture test with 12/12 INSERT (one signature), 12/12 DIMENSION_LINEAR (seven
   signatures) and 2/2 VIEWPORT (two signatures) exact `MS + MC + payload + CRC` assertions.
5. Mutation/inverse coverage: INSERT scale branch and block ref; dimension user text, measurement,
   rotation branch and nullable block slot; viewport status, empty/nonempty style sheet, frozen-layer
   vector and nullable/non-null visual roles.
6. Require original native equality through DSL, pack, diff/apply/inverse/absorb, mutation/inverse,
   analyzer and composer before reducing the ledger's 12 + 12 + 2 remaining counts.

## AC1024 BLOCK and ENDBLK exact-frame checklists (2026-08-14)

Ticket probe: `🧪️dwg-block-endblk-frame-probe.py`. It decodes every type-4/type-5 frame from the
fixture's logical Objects pages and validates every native CRC. LibreDWG's primary `dwg.spec`
declares the R13+ BLOCK body as the block name followed by common entity handle data, and ENDBLK as
common entity handle data only. The measured AC1024 split streams confirm that declaration without
an opaque remainder.

### Census closure

The fixture object map contains exactly these fixed entity-tag codes:

| fixed type | class | count |
|---:|---|---:|
| 4 | BLOCK | 10 |
| 5 | ENDBLK | 10 |
| 7 | INSERT | 12 |
| 17 | ARC | 12 |
| 19 | LINE | 40 |
| 21 | DIMENSION_LINEAR | 12 |
| 34 | VIEWPORT | 2 |
| 77 | LWPOLYLINE | 16 |
| **total** | | **114** |

Therefore there is no additional fixed LINE/geometry/entity family hidden behind the earlier
projection inventory. After the LINE, ARC, LWPOLYLINE, INSERT, DIMENSION_LINEAR and VIEWPORT
checklists, the exact remaining fixed entity bodies requested here are precisely BLOCK 10 and
ENDBLK 10. Dynamic class types remain separate ledger cohorts and must not be folded into this
fixed-type result.

### Shared marker entity main data

Both variants use the normal AC1024 entity prefix and the complete common entity main sequence:
graphic flag/payload, entity mode, reactor count, extension-dictionary-missing bit, CMC color,
linetype scale, linetype/plotstyle/material selectors, shadow flags, full/face/edge visual-style
flags, invisibility and lineweight. The fixture branch histogram is identical across both 10-frame
sets:

- BOT selector 0 and object-handle code 0 on 10/10; no EED and no graphic payload;
- entity mode 0 on eight, paper-space mode 1 on one and model-space mode 2 on one;
- zero reactors via BL selector 2; no extension dictionary;
- ByLayer color index 256 via BS selector 3; linetype scale 1 via BD selector 1;
- linetype, plotstyle and material selectors 0; shadow 0 and no visual styles;
- invisibility 0 via BS selector 2 and lineweight 29.

The main reader ends at bit 73 for the eight mode-0 objects and bit 65 for the two implicit-space
objects. These are absolute payload positions after BOT/object handle/common data, not persisted
layout. The writer derives them from compact BOT/handle/common encodings.

Common handle order is owner only when entity mode is zero, then layer. All ten layers resolve to
`0x10`. The model-space and paper-space marker pairs omit the owner slot and are semantically owned
by their entity mode. The other eight pairs carry the same explicit BLOCK_HEADER owners:
`0x238 0x110d 0x1145 0x195a 0x1f57 0x1fa4 0x201e 0x2077`. Require exactly one BLOCK and one
ENDBLK per owner identity, where mode 1/2 are two distinct implicit identities.

### BLOCK — 10 frames

Typed logical body: one Unicode block name. The ten values are `*Model_Space`, `*Paper_Space`,
`_ArchTick`, `Door - Imperial`, `Window - Imperial`, `_ClosedBlank`, `*U4`, `*U5`, `*U6` and `*U7`.
No BLOCK scalar is present in the main stream. The name is one `T` value in the independent R2010
string stream; all ten have a true presence bit. Persist the semantic name once. If BLOCK_HEADER is
the aggregate source of truth, decoding must validate marker name equality and the writer derives
this native duplicate from that header rather than retaining a second wire/layout value.

String-bit histogram is `{58:4,154:1,202:3,250:1,282:1}`. Encode TU from the semantic string, append
the RS string-bit count and true presence bit. Native string bit count and presence are derived.

| payload / total | MC / data / class-end / string bits | fixture handles |
|---|---|---|
| 22 / 27 | 28 / 148 / 73 / 58 | `1f58 1fa5 201f 2078` |
| 35 / 40 | 36 / 244 / 73 / 154 | `23a` |
| 38 / 43 | 20 / 284 / 65 / 202 | `20 5a` |
| 40 / 45 | 28 / 292 / 73 / 202 | `195b` |
| 47 / 52 | 36 / 340 / 73 / 250 | `1138` |
| 51 / 56 | 36 / 372 / 73 / 282 | `116b` |

For every row, `data_bits = class_end + string_bits + 17`. Handle layouts are owner/layer on eight
and layer only on two; code histograms are `(8,5)` five, `(12,5)` three and `(5)` two. Four one-fill
bits terminate every BLOCK payload.

Per-frame native CRC oracle after seed-`0xC0C1` CRC-16 over contiguous `MS + MC + payload`:
`20:bb32 5a:fe36 23a:cdca 1138:6f49 116b:e4cf 195b:2ae5 1f58:1de9 1fa5:1ac5
201f:3505 2078:9503`.

### ENDBLK — 10 frames

Typed logical body is an explicit empty ENDBLK variant; it has no class scalar, string or class
handle. After common main, write a false independent-string presence bit, then the common handles.
Do not persist the false bit as state and do not substitute a generic empty object.

| payload / total | MC / data / class-end / string bits | fixture handles |
|---|---|---|
| 11 / 16 | 22 / 66 / 65 / 0 | `21 5b` |
| 14 / 19 | 38 / 74 / 73 / 0 | `23b 1144 116f 195f 1f59 1fa6 2020 2079` |

Here `data_bits = class_end + 1` for the false string-presence bit. Handle layouts are owner/layer
on eight and layer only on two. Explicit owners use code 12 and layers code 5; the implicit-space
rows contain only layer code 5. Six one-fill bits terminate every ENDBLK payload.

Per-frame CRC oracle:
`21:d3ad 5b:39a4 23b:9d86 1144:15e8 116f:51cc 195f:de08 1f59:0b5e 1fa6:2394
2020:e0d4 2079:7c9c`.

### Symmetric decoder/writer and acceptance gates

1. Add tagged `BlockBegin` and `BlockEnd` entity bodies, not a name bag or unsupported frame. Keep
   placement/owner/layer in the shared entity aggregate and derive native mode/handle branches.
2. Decode main, strings and handles with independent bounds. BLOCK must consume exactly one TU and
   exhaust its string slice; ENDBLK must declare no string slice. Main end must equal string start.
3. Resolve the ten semantic owner identities and enforce reciprocal BLOCK_HEADER begin/end slots.
   Mode 1/2 omit native owner handles; mode 0 requires exactly one typed BLOCK_HEADER owner.
4. Writer order is BOT, object handle, empty EED, common main, BLOCK name string or ENDBLK false
   string bit, common owner when applicable, layer, terminal one-fill, then CRC. The generic handle
   encoder chooses canonical relative code 8/12 or absolute code from semantic target distance.
5. Extend the existing fixture test with 10/10 BLOCK and 10/10 ENDBLK exact
   `MS + MC + payload + CRC` assertions across all signatures above. Also assert all 20 CRCs,
   terminal-fill widths/patterns, exact owner pairing and absence of unread main/string/handle bits.
6. Mutation/inverse coverage changes a header name and proves its derived BLOCK TU changes and
   restores exactly; moves an explicit marker pair between headers; and rejects orphan, duplicate,
   mismatched begin/end and invalid implicit-space ownership atomically.
7. Credit these ledger rows only after exact native equality also survives logical DSL/pack,
   diff/apply/inverse/absorb, mutation/inverse, analyzer and composer. No native frame or string
   layout state may enter the snapshot.

## AC1024 custom types 506, 520 and 544 exact-frame checklists (2026-08-14)

Ticket probe: `🧪️dwg-custom-506-520-544-frame-probe.py`. It decodes all 73 fixture frames through
bounded main, independent string and handle streams and validates every CRC. Field names/order come
from LibreDWG `dwg2.spec` definitions for `VISUALSTYLE`, `AcDbEvalExpr`/
`AcDbBlockGripExpr`, `AcDbAssocDependency` and `AcDbAssocPersSubentId`. That source is research
evidence, not a runtime dependency. The all-frame boundary/CRC result closes the earlier candidate
layouts without introducing raw prefixes or tails.

### Shared custom-object frame order

All 73 use BOT selector 1, object-handle code 0 and empty EED. Common object main data is reactor
count `BL`, then extension-dictionary-missing `B`. Common handles are owner, ordered reactors and
optional extension dictionary, followed by class handles. Then comes terminal one-fill and the
CRC-16 seeded `0xC0C1` over contiguous `MS + MC + payload`.

Every frame has a true R2010 string stream. For every signature below:
`data_bits = class_end + string_bits + 17`, where the 17 derived bits are the RS string-size and
presence bit. Neither native size, selector, presence nor terminal fill is snapshot state.

### Type 506 `VISUALSTYLE` — 19 frames

Typed body is the description, style type, extension-lighting model, internal-only flag and a fixed
named record of 28 properties. Each property is immediately followed by its typed operation/
modifier `BS`. Property order is:

1. face lighting model `BL`; 2. face lighting quality `BL`; 3. face color mode `BL`; 4. face
   modifier `BS`; 5. face opacity `BD`; 6. face specular `BD`; 7. face monochrome color `CMC`;
8. edge model `BL`; 9. edge style `BL`; 10. intersection color `CMC`; 11. obscured color `CMC`;
12. obscured linetype `BL`; 13. intersection linetype `BL`; 14. crease angle `BD`; 15. edge
   modifier `BL`; 16. edge color `CMC`; 17. edge opacity `BD`; 18. edge width `BL`; 19. edge
   overhang `BL`; 20. edge jitter `BL`; 21. silhouette color `CMC`; 22. silhouette width `BL`;
23. halo gap `BL`; 24. isolines `BL`; 25. hide precision `B`; 26. display settings `BL`;
27. display brightness `BD`; 28. display shadow type `BL`.

Description is the sole string on all 19 frames. The semantic set is `2dWireframe`, `3D Hidden`,
`3dWireframe`, `Basic`, `Brighten`, `ColorChange`, `Conceptual`, `Dim`, `EdgeColorOff`,
`Facepattern`, `Flat`, `FlatWithEdges`, `Gouraud`, `GouraudWithEdges`, `JitterOff`, `Linepattern`,
`OverhangOff`, `Realistic`, `Thicken`. Style types cover `0..9`, `11..16` and `20..22`; selector
histogram is BL1 on 18 and BL2 zero on one. Extension lighting is 2 via BS1 on all; internal-only is
true 14/false 5.

Fixture property/selector oracle:

| property | semantic values (count) | value selector |
|---|---|---|
| face lighting model | `2:14,1:2,0:2,3:1` | BL1 17, BL2 2 |
| face lighting quality | `2:16,1:2,0:1` | BL1 18, BL2 1 |
| face color mode | `1:14,0:3,2:1,3:1` | BL1 16, BL2 3 |
| face modifier | `0:15,2:4` | BS2 15, BS1 4 |
| face opacity / specular | `0.6:19` / `30:19` | BD0 / BD0 on 19 |
| edge model / style | `1:14,0:3,2:2` / `4:16,2:2,0:1` | BL1 16+BL2 3 / BL1 18+BL2 1 |
| obscured / intersection linetype | `1:17,2:1,7:1` / `1:18,7:1` | BL1 on 19 each |
| crease angle | `1:17,40:2` | BD1 17, BD0 2 |
| edge modifier | `8:10,0:6,12:1,10:1,9:1` | BL1 13, BL2 6 |
| edge opacity / width / overhang / jitter | `1 / 1 / 6 / 2` on 19 | BD1 / BL1 / BL1 / BL1 |
| silhouette width | `5:17,3:2` | BL1 on 19 |
| halo / isolines / hide precision | `0 / 0 / false` on 19 | BL2 / BL2 / B |
| display settings | `1:18,13:1` | BL1 on 19 |
| brightness | `0:17,-50:1,50:1` | BD2 17, BD0 2 |
| shadow type | `0:19` | BL2 on 19 |

All five CMC properties have index zero via BS2, RGB/method word via BL0 and flag zero; there are no
color-name/book strings. Decode those words into typed color method/RGB concepts and regenerate the
word. Do not retain packed CMC values. The observed semantic color set corresponds to words
`c2ffffff`, `c2808080`, `c3000007`, `c8000000`, `c2787878`; field-specific distributions are
asserted by the probe.

Modifier pattern is highly regular and is a useful exactness gate. Sixteen built-in styles use
modifier 1 for every property. The user styles `JitterOff` (`0x323`), `OverhangOff` (`0x324`) and
`EdgeColorOff` (`0x325`) use modifier 0 for every property except edge-modifier, whose operation is
2. All modifier 0/1 values use BS2/BS1 respectively; edge operation 2 uses BS1.

Common data is one reactor via BL1 and missing extension dictionary on every frame. Handle layout is
owner then reactor, both resolving to `0x99`; there are no class handles and no terminal fill bits.
Native handle-code histogram is `(12,4)` 15, `(4,4)` 3 and `(8,4)` 1.

Exact signature groups (`payload/total : MC,data,class,string : handles`) are:

```text
103/108:32,792,621,154:323       107/112:32,824,621,186:324
109/114:32,840,621,202:325       118/123:24,920,829,74:9a
120/125:32,928,821,90:9e         125/130:32,968,829,122:9c
126/131:32,976,837,122:a6        126/131:32,976,901,58:a4
129/134:32,1000,829,154:a3       132/137:32,1024,821,186:9f a0
134/139:32,1040,837,186:a7 a8 a9 136/141:32,1056,901,138:a5
137/142:32,1064,893,154:a1       138/143:32,1072,837,218:9b
140/145:32,1088,901,170:a2       144/149:32,1120,837,266:9d
```

CRC oracle:
`9a:767e 9b:805c 9c:5e7e 9d:95cb 9e:8fd4 9f:abe5 a0:752d a1:9981 a2:d4e1
a3:336e a4:11c1 a5:a320 a6:29a4 a7:8d81 a8:3b92 a9:6afd 323:6f8f 324:54da 325:6edf`.

Deterministic writer order is common main, style scalars, the 28 value/modifier pairs, description
TU string, common owner/reactor handles, then CRC. Select compact BL/BS/BD from semantic values;
encode all five typed CMCs atomically. Reject a missing named property, invalid operation, unsupported
color method, isoline overflow or crease outside the standard range.

### Type 520 `BLOCKGRIPLOCATIONCOMPONENT` — 23 frames

Typed body is `{evaluation_expression, grip_type, grip_expression}`. Main/string order:

1. signed parent ID `BLd`;
2. major and minor versions `BL`;
3. signed value discriminator `BSd`, followed by its tagged value (`BD`, `2RD`, `T`, `BL`, class
   handle or `BS`);
4. node ID `BL`;
5. grip type `BL`;
6. Unicode grip expression `T`.

The fixture has parent ID -1 via BL0, major 29 via BL1 and minor 2 via BL1 on all 23. Nineteen use
value discriminator -9999 via BS0 and the empty variant. Four use discriminator 40 via BS1 and a
typed double: zero via BD2 twice, 36 and -2 via BD0 once each. There is no tag-91 class handle in
this fixture; the schema/writer must nevertheless implement its typed object-reference arm and put
that handle after common handles only when selected.

Node IDs are 23 distinct values, all BL1. Grip-type histogram is
`1:2,8:2,26:3,31:3,46:2,54:2,120:3,192:2,222:2,227:2`, all BL1. Grip strings are
`UpdatedBaseX:5, UpdatedBaseY:5, UpdatedEndX:4, UpdatedEndY:4, UpdatedFlip:3,
UpdatedX:1, UpdatedY:1`; string bits are 138 on two, 186 on eleven and 202 on ten.

Common data is zero reactors via BL2 and missing extension dictionary on every frame. The only
handle is owner code 12: 14 resolve to evaluation graph `0x110f`, nine to `0x1155`. Nineteen frames
have two terminal one-fill bits; the four frames whose handle stream is byte-aligned have zero.

| payload / total | MC / data / class / string | terminal | fixture handles |
|---|---|---|---|---|
| 38 / 43 | 18 / 286 / 131 / 138 | `11` | `1120 1121` |
| 43 / 48 | 16 / 328 / 125 / 186 | none | `1162 1163` |
| 44 / 49 | 18 / 334 / 131 / 186 | `11` | `1112 1113 1116 1117 111a 1123 1158` |
| 46 / 51 | 18 / 350 / 131 / 202 | `11` | `111b 111c 1124 1125 1128 1129 1159 115a 115e 115f` |
| 51 / 56 | 16 / 392 / 189 / 186 | none | `1167 1168` |

CRC oracle:
`1112:f08c 1113:4a8d 1116:2012 1117:b211 111a:fb13 111b:4183 111c:2e82
1120:d908 1121:327b 1123:860b 1124:58c9 1125:e288 1128:a464 1129:1e25
1158:ccbf 1159:5db8 115a:6779 115e:805b 115f:8219 1162:fd2a 1163:e40e
1167:8a13 1168:3218`.

Writer derives the signed compact encodings from the tagged expression value, writes grip string in
the independent string stream, then owner and conditional tag-91 reference, terminal one-fill and
CRC. Validate unique node IDs within each evaluation graph and ensure each grip type/expression
resolves to a graph node; reject a discriminator/value mismatch atomically.

### Type 544 `ACDBASSOCGEOMDEPENDENCY` — 31 frames

Typed body is an inherited `AssociativeDependency` followed by the geometry dependency suffix.
Dependency main/string order is class version `BS`, status `BL`, read/write/attached/delegating
`B`, signed order `BLd`, dependent-on reference, has-name `B` plus conditional name `T`, two link
references, dependency-body reference, then signed body ID `BLd`. Because references are split into
the handle stream, main data continues from order directly to has-name and body ID.

Geometry suffix is class version `BS`, enabled `B`, persistent-subentity class `T`, then
dependent-on-compound-object `B`. The fixture is one uniform branch:

- dependency version 1 via BS1, status 0 via BL2;
- read/write/attached/delegating all true;
- order -10000 via BL0; no name;
- body ID via BL1, with `1:3,4:3,5:3` and each of
  `9,12,13,16,17,19,30,31,32,33,34,35,37,70,79,80,81,82,83,84,85,86` once;
- derived geometry version 0 via BS2; enabled true;
- persistent class `AcDbAssocSingleEdgePersSubentId` on all 31, occupying 506 string bits;
- dependent-on-compound-object false.

Common data is zero reactors via BL2 and missing extension dictionary. Handle order is owner,
dependent-on object, link A/read-dependency, link B/node and dependency body. Owners group as
`0x1149:3,0x2026:3,0x207f:3,0x1f2c:22`; all 31 dependent-on targets are distinct and non-null. The
other three class slots are explicitly null on all frames. Native handle codes are exactly
`(12,4,4,4,3)`; semantic soft/hard reference roles come from the typed dependency definition, not
from these relative native encoding codes. Five one-fill bits terminate every payload.

There are two exact signatures; main and string sizes are identical and only handle compactness
changes:

| payload / total | MC / data / class / string | fixture handles |
|---|---|---|
| 87 / 92 | 69 / 627 / 104 / 506 | `114d 114e 114f 202a 202b 202c 2083 2084 2085` |
| 88 / 93 | 77 / 627 / 104 / 506 | `2106 210c 210d 2119 211a 211c 2144 2145 2146 2147 2148 2149 214b 21ef 2232 2233 2234 2235 2236 2237 2238 2239` |

CRC oracle:
`114d:3b8a 114e:1c7f 114f:d91f 202a:1595 202b:2be0 202c:225f 2083:736e
2084:81c4 2085:848d 2106:ed07 210c:71cf 210d:7bac 2119:3b1d 211a:5a05
211c:ab83 2144:d048 2145:1a34 2146:462e 2147:4c4d 2148:d3dc 2149:d9bf
214b:3253 21ef:ea79 2232:f04b 2233:45da 2234:d83e 2235:adae 2236:0ce7
2237:862f 2238:c92c 2239:7d0d`.

Writer order is common main, dependency scalars, conditional name, body ID, geometry suffix,
persistent-subentity class string, common handles, dependency handles in declared role order,
five one-fill bits and CRC. Class versions and stream sizes are derived. Require each dependent-on
target to reciprocally reference the dependency where the standard graph exposes that relation;
body ID must be unique within its owning action. Preserve nullable standard slots as typed options
and emit their native null handles in order rather than omitting them.

### Production and acceptance gates

1. Add three tagged bodies and shared typed `EvaluationExpression`, `AssociativeDependency`,
   `PersistentSubentityId`, CMC and property-operation concepts. Do not add generic scalar/handle
   bags or opaque custom-body bytes.
2. Bound and exhaust main/string/handle streams independently. Require the exact terminal-fill
   width/pattern per signature and validate CRC before accepting a frame.
3. Extend the existing fixture test with exact per-frame assertions for 19/19 type 506, 23/23 type
   520 and 31/31 type 544. Assert the branch/value histograms, graph relations and CRC oracles above.
4. Mutation/inverse coverage changes a visual property plus operation, switches an evaluation
   expression between empty/double/reference arms, and changes dependency target/body ID while
   proving exact inverse restoration and graph validation.
5. Require exact native equality through logical snapshot DSL/pack, diff/apply/inverse/absorb,
   mutation/inverse, analyzer and composer before reducing any of these 73 ledger counts. No BOT,
   selector, packed CMC, string-size, fill, CRC or native handle-code state may be persisted.

## AC1024 types 541/545 and P11 thin-body exact-frame checklists (2026-08-14)

Ticket probe: `🧪️dwg-custom-541-545-thin-frame-probe.py`. It independently decodes and bounds the
23 type-541, 18 type-545 and 33 P11 thin-body frames, exhausts main/string/handle streams and
validates all 74 CRCs. The P11 cohort is exactly type 559 ×12, type 549 ×12, type 543 ×6, type 522
×2 and type 547 ×1. No production source or test was changed and Nx was not run.

All 74 use BOT selector 1, direct object-handle code 0 and empty EED. Common object order remains
reactor count `BL`, extension-dictionary-missing `B`, then owner, reactor vector and optional
extension dictionary in the handle stream. Every listed frame lacks an extension dictionary.
CRC is little-endian CRC-16 seeded `0xC0C1` over contiguous `MS + MC + payload`.

### Type 541 `ACDBASSOCVALUEDEPENDENCY` — 23 frames

The exact body is not dependency-core-only. Persist:

```text
AssocValueDependency {
    dependency: AssocDependency,
    cached_value: EvalVariant,
    value_name: String,
}
```

Native value-dependency class version zero is derived. The inherited dependency order is version
`BS`, status `BL`, read/write/attached/delegating `B`, order `BLd`, dependent-on handle, optional
dependency name, two nullable chain links, nullable body handle and body ID `BLd`. The specialization
then writes version `BS`, typed cached `EvalVariant`, and value-name `T`.

Fixture branches are uniform except identifiers/value magnitude:

- inherited version 1 via BS1; status 0 via BL2;
- read true, write false, attached true, delegating true; order 0 via BL2;
- inherited name absent; body ID via BL1 with histogram
  `1:5,2:3,6:3,14:1,20:1,22:1,24:1,40:1,42:1,44:1,46:1,48:1,50:1,71:1,87:1`;
- specialization version 0 via BS2;
- cached variant code 90 via BS1, always typed Integer32; values
  `6:14,144:4,4:1,36:1,42:1,60:1,600:1`; BL1 on 22 and BL0 on 600;
- semantic value name empty on all frames; its TU is still a present two-bit string stream.

Common reactors are zero via BL2. Handle order is owner, dependent-on, chain link A, chain link B,
dependency body. Native code histogram is `(12,4,4,4,3)` 20 and `(8,4,4,4,3)` three. All
dependent-on refs resolve to type-545 variables. Body refs are null 23/23. Link A is non-null on five
and link B on six, forming the dependency chains; public schema names should become
previous/next only after reciprocal ordering validates direction. Three one-fill bits terminate all
frames.

| payload / total | MC / data / class / string | fixture handles |
|---|---|---|
| 21 / 26 | 59 / 109 / 90 / 2 | `114a 2027 2080` |
| 22 / 27 | 67 / 109 / 90 / 2 | `1150 202d 2086` |
| 23 / 28 | 75 / 109 / 90 / 2 | `2124 212a 2158 215d 2163 2168 2179 21ec 2252` |
| 24 / 29 | 83 / 109 / 90 / 2 | `21f0` |
| 25 / 30 | 91 / 109 / 90 / 2 | `211f 2153` |
| 26 / 31 | 75 / 133 / 114 / 2 | `2109` |
| 26 / 31 | 99 / 109 / 90 / 2 | `2127 212d 2160 216b` |

CRC oracle:
`114a:a65c 1150:8ece 2027:92b6 202d:e8e6 2080:5ac8 2086:649d 2109:2cb4
211f:9913 2124:d638 2127:1d95 212a:7094 212d:cb87 2153:fed1 2158:8b19
215d:c85b 2160:b372 2163:2849 2168:7d8c 216b:5a1c 2179:4840 21ec:354b
21f0:c514 2252:69b4`.

Writer order is common main, inherited dependency scalars, specialization version/cached variant,
inherited optional name plus value-name string, common handles, four dependency-role handles,
conditional cached-value object handle, three one-fill and CRC. The value code is derived from the
typed variant. Validate cached value against the depended-on variable's evaluated value without
normalizing a legitimate stale standard cache during an unchanged roundtrip.

### Type 545 `ACDBASSOCVARIABLE` — 18 frames

Typed body is inherited `AssocAction`, variable state and a typed expression-reference binding:

```text
AssocVariable {
    action: AssocAction,
    name, expression, evaluator_id, description,
    value: EvalVariant,
    mergeable, mergeable_variable_name, must_merge,
    referenced_value_dependencies: [Handle<AssocValueDependency>],
}
```

Variable class version 2 and reference-binding version zero are derived. Action order is version
`BS`, status `BL`, owning network/action body handles, action index `BL`, maximum dependency index
`BL`, inherited dependency count `BL`, ownership bits and handles. AC1024 version 1 omits the R2013
owned-parameter/value-parameter extension. Variable order is four `T` strings, `EvalVariant`,
mergeability presence/name, must-merge, then the expression-reference binding. The binding count is
present when maximum dependency index is nonzero and is followed by hard-owned value-dependency
refs; its zero binding version follows the collection.

Fixture action branches:

- version 1 via BS1 and status 0 via BL2 on all;
- action indices `2:3,4:3`, and `5,6,7,8,12,13,14,15,16,17,26,30` once, all BL1;
- maximum dependency index 0 via BL2 on 13 and 1 via BL1 on five;
- inherited action dependency list empty on all 18; expression-reference binding empty on the same
  13 and count 1 via BL1 on the five index-1 variables;
- one common reactor on 16, four reactors on two, all counts via BL1.

Variable class version is 2 via BL1. Names are `W:3,H:3` and one each of `bldDEPTH`, `bldWALL`,
`Wall2`, `Wall3`, `iWALL1..4`, `Room1..3`, `hall`. Expressions are empty twice, numeric `6:4`,
`144:4`, and `42`, `60`, `600` once, plus `bldWALL:2` and `iWALL1:3`. Evaluator ID is
`AcDbCalc:1.0`; description is empty. Cached `EvalVariant` is code-90 Integer32 on all 18 with
values `6:9,144:4,4:1,36:1,42:1,60:1,600:1`; BL1 on 17 and BL0 for 600. Mergeable and must-merge
are false, so mergeable-variable name is semantically absent.

The five symbolic expressions (`Wall2`, `Wall3`, `iWALL2`, `iWALL3`, `iWALL4`) own exactly one
type-541 dependency: `2127 212d 2160 216b 21f0`. Each dependency owner points back to its variable
and its dependent-on ref resolves to the referenced variable. This is the graph proof for the
binding name; it is not a raw trailing handle.

Common/class handle order is owner, reactors, owning network, nullable action body, then conditional
referenced-value dependencies. Owner and owning-network refs agree in four network groups:
`0x1148:2,0x2034:2,0x208d:2,0x1f26:12`. Action body is null 18/18. Terminal fill is `1111` on the
13 constant/no-reference frames and `11` on the five one-reference frames.

| payload / total | MC / data / class / string | handles / branch |
|---|---|---|
| 54 / 59 | 76 / 356 / 107 / 232 | `1153 1154` |
| 57 / 62 | 76 / 380 / 107 / 256 | `2031 208a` |
| 59 / 64 | 76 / 396 / 107 / 272 | `2030 2089` |
| 68 / 73 | 84 / 460 / 107 / 336 | `2251` |
| 70 / 75 | 84 / 476 / 107 / 352 | `2157 2162 2178` |
| 77 / 83 | 156 / 460 / 107 / 336 | `2152`, four reactors |
| 79 / 84 | 84 / 548 / 131 / 400 | `2108` |
| 79 / 85 | 156 / 476 / 107 / 352 | `211e`, four reactors |
| 83 / 88 | 106 / 558 / 125 / 416 | `2123 2129 215c 2167 21eb`, one binding |

CRC oracle:
`1153:0be9 1154:247f 2030:e0d0 2031:a33d 2089:23ec 208a:1486 2108:a09f
211e:66e2 2123:e30c 2129:a074 2152:9f55 2157:f6ce 215c:82dc 2162:0ecb
2167:30eb 2178:dd3a 21eb:99a4 2251:8618`.

Writer derives action/variable versions, counts and EvalVariant code, emits all main fields, four
primary strings and conditional merge name, common/action/binding handles, terminal one-fill and
CRC. Validate symbol references against the owned value-dependency collection and evaluator graph;
reject missing, duplicate or cyclic bindings atomically.

### P11 thin-body cohort — 33 frames

#### Type 522 `ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION` — two

Typed body is a purge-preventer marker plus protected BLOCK_HEADER reference; native flag/version 1
is derived. Both use one reactor via BL1, missing xdictionary, no string stream, and handles owner,
reactor, protected block with native codes `(12,4,5)`. Owners/reactors are `0x110e` and `0x1146`;
protected blocks are `0x110d` and `0x1145`. Signature is payload/total `16/21`, MC/data/class
`70/58/57`, terminal `111111`. CRCs: `1137:c2bc 116a:583d`.

#### Type 543 `BLOCKPARAMDEPENDENCYBODY` — six

Typed body stores dependency-body name; native dependency-body version 1, dimension-base version 1
and derived class version 0 encode BS1/BS1/BS2. The name is the sole TU. Common reactors zero BL2;
owner is the sole handle with native code 8; no terminal fill.

| payload / total | data / class / string | handles / names |
|---|---|---|
| 20 / 25 | 152 / 61 / 74 | `2029 2082`, `H=6"` |
| 26 / 31 | 200 / 61 / 122 | `202f 2088`, `W=3'-6"`, `W=5'-0"` |
| 28 / 33 | 216 / 61 / 138 | `114c`, `H=4.0000` |
| 30 / 35 | 232 / 61 / 154 | `1152`, `W=36.0000` |

CRCs: `114c:0bae 1152:d43d 2029:39e3 202f:f97a 2082:018c 2088:4a5b`.

#### Type 547 `ACDB_DYNAMICBLOCKPROXYNODE` — one

Typed body is exactly `EvaluationExpression`: parent -1 BL0, major 29 BL1, minor 2 BL1, empty
discriminator -9999 BS0, node ID 226 BL1. It has zero reactors BL2, no string, owner `0x1155` code
12, no conditional value handle, and terminal `111111`. Handle `0x1164`; payload/total `18/23`,
MC/data/class `22/122/121`; CRC `10f2`.

#### Type 549 `ASSOCDIMDEPENDENCYBODY` — twelve

Typed body stores dependency-body name; versions are dependency-body 1 BS1, dimension-base 1 BS1,
class 1 BS1. Common reactors zero BL2; owner is the sole code-8 handle; no fill. The twelve names
are exactly the dimension user texts already listed in the DIMENSION_LINEAR checklist.

| payload / total | data / class / string | fixture handles |
|---|---|---|
| 31 / 36 | 240 / 69 / 154 | `2155` |
| 33 / 38 | 256 / 69 / 170 | `2121` |
| 35 / 40 | 272 / 69 / 186 | `2254` |
| 37 / 42 | 288 / 69 / 202 | `215a 2165 217b` |
| 39 / 44 | 304 / 69 / 218 | `2126 212c 215f 216a 21ee` |
| 43 / 48 | 336 / 69 / 250 | `210b` |

CRCs: `210b:7211 2121:bbf5 2126:79b5 212c:b696 2155:f754 215a:2065
215f:be2a 2165:6eee 216a:89d3 217b:2397 21ee:057c 2254:eadd`.

#### Type 559 `ACDB_BLOCKREPRESENTATION_DATA` — twelve

Typed body is a representation marker plus represented BLOCK_HEADER reference; native flag/version
1 is derived. All use one reactor BL1, no string, handles owner/reactor/represented block with codes
`(8,4,5)`, and terminal `111111`. Owner equals reactor for each representation. Represented blocks
are `0x110d` four and `0x1145` eight. One signature: payload/total `15/20`, MC/data/class
`62/58/57`; handles `1f40 1f8d 1fe0 1ffa 2014 206d 20aa 20b7 20c4 20d1 20de 20eb`.
CRCs: `1f40:a900 1f8d:eabc 1fe0:0cc1 1ffa:0795 2014:9b73 206d:b0a7
20aa:348e 20b7:fea7 20c4:d073 20d1:dde7 20de:1a4e 20eb:c9e7`.

### P11 production and lifecycle gates

1. Implement shared `AssocDependency`, `AssocAction`, `EvalVariant`, `EvaluationExpression`,
   dependency-body and marker/reference concepts once, then tagged class bodies. No generic custom
   scalar/handle bags are permitted.
2. Validate every count, optional branch, graph backreference, stream boundary, terminal fill and
   CRC before admitting the logical object. Unsupported EvalVariant arms reject atomically.
3. Writer order is common main, inherited core, derived typed body, independent strings, common
   handles, inherited handles, derived handles, fill and CRC. Versions/counts/tags/selectors derive
   from logical concepts.
4. Extend the existing fixture test with 23/23 + 18/18 + 33/33 exact-frame assertions and the
   branch/signature/CRC oracles above. Add mutation/inverse for value cache/name, variable expression
   binding, dependency-body name and marker block reference.
5. Credit ledger rows only after native exactness survives logical DSL/pack,
   diff/apply/inverse/absorb, mutation/inverse, analyzer and composer. Persist no diagnostic bits,
   frame bytes, selector, string-size, fill, CRC or native handle code.

## AC1024 semantic objects to AcDbObjects and Handles reconstruction (2026-08-14)

This read-only reconstruction is executable in `🧪️dwg-object-handles-reconstruction-probe.py`;
its stable fixture output is `🧪️dwg-object-handles-reconstruction.log`. It validates every frame
CRC, every handle-map block CRC, every object-handle/key equality, complete object-section coverage,
and byte-identical re-encoding of the handle map. No production file was edited and no Nx task was
started.

### Critical census correction: 663, not 652

The handle map has **two independently based non-empty blocks**. The earlier inventory retained
`last_handle` and `last_address` across the block boundary. That transformed the second block's 11
valid addresses into out-of-range values; `r2010_object_inventory` and
`decode_r2004_object_records` then silently continued past them. Resetting both accumulators at
each block, as required by ODA section 23 and LibreDWG's `read_2007_section_handles`, yields 663
unique handles and 663 valid frames. All frame CRCs pass and the address-sorted frames cover
logical offsets `4..213182` contiguously with no gap or overlap.

The second block adds these real objects:

| handle | address | fixed/dynamic type | impact on prior ledger |
|---:|---:|---:|---|
| `0x2255` | 4,894 | 21 | `DIMENSION_LINEAR` +1 |
| `0x2256` | 33,022 | 545 | `ACDBASSOCVARIABLE` +1 |
| `0x2257` | 54,578 | 541 | `ACDBASSOCVALUEDEPENDENCY` +1 |
| `0x2258` | 56,336 | 542 | `ACDBASSOCDEPENDENCY` +1 |
| `0x2259` | 208,647 | 549 | `ASSOCDIMDEPENDENCYBODY` +1 |
| `0x225a` | 57,216 | 541 | `ACDBASSOCVALUEDEPENDENCY` +1 |
| `0x2266` | 5,048 | 21 | `DIMENSION_LINEAR` +1 |
| `0x2267` | 33,110 | 545 | `ACDBASSOCVARIABLE` +1 |
| `0x2268` | 56,361 | 541 | `ACDBASSOCVALUEDEPENDENCY` +1 |
| `0x2269` | 56,659 | 542 | `ACDBASSOCDEPENDENCY` +1 |
| `0x226a` | 208,729 | 549 | `ASSOCDIMDEPENDENCYBODY` +1 |

The corrected affected counts are type 21 = 14, 541 = 26, 542 = 20, 545 = 20 and 549 = 14.
The consolidated ledger above is therefore 286 accepted / 663 total / 377 remaining. Any gate
asserting 652 is now an anti-acceptance bug because it proves eleven semantic objects were dropped.

### AcDbObjects deterministic materializer

The logical snapshot must retain an **ordered collection of typed semantic objects**, not frame
addresses or frame bytes. ODA section 20 explicitly permits objects in any order, so imported
object order is a standard logical ordering concept. It is distinct from handle-map order: in this
fixture the handle map is strictly ascending by handle while its addresses contain 147 negative
deltas. Sorting semantic objects by handle before frame emission therefore cannot reproduce the
fixture.

The writer sequence is:

1. Begin the AC1024 logical object section with little-endian `RL 0x00000dca`. This is the standard
   R18+ AcDbObjects prologue specified by ODA, not retained source bytes.
2. Iterate the snapshot's semantic object collection in logical object order. Before each frame,
   record `address = section.len()` against its unique nonzero handle.
3. Dispatch the typed body to its R2010 frame writer. Emit `MS payload_size`, `UMC
   handle_stream_bits`, the byte-aligned BOT/object handle/EED/main/string/handle streams, terminal
   one-fill required by that typed body, and little-endian frame CRC over every byte except the CRC.
   Unsupported bodies or incomplete bounded streams reject atomically.
4. Append the frame and advance solely by the emitted frame length. No address, frame length,
   selector, fill, or checksum is stored in the artifact.
5. Require the final `(handle,address)` set to be one-to-one and every frame interval to be exactly
   adjacent in address order. Build Handles only after all frame addresses are known.

Fixture order oracles are first
`0x1@4,0x1fa4@45,0x1fa9@144,0x1faa@254,0x1fab@362,0x1fac@460,0x1fad@527,0x1fae@594`
and last
`0x20e4@212729,0x20ee@212772,0x20ef@212892,0x20f0@212939,0x20f1@213056,0x114c@213099,0x2029@213132,0x2082@213157`.
The SHA-256 of the address-ordered handle sequence encoded as little-endian `u64` values is
`3fdcc6fa7bc98d3fce64072ba38501ef29cb87ce70d18fc9fe0d6d8804bc70ad`.

The completed semantic AcDbObjects payload is 213,182 bytes with SHA-256
`d50dff10271442fe9c7d8812ce40a29dc38f3665fe34aa5ffd1fecffdb323cc6`.
Its first four bytes are `ca0d0000`; 663 frames cover every subsequent byte through offset 213,182.

### Handles deterministic materializer

After frame emission, sort the `(handle,address)` index by strictly ascending handle. Handle and
address bases are both zero at the start of **every** block.

- Encode `handle - last_handle` as unsigned modular char: low seven-bit groups first, continuation
  in bit 7.
- Encode `address - last_address` as signed modular char: magnitude groups first; the final byte's
  bit 6 is the negative sign. Positive terminal groups must remain below `0x40`, so a positive value
  whose final magnitude group would set bit 6 uses another continuation group.
- Append a complete pair, then close the block when the declared size (two-byte header plus pair
  payload) is greater than 2,030 bytes. This post-pair boundary rule is fixture-significant: the
  first declared size is 2,033, not a preflight-truncated 2,030/2,032.
- Backpatch the two-byte **big-endian** declared size. Append big-endian CRC16 seeded with `0xC0C1`
  over the size bytes plus pair payload. Reset both bases before the next block.
- After the last non-empty block, emit the size-two empty block `0002` and its CRC `01d0`. There is
  no trailing fill or extra sentinel.

Exact block oracles:

| block | section offset | declared / pair bytes | entries | first / last | address delta signs | CRC |
|---:|---:|---:|---:|---|---|---:|
| 0 | 0 | 2,033 / 2,031 | 652 | `0x1@4` / `0x2254@208563` | 507 positive, 145 negative | `e277` |
| 1 | 2,035 | 44 / 42 | 11 | `0x2255@4894` / `0x226a@208729` | 9 positive, 2 negative | `56f0` |
| 2 | 2,081 | 2 / 0 | 0 | terminator | none | `01d0` |

The resulting Handles payload is 2,085 bytes with SHA-256
`5e877e4b7374d7c343f37052f0d8e930a37612b3a0f04aadb77b9f6afc9b90df`.
The executable probe reconstructs those bytes exactly from the 663 logical handle/address pairs.

### Page slicing and fixture section oracles

Only after both logical payloads are complete may the ordinary R2004-family page layer split and D2
compress them. AcDbObjects uses seven full `0x7400` slices and one 5,310-byte tail; Handles is one
2,085-byte slice. Page descriptors use these logical offsets and the compressed byte lengths below.

| page | logical offset | logical bytes | compressed bytes | logical SHA-256 |
|---:|---:|---:|---:|---|
| 7 | 0 | 29,696 | 17,145 | `2d1f57eba3a25cca4c9f40ec0881050fe323cf94ebfe5520c70f27fc5da961f7` |
| 8 | 29,696 | 29,696 | 11,080 | `887e919c98b9af167ac46c9b90f4f38319f5ff7466a29700389c22a13221aecf` |
| 9 | 59,392 | 29,696 | 4,380 | `c0c09b43b00b69baebaa5a81820d71f6269faacbe8c2120feff2c0c6b0bc894f` |
| 10 | 89,088 | 29,696 | 2,246 | `2089267e7bd83d8563b269d4b882a5221b361044c1f5714324f8ab69bac16398` |
| 11 | 118,784 | 29,696 | 3,378 | `5b7bfe1ed0cd46efd76695cde7566f465d6d12895dea9cdbbcae972e4156c3a2` |
| 12 | 148,480 | 29,696 | 4,448 | `4c0acc2895895d9ae2211e70374c412f06bd4a4090a1aebb13961eb643f45564` |
| 13 | 178,176 | 29,696 | 3,490 | `52be397a7db9c95c7932d80317dadee7479b8a238eea3a676cd08ab07eeda0b9` |
| 14 | 207,872 | 5,310 | 1,711 | `878e2883f8e3b11c2ffde8ce694cd9e2820764344d4acce4ab5c7d0c69027c14` |
| 17 Handles | 0 | 2,085 | 1,907 | `5e877e4b7374d7c343f37052f0d8e930a37612b3a0f04aadb77b9f6afc9b90df` |

The outer allocations remain pages 7–14 at
`0x16260,0x1a580,0x1d100,0x1e240,0x1eb40,0x1f8a0,0x20a20,0x21800` with
allocations `17184,11136,4416,2304,3424,4480,3552,1760`; Handles page 17 is at `0x22080`
with allocation 1,952. The section-info semantic sizes are AcDbObjects 213,182 and Handles 2,085.

### Live implementation gap checklist

1. `decode_r2004_handle_map` currently declares cumulative `handle` and `address` outside its block
   loop, never verifies any block CRC, and exits before validating the terminator CRC. Move both bases
   inside the non-empty block, validate exact block consumption and every CRC, and reject trailing data.
2. `r2010_object_inventory` and `decode_r2004_object_records` use `continue` for invalid address,
   prefix, payload, type and handle mismatches. Replace those lossy paths with handle-qualified typed
   errors and assert exactly 663 records. No native object may disappear during import.
3. `decode_r2004_object_records` currently iterates handle-map order. Decode the strict map first,
   associate frames by address, and preserve the address-sorted order in the snapshot's ordered
   semantic object collection. Do not persist addresses.
4. `encode_r2004_canonical` still sends the reduced `drawing` through `dwg_to_bytes` as
   AcDbObjects and emits an empty Handles section. Replace this with a single semantic object
   materializer that dispatches the existing `encode_r2010_*_frame` functions, records addresses,
   and calls the structured handle-map writer above.
5. `finish_r2010_object_frame` is the correct per-frame boundary. Every remaining body cohort must
   reach it through a typed dispatcher; the older `dwg_write_object` framing is not an AC1024 path.
6. Extend the existing fixture test, not a new test file, with exact 663-record census, exact section
   hashes/lengths, all frame and block CRCs, byte-identical Handles reconstruction, and byte-identical
   AcDbObjects reconstruction. A body mutation may change downstream addresses and block bytes;
   applying its inverse must restore both section hashes and the original native file.
7. Run the same original-byte assertions through logical DSL/pack, diff/apply/inverse/absorb,
   mutation/inverse, analyzer/composer and native serialization. Anti-shadow scans must forbid frame,
   address, block, checksum, page payload, compressed payload and native section state in snapshots
   and facets; ordered typed objects and semantic preview image data remain legitimate concepts.

Primary authority: [ODA DWG specification sections 20 and 23](https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf).
Independent implementation cross-checks: LibreDWG
[`decode_r2007.c`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/decode_r2007.c)
resets `last_offset` for each block, and
[`encode.c`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/encode.c) shows handle
sorting, post-pair block closure, per-block base reset, CRC, and the size-two terminator.

## AC1024 recovered second-block frame characterization (2026-08-14)

`🧪️dwg-recovered-handle-block-frames-probe.py` parses the eleven frames recovered by block-local
Handles bases through the same typed readers used by the earlier cohort probes. Its evidence log is
`🧪️dwg-recovered-handle-block-frames.log`. All 11 frame prefixes, main/string/handle boundaries,
terminal fills, object-handle keys and CRCs validate. The recovered frames introduce no opaque or
unmodeled body variant; they extend five already identified typed cohorts. The type-542 analysis
below upgrades that cohort from a field matrix to a complete 20-frame exact-frame checklist.

### Recovered frame inventory and semantic graph

| handle | class; payload / frame; MC / data / class / string; fill; CRC | typed values | resolved semantic edges |
|---:|---|---|---|
| `2255` | DIMENSION_LINEAR; 149 / 154; 87 / 1105 / 870 / 218; `1111111`; `b1c7` | measurement 6, text `Wall1=bldWALL`; standard default extrusion/scales/angles | reactor `2258`, layer `1b81`, DIMSTYLE `242`, null anonymous block |
| `2256` | ACDBASSOCVARIABLE; 83 / 88; 106 / 558 / 125 / 416; `11`; `0f2a` | action index 31; `Wall1 = bldWALL`; integer cache 6; max dependency 1 and one referenced value dependency | owner/network `1f26`, reactor `2257`, null action body, referenced dependency `225a` |
| `2257` | ACDBASSOCVALUEDEPENDENCY; 23 / 28; 75 / 109 / 90 / 2; `111`; `82e9` | read dependency, attached/delegating, body ID 89; integer cache 6; empty value name | owner constraint group `1f2c`, dependent-on variable `2256`; null readdep/node/body |
| `2258` | ACDBASSOCDEPENDENCY; 20 / 25; 91 / 69 / 68 / 0; `111`; `5330` | write dependency, attached/delegating, body ID 90 | owner constraint group `1f2c`, dependent-on dimension `2255`, dependency body `2259`; null readdep/node |
| `2259` | ASSOCDIMDEPENDENCYBODY; 39 / 44; 8 / 304 / 69 / 218; no fill; `b14c` | versions 1/1/1, name `Wall1=bldWALL` | hard-owned by dependency `2258` |
| `225a` | ACDBASSOCVALUEDEPENDENCY; 24 / 29; 83 / 109 / 90 / 2; `111`; `c81b` | read dependency, attached/delegating, body ID 1; integer cache 6; empty value name | owner variable `2256`, dependent-on `bldWALL` variable `211e`, readdep `212d`; null node/body |
| `2266` | DIMENSION_LINEAR; 153 / 158; 87 / 1137 / 870 / 250; `1111111`; `da31` | measurement 480, text `bldWIDTH=40'-0"`; standard default extrusion/scales/angles | reactor `2269`, layer `1b81`, DIMSTYLE `242`, null anonymous block |
| `2267` | ACDBASSOCVARIABLE; 79 / 84; 84 / 548 / 131 / 400; `1111`; `d5df` | action index 32; `bldWIDTH = 480`; integer cache 480; no referenced dependency | owner/network `1f26`, reactor `2268`, null action body |
| `2268` | ACDBASSOCVALUEDEPENDENCY; 26 / 31; 75 / 133 / 114 / 2; `111`; `ce33` | read dependency, attached/delegating, body ID 91; integer cache 480; empty value name | owner constraint group `1f2c`, dependent-on variable `2267`; null readdep/node/body |
| `2269` | ACDBASSOCDEPENDENCY; 20 / 25; 91 / 69 / 68 / 0; `111`; `c44e` | write dependency, attached/delegating, body ID 92 | owner constraint group `1f2c`, dependent-on dimension `2266`, dependency body `226a`; null readdep/node |
| `226a` | ASSOCDIMDEPENDENCYBODY; 43 / 48; 8 / 336 / 69 / 250; no fill; `22ad` | versions 1/1/1, name `bldWIDTH=40'-0"` | hard-owned by dependency `2269` |

CRC values in this table are the decoded little-endian frame `RS` values; the corresponding two
trailing bytes are reversed on the wire. Addresses and frame metadata are validation products only
and must not enter the logical schema.

The two dimensions form complete reciprocal triples:

```text
2255 DIMENSION_LINEAR --reactor--> 2258 ASSOCDEPENDENCY --body--> 2259 ASSOCDIMDEPENDENCYBODY
        ^                              |                                  |
        +--------- dependent-on -------+---------------- owner -----------+

2266 DIMENSION_LINEAR --reactor--> 2269 ASSOCDEPENDENCY --body--> 226a ASSOCDIMDEPENDENCYBODY
        ^                              |                                  |
        +--------- dependent-on -------+---------------- owner -----------+
```

The variable side similarly proves two ordinary value/cache relationships and one expression
binding. `2256 Wall1` owns/references `225a`; `225a` points to existing variable `211e bldWALL`
and the existing dependency chain through `212d`. Thus `Wall1 = bldWALL` is the sixth symbolic
variable binding, not an isolated string. `2267 bldWIDTH` is a constant variable whose reactor
`2268` caches the same integer 480. Graph validation must require every variable reactor to point
back to that variable, every dimension body owner to be its dependency, dimension user text and
body name to agree, and expression references to resolve through the dependency graph.

### Type 542 ACDBASSOCDEPENDENCY — exact 20-frame checklist

Type 542 has no class-local prefix or suffix beyond the standard dependency core. Exact order after
common non-entity state is:

1. class version `BS` = 1;
2. status `BL` = 0;
3. `is_read_dependency B`, `is_write_dependency B`, `is_attached_to_object B`,
   `is_delegating_to_owning_action B`;
4. signed order `BLd` = 0;
5. dependent-on object handle;
6. has-name `B` = false, so no `T` name;
7. read-dependency handle;
8. dependency-node handle;
9. dependency-body handle;
10. signed dependency-body ID `BLd`, always BL selector 1 in this fixture;
11. absent string-stream marker, then the handle stream and terminal `111` fill.

The logical names in steps 5–9 correspond to LibreDWG's `dep_on`, `readdep`, `node`, and
`dep_body`. The fixture's deterministic native handle nibbles are owner code 12, then codes
`4,4,4,3` for those four roles. This corrects the earlier probe's unused candidate-code list
`3,4,3,4`; native handle nibbles are derived by the writer from the role and target, never persisted.

All 20 frames have zero reactors via BL2, missing extension dictionary, class-version BS1, status
BL2, order BL2, no name/string bits, and exact terminal `111`. Read is false, write and delegating
are true throughout. Attached is false on the six linked parameter dependencies
`114b,1151,2028,202e,2081,2087` and true on the fourteen dimension dependencies, including the two
recovered frames.

| signature | fixture handles |
|---|---|
| payload/frame 20/25; handle/data/class 91/69/68 | `210a 2120 2125 212b 2154 2159 215e 2164 2169 217a 21ed 2253 2258 2269` |
| payload/frame 21/26; handle/data/class 99/69/68 | `114b 1151 2081 2087` |
| payload/frame 23/28; handle/data/class 115/69/68 | `2028 202e` |

CRC oracle:
`114b:b3ff 1151:e98d 2028:4221 202e:74eb 2081:b373 2087:e0ff 210a:da33
2120:624a 2125:d560 212b:162b 2154:fa56 2159:deec 215e:ac06 2164:8f7d
2169:6bc6 217a:6270 21ed:9171 2253:d9dc 2258:5330 2269:c44e`.

Writer derives all selectors, name presence, target-relative handle encodings, body IDs and CRC.
It rejects a read/write combination inconsistent with the resolved target role, a missing body for
an attached dimension dependency, a body whose owner does not point back, or any residual main,
string or handle bit.

### Existing cohort coverage and corrected branch histograms

| cohort | recovered coverage | schema consequence |
|---|---|---|
| DIMENSION_LINEAR 14 | `2255` uses an existing 149-byte payload group; `2266` adds the sole 153-byte/158-frame signature. Both use the established R2010 common/linear field order and terminal `1111111` | Existing typed checklist covers both; update count, user-text/measurement fixtures, CRC set, and exact test from 12 to 14 |
| type 541, 26 | payloads 23, 24 and 26 and integer caches 6/480 are existing EvalVariant/body branches; all retain terminal `111` | Existing typed schema covers all three; update all-frame census and graph validation. The owner-code histogram is code 12 on 23 and code 8 on three |
| type 542, 20 | recovered payload 20 branch is shared by fourteen frames; exact core and all handle roles now proven across the cohort | Promote from matrix to exact-frame checklist; correct native role-code prescription as above |
| type 545, 20 | `2256` uses the established one-reference tail `11`; `2267` uses constant tail `1111`. Integer value 480 exercises BL0 while 6 uses BL1 | Existing typed action/variable/EvalVariant schema covers both. Corrected maxima: 14 variables with dependency index 0 and six with index 1; 14 no-reference and six one-reference frames; 18 one-reactor and two four-reactor frames |
| type 549, 14 | payloads 39 and 43 are existing dependency-body string-length groups; exact versions and no-fill handle boundary are unchanged | Existing typed schema covers both; update names, CRCs and owner/body reciprocity assertions from 12 to 14 |

The corrected variable names add `Wall1` and `bldWIDTH`; integer caches add one 6 and one 480.
The six symbolic expressions with one referenced dependency are now `Wall1`, `Wall2`, `Wall3`,
`iWALL2`, `iWALL3`, and `iWALL4`. The earlier five-binding list and all 12/18/23-frame test bounds
must be rejected as stale even if their original subsets remain green.

### Production and acceptance gates

1. Fix block-local Handles accumulation first so all 663 identities reach the decoder. Silent
   address/frame `continue` paths remain forbidden.
2. Reuse the existing typed DIMENSION_LINEAR, associative dependency/value dependency, variable and
   dependency-body variants; do not introduce a recovered-block variant or native-order flag.
3. Correct dependency handle roles to named `dependent_on`, `read_dependency`, `node`, and
   `dependency_body`; remove generic link-A/link-B names from schema/facets before implementation.
4. Extend the existing exact-frame fixture test to 14/26/20/20/14 for types 21/541/542/545/549,
   asserting the signatures, fields, edges, fills and CRCs above. Every same-class frame must pass.
5. Add mutation/inverse coverage for dimension text/measurement and dependency body, variable
   expression/reference and EvalVariant cache, and dependency target/body ownership. Inverse must
   restore both object-section and Handles hashes from the reconstruction checklist.
6. Credit no recovered frame until typed body decode/write and original bytes remain exact through
   logical DSL/pack, diff/apply/inverse/absorb, mutation/inverse, analyzer/composer and native export.

## Reconciled 663-frame ledger and next-100 queue (2026-08-14)

This reconciliation incorporates the block-local Handles reset and the implementation lane's
combined runtime exact-frame gate for XRECORD 145, DICTIONARY/WDFLT 84, table controls 9, table
records 50 including DIMSTYLE 2, LINE 40, ARC 12 and LWPOLYLINE 16. Those seven cohorts total
**356 exact frames**. The denominator is the CRC-validated 663-frame inventory, so **307 remain**:
53.70% credited and 46.30% remaining. Whole-file equality remains red and is not inferred from
these frame gates.

### Family reconciliation

| family | fixture | exact credited | remaining | current frontier |
|---|---:|---:|---:|---|
| fixed entities | 82 | 68 | 14 | DIMENSION_LINEAR 14 exact-ready |
| dictionary/XRECORD spine | 237 | 229 | 8 | DICTIONARYVAR 8 matrix-ready |
| block/entity graph | 32 | 0 | 32 | BLOCK/ENDBLK/INSERT exact-ready |
| table controls/records | 59 | 59 | 0 | complete |
| fixed support | 6 | 0 | 6 | VIEWPORT exact-ready; three support variants matrix-ready |
| style/context custom | 50 | 0 | 50 | VISUALSTYLE exact-ready; two semantic gates remain |
| dynamic-block custom | 71 | 0 | 71 | type 520 and thin marker bodies exact-ready |
| associative custom | 126 | 0 | 126 | 117 exact-ready, five matrix-ready, four gated |
| **total** | **663** | **356** | **307** | **next 100 below have complete typed prescriptions** |

### Every remaining cohort ranked by readiness, then count

**R0 exact-ready** means the ticket has all-frame bounded main/string/handle evidence, typed field
and role names, signatures/CRCs and a deterministic writer prescription with no unresolved concept.
These 16 cohorts contain 222 frames.

| rank | type/class | frames | leverage/readiness |
|---:|---|---:|---|
| 1 | 544 `ACDBASSOCGEOMDEPENDENCY` | 31 | exact 31-frame matrix; shared dependency core plus two-field typed suffix |
| 2 | 541 `ACDBASSOCVALUEDEPENDENCY` | 26 | exact 26-frame matrix; shared dependency core plus typed EvalVariant/name |
| 3 | 520 `BLOCKGRIPLOCATIONCOMPONENT` | 23 | exact 23-frame matrix; typed evaluation-expression core, no raw prefix |
| 4 | 542 `ACDBASSOCDEPENDENCY` | 20 | exact 20-frame base-core matrix; no class-local suffix |
| 5 | 545 `ACDBASSOCVARIABLE` | 20 | exact 20-frame action/variable/EvalVariant graph matrix |
| 6 | 506 `VISUALSTYLE` | 19 | exact 19-frame fixed 28-property/modifier record; no class handles |
| 7 | 21 `DIMENSION_LINEAR` | 14 | standard entity; entity common/frame writer already green; two recovered frames included |
| 8 | 549 `ASSOCDIMDEPENDENCYBODY` | 14 | exact thin string body; shared dependency-body core |
| 9 | 7 `INSERT` | 12 | standard entity and block graph, one fixture signature |
| 10 | 559 `ACDB_BLOCKREPRESENTATION_DATA` | 12 | exact marker plus represented-block reference |
| 11 | 4 `BLOCK` | 10 | exact marker entity checklist |
| 12 | 5 `ENDBLK` | 10 | exact marker entity checklist |
| 13 | 543 `BLOCKPARAMDEPENDENCYBODY` | 6 | exact thin string body; shared dependency-body core |
| 14 | 34 `VIEWPORT` | 2 | exact standard entity checklist; larger body but fully named |
| 15 | 522 `ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION` | 2 | exact marker plus protected-block reference |
| 16 | 547 `ACDB_DYNAMICBLOCKPROXYNODE` | 1 | exact typed evaluation-expression body |

**R1 matrix-ready** means the standard field matrix is named and has no known semantic gate, but an
all-frame exact probe/checklist is still required before production. These 22 cohorts contain 74
frames.

| rank | type/class | frames | next evidence needed |
|---:|---|---:|---|
| 17 | 507 `SCALE` | 17 | promoted by the 2026-08-15 bounded exact oracle below; production/facet gate pending |
| 18 | 503 `DICTIONARYVAR` | 8 | simple schema-byte/value frame signatures and CRCs |
| 19 | 516 `SORTENTSTABLE` | 7 | ordered entity/sort-handle pair signatures |
| 20 | 535 `BLOCKSTRETCHACTION` | 6 | point/index/code collection bounds and exact handle order |
| 21 | 539 `ACDBASSOCNETWORK` | 5 | promoted by the 2026-08-15 bounded oracle below; production/facet gate pending |
| 22 | 528 `BLOCKLINEARGRIP` | 4 | promoted by the 2026-08-15 bounded oracle below; production/facet gate pending |
| 23 | 505 `MATERIAL` | 3 | R2010 extension-presence proof |
| 24 | 530 `BLOCKFLIPGRIP` | 3 | promoted by the 2026-08-15 bounded oracle below; production/facet gate pending |
| 25 | 537 `BLOCKFLIPACTION` | 3 | action connection/dependency signatures |
| 26 | 521 `BLOCKMOVEACTION` | 2 | connection-point and offset signatures |
| 27 | 527 `BLOCKLINEARPARAMETER` | 2 | promoted by the 2026-08-15 bounded oracle below; production/facet gate pending |
| 28 | 533 `BLOCKALIGNMENTPARAMETER` | 2 | two-point parameter/alignment branch proof |
| 29 | 534 `BLOCKALIGNMENTGRIP` | 2 | grip orientation signatures |
| 30 | `LAYOUT` | 2 | page-setup/layout strings, UCS and viewport collection boundaries |
| 31 | `MLINESTYLE` | 1 | promoted by the 2026-08-15 bounded exact oracle below; production/facet gate pending |
| 32 | `ACDBPLACEHOLDER` | 1 | common-object-only exact frame |
| 33 | 508 `MLEADERSTYLE` | 1 | promoted by the 2026-08-15 bounded exact oracle below; production/facet gate pending |
| 34 | 532 `BLOCKVISIBILITYGRIP` | 1 | promoted by the 2026-08-15 bounded oracle below; production/facet gate pending |
| 35 | 536 `BLOCKSCALEACTION` | 1 | action/connection/dependency exact frame |
| 36 | 538 `BLOCKBASEPOINTPARAMETER` | 1 | parameter core exact frame |
| 37 | 546 `BLOCKVERTICALCONSTRAINTPARAMETER` | 1 | constraint-parameter core exact frame |
| 38 | 548 `BLOCKHORIZONTALCONSTRAINTPARAMETER` | 1 | constraint-parameter core exact frame |

**R2 semantic-name gate is now empty.** The five former R2 cohorts have code-ready typed schemas and
strict fixture invariants. They remain uncredited until their same-class decoder/writer tests pass.

| rank | type/class | frames | resolved concept |
|---:|---|---:|---|
| 39 | 540 `ACDBASSOC2DCONSTRAINTGROUP` | 4 | public new-constraint validation policy; always-null compatibility handle derived; class-discriminated constraint union |
| 40 | 529 `BLOCKFLIPPARAMETER` | 3 | `updated_flip_node_id`, validated against the owning evaluation graph |
| 41 | 517 `ACAD_EVALUATION_GRAPH` | 2 | typed edge `{from,to,ref_count,invertible,suppressed}`; intrusive indexes and cycle presence derived |
| 42 | 504 `TABLESTYLE` | 1 | R24 format version, public table-style bit flags and typed cell-style identity/class selectors |
| 43 | 531 `BLOCKVISIBILITYPARAMETER` | 1 | `evaluation_history: Stateless | Required`, fixture `Stateless` |

The readiness partition is exact: 233 R0 + 74 R1 + 0 R2 = 307 remaining.

### Fastest next 100 exact frames

Use the shared-core queue below rather than interleaving unrelated one-off standard objects:

| order | implementation slice | newly exact | cumulative | reason |
|---:|---|---:|---:|---|
| 1 | type 542 dependency base | 20 | 20 | establishes common typed dependency data/handles once; no suffix |
| 2 | type 541 value dependency | 26 | 46 | reuses 542 core; adds only version, EvalVariant and value name |
| 3 | type 544 geometry dependency | 31 | 77 | reuses 542 core; adds enabled/persistent-subentity typed suffix |
| 4 | type 520 grip-location component | 23 | **100** | one evaluation-expression/grip body; exact all-frame matrix, no class-local raw state |

This is exactly 100 frames in four body variants. The first three share the same dependency core,
role validation and most handle emission, so they should be one implementation workflow with three
tagged suffixes. Type 520 then establishes the evaluation-expression core needed by several later
dynamic-block cohorts. All four are R0 and have no unresolved semantic-name gate. After this queue,
the ledger would be 456/663 exact, 207 remaining (68.78% / 31.22%) if and only if all four
same-class runtime gates pass.

The low-risk standard fallback queue is DIMENSION_LINEAR 14, INSERT 12, BLOCK/ENDBLK 20,
DICTIONARYVAR 8 and the thin 549/559/543/522/547 bodies (35), totaling 89 before support/style
one-offs. It has lower shared count leverage and should follow the exact-100 dependency queue.

### Stale bounds and acceptance assertions

1. `architectural/.../test.rs::every_imported_object_has_a_typed_standard_body` still reports
   `652`; change the diagnostic bound to 663. This test remains red until all 307 bodies are live.
2. `real_fixture_r2010_object_frames_are_logically_identified` asserts only `>100`; it must assert
   exactly 663 unique handle/frame pairs, every CRC, and no skipped address/prefix/payload/type path.
3. The historical DIMENSION_LINEAR probe/test bounds 12 must become 14 and include `2255,2266`.
4. Type 541/545/549 bounds 23/18/12 must become 26/20/14; type 542 must be added at 20. Their
   exact CRC/signature corrections are in the recovered-frame section above.
5. Historical global values 652/634/366/377 and family values fixed-entity 80/82 remaining,
   associative 117, and accepted 286 are stale. Current values are 663 total, 307 remaining,
   fixed-entity 14 remaining, associative 126 remaining, accepted 356.
6. Do **not** change ObjFreeSpace's typed approximate registered-object value 679 to 663. It is a
   distinct standard allocator statistic, not the Handles frame count.
7. Table-record exact bounds are now 50/50 including DIMSTYLE 2; any 48/50 or 57/59 ledger gate is
   stale. LINE/ARC/LWPOLYLINE bounds remain 40/12/16 and are green.

## R2 zero-gate semantic handoff (read-only, 2026-08-14)

The former R2 bucket contains exactly 11 frames: type 540 x4, 529 x3, 517 x2, 504 x1 and 531 x1.
Primary wire declarations, public ObjectARX concepts and the corrected 663-frame fixture map now give
each value a typed standard role. No production file was changed and no Nx target was started. Evidence
is in `🧪️dwg-r2-zero-gate-frame-probe.py/.log`; downloaded primary-source extracts remain ticket-local.

### Type 529 `BLOCKFLIPPARAMETER` — 3

The group-96 scalar is `updated_flip_node_id`, not a state ordinal. It is 27, 50 and 128 while the
corresponding parameter expression node is 26, 31 and 120; each value resolves inside the owning type-517
evaluation graph. The following group-309 value is the updated-flip expression name (`UpdatedFlip` in
all three), not an opaque tooltip. Logical suffix after `DwgBlock2PointParameter`:

1. flip label, description, base-state label and flipped-state label;
2. label point;
3. `updated_flip_node_id`;
4. `updated_flip_expression_name`.

Writer derives all inherited format markers/counts, requires two distinct state labels, a finite label
point and graph membership of `updated_flip_node_id`, then emits the suffix in that order. Fixture frame
signatures `(payload,total,handle bits,data bits,class end,string bits,fill)` are
`(195,200,20,1540,455,1068,1111)`, `(207,212,20,1636,519,1100,1111)` and
`(223,228,10,1774,625,1132,11)`; CRCs are `1118:461e 111d:4d63 1156:236d`.

### Type 531 `BLOCKVISIBILITYPARAMETER` — 1

The group-91 Boolean is the evaluation-history policy. Model it as
`DwgVisibilityHistory::{Stateless, Required}`; the fixture is `Stateless`. This avoids both a generic
Boolean and a source-local name. The rest is the typed `DwgVisibilityState { name, visible_blocks,
controlled_parameters }` collection already prescribed. The fixture has 11 eligible blocks and five
states (`Open 30º`, `Open 45º`, `Open 60º`, `Open 90º`, `Closed`), with visible-block counts
4/4/4/4/3 and 21 controlled parameters per state. Every reference resolves to a block/entity or dynamic
parameter/action in the owning graph. Frame signature is `(673,679,3256,2128,415,1696,empty)`, handle
`111e`, CRC `7e73`. The writer derives all counts and rejects duplicate state names, foreign graph
members and history-required state without a valid predecessor relation.

### Type 517 `ACAD_EVALUATION_GRAPH` — 2

The logical schema is a DAG of `DwgEvaluationNode { id, expression }` and
`DwgEvaluationEdge { from, to, ref_count, invertible, suppressed }`. The wire's first-node value and
duplicate are the final/highest assigned node ID (79 and 230), derived as `max(node.id)` for these
canonically ID-ordered graphs, not `max(node.id)+1`. Per-node storage
ordinal, constant edge marker 32, and the four slots are derived first/last incoming and first/last
outgoing edge indexes. Per-edge storage ordinal, state flags, four intrusive previous/next incoming and
outgoing indexes, and optional inverse-edge index are derived from the typed edge list. The public
`AcDbEvalEdgeInfo` concepts supply the only persisted edge semantics: endpoints, reference count,
invertible and suppressed. Acyclic graphs carry no active-cycle field; a cycle-bearing candidate is
rejected until represented by the standard activated-cycle concept, never captured as a presence bit.

The two graphs contain 38/19 nodes and 43/14 edges. Expression handles resolve in node order to dynamic
block parameters, grips and actions; the observed compact handle code is 3 although the absolute DWG role
is hard-owner. Frame signatures are `(1525,1531,944,11256,11255,0,empty)` and
`(660,666,502,4778,4777,0,111111)`; CRCs `110f:03bb 1155:4919`. Decoder validates duplicate-root equality,
node/edge uniqueness, closed adjacency and a topological ordering. Writer rebuilds every intrusive index
from that ordering before exact-frame comparison.

### Type 504 `TABLESTYLE` — 1

Logical schema is `DwgTableStyle { description, bit_flags, template_style, table, title, header, data }`.
The outer RC is the derived R24 discriminator 0. The first BL is the R24 format version 0. The second BL
is the public table-style bit flags, fixture value 101. The following handle is the optional template/base
style, null here. Cell-style identity is a derived native mapping rather than snapshot state: `4/Table`,
then exactly `1/_TITLE`, `2/_HEADER`, `3/_DATA`. Counts and discriminator are derived; only description,
public flags, optional template and the four role-named cell-style formats are logical.
Cell/content/border order is the ODA 20.4.101.3/4 order already listed above.

Fixture handle `87` has `(payload,total,handle bits,data bits,string bits) =
(836,842,258,6430,554)`, description `Standard`, one reactor, extension dictionary `104`, null template
and CRC `1784` (decimal probe output 6020). Writer emits the native Table/Title/Header/Data selector order,
derives the built-in count 3 and rejects custom styles here (they belong to `CELLSTYLEMAP`).

### Type 540 `ACDBASSOC2DCONSTRAINTGROUP` — 4

The Boolean is the public `do_not_check_newly_added_constraints` evaluation policy; all four are false.
The extra handle is null in all four and therefore is a derived null compatibility slot, absent from the
logical schema. It must be emitted as null and any non-null import rejected until it maps to a named
standard relationship. The remaining handles are the owning network followed by ordered member actions;
they resolve only to value, base and geometry dependency classes.

Each node has one standard runtime class discriminator in the separated string stream and a corresponding
`AcConstraintGroupNode` identity/connection core in the main stream. The fixture's named variants are:

| typed variant | four-frame occurrences | typed suffix after inherited core |
|---|---:|---|
| `ConstrainedImplicitPoint` | 68 | constrained-geometry dependency/node, point, point kind/index, curve node |
| `PointCurveConstraint` | 68 | geometric-constraint owner, implied/active |
| `ConstrainedBoundedLine` | 31 | constrained-geometry dependency/node, line origin/direction, start/end bounds |
| `PointCoincidenceConstraint` | 28 | geometric-constraint owner, implied/active |
| `DistanceConstraint` | 20 | explicit value/dimension dependencies, direction kind, conditional direction |
| `PerpendicularConstraint` | 8 | geometric-constraint core |
| `HorizontalConstraint` | 7 | geometric-constraint core |
| `ParallelConstraint` | 8 | geometric-constraint core plus datum-line index |
| `MidPointConstraint` | 6 | geometric-constraint core |
| `EqualLengthConstraint` | 6 | geometric-constraint core |
| `ColinearConstraint` | 6 | geometric-constraint core |
| `ConstrainedDatumLine` | 5 | constrained-line geometry core |
| `FixedConstraint` | 4 | geometric-constraint core |
| `VerticalConstraint` | 2 | geometric-constraint core |

These 267 occurrences are the complete node population, not 267 typed nodes plus 82 base nodes. The
previously reported 33/250/33/33 values are maximum-node-ID watermarks; actual counts are 31/174/31/31
and exactly equal the class-string counts. Never persist the discriminator spelling independently: the
writer derives it from the union case. Shared AC1024 main order is node ID, derived connection count and
ordered connection IDs, then the typed suffix; no separate R2013 status field is present in these R2010
records.
Handles from typed suffixes follow the group/action handles in node order.

Frames `1149`, `1f2c`, `2026`, `207f` have payload/total/handle-bit signatures
`1898/1904/459`, `11375/11381/2831`, `1923/1929/467`, `1931/1937/467`; CRCs
`aa02 059f 2c14 94c5`. Work planes are the canonical XY plane, maximum node IDs are 33/250/33/33 and
node counts are 31/174/31/31. The writer
must validate orthonormal work plane, closed connection IDs, dependency ownership, variant-specific
handles and full main/string/handle exhaustion. Candidate fallback, raw node tail and source class strings
are forbidden.

### Ledger effect and implementation order

This research moves 11 frames from semantic-gated R2 to schema-ready R0 without adding runtime credit:
accepted remains 356/663 and remaining remains 307. The readiness split is now 233 R0 + 74 R1 + 0 R2.
Implement 529 and 531 after the shared block-parameter core, 517 after EvalExpr, 504 after cell-style
primitives, and 540 last after dependency/action bodies plus the typed constraint-node union. Admission
requires each same-class exact-frame gate; no cohort receives credit from this report alone.

## AcDbObjects and two-block Handles no-shadow writer handoff (read-only, 2026-08-14)

This section supersedes the stale live-gap wording in the earlier reconstruction section. It reconciles
the current logical schema, current IO implementation and corrected 663-frame fixture probes into one
code-ready serializer boundary. No production file was edited and no Nx target was started for this
handoff.

### Current implementation reconciliation

The persistent authority is
`DwgSnapshot::drawing.objects: Vec<DwgLogicalObject>` in the AC1024 snapshot component. Each object owns
its semantic handle, type/class/category, named owner/reactor/extension-dictionary references, typed EED
and an optional tagged body. The live tagged body union currently dispatches Dictionary, TableControl,
TableRecord, XRecord, Entity (LINE/ARC/LWPOLYLINE), AssociativeDependency and
AssociativeValueDependency. This is the correct architectural seam, but it does not yet cover all 663
fixture bodies.

| live location | reconciled state | required change |
|---|---|---|
| IO `decode_r2004_handle_map`, line 2295 | handle and address bases are already reset inside each non-empty block, correcting the old 652-frame loss | validate every big-endian block CRC, require exact payload consumption, validate the size-two terminator and its CRC, and reject every trailing byte |
| IO `r2010_object_inventory`, line 2392 | remains diagnostic and silently `continue`s after invalid address/prefix/payload/type/handle reads | make it call the strict frame-index/parser path or return a handle-qualified error; it may never define an acceptance count by skipping |
| IO `decode_r2004_object_records`, line 3724 | validates frame CRC/self handle and bounded streams for implemented families, but traverses the handle map's handle order | parse the complete map, sort a transient copy by address, prove contiguous coverage, and store semantic objects in that address order; do not store addresses |
| IO `finish_r2010_object_frame`, line 2749 | correct common frame boundary: one-fill, MS, UMC handle-bit length and little-endian CRC | retain as the sole finalizer for every typed AC1024 object encoder |
| IO family encoders, lines 2772–3519 | exact encoders exist for the seven live body families and three entity cases | add one exhaustive `encode_r2010_object_frame` dispatcher; reject `body=None`, a body/type mismatch, or any not-yet-typed class atomically |
| IO `encode_r2004_canonical`, line 642 | converts the reduced geometry view through the unrelated `dwg_to_bytes` codec, labels that output section ID 1, leaves Handles empty, invents eight section IDs, reverse-sorts them and uses literal-only D2 | replace the reduced-drawing call with the paired object/handle materializer below; feed its products into fixture section IDs 7 and 4 before page slicing; then use the complete 13 named-section plan |
| IO `encode_section_info`, line 606 and `write_data_page`, line 561 | operate on ephemeral content/pages, which is the correct lifetime | derive descriptors only after every named payload and compressed page is final; correct max-allocation/compression/encryption fields, page word 4 and deterministic fill as documented earlier |

The snapshot's `Vec` order is not incidental container layout. ODA permits logical objects in any order,
and the imported address order is the producer's standard object ordering. Preserve that order through
DSL/pack, diff, mutation and inverse. Handle order is a separate derived index used only while writing the
Handles section. Existing-object upsert must retain its vector position; insert/delete/move must be
explicit ordered semantic operations so an inverse restores the original order.

### Serializer-only API and ownership

Keep every helper private to AC1024 IO. A suitable decomposition is:

```text
encode_r2010_object_frame(&DwgLogicalObject) -> Result<Vec<u8>>
materialize_r2010_objects(&[DwgLogicalObject]) -> Result<(Vec<u8>, Vec<(u64, u64)>)>
materialize_r2004_handles(&[(u64, u64)]) -> Result<Vec<u8>>
materialize_r2004_named_sections(&DwgSnapshot) -> Result<Vec<CanonicalR2004Section>>
encode_r2004_pages(Vec<CanonicalR2004Section>) -> Result<Vec<u8>>
```

The byte vectors and `(handle,address)` pairs are serializer-local temporaries and are dropped after the
native output is complete. They must not appear in the snapshot, artifact, diff, mutation, DSL/pack,
facets or public APIs. The first function is an exhaustive match over both `DwgLogicalObjectBody` and
the body's typed subvariant; it validates the declared `type_code`, class name/category and all named
handle roles before calling the family encoder. `referenced_handles` is never a fallback body or a way to
replay an unsupported handle stream.

### Strict import and logical-order recovery

1. Decode the entire Handles payload block by block. Read the two-byte big-endian declared size; for a
   non-empty block require `size > 2`, set both bases to zero, parse pairs only through
   `block_start + size`, then compare the following two-byte big-endian CRC with CRC-16 seed `0xC0C1`
   over the declared-size bytes plus pair payload.
2. For the size-two block require empty payload, following CRC `01d0`, physical end exactly 2,085 for
   the fixture, and no ignored bytes. Require unique nonzero handles and nonnegative in-range addresses.
3. Clone/sort the resulting pairs by address. Require first address 4; interpret bytes `0..4` as the
   standard R18+ AcDbObjects prologue and validate `ca0d0000` for this fixture. For each address, decode
   MS then UMC, align to the payload, calculate `frame_end = prefix + payload_size + 2`, verify the
   little-endian frame CRC over prefix+payload, and require `frame_end == next_address`. The last frame
   must end at the logical section length 213,182.
4. Decode BOT/self handle/EED/main/string/handle streams with the typed body decoder. Require self
   handle equal to the map key, exact main/string/handle consumption and the body-specific terminal
   one-fill. Any unknown body or residual bit is an import error, not `body=None` and not a raw tail.
5. Push objects into `drawing.objects` in this proven address order. Discard the temporary address map,
   sizes, bit selectors, CRCs and section bytes after semantic projection.

The address-order handle oracle begins
`1,1fa4,1fa9,1faa,1fab,1fac,1fad,1fae` and ends
`20e4,20ee,20ef,20f0,20f1,114c,2029,2082`; its little-endian-u64 SHA-256 is
`3fdcc6fa7bc98d3fce64072ba38501ef29cb87ce70d18fc9fe0d6d8804bc70ad`. Import must assert
663 adjacent, CRC-valid frames rather than the weaker `>100` inventory gate.

### AcDbObjects materialization

1. Validate all logical object handles are unique and nonzero, every class/type/body combination is
   supported, every named reference resolves where the standard requires it, and the ordered collection
   is complete. Do this before emitting native output so a dirty unsupported snapshot fails atomically.
2. Start a serializer-local output with the standard four-byte R18+ prologue `ca0d0000`. Its value is a
   version-standard writer rule, never stored syntax.
3. Iterate `drawing.objects` in vector order. Record `(object.handle, output.len())` immediately before
   dispatch, append the exact typed frame returned by `encode_r2010_object_frame`, and advance only by
   the emitted length.
4. Every family writer emits BOT, self handle, typed EED, typed main and R2010 string concepts, then
   named handle roles in standard order. `finish_r2010_object_frame` appends terminal one-fill until the
   combined data/handle payload is byte-aligned, prefixes `MS(payload byte count)` and
   `UMC(handle-stream bit count)`, and appends the little-endian seed-`C0C1` CRC over prefix+payload.
5. Prove the emitted intervals adjacent and the pair set one-to-one. Only then pass the pair index to
   the Handles materializer.

Fixture acceptance is exact length 213,182 and SHA-256
`d50dff10271442fe9c7d8812ce40a29dc38f3665fe34aa5ffd1fecffdb323cc6`. First addresses are
`1@4,1fa4@45,1fa9@144,1faa@254,1fab@362,1fac@460,1fad@527,1fae@594`; last addresses are
`20e4@212729,20ee@212772,20ef@212892,20f0@212939,20f1@213056,114c@213099,2029@213132,2082@213157`.

### Two-block Handles materialization

1. Copy the transient `(handle,address)` pairs and sort them by strictly ascending handle. Do not reorder
   the logical object vector.
2. Open a block with a two-byte size placeholder and set `last_handle=0,last_address=0`.
3. For each pair, encode `handle-last_handle` as unsigned modular char (low seven-bit groups first,
   continuation in bit 7). Encode `address-last_address` as signed modular char: low magnitude groups
   first; the terminal byte uses bit 6 for a negative sign. A positive terminal magnitude group with bit
   6 set must be continued with another terminal group so it cannot be misread as negative.
4. Append the complete pair, update both bases, and only then test the producer boundary. Close when
   `2 + pair_payload_len > 2030`; this post-pair rule intentionally makes fixture block 0's declared size
   2,033 rather than preflighting it down to 2,030/2,032.
5. Backpatch the declared size in big-endian order. Append big-endian CRC-16 seed `0xC0C1` over size
   bytes plus pair payload. Open the next block and reset **both** bases to zero.
6. After the remaining pairs, close the non-empty block identically. Append the empty terminator
   `0002 01d0`. Emit no fill, zero trailer or extra sentinel.

Fixture blocks are `(offset,size,pair bytes,entries,crc)` =
`(0,2033,2031,652,e277)`, `(2035,44,42,11,56f0)`, and `(2081,2,0,0,01d0)`.
Block 0 spans handles `1..2254`; block 1 spans `2255..226a` and deliberately returns to addresses
4,894/33,022/54,578/56,336/208,647/... because its address base reset. Sign histograms are 507
positive/145 negative and 9 positive/2 negative. Exact output is 2,085 bytes with SHA-256
`5e877e4b7374d7c343f37052f0d8e930a37612b3a0f04aadb77b9f6afc9b90df`.

### Handoff into named sections, D2 pages and outer container

`encode_r2004_canonical` must materialize all named semantic payloads before it allocates any page:

1. Materialize AcDbObjects and Handles as a pair; assign fixture section identities
   `7/AcDbObjects` and `4/Handles`. Materialize Header 1, AuxHeader 2, Classes 3, Template 5,
   ObjFreeSpace 6, RevHistory 8, SummaryInfo 9, Preview 10, AppInfo 11, AppInfoHistory 12 and
   FileDepList 13 from typed logical state. Preview image pixels/palette are legitimate semantic media;
   bitmap container headers/padding are derived.
2. Use descriptor order `0,13,12,11,10,9,8,7,6,5,4,3,2,1`. Use physical ordinary-page order
   1 SummaryInfo, 2 Preview, 3 AppInfo, 4 AppInfoHistory, 5 FileDepList, 6 RevHistory, 7–14
   AcDbObjects, 15 ObjFreeSpace, 16 Template, 17 Handles, 18 Classes, 19 AuxHeader, 20 Header. IDs
   21/22 remain absent; 23/24 are Section Info/Page Map system pages.
3. Slice each algorithm-2 payload independently at `0x7400`; D2-compress each slice with the already
   researched exact Autodesk match policy. Objects slices have logical sizes
   `29696 × 7 + 5310` and compressed sizes
   `17145,11080,4380,2246,3378,4448,3490,1711`; Handles compresses 2,085 to 1,907.
4. Allocate pages only after compression. Objects pages 7–14 use addresses
   `16260,1a580,1d100,1e240,1eb40,1f8a0,20a20,21800` and allocations
   `17184,11136,4416,2304,3424,4480,3552,1760`; Handles page 17 is `22080`, allocation 1,952.
   These are fixture oracles, not snapshot inputs.
5. Generate each clear 32-byte data-page header from the final section ID, payload length, allocation,
   logical offset and seeded checksums; encrypt it from its generated file address and generate LCG fill.
   Build Section Info only now, because its logical lengths/page counts/compressed lengths depend on all
   preceding stages. Then build Page Map, system checksums/trailers/fill, primary header, encrypted second
   header and EOF as already specified in the outer-writer matrix.

The boundary is therefore strictly:

```text
ordered typed logical objects
  -> exact typed frames + ephemeral object addresses
  -> AcDbObjects + derived handle-sorted two-block Handles
  -> named logical section byte values
  -> per-section 0x7400 slices + deterministic D2 payloads
  -> generated descriptors/pages/checksums/fill/headers
  -> native DWG bytes
```

No later stage may feed state backward into the snapshot. Mutating one frame may change every subsequent
address, modular address delta, Handles block boundary, compressed page, page checksum and outer address;
that cascade is expected derivation. Applying the inverse must restore both logical order and the exact
object/handle section hashes, then the complete original native bytes.

### No-shadow and acceptance gates

Forbid these fields/tokens in Rust schema and every facet: source/native/section/page/frame bytes;
AcDbObjects prologue bytes; object address/frame length; MS/UMC selector or encoded value; data/string/
handle bit offsets; terminal fill; CRC/checksum; handle-map pairs/deltas/block sizes/block boundaries;
section IDs/order/compression flags as imported layout; D2 tokens/compressed payloads; page number/address/
allocation/fill; encrypted header or second-header bytes. Serializer-local variables with these meanings
are required and valid; persisted forms are not.

Extend the existing fixture tests, without a new test file, in this order:

1. strict import: three Handles blocks, 663 unique pairs, every map/frame CRC, adjacent address intervals,
   exact type census and zero skipped path;
2. all 663 typed body decoders and matching typed frame encoders; unsupported cohorts keep whole-file
   export red rather than being omitted;
3. exact AcDbObjects length/hash and Handles length/hash plus the two non-empty block/terminator oracles;
4. exact D2 payload per page, clear/encrypted data headers, allocations/fill, Section Info/Page Map,
   system trailers, second header, file length and original fixture SHA-256;
5. original fixture as the baseline through logical snapshot, DSL, pack, diff/apply/inverse/absorb,
   mutation/inverse, analyzer/composer and native IO. Never canonicalize an initial export into a substitute
   baseline.

The implementation is not accepted from the 356 currently green exact-frame cohort tests. It becomes
accepted only when all 663 typed frames and the full native lifecycle return the original
`architectural_example.dwg` byte-for-byte.

## Accepted-ledger reconciliation after the dependency-core wave (read-only, 2026-08-14)

The implementation lane reports exact-frame acceptance for type 542 x20, type 541 x26 and type 544
x31. This reconciliation credits exactly those 77 frames against the prior evidence-backed 356; this
read-only lane did not rerun Nx. The strict ledger is therefore **433/663 accepted, 230 remaining**
(65.31% / 34.69%). Whole-file equality remains a separate red gate until every remaining typed body and
the outer writer are complete.

### Family totals

| family | fixture | accepted | remaining | next exact-ready frontier |
|---|---:|---:|---:|---|
| fixed entities | 82 | 68 | 14 | type 21 DIMENSION_LINEAR x14 |
| dictionary/XRECORD spine | 237 | 229 | 8 | type 503 DICTIONARYVAR x8 is matrix-ready, not exact-ready |
| block/entity graph | 32 | 0 | 32 | BLOCK x10, ENDBLK x10 and INSERT x12 |
| table controls/records | 59 | 59 | 0 | complete |
| fixed support | 6 | 0 | 6 | VIEWPORT x2 exact-ready; four matrix-ready support frames |
| style/context custom | 50 | 0 | 50 | type 506 VISUALSTYLE x19 first; remaining exact/matrix rows are enumerated below |
| dynamic-block custom | 71 | 0 | 71 | type 520 x23 first; remaining exact/matrix rows are enumerated below |
| associative custom | 126 | 77 | 49 | type 545 x20 and type 549 x14 first; remaining exact/matrix rows are enumerated below |
| **total** | **663** | **433** | **230** | **156 exact-ready + 74 matrix-ready** |

The accepted 433 are exactly XRECORD 145 + DICTIONARY/WDFLT 84 + table controls 9 + table records 50
+ LINE 40 + ARC 12 + LWPOLYLINE 16 + dependency base 20 + value dependency 26 + geometry dependency
31. No other research-only cohort receives runtime credit.

### Remaining exact-ready cohorts: 156

R0 means the ticket already contains an all-frame bounded field/handle prescription, exact signatures/
CRCs and a symmetric writer order. Rows are grouped by the recommended implementation wave, not merely
sorted by count.

| wave | type/class | frames | readiness and reuse |
|---|---|---:|---|
| immediate | 520 `BLOCKGRIPLOCATIONCOMPONENT` | 23 | completes the original dependency-wave 100 and establishes the evaluation-expression/grip core |
| next-100 | 547 `ACDB_DYNAMICBLOCKPROXYNODE` | 1 | direct thin reuse of type 520's evaluation-expression core |
| next-100 | 545 `ACDBASSOCVARIABLE` | 20 | reuses accepted type 541 EvalVariant/value concepts and the accepted dependency core |
| next-100 | 549 `ASSOCDIMDEPENDENCYBODY` | 14 | exact thin typed dependency-body string/core, including two recovered frames |
| next-100 | 4 `BLOCK` | 10 | exact marker-entity body; implement with ENDBLK before INSERT |
| next-100 | 5 `ENDBLK` | 10 | exact marker-entity body and reciprocal block ownership |
| next-100 | 7 `INSERT` | 12 | exact entity checklist; consumes the now-typed block graph |
| next-100 | 21 `DIMENSION_LINEAR` | 14 | exact standard entity matrix; common entity/frame primitives are already accepted |
| next-100 | 506 `VISUALSTYLE` | 19 | exact fixed 28-property/modifier record with no class-local handles |
| residual-R0 | 559 `ACDB_BLOCKREPRESENTATION_DATA` | 12 | thin represented-block marker/reference |
| residual-R0 | 543 `BLOCKPARAMDEPENDENCYBODY` | 6 | thin typed parameter dependency-body string/core |
| residual-R0 | 540 `ACDBASSOC2DCONSTRAINTGROUP` | 4 | code-ready typed constraint-node union; high implementation complexity |
| residual-R0 | 529 `BLOCKFLIPPARAMETER` | 3 | typed updated-flip node/expression; requires evaluation-graph membership validation |
| residual-R0 | 34 `VIEWPORT` | 2 | exact standard entity matrix |
| residual-R0 | 522 `ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION` | 2 | thin protected-block marker/reference |
| residual-R0 | 517 `ACAD_EVALUATION_GRAPH` | 2 | typed nodes/semantic edges; writer derives intrusive indexes |
| residual-R0 | 504 `TABLESTYLE` | 1 | typed cell-style identities and public flags |
| residual-R0 | 531 `BLOCKVISIBILITYPARAMETER` | 1 | typed states and stateless evaluation-history policy |
| **total R0** |  | **156** | **23 immediate + 100 next + 33 residual** |

The arithmetic is exact: 23 + (1+20+14+10+10+12+14+19) +
(12+6+4+3+2+2+2+1+1) = 156.

### Remaining matrix-ready cohorts: 74

These retain no known semantic-name gate but still require their all-frame bounded probe and exact writer
checklist before implementation credit. Counts remain unchanged from the prior ledger:

| type/class | frames | type/class | frames |
|---|---:|---|---:|
| 507 SCALE | 17 | 503 DICTIONARYVAR | 8 |
| 516 SORTENTSTABLE | 7 | 535 BLOCKSTRETCHACTION | 6 |
| 539 ACDBASSOCNETWORK | 5 | 528 BLOCKLINEARGRIP | 4 |
| 505 MATERIAL | 3 | 530 BLOCKFLIPGRIP | 3 |
| 537 BLOCKFLIPACTION | 3 | 521 BLOCKMOVEACTION | 2 |
| 527 BLOCKLINEARPARAMETER | 2 | 533 BLOCKALIGNMENTPARAMETER | 2 |
| 534 BLOCKALIGNMENTGRIP | 2 | LAYOUT | 2 |
| MLINESTYLE | 1 | ACDBPLACEHOLDER | 1 |
| 508 MLEADERSTYLE | 1 | 532 BLOCKVISIBILITYGRIP | 1 |
| 536 BLOCKSCALEACTION | 1 | 538 BLOCKBASEPOINTPARAMETER | 1 |
| 546 BLOCKVERTICALCONSTRAINTPARAMETER | 1 | 548 BLOCKHORIZONTALCONSTRAINTPARAMETER | 1 |
| **total** | **74** |  |  |

Thus the strict remainder is 156 R0 + 74 R1 = 230. R2 remains empty.

### Immediate type 520 and the subsequent fastest exact 100

Type 520 x23 remains the immediate next cohort because it is the final slice of the previously prescribed
100-frame dependency wave. If its same-class exact runtime gate passes, the ledger becomes **456/663,
207 remaining** (68.78% / 31.22%). Do not pre-credit it from its research matrix.

After type 520, the fastest subsequent 100 uses only already exact-ready cohorts and deliberately follows
shared primitives and graph dependencies:

| order | implementation slice | newly exact | cumulative after type 520 | reason |
|---:|---|---:|---:|---|
| 1 | type 547 dynamic-block proxy node | 1 | 1 | smallest direct reuse of the just-landed type-520 evaluation-expression core |
| 2 | type 545 associative variable | 20 | 21 | reuses accepted 541 EvalVariant/value and 542 dependency concepts |
| 3 | type 549 dimension dependency body | 14 | 35 | thin typed body and completes the corrected recovered-frame cohort |
| 4 | BLOCK + ENDBLK | 20 | 55 | one paired marker/entity workflow establishes reciprocal block boundaries |
| 5 | INSERT | 12 | 67 | builds on the typed block graph and existing entity-common writer |
| 6 | DIMENSION_LINEAR | 14 | 81 | existing exact standard-entity checklist and accepted frame primitives |
| 7 | VISUALSTYLE | 19 | **100** | independent fixed-size exact matrix closes the count without research-gated frames |

If type 520 and this subsequent queue all pass their full same-class original-frame gates, the ledger is
**556/663 accepted, 107 remaining** (83.86% / 16.14%). The residual 107 are exactly 33 R0 plus 74 R1.
The 33 R0 are types 559/543/540/529/34/522/517/504/531; they should follow in increasing dependency and
writer complexity, with type 540 last. No queue result changes the independent requirement for strict
663-object import, exact AcDbObjects/Handles reconstruction and whole-file lifecycle equality.

## Type 545 ACDBASSOCVARIABLE live-readiness oracle (read-only, 2026-08-14)

This section reconciles the original 18-frame type-545 probe with the two frames recovered after the
Handles block-local base fix. The existing ticket probe was rerun read-only; no production file was
edited and no Nx target was started. The complete cohort is **20**, not 18. It contains 14 constant/no-
reference variables and six symbolic variables with one value-dependency binding each.

### Semantic schema

The logical body is an associative action plus the standard variable state:

```text
DwgAssociativeActionDependency {
    owned: bool,
    dependency_handle: Handle<DwgAssociativeDependency>,
}

DwgAssociativeAction {
    status: UpToDate,
    owning_network_handle: Handle<DwgAssociativeNetwork>,
    action_body_handle: Option<Handle<DwgAssociativeActionBody>>,
    action_index: i32,
    maximum_dependency_index: i32,
    dependencies: Vec<DwgAssociativeActionDependency>,
}

DwgAssociativeVariable {
    action: DwgAssociativeAction,
    name: String,
    expression: String,
    evaluator_id: String,
    description: String,
    evaluated_value: DwgEvaluationVariant,
    mergeable: bool,
    mergeable_variable_name: Option<String>,
    must_merge: bool,
    referenced_value_dependency_handles: Vec<Handle<DwgAssociativeValueDependency>>,
}
```

The common object's owner and reactors remain on `DwgLogicalObject`; owning network and action body are
distinct named action concepts even though owner and network resolve to the same object throughout this
fixture. `maximum_dependency_index` is a public action allocation concept, not an encoded selector; the
fixture requires it to agree with the value-dependency bindings (0/empty or 1/one). Native class versions,
counts, binding version, EvalVariant group code, compact handle nibbles, string offsets and terminal fill
are derived and must not enter the schema. A generic `referenced_handles` bag is not part of this body.

### Exact main and string streams

After BOT, self handle and the zero-EED terminator, all 20 frames emit:

1. common-object reactor count `BL` (18 use count 1, two use count 4), then missing-extension-dictionary
   `B=true`;
2. action version `BS=1`, action status `BL=0`, action index `BL`, maximum dependency index `BL`, and
   inherited action-dependency count `BL=0`; the AC1024 action version omits the R2013 extension;
3. variable version `BL=2`;
4. typed `EvalVariant` group code `BS=90`, followed by its Integer32 `BL` value;
5. mergeable `B=false`, must-merge `B=false`;
6. only when maximum dependency index is 1, referenced-value-dependency count `BL=1`;
7. reference-binding version `BS=0`.

The independent R2010 string stream contains, in declaration order, `name`, `expression`,
`evaluator_id`, `description`, then `mergeable_variable_name` only when mergeable. All 20 have evaluator
`AcDbCalc:1.0`, empty description and no mergeable-variable string. The stream is nevertheless present
on every frame and must be consumed exactly.

Action indices are 2 x3, 4 x3, and 5/6/7/8/12/13/14/15/16/17/26/30/31/32 x1. Maximum dependency
index is 0 via BL2 on 14 and 1 via BL1 on six. Evaluated values are Integer32
`6 x10, 144 x4, 4/36/42/60/480/600 x1`; 480 and 600 use the BL0 branch while the other 18 use BL1.
String-bit histograms are `232:2,256:2,272:2,336:2,352:4,400:2,416:6`; class-end histograms are
`107:12,125:6,131:2`.

The name/expression census is:

- names `W x3`, `H x3`, then one each of `bldDEPTH`, `bldWALL`, `bldWIDTH`, `Wall1`, `Wall2`,
  `Wall3`, `iWALL1`, `iWALL2`, `iWALL3`, `iWALL4`, `Room1`, `Room2`, `Room3`, `hall`;
- expressions empty x2, `6` x4, `144` x4, `bldWALL` x3, `iWALL1` x3, then `42`, `60`, `480`,
  `600` x1.

### Five original binding cases and the recovered sixth

The requested five original binding cases remain a useful regression subset:

| variable | semantic expression | owned/referenced value dependency | dependency target | variable reactor |
|---:|---|---:|---:|---:|
| `2123` `Wall2` | `bldWALL` | `2127` | `211e` `bldWALL` | `2124` |
| `2129` `Wall3` | `bldWALL` | `212d` | `211e` `bldWALL` | `212a` |
| `215c` `iWALL2` | `iWALL1` | `2160` | `2152` `iWALL1` | `215d` |
| `2167` `iWALL3` | `iWALL1` | `216b` | `2152` `iWALL1` | `2168` |
| `21eb` `iWALL4` | `iWALL1` | `21f0` | `2152` `iWALL1` | `21ec` |

The recovered frame `2256 Wall1 = bldWALL` is a mandatory **sixth** binding: it owns/references
dependency `225a`, which targets `211e bldWALL`; reactor `2257` is the constraint-group cache dependency.
Any live gate that asserts five bindings or 18 variables is stale and still drops the second Handles
block. Each type-541 dependency's common owner must be its type-545 variable, its dependent-on target
must have the expression's symbol name, its Integer32 cache must equal the variable's unchanged evaluated
value, and the variable's reactor/cache graph must close. The standard dependency-chain links also must
resolve, but their compact nibbles or imported chain positions are never persisted.

### Exact handle stream and native role codes

The handle stream follows all main/string concepts in this order:

1. common owner;
2. common reactors in logical order;
3. optional extension dictionary (absent on all 20);
4. action owning network;
5. nullable action body;
6. referenced value-dependency handles in binding order.

The derived native code histograms are:

- one-reactor/no-binding: 12 frames; codes `(12,4,4,3)` x8 and `(10,4,4,3)` x4;
- one-reactor/one-binding: six frames; codes `(12,4,4,3,3)` x6;
- four-reactor/no-binding: two frames; codes `(12,4,4,4,4,4,3)` x2.

Thus common owner uses relative code 12 on 16 and code 10 on four, every reactor and owning-network role
uses code 4, nullable action body uses code 3, and every bound value dependency uses code 3. These nibbles
are serializer decisions derived from role/base/target. Store only absolute semantic handles. Owner and
network groups are `1148 x2`, `2034 x2`, `208d x2`, and `1f26 x14`; action body is null 20/20.

### Complete 20-frame signatures, fill and CRC

| payload/frame; handle/data/class/string bits; fill | fixture handles |
|---|---|
| `54/59; 76/356/107/232; 1111` | `1153 1154` |
| `57/62; 76/380/107/256; 1111` | `2031 208a` |
| `59/64; 76/396/107/272; 1111` | `2030 2089` |
| `68/73; 84/460/107/336; 1111` | `2251` |
| `70/75; 84/476/107/352; 1111` | `2157 2162 2178` |
| `77/83; 156/460/107/336; 1111` | `2152` (four reactors) |
| `79/84; 84/548/131/400; 1111` | `2108 2267` |
| `79/85; 156/476/107/352; 1111` | `211e` (four reactors; four-byte frame prefix) |
| `83/88; 106/558/125/416; 11` | `2123 2129 215c 2167 21eb 2256` |

Prefix size is three bytes on 18 frames and four on two. The 14 no-binding frames terminate in four one
bits; the six binding frames terminate in two one bits. CRC-16 is little-endian on the frame and seeded
with `C0C1` over MS/UMC/payload:

`1153:0be9 1154:247f 2030:e0d0 2031:a33d 2089:23ec 208a:1486 2108:a09f
211e:66e2 2123:e30c 2129:a074 2152:9f55 2157:f6ce 215c:82dc 2162:0ecb
2167:30eb 2178:dd3a 21eb:99a4 2251:8618 2256:0f2a 2267:d5df`.

The exact writer order is common main, action main, variable main, independent string stream, common
handles, action handles, binding handles, terminal one-fill, then CRC. Reader acceptance requires exact
main/string/handle exhaustion; writer acceptance requires every full frame above to match, not only the
five original binding frames.

### Append-only Rust and facet tags

The live Rust body union now owns stable DSL kinds 0..9 and payload field IDs 1..10, with
`BlockGripLocationComponent` at kind 8 / payload 9 and `DynamicBlockProxyNode` at kind 9 / payload 10.
Append type 545 as `AssociativeVariable(DwgAssociativeVariable)` at **kind 10 / payload field 11**. Do
not insert it before an existing body or renumber any existing tag. Add the new
action/dependency/variable records before the body union and derive their DSL fields structurally;
versions, counts, selectors and fill receive no DSL fields.

The append-only protobuf body chain is associative geometry dependency 10, block grip location component
11, dynamic block proxy node 12, then associative variable **13**. Preserve those published numbers in
snapshot, artifact and diff protobuf mirrors. Suggested message tags are append-only and semantic:

```text
DwgAssociativeActionDependency: owned=1, dependency_handle=2
DwgAssociativeAction: status=1, owning_network_handle=2, action_body_handle=3,
                      action_index=4, maximum_dependency_index=5, dependencies=6
DwgAssociativeVariable: action=1, name=2, expression=3, evaluator_id=4, description=5,
                        evaluated_value=6, mergeable=7, mergeable_variable_name=8,
                        must_merge=9, referenced_value_dependency_handles=10
DwgLogicalObjectBody: ... geometry_dependency=10, block_grip_location_component=11,
                      dynamic_block_proxy_node=12, associative_variable=13
```

TypeScript appends `{ kind: 'associativeVariable'; value: DwgAssociativeVariable }`; GraphQL appends
`DwgAssociativeVariable` to `DwgLogicalObjectBody`; JSON/text/binary facets append the same named fields
in their structured schema order. Never expose native type 545, version 1/2/0, group code 90, BL/BS
branches, string/handle bit lengths, compact handle codes, fill or CRC as facet fields.

### Live acceptance gate

Extend the existing fixture test only. Require 20 attempted/decoded/re-encoded type-545 frames, 14/6
max-index and binding branches, 18/2 reactor branches, the six graph-closed bindings, nine exact signature
groups and every CRC above. Add a constant-value mutation (for example `bldWIDTH` 480), a symbolic
binding mutation and their inverses; invalid symbol, wrong dependency owner/target/cache, duplicate
binding, non-null action body mismatch or cycle must reject atomically. Credit the cohort only after exact
frames survive logical DSL/pack, diff/apply/inverse/absorb, mutation/inverse, analyzer/composer and native
IO with no source/raw/frame shadow state.

## Type 549 ASSOCDIMDEPENDENCYBODY live-readiness oracle (read-only, 2026-08-14)

This reconciliation combines the twelve original P11 frames with `2259` and `226a` recovered by the
second Handles block. The existing ticket probes were rerun read-only; no production file was edited and
no Nx target was started. The exact cohort is **14**, and every frame reduces to one typed semantic name
plus its common-object owner relationship. No class-local handle, raw prefix or diagnostic tail exists.

### Logical body and graph meaning

```text
DwgAssociativeDimensionDependencyBody {
    name: String,
}
```

The native dependency-body version 1, dimension-base version 1 and class version 1 are fixed AC1024
writer rules. They are not persisted fields. The `DwgLogicalObject.owner_handle` is the sole semantic
handle and must resolve to a type-542 `ACDBASSOCDEPENDENCY`; it is not duplicated inside the body.

The complete closed graph invariant is:

```text
DIMENSION_LINEAR --reactor--> ACDBASSOCDEPENDENCY --dependency_body_handle--> type 549 body
        ^                              |                                      |
        +--------- dependent_on -------+---------------- owner ---------------+
```

For each of the 14 triples, the type-542 dependency is a write dependency, is attached/delegating,
targets the dimension, names this body as its dependency body, and has a body ID appropriate to the
constraint graph. The dimension's reactor list contains the dependency, the body's owner points back to
that dependency, and `body.name == dimension.user_text`. The recovered triples are
`2255 -> 2258 -> 2259` with `Wall1=bldWALL` and `2266 -> 2269 -> 226a` with
`bldWIDTH=40'-0"`. Any decoder/test capped at twelve loses valid semantic objects.

The full body/owner census is:

| body | owner dependency | semantic name |
|---:|---:|---|
| `210b` | `210a` | `bldDEPTH=50'-0"` |
| `2121` | `2120` | `bldWALL=6"` |
| `2126` | `2125` | `Wall2=bldWALL` |
| `212c` | `212b` | `Wall3=bldWALL` |
| `2155` | `2154` | `iWALL1=6"` |
| `215a` | `2159` | `Room1=12'-0"` |
| `215f` | `215e` | `iWALL2=iWALL1` |
| `2165` | `2164` | `Room2=12'-0"` |
| `216a` | `2169` | `iWALL3=iWALL1` |
| `217b` | `217a` | `Room3=12'-0"` |
| `21ee` | `21ed` | `iWALL4=iWALL1` |
| `2254` | `2253` | `hall=12'-0"` |
| `2259` | `2258` | `Wall1=bldWALL` |
| `226a` | `2269` | `bldWIDTH=40'-0"` |

### Exact main, string and handle order

After BOT, self handle and zero-EED terminator, the physical streams are:

1. main: common reactor count `BL=0` via selector 2, missing-extension-dictionary `B=true`;
2. main: dependency-body version `BS=1` via selector 1;
3. main: dimension-base version `BS=1` via selector 1;
4. string: exactly one present TU `name` in the independent R2010 string stream;
5. main: dimension dependency-body class version `BS=1` via selector 1;
6. handle: common owner only, native code 8;
7. no terminal fill; append little-endian frame CRC-16 seeded `C0C1` over MS/UMC/payload.

All frames use direct BOT selector 1, self-handle code 0, no EED, no reactors, missing xdictionary,
handle-stream length 8 bits and class-main end bit 69. The TU is the only variable-width concept.
Versions, BS/BL selectors, TU length/presence encoding, string offset, owner compact code, frame sizes and
CRC are derived serializer state.

### Six signatures and fourteen CRCs

| payload/frame; handle/data/class/string bits; fill | fixture handles |
|---|---|
| `31/36; 8/240/69/154; none` | `2155` |
| `33/38; 8/256/69/170; none` | `2121` |
| `35/40; 8/272/69/186; none` | `2254` |
| `37/42; 8/288/69/202; none` | `215a 2165 217b` |
| `39/44; 8/304/69/218; none` | `2126 212c 215f 216a 21ee 2259` |
| `43/48; 8/336/69/250; none` | `210b 226a` |

Every frame prefix is three bytes. `handle_bits=8` leaves the complete payload byte-aligned, so a writer
that appends one-fill is wrong. CRC oracle:

`210b:7211 2121:bbf5 2126:79b5 212c:b696 2155:f754 215a:2065
215f:be2a 2165:6eee 216a:89d3 217b:2397 21ee:057c 2254:eadd
2259:b14c 226a:22ad`.

The exact writer sequence is common main, the two inherited derived versions, derived class version,
independent name string, common owner, zero fill, CRC. The logical declaration may present `name` before
class version, but the R2010 materializer must preserve the measured split: all main bits end at 69 while
the TU occupies the independent string stream before the handle stream.

### Append-only Rust and facet tags

Do not consume the tags owned by types 547 and 545. The live Rust body union assigns type 547 kind 9 /
payload 10 and type 545 kind 10 / payload 11. Append
`AssociativeDimensionDependencyBody(DwgAssociativeDimensionDependencyBody)` at **kind 11 / payload
field 12**. The body record has only DSL field `name=0`; versions, stream metadata and owner are not body
fields.

Preserve the live protobuf body assignments through type 545 and append type 549 at field **14** in
snapshot, artifact and diff mirrors:

```text
DwgAssociativeDimensionDependencyBody: name=1
DwgLogicalObjectBody:
  ... associative_geometry_dependency=10
  block_grip_location_component=11
  dynamic_block_proxy_node=12
  associative_variable=13
  associative_dimension_dependency_body=14
```

TypeScript appends
`{ kind: 'associativeDimensionDependencyBody'; value: DwgAssociativeDimensionDependencyBody }` after
the reserved type-545 member. GraphQL appends `DwgAssociativeDimensionDependencyBody` to the body union.
JSON, text and binary facets append the one named semantic field and union case without a native type
number, version, encoded TU, owner-role nibble, fill or CRC field. The common owner remains the existing
`DwgLogicalObject.ownerHandle`/field 5 in all mirrors.

### Strict lifecycle assertions

Extend the existing fixture test only and require:

1. 14 attempted, decoded and re-encoded type-549 frames, with all six signature groups and every CRC;
2. exact main/string/handle exhaustion, derived versions `1/1/1`, zero reactors, absent xdictionary,
   owner code 8, handle length 8 and zero terminal fill on every frame;
3. 14 closed dimension/dependency/body triples, including recovered `2259` and `226a`, with body-owner,
   dependency-body, dependency-target, dimension-reactor and name/user-text reciprocity;
4. rejection of a non-type-542 owner, missing/reused body, divergent user text, wrong dependency target,
   residual bit, extra handle, nonzero fill or unsupported version before the snapshot is admitted;
5. a coordinated semantic name/user-text mutation and inverse. Mutating only one side must reject
   atomically; the inverse must restore all affected object frames, AcDbObjects/Handles and native bytes;
6. exact original fixture bytes after logical DSL/pack, diff/apply/inverse/absorb, mutation/inverse,
   analyzer/composer and native IO. No native name encoding, source frame, offset, selector, fill or CRC
   may survive in snapshot or facets.

Type 549 receives its 14-frame ledger credit only after all assertions above run green; this research
oracle alone does not change the accepted total.

## INSERT x12 live-readiness oracle after the entity-authority cut (read-only, 2026-08-14)

The existing bounded INSERT probe was rerun read-only and its stored frame CRCs were extracted from the
already validated fixture frames. No production file was edited and no Nx target was started. The cohort
is exactly twelve fixed type-7 entities, one signature each. The post-cut authority is the ordered
`DwgLogicalDrawing.objects` collection: there is one `DwgLogicalObject` per native INSERT self handle,
and its tagged entity body is the sole persisted geometric INSERT value. Any legacy drawing/entity list
is a derived view and must not participate in snapshot/diff/mutation/IO.

### Logical authority and body

```text
DwgInsertEntity {
    common: DwgEntityCommon,
    insertion: [f64; 3],
    scale: [f64; 3],
    rotation: f64,
    extrusion: [f64; 3],
    block_header_handle: Handle<DwgBlockHeaderTableRecord>,
    attribute_handles: Vec<Handle<DwgAttributeEntity>>,
    sequence_end_handle: Option<Handle<DwgSequenceEndEntity>>,
}
```

The outer `DwgLogicalObject` remains authoritative for self handle, optional explicit owner, reactors,
extension dictionary and EED. INSERT must not repeat those fields. `DwgEntityCommon` remains authoritative
for mode, color, linetype scale/reference, plot style, material, shadow, invisibility, lineweight, layer
and the corresponding optional style handles. The INSERT body adds only transform, block-header and
ordered attribute/SEQEND relationships. It must not duplicate the same relationships in
`referenced_handles`.

Attribute and SEQEND handles reference separate objects in the same ordered object collection; their
semantic bodies are never inlined into INSERT. The block header's standard INSERT-backreference
collection remains its own reverse relationship and must contain this INSERT exactly once. A mutation
updates the forward/reverse graph atomically. This preserves one authority per object while validating
the standard bidirectional graph.

For the fixture, all twelve are model-space mode, so there is no encoded common owner handle and
`DwgLogicalObject.owner_handle` is absent. Reactors are empty. Every INSERT has one distinct present
extension dictionary. Attributes are empty and SEQEND is absent throughout.

### Exact common entity stream

After fixed BOT type 7, self handle and zero-EED terminator, every frame has the same common semantics and
branches:

| concept | semantic value / native branch |
|---|---|
| graphic data | absent, `B=0` |
| entity mode / owner | ModelSpace, native mode 2; no owner handle |
| reactors | empty, count 0 via BL selector 2 |
| extension dictionary | present; one code-3 common handle |
| color | ByLayer/index 256 via BS selector 3; no alpha/RGB/name/book/reference branches |
| linetype scale | 1.0 via BD selector 1 |
| linetype / plot style / material | ByLayer native mode 0; no conditional handles |
| shadow | 0; no shadow handle |
| visual styles | full/face/edge absent |
| invisibility | 0 via BS selector 2 |
| lineweight | 29 |
| layer | non-null code-5 common handle |

The fixed BOT selector is 0 and self-handle code is 0 on all twelve. Common values above are semantic;
compact selectors and handle codes remain derived.

### Exact INSERT body stream and general symmetric branch

Fixture class-main order is:

1. insertion X/Y/Z as `3BD`, selectors `(0,0,2)`; Z is the standard zero;
2. scale selector `BB=3`, deriving `(1,1,1)` with no scale payload;
3. rotation `BD` selector 0/full RD;
4. extrusion `3BD`, selectors `(2,2,1)`, deriving `(0,0,1)`;
5. has-attributes `B=false`; attribute count is therefore physically absent;
6. false R2010 string-stream marker;
7. common extension dictionary code 3, common layer code 5, class block-header code 5;
8. five terminal one-fill bits, then little-endian CRC-16 seed `C0C1` over MS/UMC/payload.

The symmetric writer derives the scale selector from logical scale in this priority: exact `(1,1,1)`
uses selector 3; three bitwise-equal values use selector 2 plus X RD; exact X=1 uses selector 1 plus Y/Z
DD from 1; otherwise selector 0 plus X RD and Y/Z DD from X. It derives `has_attributes` and count from
the ordered handle collection. Valid relationship branches are:

- empty attributes and absent SEQEND -> `has_attributes=false`, no count, no child handles;
- nonempty attributes and one SEQEND -> `has_attributes=true`, derived count, each attribute code 4 in
  logical order, then SEQEND code 3;
- every other combination rejects atomically. A count, Boolean, compact handle code or encoded scale
  selector is never persisted.

All vectors must contain exactly three finite values; scale components must be nonzero; extrusion must be
a valid normal. Every attribute must be owned by this INSERT, every SEQEND must close exactly this
attribute sequence, and the target block header must list the INSERT in its backreferences.

### Fixture transform and handle oracle

All rows have Z=0, scale `(1,1,1)`, extrusion `(0,0,1)`, no attributes/SEQEND and the exact common fields
above.

| INSERT | insertion X / Y | rotation | xdictionary / layer / block header | CRC |
|---:|---:|---:|---|---:|
| `1f3d` | `313.14379801753614 / 212.5123802402951` | π | `1f3e / 83f / 1f57` | `6d84` |
| `1f8a` | `445.143798017536 / 380.5123802402951` | 3π/2 | `1f8b / 83f / 1fa4` | `0a54` |
| `1fdd` | `445.143798017536 / 530.5123802402951` | 3π/2 | `1fde / 83f / 1fa4` | `f278` |
| `1ff7` | `445.143798017536 / 674.5123802402951` | 3π/2 | `1ff8 / 83f / 1fa4` | `0560` |
| `2011` | `300.143798017536 / 475.4486348777122` | 3π/2 | `2012 / 843 / 201e` | `5af5` |
| `206a` | `450.14379801753614 / 475.4486348777122` | 3π/2 | `206b / 843 / 2077` | `60db` |
| `20a7` | `300.143798017536 / 625.4486348777122` | 3π/2 | `20a8 / 843 / 201e` | `e21c` |
| `20b4` | `450.14379801753614 / 625.4486348777122` | 3π/2 | `20b5 / 843 / 2077` | `b870` |
| `20c1` | `300.143798017536 / 769.4486348777123` | 3π/2 | `20c2 / 843 / 201e` | `8f16` |
| `20ce` | `450.14379801753614 / 769.4486348777123` | 3π/2 | `20cf / 843 / 2077` | `c009` |
| `20db` | `429.95384952627774 / 213.5123802402951` | π | `20dc / 843 / 201e` | `7676` |
| `20e8` | `643.4231528590517 / 213.5123802402951` | π | `20e9 / 843 / 201e` | `3121` |

Layer histogram is `83f x4, 843 x8`; block-header histogram is
`1f57 x1, 1fa4 x3, 201e x5, 2077 x3`. Every xdictionary is unique and immediately follows its INSERT
handle in this fixture, but adjacency is not logical state.

### One exact frame signature

All twelve frames have prefix 3 bytes, payload 45, total frame 50, handle-stream length 77 bits,
data-main end 283, class-main end 282, absent string stream, handle roles
`(extension_dictionary, layer, block_header)` with codes `(3,5,5)`, and terminal `11111`. The twelve
CRCs in the table are stored as little-endian frame RS values after the payload. Acceptance compares the
entire `MS + UMC + payload + CRC`, not only body coordinates or the checksum.

### Append-only entity and facet arms

INSERT is an entity-body append, not a new top-level object-body kind in Rust. Preserve existing
`DwgEntityBody` kinds 0 Line / 1 Arc / 2 LwPolyline and append
`Insert(DwgInsertEntity)` at **kind 3 / payload field 4**. Add `DwgInsertEntity` beside the other entity
records. The top-level Rust `DwgLogicalObjectBody::Entity` tag remains kind 4 / payload field 5.

The protobuf body facet historically flattened entity cases. Preserve every live assignment through
type 549—geometry dependency 10, block grip 11, dynamic proxy 12, associative variable 13 and
associative dimension dependency body 14—then append INSERT at body field **15**. Suggested semantic
message tags are:

```text
DwgInsertEntity:
  common=1, insertion=2, scale=3, rotation=4, extrusion=5,
  block_header_handle=6, attribute_handles=7, sequence_end_handle=8
DwgLogicalObjectBody: ... associative_dimension_dependency_body=14, insert=15
```

Use the same append-only field 15 in snapshot, artifact and diff protobuf mirrors. TypeScript appends
`{ kind: 'insert'; value: DwgInsertEntity }` to `DwgEntityBody`; JSON appends an `insertEntity` definition
and entity-union arm. GraphQL defines `DwgInsertEntity` and appends it to the currently flattened
`DwgLogicalObjectBody` union. Text and binary facets append the same semantic fields/arm. None may expose
`has_attributes`, count, scale selector, owner mode bits, string marker, compact handle codes, frame
lengths, fill or CRC.

### Strict lifecycle assertions

Extend the existing fixture test only and require:

1. 12 attempted, decoded and re-encoded INSERT frames, the single signature, all full frames and all
   twelve CRCs above;
2. exact common/main/string/handle exhaustion, model-space implicit owner, zero reactors, distinct present
   xdictionary, no attributes/SEQEND and five one-fill bits for every fixture frame;
3. unique object authority: exactly one handle-keyed INSERT body, no persisted mirror entity, no duplicated
   xdictionary/block/attribute/SEQEND handles in `referenced_handles`, and derived geometry views unable to
   influence serialization;
4. reciprocal block-header backreferences, xdictionary ownership, layer resolution, and—on a synthetic
   typed branch—ordered attribute ownership plus exactly one closing SEQEND;
5. insertion/rotation/block-header mutations and inverse, plus a coordinated attribute+SEQEND mutation.
   Missing child, duplicate child, mismatched owner, invalid transform or one-sided block backreference
   rejects atomically; inverse restores object order, AcDbObjects/Handles and original native bytes;
6. original fixture equality through logical DSL/pack, diff/apply/inverse/absorb, mutation/inverse,
   analyzer/composer and native IO, with anti-shadow coverage across Rust and every facet.

INSERT receives its 12-frame ledger credit only after these runtime assertions are green; this report
does not pre-credit it.

## Historical remaining-ledger reconciliation at 523/663 (read-only, 2026-08-14)

> **Superseded runtime baseline.** DIMENSION_LINEAR x14 subsequently raised acceptance to 537/663;
> VISUALSTYLE x19 then raised it to the current 556/663. The tables in this section preserve the planning
> arithmetic before those two gates and must not be used as the live numerator. The current ledger and
> post-queue 89-frame inventory are recorded below the type-543 oracle.

The implementation lane now reports exact same-class runtime gates for the six cohorts that followed
the 433-frame dependency-core baseline: type 520 x23, type 547 x1, type 545 x20, type 549 x14,
BLOCK/ENDBLK x20 and INSERT x12. Those are exactly 90 additional frames, so the strict evidence ledger
is **523/663 accepted, 140 remaining** (78.88% / 21.12%). This read-only lane did not rerun Nx and does
not credit the queued DIMENSION_LINEAR or VISUALSTYLE work before its exact-frame gates report green.

### Reconciled family totals

| family | fixture | accepted | remaining | exact-ready frontier |
|---|---:|---:|---:|---|
| fixed entities | 82 | 68 | 14 | DIMENSION_LINEAR x14 queued |
| dictionary/XRECORD spine | 237 | 229 | 8 | DICTIONARYVAR x8 remains matrix-ready |
| block/entity graph | 32 | 32 | 0 | BLOCK/ENDBLK/INSERT complete |
| table controls/records | 59 | 59 | 0 | complete |
| fixed support | 6 | 0 | 6 | VIEWPORT x2 exact-ready; LAYOUT x2, MLINESTYLE x1 and PLACEHOLDER x1 matrix-ready |
| style/context custom | 50 | 0 | 50 | VISUALSTYLE x19 queued; TABLESTYLE x1 and EVALUATION_GRAPH x2 exact-ready |
| dynamic-block custom | 71 | 24 | 47 | type 559 x12, 529 x3, 522 x2 and 531 x1 exact-ready |
| associative custom | 126 | 111 | 15 | type 543 x6 and 540 x4 exact-ready; type 539 x5 matrix-ready |
| **total** | **663** | **523** | **140** | **66 exact-ready, 74 matrix-ready** |

The accepted numerator is independently reproducible as 145 XRECORD + 84 DICTIONARY/WDFLT + 9 table
controls + 50 table records + 40 LINE + 12 ARC + 16 LWPOLYLINE + 20 type 542 dependencies + 26 type
541 value dependencies + 31 type 544 geometry dependencies + 23 type 520 grip-location components +
1 type 547 proxy node + 20 type 545 variables + 14 type 549 dimension bodies + 20 BLOCK/ENDBLK +
12 INSERT = 523.

### Recovered-count invariants

The block-local Handles reset added exactly eleven valid frames and no extra cohort:

| cohort | old bound | recovered | authoritative bound | acceptance status |
|---|---:|---:|---:|---|
| type 21 DIMENSION_LINEAR | 12 | 2 | 14 | queued, not credited |
| type 542 ACDBASSOCDEPENDENCY | 18 | 2 | 20 | exact green and credited |
| type 541 ACDBASSOCVALUEDEPENDENCY | 23 | 3 | 26 | exact green and credited |
| type 545 ACDBASSOCVARIABLE | 18 | 2 | 20 | exact green and credited |
| type 549 ASSOCDIMDEPENDENCYBODY | 12 | 2 | 14 | exact green and credited |
| **total** | **83** | **11** | **94** | 80 credited; DIMENSION_LINEAR 14 remains queued |

All fixture tests, decoder attempt bounds, same-class exact-frame assertions and facet inventories must use
these authoritative counts. In particular, no stale `652`, DIMENSION_LINEAR `12`, type-542 `18`,
type-541 `23`, type-545 `18` or type-549 `12` gate remains valid. Type 547 remains one frame; it must not
be confused with the adjacent type-545/type-549 facet tags.

### Fastest exact-ready next 50

The implementation lane already queued DIMENSION_LINEAR x14 and VISUALSTYLE x19. Together they are 33
frames and would move the ledger to **556/663**, leaving 107. The fastest low-risk continuation is to
reuse two bodies whose required cores are now accepted:

| order | atomic cohort | frames | cumulative new | implementation reuse |
|---:|---|---:|---:|---|
| 1 | type 21 DIMENSION_LINEAR | 14 | 14 | accepted common entity/frame writer; corrected two recovered signatures already enumerated |
| 2 | type 506 VISUALSTYLE | 19 | 33 | one fixed 28-property/modifier layout; no class-local handles |
| 3 | type 543 BLOCKPARAMDEPENDENCYBODY | 6 | 39 | reuse accepted type-549 dependency-body/version/string framing; no class handles |
| 4 | type 559 ACDB_BLOCKREPRESENTATION_DATA | 12 | **51** | thin version/flag plus represented-block reference; block graph is now accepted |

Exact-frame credit is cohort-atomic, so the fastest safe implementation wave is **51**, not an artificial
five-of-twelve split that would claim 50. When all four gates are green the ledger becomes **574/663
accepted, 89 remaining**. A literal 50-frame composition exists only by substituting several smaller but
more complex cohorts (for example type 559 x12 + VIEWPORT x2 + type 522 x2 + TABLESTYLE x1 after the
queued 33); it is slower and raises four schema/writer surfaces instead of reusing the accepted dependency
and block cores.

After this 51-frame wave, only 15 further exact-ready frames remain: type 540 x4, type 529 x3, VIEWPORT
x2, type 522 x2, EVALUATION_GRAPH x2, TABLESTYLE x1 and type 531 x1. The other 74 are still matrix-ready,
not exact-ready. Therefore a request for an additional 50 *after* DIMENSION_LINEAR/VISUALSTYLE cannot be
satisfied honestly from current R0 evidence: complete the 33 residual R0 frames, then promote at least
17 R1 frames through bounded all-frame probes. SCALE x17 is the only single-cohort count match, but it
must not receive credit until its semantic matrix, exact signatures and symmetric writer gate are live.

Whole-file acceptance remains independent of the frame numerator: all 663 typed objects, derived
AcDbObjects/Handles, named sections, D2 pages and the complete original-byte lifecycle must still pass.

## Type 543 BLOCKPARAMDEPENDENCYBODY live-readiness oracle (read-only, 2026-08-14)

The existing bounded fixture probe was rerun read-only. It finds exactly **six** CRC-valid type-543
frames. None belongs to the eleven-frame block-local Handles recovery, so six is both the old and the
authoritative count. This report does not give the cohort runtime credit and no Nx target was started.

The standard inheritance and native field order are confirmed by LibreDWG's
[`BLOCKPARAMDEPENDENCYBODY` prescription](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L5692-L5702):
`AcDbAssocDependencyBody`, `AcDbImpAssocDimDependencyBodyBase`, then
`AcDbBlockParameterDependencyBody`. The logical model must not persist those implementation class
versions:

```text
DwgBlockParameterDependencyBody {
    name: String,
}
```

Native `adb_version=1`, `dimbase_version=1` and `class_version=0` are deterministic AC1024 writer
constants. The body does **not** contain or inherit a semantic `DwgAssociativeDependency`, associative
action or `DwgEvaluationExpression` value. Those are separate handle-keyed graph objects. Its sole
semantic link is the outer object's owner handle, which resolves to the type-542 dependency whose
`dependency_body_handle` points back to this body.

### Exact graph and semantic names

All six owners are the immediately preceding type-542 object, and every body is the sole body of that
dependency. The dependencies form two three-node parameter chains:

| body | owner dependency | owner action/network | dependent-on parameter | previous dependency | next dependency | body ID | semantic name |
|---|---|---|---|---|---|---:|---|
| `114c` | `114b` | `1149` | `1160` | null | `2028` | 3 | `H=4.0000` |
| `2029` | `2028` | `2026` | `1160` | `114b` | `2081` | 3 | `H=6"` |
| `2082` | `2081` | `207f` | `1160` | `2028` | null | 3 | `H=6"` |
| `1152` | `1151` | `1149` | `1165` | null | `202e` | 7 | `W=36.0000` |
| `202f` | `202e` | `2026` | `1165` | `1151` | `2087` | 7 | `W=5'-0"` |
| `2088` | `2087` | `207f` | `1165` | `202e` | null | 7 | `W=3'-6"` |

The previous/next columns are the resolved type-542 dependency-link handles. They remain fields of the
dependency object and must not be copied into the body or a generic `referenced_handles` mirror. Import
must validate the reciprocal body edge, matching dependency body ID across each chain and one consistent
dependent-on parameter per chain. It must not infer that equal parameter handles require equal display
names: the fixture intentionally carries different unit/value spellings along each chain.

### Main, string and handle streams

Every frame uses a three-byte MS prefix, BOT selector 1, self-handle code 0, empty EED, zero reactors via
BL selector 2 and `xdic_missing=true`. Exact logical/native order after common object state is:

1. `adb_version` BS selector 1, value 1;
2. `dimbase_version` BS selector 1, value 1;
3. one TU `name` in the R2010 string stream;
4. `class_version` BS selector 2, value 0 in the main stream;
5. string-stream framing derived from the encoded TU length;
6. sole owner handle, native role code 8 and zero payload because every owner is `self-1`;
7. CRC16 over the complete prefix and payload.

Main data ends exactly at bit 61 for all six frames. The string reader and main reader meet at the same
boundary, the handle stream is exactly eight bits, and **zero terminal fill bits** remain. There is no
extension-dictionary handle, reactor handle, evaluation handle, parameter handle or class-local handle.

| body handles | payload / total bytes | data / class / string bits | name branch | CRC |
|---|---|---|---|---|
| `2029 2082` | 20 / 25 | 152 / 61 / 74 | `H=6"` | `39e3 018c` |
| `202f 2088` | 26 / 31 | 200 / 61 / 122 | `W=5'-0"`, `W=3'-6"` | `f97a 4a5b` |
| `114c` | 28 / 33 | 216 / 61 / 138 | `H=4.0000` | `0bae` |
| `1152` | 30 / 35 | 232 / 61 / 154 | `W=36.0000` | `d43d` |

The encoder accepts only a type-543/class-name/body match and a resolved reciprocal type-542 owner.
It derives every selector, version, string marker/size, relative handle nibble, length and CRC. Any EED,
reactor, extension dictionary, extra handle, trailing bit, dangling owner, duplicate body or mismatched
dependency-body edge rejects atomically rather than falling through to an unsupported body.

### Append-only schema and facet placement

The live Rust top-level `DwgLogicalObjectBody` currently ends at associative-dimension dependency body
kind 11/payload field 12. DIMENSION_LINEAR is nested in `DwgEntityBody` and does not consume a top-level
Rust tag. VISUALSTYLE is queued ahead of this cohort, so reserve VisualStyle kind 12/field 13 and append
`BlockParameterDependencyBody` at **kind 13/payload field 14**. If merge order changes, inspect the live
maximum and append after it; never renumber or reuse an occupied field.

The live flattened protobuf body ends at INSERT field 17. The queued order reserves DIMENSION_LINEAR 18
and VISUALSTYLE 19, then appends `DwgBlockParameterDependencyBody block_parameter_dependency_body = 20`.
Its message contains only `string name = 1`. Snapshot, artifact and diff protobuf mirrors must use the
same append-only assignment; no native version or owner handle is duplicated inside the body message.

Append the same semantic arm to TypeScript, JSON Schema and the GraphQL body union. Extend the structured
text EBNF/G4/Semio grammar and structured binary Kaitai/ABNF/Spicy/Semio protocol with exactly `name` and
the tagged arm. Artifact/diff/mutation routes continue to carry the logical snapshot/body structurally;
they must not add a native-frame, class-version, stream, fill, CRC or JSON/native envelope.

### Exact acceptance gates

Extend the existing AC1024 fixture tests and require:

1. type census attempted/decoded/encoded `6/6/6`, four exact signatures, six exact complete frames and
   CRC set `114c:0bae 1152:d43d 2029:39e3 202f:f97a 2082:018c 2088:4a5b`;
2. exact bit-61 main exhaustion, exact TU string exhaustion, one code-8 owner and zero trailing bits for
   every frame;
3. the two three-node dependency chains above, reciprocal body ownership, body IDs 3/7 and exact names;
4. name mutation plus inverse, dependency/body coordinated mutation plus inverse, and atomic rejection
   of a one-sided edge, cross-chain owner, duplicate body, wrong type/class or invalid empty name;
5. structural snapshot DSL/pack, diff/apply/inverse/absorb, mutation/inverse, analyzer/composer and native
   IO preserve the original fixture baseline without persisted native syntax or container state;
6. Rust and every facet anti-shadow scans include this arm and reject source/physical/lexical/native/
   frame/stream/fill/CRC/class-version fields.

At the type-543 oracle handoff, DIMENSION_LINEAR had raised runtime credit from 523 to **537/663**, so
type 543 alone would have moved that contemporary baseline to **543/663**. VISUALSTYLE subsequently
raised the live baseline to **556/663**; if type 543 lands against that baseline, it moves to **562/663**.

## Current 556/663 ledger and post-queue remaining 89 (read-only, 2026-08-14)

Runtime evidence now credits DIMENSION_LINEAR x14 and VISUALSTYLE x19 in addition to the former 523.
The current strict ledger is therefore **556/663 accepted, 107 remaining** (83.86% / 16.14%). Type 543
x6 and type 559 x12 are queued but remain uncredited until their own exact same-class gates pass. If both
pass, the ledger becomes **574/663 accepted, 89 remaining** (86.58% / 13.42%). This read-only lane did
not run Nx.

### Current family reconciliation

| family | fixture | accepted now | remaining now | after queued 543/559 |
|---|---:|---:|---:|---:|
| fixed entities | 82 | 82 | 0 | 0 |
| dictionary/XRECORD spine | 237 | 229 | 8 | 8 |
| block/entity graph | 32 | 32 | 0 | 0 |
| table controls/records | 59 | 59 | 0 | 0 |
| fixed support | 6 | 0 | 6 | 6 |
| style/context custom | 50 | 19 | 31 | 31 |
| dynamic-block custom | 71 | 24 | 47 | 35 after type 559 x12 |
| associative custom | 126 | 111 | 15 | 9 after type 543 x6 |
| **total** | **663** | **556** | **107** | **89** |

The accepted 556 are exactly 145 XRECORD + 84 DICTIONARY/WDFLT + 9 table controls + 50 table records +
40 LINE + 12 ARC + 16 LWPOLYLINE + 14 DIMENSION_LINEAR + 20 type 542 + 26 type 541 + 31 type 544 +
23 type 520 + 1 type 547 + 20 type 545 + 14 type 549 + 20 BLOCK/ENDBLK + 12 INSERT + 19 VISUALSTYLE.
No research-only or queued cohort is pre-credited.

The eleven frames recovered by block-local Handles reset are all already represented in that numerator:
DIMENSION_LINEAR +2 gives 14; type 542 +2 gives 20; type 541 +3 gives 26; type 545 +2 gives 20; and
type 549 +2 gives 14. Type 543 remains six and type 559 remains twelve; neither recovered an extra frame.
There are therefore no hidden recovered frames in the post-queue 89.

### Complete post-queue 89-frame inventory

R0 means an exact bounded all-frame prescription, signatures/CRCs and symmetric writer order already
exist in this ticket. R1 means the standard semantic matrix exists but an all-frame bounded fixture probe
and exact writer gate must still be completed. The 89 are exactly **15 R0 + 74 R1**.

#### R0 exact-ready — 15

| recommended order | type/class | count | dependency and implementation note |
|---:|---|---:|---|
| 1 | 522 `ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION` | 2 | thinnest body: derived version plus one protected-block reference; block graph is accepted |
| 2 | 34 `VIEWPORT` | 2 | complete standard-entity oracle; reuse accepted entity common/frame primitives |
| 3 | 517 `ACAD_EVALUATION_GRAPH` | 2 | typed nodes/edges; derive intrusive indexes before dependent dynamic-parameter validation |
| 4 | 529 `BLOCKFLIPPARAMETER` | 3 | typed flip node/expression; validate membership against the now-typed evaluation graph |
| 5 | 504 `TABLESTYLE` | 1 | complete R24 table/cell-style semantic prescription; independent of dynamic graph |
| 6 | 531 `BLOCKVISIBILITYPARAMETER` | 1 | typed states and stateless evaluation-history policy; reuse evaluation/parameter cores |
| 7 | 540 `ACDBASSOC2DCONSTRAINTGROUP` | 4 | exact constraint-node union but highest complexity; keep last among R0 |
| **R0 total** |  | **15** |  |

This order minimizes new primitives: type 522 and VIEWPORT are independent; EVALUATION_GRAPH then
unlocks graph validation for type 529/531; TABLESTYLE is isolated; type 540 remains last because its
four frames contain the broadest typed constraint-node union. Each cohort remains atomic for credit.

#### R1 matrix-ready — 74

| promotion order | type/class | count | bounded-probe requirement before implementation credit |
|---:|---|---:|---|
| 1 | `ACDBPLACEHOLDER` | 1 | common-object-only frame; prove exact empty class body and handles |
| 2 | 503 `DICTIONARYVAR` | 8 | typed schema/default/value record; exhaust main/string/owner streams |
| 3 | 507 `SCALE` | 17 | name, paper/drawing units and scale flag; derive defaults and exact string branches |
| 4 | 516 `SORTENTSTABLE` | 7 | ordered entity/sort-handle pairs; derive count and preserve pairing |
| 5 | `MLINESTYLE` | 1 | standard line-style collection; derive element count and line properties |
| 6 | 508 `MLEADERSTYLE` | 1 | complete named style record and referenced styles/blocks |
| 7 | 539 `ACDBASSOCNETWORK` | 5 | reuse accepted associative action/dependency graph and validate membership order |
| 8 | 505 `MATERIAL` | 3 | typed material channels/maps and handles; no packed property record |
| 9 | 521 `BLOCKMOVEACTION` | 2 | reuse accepted evaluation/action cores; prove connection-point branches |
| 10 | 527 `BLOCKLINEARPARAMETER` | 2 | typed two-point parameter and value set; prove strings/handles |
| 11 | 528 `BLOCKLINEARGRIP` | 4 | reuse accepted grip/EvalExpr core; prove per-frame grip branches |
| 12 | 530 `BLOCKFLIPGRIP` | 3 | reuse grip core and typed flip state |
| 13 | 532 `BLOCKVISIBILITYGRIP` | 1 | reuse grip core and visibility parameter relation |
| 14 | 533 `BLOCKALIGNMENTPARAMETER` | 2 | typed alignment parameter/value-set branches |
| 15 | 534 `BLOCKALIGNMENTGRIP` | 2 | reuse grip/alignment primitives |
| 16 | 535 `BLOCKSTRETCHACTION` | 6 | typed connection points, stretch polygon and ordered handle entries |
| 17 | 536 `BLOCKSCALEACTION` | 1 | action/connection/dependency core exact frame |
| 18 | 537 `BLOCKFLIPACTION` | 3 | action core plus typed flip connection/state |
| 19 | 538 `BLOCKBASEPOINTPARAMETER` | 1 | parameter core plus base-point fields |
| 20 | 546 `BLOCKVERTICALCONSTRAINTPARAMETER` | 1 | constraint-parameter core; validate vertical semantic role |
| 21 | 548 `BLOCKHORIZONTALCONSTRAINTPARAMETER` | 1 | constraint-parameter core; validate horizontal semantic role |
| 22 | `LAYOUT` | 2 | complete page/layout/UCS/extents/viewport graph; broadest support record, keep last |
| **R1 total** |  | **74** |  |

The promotion order is dependency-aware rather than count-only. The first four R1 cohorts add 33 frames
with small, isolated schemas; after all 15 R0 are green, they raise the post-queue ledger by 48 to
**622/663**, leaving 41. Adding MLINESTYLE and MLEADERSTYLE yields a clean next-50 wave at **624/663**.
Those two singleton cohorts are not currently R0: their bounded signatures must be established before
credit. The remaining R1 dynamic classes should be promoted base-before-derived in the order above, and
LAYOUT last because it closes the widest object graph.

### Fastest implementation sequence from the live baseline

1. Finish queued type 543 x6 and type 559 x12; exact gates move 556 to 574 and leave the 89 inventoried
   above.
2. Implement the seven R0 rows in order, moving 574 to 589 and leaving 74.
3. Promote and implement PLACEHOLDER, DICTIONARYVAR, SCALE and SORTENTSTABLE from R1, moving 589 to 622.
4. Probe and implement MLINESTYLE and MLEADERSTYLE for an exact cohort-atomic next-50 after the queued
   wave, moving 622 to 624.
5. Continue the remaining R1 rows base-before-derived; never credit a matrix without full stream
   exhaustion, exact frame bytes/CRC and structured lifecycle coverage.

Frame totals do not supersede the native acceptance contract. The derived AcDbObjects/Handles sections,
named sections, D2 pages and full original-byte lifecycle remain red until all 663 logical objects are
typed and the deterministic writer reproduces the fixture exactly.

## Type 522 ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION exact-ready oracle (read-only, 2026-08-14)

The bounded thin-body fixture probe was rerun read-only and validates exactly **two** type-522 frames.
Neither was added by the block-local Handles recovery, so two is the authoritative old and current count.
Both complete frames pass CRC16. No Nx target was started and this report does not award runtime credit.

### Minimal logical authority

The class identity already expresses that this object prevents a dynamic block from being purged. The
only body value that can vary semantically in AC1024 is the protected block-header reference:

```text
DwgDynamicBlockPurgePreventer {
    protected_block_header_handle: Handle<DwgBlockHeaderTableRecord>,
}
```

The native BS value 1 is the AC1024 class/version declaration and is derived by the writer. It is not a
logical status, user-editable flag or persisted version field. The outer `DwgLogicalObject` remains the
sole authority for self handle, owner, reactors, extension dictionary and EED. The body owns only the
protected-block relation; it must not duplicate that relation in generic `referenced_handles`.

Exact fixture graph:

| object | owner dictionary | sole reactor | protected BLOCK_HEADER | native version |
|---|---|---|---|---:|
| `1137` | `110e` | `110e` | `110d` | 1 |
| `116a` | `1146` | `1146` | `1145` | 1 |

Handles `110e` and `1146` are standard type-42 DICTIONARY objects. Handles `110d` and `1145` are standard
type-49 BLOCK_HEADER table records. Owner and sole reactor intentionally resolve to the same dictionary
in each fixture graph; import must preserve the two distinct semantic roles rather than deduplicating the
reactor vector. The protected target must resolve to a BLOCK_HEADER, not merely any existing handle.

### Exact main, string and handle streams

Both frames have the identical structural signature:

| property | exact fixture value |
|---|---|
| prefix / payload / complete frame | 3 / 16 / 21 bytes |
| handle stream / data end / class-main end | 70 / 58 / 57 bits |
| BOT / self handle | selector 1 / code 0 |
| EED | empty |
| reactor count | one, BL selector 1 |
| extension dictionary | missing |
| class value | BS selector 1, value 1 |
| string stream | absent; one derived zero marker at bit 57 |
| handle roles and native codes | owner 12, reactor 4, protected block 5 |
| terminal fill | six one-bits `111111` |

The writer sequence is BOT, self handle, empty EED, common object main, derived BS1 class version, absent
string marker, then common handles `(owner, reactor)`, protected-block handle, six one-fill bits and CRC.
The handle stream begins at bit 58 and is exactly 70 bits including fill. It must exhaust at the payload
boundary; there is no class-local string, evaluation handle, extension-dictionary handle or second block
reference.

Exact frame oracles:

| object | resolved handle values | CRC16 |
|---|---|---|
| `1137` | owner/reactor `110e`, protected block `110d` | `c2bc` |
| `116a` | owner/reactor `1146`, protected block `1145` | `583d` |

The encoder rejects atomically if the body/type/class disagree; the protected handle is null, duplicated
or not a BLOCK_HEADER; owner/reactor roles do not resolve to the expected dictionary relationship; an
extension dictionary/EED/extra handle is introduced; or main, string, handle and fill consumption is not
exact. It derives prefix lengths, selectors, handle nibbles, fill and CRC from logical state.

### Append-only schema and facet tags

The live Rust top-level object-body frontier is VisualStyle kind 12/field 13 followed by
BlockParameterDependencyBody kind 13/field 14. Type 559 is queued next and reserves kind 14/field 15.
Append `DynamicBlockPurgePreventer` after it at **kind 15/payload field 16**. Its record has one field:
`protected_block_header_handle` at field 0 in the Rust structured DSL record. If the queued landing order
changes, append after the actual live maximum and update every mirror together; never renumber an existing
kind or reuse an occupied field.

The live flattened protobuf frontier is VisualStyle field 19. Reserve type 543 at 20 and type 559 at 21,
then append `DwgDynamicBlockPurgePreventer dynamic_block_purge_preventer = 22`. Its message contains only
`uint64 protected_block_header_handle = 1`. Snapshot, artifact and diff protobuf mirrors must preserve
the same assignment.

Append the arm and record to TypeScript, JSON Schema and the GraphQL body union. Extend EBNF/G4/Semio
structured text and Kaitai/ABNF/Spicy/Semio structured binary facets with only the protected-block field.
Artifact/diff/mutation codecs carry this logical body structurally. No facet may expose `version`, native
flag/selector, owner/reactor mirrors, type/frame bytes, stream offsets, handle codes, terminal fill or CRC.

### Runtime acceptance gates

Extend the existing AC1024 fixture tests and require:

1. attempted/decoded/encoded `2/2/2`, one exact signature, both complete frames and exact CRCs
   `1137:c2bc 116a:583d`;
2. class-main end bit 57, absent string marker, data end bit 58, role codes `(12,4,5)`, exactly six one-fill
   bits and complete stream exhaustion for both frames;
3. dictionary owner/reactor identity and protected BLOCK_HEADER type resolution without a duplicated
   generic reference;
4. protected-block mutation plus inverse, coordinated owner/reactor mutation plus inverse, and atomic
   rejection of missing/wrong-class targets or one-sided graph edits;
5. logical DSL/pack, diff/apply/inverse/absorb, mutation/inverse, analyzer/composer and native IO retain
   the original fixture baseline; Rust and every facet anti-shadow gate includes the new arm.

Against the planned post-type-543/type-559 ledger of 574/663, a green type-522 gate moves acceptance to
**576/663** and leaves 87. It receives no credit from this read-only oracle alone.

## Type 517 ACAD_EVALUATION_GRAPH x2 exact-ready oracle (read-only, 2026-08-14)

The existing zero-gate fixture probe was rerun read-only. It validates exactly **two** type-517 frames,
both CRC-valid and neither added by the block-local Handles recovery. No Nx target was started and this
report awards no runtime credit.

AutoCAD defines `AcDbEvalGraph` as the persistent directed acyclic graph that hard-owns its expression
nodes, assigns unique node IDs and maintains edge reference counts. Edges identify their endpoint nodes
and activation metadata ([Autodesk graph overview](https://help.autodesk.com/cloudhelp/2025/ESP/OARXMAC-DevGuide/files/GUID-B520F4A9-20D0-420D-B4A3-FBEC835D0E42.htm),
[node/edge contract](https://help.autodesk.com/cloudhelp/2022/ENU/OARX-DevGuide/files/GUID-74E0EB04-41BF-4E56-85A8-2318B25C9BBE.htm)).
The AC1024 physical order is corroborated by LibreDWG's
[`EVALUATION_GRAPH` declaration](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3361-L3405).

### Logical schema and live evaluation authority

```text
DwgEvaluationGraphNode {
    id: u32,
    expression_handle: Handle<DwgEvaluationExpressionOwner>,
}

DwgEvaluationGraphEdge {
    from_node_id: u32,
    to_node_id: u32,
    reference_count: u32,
    invertible: bool,
    suppressed: bool,
}

DwgEvaluationGraph {
    nodes: Vec<DwgEvaluationGraphNode>,
    edges: Vec<DwgEvaluationGraphEdge>,
}
```

Nodes and edges are ordered semantic graph collections; node IDs are the standard stable
`AcDbEvalNodeId` values, and endpoints use those IDs. The graph does not inline an evaluation expression
or `DwgEvaluationVariant`. The referenced handle-keyed object remains the sole authority for its live
`DwgEvaluationExpression`/variant payload and must have outer owner equal to the graph handle. Its
expression `node_id` must equal the graph node ID.

Do not persist the native header copy, node/edge counts, storage ordinals, adjacency indexes, edge list
links, inverse indexes, constant marker 32, active-cycle presence bit or handle codes. They are indexes
and materialization state. The fixture graphs are acyclic, all edges are directed, non-invertible and
unsuppressed, so no activated-cycle or inverse-edge concept is present.

### Exact graph contents

| graph | owner and sole reactor | nodes / edges | derived node-ID watermark | expression handles | CRC |
|---|---|---:|---:|---|---|
| `110f` | DICTIONARY `110e` / `110e` | 38 / 43 | 79 | contiguous `1110..1135` | `03bb` |
| `1155` | DICTIONARY `1146` / `1146` | 19 / 14 | 230 | contiguous `1156..1168` | `4919` |

Ordered node IDs are:

```text
110f: 1 2 3 4 8 9 10 11 26 27 28 29 30 31 46 47 48 49 50 51 52 53 54 55 56 57
      65 66 67 68 69 70 72 74 75 76 77 79
1155: 120 128 136 144 152 184 192 200 208 216 222 223 224 225 226 227 228 229 230
```

The first native header value and its duplicate are 79/79 and 230/230. They equal the final/highest
assigned node ID, **not** the next allocatable `max+1`; the earlier report wording is corrected above.

Expression target type histograms prove integration with the live evaluation core:

| graph | target type histogram |
|---|---|
| `110f` | 520 x14, 535 x6, 527 x2, 528 x2, 529 x2, 530 x2, 521 x2, 537 x2, and 531/532/533/534/536/538 x1 each |
| `1155` | 520 x9, 528 x2, and 529/530/537/533/534/546/547/548 x1 each |

The exact ordered semantic edges are `(from,to,reference_count)`:

```text
110f:
(1,3,1) (1,4,1) (2,1,2) (8,10,1) (8,11,1) (9,8,2)
(26,29,1) (26,30,1) (26,28,1) (27,26,1)
(47,46,1) (46,48,1) (46,49,1)
(31,52,1) (31,53,1) (31,51,1) (50,31,1)
(55,54,1) (54,56,1) (54,57,1)
(1,65,2) (1,66,2) (1,67,2) (1,68,2) (1,69,2) (1,70,5)
(8,72,2) (1,74,2) (8,75,2) (26,76,4) (31,77,4)
(76,79,1) (76,65,1) (76,66,1) (76,67,1) (76,68,1) (76,69,1)
(77,79,1) (77,65,1) (77,66,1) (77,67,1) (77,68,1) (77,69,1)

1155:
(120,144,1) (120,152,1) (120,136,1) (128,120,1) (120,184,4)
(200,192,1) (192,208,1) (192,216,1)
(222,224,1) (222,225,1) (223,222,2)
(227,229,1) (227,230,1) (228,227,2)
```

Reference-count histograms are `1x30,2x10,4x2,5x1` for `110f` and `1x11,2x2,4x1` for `1155`.
A Kahn traversal consumes 38/38 and 19/19 nodes, proving both are closed DAGs. Every native inverse-edge
index is `-1`, consistent with the fixture's non-invertible edge state.

### Derived native index policy

The writer deterministically maps the ordered logical graph to AC1024 as follows:

1. validate unique nonzero node IDs, one expression object per node, matching expression `node_id`, graph
   ownership, unique edge identities, positive reference counts, closed endpoints and acyclicity;
2. assign native node storage ordinal from the ordered node vector and edge storage ordinal from the
   ordered edge vector;
3. derive header watermark and duplicate from the final/highest node ID and derive both counts;
4. emit for each node: storage ordinal `BL`, constant marker 32 `BL`, semantic node ID `BLd`, then
   `[first incoming, last incoming, first outgoing, last outgoing]` edge ordinals derived by scanning the
   ordered edge vector; use `-1` for empty adjacency;
5. emit for each edge: storage ordinal `BL`, derived reserved/next value 0 `BLd`, reference count, source
   node ordinal, target node ordinal, then derived
   `[previous incoming, next incoming, previous outgoing, next outgoing, inverse edge]`; fixture inverse
   is `-1` throughout;
6. omit active-cycle state for the validated DAG, then write the common handles and expression handles in
   node order, derive fill and CRC.

Mutations may reorder the semantic vectors only as an explicit logical change. The native ordinal/index
web is rebuilt from the resulting order and never copied into snapshot/diff/mutation state. A cyclic
mutation rejects atomically; future standard activated-cycle support requires a typed semantic concept,
not retention of the native presence bit.

### Exact streams, roles, fill and CRC

Both use a four-byte MS prefix, BOT selector 1, self-handle code 0, empty EED, one reactor via BL selector
1, missing extension dictionary and no string stream. Main ends one bit before the handle boundary; that
last main bit is the derived absent-string marker.

| graph | payload / total bytes | handle / data / class bits | common handle roles | node handles | terminal fill |
|---|---|---|---|---|---|
| `110f` | 1525 / 1531 | 944 / 11256 / 11255 | owner code 8, reactor code 4, both `110e` | 38 code-3 expression handles | none |
| `1155` | 660 / 666 | 502 / 4778 / 4777 | owner code 12, reactor code 4, both `1146` | 19 code-3 expression handles | `111111` |

The logical expression relation is hard ownership even though the fixture's target-relative compact
handle encoding is native code 3 rather than LibreDWG's declarative absolute role code 5. The writer
derives the compact form from role/target and persists neither code. It must preserve owner and the
same-valued reactor as distinct outer-object roles, exactly exhaust main/absent-string/handle/fill bits,
then reproduce CRCs `110f:03bb 1155:4919`.

### Append-only schema and facet tags

Following the planned VisualStyle, type-543, type-559 and type-522 arms, append `EvaluationGraph` to the
Rust top-level body at **kind 16/payload field 17**. Suggested structured record fields are graph
`nodes=0, edges=1`; node `id=0, expression_handle=1`; edge
`from_node_id=0, to_node_id=1, reference_count=2, invertible=3, suppressed=4`.

The corresponding flattened protobuf body arm is
`DwgEvaluationGraph evaluation_graph = 23`, after VisualStyle 19, type 543 at 20, type 559 at 21 and
type 522 at 22. Suggested protobuf messages use graph `nodes=1, edges=2`; node `id=1,
expression_handle=2`; edge `from_node_id=1, to_node_id=2, reference_count=3, invertible=4,
suppressed=5`. Inspect the live maximum at landing time and append after it if concurrent order changes;
never renumber an occupied tag.

Append the same semantic records/arm to TypeScript, JSON Schema and GraphQL, and extend EBNF/G4/Semio
text plus Kaitai/ABNF/Spicy/Semio binary facets. Artifact/diff/mutation codecs carry nodes and edges
structurally. Anti-shadow gates must reject native header copies, counts, ordinals, adjacency/link/inverse
indexes, markers, stream state, handle codes, fill, CRC and native/JSON envelopes.

### Runtime acceptance gates

Extend the existing AC1024 fixture tests and require:

1. attempted/decoded/encoded `2/2/2`, both complete signatures and CRCs `110f:03bb 1155:4919`;
2. exact ordered IDs, expression handles/type histograms, all 57 semantic edges/reference counts, closed
   ownership and DAG validation;
3. derived native adjacency/link indexes exactly match every fixture slot, with no persisted index field;
4. exact common/main/absent-string/handle/fill exhaustion and expression handle order for both frames;
5. node/edge/reference-count mutation plus inverse; atomic rejection of dangling endpoint, duplicate ID,
   owner/node-ID mismatch, zero reference count, illegal inverse/suppression state or cycle;
6. original fixture equality through logical DSL/pack, diff/apply/inverse/absorb, mutation/inverse,
   analyzer/composer and native IO, plus Rust/facet anti-shadow coverage.

Against the planned post-type-522 ledger of 576/663, a green type-517 gate moves acceptance to
**578/663** and leaves 85. This read-only oracle alone gives no credit.

## AC1024 `TABLESTYLE` type 504 exact-ready oracle (2026-08-15)

This read-only prescription resolves the last unnamed TABLESTYLE scalars and fully bounds the fixture's
single R24 frame. It is based on LibreDWG's
[`CellStyle_fields`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L213-L281) and
[`TABLESTYLE`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L924-L1010) orders, checked
against Autodesk's public
[`TABLESTYLE` DXF concepts](https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-DXF/files/GUID-0DBCA057-9F6C-4DEB-A66F-8A9B3C62FB1A.htm).
The complete bounded fixture parse ends at main bit 5859, string bit 6413 and handle bit 6686, leaving
only the derived terminal fill `11` before the 6688-bit payload boundary.

### Minimal logical schema

The snapshot body is:

```text
DwgTableStyle {
  description, bit_flags, template_style_handle?,
  table: DwgCellStyle, title: DwgCellStyle,
  header: DwgCellStyle, data: DwgCellStyle
}
DwgCellStyle {
  property_override_flags, merge_flags, background_color, content_layout,
  content_format: DwgCellContentFormat, margins: DwgCellMargins,
  borders: { Top?, HorizontalInside?, Bottom?, Left?, VerticalInside?, Right? }
}
DwgCellContentFormat {
  property_override_flags, property_flags, value_data_type, value_unit_type,
  value_format_string, rotation, block_scale, alignment, content_color,
  text_style_handle?, text_height
}
DwgCellMargins {
  vertical, horizontal, bottom, right, horizontal_spacing, vertical_spacing
}
DwgCellBorder {
  override_flags, border_type, color, lineweight, linetype_handle?, visible,
  double_line_spacing
}
```

The four roles are fields, not an identity vector. The writer derives R24 discriminator `RC=0`, format
version `BL=0`, base identity `(id=4,type=2,name=Table)`, override count 3 and native override identities
in exact selector order `(1,1,_TITLE)`, `(2,1,_HEADER)`, `(3,2,_DATA)`. It also derives cell data flags,
margin-presence flags, border count and border index masks `1,2,4,8,16,32`. Custom named cell styles are
rejected from this body and belong to `CELLSTYLEMAP`. None of those native discriminators, identities,
selectors, counts, masks, compact branches or stream positions is persisted.

### Exact cell-style fixture matrix

All four cells have cell type 5, data flags 1, property-override flags 0, background color
`index=0,rgb=c8000000,flag=0`, content layout 1, content-format override/property flags 0, value data type
512, unit type 0, empty value-format string, rotation 0, block scale 1, content color
`index=0,rgb=c1000000,flag=0`, margin override 1, margins `0.06,0.06,0.06,0.06,0.18,0.18`. The differences
are fully semantic:

| role | native identity | merge flags | alignment | text style | height | borders |
|---|---|---:|---:|---|---:|---:|
| Table | `4,2,Table` | 0 | 1 | null | 0.18 | 0 |
| Title | `1,1,_TITLE` | 32768 | 5 | `11` | 0.25 | 6 |
| Header | `2,1,_HEADER` | 0 | 5 | `11` | 0.18 | 6 |
| Data | `3,2,_DATA` | 0 | 2 | `11` | 0.18 | 6 |

Each of the 18 borders appears in mask order Top, HorizontalInside, Bottom, Left, VerticalInside, Right
and has override flags 0, border type 1, color `index=0,rgb=c1000000,flag=0`, lineweight -2, null
linetype, visibility 0 and double-line spacing 0.045. There are no color names, color-book names or
nonempty content-format strings. Values such as margin/border presence and the standard identity tuple
must be regenerated from the logical record; they are not defaults that permit data to be omitted.

### Native stream and handle order

The main stream is BOT/self/EED, common object prefix, `RC` discriminator, format-version `BL`, public
bit-flags `BL`, then the four cell records in Table/Title/Header/Data order. The string stream contains
`Standard`, `Table`, `_TITLE`, `_HEADER`, `_DATA` and four empty format strings in their corresponding
field order. The handle stream has 26 logical positions:

1. owner `86` code 8, reactor `86` code 4 and extension dictionary `104` code 3;
2. null template-style handle code 3;
3. one content text-style handle per cell: null for Table and `11` for Title/Header/Data, all code 5;
4. six null border-linetype handles per Title/Header/Data, all code 5.

Compact handle codes are writer choices derived from each named role and target. The exact frame oracle
is self handle `87`, four-byte MS frame prefix, payload/total `836/842`, handle/data/string bits
`258/6430/554`, main/string/handle ends `5859/6413/6686`, terminal fill `11`, one reactor, empty EED and
CRC `0x1784`. The parser and writer must exhaust and reproduce every boundary exactly.

### Append-only schema/facet landing

If the planned VisualStyle, type-543, type-559, type-522 and type-517 arms land in the documented order,
append TABLESTYLE at Rust body kind **17/payload field 18** and protobuf body field **24**. Inspect the
live maximum at landing and append after it if concurrent work changed that frontier; never renumber an
occupied tag. Suggested protobuf TABLESTYLE fields are `description=1, bit_flags=2,
template_style_handle=3, table=4, title=5, header=6, data=7`. Nested records should follow the logical
field order above and use role-named borders rather than native masks.

Extend the matching TypeScript, JSON Schema, GraphQL, EBNF/G4/Semio text and Kaitai/ABNF/Spicy/Semio
binary facets with the same typed records. Artifact, diff and mutation persistence carries those records
structurally. Anti-shadow assertions reject the RC/version copies, cell IDs/type/name constants,
selectors, counts, flags used only for conditional presence, border masks, compact numeric branches,
stream sizes/positions, fill, CRC, native frames and JSON/native envelopes.

### Strict acceptance gate

Extend the existing AC1024 fixture test and require:

1. attempted/decoded/encoded `1/1/1`, exact full frame and CRC `87:1784`;
2. exact four role-named styles and every nested color/content/margin/border value in the table above;
3. exact main/string/handle exhaustion, all 26 named handle roles and derived terminal fill `11`;
4. writer derivation of R24 discriminator/version, standard identities, native override order, counts,
   conditional flags and border masks, with none present in the snapshot/facets;
5. mutation plus inverse for flags, a content value, a margin, a text-style handle and a border field;
   atomic rejection of invalid enum, missing standard role, illegal custom identity, foreign/nullability
   violation, count overflow or unresolved handle;
6. original fixture equality through logical DSL/pack, diff/apply/inverse/absorb, mutation/inverse,
   analyzer/composer and native IO, plus Rust/facet anti-shadow coverage.

Against the planned post-type-517 ledger of 578/663, a green TABLESTYLE gate moves acceptance to
**579/663** and leaves 84. This read-only oracle alone gives no runtime credit.

## AC1024 `ACDBASSOC2DCONSTRAINTGROUP` type 540 exact-ready oracle (2026-08-15)

This read-only oracle replaces the stale 349-node/base-node interpretation above. The source hierarchy
comes from LibreDWG's
[`AcConstraintGroupNode` and derived-field macros](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L5605-L5695)
and the group order from
[`ASSOC2DCONSTRAINTGROUP`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L5697-L5718),
checked against Autodesk's public
[`AcDbAssoc2dConstraintGroup` model](https://help.autodesk.com/cloudhelp/2018/ENU/OARX-RefGuide/files/OREF-__MEMBERTYPE_Methods_AcDbAssoc2dConstraintGroup.html).
The separated class stream has exactly 31/174/31/31 entries. The preceding 33/250/33/33 main values
are maximum-node-ID watermarks, not counts, so every one of the **267** nodes is typed and there is no
base/unknown node variant in this fixture.

### Logical group and typed node union

```text
DwgAssoc2dConstraintGroup {
  action: DwgAssocAction,
  do_not_check_newly_added_constraints,
  work_plane: { origin, x_axis, y_axis },
  member_actions: [Handle],
  nodes: [DwgConstraintNode]
}
DwgConstraintNodeCore { id, connected_node_ids }
DwgGeomConstraintCore { node, owner_node_id, is_implied, is_active }
DwgConstraintGeometryCore { node, geometry_dependency_handle?, geometry_node_id }
DwgExplicitConstraintCore {
  geometric, value_dependency_handle, dimension_dependency_handle
}
```

The union and exact inherited suffix order are:

| union case | count | fields after `DwgConstraintNodeCore` |
|---|---:|---|
| `ConstrainedImplicitPoint` | 68 | constraint-geometry core; conditional point when a geometry dependency exists; point kind, point index, curve-node ID |
| `PointCurveConstraint` | 68 | geometric-constraint core |
| `ConstrainedBoundedLine` | 31 | constraint-geometry core; line origin, line direction, `is_ray`, start point, end point |
| `PointCoincidenceConstraint` | 28 | geometric-constraint core |
| `DistanceConstraint` | 20 | explicit-constraint core; direction kind; conditional direction vector |
| `PerpendicularConstraint` | 8 | geometric-constraint core |
| `HorizontalConstraint` | 7 | geometric-constraint core |
| `ParallelConstraint` | 8 | geometric-constraint core; datum relation derived from typed graph connections |
| `MidPointConstraint` | 6 | geometric-constraint core |
| `EqualLengthConstraint` | 6 | geometric-constraint core |
| `ColinearConstraint` | 6 | geometric-constraint core |
| `ConstrainedDatumLine` | 5 | constraint-geometry core; origin and direction |
| `FixedConstraint` | 4 | geometric-constraint core |
| `VerticalConstraint` | 2 | geometric-constraint core |

The node union case is the semantic class identity. Native class strings are derived from it and are not
stored alongside it. Native evidence corrects the upstream status prescription for this AC1024 cohort:
the main node order is signed node ID `BLd`, derived connection count `BL`, ordered connection IDs `BL`,
then the case suffix, with no node-status field. In frame `0x1149` the first node begins at main bit 124
and decodes exactly as id `8`, count `1`, connection `[9]`; interpreting the following bits as an `RC`
instead yields `64` and the impossible count `66`. Its suffix then yields geometry node id `0`, implicit
point type `0`, point index `-1`, and curve id `7`; the second node begins at bit 208 as id `16`, count
`3`, connections `[17,25,26]`. The writer derives maximum node ID, node count, connection counts, class
strings and all conditional presences from the logical graph. It must neither consume nor emit the
LibreDWG `PRE(R_2013b)` status field for these four native frames.

The first bounded-line case supplies a byte-exact boundary oracle. In frame `0x1149` it begins at main
bit 388 as id `1`, count `6`, connections `[3,5,12,19,20,27]`, followed by geometry-node ID `0`; its
typed suffix begins at bit 470: origin `(0,0,0)`, direction `(0,-1,0)`, `is_ray=false`, start
`(0,0,0)`, end `(0,-4,0)`, ending at bit 623. The following `PointCurveConstraint` begins exactly at
bit 623 as id `9`, count `2`, connections `[8,7]`. Therefore the bounded-line codec order above is
correct; any decoder that reaches this case at bit 472 has already over-consumed the preceding
`FixedConstraint`/node boundary by 84 bits and must not compensate inside the bounded-line suffix.
The apparent 32-bit anomaly before the bounded case was a one-bit inheritance error. Native AC1024
`AcGeomConstraint` stores `owner_id BL` followed by only `is_implied B`; `is_active` is the derived
logical value `true` and has no native bit. Consequently the preceding MidPoint ends at bit 355 and the
Fixed case decodes cleanly from bit 355 as id `32`, count `1`, connection `[14]`, owner `0`,
`is_implied=false`, ending exactly at bit 388. After the bounded case, PointCurve id `9` ends at bit
666 and the following PointCurve begins at bit 666 as id `17`, count `2`, connections `[16,13]`.
The large frame uses the same one-Boolean geometric core. Horizontal and Vertical share the existing
typed datum-line-index suffix with Parallel: Vertical begins at bit 404 as id `122`, count `1`,
connection `[116]`, owner `0`, implied false, then datum-line index `69`. The earlier `010100`
"framing" diagnosis was false; those bits are the start of this standard `BL` datum value. Horizontal,
Vertical must therefore use the typed datum payload and never persist an unnamed segment. Parallel does
not carry this suffix in the native AC1024 fixture; its datum relation is derived from its typed graph
connections.

In large frame `0x1f2c`, Parallel node 36 begins at bit 3978 as id `212`, count `2`, connections
`[61,23]`, owner `0`, implied false, and ends at bit 4021 with no datum-index suffix. Bounded-line node
37 begins exactly at bit 4021 as id `61`, count `4`, connections `[63,65,209,212]`, geometry-node ID
`0`; its suffix begins at bit 4083 with origin `(769.143798017536,218.5123802402951,0)`, direction
`(0,1,0)`, `is_ray=false`, start equal to origin, and end
`(769.143798017536,356.5123802402951,0)`, ending at bit 4492. Distance node 38 then begins at bit 4492
as id `125`, count `2`, connections `[78,95]`.

With this correction the read-only structural oracle consumes all **267/267** typed nodes. Main-stream
end positions equal the separated string-stream starts exactly in all four frames:
`0x1149 2782/2782`, `0x1f2c 20412/20412`, `0x2026 2974/2974`, and `0x207f 3038/3038`.

### Fixture action/dependency graph

All groups have action class version 1, action status 0, no action-core dependency entries, group version
0, `do_not_check_newly_added_constraints=false`, the XY work plane
`[(0,0,0),(1,0,0),(0,1,0)]`, null action-body and null group dimension-dependency slots. Action index and
maximum dependency index are respectively `3/7, 2/92, 3/7, 3/7` for handles
`1149,1f2c,2026,207f`.

The ordered member-action vectors contain 7/50/7/7 references. Across the four frames they resolve to
exactly 31 type-544 geometry dependencies, 20 type-541 value dependencies and 20 type-542 dimension
dependencies. Typed node handles close that ownership graph exactly:

- all 31 bounded-line geometry handles are code 4 and resolve to those type-544 members;
- all 68 implicit-point and five datum-line geometry roles are standard null code-4 handles;
- every distance node has one code-5 value handle to type 541 and one code-5 dimension handle to type
  542, producing 20 matched pairs.

The logical validator requires unique nonnegative node IDs, `max(id)` equal to the derived watermark,
closed and reciprocal undirected connection lists, and no duplicate connection. Every owner/curve/
geometry node ID must resolve to the required union case. Every nonnull typed dependency handle must be
present once in the ordered member-action relation, owned by this group action, and reciprocally point
back through its dependency/action core. The work-plane axes must be finite, unit, orthogonal and
right-handed. A mutation violating any graph invariant rejects atomically.

### Exact native streams, handles, fill and CRC

There are no reactors or extension dictionaries. The common owner and action owning-network roles resolve
to the same type-539 network in each frame. After the common owner, native handle order is owning network,
null action body, null group dimension slot, ordered member actions, then typed node handles in node order.
Members use compact code 3; geometry roles code 4; distance value/dimension roles code 5. Codes are writer
choices and never schema fields.

| handle | payload / total | handle / data / string bits | main end / string end | node max / count | typed node handle roles | fill | CRC |
|---|---|---|---|---|---:|---|---|
| `1149` | 1898 / 1904 | 459 / 14725 / 11926 | 2782 / 14708 | 33 / 31 | 16 | `111` | `aa02` |
| `1f2c` | 11375 / 11381 | 2831 / 88169 / 67724 | 20412 / 88136 | 250 / 174 | 96 | `1111111` | `059f` |
| `2026` | 1923 / 1929 | 467 / 14917 / 11926 | 2974 / 14900 | 33 / 31 | 16 | `111` | `2c14` |
| `207f` | 1931 / 1937 | 467 / 14981 / 11926 | 3038 / 14964 | 33 / 31 | 16 | `111` | `94c5` |

The small frames each have ten post-common outer roles (owning network, action body, group dimension and
seven members); the large frame has 53. Including the common owner yields 27 logical handle positions per
small frame and 150 for the large frame. Exact-frame admission requires main, class-string and handle
readers to reach the boundaries above before consuming the stated one-bit fill and validating the CRC.

### Append-only schema and facet landing

Following the planned VisualStyle, type-543, type-559, type-522, type-517 and TABLESTYLE arms, append the
constraint group at Rust body **kind 18/payload field 19** and protobuf body field **25**. Inspect the live
maximum at landing and append after it if concurrent work changed that frontier; never reuse or renumber
an occupied tag. Suggested protobuf group fields are `action=1,
do_not_check_newly_added_constraints=2, work_plane=3, member_actions=4, nodes=5`. The node record should
carry `id=1, connected_node_ids=2` and one append-only typed union arm; inherited cores are structured
messages, not flattened native fields.

Extend TypeScript, JSON Schema, GraphQL, EBNF/G4/Semio text and Kaitai/ABNF/Spicy/Semio binary facets with
the same union and cores. Artifact/diff/mutation codecs persist the logical graph structurally.
Anti-shadow assertions reject native class strings, maximum/count copies, node-status and connection-count
mirrors, group/version constants, null compatibility slots, conditional-presence flags, handle codes, stream boundaries, fill,
CRC, raw node tails and JSON/native envelopes.

### Strict acceptance gate

Extend the existing AC1024 fixture test and require:

1. attempted/decoded/encoded `4/4/4`, exact complete frame signatures and all four CRCs above;
2. the exact 267-case histogram, node order, IDs, connections and every case-specific scalar/vector;
3. graph closure across all node IDs and all 71 member dependencies, including the exact 31/20/20 typed
   dependency partition and 31/20/20 reciprocal node-handle partition;
4. exact main/class-string/handle exhaustion and `111/1111111/111/111` fill patterns;
5. node scalar, vector, connection, dependency handle and union-case mutation plus inverse; atomic
   rejection of dangling/asymmetric/duplicate graph relations, invalid union, foreign dependency,
   non-orthonormal plane, non-derived count/watermark or illegal AC1024 status;
6. original fixture equality through logical DSL/pack, diff/apply/inverse/absorb, mutation/inverse,
   analyzer/composer and native IO, plus Rust/facet anti-shadow coverage.

Against the planned post-TABLESTYLE ledger of 579/663, a green type-540 gate moves acceptance to
**583/663** and leaves 80. This read-only oracle alone gives no runtime credit.

## AC1024 `SCALE` type 507 x17 bounded exact oracle (2026-08-15)

The semantic record is exactly
`DwgAnnotationScale { name, paper_units, drawing_units, is_unit_scale }`. LibreDWG's
[`SCALE` order](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L1192-L1202) is checked
against Autodesk's public annotation-scale rule that the scale factor is
[`paper_units / drawing_units`](https://help.autodesk.com/cloudhelp/2019/ENU/OARX-ManagedRefGuide/files/OREFNET-Autodesk_AutoCAD_DatabaseServices_AnnotationScale_Scale.html).
Name is semantic display text, not regenerated from the ratio. Both unit values must be finite and
strictly positive; the computed factor is derived and is not another snapshot field.

The leading `BS=0` is the derived AC1024 scale-format flag and must be emitted with selector 2. The two
`BD` values use the standard one-bit default branch exactly when the logical value is 1.0 and full 64-bit
form otherwise. `is_unit_scale` is semantic: fixture `1:1` with units `1/1` is true, while
`1'-0\" = 1'-0\"` with equal units `12/12` is false. Therefore ratio equality alone cannot derive the
flag; validation requires `true => name == "1:1" && paper_units == 1 && drawing_units == 1`, while a
false custom/equal scale remains legal.

### Exact semantic and frame matrix

All values below are exact binary64 values; the fractional entries are powers-of-two fractions and incur
no decimal approximation.

| handle | name | paper / drawing | unit | payload / total | handle / data / string bits | main / string end | CRC |
|---|---|---|---:|---|---|---|---|
| `b7` | `1:1` | 1 / 1 | 1 | 19 / 24 | 31 / 121 / 58 | 46 / 104 | `8c8d` |
| `18b` | `1'-0" = 1'-0"` | 12 / 12 | 0 | 57 / 62 | 39 / 417 / 218 | 182 / 400 | `bfe9` |
| `19f` | `1/128" = 1'-0"` | 0.0078125 / 12 | 0 | 59 / 64 | 39 / 433 / 234 | 182 / 416 | `c523` |
| `1a0` | `1/64" = 1'-0"` | 0.015625 / 12 | 0 | 57 / 62 | 39 / 417 / 218 | 182 / 400 | `2229` |
| `1a1` | `1/32" = 1'-0"` | 0.03125 / 12 | 0 | 57 / 62 | 39 / 417 / 218 | 182 / 400 | `6fee` |
| `1a2` | `1/16" = 1'-0"` | 0.0625 / 12 | 0 | 57 / 62 | 39 / 417 / 218 | 182 / 400 | `2c17` |
| `1a3` | `3/32" = 1'-0"` | 0.09375 / 12 | 0 | 57 / 62 | 39 / 417 / 218 | 182 / 400 | `1242` |
| `1a4` | `1/8" = 1'-0"` | 0.125 / 12 | 0 | 55 / 60 | 39 / 401 / 202 | 182 / 384 | `f2ff` |
| `1a5` | `3/16" = 1'-0"` | 0.1875 / 12 | 0 | 57 / 62 | 39 / 417 / 218 | 182 / 400 | `dbb5` |
| `1a6` | `1/4" = 1'-0"` | 0.25 / 12 | 0 | 55 / 60 | 39 / 401 / 202 | 182 / 384 | `c38c` |
| `1a7` | `3/8" = 1'-0"` | 0.375 / 12 | 0 | 55 / 60 | 39 / 401 / 202 | 182 / 384 | `a815` |
| `1a8` | `1/2" = 1'-0"` | 0.5 / 12 | 0 | 55 / 60 | 39 / 401 / 202 | 182 / 384 | `4d63` |
| `1a9` | `3/4" = 1'-0"` | 0.75 / 12 | 0 | 55 / 60 | 39 / 401 / 202 | 182 / 384 | `34fa` |
| `1aa` | `1" = 1'-0"` | 1 / 12 | 0 | 43 / 48 | 39 / 305 / 170 | 118 / 288 | `001b` |
| `1ab` | `1-1/2" = 1'-0"` | 1.5 / 12 | 0 | 59 / 64 | 39 / 433 / 234 | 182 / 416 | `da90` |
| `1ac` | `3" = 1'-0"` | 3 / 12 | 0 | 51 / 56 | 39 / 369 / 170 | 182 / 352 | `5b57` |
| `1ad` | `6" = 1'-0"` | 6 / 12 | 0 | 51 / 56 | 39 / 369 / 170 | 182 / 352 | `9161` |

The `b7` main ends at bit 46 because both unit values use the one-bit 1.0 branch. Handle `1aa` ends at
118 because only paper units use that branch; all other non-unit entries end at 182. The separated TU
name stream begins at main end and reaches the string-end column exactly; the standard 17-bit string
footer then reaches the data-bit boundary. Every name is nonempty and unique in the owning scale list.

### Common object and exact owner stream

Every frame has self handle code 0, empty EED, one reactor via `BL` selector 1 and a missing extension
dictionary. Both owner and reactor resolve to handle `b6`, the type-42 scale-list dictionary. Handle `b7`
uses owner code 8 and reactor code 4; the later 16 frames use code 4 for both. There are no SCALE class
handles. After the two common roles, all frames end with terminal fill `1111111`; handle bits are 31 for
`b7` and 39 for the other 16. The writer derives compact codes and fill from the named owner/reactor
relations and persists neither.

### Append-only schema and facet landing

Following the planned VisualStyle, type-543, type-559, type-522, type-517, TABLESTYLE and type-540 arms,
append SCALE at Rust body **kind 19/payload field 20** and protobuf body field **26**. Inspect the live
maximum at landing and append after it if concurrent work changed the frontier; never reuse or renumber an
occupied tag. Suggested protobuf fields are `name=1, paper_units=2, drawing_units=3,
is_unit_scale=4`.

Add the same four semantic fields/arm to TypeScript, JSON Schema, GraphQL, EBNF/G4/Semio text and
Kaitai/ABNF/Spicy/Semio binary facets. Artifact, diff and mutation codecs carry them structurally.
Anti-shadow gates reject native format flag, derived factor, BD selector/default state, TU length/string
offsets, data/string/handle sizes, compact handle codes, fill, CRC, native frames and JSON/native
envelopes.

### Strict acceptance gate

Extend the existing AC1024 fixture test and require:

1. attempted/decoded/encoded `17/17/17`, exact semantic rows, frame sizes and all 17 CRCs above;
2. exact main/TU-string/common-handle exhaustion and terminal fill `1111111` on every frame;
3. deterministic flag `BS=0`, 1.0-default/full-BD selection and the exact owner/reactor roles;
4. nonempty unique names, finite positive units and canonical true-unit-scale validation without deriving
   a false equal-ratio entry into true;
5. name, paper units, drawing units and unit-scale mutation plus inverse; atomic rejection of empty/
   duplicate name, nonfinite/nonpositive unit, invalid canonical unit marker, foreign owner or nonzero
   native format flag;
6. original fixture equality through logical DSL/pack, diff/apply/inverse/absorb, mutation/inverse,
   analyzer/composer and native IO, plus Rust/facet anti-shadow coverage.

Against the planned post-type-540 ledger of 583/663, a green SCALE gate moves acceptance to
**600/663** and leaves 63. This read-only oracle alone gives no runtime credit.

## AC1024 `MLINESTYLE` x1 and type-508 `MLEADERSTYLE` x1 bounded exact oracles (2026-08-15)

These two style objects persist ordered semantic formatting records, not native flag/count/handle
envelopes. The MLINE prescription follows LibreDWG's
[`MLINESTYLE` order](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec#L4513-L4578) and
Autodesk's authoritative
[`MLINESTYLE` concepts](https://help.autodesk.com/cloudhelp/2023/ENU/AutoCAD-DXF/files/GUID-3EC12E5B-F5F6-484D-880F-D69EBE186D79.htm).
The multileader prescription follows LibreDWG's
[`MLEADERSTYLE` order](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L1460-L1546) and
Autodesk's
[`MLEADERSTYLE` field table](https://help.autodesk.com/cloudhelp/2025/CHT/AutoCAD-DXF/files/GUID-0E489B69-17A4-4439-8505-9DCE032100B4.htm).

### `MLINESTYLE` handle `18`

The logical record is:

```text
DwgMlineStyle {
  name, description,
  fill_enabled, display_miters,
  start_caps: { square, inner_arcs, round_outer_arcs },
  end_caps: { square, inner_arcs, round_outer_arcs },
  fill_color, start_angle, end_angle,
  elements: [DwgMlineStyleElement { offset, color, linetype }]
}
```

`name` is the semantic dictionary key; the native body name is its validated redundant projection and
must match the owning `ACAD_MLINESTYLE` entry. The writer derives the flag bitset and element count. The
fixture is `STANDARD`, empty description, all eight flag concepts false, typed fill color `ByLayer`,
start/end angle `pi/2`, and two elements in required descending-offset order:

| element | offset | color | linetype |
|---:|---:|---|---|
| 0 | 0.5 | `ByLayer` | `ByLayer` |
| 1 | -0.5 | `ByLayer` | `ByLayer` |

The two colors and fill color are native CMC `index=0,rgb=c0000000,flag=0`, deterministically derived
from typed `ByLayer`. The R2010 linetype sentinel 32767 is likewise derived from typed `ByLayer`; the
logical enum also supports `ByBlock`, `Continuous` and a named linetype relation without persisting an
index. Offsets remain ordered because the public MLINE style requires descending offsets.

The bounded frame is payload/total `83/88`, handle/data/string bits `26/638/140`, main/string ends
`481/621`, self handle code 0, empty EED, one reactor, no extension dictionary and CRC `7dc9`. Owner code
8 and reactor code 4 both resolve to type-42 dictionary handle `17`. There are no class handles. The
17-bit string footer reaches the handle boundary and the common handle stream ends with fill `11`.
Native branches are flag `BS0` selector 2, full `BD` for both angles and offsets, element count `RC2`,
and full signed `BS32767` for each linetype sentinel.

### `MLEADERSTYLE` handle `d8`

The logical record uses standard enums rather than numeric discriminants:

```text
DwgMLeaderStyle {
  content_type, draw_mleader_order, draw_leader_order, max_segment_points,
  first_segment_angle, second_segment_angle,
  leader: { kind, color, linetype_style, lineweight },
  landing: { enabled, gap }, dogleg: { enabled, length },
  description, arrow: { symbol?, size },
  text: {
    default_content, style, left_attachment, right_attachment,
    angle, alignment, color, height, frame, always_left, align_space,
    attachment_direction, top_attachment, bottom_attachment
  },
  block: { content?, color, scale, use_scale, rotation, use_rotation, connection },
  overall_scale, property_overrides_changed, annotative, break_size
}
```

Fixture semantics are:

| group | exact logical values |
|---|---|
| creation/content | MText content; DrawLeaderFirst; DrawLeaderHeadFirst; maximum two points; first/second angle 0 |
| leader | StraightLeader; typed `ByBlock` color; linetype style handle `14`; signed lineweight -2 |
| landing/dogleg | enabled, gap 0.09; enabled, length 0.36 |
| arrow | null symbol; size 0.18 |
| text | description `Standard`; empty default content; style handle `11`; left/right attachment MiddleOfTop; HorizontalAngle; LeftAlignment; typed `ByBlock`; height 0.18; no frame; not forced left; align space 0.18 |
| vertical text attachment | horizontal direction; top/bottom AttachmentCenter |
| block | null content; typed `ByBlock`; scale `(1,1,1)` enabled; rotation 0 enabled; ConnectExtents |
| overall | scale 1; property overrides unchanged; non-annotative; break size 0.125 |

The enum names follow Autodesk's public
[`ContentType`, draw-order, leader, text-angle/alignment and attachment enums](https://help.autodesk.com/view/OARX/2024/ENU/?guid=OARX-RefGuide-__MEMBERTYPE_Enumerations_AcDbMLeaderStyle).
Top/bottom value 9 is specifically `AttachmentCenter`; block connection 0 is `ConnectExtents`.
Colors are native CMC `index=0,rgb=c1000000,flag=0`, derived from typed `ByBlock`.

The exact frame is payload/total `120/125`, handle/data/string bits `72/888/140`, main/string ends
`731/871`, self handle code 0, empty EED, one reactor, no extension dictionary, **no terminal fill**, and
CRC `f49c`. Owner code 8 and reactor code 4 both resolve to type-42 dictionary handle `d7`. Class handles
then appear as line type code 5 -> handle `14` (type 57), null arrow code 5, text style code 5 -> handle
`11` (type 53), null block code 5. The four roles are typed optional/required relations; neither compact
codes nor null slots are retained separately.

Class version 2 is derived for AC1024 and R2013 `text_extended` is prohibited. The writer chooses the
standard compact branches from values: zero `BS/BD` selector 2, one `BS/BD` selector 1, full binary64 for
0.09/0.18/0.36/0.125, full signed `BL` for -2, and one-bit booleans. This reconstructs main bit 731
without persisted selector/default state.

### Append-only schemas and facets

Following the planned chain through SCALE, append MLINESTYLE at Rust body **kind 20/payload field 21**
and protobuf body field **27**, then MLEADERSTYLE at Rust **kind 21/payload field 22** and protobuf field
**28**. Inspect the live maxima at landing and append after them if concurrent work changed the frontier;
never reuse or renumber occupied tags. Suggested MLINE protobuf fields are `name=1, description=2,
fill_enabled=3, display_miters=4, start_caps=5, end_caps=6, fill_color=7, start_angle=8,
end_angle=9, elements=10`. Suggested MLEADER fields follow the logical groups above rather than the
native flat order.

Add both records/unions to TypeScript, JSON Schema, GraphQL, EBNF/G4/Semio text and
Kaitai/ABNF/Spicy/Semio binary facets. Artifact/diff/mutation codecs persist typed nested records and
relations structurally. Anti-shadow gates reject native class version, bitsets, collection counts,
numeric enum/sentinel mirrors, CMC words, conditional/default selectors, handle codes/null slots,
stream sizes/positions, fill, CRC, raw frames and JSON/native envelopes.

### Strict acceptance gates

Extend the existing AC1024 fixture test and require:

1. MLINE and MLEADER attempted/decoded/encoded `1/1/1` each, exact complete frames and CRCs
   `18:7dc9 d8:f49c`;
2. exact semantic values above, collection/enum/color decoding and main/string/handle exhaustion;
3. derived MLINE flag/count/ByLayer sentinel and validated dictionary-key/name equality;
4. exact MLEADER optional references, AC1024 class version/conditional fields and no terminal fill;
5. element insert/reorder/scalar/linetype mutation plus inverse, and MLEADER enum/scalar/style/block
   mutation plus inverse; atomic rejection of unordered/empty MLINE elements, duplicate dictionary key,
   invalid enum/angle/scale, unresolved required style, foreign relation or illegal AC1024 extension;
6. original fixture equality through logical DSL/pack, diff/apply/inverse/absorb, mutation/inverse,
   analyzer/composer and native IO, plus Rust/facet anti-shadow coverage.

Against the planned post-SCALE ledger of 600/663, green MLINESTYLE and MLEADERSTYLE gates move acceptance
to **602/663** and leave 61. These read-only oracles alone give no runtime credit.

## 2026-08-15 ACDBASSOCNETWORK Type 539 x5 Bounded Exact Oracle

### Authority and logical cut

Primary layout authority is LibreDWG's
[`AcDbAssocNetwork` record](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3030-L3050):
the inherited `AcDbAssocAction` core, network version `BS`, network action index `BL`, ordered action
count and per-action Boolean, then a second action-vector count; handles follow the inherited action
handles, ordered members, then the second vector. Autodesk defines the class as an action which keeps a
network of actions and permits hierarchical networks; `getActions()` returns the actions owned by the
network. The fixture resolves the encoding ambiguity more narrowly than the field names alone.

The strict logical schema is:

```text
DwgAssocNetwork {
  action: DwgAssocAction,
  network_action_index,
  actions: [DwgObjectRef<DwgAssocAction>]
}
```

`actions` is the sole ordered membership authority. Do **not** persist native `network_version`, either
count, the per-member `is_owned` bit, the second `owned_actions` vector, compact-number selectors, handle
codes, stream positions, fill or CRC. The five-frame evidence shows every member is semantically owned:
all 28 targets have an action-core `owning_network` back-reference equal to the containing network.
However, the four members which are themselves networks use `is_owned=false` and code 4, while all 24
non-network members use `is_owned=true` and code 3. The deterministic AC1024 writer therefore derives
the selector and handle strength from the target kind: nested network -> false/soft pointer; leaf action
-> true/hard owner. The second native vector has count zero in every frame and is emitted empty for this
AC1024 logical form; it is not reconstructed from the leaf-action subset.

`network_version=0` is the closed AC1024 constant. `network_action_index` remains a named semantic
allocator/index concept because its five values are not derivable from ordering. The inherited action
core remains the accepted typed action/dependency graph: status, action index, maximum dependency index,
ordered dependencies, owning-network relation and optional action body.

### Exact fixture graph

| network | action index / network index | ordered members by type | member-strength branch | parent network |
|---|---|---|---|---|
| `1148` | 59 / 4 | `1153` variable, `1154` variable, `1149` constraint group | 3 true/code 3 | `1bcb` |
| `1bcb` | 0 / 100 | `1148`, `1f26`, `2034`, `208d` networks | 4 false/code 4 | null root |
| `1f26` | 98 / 32 | `1f2c` constraint group; variables `2108,211e,2123,2129,2152,2157,215c,2162,2167,2178,21eb,2251,2256,2267` | 15 true/code 3 | `1bcb` |
| `2034` | 99 / 7 | `2026` constraint group, variables `2030,2031` | 3 true/code 3 | `1bcb` |
| `208d` | 100 / 4 | `207f` constraint group, variables `2089,208a` | 3 true/code 3 | `1bcb` |

This is one rooted hierarchy: root `1bcb` contains four subnetworks, and those subnetworks contain four
type-540 constraint groups plus all 20 recovered type-545 variables. Membership order is significant and
must be preserved exactly. Every one of the 28 target action prefixes was independently bounded and its
`owning_network` relation equals the table's containing network. The five network action cores themselves
have zero dependencies, null action bodies, status 0 and maximum dependency index 0. No member is
duplicated within a network and no child occurs in two networks.

### Stream, branch, fill and CRC oracle

All frames have self handle code 0, empty EED, one reactor (`BL` selector 1), missing extension
dictionary, no string stream, action class version 1, network version 0, zero action-core dependencies
and zero second-vector entries. Owner code 8 and reactor code 4 resolve to each network's type-42
dictionary; owning-network is code 4 (null only for root); action body is code 3/null. Nonzero action and
network indices and all action counts use full-value selector 1; zero values/counts use selector 2.

| handle | payload / frame bytes | handle / data / class bits | terminal one-fill | CRC |
|---|---:|---:|---:|---:|
| `1148` | 30 / 36 | 139 / 101 / 100 | 3 bits | `20c5` |
| `1bcb` | 30 / 36 | 146 / 94 / 93 | 2 bits | `957c` |
| `1f26` | 68 / 74 | 431 / 113 / 112 | 7 bits | `7758` |
| `2034` | 30 / 36 | 139 / 101 / 100 | 3 bits | `24f0` |
| `208d` | 30 / 36 | 139 / 101 / 100 | 3 bits | `b338` |

For every row the main reader stops one bit before the handle-data boundary because the absent-string
marker occupies that final data bit. The handle writer emits common roles, inherited action roles,
ordered member handles, no second-vector handles, then one-fill to the payload boundary and CRC16 over
the complete framed prefix plus payload. These sizes and CRCs are fixture oracles, never schema fields.

### Append-only schemas and facets

Following the planned chain through MLEADERSTYLE, append `DwgAssocNetwork` at Rust body **kind 22/payload
field 23** and protobuf body field **29**. Inspect live maxima at landing and append after them if
concurrent work changed the frontier; never reuse or renumber occupied tags. Suggested protobuf fields
are `action=1`, `network_action_index=2`, `actions=3` with repeated typed action references. There is no
member wrapper because native ownership/strength is derived.

Add the body and union arm to TypeScript, JSON Schema, GraphQL, EBNF/G4/Semio text and
Kaitai/ABNF/Spicy/Semio binary facets. Artifact/diff/mutation codecs persist the action core, semantic
index and ordered action references structurally. Anti-shadow gates reject version/count mirrors,
member ownership flags, duplicate owned-action vectors, numeric handle codes, stream sizes/positions,
fill, CRC, raw frames and JSON/native envelopes.

### Strict lifecycle gate

Extend the existing AC1024 fixture test and require:

1. type 539 attempted/decoded/encoded `5/5/5`, five complete exact frames and CRCs
   `1148:20c5 1bcb:957c 1f26:7758 2034:24f0 208d:b338`;
2. exact action/network indices, ordered membership lists and root/subnetwork hierarchy above;
3. all 28 reciprocal `member -> owning_network` proofs, exact target cohort totals network/constraint
   group/variable `4/4/20`, null owning-network only on root, and no duplicates or multi-parent actions;
4. deterministic derivation of four false/code-4 nested edges, 24 true/code-3 leaf edges and five empty
   second vectors, with exact main/string/handle exhaustion and terminal fill;
5. member insert/remove/reorder, index edit and nested-network move plus inverse; atomic rejection of
   cycles, duplicate/dangling/non-action members, inconsistent back-references, multiple roots or foreign
   ownership;
6. original fixture equality through logical DSL/pack, diff/apply/inverse/absorb, mutation/inverse,
   analyzer/composer and native IO, plus Rust/facet anti-shadow coverage.

Against the planned post-style ledger of 602/663, a green type-539 gate moves acceptance to **607/663**
and leaves 56. This read-only oracle alone gives no runtime credit.

## 2026-08-15 Dynamic Linear/Grip Type 527/528/530/532 x10 Bounded Exact Oracle

### Authority and shared logical cores

Primary layout authority is LibreDWG's
[`AcDbBlockElement`, parameter and grip cores](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3208-L3258),
[`BLOCKVISIBILITYGRIP`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3371-L3375),
[`BLOCKLINEARPARAMETER`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3397-L3406),
[`BLOCKFLIPGRIP`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3428-L3437), and
[`BLOCKLINEARGRIP`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3439-L3444).
Autodesk's dynamic-block documentation supplies the semantic vocabulary: a linear parameter defines a
distance between two key points, exposes a label/description and optional value set, and has startpoint
or midpoint base location; grip cycling makes a grip an alternate insertion point with a defined order.

Reuse the accepted logical cores structurally:

```text
DwgBlockElement { expression, name }
DwgBlockGrip {
  element, location, insertion_cycling, insertion_cycling_weight,
  updated_x_expression, updated_y_expression
}
DwgBlockTwoPointParameter {
  element, show_properties, chain_actions,
  definition_base, definition_end,
  properties[4], property_expression_ids[4], base_location
}
DwgBlockLinearParameter {
  parameter, distance_name, distance_description, label_offset,
  allowed_values
}
DwgBlockLinearGrip { grip, orientation }
DwgBlockFlipGrip {
  grip_without_xy_aliases, updated_flip_expression,
  updated_base_x_expression, updated_base_y_expression, orientation
}
DwgBlockVisibilityGrip { grip }
```

The native BlockElement version pairs are `29/2` in all ten frames and its application marker is zero;
derive and validate them rather than storing version/marker mirrors. Each evaluation expression has null
parent, versions `29/2`, null value and a semantic node ID. The two upstream grip integers are not opaque
states: the fixture proves they are expression IDs. Linear grips bind them to `UpdatedEndX/Y`, the
visibility grip to `UpdatedX/Y`; flip grips instead bind the first derived integer to `UpdatedFlip` and
the inherited pair to `UpdatedBaseX/Y`. Store typed expression relations and derive the integer IDs.

For the two linear parameters, property connection counts and the four-state cardinality are derived.
The four property-expression slots are typed optional expression IDs; the sole nonzero slot resolves to
the corresponding linear grip. Native value-set flags 8 plus zero minimum/maximum/increment select an
explicit allowed-value list. Persist that typed list, not the flags or inactive range fields. The native
`distance` scalar is the parameter-label offset (it is -15 for a 30-unit door definition and -10 for a
5-unit wall definition), not the definition-point distance or selected allowed value.

### Exact linear-parameter values and graph links

| handle / graph | element / node | definition base -> end | property connections / expression slots | label, description, offset | allowed values |
|---|---|---|---|---|---|
| `1110` / `110f` | `doorsize` / 1 | `(0,0,0)` -> `(30,-1.7763568394002505e-15,0)` | empty, empty, `(2,DisplacementX)`, `(2,DisplacementY)` / `[null,2,null,null]`; node 2 is grip `1111` | `Door Size`; `Sets the door size`; -15 | `[24,28,32,36,40]` |
| `1114` / `110f` | `wall` / 8 | `(0,0,0)` -> `(0,-5,0)` | empty, empty, `(9,DisplacementX)`, `(9,DisplacementY)` / `[null,9,null,null]`; node 9 is grip `1115` | `Wall Thickness`; `Sets the wall thickness`; -10 | `[4,6]` |

Both expose properties, do not chain actions, use StartPoint base location, have null EvalExpression
values and are owned by accepted evaluation graph `110f`. The near-zero endpoint residue is the semantic
binary64 coordinate and must survive. Connection codes equal the related grip expression IDs and are
derived from the typed graph relation; names are the standard `DisplacementX/Y` roles.

### Exact grip values and expression graph

| kind / handle / graph | own node; typed component relations | location | orientation | cycling / weight |
|---|---|---|---|---|
| linear `1111` / `110f` | 2; `UpdatedEndX` 3 -> `1112`, `UpdatedEndY` 4 -> `1113` | `(30,-1.7763568394002505e-15,0)` | same | false / -1 |
| linear `1115` / `110f` | 9; X 10 -> `1116`, Y 11 -> `1117` | `(0,-5,0)` | same | false / -1 |
| linear `1161` / `1155` | 223; X 224 -> `1162`, Y 225 -> `1163` | `(-1.245213028183116,-5.556481413226152,0)` | `(0,1,0)` | true / -1 |
| linear `1166` / `1155` | 228; X 229 -> `1167`, Y 230 -> `1168` | `(36,-2,0)` | `(1,0,0)` | true / -1 |
| flip `1119` / `110f` | 27; `UpdatedFlip` 28 -> `111a`, `UpdatedBaseX` 29 -> `111b`, `UpdatedBaseY` 30 -> `111c` | `(15,-12.5,0)` | `(1.7763568394002505e-15,12.5,0)` | false / -1 |
| flip `1122` / `110f` | 50; flip 51 -> `1123`, base X 52 -> `1124`, base Y 53 -> `1125` | `(15,-5,0)` | `(-15,0,0)` | false / -1 |
| flip `1157` / `1155` | 128; flip 136 -> `1158`, base X 144 -> `1159`, base Y 152 -> `115a` | `(0,3,0)` | `(0,3,0)` | false / -1 |
| visibility `111f` / `110f` | 47; `UpdatedX` 48 -> `1120`, `UpdatedY` 49 -> `1121` | `(-5,15,0)` | none | false / -1 |

All ten target relations resolve to accepted type-520 `DwgBlockGripLocationComponent` records whose
expression strings match the named roles above. The owning graph's ordered node table points to every
parameter/grip and component object. Grip names are `End Grip` for all four linear grips and `Grip` for
all flip/visibility grips. Orientations are finite and nonzero; they are semantic vectors and are not
normalized because the fixture legitimately uses scaled vectors.

### Stream, branch, fill and CRC oracle

All ten frames have self handle code 0, empty EED, zero reactors (`BL` selector 2), missing extension
dictionary and no class-local handles because every evaluation value is null rather than tag 91. The
only handle is the common owner relation to type-517 graph `110f` or `1155`; parameter `1110` uses code 8,
all other frames use code 12. All IDs/nonzero counts use full selector 1, zeros use selector 2, ordinary
coordinates use full `BD`, and 0/1 coordinate defaults use selectors 2/1 where available. Signed -1
cycling weight uses full `BLd` bits.

| class / handle | payload / frame bytes | handle / data / class / string bits | terminal one-fill | CRC |
|---|---:|---:|---:|---:|
| linear parameter `1110` | 228 / 233 | 12 / 1812 / 769 / 1026 | 4 | `3825` |
| linear parameter `1114` | 208 / 213 | 18 / 1646 / 507 / 1122 | 2 | `db09` |
| linear grip `1111` | 80 / 85 | 19 / 621 / 466 / 138 | 3 | `8c81` |
| linear grip `1115` | 64 / 69 | 19 / 493 / 338 / 138 | 3 | `2d68` |
| linear grip `1161` | 64 / 69 | 19 / 493 / 338 / 138 | 3 | `05f7` |
| linear grip `1166` | 64 / 69 | 19 / 493 / 338 / 138 | 3 | `521e` |
| flip grip `1119` | 73 / 78 | 17 / 567 / 476 / 74 | 1 | `3469` |
| flip grip `1122` | 65 / 70 | 17 / 503 / 412 / 74 | 1 | `a3d4` |
| flip grip `1157` | 57 / 62 | 17 / 439 / 348 / 74 | 1 | `040f` |
| visibility grip `111f` | 55 / 60 | 17 / 423 / 332 / 74 | 1 | `a543` |

For each frame `class + string-marker + strings == data boundary`, the owner handle exhausts the handle
stream before the listed one-fill, and CRC16 covers the exact framed prefix plus payload. Sizes, selector
branches, string widths, fill and CRC are fixture oracles only.

### Append-only schemas and facets

Following the planned type-539 frontier, append linear parameter, linear grip, flip grip and visibility
grip at Rust body **kinds 23/24/25/26 and payload fields 24/25/26/27**, and protobuf body fields
**30/31/32/33** respectively. Inspect live maxima at landing and append after them if concurrent work
changed the frontier; never reuse or renumber occupied tags. Embed shared element/parameter/grip records
structurally rather than repeating their fields.

Add all records and union arms to TypeScript, JSON Schema, GraphQL, EBNF/G4/Semio text and
Kaitai/ABNF/Spicy/Semio binary facets. Codecs persist typed points, value constraints and expression
relations. Anti-shadow gates reject version/marker mirrors, numeric expression tags beside typed values,
connection/count/cardinality mirrors, native value-set flags/inactive range values, grip-state integer
mirrors, selector branches, handle codes, stream sizes/positions, fill, CRC, raw frames and JSON/native
envelopes.

### Strict lifecycle gate

Extend the existing AC1024 fixture test and require:

1. attempted/decoded/encoded counts type 527/528/530/532 = `2/4/3/1` each and exact ten frames/CRCs above;
2. exact logical strings, points, label offsets, allowed-value lists, cycling values and orientations;
3. all 19 grip-component role resolutions, both parameter-to-grip property relations and owning
   evaluation-graph membership, with graph node/expression IDs unique and bounded;
4. derived core versions/marker, connection/value-set/count fields and exact main/string/handle/fill
   exhaustion;
5. parameter endpoint/label/value-list and grip location/orientation/cycling mutations plus inverse;
   expression-relation reorder/replace plus inverse; atomic rejection of dangling/wrong-kind graph refs,
   duplicate node IDs, role/string mismatch, invalid value constraint, nonfinite geometry, zero
   orientation, illegal base location or unsupported EvalExpression tag;
6. original fixture equality through logical DSL/pack, diff/apply/inverse/absorb, mutation/inverse,
   analyzer/composer and native IO, plus Rust/facet anti-shadow coverage.

Against the projected post-network ledger of 607/663, green gates for this ten-frame batch move
acceptance to **617/663** and leave 46. This read-only oracle alone gives no runtime credit.

## 2026-08-15 Code-Ready Named-Section Outer-Writer Handoff

This consolidates the six remaining typed named sections into one implementation contract for the
canonical AC1024 writer. It supersedes any live fallback which drops one of these sections or accepts
native section bytes. The decoder boundary is `semantic section bytes -> typed logical value`; the
encoder boundary is `typed logical value -> exact semantic section bytes`. Zero padding, D2 tokens,
stored-page capacity, page addresses, record offsets, sentinels, checksums and encryption remain
ephemeral outer-writer products.

### Section and page matrix

| ID / section | Exact semantic bytes | Page | Materialization before page header | Fixture payload / allocation |
|---|---:|---:|---|---:|
| 1 Header | 896 | 20 | zero-pad to `0x7400`, fresh exact D2 stream | 946 / 992 |
| 2 AuxHeader | 123 | 19 | zero-pad to `0x7400`, fresh exact D2 stream | 205 / 256 |
| 6 ObjFreeSpace | 89 | 15 | zero-pad to `0x7400`, fresh exact D2 stream | 169 / 224 |
| 8 RevHistory | 16 | 6 | zero-pad to `0x7400`, fresh exact D2 stream | 135 / 192 |
| 10 Preview | 86,191 | 2 | stored capacity 87,040; append 849 derived zero bytes | 87,040 / 87,072 |
| 12 AppInfoHistory | 1,390 | 4 | stored capacity 1,408; append 18 derived zero bytes | 1,408 / 1,440 |

The ordinary-page physical order is Preview 2, AppInfoHistory 4, RevHistory 6, ObjFreeSpace 15,
AuxHeader 19, Header 20. Header checksum/data checksum pairs are respectively
`b7a10127/fb4cfbad`, `80a113e2/323110de`, `20801131/43c80cd4`,
`945804cb/3ca700db`, `3ec2d57d/f187d0bf`, and `c601065b/2bf00312` when listed in the table's
section order. Descriptor logical size always remains the semantic byte count, never the padded or
compressed size.

### Exact semantic encoders

#### Header

Encode the complete typed AC1024 `header_variables.spec` sequence documented above into three
ephemeral writers: main bits, R2010 strings and handles. Then materialize in this order:

```text
start_sentinel[16]
u32le(section_data_length = 858)
u32le(combined_stream_boundary = 6136)
main/string/handle materialization[854]
u16le(crc16_c0c1(bytes[16..878]) = 0xd084)
end_sentinel[16]
```

The result is 896 bytes. Maintenance release 2 omits the conditional R2010 high-size word. The schema
must contain every named system-variable concept from the earlier field matrix, including all dimension
concepts, controls/dictionaries, terminal references, render variables and ordered strings. It must not
contain sentinels, section length, stream boundary, cursor positions, CRC or generic value vectors.

#### AuxHeader

The exact little-endian primitive order is
`<3B H H I i H H I 4H 6H 5I 4I Q H H 8I>` and the fixture tuple is:

```text
(255,119,1, 29,2, 105,-1, 32,12,1, 22,46,22,46,
 4,1381,261,2600,0,1, 0,0,0,16908544,65538,
 2454804,72759955,2454806,74552875, 8845, 0,40,
 0,0,0,105,0,0,0,0)
```

Map this in order to derived intro; target/maintenance version; semantic total saves; derived minus-one;
two save partitions and generation marker; two typed legacy stamps; a closed named compatibility profile
of six shorts plus five longs; created and updated `DwgJulianDate` pairs; semantic handle seed; derived
reserved zero plus terminal save generation; and eight terminal profile words, whose fourth word repeats
total saves. Constants and repeated totals are derived/validated rather than persisted. The cursor must
equal 123; there is no inner sentinel or CRC.

#### RevHistory

Encode little-endian `u32 format_major`, `u32 format_minor`, derived `u32 revisions.len`, then each typed
`u32 DwgRevisionCode` in order. Fixture value is `(0,0,1,[0])`; exact cursor 16. Reject count mismatch,
trailing bytes or an unknown untyped tail.

#### ObjFreeSpace

Encode little-endian `<Q Q I I B 8Q>`:

```text
reserved_zero = 0
approximate_registered_object_count = 679
updated = (2454806, 74552875)
bound_count = 4
bounds_u128_low_high = [(50,0), (100,0), (512,0), (0xffffffff,0)]
```

The reserved word, count and four bounds are AC1024 derivations. The registered-object value is a typed
graph projection/validation concept and is deliberately 679 even though the recovered framed-object
count is 663; do not substitute the frame count. Updated time must agree with Header/AuxHeader. Exact
cursor 89.

#### Preview

The only snapshot authority is the typed indexed image: width 329, height 256, bottom-up origin,
256 logical RGBA palette entries, 84,224 pixel indices, and background palette index 226. Pixel SHA-256
is `6fcf843df14f3783b010a85458f2dfca5ec264bae11d1e997475e4d1ec957bcd`.

Materialize the semantic section in this order:

```text
start_sentinel[16]
u32le(overall_size = 86155)
u8(record_count = 2)
record(code=1, u32le absolute_start=487, u32le size=80)
record(code=2, u32le absolute_start=567, u32le size=86056)
fixed_zero_bitmap_header_record[80]
BITMAPINFOHEADER <IiiHHIIiiII> = (40,329,256,1,8,0,84992,0,0,256,0)
palette[256] as BGRA, reserved byte derived zero
256 bottom-up rows: 329 logical indices then [0xe2,0xe2,0xe2]
end_sentinel[16]
```

Row stride is 332 and DIB pixel bytes are 84,992. Record starts derive from final Preview page payload
address `0x1c0`; they are not model state. The semantic cursor must equal 86,191 before the stored-page
padding. Code 2 is the indexed-bitmap variant; code 3 must map to a distinct typed WMF variant, and all
unsupported codes are rejected.

#### AppInfoHistory

Encode two fixed-width typed identifiers, `u32le class_version`, a T16 list name, `u32le entry_count`,
then each entry's 16-byte typed digest and canonical T16 value. T16 is `u16le(code_unit_count + 1)`,
UTF-16LE code units, and one `u16le(0)` terminator. Exact fixture prefix:

```text
history_id_1 = 53de381dec4321ca9619e1e2171a2a67
history_id_2 = 3bd97ff73cbbce08a053d8edd28dc5c7
class_version = 0
list_name = "AppInfoDataList"
entry_count = 4
```

Ordered entries and T16 text lengths including neither count nor terminator are:

1. digest `1bd848f3cc0a3e4dbab1cf81f7b450b3`, 13 code units, `18.0.40.0.200`;
2. digest `b8d0f025a1d79349b2fa9bf9286fa1fd`, 113 code units,
   `Autodesk DWG.  This file is a Trusted DWG last saved by an Autodesk application or Autodesk licensed application.`;
3. digest `e0859ff2f94f6810ab9108002b27b3d9`, 333 code units, canonical property-set rendering
   `<prop_set fmt_id="{f29f85e0-4ff9-1068-ab91-08002b27b3d9}"><prop id="8"><string>Brian</string></prop><prop id="10"><datetime>2008-12-05T20:42:32</datetime></prop><prop id="258"><string>AutoCAD 2009</string></prop><prop id="259"><string>D.40.0.200</string></prop><prop id="12"><datetime>2008-12-03T20:12:39</datetime></prop></prop_set>`;
4. digest `e8e09651c5ceb244a8bff6e83b859d44`, 159 code units, canonical product rendering with literal outer
   quotes and escaped inner quotes:
   `"<ProductInformation name =\"AutoCAD\" build_version=\"D.40.0.200\" registry_version=\"18.0\" install_id_string=\"ACAD-8001:409\" registry_localeID=\"1033\"/>"`.

Persist entry variants and typed property/product fields, never either rendered markup string. Preserve
the property order `8,10,258,259,12` and product attribute order shown. The semantic cursor must equal
1,390 before 18 stored-page zeros.

### Code-level assembly and rejection gates

1. Implement one strict `decode_<section>(semantic: &[u8]) -> TypedSection` and one
   `encode_<section>(&TypedSection) -> Vec<u8>` for each row. Decoders require exact cursor exhaustion;
   encoders assert the fixture-independent derived invariants and the fixture test asserts exact sizes.
2. Materialize all logical objects/handles first. Only then derive `HANDSEED`, AuxHeader repetitions,
   ObjFreeSpace statistics and Header handle references. A failed derivation rejects the entire export.
3. Materialize semantic section bytes before any descriptor/page allocation. Stored sections pad only to
   their capacities; ordinary sections pad to `0x7400` then independently reset the exact D2 encoder.
4. Allocate pages in physical order, then derive Preview absolute record starts, clear page headers,
   checksums, encrypted headers, section descriptors and maps. No generated value flows back into the
   logical snapshot.
5. Existing lifecycle tests must prove typed decode/encode equality for all six payloads, field mutation
   plus inverse, anti-shadow Rust/facet scans, exact D2/stored payloads, and final original fixture bytes
   through native IO, DSL/pack, diff/apply/inverse/absorb, mutation/inverse and analyzer/composer.

## 2026-08-15 Full AC1024 Header Logical-Model Handoff

This is the final read-only Header oracle for the outer writer. Fixture values were independently decoded
from the 896-byte native `AcDb:Header` with LibreDWG 0.13.3 and reconciled against the AC1024 branches of
`header_variables.spec`; the existing framing probe independently proves length, boundary and CRC. The
field order below is serialization order. `B`, `BS`, `BSd`, `BL`, `BLd`, `BD`, `TIMEBLL`, `CMC`, `2RD`,
`3BD`, `RC`, `H` and `T` name standard codecs, not schema representations.

### Logical schema cut

Persist only named standard values: booleans/enums/scalars, typed colors, dates, points/vectors, strings,
and typed object relations. Do not persist compact-number selectors, handle code/length nibbles, numeric
handle duplicates next to relations, main/string/handle cursors, sentinels, section sizes, bit counts,
CRC, padding, D2 bytes, or LibreDWG `unknown_*` fields. Header references are relations to the one logical
object graph; fixture handles below are resolution oracles only. `HANDSEED` is derived after final handle
allocation and is never independently mutable.

Recommended nested records are `DwgUnitConversions`, `DwgDrawingModes`, `DwgDisplaySettings`,
`DwgUserSettings`, `DwgTimeState`, `DwgCurrentState`, `DwgSpaceGeometry` (paper/model),
`DwgDimensionSettings`, `DwgHeaderCatalogRelations`, `DwgDrawingPolicy`, `DwgRenderSettings`, and
`DwgHeaderStrings`. Nesting must preserve the following flat wire order.

### Main-stream order and exact fixture values

#### Units, modes and integer/display settings

| Codec | Ordered named concepts and fixture values |
|---|---|
| `BD x4` | conversion ratios `unit1=412148564080`, `unit2=1`, `unit3=1`, `unit4=1` |
| `B` | `DIMASO=1, DIMSHO=1, PLINEGEN=0, ORTHOMODE=0, REGENMODE=1, FILLMODE=1, QTEXTMODE=0, PSLTSCALE=1, LIMCHECK=0` |
| `B` | `USRTIMER=1, SKPOLY=0, ANGDIR=0, SPLFRAME=0, MIRRTEXT=0, WORLDVIEW=1, TILEMODE=1, PLIMCHECK=0, VISRETAIN=1, DISPSILH=0, PELLIPSE=0` |
| `BS` | `PROXYGRAPHICS=1` |
| `BSd/BS` | `TREEDEPTH=3020, LUNITS=4, LUPREC=5, AUNITS=0, AUPREC=2, ATTMODE=1, PDMODE=0` |
| `BSd x5` | `USERI1..5 = 0,0,0,0,0` |
| `BS` | `SPLINESEGS=8, SURFU=6, SURFV=6, SURFTYPE=6, SURFTAB1=6, SURFTAB2=6, SPLINETYPE=6, SHADEDGE=3, SHADEDIF=70, UNITMODE=0, MAXACTVP=64, ISOLINES=4, CMLJUST=0, TEXTQLTY=50` |

The main decoder encounters the derived compatibility markers described below between the ratios and
`DIMASO`, after `LIMCHECK`, and before `USERI1`; they consume native bits but are not logical fields.

#### Scalar, time and current-state settings

| Codec | Ordered named concepts and fixture values |
|---|---|
| `BD` | `LTSCALE=1, TEXTSIZE=0.2, TRACEWID=0.05, SKETCHINC=0.1, FILLETRAD=0, THICKNESS=0, ANGBASE=0, PDSIZE=0, PLINEWID=0` |
| `BD x5` | `USERR1..5 = 0,0,0,0,0` |
| `BD` | `CHAMFERA=0, CHAMFERB=0, CHAMFERC=0, CHAMFERD=0, FACETRES=0.5, CMLSCALE=1, CELTSCALE=1` |
| `TIMEBLL` | universal creation `TDUCREATE=(2454804,72759955)`; universal update `TDUUPDATE=(2454806,74552875)` |
| `TIMEBLL` | editing duration `TDINDWG=(0,6518776)`; user timer `TDUSRTIMER=(0,6518761)` |
| `CMC` | current entity color `CECOLOR=index 0` |
| `BD` | `PSVPSCALE=0` |

Local `TDCREATE`/`TDUPDATE` are computed views from universal dates and the typed timezone; they are not
another native field pair. The three 32-bit producer markers between `TDUUPDATE` and `TDINDWG` are not
timestamps and must not become schema state.

#### Paper-space geometry

All points are ordered XYZ and all limits XY:

| Codec | Ordered named concepts and fixture values |
|---|---|
| `3BD` | `PINSBASE=(0,0,0)` |
| `3BD` | `PEXTMIN=(0.62883212269412,0.79966732798827,-1.125e-11)` |
| `3BD` | `PEXTMAX=(9.02982107914386,7.20016027280744,3e-13)` |
| `2RD` | `PLIMMIN=(-0.70054181917446,-0.22810038619154)`; `PLIMMAX=(10.29945794052965,8.27189937351257)` |
| `BD` | `PELEVATION=0` |
| `3BD` | `PUCSORG=(0,0,0), PUCSXDIR=(1,0,0), PUCSYDIR=(0,1,0)` |
| `BS` | `PUCSORTHOVIEW=0` |
| `3BD x6` | `PUCSORGTOP, PUCSORGBOTTOM, PUCSORGLEFT, PUCSORGRIGHT, PUCSORGFRONT, PUCSORGBACK = (0,0,0)` each |

The paper UCS name, orthographic reference and base are null typed relations in the handle stream.

#### Model-space geometry

| Codec | Ordered named concepts and fixture values |
|---|---|
| `3BD` | `INSBASE=(0,0,0)` |
| `3BD` | `EXTMIN=(-288.76172308672358,19.80568762453697,-3.491819e-8)` |
| `3BD` | `EXTMAX=(1152.17454179530682,861.51960331960095,8.149357e-8)` |
| `2RD` | `LIMMIN=(0,0)`; `LIMMAX=(12,9)` |
| `BD` | `ELEVATION=0` |
| `3BD` | `UCSORG=(0,0,0), UCSXDIR=(1,0,0), UCSYDIR=(0,1,0)` |
| `BS` | `UCSORTHOVIEW=0` |
| `3BD x6` | `UCSORGTOP, UCSORGBOTTOM, UCSORGLEFT, UCSORGRIGHT, UCSORGFRONT, UCSORGBACK = (0,0,0)` each |

The model UCS name, orthographic reference and base are null typed relations in the handle stream.

#### Dimension settings

This table is the complete AC1024 dimension main-stream order. Empty dimension formatting strings are
listed separately in the string stream, and dimension object/style relations separately in the handle
stream.

| Codec | Ordered named concepts and fixture values |
|---|---|
| `BD` | `DIMSCALE=1, DIMASZ=0.125, DIMEXO=0.0625, DIMDLI=0.25, DIMEXE=0.125, DIMRND=0, DIMDLE=0.125, DIMTP=0, DIMTM=0` |
| `BD,BD,BS,CMC` | `DIMFXL=1, DIMJOGANG=0.78539816339745, DIMTFILL=0, DIMTFILLCLR=index 0` |
| `B` | `DIMTOL=0, DIMLIM=0, DIMTIH=0, DIMTOH=0, DIMSE1=0, DIMSE2=0` |
| `BS` | `DIMTAD=1, DIMZIN=3, DIMAZIN=2, DIMARCSYM=0` |
| `BD` | `DIMTXT=0.125, DIMCEN=0.125, DIMTSZ=0, DIMALTF=25.4, DIMLFAC=1, DIMTVP=0, DIMTFAC=1, DIMGAP=0.09, DIMALTRND=0` |
| `B/BS` | `DIMALT=0, DIMALTD=2, DIMTOFL=0, DIMSAH=0, DIMTIX=0, DIMSOXD=0` |
| `CMC x3` | `DIMCLRD=index 0, DIMCLRE=index 0, DIMCLRT=index 0` |
| `BS` | `DIMADEC=2, DIMDEC=5, DIMTDEC=5, DIMALTU=2, DIMALTTD=2, DIMAUNIT=0, DIMFRAC=1, DIMLUNIT=4, DIMDSEP=46, DIMTMOVE=0, DIMJUST=0` |
| `B/BS` | `DIMSD1=0, DIMSD2=0, DIMTOLJ=1, DIMTZIN=0, DIMALTZ=0, DIMALTTZ=0, DIMUPT=0, DIMATFIT=3, DIMFXLON=0` |
| `B,BD,BD` | `DIMTXTDIRECTION=0, DIMALTMZF=100, DIMMZF=100` |
| `BSd x2` | `DIMLWD=-2, DIMLWE=-2` |

#### Text stack, drawing policy and rendering

| Codec | Ordered named concepts and fixture values |
|---|---|
| `BS` | `TSTACKALIGN=1, TSTACKSIZE=70` |
| derived `BLx` | packed `FLAGS=0x2a1d` materializes semantic `CELWEIGHT=-1, ENDCAPS=0, JOINSTYLE=0, LWDISPLAY=0, XEDIT=1, EXTNAMES=1, PSTYLEMODE=1, OLESTARTUP=0` |
| `BS` | `INSUNITS=1, CEPSNTYPE=0`; therefore no `CPSNID` relation |
| `RC` | `SORTENTS=127, INDEXCTL=0, HIDETEXT=1, XCLIPFRAME=0, DIMASSOC=2, HALOGAP=0` |
| `BS/RC` | `OBSCOLOR=257, INTERSECTIONCOLOR=257, OBSLTYPE=0, INTERSECTIONDISPLAY=0` |
| `B` | `CAMERADISPLAY=0` |
| `BD` | `STEPSPERSEC=2, STEPSIZE=6, 3DDWFPREC=2, LENSLENGTH=50, CAMERAHEIGHT=0` |
| `RC` | `SOLIDHIST=1, SHOWHIST=1` |
| `BD` | `PSOLWIDTH=0.25, PSOLHEIGHT=4, LOFTANG1=1.5707963267949, LOFTANG2=1.5707963267949, LOFTMAG1=0, LOFTMAG2=0` |
| `BS/RC` | `LOFTPARAM=7, LOFTNORMALS=1` |
| `BD` | `LATITUDE=37.795, LONGITUDE=-122.39400000000001, NORTHDIRECTION=0` |
| `BLd` | `TIMEZONE=-8000` |
| `RC` | `LIGHTGLYPHDISPLAY=1, TILEMODELIGHTSYNCH=1, DWFFRAME=2, DGNFRAME=0` |
| `B/CMC` | `REALWORLDSCALE=1, INTERFERECOLOR=index 256` |
| `RC/BD` | `CSHADOW=0, SHADOWPLANELOCATION=0` |

### Handle-stream order and typed relation oracles

All handles below are logical object relations. Encode them in this exact encounter order with the
standard Header handle writer; derive native code and byte width from the relation and resolved handle.
`null` emits the standard null handle. A stale numeric ID beside the relation is forbidden.

| Ordered relation group | Fixture-resolved handles |
|---|---|
| current state | `HANDSEED=8845` derived; `CLAYER=2109, TEXTSTYLE=376, CELTYPE=21, CMATERIAL=150, DIMSTYLE=578, CMLSTYLE=24` |
| paper UCS | `PUCSNAME=null, PUCSORTHOREF=null, PUCSBASE=null` |
| model UCS | `UCSNAME=null, UCSORTHOREF=null, UCSBASE=null` |
| dimension references | `DIMTXSTY=17, DIMLDRBLK=null, DIMBLK=568, DIMBLK1=null, DIMBLK2=null, DIMLTYPE=null, DIMLTEX1=null, DIMLTEX2=null` |
| table controls | `BLOCK=1, LAYER=2, STYLE=3, LTYPE=5, VIEW=6, UCS=7, VPORT=8, APPID=9, DIMSTYLE=10` |
| core dictionaries | `ACAD_GROUP=13, ACAD_MLINESTYLE=23, NAMED_OBJECT=12` |
| document dictionaries | `LAYOUT=26, PLOTSETTINGS=25, PLOTSTYLENAME=14, MATERIAL=114, COLOR=115, VISUALSTYLE=153` |
| terminal block/linetype relations | `BLOCK_RECORD_PSPACE=88, BLOCK_RECORD_MSPACE=31, LTYPE_BYLAYER=21, LTYPE_BYBLOCK=20, LTYPE_CONTINUOUS=22` |
| render visual styles | `INTERFEREOBJVS=null, INTERFEREVPVS=null, DRAGVS=null` |

Every non-null relation must resolve to its required object/table/dictionary kind. `CLAYER`, `TEXTSTYLE`,
`CELTYPE`, `DIMSTYLE`, `CMLSTYLE`, dimension named blocks/linetypes and standard terminal references must
also resolve to the semantic names required by their corresponding standard concepts. The fixture's
current named values include layer `Wall`, text style `Notes`, linetype `ByLayer`, dimension style
`Architectural`, multiline style `Standard`, dimension text style `Standard`, and arrow block
`_ArchTick`.

### R2010 string-stream order and exact values

After locating the separate Header string stream from the standard size flag, decode/encode exactly:

```text
unit1_name = "m"
unit2_name = ""
unit3_name = ""
unit4_name = ""
MENU = "."
DIMPOST = ""
DIMAPOST = ""
DIMALTMZS = ""
DIMMZS = ""
HYPERLINKBASE = ""
STYLESHEET = ""
FINGERPRINTGUID = "{AE360294-492A-4B40-8D12-1DA91F648E9C}"
VERSIONGUID = "{83F64250-0F55-40D4-AE09-768E87CF41F7}"
PROJECTNAME = ""
```

These are semantic strings, but UTF-16 code-unit lengths, terminators, string-stream size/flag and bit
alignment are derived. GUIDs should use a validated typed GUID scalar and canonical brace/uppercase
rendering rather than an unconstrained lexical shadow.

### Required derived compatibility values

The native AC1024 stream contains undocumented producer-profile words. They are required to reproduce
this Autodesk 2009 fixture but are not named standard concepts and therefore must not be persisted or
offered as mutation fields. Select them from the same closed typed Autodesk-2009 application/version
profile used by AuxHeader/AppInfo and reject an unsupported profile:

| Position in main order | Native codec/value | Deterministic treatment |
|---|---|---|
| after four conversion ratios | `BLx 2454805`, `BL 60784745` | derived producer date/profile pair |
| after `LIMCHECK` | `B 0` | derived AC1024 feature marker |
| after `PDMODE` | `BL 808464432 (0x30303030)`, `BL 486869248 (0x1d050900)`, `BL 1295333680 (0x4d353930)` | derived producer-generation triple |
| after `TDUUPDATE` | `BL 1145320500 (0x44443434)`, `BL 336396546 (0x140d0102)`, `BL 959722801 (0x39343531)` | derived producer-generation triple |
| after `CAMERADISPLAY` | `BL 0`, `BL 10`, `BD 1` | derived R2007+/R2010 render-profile triple |
| after `SHADOWPLANELOCATION` | `BS 1`, `BS 180`, `BS 36874`, `BS 32843` | derived terminal compatibility profile |

Also derive and validate: Header sentinels; section-data length 858; combined stream boundary 6,136;
maintenance-release-2 omission of the high size word; packed `FLAGS=0x2a1d`; every bitcode selector;
handle code/width and `HANDSEED=8845`; CMC native flags; string-stream framing; partial-byte fill;
CRC16 `0xd084` over bytes `16..878`; final semantic size 896; zero-padding to `0x7400`; exact D2 length
946; and page-20 allocation/checksums `992`, `b7a10127/fb4cfbad`.

### Code-ready decoder/writer sequence

1. Verify fixed start sentinel and bounded 896-byte section. Read `size=858` and `bitsize=6136`; reject
   the maintenance-release-2 high-size branch and any value that cannot delimit the three streams.
2. Decode main values in the tables' exact order, consuming/validating derived profile words in place.
   Decode colors into typed semantics and unpack `FLAGS` immediately into its eight named concepts.
3. Decode string and handle streams independently in the standard encounter orders above. Resolve every
   handle to the logical graph and discard native code/width. Require exact stream cursor exhaustion and
   only standard terminal fill.
4. Validate all cross-section invariants: UTC dates against AuxHeader/ObjFreeSpace; application profile
   against AuxHeader/AppInfo; dictionary/control/standard-name relations against typed objects; terminal
   refs and handle seed against the complete allocation graph.
5. Writer performs the inverse from typed values, using separate ephemeral main/string/handle writers.
   It derives profile words, packed flags, bit selectors, string framing, handle spelling, boundary,
   lengths, CRC and sentinels. Any unsupported logical value/profile or unresolved relation rejects the
   whole export atomically.
6. The existing lifecycle gate must assert field-by-field fixture values above, exact 896 semantic bytes,
   exact D2 page payload, mutation/inverse for each nested group, invalid-relation/profile rejection,
   anti-shadow scans across Rust/facets/codecs, and original-byte equality through every required route.

## 2026-08-15 SectionInfo Fixed-Name Scratch Derivation

The 64-byte `name` member in each of the 14 native SectionInfo descriptors is not zero-filled by the
Autodesk 2009 producer. The visible name and its NUL overwrite a deterministic scratch image; the bytes
after that NUL are allocator/work-buffer residue. They are not document concepts and must never enter the
snapshot, diff, mutation, DSL, pack or facets. Exact export can reproduce them without replay by rebuilding
the producer scratch image from typed AppInfoHistory, compressed PageMap, the ordered provisional page
plan, and a closed Autodesk-2009 scratch profile.

### Absolute buffer construction

Build one ephemeral `scratch[1684]` using offsets in the decoded SectionInfo body:

1. Zero it. Canonically encode typed AppInfoHistory with the producer's provisional timestamp pass,
   described below, into `scratch[0..1390]`; leave its stored-capacity padding `1390..1408` zero.
2. Copy exact deterministically compressed PageMap bytes `page_map_d2[0..170]` to `scratch[0..170]`.
   This overlay explains both the reserved descriptor and the first five tail bytes of ID 13.
3. Write the generated provisional page/next-descriptor staging fragments in the table below. These are
   a projection of the ordered page plan during Autodesk's pre-gap/pre-finalization pass, not retained
   bytes.
4. For each final descriptor, start `name[64] = scratch[name_position..name_position+64]`, copy the ASCII
   section name at offset zero, and write one NUL immediately after it. Do not clear anything else.
5. ID 0 follows the same rule with an empty name: its `name_position=52`, so only byte 52 becomes NUL and
   bytes `53..116` remain compressed PageMap bytes.

This absolute construction is important: slicing each source from offset zero gives the wrong result.

### All 14 exact name-buffer mappings

| ID / final name | SectionInfo name span | NUL / retained-tail span | Deterministic pre-overwrite source |
|---|---:|---:|---|
| 0 / empty | `52..116` | `52 / 53..116` | `PageMapD2[52..116]`, then NUL at its first byte |
| 13 / `AcDb:FileDepList` | `148..212` | `164 / 165..212` | `PageMapD2[165..170]` then provisional AppInfoHistory `[170..212]` |
| 12 / `AcDb:AppInfoHistory` | `260..324` | `279 / 280..324` | provisional AppInfoHistory `[280..324]` |
| 11 / `AcDb:AppInfo` | `372..436` | `384 / 385..436` | provisional AppInfoHistory `[385..436]` |
| 10 / `AcDb:Preview` | `484..548` | `496 / 497..548` | provisional AppInfoHistory `[497..548]` |
| 9 / `AcDb:SummaryInfo` | `596..660` | `612 / 613..660` | provisional AppInfoHistory `[613..660]` |
| 8 / `AcDb:RevHistory` | `708..772` | `723 / 724..772` | provisional AppInfoHistory `[724..772]` |
| 7 / `AcDb:AcDbObjects` | `820..884` | `836 / 837..884` | provisional AppInfoHistory `[837..884]` |
| 6 / `AcDb:ObjFreeSpace` | `1044..1108` | `1061 / 1062..1108` | AppInfoHistory `[1062..1076]`, then staging fragment A |
| 5 / `AcDb:Template` | `1156..1220` | `1169 / 1170..1220` | AppInfoHistory `[1170..1188]`, then staging fragment B |
| 4 / `AcDb:Handles` | `1268..1332` | `1280 / 1281..1332` | AppInfoHistory `[1281..1300]`, then staging fragment C |
| 3 / `AcDb:Classes` | `1380..1444` | `1392 / 1393..1444` | AppInfoHistory zero padding `[1393..1408]`, then staging fragment D |
| 2 / `AcDb:AuxHeader` | `1492..1556` | `1506 / 1507..1556` | zero byte, then staging fragment E |
| 1 / `AcDb:Header` | `1604..1668` | `1615 / 1616..1668` | staging fragment F |

The equality was checked at every byte. For example ID 12's entire 44-byte tail equals generated
AppInfoHistory at the same absolute offsets, while ID 13 changes source exactly at absolute offset 170,
the end of the compressed PageMap.

### Provisional AppInfoHistory pass

The scratch pass uses the same typed identifiers, digests, ordered property set, product information,
UTF-16 renderer and field order as the final AppInfoHistory encoder. Its sole fixture-visible difference
is the property-10 DateTime normalization: final `2008-12-05T20:42:32` is rendered provisionally as
`2008-12-04T23:42:32`, a profile-defined `-21 h` save-stage adjustment. Consequently ID 9's tail is:

```text
00 + UTF16LE("datetime>2008-12-04T23:")
```

Only the UTF-16 characters at absolute offsets 650 (`4` versus final `5`) and 656 (`3` versus final `0`)
differ from the final semantic AppInfoHistory section. The adjustment is a named rule of the closed
`Autodesk2009D400200ScratchProfile`; it is not another persisted timestamp or a retained lexical string.
All other fixture-visible AppInfoHistory slices are the ordinary canonical render of typed fields.

### Generated staging fragments

Use little-endian helpers `page(page_number, d2_size, logical_offset=0)` = `u32,u32,u64`, and
`next(size, pages=1, max=0x7400)` = `u64,u32,u32`. The producer pre-gap pass numbers D2 pages two below
their final physical page number. The exact fragments are:

| Fragment / absolute span | Generated content |
|---|---|
| A / `1076..1108` | `page(13,169,0)` then `next(6,1,0x7400)` |
| B / `1188..1220` | `page(14,129,0)` then `next(2093,1,0x7400)` |
| C / `1300..1332` | `page(15,1921,0)` then `next(8207,1,0x7400)` |
| D / `1408..1444` | `u32 scratch_cookie(Classes)=0xeb09a3a4`, `page(16,4661,0)`, then `next(123,1,0x7400)` |
| E / `1508..1556` | `fixed_name16("AcDb:Handles")`, `page(17,203,0)`, then `next(896,1,0x7400)` |
| F / `1616..1668` | four zeros, `fixed_name16("AcDb:Classes")`, `page(18,942,0)`, `fixed_name12("AcDb:Header")`, `u32 scratch_cookie(Header)=0xeb34ab24` |

`fixed_name16` writes ASCII, NUL and derived zero fill to 16; `fixed_name12("AcDb:Header")` is exactly
the eleven ASCII bytes plus NUL. The two cookies are generated internal scratch sentinels selected by
typed section identity and the closed Autodesk-2009 producer profile. They are never artifact fields.

The provisional planner is likewise a profile-defined first pass over the logical sections, before the
two absent physical IDs 21/22 and before final cross-section patches. Its fixture matrix is:

| Section | Final page / D2 | Provisional page / D2 | Final semantic | Provisional semantic used by following `next` |
|---|---:|---:|---:|---:|
| ObjFreeSpace | 15 / 169 | 13 / 169 | 89 | unchanged |
| Template | 16 / 129 | 14 / 129 | 6 | 6 |
| Handles | 17 / 1907 | 15 / 1921 | 2085 | 2093 |
| Classes | 18 / 4656 | 16 / 4661 | 8194 | 8207 |
| AuxHeader | 19 / 205 | 17 / 203 | 123 | 123 |
| Header | 20 / 946 | 18 / 942 | 896 | 896 |

Implement this as a typed `Autodesk2009ScratchPlan` derived from section identities, final logical
materializations and the documented producer-pass deltas. Do not encode the table as byte literals and do
not expose it in public schemas. Unsupported application/build/version profiles reject exact export.

### Anti-replay implementation gate

Replace the live per-ID byte-slice patches with a single `derive_section_info_scratch(profile,
typed_history, compressed_page_map, provisional_page_plan)` function. The SectionInfo writer may only
copy absolute scratch slices and overwrite the final name/NUL. Tests must assert all 14 complete 64-byte
buffers, then assert that mutating a typed history field changes only the naturally intersecting scratch
slices and inverse restores the fixture. Anti-shadow scans must reject persisted name buffers, tails,
cookies, provisional lengths, PageMap bytes and profile scratch strings.

## 2026-08-15 Header CECOLOR-to-PEXTMIN Boundary Correction

The invalid first `PEXTMIN` BD selector was not a geometry-codec problem and no additional scalar is
version-gated between `PINSBASE` and `PEXTMIN`. The missing field is `HANDSEED`, whose primary-spec macro
is `FIELD_DATAHANDLE`, not `FIELD_HANDLE`. In AC1024 it remains inline in the Header main stream while
ordinary Header object relations are routed to the separated handle stream.

The exact fixture boundary, with positions relative to the 854-byte stream at Header semantic offset 24,
is:

| Main bits | Field | Exact native spelling/result |
|---:|---|---|
| through 1,121 | `CECOLOR` | typed index 0; CMC completes at bit 1,121 |
| `1,121..1,145` | `HANDSEED` data handle | H code 0, size 2, value `0x228d` / 8,845 |
| `1,145..1,147` | `PSVPSCALE` | BD selector `10`, value 0 |
| `1,147..1,153` | `PINSBASE` | three BD selector-`10` zeros |
| `1,153..1,351` | `PEXTMIN` | three ordinary raw BDs, each selector `00`; values `(0.62883212269412,0.79966732798827,-1.12497908679288e-11)` |
| `1,351..1,549` | `PEXTMAX` | three raw BDs, each selector `00`; values `(9.02982107914386,7.20016027280744,2.955929880312366e-13)` |

This independently matches LibreDWG's native trace: CECOLOR finishes at section `164.1`, HANDSEED at
`167.1`, PSVPSCALE at `167.3`, PINSBASE at `168.1`, PEXTMIN at `192.7`, and PEXTMAX at `217.5` after
adding the 24-byte Header framing prefix.

Code-level correction:

1. immediately after CMC, `main.read_handle()` and require `(code=0,value=8845)`; assign the derived
   logical handle seed validation target;
2. writer emits `main.write_handle(0, derived_handle_seed)` at the same point;
3. remove `handle_seed` from the separated handle stream entirely;
4. the first separated handle at boundary 6,136 is `CLAYER` with code 5/value 2,109, followed by
   `TEXTSTYLE`, `CELTYPE`, `CMATERIAL`, `DIMSTYLE`, and `CMLSTYLE`;
5. keep `PSVPSCALE`, `PINSBASE`, `PEXTMIN` and all later geometry codecs unchanged.

Without the 24-bit data handle, a decoder interprets its leading `00` as PSVPSCALE's raw-BD selector,
consumes 66 bits of handle/geometry data as a nonsensical double, and inevitably reaches selector `11`
inside the first extent. Skipping arbitrary bits would hide the semantic field and shift the later handle
relations; only the typed inline data-handle correction is valid. The earlier handoff table entry which
grouped HANDSEED with the separated current-state relations is superseded by this section.

## 2026-08-15 Header String Stream and Handle-Origin Correction

The apparent 32-bit gap after the R2010 Header string footer is an origin error, not an omitted string or
another persisted Header concept. The stored Header `bitsize` value 6,136 is measured from the start of
the four-byte `bitsize` word at Header byte 20. The semantic bit reader used by the live R2010 Header
implementation begins after that word at Header byte 24. Therefore the separated handle stream begins at
semantic-stream bit `6,136 - 32 = 6,104`.

All 14 primary-spec `TU` fields decode exactly and contiguously from bit 4,779. The fixture oracle is:

| Ordered logical field | Value | Bit interval | Encoded bits |
|---|---|---:|---:|
| `unit1_name` | `"m"` | `4,779..4,805` | 26 |
| `unit2_name` | `""` | `4,805..4,807` | 2 |
| `unit3_name` | `""` | `4,807..4,809` | 2 |
| `unit4_name` | `""` | `4,809..4,811` | 2 |
| `MENU` | `"."` | `4,811..4,837` | 26 |
| `DIMPOST` | `""` | `4,837..4,839` | 2 |
| `DIMAPOST` | `""` | `4,839..4,841` | 2 |
| `DIMALTMZS` | `""` | `4,841..4,843` | 2 |
| `DIMMZS` | `""` | `4,843..4,845` | 2 |
| `HYPERLINKBASE` | `""` | `4,845..4,847` | 2 |
| `STYLESHEET` | `""` | `4,847..4,849` | 2 |
| `FINGERPRINTGUID` | `"{AE360294-492A-4B40-8D12-1DA91F648E9C}"` | `4,849..5,467` | 618 |
| `VERSIONGUID` | `"{83F64250-0F55-40D4-AE09-768E87CF41F7}"` | `5,467..6,085` | 618 |
| `PROJECTNAME` | `""` | `6,085..6,087` | 2 |

Each value uses the standard AC1024 `TU` codec: a `BS` UTF-16 code-unit count followed by that many
little-endian `RS` code units. Their combined encoded content is exactly 1,308 bits. The ordinary R2010
string footer follows immediately: `RS(1308)` at `6,087..6,103`, then present `B(true)` at
`6,103..6,104`. The exact invariants are `content_end - content_start = 1,308` and
`footer_end = handle_start = 6,104`.

The four previously reported terminal values `[1, 180, 36874, 32843]` are not an independent main-stream
tail. Reading four `BS` values over the first 56 bits of the string stream happens to produce exactly that
sequence. Emitting them and then emitting the strings duplicates the same physical bits and is invalid.
They may be retained only as a derived diagnostic of the already encoded string prefix, never as schema
state or a separate writer step.

Native bits `6,104..6,136` prove the corrected handle origin. Grouped from bit 6,104 they are
`52 08 3d 52`: the complete `CLAYER` handle `(code=5,size=2,value=0x083d)` at `6,104..6,128`, followed
by the first byte of the `TEXTSTYLE` handle `(code=5,size=2,value=0x0178)` beginning at 6,128. Starting
handles at 6,136 lands eight bits into `TEXTSTYLE` and cannot decode a valid handle header.

Code-ready decode/write order:

1. stop the Header main-field cursor at bit 4,779 after `SHADOWPLANELOCATION` and do not read or write a
   separate terminal profile;
2. decode/encode the 14 typed `TU` fields in the table order, requiring content end 6,087;
3. decode/encode the `RS` content bit count 1,308 and present bit true, requiring footer end 6,104;
4. derive `handle_start = stored_bitsize - 32` because the stored origin is byte 20 and the semantic
   reader origin is byte 24; reject any other relationship;
5. decode/encode separated Header handles from bit 6,104, beginning `CLAYER`, `TEXTSTYLE`, `CELTYPE`,
   `CMATERIAL`, `DIMSTYLE`, `CMLSTYLE`; keep the inline `HANDSEED` correction from the preceding section;
6. continue serializing stored `bitsize=6,136`; it is a derived boundary marker, not the semantic-reader
   cursor and not artifact state.

Exact-frame tests must assert every TU endpoint, the 1,308-bit footer, both coordinate origins, the first
two handle encodings, and full Header byte identity. Anti-shadow tests must reject persisted terminal
profiles, boundary adjustments, or duplicate raw string/handle regions.

## 2026-08-15 Header Terminal Compatibility and Framing Tail

The 80 bits after the final named Header relation are neither an opaque cookie nor part of the section
CRC. They decode exactly as the four R14+ compatibility shorts named `unknown_54..unknown_57` by the
primary Header-variable specification, followed by the documented R2004+ terminal fields and derived
bit fill. Their physical placement is after the separated handle stream in this AC1024 materialization.

The exact fixture interpretation, relative to the 854-byte semantic stream beginning at Header byte 24,
is:

| Bits | Typed/framing role | Decoded value | Codec |
|---:|---|---:|---|
| `6,752..6,770` | producer compatibility short 54 | `0xbfc4` / 49,092 | `BS`, selector `00` plus little-endian `RS` |
| `6,770..6,788` | producer compatibility short 55 | `0x122d` / 4,653 | `BS`, selector `00` plus little-endian `RS` |
| `6,788..6,806` | producer compatibility short 56 | `0xa23e` / 41,534 | `BS`, selector `00` plus little-endian `RS` |
| `6,806..6,824` | producer compatibility short 57 | `0xb717` / 46,871 | `BS`, selector `00` plus little-endian `RS` |
| `6,824..6,826` | R2004+ terminal long 1 | zero | `BL`, selector `10` |
| `6,826..6,828` | R2004+ terminal long 2 | zero | `BL`, selector `10` |
| `6,828..6,829` | terminal presence/end marker | true | `B` |
| `6,829..6,832` | terminal spear-shift fill | three one bits | derived fill to the next byte |

Encoding those typed values and framing bits yields the complete native tail
`31 2f c2 d1 20 fa 88 17 b7 af`. This is an independent bit-code reconstruction, not a retained byte
literal. The ODA Header table independently specifies four R14+ bit shorts immediately before the data
section CRC. ACadSharp's `DwgHeaderWriter` independently specifies the two R2004+ bit-long zeros and
terminal bit before `WriteSpearShift`; the fixture oracle corrects that generic writer's assumed false/zero
fill to this producer's true/one-fill spelling.

The actual Header CRC remains the following little-endian `84 d0` at section bytes `878..880`. It equals
`0xd084`, the CRC16 with seed `0xc0c1` over section bytes `16..878`, including this terminal tail. The
leading tail bytes `31 2f` are not a CRC: they are the packed beginning of compatibility short 54.

Code-ready materialization rule:

1. encode every typed Header relation through `DRAGVS`; the handle cursor then equals bit 6,752;
2. append the four `BS` values from the closed `Autodesk2009D400200HeaderCompatibilityProfile`;
3. append `BL(0)`, `BL(0)`, `B(true)` and three derived one-fill bits;
4. require semantic stream bit length 6,832 / byte length 854;
5. assemble begin sentinel, section size 858, bitsize 6,136, stream, derived CRC16 `0xd084`, and end
   sentinel, requiring total Header length 896.

The four compatibility values are producer-profile validation constants, not drawing semantics and not
persisted schema fields. The zeros, marker and fill are framing derived during native serialization. A
decoder must validate all eight components and reject another profile atomically. Anti-shadow tests must
reject any Header-tail byte array, compatibility-short artifact fields, persisted CRC, or persisted fill.

This section supersedes the preceding section's claim that the four compatibility shorts alias the first
56 string bits. The string stream still occurs exactly once at bits `4,779..6,104`; only the interpretation
of `unknown_54..unknown_57` changes. LibreDWG's trace was reading those four values through its main
cursor over the separated string region and therefore did not identify their true terminal physical
location. The native tail's complete 80-bit structured decode is the authoritative fixture oracle.

## 2026-08-15 R2004 Primary-Header Magic Tail Correction

The final 20 bytes of the 256-byte file preamble have a complete standard derivation. They are not a
second encrypted payload and contain no additional logical fields. The ODA R2004 file-header rule states
that the next `0x14` bytes after the 108-byte encrypted record are copied from the magic-number sequence
starting at `0x100 - 0x14 = 0xec`.

Generate a 256-byte magic table with unsigned 32-bit wrapping arithmetic:

1. initialize `state = 1`;
2. for each table index, set `state = state * 0x343fd + 0x269ec3`;
3. emit `(state >> 16) & 0xff`.

The two uses of that one generated table are distinct:

| File span | Magic-table span | Materialization rule |
|---:|---:|---|
| `0x80..0xec` | `0..108` | XOR the 108-byte typed R2004 header record with the generated bytes |
| `0xec..0x100` | `236..256` / `0xec..0x100` | copy generated bytes directly |

Indices 108..235 are not consumed by either operation. The fixture's direct extension is exactly
`f8466a0496730ed9162f6768d4f74a4ad0576876`, equal byte-for-byte to generated table indices 236..255.
The previously reported value `4134f74dbaf3701c8ffa8ee8661d838683e80fa0` resulted only from
incorrectly XORing the direct extension with table indices 108..127. It has no plaintext interpretation;
the apparent 12-byte prefix and remaining eight-byte suffix are both artifacts of the wrong operation.

The writer must share one pure `r2004_magic_byte(index)` derivation between record encryption and direct
tail generation, require exact preamble length `0x100`, and persist neither magic bytes, ciphertext,
extension bytes nor the erroneous apparent plaintext. Tests must generate all 256 magic bytes from the
LCG, assert the two disjoint table spans, decrypt and validate the typed 108-byte record, and assert the
complete native preamble. Anti-shadow scans must reject any preamble extension or padding byte array.

## 2026-08-15 Exact Native and Lifecycle Gate Evidence

The final fixture used by every gate is `/Users/ueli/Documents/semio/temp/architectural_example.dwg`,
length `148,638`, SHA-256
`52d14a7bdb946099d3cf16fd276d19bd8924348fd02b2ddd0003cd4f6b34cce7`.

All accepted gates were executed through Nx against `@semio-tech/stdio-plugin:test-long`:

| Test filter | Result | Proven route |
|---|---:|---|
| `header_semantic_section_reencodes_exactly` | `1 passed` | typed Header main/string/handle streams, terminal compatibility fields, CRC and sentinels produce the exact 896-byte semantic section |
| `real_decode_stays_lossless_on_reencode` | `1 passed` | complete native decode followed by deterministic whole-file materialization produces all 148,638 fixture bytes exactly |
| `snapshot_pack_preserves_signed_zero_semantics` | `1 passed` | generic ArtifactPack preserves the XRECORD IEEE `-0.0` value which changes the native object stream if normalized |
| `exact_fixture_roundtrips_through_snapshot_diff_mutation_and_raw_io` | `1 passed` | raw native IO, snapshot DSL, snapshot pack, analyzer from DSL, composer from pack, empty diff apply, no-op mutation, version mutation/redecode, inverse and absorb all export the exact fixture |
| `well_known_fixture_lossless_system_roundtrip` | `1 passed` | mutation text/binary codecs, persisted diff text/binary codecs, set-snapshot application, diff from default, inverse mutation and native IO all preserve exact export |

The shared pack edge-case test was updated to require negative-zero sign preservation across scalar,
packed-sequence, table and DSL-number paths. The encompassing framework-os test target reached
`862 passed`; its sole failure was the unrelated pre-existing cross-artifact fixture-sweep rejection
law, not the pack value tests. `git diff --check` completed with no diagnostics for the DWG IO, DWG
architectural tests and shared pack value codec.

No native source bytes, page buffers, physical sections, offsets, compression state, compatibility
tails, preamble bytes or raw replay fields are persisted in the snapshot, diff or mutation schemas.
All native framing is derived only inside serialization from logical standard concepts and closed
producer/standard profiles validated during deserialization.
